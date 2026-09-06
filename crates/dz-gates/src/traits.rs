use crate::digit::Digit;
use crate::gatecount::GateCount;

/// The five functionally-complete Post-algebra primitives for 12-valued
/// logic (Section 2 of PLAN.md). Any `f: Z12^n -> Z12` can be expressed in
/// max-of-min-of-literals canonical form using only these.
///
/// Implementations carry their own gate-invocation counters (`cost`), so
/// different backends (MVL truth table, CRT decomposition, oracle table)
/// can be compared by circuit cost, not just by correctness.
pub trait DozGates {
    /// Successor mod 12 (`cycle(x) = x + 1 mod 12`, expressed as a single
    /// unary gate -- not native addition).
    fn cycle(&self, x: Digit) -> Digit;
    /// The smaller of two digits under the natural dozenal order.
    fn min(&self, x: Digit, y: Digit) -> Digit;
    /// The larger of two digits under the natural dozenal order.
    fn max(&self, x: Digit, y: Digit) -> Digit;
    /// Literal/indicator gate: `MAX` (i.e. `Digit::MAX`) if `x == j`, else `Digit::ZERO`.
    fn lit(&self, j: u8, x: Digit) -> Digit;
    /// Threshold gate: `MAX` if `x >= j`, else `Digit::ZERO`.
    fn thr(&self, j: u8, x: Digit) -> Digit;
    /// Snapshot of gate invocation counts since construction/last reset.
    fn cost(&self) -> GateCount;
    /// Backend-specific overhead the five headline counters cannot see:
    /// the number of full internal-representation -> tag reconstructions
    /// performed since the last reset.
    ///
    /// Zero by default, and that default is the *truth* for every backend
    /// whose internal representation already is the tag (MVL, the table
    /// oracle). Only `CrtGates` overrides it, because `Z12 = Z4 x Z3`
    /// destroys the order and every `min`/`max`/`thr`/`cycle` has to
    /// rebuild the digit before it can compare anything
    /// (`docs/02-crt-vs-mvl.md`).
    ///
    /// Deliberately *not* a field of `GateCount`: that struct stays
    /// backend-agnostic at the level of the five primitives, so the same
    /// circuit measured on two backends yields the same counters by
    /// construction. This is the sibling number that explains why equal
    /// counters do not mean equal cost. Like `GateCount` and the depth
    /// levels, it is `u64` bookkeeping about the *shape* of the run, never
    /// a dozenal value.
    fn reconstruction_ops(&self) -> u64 {
        0
    }
    /// Reset the invocation counters to zero.
    fn reset_cost(&self);
    /// Human-readable backend name, for reports/docs.
    fn name(&self) -> &'static str;
}
