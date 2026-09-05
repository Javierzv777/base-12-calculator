//! `dz-eval` -- Layer 4: AST evaluator. Errors (division by zero, etc.) are
//! values (`Result<DzFixed, EvalError>`), per PLAN.md Section 6.
//!
//! `DozOp` is the extension point: adding a function to the calculator
//! means implementing `DozOp` and registering it in an `OpRegistry` --
//! zero changes to `evaluate` or the core grammar (PLAN.md Section 6).

use dz_gates::DozGates;
use dz_int::DivRemError;
use dz_num::{DzFixed, DzFloat, DzRational, RationalError};
use dz_parse::Expr;
use std::collections::HashMap;

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

/// Extension point (PLAN.md Section 6): implement this for a new function
/// and register it in an `OpRegistry`; `evaluate` needs no changes.
pub trait DozOp {
    fn name(&self) -> &'static str;
    fn arity(&self) -> usize;
    fn apply(&self, g: &dyn DozGates, args: &[DzFixed]) -> Result<DzFixed, EvalError>;
}

#[derive(Default)]
pub struct OpRegistry {
    ops: HashMap<&'static str, Box<dyn DozOp>>,
}

impl OpRegistry {
    pub fn new() -> Self {
        OpRegistry { ops: HashMap::new() }
    }

    pub fn register(&mut self, op: Box<dyn DozOp>) {
        self.ops.insert(op.name(), op);
    }

    pub fn call(&self, g: &dyn DozGates, name: &str, args: &[DzFixed]) -> Result<DzFixed, EvalError> {
        let op = self
            .ops
            .get(name)
            .ok_or_else(|| EvalError::UnknownFunction(name.to_string()))?;
        if args.len() != op.arity() {
            return Err(EvalError::WrongArity { expected: op.arity(), got: args.len() });
        }
        op.apply(g, args)
    }
}

/// Example extension: absolute value, demonstrating the `DozOp` mechanism.
pub struct AbsOp;
impl DozOp for AbsOp {
    fn name(&self) -> &'static str {
        "abs"
    }
    fn arity(&self) -> usize {
        1
    }
    fn apply(&self, _g: &dyn DozGates, args: &[DzFixed]) -> Result<DzFixed, EvalError> {
        let v = &args[0];
        Ok(if v.is_negative() { v.neg() } else { v.clone() })
    }
}

/// Evaluate an AST built by `dz-parse` into a `DzFixed`, using the given
/// `DozGates` backend for all underlying digit-level work.
pub fn evaluate<G: DozGates>(g: &G, expr: &Expr) -> Result<DzFixed, EvalError> {
    match expr {
        Expr::Number { int_digits, frac_digits } => {
            Ok(DzFixed::from_digits(false, int_digits, frac_digits))
        }
        Expr::Neg(inner) => Ok(evaluate(g, inner)?.neg()),
        Expr::Add(a, b) => Ok(evaluate(g, a)?.add(g, &evaluate(g, b)?)),
        Expr::Sub(a, b) => Ok(evaluate(g, a)?.sub(g, &evaluate(g, b)?)),
        Expr::Mul(a, b) => Ok(evaluate(g, a)?.mul(g, &evaluate(g, b)?)),
        Expr::Div(a, b) => {
            let av = evaluate(g, a)?;
            let bv = evaluate(g, b)?;
            Ok(av.div(g, &bv, dz_num::DEFAULT_DIV_SCALE)?)
        }
        Expr::Mod(a, b) => {
            let av = evaluate(g, a)?;
            let bv = evaluate(g, b)?;
            Ok(av.rem(g, &bv)?)
        }
        Expr::Pow(base, exp) => {
            let base_v = evaluate(g, base)?;
            let exp_v = evaluate(g, exp)?;
            if exp_v.is_negative() {
                return Err(EvalError::NegativeExponent);
            }
            // v1 supports non-negative integer exponents only (PLAN.md
            // Section 6 lists `^` in the initial operator set without
            // requiring fractional/negative exponents yet).
            let exponent = dzfixed_to_u32_exponent(g, &exp_v)?;
            Ok(base_v.pow(g, exponent))
        }
    }
}

/// Evaluate into an exact `DzRational` -- the REPL's default mode, since
/// `1/3` should print as the exact fraction `1/3`, not a truncated
/// fixed-point approximation (PLAN.md Phase 5).
pub fn evaluate_rational<G: DozGates>(g: &G, expr: &Expr) -> Result<DzRational, EvalError> {
    match expr {
        Expr::Number { int_digits, frac_digits } => {
            Ok(DzRational::from_digits(g, false, int_digits, frac_digits))
        }
        Expr::Neg(inner) => Ok(evaluate_rational(g, inner)?.neg()),
        Expr::Add(a, b) => Ok(evaluate_rational(g, a)?.add(g, &evaluate_rational(g, b)?)?),
        Expr::Sub(a, b) => Ok(evaluate_rational(g, a)?.sub(g, &evaluate_rational(g, b)?)?),
        Expr::Mul(a, b) => Ok(evaluate_rational(g, a)?.mul(g, &evaluate_rational(g, b)?)?),
        Expr::Div(a, b) => Ok(evaluate_rational(g, a)?.div(g, &evaluate_rational(g, b)?)?),
        Expr::Mod(a, b) => Ok(evaluate_rational(g, a)?.rem(g, &evaluate_rational(g, b)?)?),
        Expr::Pow(base, exp) => {
            let base_v = evaluate_rational(g, base)?;
            let exp_v = evaluate_rational(g, exp)?;
            if exp_v.is_negative() {
                return Err(EvalError::NegativeExponent);
            }
            let exponent = rational_to_u32_exponent(g, &exp_v)?;
            Ok(base_v.pow(g, exponent)?)
        }
    }
}

/// Evaluate into DFP (dozenal floating point): builds the exact rational
/// result of the whole expression (so intermediate rounding doesn't
/// compound across operations) and rounds only once at the end. A stricter
/// alternative -- rounding after every operation, as real hardware FP does
/// -- is noted as a possible extension in `docs/03-formato-dfp.md`.
pub fn evaluate_dfp<G: DozGates>(g: &G, expr: &Expr) -> Result<DzFloat, EvalError> {
    let exact = evaluate_rational(g, expr)?;
    let approx = exact.to_fixed(g, dz_num::DEFAULT_DIV_SCALE);
    Ok(DzFloat::from_dzfixed(g, &approx))
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
    let mut probe = dz_num::DzFixed::from_int(dz_int::Dz::from_magnitude(false, vec![dz_gates::Digit::ZERO]));
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
}
