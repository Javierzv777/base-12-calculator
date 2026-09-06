//! `dz-int` -- Layer 1: arbitrary-precision dozenal integers.
//!
//! `Dz` never applies a native arithmetic operator to a `Digit`'s tag. All
//! arithmetic is composed from `dz_gates::circuits` (built from
//! `DozGates` primitives) plus `Vec`/`usize` bookkeeping (indices, lengths,
//! loop counters), which PLAN.md Section 0 explicitly allows.
//!
//! Phase 7 added two pieces of instrumentation here, both designed so the
//! production path is byte-for-byte what it was: whole-expression circuit
//! depth (see `dzint`'s module docs) and the subtraction trace (see
//! `trace`).

pub mod dzint;
pub mod trace;

pub use dzint::{
    dz_to_small_usize, from_base10_digits, pow10, small_usize_to_dz, to_base10_digits, DivRemError,
    Dz, NoSubTrace, Sign, SubTraceSink,
};
pub use trace::{subtract_trace, SubTrace};
