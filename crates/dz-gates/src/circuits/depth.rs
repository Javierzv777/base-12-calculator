//! Circuit-depth instrumentation, via a signal abstraction over the gate
//! primitives.
//!
//! # Why this module exists
//!
//! `GateCount` counts *invocations*: it is a flat tally with no notion of
//! which gate fed which. Depth (the length of the longest chain of gates
//! from a primary input to an output) is a property of the data-flow graph,
//! and a `Digit` carries no provenance, so depth cannot be recovered from a
//! counter. Rather than write every circuit twice -- once "normal" and once
//! "instrumented", which would inevitably drift apart -- the circuits in
//! `circuits::adder` are written **once**, generic over a `GateNet`, and
//! this module supplies the two instantiations:
//!
//! * `Values` -- signals are plain `Digit`s; every method delegates straight
//!   to the underlying `DozGates` backend. Zero added cost, so the public
//!   `half_adder`/`ripple_add` wrappers behave and count exactly as before.
//! * `Levels` -- signals are `Sig { value, level }`; the value is still
//!   computed by the backend (so a depth run is also a correctness run),
//!   and the level is propagated alongside it.
//!
//! # The depth model (explicit, so tests can pin it)
//!
//! * A primary input has level `0`.
//! * A constant (`Digit::ZERO`, `Digit::ONE`, `Digit::MAX`, anything
//!   introduced by `lift`) has level `0`.
//! * Every primitive invocation yields `1 + max(levels of its signal
//!   inputs)`. `cycle` is unary. `lit(j, x)` and `thr(j, x)` are unary in
//!   `x`: the threshold/literal index `j` is a constant of the circuit
//!   structure (it selects which gate this is), not a signal, so it
//!   contributes no level.
//! * The depth of a circuit is the maximum level over all of its outputs.
//!
//! Consequently `cycle_pow(g, x, j)` contributes `j` levels, and a linear
//! `fold` of twelve `max`es contributes twelve levels while a balanced tree
//! over the same twelve terms would contribute four. That difference is the
//! entire point of the carry-lookahead experiment.
//!
//! # Non-contamination
//!
//! A `Levels` run still calls the backend for values, so it *does* bump the
//! backend's `GateCount`. Gate-count measurements and depth measurements
//! must therefore be taken in separate passes with separate (or `reset_cost`
//! -ed) backend instances -- the same discipline `lo_hi_table()` in
//! `circuits::multiplier` already uses for its throwaway synthesis backend.
//! Nothing was added to `GateCount` itself: `GateCount::total()` and its
//! `Add` impl still sum exactly the five published primitive counters.

use crate::digit::Digit;
use crate::traits::DozGates;

/// A network of 12-valued gates operating on some signal type.
///
/// The five methods mirror `DozGates` exactly; `lift` injects a constant or
/// a primary input, and `value` recovers the dozenal value a signal carries.
pub trait GateNet {
    /// The signal type flowing through this network.
    type S: Copy;

    /// Successor mod 12.
    fn cycle(&self, x: Self::S) -> Self::S;
    /// Smaller of two signals.
    fn min(&self, x: Self::S, y: Self::S) -> Self::S;
    /// Larger of two signals.
    fn max(&self, x: Self::S, y: Self::S) -> Self::S;
    /// Literal/indicator gate for the constant index `j`.
    fn lit(&self, j: u8, x: Self::S) -> Self::S;
    /// Threshold gate for the constant index `j`.
    fn thr(&self, j: u8, x: Self::S) -> Self::S;
    /// Inject a primary input or a constant. Level `0` by definition.
    fn lift(&self, d: Digit) -> Self::S;
    /// Inject a signal that is already known to arrive at `level`, because
    /// an earlier circuit in the same evaluation produced it.
    ///
    /// This is what makes depth *compose* across a layer boundary: a value
    /// that came out of one `ripple_add` re-enters the next circuit at the
    /// level it left the previous one, so the depth of a whole expression
    /// is the depth of one data-flow graph rather than the sum of
    /// unrelated per-operation measurements. Instantiations that carry no
    /// level (`Values`) ignore the argument, hence the default body.
    fn lift_at(&self, d: Digit, _level: u32) -> Self::S {
        self.lift(d)
    }
    /// Recover the dozenal value carried by a signal.
    fn value(s: Self::S) -> Digit;
    /// Recover the arrival level of a signal. `0` for instantiations that
    /// do not track depth.
    fn level(_s: Self::S) -> u32 {
        0
    }
}

