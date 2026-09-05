//! Digit comparator, built via a decoder over the second operand -- the
//! same technique `half_adder`'s carry formula uses -- so that "x >= y" for
//! two *variable* digits can be computed purely from `thr`/`lit`/`min`/`max`
//! (recall `thr(j, x)` alone only supports a constant `u8` threshold `j`,
//! not a `Digit` operand).

use crate::digit::{Digit, RADIX};
use crate::traits::DozGates;

/// MVL-boolean `x >= y`: `MAX_{j=0..=11} MIN( lit(j, y), thr(j, x) )`.
/// Returns `Digit::MAX` (true) or `Digit::ZERO` (false) -- an MVL boolean
/// flag, not a numeric digit (see `circuits::adder` docs on the same
/// distinction for carry).
pub fn ge<G: DozGates>(g: &G, x: Digit, y: Digit) -> Digit {
    let mut out = Digit::ZERO;
    let mut j: u8 = 0;
    while j < RADIX {
        let branch = g.min(g.lit(j, y), g.thr(j, x));
        out = g.max(out, branch);
        j += 1;
    }
    out
}

/// Convert an MVL-boolean flag (`Digit::ZERO`/`Digit::MAX`) into a native
/// `bool`. This is an equality comparison against a fixed constant, not
/// arithmetic on a digit's numeric value -- the same category of operation
/// as `carry != Digit::ZERO` used elsewhere to decide `Vec` growth.
pub fn flag_to_bool(flag: Digit) -> bool {
    flag != Digit::ZERO
}
