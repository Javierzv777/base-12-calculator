//! Digit comparator, built via a decoder over the second operand -- the
//! same technique `half_adder`'s carry formula uses -- so that "x >= y" for
//! two *variable* digits can be computed purely from `thr`/`lit`/`min`/`max`
//! (recall `thr(j, x)` alone only supports a constant `u8` threshold `j`,
//! not a `Digit` operand).
//!
//! Written once over `circuits::depth::GateNet` and exposed twice, exactly
//! as `circuits::adder` is: `ge` is the `Values` instantiation (identical
//! gate count to before this generalization) and `ge_net` is what a
//! depth-tracking caller uses.

use crate::circuits::depth::{GateNet, Values};
use crate::digit::{Digit, RADIX};
use crate::traits::DozGates;

/// MVL-boolean `x >= y`: `MAX_{j=0..=11} MIN( lit(j, y), thr(j, x) )`.
/// Returns `Digit::MAX` (true) or `Digit::ZERO` (false) -- an MVL boolean
/// flag, not a numeric digit (see `circuits::adder` docs on the same
/// distinction for carry).
pub fn ge_net<N: GateNet>(net: &N, x: N::S, y: N::S) -> N::S {
    let mut out = net.lift(Digit::ZERO);
    let mut j: u8 = 0;
    while j < RADIX {
        let branch = net.min(net.lit(j, y), net.thr(j, x));
        out = net.max(out, branch);
        j += 1;
    }
    out
}

/// MVL-boolean `x >= y`. See `ge_net`.
pub fn ge<G: DozGates>(g: &G, x: Digit, y: Digit) -> Digit {
    ge_net(&Values(g), x, y)
}

/// Convert an MVL-boolean flag (`Digit::ZERO`/`Digit::MAX`) into a native
/// `bool`. This is an equality comparison against a fixed constant, not
/// arithmetic on a digit's numeric value -- the same category of operation
/// as `carry != Digit::ZERO` used elsewhere to decide `Vec` growth.
pub fn flag_to_bool(flag: Digit) -> bool {
    flag != Digit::ZERO
}
