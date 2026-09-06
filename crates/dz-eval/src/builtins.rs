//! The built-in function set: constants, rounding, roots, exponentials
//! and trigonometry.
//!
//! Every entry is the same shape -- a `SimpleFn` with two function
//! pointers -- so this file is a table of what the calculator knows, not a
//! pile of trait implementations. The mathematics lives in `dz-sci`; what
//! is written here is only the binding of a name and an arity to it, plus
//! the choice of whether the result is exact.

use crate::functions::{Arity, Category, DozFunction, FnContext, FnInfo, FnRegistry, FnValue};
use crate::EvalError;
use dz_gates::{DozGates, DynGates};
use dz_num::{DzFixed, DzRational};
use dz_sci::{constants, elementary, exact, trig};
use std::cmp::Ordering;

type FixedImpl = fn(&dyn DozGates, &[DzFixed], &FnContext) -> Result<FnValue<DzFixed>, EvalError>;
type RationalImpl =
    fn(&dyn DozGates, &[DzRational], &FnContext) -> Result<FnValue<DzRational>, EvalError>;

/// A registry entry. `rational` is `None` for functions that have no exact
/// implementation (every transcendental one), in which case the trait's
/// default fixed-point route applies.
struct SimpleFn {
    name: &'static str,
    arity: Arity,
    category: Category,
    description: &'static str,
    always_exact: bool,
    fixed: FixedImpl,
    rational: Option<RationalImpl>,
}

impl DozFunction for SimpleFn {
    fn name(&self) -> &'static str {
        self.name
    }
    fn arity(&self) -> Arity {
        self.arity
    }
    fn category(&self) -> Category {
        self.category
    }
    fn description(&self) -> &'static str {
        self.description
    }
    fn always_exact(&self) -> bool {
        self.always_exact
    }
    fn info(&self) -> FnInfo {
        FnInfo {
            name: self.name,
            arity: self.arity,
            category: self.category,
            description: self.description,
            always_exact: self.always_exact,
        }
    }
    fn apply_fixed(
        &self,
        g: &dyn DozGates,
        args: &[DzFixed],
        ctx: &FnContext,
    ) -> Result<FnValue<DzFixed>, EvalError> {
        (self.fixed)(g, args, ctx)
    }
    fn apply_rational(
        &self,
        g: &dyn DozGates,
        args: &[DzRational],
        ctx: &FnContext,
    ) -> Result<FnValue<DzRational>, EvalError> {
        if let Some(exact_impl) = self.rational {
            return exact_impl(g, args, ctx);
        }
        let gates = DynGates(g);
        let working = ctx.scale + dz_sci::GUARD_DIGITS;
        let fixed_args: Vec<DzFixed> = args.iter().map(|a| a.to_fixed(&gates, working)).collect();
        let out = (self.fixed)(g, &fixed_args, ctx)?;
        Ok(FnValue {
            value: DzRational::from_fixed(&gates, &out.value),
            exact: out.exact,
        })
    }
}

// ---------------------------------------------------------------- constants

fn fixed_pi(_g: &dyn DozGates, _a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::approximate(constants::pi(ctx.scale)))
}

fn fixed_tau(_g: &dyn DozGates, _a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::approximate(constants::two_pi(ctx.scale)))
}

fn fixed_euler(_g: &dyn DozGates, _a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::approximate(constants::euler(ctx.scale)))
}

// ----------------------------------------------------------------- rounding

fn fixed_abs(_g: &dyn DozGates, a: &[DzFixed], _c: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::exact(exact::abs_fixed(&a[0])))
}
fn rational_abs(
    _g: &dyn DozGates,
    a: &[DzRational],
    _c: &FnContext,
) -> Result<FnValue<DzRational>, EvalError> {
    Ok(FnValue::exact(exact::abs_rational(&a[0])))
}

fn fixed_sign(_g: &dyn DozGates, a: &[DzFixed], _c: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::exact(exact::sign_fixed(&a[0])))
}
fn rational_sign(
    _g: &dyn DozGates,
    a: &[DzRational],
    _c: &FnContext,
) -> Result<FnValue<DzRational>, EvalError> {
    Ok(FnValue::exact(exact::sign_rational(&a[0])))
}

fn fixed_floor(g: &dyn DozGates, a: &[DzFixed], _c: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::exact(exact::floor_fixed(&DynGates(g), &a[0])))
}
fn rational_floor(
    g: &dyn DozGates,
    a: &[DzRational],
    _c: &FnContext,
) -> Result<FnValue<DzRational>, EvalError> {
    Ok(FnValue::exact(exact::floor_rational(&DynGates(g), &a[0])))
}

fn fixed_ceil(g: &dyn DozGates, a: &[DzFixed], _c: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::exact(exact::ceil_fixed(&DynGates(g), &a[0])))
}
fn rational_ceil(
    g: &dyn DozGates,
    a: &[DzRational],
    _c: &FnContext,
) -> Result<FnValue<DzRational>, EvalError> {
    Ok(FnValue::exact(exact::ceil_rational(&DynGates(g), &a[0])))
}

