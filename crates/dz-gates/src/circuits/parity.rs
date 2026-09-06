//! Digit parity, needed by DFP's half-even rounding (a tie rounds to the
//! digit that makes the retained mantissa *even*). Since 12 is even, a
//! dozenal digit's parity is exactly its ordinary even/odd-ness as a
//! quantity 0..11 -- built as a decoder over the six even tags, the same
//! `MAX_j MIN(lit(j,x), ...)` pattern used everywhere else in this module.
//! `0, 2, 4, 6, 8, 10` below are constant loop values (which tags count as
//! "even"), not a computation on any particular digit's tag.
//!
//! One circuit, two instantiations (`circuits::depth::GateNet`), like every
//! other circuit in this crate.

use crate::circuits::depth::{GateNet, Values};
use crate::digit::Digit;
use crate::traits::DozGates;

const EVEN_TAGS: [u8; 6] = [0, 2, 4, 6, 8, 10];

/// MVL-boolean: true (`Digit::MAX`) if `d` is one of the six even digits.
pub fn is_even_digit_net<N: GateNet>(net: &N, d: N::S) -> N::S {
    let mut out = net.lift(Digit::ZERO);
    for &j in EVEN_TAGS.iter() {
        out = net.max(out, net.lit(j, d));
    }
    out
}

/// MVL-boolean: `Digit::MAX` if `d` is one of the six even digits, else
/// `Digit::ZERO`.
pub fn is_even_digit<G: DozGates>(g: &G, d: Digit) -> Digit {
    is_even_digit_net(&Values(g), d)
}
