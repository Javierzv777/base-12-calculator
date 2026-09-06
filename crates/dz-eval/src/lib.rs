//! `dz-eval` -- Layer 4: AST evaluator. Errors (division by zero, etc.) are
//! values (`Result<DzFixed, EvalError>`), per PLAN.md Section 6.
//!
//! `DozFunction` + `FnRegistry` (module `functions`) are the extension
//! point: adding a function to the calculator means implementing the trait
//! and registering it -- zero changes to `evaluate` or the grammar. The
//! built-in set (module `builtins`) binds names to `dz-sci`'s algorithms,
//! which is how `sin`, `ln` and `sqrt` reach the expression language.
//!
//! Two evaluation entry points exist per mode:
//!
//!   - the plain ones (`evaluate`, `evaluate_rational`, `evaluate_dfp`),
//!     unchanged since Phase 5, which use the built-in registry and the
//!     default working scale;
//!   - the `_with` ones, which take an `EvalConfig` (registry + scale) and
//!     return an `Evaluated<T>`: the value plus whether any part of the
//!     expression was approximated. Interfaces use these, so a result that
//!     went through `sin` can be shown as an approximation instead of
//!     being printed as an exact fraction that happens to have twelve
//!     correct digits.

pub mod builtins;
pub mod functions;

pub use functions::{Arity, Category, DozFunction, FnContext, FnInfo, FnRegistry, FnValue};

use dz_gates::DozGates;
use dz_int::DivRemError;
use dz_num::{DzFixed, DzFloat, DzRational, RationalError};
use dz_parse::Expr;
use std::sync::OnceLock;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvalError {
    DivisionByZero,
    NegativeExponent,
    /// The exponent of `^` is not a non-negative integer (e.g. `2^1;6`).
    NonIntegerExponent,
    /// The exponent of `^` is a non-negative integer but exceeds the
    /// practical bound the REPL is willing to compute (e.g. `2^2000`).
    ExponentTooLarge,
    UnknownFunction(String),
    WrongArity { expected: usize, got: usize },
    /// A function was called outside its domain (`ln(0)`, `asin(2)`).
    Domain(String),
    /// A result would need an impractical number of digits (`exp` of a
    /// very large argument).
    Overflow,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::DivisionByZero => write!(f, "division by zero"),
            EvalError::NegativeExponent => write!(f, "negative exponents are not supported"),
            EvalError::NonIntegerExponent => {
                write!(f, "non-integer exponents are not supported")
            }
            EvalError::ExponentTooLarge => write!(f, "exponent is too large"),
            EvalError::UnknownFunction(name) => write!(f, "unknown function '{}'", name),
            EvalError::WrongArity { expected, got } => {
                write!(f, "wrong number of arguments: expected {}, got {}", expected, got)
            }
            EvalError::Domain(what) => write!(f, "argument outside domain: {}", what),
            EvalError::Overflow => write!(f, "result is too large to represent"),
        }
    }
}
impl std::error::Error for EvalError {}

impl From<DivRemError> for EvalError {
    fn from(_: DivRemError) -> Self {
        EvalError::DivisionByZero
    }
}

impl From<RationalError> for EvalError {
    fn from(_: RationalError) -> Self {
        EvalError::DivisionByZero
    }
}

impl From<dz_sci::SciError> for EvalError {
    fn from(e: dz_sci::SciError) -> Self {
        match e {
            dz_sci::SciError::Domain(what) => EvalError::Domain(what.to_string()),
            dz_sci::SciError::DivisionByZero => EvalError::DivisionByZero,
            dz_sci::SciError::Overflow => EvalError::Overflow,
        }
    }
}

/// The built-in registry, built once. Interfaces that add their own
/// functions build their own `FnRegistry` instead of mutating this one.
pub fn default_registry() -> &'static FnRegistry {
    static REGISTRY: OnceLock<FnRegistry> = OnceLock::new();
    REGISTRY.get_or_init(FnRegistry::with_builtins)
}

/// What a single evaluation needs beyond the expression itself: which
/// functions exist, and how many fractional digits the approximate ones
/// should work to.
#[derive(Clone, Copy)]
pub struct EvalConfig<'r> {
    pub registry: &'r FnRegistry,
    pub scale: usize,
}