/// Value-only instantiation: signals are `Digit`s, every call delegates to
/// the backend. This is what the public adder entry points use, so their
/// gate counts are byte-for-byte what they were before the generalization.
pub struct Values<'g, G: DozGates>(pub &'g G);

impl<G: DozGates> GateNet for Values<'_, G> {
    type S = Digit;

    fn cycle(&self, x: Digit) -> Digit {
        self.0.cycle(x)
    }
    fn min(&self, x: Digit, y: Digit) -> Digit {
        self.0.min(x, y)
    }
    fn max(&self, x: Digit, y: Digit) -> Digit {
        self.0.max(x, y)
    }
    fn lit(&self, j: u8, x: Digit) -> Digit {
        self.0.lit(j, x)
    }
    fn thr(&self, j: u8, x: Digit) -> Digit {
        self.0.thr(j, x)
    }
    fn lift(&self, d: Digit) -> Digit {
        d
    }
    fn value(s: Digit) -> Digit {
        s
    }
}

/// A dozenal value tagged with the depth of the gate chain that produced it.
///
/// `level` is plain `u32` bookkeeping about the circuit's shape -- never a
/// dozenal quantity -- so native arithmetic on it is the sanctioned kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sig {
    /// The dozenal value, computed by the underlying backend exactly as in
    /// a normal run.
    pub value: Digit,
    /// Longest chain of primitive invocations from any primary input.
    pub level: u32,
}

/// Depth-instrumented instantiation: values still come from the backend,
/// levels are accumulated as `1 + max(input levels)`.
pub struct Levels<'g, G: DozGates>(pub &'g G);

/// `1 + max(a, b)`, written without any native operator the purity lint
/// forbids (all of this is `u32` circuit bookkeeping regardless).
fn after(a: u32, b: u32) -> u32 {
    let m = if a >= b { a } else { b };
    m + 1
}

impl<G: DozGates> GateNet for Levels<'_, G> {
    type S = Sig;

    fn cycle(&self, x: Sig) -> Sig {
        Sig {
            value: self.0.cycle(x.value),
            level: x.level + 1,
        }
    }
    fn min(&self, x: Sig, y: Sig) -> Sig {
        Sig {
            value: self.0.min(x.value, y.value),
            level: after(x.level, y.level),
        }
    }
    fn max(&self, x: Sig, y: Sig) -> Sig {
        Sig {
            value: self.0.max(x.value, y.value),
            level: after(x.level, y.level),
        }
    }
    fn lit(&self, j: u8, x: Sig) -> Sig {
        Sig {
            value: self.0.lit(j, x.value),
            level: x.level + 1,
        }
    }
    fn thr(&self, j: u8, x: Sig) -> Sig {
        Sig {
            value: self.0.thr(j, x.value),
            level: x.level + 1,
        }
    }
    fn lift(&self, d: Digit) -> Sig {
        Sig { value: d, level: 0 }
    }
    fn lift_at(&self, d: Digit, level: u32) -> Sig {
        Sig { value: d, level }
    }
    fn value(s: Sig) -> Digit {
        s.value
    }
    fn level(s: Sig) -> u32 {
        s.level
    }
}

/// Depth of a circuit whose outputs are `sigs`: the maximum output level.
/// An empty output list has depth `0`.
pub fn max_level(sigs: &[Sig]) -> u32 {
    let mut best: u32 = 0;
    for s in sigs {
        if s.level > best {
            best = s.level;
        }
    }
    best
}

