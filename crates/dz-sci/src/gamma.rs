//! The Gamma function, by argument-reduction recurrence plus Stirling's
//! asymptotic series -- the same shape as `elementary::ln`'s radix
//! reduction and `trig::atan`'s half-angle reduction: push the argument
//! into a range where a series converges fast, run the series, undo the
//! push.
//!
//! # Why this algorithm and not a Lanczos-style coefficient table
//!
//! CLAUDE.md's purity rule forbids computing this with `f64` and only
//! formatting the result in dozenal -- that would import Gamma's *meaning*
//! from native arithmetic instead of deriving it from the crate's own
//! primitives, exactly what the project exists to avoid (see the commit
//! `18565f8` note in CLAUDE.md). A Lanczos approximation would technically
//! satisfy "no native operator touches a `Dz`" if its magic coefficients
//! were typed in as `DzFixed` literals, but those coefficients are fitted
//! numerically against `f64` machinery this project doesn't have and
//! doesn't want to reimplement; they would not be *derived* from anything
//! in this crate, just imported wholesale. Stirling's series, in contrast,
//! is a textbook asymptotic expansion in the Bernoulli numbers -- exact
//! rationals, the same category of "mathematical constant typed in as a
//! literal" the rest of this crate already uses for factorial denominators
//! and the `1/6`, `1/30`, ... coefficients are no different in kind from
//! the `1/2`, `1/3` this crate already builds via `small_int`/`div`.
//!
//! # The recurrence
//!
//! `Gamma(x) = Gamma(x+n) / (x * (x+1) * ... * (x+n-1))`, so pushing `x` up
//! by whole steps until it clears a threshold and dividing back out is
//! exact bookkeeping (the product is gate-composed multiplication of exact
//! values, not an approximation). Only `Gamma` of the pushed-up argument
//! needs a series, and Stirling's converges geometrically once the
//! argument is comfortably above the threshold -- 12 was picked because it
//! is this project's own radix, not because it is special to Stirling.
//!
//! `ln Gamma(z) = (z - 1/2)*ln(z) - z + (1/2)*ln(2*pi)
//!                + sum_{k=1}^{n} B_2k / (2k*(2k-1)*z^(2k-1))`
//!
//! computed in log space and undone with `exp`, exactly as `ln`/`exp`
//! already cooperate elsewhere in this crate (`constants::ln12`).

use crate::constants;
use crate::elementary::{exp, ln};
use crate::util::{cmp, div, half, one, round_to_scale, small_int, truncate};
use crate::{SciError, GUARD_DIGITS};
use dz_gates::DozGates;
use dz_num::DzFixed;
use std::cmp::Ordering;

/// How far the recurrence pushes the argument before switching to the
/// series. Chosen to be this project's own radix: not a property of
/// Stirling's series (any value from about 6 up works), just a dozenal
/// number that keeps the reduction loop's step count legible.
const REDUCTION_THRESHOLD: usize = 12;

/// Safety valve, mirrors `elementary::MAX_ITERATIONS` -- never reached for
/// sane arguments, exists so a pathological input cannot spin forever.
const MAX_REDUCTION_STEPS: usize = 4000;

/// `B_2k / (2k*(2k-1))` for `k = 1..=5`, i.e. the coefficients Stirling's
/// series actually multiplies against `z^-(2k-1)`, pre-divided by the
/// `2k*(2k-1)` the series formula carries. The Bernoulli numbers
/// (`B_2=1/6, B_4=-1/30, B_6=1/42, B_8=-1/30, B_10=5/66`) are exact
/// mathematical constants, the same category as the `1/2`, `1/3` this
/// crate already types in as literals -- not a fitted/imported table.
/// `(negative, numerator, denominator)`.
const STIRLING_COEFFICIENTS: [(bool, usize, usize); 5] = [
    (false, 1, 12),    // B2/(2*1)   =  (1/6)/2   =  1/12
    (true, 1, 360),    // B4/(4*3)   = (-1/30)/12 = -1/360
    (false, 1, 1260),  // B6/(6*5)   =  (1/42)/30 =  1/1260
    (true, 1, 1680),   // B8/(8*7)   = (-1/30)/56 = -1/1680
    (false, 1, 1188),  // B10/(10*9) =  (5/66)/90 =  1/1188
];