impl EvalConfig<'static> {
    pub fn defaults() -> EvalConfig<'static> {
        EvalConfig {
            registry: default_registry(),
            scale: dz_sci::DEFAULT_SCI_SCALE,
        }
    }
}

impl EvalConfig<'_> {
    fn fn_context(&self) -> FnContext {
        FnContext { scale: self.scale }
    }
}

/// A value plus whether any step that produced it was approximated.
/// `1/3 + 1` is exact; `sin(1) + 1` is not, and stays flagged all the way
/// out to the display.
#[derive(Clone, Debug)]
pub struct Evaluated<T> {
    pub value: T,
    pub approximate: bool,
}

/// Evaluate an AST built by `dz-parse` into a `DzFixed`, using the given
/// `DozGates` backend for all underlying digit-level work.
pub fn evaluate<G: DozGates>(g: &G, expr: &Expr) -> Result<DzFixed, EvalError> {
    Ok(evaluate_fixed_with(g, expr, &EvalConfig::defaults())?.value)
}

/// Evaluate into an exact `DzRational` -- the REPL's default mode, since
/// `1/3` should print as the exact fraction `1/3`, not a truncated
/// fixed-point approximation (PLAN.md Phase 5).
pub fn evaluate_rational<G: DozGates>(g: &G, expr: &Expr) -> Result<DzRational, EvalError> {
    Ok(evaluate_rational_with(g, expr, &EvalConfig::defaults())?.value)
}

/// Evaluate into DFP (dozenal floating point): builds the exact rational
/// result of the whole expression (so intermediate rounding doesn't
/// compound across operations) and rounds only once at the end. A stricter
/// alternative -- rounding after every operation, as real hardware FP does
/// -- is noted as a possible extension in `docs/03-formato-dfp.md`.
pub fn evaluate_dfp<G: DozGates>(g: &G, expr: &Expr) -> Result<DzFloat, EvalError> {
    Ok(evaluate_dfp_with(g, expr, &EvalConfig::defaults())?.value)
}

/// Fixed-point evaluation with an explicit registry and working scale.
pub fn evaluate_fixed_with<G: DozGates>(
    g: &G,
    expr: &Expr,
    config: &EvalConfig<'_>,
) -> Result<Evaluated<DzFixed>, EvalError> {
    let (value, approximate) = eval_fixed(g, expr, config)?;
    Ok(Evaluated { value, approximate })
}

/// Exact-rational evaluation with an explicit registry and working scale.
pub fn evaluate_rational_with<G: DozGates>(
    g: &G,
    expr: &Expr,
    config: &EvalConfig<'_>,
) -> Result<Evaluated<DzRational>, EvalError> {
    let (value, approximate) = eval_rational(g, expr, config)?;
    Ok(Evaluated { value, approximate })
}

/// DFP evaluation with an explicit registry and working scale.
pub fn evaluate_dfp_with<G: DozGates>(
    g: &G,
    expr: &Expr,
    config: &EvalConfig<'_>,
) -> Result<Evaluated<DzFloat>, EvalError> {
    let (exact, approximate) = eval_rational(g, expr, config)?;
    let approx = exact.to_fixed(g, dz_num::DEFAULT_DIV_SCALE);
    Ok(Evaluated {
        value: DzFloat::from_dzfixed(g, &approx),
        approximate,
    })
}

