//! Phase 7: the IPC surface of the research panel.
//!
//! Same discipline as `commands`: no dozenal logic lives here. Traces
//! arrive from `dz-api` already resolved to display characters, and a CSV
//! arrives as a finished document. The frontend animates and saves; it
//! never recomputes.

use crate::commands::{gate_count_dto, GateCountDto, OperationDto};
use crate::state::{AppState, Face};
use dz_api::{BackendCost, DisplayAlphabet, Trace, TraceKind, TraceRequest};
use serde::Serialize;
use tauri::State;

#[derive(Clone, Debug, Serialize)]
pub struct TraceStepDto {
    pub position: usize,
    pub a: String,
    pub b: String,
    pub carry_in: String,
    pub sum: String,
    pub carry_out: String,
}

/// Render one canonical character through the presentation alphabet
/// (`docs/07`). A trace comes out of `dz-api` in canonical glyphs; this is
/// the boundary where it becomes what the user asked to see. The CSV
/// export goes the other way, from `session.last_trace`, which is still
/// canonical -- an exchange document must not depend on a visual
/// preference.
fn glyph(alphabet: &DisplayAlphabet, c: char) -> String {
    alphabet.from_canonical(&c.to_string())
}

fn step_dtos(alphabet: &DisplayAlphabet, steps: &[dz_api::TraceStep]) -> Vec<TraceStepDto> {
    steps
        .iter()
        .map(|s| TraceStepDto {
            position: s.position,
            a: glyph(alphabet, s.a),
            b: glyph(alphabet, s.b),
            carry_in: glyph(alphabet, s.carry_in),
            sum: glyph(alphabet, s.sum),
            carry_out: glyph(alphabet, s.carry_out),
        })
        .collect()
}

/// One partial row of a multiplication, with its own nested addition
/// trace: what the viewer renders as "multiplying by digit N gives partial
/// row X; accumulating it runs [these steps]".
#[derive(Clone, Debug, Serialize)]
pub struct MulRowDto {
    pub index: usize,
    pub multiplier_digit: String,
    pub row: String,
    pub shifted: String,
    pub accumulate_steps: Vec<TraceStepDto>,
    pub accumulated: String,
}

/// Mirrors `dz_api::Trace`, tagged by `kind`.
///
/// The three shapes are genuinely different -- a subtraction has
/// complement stages, a multiplication has nested rows -- so they stay
/// three variants. Flattening them into one bag of optional fields would
/// only move the switch into the renderer and lose the guarantee that a
/// subtraction always carries its complement stages.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum TraceDto {
    Add {
        a: String,
        b: String,
        steps: Vec<TraceStepDto>,
        result: String,
    },
    Sub {
        a: String,
        b: String,
        ones_complement: String,
        complement_steps: Vec<TraceStepDto>,
        radix_complement: String,
        add_steps: Vec<TraceStepDto>,
        raw_sum: String,
        borrowed: bool,
        decode_ones_complement: String,
        decode_steps: Vec<TraceStepDto>,
        result: String,
    },
    Mul {
        a: String,
        b: String,
        rows: Vec<MulRowDto>,
        result: String,
    },
}

