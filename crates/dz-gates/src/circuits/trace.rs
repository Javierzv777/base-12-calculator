//! Step-by-step traces of the circuits, for the Phase 7 research panel
//! (`docs/05-fase7-panel-investigacion.md`).
//!
//! # Why this lives here and not above
//!
//! Only the circuit itself knows, position by position, which `x_i`/`y_i`/
//! `carry_in` fed which full adder. Reconstructing that from outside (say,
//! by calling `ripple_add` repeatedly on growing prefixes) would measure
//! something else and would rot the moment the circuit changed. So the
//! trace is taken from inside the one existing loop, via the callback
//! `ripple_add_net_traced` and the sink `multiply_net_traced` already
//! accept -- no circuit is written twice.
//!
//! # Non-contamination
//!
//! Recording a step reads signals the adder has already produced and
//! pushes them onto a `Vec`. No primitive is invoked, so a `GateCount`
//! taken around any function here equals the one taken around the
//! corresponding untraced function on the same inputs. `tests/trace.rs`
//! pins that as a regression, in the same spirit as the Phase 8 depth
//! work.
//!
//! # Scope
//!
//! Addition and multiplication are traced here. Subtraction is traced in
//! `dz-int` (`dz_int::trace`), because "subtract" is not a circuit at this
//! layer: it is the twelve's-complement *strategy* that `dz-int` builds on
//! top of these circuits, and a trace of it has to show the complement
//! steps to be honest about what actually ran.
//!
//! Carry-lookahead is deliberately not traced: its Kogge-Stone prefix
//! network has no "the carry crawls through position i" step to show, so a
//! faithful viewer for it would be a different visualization (prefix
//! stages, not digit positions). That is out of scope for Phase 7.

use crate::circuits::adder::ripple_add_net_traced;
use crate::circuits::depth::Values;
use crate::circuits::multiplier::{multiply_net_traced, MulTraceSink};
use crate::digit::Digit;
use crate::traits::DozGates;

/// One digit position's worth of what the step-by-step viewer shows.
///
/// All fields are already-resolved `Digit`s: the viewer displays them in
/// sequence and never recomputes anything from them. `position` is which
/// digit place (0 = least significant), plain `usize` bookkeeping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepRecord {
    pub position: usize,
    pub a: Digit,
    pub b: Digit,
    pub carry_in: Digit,
    pub sum: Digit,
    pub carry_out: Digit,
}

/// Step-by-step trace of `ripple_add`: the same result `ripple_add`
/// returns, plus one `StepRecord` per digit position, ordered from least
/// significant to most significant.
pub fn ripple_add_trace<G: DozGates>(
    g: &G,
    a: &[Digit],
    b: &[Digit],
) -> (Vec<Digit>, Vec<StepRecord>) {
    let mut steps: Vec<StepRecord> = Vec::new();
    let sum = ripple_add_net_traced(&Values(g), a, b, |position, x, y, carry_in, s, carry_out| {
        steps.push(StepRecord {
            position,
            a: x,
            b: y,
            carry_in,
            sum: s,
            carry_out,
        });
    });
    (sum, steps)
}

/// One multiplier digit's contribution to a schoolbook multiplication:
/// the partial row it generates, where that row sits, and how the row is
/// folded into the running total.
///
/// This is the nested shape a multiplication viewer needs: "multiplying by
/// digit `index` (`multiplier_digit`) gives partial row `row`, shifted to
/// `shifted`; accumulating it runs these carry steps and leaves
/// `accumulated`."
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartialRow {
    /// Which digit of the multiplier produced this row (0 = least
    /// significant), which is also how many places the row is shifted.
    pub index: usize,
    pub multiplier_digit: Digit,
    /// `a` scaled by `multiplier_digit`, little-endian.
    pub row: Vec<Digit>,
    /// The same row moved `index` places left (zeros prepended).
    pub shifted: Vec<Digit>,
    /// The ripple-carry trace of folding `shifted` into the accumulator.
    pub accumulate: Vec<StepRecord>,
    /// The accumulator after this row was added.
    pub accumulated: Vec<Digit>,
}

/// A whole multiplication, as a sequence of partial rows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MulTrace {
    pub a: Vec<Digit>,
    pub b: Vec<Digit>,
    pub rows: Vec<PartialRow>,
    pub product: Vec<Digit>,
}

/// Collects a `MulTrace` while `multiply_net_traced` runs.
struct MulRecorder {
    rows: Vec<PartialRow>,
}

impl MulTraceSink<Digit> for MulRecorder {
    fn row(&mut self, index: usize, multiplier_digit: Digit, row: &[Digit], shifted: &[Digit]) {
        self.rows.push(PartialRow {
            index,
            multiplier_digit,
            row: row.to_vec(),
            shifted: shifted.to_vec(),
            accumulate: Vec::new(),
            accumulated: Vec::new(),
        });
    }

    fn accumulate_step(
        &mut self,
        _index: usize,
        position: usize,
        a: Digit,
        b: Digit,
        carry_in: Digit,
        sum: Digit,
        carry_out: Digit,
    ) {
        // The sink is driven strictly in order, so the row being
        // accumulated is always the last one opened by `row`.
        if let Some(current) = self.rows.last_mut() {
            current.accumulate.push(StepRecord {
                position,
                a,
                b,
                carry_in,
                sum,
                carry_out,
            });
        }
    }

    fn row_done(&mut self, _index: usize, acc: &[Digit]) {
        if let Some(current) = self.rows.last_mut() {
            current.accumulated = acc.to_vec();
        }
    }
}

/// Step-by-step trace of `multiply`: the same product `multiply` returns,
/// plus one `PartialRow` per digit of `b`, each carrying its own nested
/// ripple-carry trace.
pub fn multiply_trace<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> MulTrace {
    let mut recorder = MulRecorder { rows: Vec::new() };
    let product = multiply_net_traced(&Values(g), a, b, &mut recorder);
    MulTrace {
        a: a.to_vec(),
        b: b.to_vec(),
        rows: recorder.rows,
        product,
    }
}
