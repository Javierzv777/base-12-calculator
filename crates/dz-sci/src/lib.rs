//! `dz-sci` -- Layer 2.5: elementary and scientific functions (`sqrt`,
//! `exp`, `ln`, `sin`, `cos`, `atan`, ...) over `dz-num`'s dozenal
//! numbers.
//!
//! # Why this is a crate and not a pile of code inside the evaluator
//!
//! PLAN.md Section 10bis-E parked transcendental functions ("requieren
//! CORDIC dozenal -- es un proyecto propio"). What actually blocked them
//! was not the mathematics but the shape of the code: the expression
//! language had no call syntax and no place to put numeric algorithms that
//! are neither a gate circuit (Layer 0) nor a number representation
//! (Layer 2). This crate is that place. It sits *above* `dz-num` and
//! *below* `dz-eval`, so the evaluator only ever registers functions --
//! it never contains numerical analysis -- and each function here is
//! ordinary series/Newton code written against `DzFixed`, with no
//! knowledge of the AST, the REPL, or the GUI.
//!
//! # The purity rule still holds, unchanged
//!
//! Nothing here applies a native operator to a dozenal value. Every
//! addition, multiplication, division and comparison goes through
//! `DzFixed`/`Dz`, i.e. through `DozGates`. The only native arithmetic is
//! on `usize` loop counters, `Vec` lengths and digit *counts* (scales),
//! exactly the category PLAN.md Section 0 permits. Concretely, that has
//! two visible consequences in the code below:
//!
//!   - series denominators (`3`, `5`, `7`, ... or `(2k)(2k+1)`) are never
//!     computed as `2 * k + 1` in `usize` and converted; they are carried
//!     as running `DzFixed` counters that advance by gate-composed
//!     addition and multiply each other by gate-composed multiplication;
//!   - halving is multiplication by the exact dozenal literal `0;6`, and
//!     scaling by a power of the radix is `scale` bookkeeping
//!     (`DzFixed::shift_point_right`), not a native shift.
//!
//! # Accuracy model
//!
//! Every function takes a target `scale` (fractional dozenal digits) and
//! computes internally at `scale + GUARD_DIGITS`, truncating (never
//! rounding) at each step, then truncates the result to `scale`. Results
//! are therefore correct to within a few units in the last internal place
//! and are reported as *approximate* by the evaluator -- with the single
//! exception of `sqrt` of a perfect square, which is verified exactly by
//! squaring the answer back (`sqrt(9)` is `3`, not `3` with a caveat).

pub mod constants;
pub mod elementary;
pub mod exact;
pub mod gamma;
pub mod trig;
pub mod util;

pub use constants::{euler, ln12, pi};
pub use elementary::{exp, ln, log12, log_base, sqrt};
pub use gamma::gamma;
pub use trig::{acos, asin, atan, cos, sin, tan};

/// Default working precision for scientific functions: 12 fractional
/// dozenal digits, matching `DzFloat`'s mantissa (`docs/03-formato-dfp.md`)
/// so a result can be shown in any of the three display modes without a
/// precision cliff between them.
pub const DEFAULT_SCI_SCALE: usize = 12;

/// Extra fractional digits carried internally and dropped at the end.
/// Four dozenal guard digits absorb roughly `12^4` truncation events,
/// which comfortably covers the few dozen truncating operations any one
/// series here performs.
pub const GUARD_DIGITS: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SciError {
    /// Argument outside the function's domain (`ln(0)`, `asin(2)`, ...).
    Domain(&'static str),
    DivisionByZero,
    /// The result (or an intermediate) would need an impractical number of
    /// digits -- e.g. `exp` of a very large argument.
    Overflow,
}

impl std::fmt::Display for SciError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SciError::Domain(what) => write!(f, "argument outside domain: {}", what),
            SciError::DivisionByZero => write!(f, "division by zero"),
            SciError::Overflow => write!(f, "result is too large to represent"),
        }
    }
}
impl std::error::Error for SciError {}