fn fixed_round(g: &dyn DozGates, a: &[DzFixed], _c: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::exact(exact::round_fixed(&DynGates(g), &a[0])))
}
fn rational_round(
    g: &dyn DozGates,
    a: &[DzRational],
    _c: &FnContext,
) -> Result<FnValue<DzRational>, EvalError> {
    Ok(FnValue::exact(exact::round_rational(&DynGates(g), &a[0])))
}

fn fixed_trunc(_g: &dyn DozGates, a: &[DzFixed], _c: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::exact(exact::trunc_fixed(&a[0])))
}
fn rational_trunc(
    g: &dyn DozGates,
    a: &[DzRational],
    _c: &FnContext,
) -> Result<FnValue<DzRational>, EvalError> {
    Ok(FnValue::exact(exact::trunc_rational(&DynGates(g), &a[0])))
}

fn fixed_min(g: &dyn DozGates, a: &[DzFixed], _c: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    let gates = DynGates(g);
    let pick = if a[0].compare(&gates, &a[1]) == Ordering::Greater {
        a[1].clone()
    } else {
        a[0].clone()
    };
    Ok(FnValue::exact(pick))
}
fn rational_min(
    g: &dyn DozGates,
    a: &[DzRational],
    _c: &FnContext,
) -> Result<FnValue<DzRational>, EvalError> {
    let gates = DynGates(g);
    let pick = if a[0].compare(&gates, &a[1]) == Ordering::Greater {
        a[1].clone()
    } else {
        a[0].clone()
    };
    Ok(FnValue::exact(pick))
}

fn fixed_max(g: &dyn DozGates, a: &[DzFixed], _c: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    let gates = DynGates(g);
    let pick = if a[0].compare(&gates, &a[1]) == Ordering::Less {
        a[1].clone()
    } else {
        a[0].clone()
    };
    Ok(FnValue::exact(pick))
}
fn rational_max(
    g: &dyn DozGates,
    a: &[DzRational],
    _c: &FnContext,
) -> Result<FnValue<DzRational>, EvalError> {
    let gates = DynGates(g);
    let pick = if a[0].compare(&gates, &a[1]) == Ordering::Less {
        a[1].clone()
    } else {
        a[0].clone()
    };
    Ok(FnValue::exact(pick))
}

// -------------------------------------------------------------------- roots

fn fixed_sqrt(g: &dyn DozGates, a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    let (value, is_exact) = elementary::sqrt(&DynGates(g), &a[0], ctx.scale)?;
    Ok(FnValue { value, exact: is_exact })
}

fn fixed_hypot(g: &dyn DozGates, a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    let gates = DynGates(g);
    let sum = a[0].mul(&gates, &a[0]).add(&gates, &a[1].mul(&gates, &a[1]));
    let (value, is_exact) = elementary::sqrt(&gates, &sum, ctx.scale)?;
    Ok(FnValue { value, exact: is_exact })
}

// -------------------------------------------------------------- exponential

fn fixed_exp(g: &dyn DozGates, a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::approximate(elementary::exp(&DynGates(g), &a[0], ctx.scale)?))
}

fn fixed_ln(g: &dyn DozGates, a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::approximate(elementary::ln(&DynGates(g), &a[0], ctx.scale)?))
}

/// `log(x)` is the base-twelve logarithm -- the dozenal counterpart of a
/// decimal calculator's `log` key. `log(b, x)` takes an explicit base.
fn fixed_log(g: &dyn DozGates, a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    let gates = DynGates(g);
    if a.len() == 1 {
        let (value, is_exact) = elementary::log12(&gates, &a[0], ctx.scale)?;
        return Ok(FnValue { value, exact: is_exact });
    }
    Ok(FnValue::approximate(elementary::log_base(
        &gates, &a[0], &a[1], ctx.scale,
    )?))
}

// ------------------------------------------------------------- trigonometry

fn fixed_sin(g: &dyn DozGates, a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::approximate(trig::sin(&DynGates(g), &a[0], ctx.scale)?))
}
fn fixed_cos(g: &dyn DozGates, a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::approximate(trig::cos(&DynGates(g), &a[0], ctx.scale)?))
}
fn fixed_tan(g: &dyn DozGates, a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::approximate(trig::tan(&DynGates(g), &a[0], ctx.scale)?))
}
fn fixed_asin(g: &dyn DozGates, a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::approximate(trig::asin(&DynGates(g), &a[0], ctx.scale)?))
}
fn fixed_acos(g: &dyn DozGates, a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::approximate(trig::acos(&DynGates(g), &a[0], ctx.scale)?))
}
fn fixed_atan(g: &dyn DozGates, a: &[DzFixed], ctx: &FnContext) -> Result<FnValue<DzFixed>, EvalError> {
    Ok(FnValue::approximate(trig::atan(&DynGates(g), &a[0], ctx.scale)?))
}

