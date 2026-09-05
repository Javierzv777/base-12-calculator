//! Adders built purely by composing `DozGates` primitives.
//!
//! No arithmetic is performed on digit tags in this file. `j` below is a
//! plain `u8` loop counter driving a decoder (as in PLAN.md Section 2's
//! `MAX_j MIN(...)` formulas) and `RADIX - j` is computing that loop
//! counter's complementary threshold parameter -- never a digit's tag.

use crate::digit::{Digit, RADIX};
use crate::traits::DozGates;

/// Apply `cycle` to `x`, `j` times (`j` is a loop counter, not a digit).
fn cycle_pow<G: DozGates>(g: &G, x: Digit, j: u8) -> Digit {
    let mut acc = x;
    let mut i: u8 = 0;
    while i < j {
        acc = g.cycle(acc);
        i += 1;
    }
    acc
}

/// Half/semi-adder for one dozenal digit pair.
///
/// `s = MAX_{j=0..=11} MIN( lit(j, y), cycle^j(x) )`
/// `c = MAX_{j=0..=11} MIN( lit(j, y), thr(12-j, x) )`
///
/// Returns `(sum_digit, carry_digit)` where carry is `Digit::ZERO` or
/// `Digit::ONE` (never anything else, since 11+11=22=1*12+10 in dozenal).
pub fn half_adder<G: DozGates>(g: &G, x: Digit, y: Digit) -> (Digit, Digit) {
    let mut sum = Digit::ZERO;
    let mut carry = Digit::ZERO;
    let mut j: u8 = 0;
    while j < RADIX {
        let branch_s = g.min(g.lit(j, y), cycle_pow(g, x, j));
        sum = g.max(sum, branch_s);

        let branch_c = g.min(g.lit(j, y), g.thr(RADIX - j, x));
        carry = g.max(carry, branch_c);
        j += 1;
    }
    // `carry` at this point is an MVL-boolean flag (`Digit::ZERO` for false,
    // `Digit::MAX` for true) -- `lit`/`thr` both signal "true" as the top
    // symbol, not as the numeric digit 1. A dozenal add carry has *weight*
    // 1 (it's added into the next position), so it must be digitized from
    // the boolean domain into the numeric domain: `MIN(flag, ONE)` yields
    // `ZERO` when flag is `ZERO` and `ONE` when flag is `MAX` (since
    // `min(MAX, ONE) == ONE`). This is the same "MIN as a mux" trick used
    // for `branch_s` above, just gating a constant instead of a variable.
    let carry = g.min(carry, Digit::ONE);
    (sum, carry)
}

/// Full adder: two half-adders plus a carry-in, carries combined with `max`
/// (safe because a dozenal digit-add carry is always 0 or 1, so the two
/// half-adder carries can never both be `Digit::ONE`... actually they can:
/// x+y может carry, and (x+y mod 12)+cin can carry too, but never both at
/// once produce more than 1 total carry-out, so MAX is exactly the OR of
/// two booleans encoded as ZERO/ONE digits, which is correct here).
pub fn full_adder<G: DozGates>(g: &G, x: Digit, y: Digit, cin: Digit) -> (Digit, Digit) {
    let (s1, c1) = half_adder(g, x, y);
    let (s2, c2) = half_adder(g, s1, cin);
    let cout = g.max(c1, c2);
    (s2, cout)
}

/// Ripple-carry addition of two little-endian multi-digit numbers.
/// Shorter operand is treated as padded with `Digit::ZERO` (this padding
/// decision, and the final length, are `Vec`/`usize` bookkeeping -- not
/// arithmetic on digit values).
pub fn ripple_add<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> Vec<Digit> {
    let n = if a.len() >= b.len() { a.len() } else { b.len() };
    let mut out = Vec::with_capacity(n + 1);
    let mut carry = Digit::ZERO;
    let mut i: usize = 0;
    while i < n {
        let da = if i < a.len() { a[i] } else { Digit::ZERO };
        let db = if i < b.len() { b[i] } else { Digit::ZERO };
        let (s, c) = full_adder(g, da, db, carry);
        out.push(s);
        carry = c;
        i += 1;
    }
    // Equality against the ZERO symbol (derived `PartialEq`), not arithmetic
    // on the tag: deciding whether to grow the `Vec` by one limb.
    if carry != Digit::ZERO {
        out.push(carry);
    }
    out
}
