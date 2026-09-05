//! `dz-num` -- Layer 2: dozenal fixed-point numbers (and, time permitting,
//! exact rationals / DFP -- see PLAN.md Section 4).
//!
//! `DzFixed` represents a value as `magnitude / 12^scale`, where
//! `magnitude: Dz` is an ordinary Layer-1 integer and `scale: usize` is
//! plain `usize` bookkeeping (how many digits follow the Humphrey point),
//! never a computation on a digit's value. All arithmetic on the
//! `magnitude` goes through `Dz`'s own gate-composed operations; this crate
//! adds no new digit-level logic of its own beyond `Vec`-level scale
//! alignment (padding one operand with trailing zero digits, i.e.
//! multiplying by a power of 12 -- itself done via `Dz::mul`, not native
//! arithmetic).

mod dfp;
mod fixed;
mod rational;

pub use dfp::{DfpKind, DzFloat};
pub use fixed::{DzFixed, DEFAULT_DIV_SCALE};
pub use rational::{DzRational, RationalError};
