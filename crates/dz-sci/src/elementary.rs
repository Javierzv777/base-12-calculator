//! Square root, exponential, logarithms.
//!
//! All four share the same shape: reduce the argument into a small range
//! where a series (or Newton's iteration) converges quickly, run the
//! series at `scale + GUARD_DIGITS` fractional dozenal digits, then undo
//! the reduction. The reductions are chosen so the undo step is cheap in
//! base 12: halving is a multiplication by the exact literal `0;6`, and
//! scaling by a power of twelve is `scale` bookkeeping.

use crate::constants;
use crate::util::{
    abs, cmp, div, double_n_times, half, int_digit_count, is_negative, leading_fraction_zeros,
    one, round_to_scale, small_int, truncate, ulp, zero, Counter,
};
use crate::{SciError, GUARD_DIGITS};
use dz_gates::{Digit, DozGates};
use dz_num::DzFixed;
use std::cmp::Ordering;

/// Iteration/term ceiling. It is never reached for sane arguments; it
/// exists so a pathological input cannot spin forever inside a GUI.
const MAX_ITERATIONS: usize = 400;

/// Square root by Newton's iteration `y <- (y + x/y)/2`, with the initial
/// guess taken from the *digit count* of `x`, so the iteration starts
/// within a factor of twelve of the answer instead of walking down from an
/// arbitrary constant.
///
/// Returns the value and whether it is exact: a perfect square is verified
/// by squaring the truncated answer back, so `sqrt(9)` reports `3` as
/// exact rather than as an approximation.
pub fn sqrt<G: DozGates>(g: &G, x: &DzFixed, scale: usize) -> Result<(DzFixed, bool), SciError> {
    if is_negative(x) {
        return Err(SciError::Domain("square root of a negative number"));
    }
    if x.is_zero() {
        return Ok((zero(), true));
    }
    let gs = scale + GUARD_DIGITS;

    // Initial guess: 12^ceil(m/2) when the value is at least one (m =
    // integer digits), or 12^-floor(j/2) when it is below one (j = leading
    // fractional zeros). Both are upper bounds on the root, which is what
    // Newton's iteration wants -- from above it decreases monotonically.
    let integer_digits = int_digit_count(x);
    let mut y = if integer_digits > 0 {
        let mut halves = 0usize;
        let mut i = 0usize;
        while i + 2 <= integer_digits {
            halves += 1;
            i += 2;
        }
        DzFixed::pow12(halves + 1)
    } else {
        let zeros = leading_fraction_zeros(x);
        let mut halves = 0usize;
        let mut i = 0usize;
        while i + 2 <= zeros {
            halves += 1;
            i += 2;
        }
        ulp(halves)
    };

    let h = half();
    let mut steps = 0usize;
    loop {
        let quotient = div(g, x, &y, gs)?;
        let next = truncate(&y.add(g, &quotient).mul(g, &h), gs);
        if next.is_zero() {
            break;
        }
        // Truncating division makes the sequence non-increasing only until
        // it settles; the first time it stops decreasing, `y` is the answer.
        if cmp(g, &next, &y) != Ordering::Less {
            break;
        }
        y = next;
        steps += 1;
        if steps > MAX_ITERATIONS {
            break;
        }
    }

    let result = round_to_scale(g, &y, scale);
    let exact = cmp(g, &result.mul(g, &result), x) == Ordering::Equal;
    Ok((result, exact))
}

