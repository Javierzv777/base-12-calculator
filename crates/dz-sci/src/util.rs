//! Small helpers shared by every algorithm in this crate: literal
//! constructors, sign handling, truncation to a working scale, and the
//! running integer counter that series denominators advance through.
//!
//! Everything here is either `Vec`/`usize` bookkeeping (allowed by
//! PLAN.md Section 0) or a call into `DzFixed`, which is gate-composed.

use crate::SciError;
use dz_gates::{Digit, DozGates};
use dz_int::Dz;
use dz_num::DzFixed;
use std::cmp::Ordering;

pub fn zero() -> DzFixed {
    DzFixed::zero()
}

pub fn one() -> DzFixed {
    DzFixed::from_digits(false, &[Digit::ONE], &[])
}

/// Exactly one half: the dozenal literal `0;6`. Base 12's advantage in
/// miniature -- halving is an exact single-digit multiplication here,
/// where base 10 would need `0.5` and base 2 a shift.
pub fn half() -> DzFixed {
    DzFixed::from_digits(false, &[Digit::ZERO], &[Digit::new(6).expect("6 is a dozenal digit")])
}

/// One unit in the last place at a given scale: `12^-scale`.
pub fn ulp(scale: usize) -> DzFixed {
    DzFixed {
        magnitude: Dz::from_magnitude(false, vec![Digit::ONE]),
        scale,
    }
}

pub fn is_negative(v: &DzFixed) -> bool {
    v.is_negative()
}

pub fn abs(v: &DzFixed) -> DzFixed {
    if v.is_negative() {
        v.neg()
    } else {
        v.clone()
    }
}

pub fn cmp<G: DozGates>(g: &G, a: &DzFixed, b: &DzFixed) -> Ordering {
    a.compare(g, b)
}

/// `a` divided by `b` at `scale` fractional digits, mapping the Layer-1
/// division-by-zero error into this crate's error type.
pub fn div<G: DozGates>(
    g: &G,
    a: &DzFixed,
    b: &DzFixed,
    scale: usize,
) -> Result<DzFixed, SciError> {
    a.div(g, b, scale).map_err(|_| SciError::DivisionByZero)
}

/// Drop fractional digits beyond `scale` (truncating toward zero). Pure
/// `Vec` slicing plus `scale` bookkeeping: no digit's value is inspected.
pub fn truncate(v: &DzFixed, scale: usize) -> DzFixed {
    if v.scale <= scale {
        return v.clone();
    }
    let drop = v.scale - scale;
    let digits = v.magnitude_digits();
    let kept: Vec<Digit> = if digits.len() > drop {
        digits[drop..].to_vec()
    } else {
        vec![Digit::ZERO]
    };
    DzFixed {
        magnitude: Dz::from_magnitude(v.is_negative(), kept),
        scale,
    }
}

/// How many digits the integer part occupies (`0` when `|v| < 1`).
/// `magnitude_digits()` is normalized, so this is exact.
pub fn int_digit_count(v: &DzFixed) -> usize {
    let len = v.magnitude_digits().len();
    len.saturating_sub(v.scale)
}

/// For `0 < |v| < 1`: how many zero digits sit between the Humphrey point
/// and the first significant digit. `12^(count+1) * v` then lands in
/// `[1, 12)`.
pub fn leading_fraction_zeros(v: &DzFixed) -> usize {
    let len = v.magnitude_digits().len();
    v.scale.saturating_sub(len)
}

/// A running integer, advanced by gate-composed addition. Series
/// denominators (`3, 5, 7, ...`, or the pairs `(2k)(2k+1)`) are produced
/// this way rather than by computing `2 * k + 1` natively and converting:
/// the counter itself is dozenal from the start.
pub struct Counter {
    value: DzFixed,
    one: DzFixed,
}

impl Counter {
    /// Starts at zero.
    pub fn new() -> Counter {
        Counter { value: zero(), one: one() }
    }

    /// The current value.
    pub fn value(&self) -> &DzFixed {
        &self.value
    }

    /// Advance by one and return the new value.
    pub fn next<G: DozGates>(&mut self, g: &G) -> DzFixed {
        self.value = self.value.add(g, &self.one);
        self.value.clone()
    }
}

impl Default for Counter {
    fn default() -> Counter {
        Counter::new()
    }
}

/// A small non-negative integer as a `DzFixed`, built by counting up with
/// gate-composed addition (`dz_int::small_usize_to_dz`), never by reading
/// `k`'s bits or digits.
pub fn small_int<G: DozGates>(g: &G, k: usize) -> DzFixed {
    DzFixed::from_int(dz_int::small_usize_to_dz(g, k))
}

/// `v` doubled, `times` times (used to undo argument halvings).
pub fn double_n_times<G: DozGates>(g: &G, v: &DzFixed, times: usize) -> DzFixed {
    let mut out = v.clone();
    let mut i = 0usize;
    while i < times {
        out = out.add(g, &out.clone());
        i += 1;
    }
    out
}

/// Round to `scale` fractional digits, half-to-even.
///
/// Every algorithm in this crate *truncates* internally -- that is what
/// keeps the error analysis simple, since truncation only ever loses, it
/// never oscillates. But truncating the final answer is what turns
/// `tan(pi/4)` into `0;↋↋↋↋↋↋↋↋` instead of `1`, so the public entry
/// points round instead, with the same half-to-even rule as the DFP
/// format (`docs/03-formato-dfp.md`).
///
/// The tie is detected by comparing the dropped remainder against half a
/// unit in the last place -- and half a unit is exact in base twelve
/// (`0;6` times the ulp), so this comparison needs no division.
pub fn round_to_scale<G: DozGates>(g: &G, v: &DzFixed, scale: usize) -> DzFixed {
    if v.scale <= scale {
        return v.clone();
    }
    let kept = truncate(v, scale);
    let remainder = abs(&v.sub(g, &kept));
    let half_ulp = ulp(scale).mul(g, &half());
    let away = match cmp(g, &remainder, &half_ulp) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => !kept.magnitude.is_even(g),
    };
    if !away {
        return kept;
    }
    let step = ulp(scale);
    if kept.is_negative() || (kept.is_zero() && v.is_negative()) {
        kept.sub(g, &step)
    } else {
        kept.add(g, &step)
    }
}
