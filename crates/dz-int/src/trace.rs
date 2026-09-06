//! Step-by-step trace of a dozenal subtraction, for the Phase 7 research
//! panel (`docs/05-fase7-panel-investigacion.md`, decision 2).
//!
//! # Why subtraction is traced here and not in `dz-gates`
//!
//! There is no subtractor circuit. `Dz::sub` is `add(negate(...))`, and the
//! opposite-sign case is `sub_magnitude_net`: take the elevens' complement
//! of `b`, add one to reach the twelve's complement, add *that* to `a`, and
//! read the result out of complement form. The strategy lives in this
//! crate, so the trace of it does too.
//!
//! The shape below is deliberately honest about that. It does **not**
//! present subtraction as if a borrow rippled through the digits: it shows
//! the complement as its own explicit stage, then the ordinary
//! ripple-carry addition that follows, and (when `a < b`) the second
//! complement that decodes the answer. A viewer that hid this would be
//! lying about which circuit ran.
//!
//! # Non-contamination
//!
//! The trace is collected through the `SubTraceSink` hook that
//! `sub_magnitude_net` already takes, so the circuit exists in exactly one
//! place and `subtract_trace` invokes no primitive that `Dz::sub` would
//! not. `tests/trace.rs` pins the resulting `GateCount` equality.

use crate::dzint::{sub_magnitude_trace_entry, SubTraceSink};
use dz_gates::circuits::StepRecord;
use dz_gates::{Digit, DozGates};

/// A whole subtraction of two non-negative magnitudes, stage by stage.
///
/// All digit fields are little-endian and already resolved: a viewer
/// displays them, it never recomputes anything from them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubTrace {
    pub a: Vec<Digit>,
    pub b: Vec<Digit>,
    /// Stage 1: elevens' complement of `b`, digit by digit (no carries).
    pub ones_complement: Vec<Digit>,
    /// Stage 2: the carry steps of `ones_complement + 1`.
    pub complement_steps: Vec<StepRecord>,
    /// Stage 2 result: the twelve's complement of `b`.
    pub radix_complement: Vec<Digit>,
    /// Stage 3: the carry steps of the real addition `a + radix_complement`.
    pub add_steps: Vec<StepRecord>,
    /// Stage 3 result, before interpretation.
    pub raw_sum: Vec<Digit>,
    /// `true` when `a < b`, i.e. `raw_sum` is itself in complement form and
    /// stages 4-5 ran to decode it.
    pub borrowed: bool,
    /// Stage 4 (only when `borrowed`): elevens' complement of `raw_sum`.
    pub decode_ones_complement: Vec<Digit>,
    /// Stage 5 (only when `borrowed`): the carry steps of that `+ 1`.
    pub decode_steps: Vec<StepRecord>,
    /// `|a - b|`.
    pub result: Vec<Digit>,
}

#[derive(Default)]
struct SubRecorder {
    ones_complement: Vec<Digit>,
    complement_steps: Vec<StepRecord>,
    radix_complement: Vec<Digit>,
    add_steps: Vec<StepRecord>,
    raw_sum: Vec<Digit>,
    borrowed: bool,
    decode_ones_complement: Vec<Digit>,
    decode_steps: Vec<StepRecord>,
}

fn step(
    position: usize,
    a: Digit,
    b: Digit,
    carry_in: Digit,
    sum: Digit,
    carry_out: Digit,
) -> StepRecord {
    StepRecord { position, a, b, carry_in, sum, carry_out }
}

impl SubTraceSink<Digit> for SubRecorder {
    fn ones_complement(&mut self, comp: &[Digit]) {
        self.ones_complement = comp.to_vec();
    }
    fn complement_step(
        &mut self,
        position: usize,
        a: Digit,
        b: Digit,
        carry_in: Digit,
        sum: Digit,
        carry_out: Digit,
    ) {
        self.complement_steps.push(step(position, a, b, carry_in, sum, carry_out));
    }
    fn radix_complement(&mut self, rc: &[Digit]) {
        self.radix_complement = rc.to_vec();
    }
    fn add_step(
        &mut self,
        position: usize,
        a: Digit,
        b: Digit,
        carry_in: Digit,
        sum: Digit,
        carry_out: Digit,
    ) {
        self.add_steps.push(step(position, a, b, carry_in, sum, carry_out));
    }
    fn raw_sum(&mut self, sum: &[Digit], borrowed: bool) {
        self.raw_sum = sum.to_vec();
        self.borrowed = borrowed;
    }
    fn decode_ones_complement(&mut self, comp: &[Digit]) {
        self.decode_ones_complement = comp.to_vec();
    }
    fn decode_step(
        &mut self,
        position: usize,
        a: Digit,
        b: Digit,
        carry_in: Digit,
        sum: Digit,
        carry_out: Digit,
    ) {
        self.decode_steps.push(step(position, a, b, carry_in, sum, carry_out));
    }
}

/// Trace `|a| - |b|` for two non-negative little-endian magnitudes, running
/// exactly the circuit `Dz::sub` runs.
pub fn subtract_trace<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> SubTrace {
    let mut recorder = SubRecorder::default();
    let (result, borrowed) = sub_magnitude_trace_entry(g, a, b, &mut recorder);
    SubTrace {
        a: a.to_vec(),
        b: b.to_vec(),
        ones_complement: recorder.ones_complement,
        complement_steps: recorder.complement_steps,
        radix_complement: recorder.radix_complement,
        add_steps: recorder.add_steps,
        raw_sum: recorder.raw_sum,
        borrowed,
        decode_ones_complement: recorder.decode_ones_complement,
        decode_steps: recorder.decode_steps,
        result,
    }
}
