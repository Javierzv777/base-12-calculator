//! Rounding and sign functions, which -- unlike everything else in this
//! crate -- are *exact*: they answer with a value the caller's own type can
//! represent perfectly, so the evaluator does not have to mark their
//! results as approximations.
//!
//! Each one comes in two flavours, fixed-point and rational, because the
//! calculator's rational mode must not lose exactness just because the
//! user asked for a floor.
//!
//! Rounding is half-to-even in both flavours, matching the DFP rounding
//! rule in `docs/03-formato-dfp.md`, and the tie is detected the same way
//! it is there: by comparing twice the remainder against the divisor,
//! never by dividing.

use crate::util::{half, one};
use dz_gates::DozGates;
use dz_int::{Dz, Sign};
use dz_num::{DzFixed, DzRational};
use std::cmp::Ordering;

fn dz_one() -> Dz {
    Dz::from_magnitude(false, vec![dz_gates::Digit::ONE])
}

fn abs_dz(n: &Dz) -> Dz {
    if n.sign() == Sign::Neg {
        n.negate()
    } else {
        n.clone()
    }
}

pub fn abs_fixed(v: &DzFixed) -> DzFixed {
    if v.is_negative() {
        v.neg()
    } else {
        v.clone()
    }
}

/// `-1`, `0` or `1`.
pub fn sign_fixed(v: &DzFixed) -> DzFixed {
    if v.is_zero() {
        DzFixed::zero()
    } else if v.is_negative() {
        one().neg()
    } else {
        one()
    }
}

/// Toward zero.
pub fn trunc_fixed(v: &DzFixed) -> DzFixed {
    v.trunc()
}

/// Toward negative infinity.
pub fn floor_fixed<G: DozGates>(g: &G, v: &DzFixed) -> DzFixed {
    let whole = v.trunc();
    if v.is_negative() && v.compare(g, &whole) != Ordering::Equal {
        whole.sub(g, &one())
    } else {
        whole
    }
}

/// Toward positive infinity.
pub fn ceil_fixed<G: DozGates>(g: &G, v: &DzFixed) -> DzFixed {
    let whole = v.trunc();
    if !v.is_negative() && v.compare(g, &whole) != Ordering::Equal {
        whole.add(g, &one())
    } else {
        whole
    }
}

/// Nearest integer, ties to even.
pub fn round_fixed<G: DozGates>(g: &G, v: &DzFixed) -> DzFixed {
    let whole = v.trunc();
    let fraction = abs_fixed(&v.sub(g, &whole));
    let away = match fraction.compare(g, &half()) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => !whole.magnitude.is_even(g),
    };
    if !away {
        return whole;
    }
    if v.is_negative() {
        whole.sub(g, &one())
    } else {
        whole.add(g, &one())
    }
}

pub fn abs_rational(v: &DzRational) -> DzRational {
    if v.is_negative() {
        v.neg()
    } else {
        v.clone()
    }
}

pub fn sign_rational(v: &DzRational) -> DzRational {
    if v.is_zero() {
        DzRational::zero()
    } else if v.is_negative() {
        DzRational::from_int(dz_one().negate())
    } else {
        DzRational::from_int(dz_one())
    }
}

/// Quotient and remainder of `num/den`, both exact. The denominator of a
/// canonical `DzRational` is never zero, so this cannot fail.
fn split(g: &impl DozGates, v: &DzRational) -> (Dz, Dz) {
    v.num
        .div_rem(g, &v.den)
        .expect("a canonical rational never has a zero denominator")
}

pub fn trunc_rational<G: DozGates>(g: &G, v: &DzRational) -> DzRational {
    let (whole, _remainder) = split(g, v);
    DzRational::from_int(whole)
}

pub fn floor_rational<G: DozGates>(g: &G, v: &DzRational) -> DzRational {
    let (whole, remainder) = split(g, v);
    if v.is_negative() && !remainder.is_zero() {
        DzRational::from_int(whole.sub(g, &dz_one()))
    } else {
        DzRational::from_int(whole)
    }
}

pub fn ceil_rational<G: DozGates>(g: &G, v: &DzRational) -> DzRational {
    let (whole, remainder) = split(g, v);
    if !v.is_negative() && !remainder.is_zero() {
        DzRational::from_int(whole.add(g, &dz_one()))
    } else {
        DzRational::from_int(whole)
    }
}

/// Nearest integer, ties to even -- decided by comparing `2*|remainder|`
/// against the denominator, so no second division is needed and the tie is
/// detected exactly.
pub fn round_rational<G: DozGates>(g: &G, v: &DzRational) -> DzRational {
    let (whole, remainder) = split(g, v);
    let magnitude = abs_dz(&remainder);
    let doubled = magnitude.add(g, &magnitude);
    let away = match doubled.compare(g, &v.den) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => !whole.is_even(g),
    };
    if !away {
        return DzRational::from_int(whole);
    }
    if v.is_negative() {
        DzRational::from_int(whole.sub(g, &dz_one()))
    } else {
        DzRational::from_int(whole.add(g, &dz_one()))
    }
}