// ---------------------------------------------------------------------
// Whole-expression depth tracking (Phase 7).
//
// `Levels` measures one circuit whose inputs are primary. A user
// expression is many circuits chained together across three crates
// (`dz-int`, `dz-num`, `dz-sci`), and the interesting number is the depth
// of the *composed* data-flow graph. Threading a `Sig` through every one
// of those layers would mean making `Dz`, `DzFixed`, `DzRational` and
// `DzFloat` generic over a signal type -- a change to every public
// signature in the project.
//
// Instead, the layer that owns dozenal values (`dz-int::Dz`) carries the
// arrival level of its own digits and re-enters each circuit through
// `GateNet::lift_at`. Everything above `dz-int` composes `Dz` operations
// and therefore propagates depth without knowing that depth exists. What
// stays here, at Layer 0, is the switch and the high-water mark: they are
// circuit instrumentation, the same category as `GateCount`, and putting
// them here keeps `dz-int` free of a second notion of "am I being
// measured".
//
// Cost invariance is the point of the design: `Levels` invokes the
// backend exactly once per primitive, exactly as `Values` does, and its
// values are identical, so control flow is identical too. Turning
// tracking on therefore cannot change a measured `GateCount` -- only the
// u32 bookkeeping alongside it. Tests pin this.
// ---------------------------------------------------------------------

use std::cell::Cell;

thread_local! {
    /// `None` when depth tracking is off (the default, and what every
    /// non-instrumented caller sees); `Some(high_water)` while a
    /// `DepthScope` is alive.
    static DEPTH_STATE: Cell<Option<u32>> = const { Cell::new(None) };
}

/// Whether the current thread is inside a `DepthScope`.
///
/// Layers that own dozenal values check this to decide which `GateNet`
/// instantiation to run. Both answers compute the same values and invoke
/// the backend the same number of times.
pub fn depth_tracking() -> bool {
    DEPTH_STATE.with(|state| state.get().is_some())
}

/// Report that a circuit produced an output arriving at `level`.
///
/// A no-op when tracking is off. Otherwise the running maximum is raised,
/// so the final figure is the longest chain of primitive invocations
/// anywhere in the evaluation -- including intermediates, which is what
/// "depth of the circuit that computed this" means for a DAG whose
/// outputs are consumed by later circuits.
pub fn record_level(level: u32) {
    DEPTH_STATE.with(|state| {
        if let Some(high_water) = state.get() {
            if level > high_water {
                state.set(Some(level));
            }
        }
    });
}

/// RAII switch enabling whole-expression depth tracking for this thread.
///
/// Nested scopes are allowed: an inner scope starts its own high-water
/// mark and, on `finish`, the previous scope's mark is restored (raised to
/// at least the inner one, since an inner circuit is part of the outer
/// one).
pub struct DepthScope {
    previous: Option<u32>,
}

impl DepthScope {
    /// Turn tracking on, remembering whatever state was already in effect.
    pub fn enter() -> DepthScope {
        let previous = DEPTH_STATE.with(|state| state.replace(Some(0)));
        DepthScope { previous }
    }

    /// The depth measured so far in this scope.
    pub fn measured(&self) -> u32 {
        DEPTH_STATE.with(|state| state.get().unwrap_or(0))
    }
}

impl Drop for DepthScope {
    /// Restoring on `Drop` rather than in an explicit `finish` matters:
    /// `dz-api` returns early with `?` on an evaluation error, and a scope
    /// that leaked would leave the whole thread measuring depth forever.
    fn drop(&mut self) {
        let measured = self.measured();
        let restored = match self.previous {
            Some(outer) if outer >= measured => Some(outer),
            Some(_) => Some(measured),
            None => None,
        };
        DEPTH_STATE.with(|state| state.set(restored));
    }
}