/// `e^x`, by halving the argument until it is at most one half, summing
/// the Taylor series there, then squaring the result back the same number
/// of times.
///
/// Negative arguments are computed as the reciprocal of the positive case:
/// the series for a negative argument alternates and loses digits to
/// cancellation, while the reciprocal of a well-conditioned positive
/// result does not.
pub fn exp<G: DozGates>(g: &G, x: &DzFixed, scale: usize) -> Result<DzFixed, SciError> {
    if x.is_zero() {
        return Ok(one());
    }
    // `e^144` already needs about fifty integer digits; past that the
    // answer is not something a calculator display can use.
    let limit = small_int(g, 144);
    if cmp(g, &abs(x), &limit) == Ordering::Greater {
        return Err(SciError::Overflow);
    }
    if is_negative(x) {
        let positive = exp(g, &x.neg(), scale + GUARD_DIGITS)?;
        if positive.is_zero() {
            return Err(SciError::Overflow);
        }
        let reciprocal = div(g, &one(), &positive, scale + GUARD_DIGITS)?;
        return Ok(round_to_scale(g, &reciprocal, scale));
    }

    let h = half();
    let mut reduced = x.clone();
    let mut squarings = 0usize;
    while cmp(g, &reduced, &h) == Ordering::Greater && squarings < 64 {
        reduced = reduced.mul(g, &h);
        squarings += 1;
    }
    // Each squaring doubles the accumulated truncation error, so the
    // working precision carries one extra digit per squaring.
    let gs = scale + GUARD_DIGITS + squarings;
    reduced = truncate(&reduced, gs);

    let mut term = one();
    let mut sum = one();
    let mut n = Counter::new();
    let mut steps = 0usize;
    loop {
        let k = n.next(g); // 1, 2, 3, ... as a dozenal counter
        term = truncate(&term.mul(g, &reduced), gs);
        term = div(g, &term, &k, gs)?;
        if term.is_zero() {
            break;
        }
        sum = sum.add(g, &term);
        steps += 1;
        if steps > MAX_ITERATIONS {
            break;
        }
    }

    let mut out = sum;
    let mut i = 0usize;
    while i < squarings {
        out = truncate(&out.mul(g, &out), gs);
        i += 1;
    }
    Ok(round_to_scale(g, &out, scale))
}

/// Split a positive `x` into `m * 12^(±e)` with `m` in `[1, 12)`, by
/// moving the Humphrey point -- `scale` bookkeeping only, no gate work.
/// Returns `(m, e, e_is_negative)`.
fn reduce_to_leading_digit(x: &DzFixed) -> (DzFixed, usize, bool) {
    let integer_digits = int_digit_count(x);
    if integer_digits > 1 {
        let e = integer_digits - 1;
        (x.shift_point_right(e), e, false)
    } else if integer_digits == 1 {
        (x.clone(), 0, false)
    } else {
        let e = leading_fraction_zeros(x) + 1;
        let m = DzFixed {
            magnitude: x.magnitude.clone(),
            scale: x.scale - e,
        };
        (m, e, true)
    }
}

/// `ln(m)` for `m` in `[1, 12]` -- the part of the logarithm that actually
/// needs a series.
///
/// Two reductions, in this order:
///   1. repeated square roots until the argument drops below `1;3` (about
///      1.25), using `ln(m) = 2^t * ln(m^(1/2^t))`. Three or four roots
///      cover the whole `[1, 12)` range, and each root is a handful of
///      Newton steps -- far cheaper than the hundred-odd extra series
///      terms the un-reduced argument would need;
///   2. the `atanh` form `ln(z) = 2*(u + u^3/3 + u^5/5 + ...)` with
///      `u = (z-1)/(z+1)`, whose terms then shrink by a factor of about
///      `u^2` (under a tenth) each step.
fn ln_core<G: DozGates>(g: &G, m: &DzFixed, gs: usize) -> Result<DzFixed, SciError> {
    let threshold = DzFixed::from_digits(
        false,
        &[Digit::ONE],
        &[Digit::new(3).expect("3 is a dozenal digit")],
    );
    let mut z = m.clone();
    let mut roots = 0usize;
    while cmp(g, &z, &threshold) == Ordering::Greater && roots < 8 {
        let (root, _exact) = sqrt(g, &z, gs)?;
        z = root;
        roots += 1;
    }

    let unit = one();
    let u = div(g, &z.sub(g, &unit), &z.add(g, &unit), gs)?;
    if u.is_zero() {
        return Ok(zero());
    }
    let u_squared = truncate(&u.mul(g, &u), gs);
    let mut term = u.clone();
    let mut sum = u;
    let two = unit.add(g, &unit);
    let mut denominator = unit.clone(); // steps to 3, 5, 7, ...
    let mut steps = 0usize;
    loop {
        term = truncate(&term.mul(g, &u_squared), gs);
        denominator = denominator.add(g, &two);
        if term.is_zero() {
            break;
        }
        let piece = div(g, &term, &denominator, gs)?;
        if piece.is_zero() {
            break;
        }
        sum = sum.add(g, &piece);
        steps += 1;
        if steps > MAX_ITERATIONS {
            break;
        }
    }
    // ln(z) = 2*sum, and ln(m) = 2^roots * ln(z).
    Ok(double_n_times(g, &sum, roots + 1))
}