fn eval_fixed<G: DozGates>(
    g: &G,
    expr: &Expr,
    config: &EvalConfig<'_>,
) -> Result<(DzFixed, bool), EvalError> {
    match expr {
        Expr::Number { int_digits, frac_digits } => {
            Ok((DzFixed::from_digits(false, int_digits, frac_digits), false))
        }
        Expr::Neg(inner) => {
            let (value, approximate) = eval_fixed(g, inner, config)?;
            Ok((value.neg(), approximate))
        }
        Expr::Add(a, b) => {
            let (av, aa) = eval_fixed(g, a, config)?;
            let (bv, ba) = eval_fixed(g, b, config)?;
            Ok((av.add(g, &bv), aa || ba))
        }
        Expr::Sub(a, b) => {
            let (av, aa) = eval_fixed(g, a, config)?;
            let (bv, ba) = eval_fixed(g, b, config)?;
            Ok((av.sub(g, &bv), aa || ba))
        }
        Expr::Mul(a, b) => {
            let (av, aa) = eval_fixed(g, a, config)?;
            let (bv, ba) = eval_fixed(g, b, config)?;
            Ok((av.mul(g, &bv), aa || ba))
        }
        Expr::Div(a, b) => {
            let (av, aa) = eval_fixed(g, a, config)?;
            let (bv, ba) = eval_fixed(g, b, config)?;
            Ok((av.div(g, &bv, dz_num::DEFAULT_DIV_SCALE)?, aa || ba))
        }
        Expr::Mod(a, b) => {
            let (av, aa) = eval_fixed(g, a, config)?;
            let (bv, ba) = eval_fixed(g, b, config)?;
            Ok((av.rem(g, &bv)?, aa || ba))
        }
        Expr::Pow(base, exp) => {
            let (base_v, ba) = eval_fixed(g, base, config)?;
            let (exp_v, ea) = eval_fixed(g, exp, config)?;
            if exp_v.is_negative() {
                return Err(EvalError::NegativeExponent);
            }
            // v1 supports non-negative integer exponents only (PLAN.md
            // Section 6 lists `^` in the initial operator set without
            // requiring fractional/negative exponents yet).
            let exponent = dzfixed_to_u32_exponent(g, &exp_v)?;
            Ok((base_v.pow(g, exponent), ba || ea))
        }
        Expr::Call { name, args } => {
            let function = config.registry.check_arity(name, args.len())?;
            let mut values = Vec::with_capacity(args.len());
            let mut approximate = false;
            for argument in args {
                let (value, was_approximate) = eval_fixed(g, argument, config)?;
                approximate = approximate || was_approximate;
                values.push(value);
            }
            let out = function.apply_fixed(g, &values, &config.fn_context())?;
            Ok((out.value, approximate || !out.exact))
        }
    }
}

fn eval_rational<G: DozGates>(
    g: &G,
    expr: &Expr,
    config: &EvalConfig<'_>,
) -> Result<(DzRational, bool), EvalError> {
    match expr {
        Expr::Number { int_digits, frac_digits } => Ok((
            DzRational::from_digits(g, false, int_digits, frac_digits),
            false,
        )),
        Expr::Neg(inner) => {
            let (value, approximate) = eval_rational(g, inner, config)?;
            Ok((value.neg(), approximate))
        }
        Expr::Add(a, b) => {
            let (av, aa) = eval_rational(g, a, config)?;
            let (bv, ba) = eval_rational(g, b, config)?;
            Ok((av.add(g, &bv)?, aa || ba))
        }
        Expr::Sub(a, b) => {
            let (av, aa) = eval_rational(g, a, config)?;
            let (bv, ba) = eval_rational(g, b, config)?;
            Ok((av.sub(g, &bv)?, aa || ba))
        }
        Expr::Mul(a, b) => {
            let (av, aa) = eval_rational(g, a, config)?;
            let (bv, ba) = eval_rational(g, b, config)?;
            Ok((av.mul(g, &bv)?, aa || ba))
        }
        Expr::Div(a, b) => {
            let (av, aa) = eval_rational(g, a, config)?;
            let (bv, ba) = eval_rational(g, b, config)?;
            Ok((av.div(g, &bv)?, aa || ba))
        }
        Expr::Mod(a, b) => {
            let (av, aa) = eval_rational(g, a, config)?;
            let (bv, ba) = eval_rational(g, b, config)?;
            Ok((av.rem(g, &bv)?, aa || ba))
        }
        Expr::Pow(base, exp) => {
            let (base_v, ba) = eval_rational(g, base, config)?;
            let (exp_v, ea) = eval_rational(g, exp, config)?;
            if exp_v.is_negative() {
                return Err(EvalError::NegativeExponent);
            }
            let exponent = rational_to_u32_exponent(g, &exp_v)?;
            Ok((base_v.pow(g, exponent)?, ba || ea))
        }
        Expr::Call { name, args } => {
            let function = config.registry.check_arity(name, args.len())?;
            let mut values = Vec::with_capacity(args.len());
            let mut approximate = false;
            for argument in args {
                let (value, was_approximate) = eval_rational(g, argument, config)?;
                approximate = approximate || was_approximate;
                values.push(value);
            }
            let out = function.apply_rational(g, &values, &config.fn_context())?;
            Ok((out.value, approximate || !out.exact))
        }
    }
}