/// `ln Gamma(z)` for `z` already pushed above `REDUCTION_THRESHOLD`, by
/// Stirling's series.
fn ln_gamma_stirling<G: DozGates>(g: &G, z: &DzFixed, gs: usize) -> Result<DzFixed, SciError> {
    let ln_z = ln(g, z, gs)?;
    let h = half();
    let z_minus_half = z.sub(g, &h);
    let mut acc = z_minus_half.mul(g, &ln_z).sub(g, z);

    // + (1/2) * ln(2*pi), both factors already derived elsewhere in this
    // crate: pi from Machin's formula, 2 from a gate-composed counter.
    let two = small_int(g, 2);
    let ln_two = ln(g, &two, gs)?;
    let pi_value = constants::pi(gs);
    let ln_pi = ln(g, &pi_value, gs)?;
    let ln_two_pi = ln_two.add(g, &ln_pi);
    acc = acc.add(g, &ln_two_pi.mul(g, &h));

    // Bernoulli series: power starts at z^-1 and advances by z^-2 each
    // term, so z^-(2k-1) is built by repeated gate-composed multiplication
    // and division, never by a native exponent on the digit value.
    let one_v = one();
    let z_squared_inv = div(g, &one_v, &z.mul(g, z), gs)?;
    let mut power = div(g, &one_v, z, gs)?;
    for (negative, numerator, denominator) in STIRLING_COEFFICIENTS {
        let coefficient = div(g, &small_int(g, numerator), &small_int(g, denominator), gs)?;
        let term = coefficient.mul(g, &power);
        acc = if negative { acc.sub(g, &term) } else { acc.add(g, &term) };
        power = truncate(&power.mul(g, &z_squared_inv), gs);
    }
    Ok(acc)
}

/// The Gamma function, `Gamma(x) = integral_0^inf t^(x-1) e^-t dt`
/// extended off the integers -- `Gamma(n) = (n-1)!` for positive integers,
/// `Gamma(1/2) = sqrt(pi)`.
///
/// Only positive `x` is supported: the reflection formula that extends
/// this to negative non-integer arguments is future work, not something
/// this call silently gets wrong (`SciError::Domain` at zero and below).
/// The result is always reported approximate, like `exp`/`ln`/the
/// trigonometric functions -- Stirling's series never terminates, so
/// unlike `sqrt` there is no cheap structural check for an exact case
/// worth adding.
pub fn gamma<G: DozGates>(g: &G, x: &DzFixed, scale: usize) -> Result<DzFixed, SciError> {
    if x.is_zero() || x.is_negative() {
        return Err(SciError::Domain("gamma of a non-positive number"));
    }
    let gs = scale + GUARD_DIGITS;
    let threshold = small_int(g, REDUCTION_THRESHOLD);

    // Gamma(x) = Gamma(x+n) / (x*(x+1)*...*(x+n-1)): push x up by whole
    // steps until it clears the threshold, accumulating the exact product
    // that undoes the push.
    let one_v = one();
    let mut xr = x.clone();
    let mut product = one_v.clone();
    let mut steps = 0usize;
    while cmp(g, &xr, &threshold) == Ordering::Less {
        product = product.mul(g, &xr);
        xr = xr.add(g, &one_v);
        steps += 1;
        if steps > MAX_REDUCTION_STEPS {
            return Err(SciError::Overflow);
        }
    }

    let ln_gamma_xr = ln_gamma_stirling(g, &xr, gs)?;
    let gamma_xr = exp(g, &ln_gamma_xr, gs)?;
    let value = div(g, &gamma_xr, &product, gs)?;
    Ok(round_to_scale(g, &value, scale))
}
