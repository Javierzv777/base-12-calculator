//! Trigonometric functions and their inverses.
//!
//! `sin`/`cos` reduce the angle modulo `2*pi` and then sum the usual
//! alternating Taylor series; `tan` is their quotient. `atan` reduces with
//! the half-angle identity `atan(z) = 2*atan(z / (1 + sqrt(1 + z^2)))`
//! until the argument is small enough for its own series to converge in a
//! dozen terms, and `asin`/`acos` are expressed through `atan`.
//!
//! Accuracy note, the same one every fixed-precision implementation
//! carries: reducing a *large* angle modulo `2*pi` costs significant
//! digits, because the reduction subtracts two nearby large numbers. The
//! reduction here is done at `scale + GUARD_DIGITS` digits, so angles of
//! ordinary size are fine and enormous ones degrade gracefully rather than
//! being rejected.

use crate::constants::{half_pi, pi, two_pi};
use crate::elementary::sqrt;
use crate::util::{abs, cmp, div, double_n_times, is_negative, one, round_to_scale, truncate};
use crate::{SciError, GUARD_DIGITS};
use dz_gates::{Digit, DozGates};
use dz_num::DzFixed;
use std::cmp::Ordering;

const MAX_TERMS: usize = 400;

/// Bring an angle into `[-pi, pi]` by subtracting a whole number of turns.
fn reduce_angle<G: DozGates>(g: &G, x: &DzFixed, gs: usize) -> Result<DzFixed, SciError> {
    let turn = two_pi(gs);
    let p = pi(gs);
    let turns = div(g, x, &turn, gs)?.trunc();
    let mut r = x.sub(g, &turns.mul(g, &turn));
    if cmp(g, &r, &p) == Ordering::Greater {
        r = r.sub(g, &turn);
    }
    if cmp(g, &r, &p.neg()) == Ordering::Less {
        r = r.add(g, &turn);
    }
    Ok(truncate(&r, gs))
}

/// Sine, from the series `x - x^3/3! + x^5/5! - ...`.
///
/// The factorials are never formed: each term is the previous one times
/// `x^2` divided by the next pair of consecutive integers, and that pair
/// is produced by a dozenal counter stepping by one, not by computing
/// `2k(2k+1)` in native arithmetic.
pub fn sin<G: DozGates>(g: &G, x: &DzFixed, scale: usize) -> Result<DzFixed, SciError> {
    let gs = scale + GUARD_DIGITS;
    let angle = reduce_angle(g, x, gs)?;
    let unit = one();
    let squared = truncate(&angle.mul(g, &angle), gs);
    let mut term = angle.clone();
    let mut sum = angle;
    let mut counter = unit.clone(); // 1, then 3, then 5, ...
    let mut subtract = true;
    let mut steps = 0usize;
    loop {
        let first = counter.add(g, &unit);
        let second = first.add(g, &unit);
        counter = second.clone();
        let denominator = first.mul(g, &second);
        term = truncate(&term.mul(g, &squared), gs);
        term = div(g, &term, &denominator, gs)?;
        if term.is_zero() {
            break;
        }
        sum = if subtract {
            sum.sub(g, &term)
        } else {
            sum.add(g, &term)
        };
        subtract = !subtract;
        steps += 1;
        if steps > MAX_TERMS {
            break;
        }
    }
    Ok(round_to_scale(g, &sum, scale))
}

/// Cosine, from `1 - x^2/2! + x^4/4! - ...`, built the same way as `sin`.
pub fn cos<G: DozGates>(g: &G, x: &DzFixed, scale: usize) -> Result<DzFixed, SciError> {
    let gs = scale + GUARD_DIGITS;
    let angle = reduce_angle(g, x, gs)?;
    let unit = one();
    let squared = truncate(&angle.mul(g, &angle), gs);
    let mut term = unit.clone();
    let mut sum = unit.clone();
    let mut counter = DzFixed::zero(); // 0, then 2, then 4, ...
    let mut subtract = true;
    let mut steps = 0usize;
    loop {
        let first = counter.add(g, &unit);
        let second = first.add(g, &unit);
        counter = second.clone();
        let denominator = first.mul(g, &second);
        term = truncate(&term.mul(g, &squared), gs);
        term = div(g, &term, &denominator, gs)?;
        if term.is_zero() {
            break;
        }
        sum = if subtract {
            sum.sub(g, &term)
        } else {
            sum.add(g, &term)
        };
        subtract = !subtract;
        steps += 1;
        if steps > MAX_TERMS {
            break;
        }
    }
    Ok(round_to_scale(g, &sum, scale))
}

