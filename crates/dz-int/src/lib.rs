//! `dz-int` -- Layer 1: arbitrary-precision dozenal integers.
//!
//! `Dz` never applies a native arithmetic operator to a `Digit`'s tag. All
//! arithmetic is composed from `dz_gates::circuits` (built from
//! `DozGates` primitives) plus `Vec`/`usize` bookkeeping (indices, lengths,
//! loop counters), which PLAN.md Section 0 explicitly allows.

pub mod dzint;

pub use dzint::{dz_to_small_usize, small_usize_to_dz, DivRemError, Dz, Sign};
