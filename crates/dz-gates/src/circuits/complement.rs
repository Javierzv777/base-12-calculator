//! Radix-complement circuits, used by `dz-int` to turn subtraction into
//! addition (PLAN.md Section 3: "complemento a 12 (radix complement)").
//!
//! Built purely from `DozGates` primitives via the same decoder-over-`j`
//! pattern as the adder: `j` is always a loop counter, never a digit tag.
//! Written once over `circuits::depth::GateNet`, instantiated for values
//! and for levels.

use crate::circuits::adder::{ripple_add, ripple_add_net};
use crate::circuits::depth::{GateNet, Values};
use crate::digit::{Digit, RADIX};
use crate::traits::DozGates;

/// Diminished-radix ("elevens'") complement of a single digit: the unique
/// `e` with `d + e == 11`. Built as a decoder over all possible values `j`
/// of `d`: `comp(d) = MAX_j MIN( lit(j, d), CONST(11-j) )`. `11-j` is
/// arithmetic on the loop counter `j` (used to pick which constant symbol
/// each decoder branch contributes), not on a digit's tag.
pub fn ones_complement_digit_net<N: GateNet>(net: &N, d: N::S) -> N::S {
    let mut out = net.lift(Digit::ZERO);
    let mut j: u8 = 0;
    while j < RADIX {
        let constant = Digit::new(RADIX - 1 - j).expect("RADIX-1-j is in 0..RADIX for j in 0..RADIX");
        let branch = net.min(net.lit(j, d), net.lift(constant));
        out = net.max(out, branch);
        j += 1;
    }
    out
}

/// Diminished-radix complement of a whole little-endian number, digit by
/// digit. Purely positional: each digit's complement is independent, so
/// this adds one decoder level and no carry chain.
pub fn ones_complement_net<N: GateNet>(net: &N, digits: &[N::S]) -> Vec<N::S> {
    digits
        .iter()
        .copied()
        .map(|d| ones_complement_digit_net(net, d))
        .collect()
}

/// Diminished-radix complement of a single digit. See
/// `ones_complement_digit_net`.
pub fn ones_complement_digit<G: DozGates>(g: &G, d: Digit) -> Digit {
    ones_complement_digit_net(&Values(g), d)
}

/// Diminished-radix complement of a whole little-endian number, digit by digit.
pub fn ones_complement<G: DozGates>(g: &G, digits: &[Digit]) -> Vec<Digit> {
    ones_complement_net(&Values(g), digits)
}

/// Fixed-width radix (12's) complement: `ones_complement(digits) + 1`,
/// truncated back to `width` digits (dropping any overflow out of the top,
/// exactly as a fixed-width complement register would).
pub fn radix_complement_net<N: GateNet>(net: &N, digits: &[N::S], width: usize) -> Vec<N::S> {
    let zero = net.lift(Digit::ZERO);
    let mut padded = digits.to_vec();
    while padded.len() < width {
        padded.push(zero);
    }
    padded.truncate(width);
    let comp1 = ones_complement_net(net, &padded);
    let one = net.lift(Digit::ONE);
    let mut sum = ripple_add_net(net, &comp1, &[one]);
    sum.truncate(width);
    while sum.len() < width {
        sum.push(zero);
    }
    sum
}

/// Fixed-width radix (12's) complement. See `radix_complement_net`.
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