/// Extract a small non-negative integer exponent from a `DzRational` by
/// counting upward from zero and gate-comparing against `v`, rather than
/// reading the value as a native integer (PLAN.md purity rule).
///
/// Bounded by `MAX_EXPONENT` to keep the REPL responsive. Distinguishes two
/// failure modes: if the running count ever overshoots `v` without ever
/// hitting it exactly, `v` cannot be a non-negative integer at all (e.g. a
/// fractional exponent like `1;6`); if the count reaches `MAX_EXPONENT`
/// while still short of `v`, `v` is a valid but impractically large integer.
fn rational_to_u32_exponent<G: DozGates>(g: &G, v: &DzRational) -> Result<u32, EvalError> {
    const MAX_EXPONENT: u32 = 1000;
    let mut n: u32 = 0;
    let mut probe = DzRational::zero();
    let one = DzRational::from_int(dz_int::Dz::from_magnitude(false, vec![dz_gates::Digit::ONE]));
    loop {
        match probe.compare(g, v) {
            std::cmp::Ordering::Equal => return Ok(n),
            std::cmp::Ordering::Greater => return Err(EvalError::NonIntegerExponent),
            std::cmp::Ordering::Less => {}
        }
        if n == MAX_EXPONENT {
            return Err(EvalError::ExponentTooLarge);
        }
        probe = probe.add(g, &one).expect("adding by one never divides by zero");
        n += 1;
    }
}