/// Natural logarithm. `x = m * 12^(±e)` with `m` in `[1,12)`, so
/// `ln(x) = ln(m) ± e*ln(12)`: the radix split is exact and free, and only
/// `ln(m)` costs a series.
pub fn ln<G: DozGates>(g: &G, x: &DzFixed, scale: usize) -> Result<DzFixed, SciError> {
    if x.is_zero() || is_negative(x) {
        return Err(SciError::Domain("logarithm of a non-positive number"));
    }
    let gs = scale + GUARD_DIGITS;
    let (m, e, e_negative) = reduce_to_leading_digit(x);
    let core = ln_core(g, &m, gs)?;
    if e == 0 {
        return Ok(round_to_scale(g, &core, scale));
    }
    let shift = small_int(g, e).mul(g, &constants::ln12(gs));
    let total = if e_negative {
        core.sub(g, &shift)
    } else {
        core.add(g, &shift)
    };
    Ok(round_to_scale(g, &total, scale))
}

/// `x` as an exact power of twelve, if it is one: the exponent, and
/// whether it is negative. A normalized magnitude of a single `1` digit is
/// the whole test -- `Vec` inspection against the ONE symbol, not
/// arithmetic.
fn power_of_twelve(x: &DzFixed) -> Option<(usize, bool)> {
    if x.is_negative() {
        return None;
    }
    let digits = x.magnitude_digits();
    let len = digits.len();
    if digits[len - 1] != Digit::ONE {
        return None;
    }
    let mut i = 0usize;
    while i + 1 < len {
        if digits[i] != Digit::ZERO {
            return None;
        }
        i += 1;
    }
    let top = len - 1;
    if top >= x.scale {
        Some((top - x.scale, false))
    } else {
        Some((x.scale - top, true))
    }
}

/// Base-12 logarithm -- the "log" a dozenal calculator wants, the way a
/// base-10 calculator's `log` key means base ten.
///
/// Exact powers of twelve are answered exactly (`log(100) = 2`, since
/// `100` dozenal is a gross): the series path would otherwise return
/// `1;EEE...` for them, correct to the digit but reading as a bug.
pub fn log12<G: DozGates>(g: &G, x: &DzFixed, scale: usize) -> Result<(DzFixed, bool), SciError> {
    if x.is_zero() || is_negative(x) {
        return Err(SciError::Domain("logarithm of a non-positive number"));
    }
    if let Some((exponent, negative)) = power_of_twelve(x) {
        let value = small_int(g, exponent);
        return Ok((if negative { value.neg() } else { value }, true));
    }
    let gs = scale + GUARD_DIGITS;
    let numerator = ln(g, x, gs)?;
    let value = div(g, &numerator, &constants::ln12(gs), gs)?;
    Ok((round_to_scale(g, &value, scale), false))
}

/// `log_base(b, x) = ln(x) / ln(b)`.
pub fn log_base<G: DozGates>(
    g: &G,
    base: &DzFixed,
    x: &DzFixed,
    scale: usize,
) -> Result<DzFixed, SciError> {
    let gs = scale + GUARD_DIGITS;
    let denominator = ln(g, base, gs)?;
    if denominator.is_zero() {
        return Err(SciError::Domain("logarithm base one"));
    }
    let numerator = ln(g, x, gs)?;
    Ok(round_to_scale(g, &div(g, &numerator, &denominator, gs)?, scale))
}
