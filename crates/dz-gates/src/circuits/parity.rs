//! Digit parity, needed by DFP's half-even rounding (a tie rounds to the
//! digit that makes the retained mantissa *even*). Since 12 is even, a
//! dozenal digit's parity is exactly its ordinary even/odd-ness as a
//! quantity 0..11 -- built as a decoder over the six even tags, the same
//! `MAX_j MIN(lit(j,x), ...)` pattern used everywhere else in this module.
//! `0, 2, 4, 6, 8, 10` below are constant loop values (which tags count as
//! "even"), not a computation on any particular digit's tag.

use crate::digit::Digit;
use crate::traits::DozGates;

const EVEN_TAGS: [u8; 6] = [0, 2, 4, 6, 8, 10];

/// MVL-boolean: `Digit::MAX` if `d` is one of the six even digits, else
/// `Digit::ZERO`.
pub fn is_even_digit<G: DozGates>(g: &G, d: Digit) -> Digit {
    let mut out = Digit::ZERO;
    for &j in EVEN_TAGS.iter() {
        out = g.max(out, g.lit(j, d));
    }
    out
}