/// Extract a small non-negative exponent from a `DzFixed` known to be an
/// integer (`scale == 0`) with a modest value, by repeatedly testing
/// against `Dz` values built from `DzFixed::pow`'s own loop -- i.e., we
/// count upward, gate-comparing, rather than reading the magnitude as a
/// native integer. Uses the caller's own backend `g` (mirroring
/// `rational_to_u32_exponent`), so `:backend crt`/`:backend table` apply
/// here too and this extraction's gate cost lands in the caller's own
/// `GateCount` rather than being computed on a throwaway `MvlGates`.
/// Same bound and same two failure modes as `rational_to_u32_exponent`.
fn dzfixed_to_u32_exponent<G: DozGates>(g: &G, v: &dz_num::DzFixed) -> Result<u32, EvalError> {
    // Practical bound; `dz-cli`'s REPL is not intended for huge exponents.
    const MAX_EXPONENT: u32 = 1000;
    let mut n: u32 = 0;
    let mut probe =
        dz_num::DzFixed::from_int(dz_int::Dz::from_magnitude(false, vec![dz_gates::Digit::ZERO]));
    let one = DzFixed::from_int(dz_int::Dz::from_magnitude(false, vec![dz_gates::Digit::ONE]));
    loop {
        match probe.compare(g, v) {
            std::cmp::Ordering::Equal => return Ok(n),
            std::cmp::Ordering::Greater => return Err(EvalError::NonIntegerExponent),
            std::cmp::Ordering::Less => {}
        }
        if n == MAX_EXPONENT {
            return Err(EvalError::ExponentTooLarge);
        }
        probe = probe.add(g, &one);
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dz_gates::{CrtGates, MvlGates};

    fn expr(src: &str) -> Expr {
        dz_parse::parse_str(src).expect("test expression should parse")
    }

    #[test]
    fn pow_integer_exponent_still_works() {
        let g = MvlGates::new();
        let e = expr("2^3");
        let expected_fixed = evaluate(&g, &expr("8")).expect("literal 8 should evaluate");
        let fixed = evaluate(&g, &e).expect("2^3 should evaluate");
        assert_eq!(fixed, expected_fixed);

        let expected_rational = evaluate_rational(&g, &expr("8")).expect("literal 8 (rational)");
        let rational = evaluate_rational(&g, &e).expect("2^3 should evaluate (rational)");
        assert_eq!(rational.compare(&g, &expected_rational), std::cmp::Ordering::Equal);

        evaluate_dfp(&g, &e).expect("2^3 should evaluate (dfp)");
    }

    #[test]
    fn pow_non_integer_exponent_is_an_error() {
        let g = MvlGates::new();
        let e = expr("2^1;6");
        assert_eq!(evaluate(&g, &e), Err(EvalError::NonIntegerExponent));
        assert!(matches!(evaluate_rational(&g, &e), Err(EvalError::NonIntegerExponent)));
        assert!(matches!(evaluate_dfp(&g, &e), Err(EvalError::NonIntegerExponent)));
    }

    #[test]
    fn pow_exponent_too_large_is_an_error() {
        let g = MvlGates::new();
        let e = expr("2^2000");
        assert_eq!(evaluate(&g, &e), Err(EvalError::ExponentTooLarge));
        assert!(matches!(evaluate_rational(&g, &e), Err(EvalError::ExponentTooLarge)));
        assert!(matches!(evaluate_dfp(&g, &e), Err(EvalError::ExponentTooLarge)));
    }

    #[test]
    fn pow_exponent_extraction_uses_callers_backend() {
        // Regression for A5: `dzfixed_to_u32_exponent` used to hard-code a
        // throwaway `MvlGates` instead of the caller's backend. Evaluating
        // with a `CrtGates` backend must not panic or silently fall back to
        // MVL -- it should compute the same, correct result.
        let g = CrtGates::new();
        let e = expr("2^3");
        let expected = evaluate(&g, &expr("8")).expect("literal 8 should evaluate under CrtGates");
        let fixed = evaluate(&g, &e).expect("2^3 should evaluate under CrtGates");
        assert_eq!(fixed, expected);
    }

    #[test]
    fn exact_functions_stay_exact_in_rational_mode() {
        let g = MvlGates::new();
        let config = EvalConfig::defaults();
        // floor(-1/3) is -1, exactly, and must not be flagged approximate.
        let out = evaluate_rational_with(&g, &expr("floor(-1/3)"), &config)
            .expect("floor of a rational evaluates");
        assert!(!out.approximate);
        let expected = evaluate_rational(&g, &expr("-1")).expect("literal -1");
        assert_eq!(out.value.compare(&g, &expected), std::cmp::Ordering::Equal);
    }

    #[test]
    fn approximate_functions_flag_the_whole_expression() {
        let g = MvlGates::new();
        let config = EvalConfig { registry: default_registry(), scale: 6 };
        let out = evaluate_rational_with(&g, &expr("sin(1) + 1"), &config)
            .expect("sin evaluates");
        assert!(out.approximate, "an expression containing sin is approximate");

        let exact = evaluate_rational_with(&g, &expr("1/3 + 1"), &config).expect("exact");
        assert!(!exact.approximate);
    }

    #[test]
    fn sqrt_of_a_perfect_square_is_reported_exact() {
        let g = MvlGates::new();
        let config = EvalConfig { registry: default_registry(), scale: 6 };
        let out = evaluate_rational_with(&g, &expr("sqrt(9)"), &config).expect("sqrt evaluates");
        assert!(!out.approximate, "sqrt(9) is exactly 3");
        let expected = evaluate_rational(&g, &expr("3")).expect("literal 3");
        assert_eq!(out.value.compare(&g, &expected), std::cmp::Ordering::Equal);
    }

    #[test]
    fn unknown_function_and_wrong_arity_are_errors() {
        let g = MvlGates::new();
        let config = EvalConfig::defaults();
        assert!(matches!(
            evaluate_fixed_with(&g, &expr("nope(1)"), &config),
            Err(EvalError::UnknownFunction(_))
        ));
        assert!(matches!(
            evaluate_fixed_with(&g, &expr("sin(1, 2)"), &config),
            Err(EvalError::WrongArity { .. })
        ));
    }

    #[test]
    fn domain_errors_are_values_not_panics() {
        let g = MvlGates::new();
        let config = EvalConfig { registry: default_registry(), scale: 4 };
        assert!(matches!(
            evaluate_fixed_with(&g, &expr("ln(0)"), &config),
            Err(EvalError::Domain(_))
        ));
        assert!(matches!(
            evaluate_fixed_with(&g, &expr("sqrt(-4)"), &config),
            Err(EvalError::Domain(_))
        ));
    }
}
