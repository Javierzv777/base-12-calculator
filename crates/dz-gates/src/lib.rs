//! `dz-gates` -- Layer 0 of the dozenal calculator: 12-valued logic gates
//! and the circuits (adder, multiplier) built purely by composing them.
//!
//! See PLAN.md Section 0 for the purity contract this crate exists to
//! enforce and `crate::digit` for the details of what's allowed where.

pub mod backends;
pub mod circuits;
pub mod digit;
pub mod gatecount;
pub mod traits;

pub use backends::{oracle_add, oracle_mul, CrtGates, MvlGates, TableGates};
pub use digit::{Digit, RADIX};
pub use gatecount::GateCount;
pub use traits::DozGates;
