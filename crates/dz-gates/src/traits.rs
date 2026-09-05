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
    /// Reset the invocation counters to zero.
    fn reset_cost(&self);
    /// Human-readable backend name, for reports/docs.
    fn name(&self) -> &'static str;
}