impl TraceDto {
    fn from_trace(alphabet: &DisplayAlphabet, trace: &Trace) -> TraceDto {
        let text = |s: &String| alphabet.from_canonical(s);
        match trace {
            Trace::Add(t) => TraceDto::Add {
                a: text(&t.a),
                b: text(&t.b),
                steps: step_dtos(alphabet, &t.steps),
                result: text(&t.result),
            },
            Trace::Sub(t) => TraceDto::Sub {
                a: text(&t.a),
                b: text(&t.b),
                ones_complement: text(&t.ones_complement),
                complement_steps: step_dtos(alphabet, &t.complement_steps),
                radix_complement: text(&t.radix_complement),
                add_steps: step_dtos(alphabet, &t.add_steps),
                raw_sum: text(&t.raw_sum),
                borrowed: t.borrowed,
                decode_ones_complement: text(&t.decode_ones_complement),
                decode_steps: step_dtos(alphabet, &t.decode_steps),
                result: text(&t.result),
            },
            Trace::Mul(t) => TraceDto::Mul {
                a: text(&t.a),
                b: text(&t.b),
                rows: t
                    .rows
                    .iter()
                    .map(|r| MulRowDto {
                        index: r.index,
                        multiplier_digit: glyph(alphabet, r.multiplier_digit),
                        row: text(&r.row),
                        shifted: text(&r.shifted),
                        accumulate_steps: step_dtos(alphabet, &r.accumulate_steps),
                        accumulated: text(&r.accumulated),
                    })
                    .collect(),
                result: text(&t.result),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct BackendCostDto {
    pub backend: String,
    pub gate_count: GateCountDto,
    pub depth: u32,
    /// Internal-representation -> tag reconstructions. Zero for MVL and
    /// for the oracle; the only column that separates the backends, since
    /// the five primitive counters are equal by construction
    /// (`docs/02-crt-vs-mvl.md`).
    pub reconstruction_ops: u64,
    /// True only for the table oracle. The panel labels it, so nobody
    /// reads "table wins" as a research result: it indexes memory instead
    /// of composing gates (`docs/01-diseno-sumador.md`).
    pub oracle: bool,
}

/// The verdict as Rust wrote it. The frontend renders these strings and
/// composes none of them: one authority for the reading of the table, so
/// the panel and the terminal say the same thing.
#[derive(Clone, Debug, Serialize)]
pub struct VerdictDto {
    pub winner: String,
    pub decided_by_reconstructions: bool,
    pub model: String,
    pub summary: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ComparisonDto {
    pub display: String,
    pub costs: Vec<BackendCostDto>,
    pub verdict: Option<VerdictDto>,
}

fn cost_dtos(costs: &[BackendCost]) -> Vec<BackendCostDto> {
    costs
        .iter()
        .map(|c| BackendCostDto {
            backend: c.backend.label().to_string(),
            gate_count: gate_count_dto(&c.gate_count),
            depth: c.depth,
            reconstruction_ops: c.reconstruction_ops,
            oracle: c.is_oracle(),
        })
        .collect()
}

/// Run one circuit step by step.
///
/// Independent of the session's display mode: a trace is over integer
/// digit positions, not over how a final value is rendered. It does honour
/// the session's backend, since "which backend am I watching" is exactly
/// the question the panel exists to ask.
///
/// The trace form belongs to the research view, which is the advanced
/// session's: `A` and `B` are typed there, in the advanced face's input
/// base and alphabet, so that is the session read here.
#[tauri::command]
pub fn trace_operation(
    state: State<AppState>,
    kind: String,
    a: String,
    b: String,
) -> Result<TraceDto, String> {
    let kind = TraceKind::parse(&kind).ok_or(format!("unknown trace kind '{}'", kind))?;
    let (backend, alphabet, input_base) = {
        let sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
        let session = sessions.get(Face::Advanced);
        (session.backend, session.alphabet, session.input_base)
    };
    let trace = dz_api::trace(&TraceRequest {
        backend,
        kind,
        a,
        b,
        alphabet,
        input_base,
    })
    .map_err(|e| e.to_string())?;
    let dto = TraceDto::from_trace(&alphabet, &trace);
    let mut sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    sessions.get_mut(Face::Advanced).last_trace = Some(trace);
    Ok(dto)
}

/// Run one operation on all three backends at once.
///
/// On demand, never on every keystroke: two thirds of that work is thrown
/// away whenever the user only wanted the answer.
///
/// `face` says whose operation this is. The panel lives next to the
/// advanced calculator, but the user can also arrive from the simple
/// face's gate line, and `144 + 11` only means what it showed there if it
/// is re-read in that face's input base. Reading the settings from the
/// face that produced the operation is what keeps the comparison about
/// the same calculation.
#[tauri::command]
pub fn compare_backends(
    state: State<AppState>,
    face: Face,
    op: OperationDto,
) -> Result<ComparisonDto, String> {
    let (mode, scale, alphabet, input_base) = {
        let sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
        let session = sessions.get(face);
        (
            session.mode,
            session.scale,
            session.alphabet,
            session.input_base,
        )
    };
    let comparison =
        dz_api::compare_backends(&op.into_operation(), mode, scale, alphabet, input_base)
            .map_err(|e| e.to_string())?;
    let dto = ComparisonDto {
        display: alphabet.from_canonical(&comparison.display),
        costs: cost_dtos(&comparison.costs),
        verdict: comparison.verdict.as_ref().map(|v| VerdictDto {
            winner: v.winner.label().to_string(),
            decided_by_reconstructions: v.decided_by_reconstructions,
            model: v.model.clone(),
            summary: v.summary.clone(),
        }),
    };
    // The panel is one place with one pair of export buttons, so what it
    // last produced is stored in one place -- the advanced session, which
    // is the panel's own -- whichever face the operation came from.
    let mut sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
    sessions.get_mut(Face::Advanced).last_comparison = Some(comparison.costs);
    Ok(dto)
}

/// Write one of the panel's tables to `path` as CSV.
///
/// The frontend picks the path with the native Save-as dialog
/// (`@tauri-apps/plugin-dialog`) and passes it here; the document itself is
/// built by `dz-api` and written by this command. The CSV text never
/// reaches JavaScript, so nobody can "fix a comma" there while Rust thinks
/// the format is something else -- the same single-authority rule the rest
/// of the project applies to formatting.
///
/// The chosen path is registered with the filesystem plugin's scope, so
/// the capability the app declares matches what the user actually granted
/// by picking that file.
#[tauri::command]
pub fn export_csv(
    app: tauri::AppHandle,
    state: State<AppState>,
    what: String,
    path: String,
) -> Result<String, String> {
    let csv = {
        let sessions = state.0.lock().map_err(|_| "session lock poisoned".to_string())?;
        // The panel's own session -- see `compare_backends`.
        let session = sessions.get(Face::Advanced);
        match what.as_str() {
            "gates" => {
                let costs = session
                    .last_comparison
                    .as_ref()
                    .ok_or("run a backend comparison first")?;
                dz_api::gate_count_csv(costs)
            }
            "trace" => {
                let trace = session.last_trace.as_ref().ok_or("run a trace first")?;
                dz_api::trace_csv(trace)
            }
            "trace-summary" => {
                let trace = session.last_trace.as_ref().ok_or("run a trace first")?;
                dz_api::trace_summary_csv(trace)
            }
            other => return Err(format!("unknown export '{}'", other)),
        }
    };

    use tauri_plugin_fs::FsExt;
    let target = std::path::PathBuf::from(&path);
    let _ = app.fs_scope().allow_file(&target);
    std::fs::write(&target, csv).map_err(|e| format!("could not write {}: {}", path, e))?;
    Ok(path)
}