/// The table. Order here is the order interfaces display.
pub fn register_all(registry: &mut FnRegistry) {
    let entries: Vec<SimpleFn> = vec![
        SimpleFn {
            name: "pi",
            arity: Arity::exact(0),
            category: Category::Constant,
            description: "ratio of a circle's circumference to its diameter",
            always_exact: false,
            fixed: fixed_pi,
            rational: None,
        },
        SimpleFn {
            name: "tau",
            arity: Arity::exact(0),
            category: Category::Constant,
            description: "a full turn, two pi",
            always_exact: false,
            fixed: fixed_tau,
            rational: None,
        },
        SimpleFn {
            name: "euler",
            arity: Arity::exact(0),
            category: Category::Constant,
            description: "base of the natural logarithm (spelled out: 'e' is the digit eleven)",
            always_exact: false,
            fixed: fixed_euler,
            rational: None,
        },
        SimpleFn {
            name: "abs",
            arity: Arity::exact(1),
            category: Category::Rounding,
            description: "absolute value",
            always_exact: true,
            fixed: fixed_abs,
            rational: Some(rational_abs),
        },
        SimpleFn {
            name: "sign",
            arity: Arity::exact(1),
            category: Category::Rounding,
            description: "minus one, zero or one",
            always_exact: true,
            fixed: fixed_sign,
            rational: Some(rational_sign),
        },
        SimpleFn {
            name: "floor",
            arity: Arity::exact(1),
            category: Category::Rounding,
            description: "largest integer at most the argument",
            always_exact: true,
            fixed: fixed_floor,
            rational: Some(rational_floor),
        },
        SimpleFn {
            name: "ceil",
            arity: Arity::exact(1),
            category: Category::Rounding,
            description: "smallest integer at least the argument",
            always_exact: true,
            fixed: fixed_ceil,
            rational: Some(rational_ceil),
        },
        SimpleFn {
            name: "round",
            arity: Arity::exact(1),
            category: Category::Rounding,
            description: "nearest integer, ties to even",
            always_exact: true,
            fixed: fixed_round,
            rational: Some(rational_round),
        },
        SimpleFn {
            name: "trunc",
            arity: Arity::exact(1),
            category: Category::Rounding,
            description: "integer part, toward zero",
            always_exact: true,
            fixed: fixed_trunc,
            rational: Some(rational_trunc),
        },
        SimpleFn {
            name: "min",
            arity: Arity::exact(2),
            category: Category::Rounding,
            description: "smaller of two values",
            always_exact: true,
            fixed: fixed_min,
            rational: Some(rational_min),
        },
        SimpleFn {
            name: "max",
            arity: Arity::exact(2),
            category: Category::Rounding,
            description: "larger of two values",
            always_exact: true,
            fixed: fixed_max,
            rational: Some(rational_max),
        },
        SimpleFn {
            name: "sqrt",
            arity: Arity::exact(1),
            category: Category::Root,
            description: "square root (exact for perfect squares)",
            always_exact: false,
            fixed: fixed_sqrt,
            rational: None,
        },
        SimpleFn {
            name: "hypot",
            arity: Arity::exact(2),
            category: Category::Root,
            description: "square root of the sum of two squares",
            always_exact: false,
            fixed: fixed_hypot,
            rational: None,
        },
        SimpleFn {
            name: "exp",
            arity: Arity::exact(1),
            category: Category::Exponential,
            description: "e raised to the argument",
            always_exact: false,
            fixed: fixed_exp,
            rational: None,
        },
        SimpleFn {
            name: "ln",
            arity: Arity::exact(1),
            category: Category::Exponential,
            description: "natural logarithm",
            always_exact: false,
            fixed: fixed_ln,
            rational: None,
        },
        SimpleFn {
            name: "log",
            arity: Arity::range(1, 2),
            category: Category::Exponential,
            description: "logarithm base twelve, or log(base, x)",
            always_exact: false,
            fixed: fixed_log,
            rational: None,
        },
        SimpleFn {
            name: "sin",
            arity: Arity::exact(1),
            category: Category::Trigonometry,
            description: "sine, argument in radians",
            always_exact: false,
            fixed: fixed_sin,
            rational: None,
        },
        SimpleFn {
            name: "cos",
            arity: Arity::exact(1),
            category: Category::Trigonometry,
            description: "cosine, argument in radians",
            always_exact: false,
            fixed: fixed_cos,
            rational: None,
        },
        SimpleFn {
            name: "tan",
            arity: Arity::exact(1),
            category: Category::Trigonometry,
            description: "tangent, argument in radians",
            always_exact: false,
            fixed: fixed_tan,
            rational: None,
        },
        SimpleFn {
            name: "asin",
            arity: Arity::exact(1),
            category: Category::Trigonometry,
            description: "arcsine, result in radians",
            always_exact: false,
            fixed: fixed_asin,
            rational: None,
        },
        SimpleFn {
            name: "acos",
            arity: Arity::exact(1),
            category: Category::Trigonometry,
            description: "arccosine, result in radians",
            always_exact: false,
            fixed: fixed_acos,
            rational: None,
        },
        SimpleFn {
            name: "atan",
            arity: Arity::exact(1),
            category: Category::Trigonometry,
            description: "arctangent, result in radians",
            always_exact: false,
            fixed: fixed_atan,
            rational: None,
        },
    ];
    for entry in entries {
        registry.register(Box::new(entry));
    }
}
