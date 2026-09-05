//! Multiplier v1: shift-add, built only from `DozGates` primitives and the
//! adder circuits -- no digit-product truth table is ever consulted at
//! runtime, and no arithmetic is performed on digit tags.
//!
//! The single-digit multiply `digit_mul` uses the same decoder trick as
//! `half_adder`'s sum formula: for every candidate value `j` of `y`
//! (`j` is a `u8` loop counter, `0..12`, never a digit tag), compute what
//! `x * j` would be by *repeated addition* (`j` applications of
//! `ripple_add`, again counted by the loop variable, not by inspecting a
//! digit), then `MAX`-select the branch gated by `lit(j, y)`. This is
//! exactly "sumas desplazadas" (shift-add) from PLAN.md Section 2 v1,
//! generalized with a literal-decoder the same way the adder is.

use crate::circuits::adder::{half_adder, ripple_add};
use crate::digit::{Digit, RADIX};
use crate::traits::DozGates;

/// Single-digit multiply via repeated addition, decoded over all possible
/// values of `y`. Returns `(low_digit, high_digit)` such that
/// `x * y = high_digit * 12 + low_digit`.
pub fn digit_mul<G: DozGates>(g: &G, x: Digit, y: Digit) -> (Digit, Digit) {
    let mut lo = Digit::ZERO;
    let mut hi = Digit::ZERO;
    let mut j: u8 = 0;
    while j < RADIX {
        // acc represents x*j in little-endian digits, built by adding x to
        // itself j times. j and k are loop counters, not digit tags.
        let mut acc: Vec<Digit> = vec![Digit::ZERO];
        let mut k: u8 = 0;
        while k < j {
            acc = ripple_add(g, &acc, &[x]);
            k += 1;
        }
        let acc_lo = if !acc.is_empty() {
            acc[0]
        } else {
            Digit::ZERO
        };
        let acc_hi = if acc.len() > 1 { acc[1] } else { Digit::ZERO };

        let sel = g.lit(j, y);
        lo = g.max(lo, g.min(sel, acc_lo));
        hi = g.max(hi, g.min(sel, acc_hi));
        j += 1;
    }
    (lo, hi)
}

/// Multiply a multi-digit little-endian number by a single digit, returning
/// a little-endian multi-digit result (length `a.len() + 1` at most).
pub fn scale_by_digit<G: DozGates>(g: &G, a: &[Digit], d: Digit) -> Vec<Digit> {
    let mut out = Vec::with_capacity(a.len() + 1);
    let mut carry = Digit::ZERO;
    let mut i: usize = 0;
    while i < a.len() {
        let (lo, hi) = digit_mul(g, a[i], d);
        let (s, c1) = half_adder(g, lo, carry);
        // hi + c1 never overflows a single digit for base-12 schoolbook
        // multiplication (max hi is 10, c1 is 0 or 1); take the sum branch.
        let (carry_out, _c2) = half_adder(g, hi, c1);
        out.push(s);
        carry = carry_out;
        i += 1;
    }
    if carry != Digit::ZERO {
        out.push(carry);
    }
    out
}

/// Shift a little-endian number left by `k` digit positions (multiply by
/// `12^k`), by prepending `k` zero digits. `k` is `usize` bookkeeping, not a
/// digit value.
fn shift_left(a: &[Digit], k: usize) -> Vec<Digit> {
    let mut out = Vec::with_capacity(a.len() + k);
    let mut i = 0;
    while i < k {
        out.push(Digit::ZERO);
        i += 1;
    }
    out.extend_from_slice(a);
    out
}

/// Full shift-add multiplication of two little-endian multi-digit numbers.
pub fn multiply<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> Vec<Digit> {
    let mut acc: Vec<Digit> = vec![Digit::ZERO];
    let mut k: usize = 0;
    while k < b.len() {
        let row = scale_by_digit(g, a, b[k]);
        let shifted = shift_left(&row, k);
        acc = ripple_add(g, &acc, &shifted);
        k += 1;
    }
    acc
}