/// Tangent, as `sin/cos`. A cosine that truncates to zero at the working
/// precision reports division by zero, which is the honest answer for an
/// odd multiple of a right angle at finite precision.
pub fn tan<G: DozGates>(g: &G, x: &DzFixed, scale: usize) -> Result<DzFixed, SciError> {
    let gs = scale + GUARD_DIGITS;
    let s = sin(g, x, gs)?;
    let c = cos(g, x, gs)?;
    if c.is_zero() {
        return Err(SciError::DivisionByZero);
    }
    Ok(round_to_scale(g, &div(g, &s, &c, gs)?, scale))
}

/// `atan(z)` for `0 <= z <= 1`, after halving the argument enough times
/// that the series converges quickly.
fn atan_core<G: DozGates>(g: &G, z0: &DzFixed, gs: usize) -> Result<DzFixed, SciError> {
    let unit = one();
    // `0;3` is exactly one quarter: another place where base twelve makes
    // a threshold an exact one-digit literal.
    let quarter = DzFixed::from_digits(
        false,
        &[Digit::ZERO],
        &[Digit::new(3).expect("3 is a dozenal digit")],
    );
    let mut z = z0.clone();
    let mut halvings = 0usize;
    while cmp(g, &z, &quarter) == Ordering::Greater && halvings < 8 {
        let (root, _exact) = sqrt(g, &unit.add(g, &z.mul(g, &z)), gs)?;
        z = div(g, &z, &unit.add(g, &root), gs)?;
        halvings += 1;
    }

    let squared = truncate(&z.mul(g, &z), gs);
    let mut term = z.clone();
    let mut sum = z;
    let two = unit.add(g, &unit);
    let mut denominator = unit.clone(); // 1, then 3, 5, 7, ...
    let mut subtract = true;
    let mut steps = 0usize;
    loop {
        term = truncate(&term.mul(g, &squared), gs);
        denominator = denominator.add(g, &two);
        if term.is_zero() {
            break;
        }
        let piece = div(g, &term, &denominator, gs)?;
        if piece.is_zero() {
            break;
        }
        sum = if subtract {
            sum.sub(g, &piece)
        } else {
            sum.add(g, &piece)
        };
        subtract = !subtract;
        steps += 1;
        if steps > MAX_TERMS {
            break;
        }
    }
    Ok(double_n_times(g, &sum, halvings))
}

/// Arctangent over the whole real line: arguments above one are folded
/// with `atan(z) = pi/2 - atan(1/z)`, so the series always runs on a value
/// at most one.
pub fn atan<G: DozGates>(g: &G, x: &DzFixed, scale: usize) -> Result<DzFixed, SciError> {
    let gs = scale + GUARD_DIGITS;
    let negative = is_negative(x);
    let magnitude = abs(x);
    let unit = one();
    let folded = cmp(g, &magnitude, &unit) == Ordering::Greater;
    let argument = if folded {
        div(g, &unit, &magnitude, gs)?
    } else {
        magnitude
    };
    let core = atan_core(g, &argument, gs)?;
    let value = if folded {
        half_pi(gs).sub(g, &core)
    } else {
        core
    };
    let signed = if negative { value.neg() } else { value };
    Ok(round_to_scale(g, &signed, scale))
}

/// Arcsine, as `atan(x / sqrt(1 - x^2))`, with the endpoints answered
/// directly (where that quotient would divide by zero).
pub fn asin<G: DozGates>(g: &G, x: &DzFixed, scale: usize) -> Result<DzFixed, SciError> {
    let gs = scale + GUARD_DIGITS;
    let unit = one();
    let magnitude = abs(x);
    match cmp(g, &magnitude, &unit) {
        Ordering::Greater => Err(SciError::Domain("arcsine outside [-1, 1]")),
        Ordering::Equal => {
            let quarter_turn = half_pi(scale);
            Ok(if is_negative(x) {
                quarter_turn.neg()
            } else {
                quarter_turn
            })
        }
        Ordering::Less => {
            let (root, _exact) = sqrt(g, &unit.sub(g, &x.mul(g, x)), gs)?;
            let ratio = div(g, x, &root, gs)?;
            Ok(round_to_scale(g, &atan(g, &ratio, gs)?, scale))
        }
    }
}

/// Arccosine, as `pi/2 - asin(x)`.
pub fn acos<G: DozGates>(g: &G, x: &DzFixed, scale: usize) -> Result<DzFixed, SciError> {
    let gs = scale + GUARD_DIGITS;
    let value = half_pi(gs).sub(g, &asin(g, x, gs)?);
    Ok(round_to_scale(g, &value, scale))
}
