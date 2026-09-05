//! Radix-complement circuits, used by `dz-int` to turn subtraction into
//! addition (PLAN.md Section 3: "complemento a 12 (radix complement)").
//!
//! Built purely from `DozGates` primitives via the same decoder-over-`j`
//! pattern as the adder: `j` is always a loop counter, never a digit tag.

use crate::circuits::adder::ripple_add;
use crate::digit::{Digit, RADIX};
use crate::traits::DozGates;

/// Diminished-radix ("elevens'") complement of a single digit: the unique
/// `e` with `d + e == 11`. Built as a decoder over all possible values `j`
/// of `d`: `comp(d) = MAX_j MIN( lit(j, d), CONST(11-j) )`. `11-j` is
/// arithmetic on the loop counter `j` (used to pick which constant symbol
/// each decoder branch contributes), not on a digit's tag.
pub fn ones_complement_digit<G: DozGates>(g: &G, d: Digit) -> Digit {
    let mut out = Digit::ZERO;
    let mut j: u8 = 0;
    while j < RADIX {
        let constant = Digit::new(RADIX - 1 - j).expect("RADIX-1-j is in 0..RADIX for j in 0..RADIX");
        let branch = g.min(g.lit(j, d), constant);
        out = g.max(out, branch);
        j += 1;
    }
    out
}

/// Diminished-radix complement of a whole little-endian number, digit by digit.
pub fn ones_complement<G: DozGates>(g: &G, digits: &[Digit]) -> Vec<Digit> {
    digits.iter().map(|&d| ones_complement_digit(g, d)).collect()
}

/// Fixed-width radix (12's) complement: `ones_complement(digits) + 1`,
/// truncated back to `width` digits (dropping any overflow out of the top,
/// exactly as a fixed-width complement register would).
pub fn radix_complement<G: DozGates>(g: &G, digits: &[Digit], width: usize) -> Vec<Digit> {
    let mut padded = digits.to_vec();
    while padded.len() < width {
        padded.push(Digit::ZERO);
    }
    padded.truncate(width);
    let comp1 = ones_complement(g, &padded);
    let mut sum = ripple_add(g, &comp1, &[Digit::ONE]);
    sum.truncate(width);
    while sum.len() < width {
        sum.push(Digit::ZERO);
    }
    sum
}
