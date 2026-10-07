//! Number-theoretic helpers on `Dz`: exact `p`-adic valuation and
//! divisibility, with the last-digit rule for divisors of twelve.
//!
//! Everything here composes `Dz::div_rem`, `Dz::add`, `Dz::compare` and the
//! one-digit decoder of `dz_gates::circuits::divisibility`. No dozenal
//! value is ever converted to a native integer: the exponent returned by
//! [`valuation`] is a `Dz` counter advanced by gate-composed `add`, exactly
//! like the series counters in `dz-sci`.

use crate::dzint::{Dz, Sign};
use dz_gates::circuits::{
    depth_tracking, divisor_row, flag_to_bool, last_digit_divisible_net, record_level, GateNet,
    Levels, Values, SMALL_DIVISOR_TAGS,
};
use dz_gates::{Digit, DozGates};
use std::cmp::Ordering;
use std::fmt;

fn two() -> Dz {
    Dz::from_magnitude(false, vec![Digit::new(2).expect("two is a dozenal digit")])
}

fn one() -> Dz {
    Dz::from_magnitude(false, vec![Digit::ONE])
}

fn magnitude_of(n: &Dz) -> Dz {
    if n.sign() == Sign::Neg {
        n.negate()
    } else {
        n.clone()
    }
}

/// Why a valuation cannot be defined.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValuationError {
    /// `v_p(0)` is infinite (every power of `p` divides zero).
    ZeroNumber,
    /// `p` must be at least two: `1^k` divides everything (no largest
    /// `k`), and zero and negatives are not a base.
    DivisorTooSmall,
}

impl fmt::Display for ValuationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValuationError::ZeroNumber => {
                write!(f, "valuation of zero is undefined (every power divides zero)")
            }
            ValuationError::DivisorTooSmall => {
                write!(f, "valuation needs a divisor of at least two")
            }
        }
    }
}
impl std::error::Error for ValuationError {}

/// `n = p^exponent * cofactor`, with `p` not dividing `cofactor`.
#[derive(Clone, Debug)]
pub struct Valuation {
    pub exponent: Dz,
    /// `|n|` with every factor `p` removed.
    pub cofactor: Dz,
}

/// The exact `p`-adic valuation of `n`: the largest `k` with `p^k | n`,
/// found by repeated `div_rem` until the remainder is nonzero.
///
/// The sign of `n` is ignored (`v_p(-n) = v_p(n)`). `n = 0` and `p < 2` are
/// errors. It always terminates: each pass strictly shrinks a positive
/// magnitude because `p >= 2`. One failing `div_rem` is always paid (the
/// one that proves the exponent is maximal).
pub fn valuation<G: DozGates>(g: &G, n: &Dz, p: &Dz) -> Result<Valuation, ValuationError> {
    if p.compare(g, &two()) == Ordering::Less {
        return Err(ValuationError::DivisorTooSmall);
    }
    if n.is_zero() {
        return Err(ValuationError::ZeroNumber);
    }
    let one = one();
    let mut exponent = Dz::zero();
    let mut rest = magnitude_of(n);
    loop {
        let (quotient, remainder) = rest.div_rem(g, p).expect("p is at least two");
        if !remainder.is_zero() {
            break;
        }
        rest = quotient;
        exponent = exponent.add(g, &one);
    }
    Ok(Valuation { exponent, cofactor: rest })
}

/// How a divisibility question was answered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DivisibilityMethod {
    /// The one-digit decoder over the last digit (`d` divides twelve).
    LastDigit,
    /// Restoring division (`d` does not divide twelve).
    DivRem,
}

/// Divisibility of `n` by `d`: `d | n`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Divisibility {
    pub divisible: bool,
    pub method: DivisibilityMethod,
}

/// `d = 0` has no divisibility to ask about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZeroDivisor;

impl fmt::Display for ZeroDivisor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "divisibility by zero is undefined")
    }
}
impl std::error::Error for ZeroDivisor {}

fn last_digit_divisible_inner<N: GateNet>(net: &N, n: &Dz, d: &Dz) -> Option<bool> {
    let dm = d.magnitude_digits();
    // Recognize d in {2,3,4,6} (one digit) or twelve (digits 0,1) with
    // gates; the matching decision is control flow on a gate flag, as in
    // `div_rem`. Which divisor it is selects a synthesized row.
    let row = if dm.len() == 1 {
        let d0 = net.lift_at(dm[0], d.level());
        let mut found = None;
        for &c in SMALL_DIVISOR_TAGS.iter() {
            let flag = net.lit(c, d0);
            record_level(N::level(flag));
            if flag_to_bool(N::value(flag)) {
                found = Some(divisor_row(Digit::new(c).expect("c is a dozenal digit")));
                break;
            }
        }
        found?
    } else if dm.len() == 2 {
        let d0 = net.lift_at(dm[0], d.level());
        let d1 = net.lift_at(dm[1], d.level());
        let flag = net.min(net.lit(0, d0), net.lit(1, d1));
        record_level(N::level(flag));
        if flag_to_bool(N::value(flag)) {
            divisor_row(Digit::ZERO)
        } else {
            return None;
        }
    } else {
        return None;
    };
    let lsd = net.lift_at(n.magnitude_digits()[0], n.level());
    let out = last_digit_divisible_net(net, &row, lsd);
    record_level(N::level(out));
    Some(flag_to_bool(N::value(out)))
}

/// Decide `d | n` by the last digit alone, if `d` is one of
/// `{2, 3, 4, 6, 12}`; `None` for any other `d` (including zero), which the
/// last-digit rule cannot answer. Sign is ignored. Generic over `GateNet`
/// internally, so inside a `DepthScope` the depth is measured too.
pub fn last_digit_divisible<G: DozGates>(g: &G, n: &Dz, d: &Dz) -> Option<bool> {
    if depth_tracking() {
        last_digit_divisible_inner(&Levels(g), n, d)
    } else {
        last_digit_divisible_inner(&Values(g), n, d)
    }
}

/// `d | n`, by the last digit when `d` divides twelve and by restoring
/// division otherwise (the fallback is `div_rem`, never a rejection). The
/// answer carries which method ran, so a caller can report it.
pub fn divisible<G: DozGates>(g: &G, n: &Dz, d: &Dz) -> Result<Divisibility, ZeroDivisor> {
    if d.is_zero() {
        return Err(ZeroDivisor);
    }
    if let Some(divisible) = last_digit_divisible(g, n, d) {
        return Ok(Divisibility { divisible, method: DivisibilityMethod::LastDigit });
    }
    let (_, remainder) = n.div_rem(g, d).expect("d is not zero");
    Ok(Divisibility { divisible: remainder.is_zero(), method: DivisibilityMethod::DivRem })
}
