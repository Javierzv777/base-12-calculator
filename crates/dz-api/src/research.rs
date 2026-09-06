//! The research-panel surface of `dz-api` (Phase 7,
//! `docs/05-fase7-panel-investigacion.md`): step-by-step circuit traces,
//! the three-backend comparison, and CSV export.
//!
//! Two rules shape everything here.
//!
//! **Nothing recomputable crosses the boundary.** Every dozenal value in
//! these types is a `char` or a `String` produced by
//! `dz_parse::alphabet::digit_to_char` -- the single formatting authority
//! the rest of the project already uses. A frontend cannot add two `char`s
//! and get a dozenal answer, which is the purity rule applied to a
//! language boundary.
//!
//! **Rust owns the CSV.** `gate_count_csv`/`trace_csv` return the finished
//! document; a caller persists the bytes and never reformats them. One
//! authority for the format, as for everything else.
//!
//! Layering note: this module names `dz_int` (Layer 1) directly, for the
//! subtraction trace. `dz-api` already reaches past its neighbour to
//! `dz_gates` (Layer 0) to build the backends a request asks for; a trace
//! is instrumentation of those same layers, not a second arithmetic path.
//! No arithmetic is performed here -- the traces come back finished.

use crate::{ApiError, BackendKind, NumberMode, Operation, Request};
use dz_gates::circuits::{multiply_trace, ripple_add_trace, StepRecord};
use dz_gates::{CrtGates, Digit, DozGates, GateCount, MvlGates, TableGates};
use dz_int::subtract_trace;
use dz_parse::alphabet::digit_to_char;
use dz_parse::{lex, DisplayAlphabet, Token};

/// One digit position of a ripple-carry run, already resolved to display
/// glyphs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraceStep {
    /// Which digit place, 0 = least significant. `usize` bookkeeping.
    pub position: usize,
    pub a: char,
    pub b: char,
    pub carry_in: char,
    pub sum: char,
    pub carry_out: char,
}

impl TraceStep {
    fn from_record(record: &StepRecord) -> TraceStep {
        TraceStep {
            position: record.position,
            a: digit_to_char(record.a),
            b: digit_to_char(record.b),
            carry_in: digit_to_char(record.carry_in),
            sum: digit_to_char(record.sum),
            carry_out: digit_to_char(record.carry_out),
        }
    }
}

fn steps_of(records: &[StepRecord]) -> Vec<TraceStep> {
    records.iter().map(TraceStep::from_record).collect()
}

/// Render little-endian digits most-significant-first, keeping every
/// position (no leading-zero trimming): the viewer lines rows up by
/// position, so a dropped digit would misalign the display.
fn render(digits: &[Digit]) -> String {
    let mut out = String::with_capacity(digits.len());
    let mut i = digits.len();
    while i > 0 {
        i -= 1;
        out.push(digit_to_char(digits[i]));
    }
    if out.is_empty() {
        out.push(digit_to_char(Digit::ZERO));
    }
    out
}

/// Which circuit the step-by-step viewer should run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceKind {
    /// Ripple-carry addition.
    Add,
    /// Twelve's-complement subtraction (complement, then add).
    Sub,
    /// Schoolbook multiplication: one partial row per multiplier digit.
    Mul,
}

impl TraceKind {
    pub fn parse(name: &str) -> Option<TraceKind> {
        match name {
            "add" => Some(TraceKind::Add),
            "sub" => Some(TraceKind::Sub),
            "mul" => Some(TraceKind::Mul),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            TraceKind::Add => "add",
            TraceKind::Sub => "sub",
            TraceKind::Mul => "mul",
        }
    }

    pub const ALL: [TraceKind; 3] = [TraceKind::Add, TraceKind::Sub, TraceKind::Mul];
}

/// The two operands of a *plain non-negative integer* circuit run.
///
/// Deliberately narrower than `Request`: a trace is defined over one
/// concrete circuit, not over the expression language. "The trace of
/// `4;6 * 3 + sin(1)`" has no single answer, and inventing one would be
/// the viewer lying about what ran.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceRequest {
    pub backend: BackendKind,
    pub kind: TraceKind,
    pub a: String,
    pub b: String,
    /// The presentation alphabet `a` and `b` are typed in (`docs/07`).
    /// Input only: the resulting `Trace` carries canonical glyphs, and an
    /// interface renders them through `DisplayAlphabet::from_canonical`.
    /// That is what keeps the CSV export alphabet-stable.
    pub alphabet: DisplayAlphabet,
}

/// A ripple-carry addition, position by position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddTrace {
    pub a: String,
    pub b: String,
    pub steps: Vec<TraceStep>,
    pub result: String,
}

/// A subtraction, stage by stage.
///
/// The shape is honest about the fact that there is no subtractor: the
/// complement stages are shown explicitly, then the ordinary addition, then
/// (only when `borrowed`) the second complement that decodes the answer.
/// See `dz_int::trace` for why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubTrace {
    pub a: String,
    pub b: String,
    /// Stage 1: elevens' complement of `b` (positional, no carries).
    pub ones_complement: String,
    /// Stage 2: carry steps of `ones_complement + 1`.
    pub complement_steps: Vec<TraceStep>,
    /// Stage 2 result: the twelve's complement of `b`.
    pub radix_complement: String,
    /// Stage 3: carry steps of `a + radix_complement`.
    pub add_steps: Vec<TraceStep>,
    /// Stage 3 result, before interpretation.
    pub raw_sum: String,
    /// `a < b`, i.e. stages 4-5 ran to decode the answer.
    pub borrowed: bool,
    /// Stage 4 (only when `borrowed`).
    pub decode_ones_complement: String,
    /// Stage 5 (only when `borrowed`).
    pub decode_steps: Vec<TraceStep>,
    pub result: String,
}

/// One multiplier digit's partial row and the nested addition that folds
/// it into the running product.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MulRow {
    /// Which digit of the multiplier, 0 = least significant. Also how many
    /// places the row is shifted.
    pub index: usize,
    pub multiplier_digit: char,
    /// The multiplicand scaled by `multiplier_digit`.
    pub row: String,
    /// The same row shifted `index` places left.
    pub shifted: String,
    /// The ripple-carry trace of accumulating `shifted`.
    pub accumulate_steps: Vec<TraceStep>,
    /// The running product after this row.
    pub accumulated: String,
}

/// A whole multiplication: a nested trace, one group per multiplier digit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MulTrace {
    pub a: String,
    pub b: String,
    pub rows: Vec<MulRow>,
    pub result: String,
}

/// What `trace` returns: one of the three shapes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Trace {
    Add(AddTrace),
    Sub(SubTrace),
    Mul(MulTrace),
}

impl Trace {
    pub fn kind(&self) -> TraceKind {
        match self {
            Trace::Add(_) => TraceKind::Add,
            Trace::Sub(_) => TraceKind::Sub,
            Trace::Mul(_) => TraceKind::Mul,
        }
    }

    /// The final value, as text.
    pub fn result(&self) -> &str {
        match self {
            Trace::Add(t) => &t.result,
            Trace::Sub(t) => &t.result,
            Trace::Mul(t) => &t.result,
        }
    }
}

/// Read one plain, non-negative dozenal integer literal.
///
/// Uses the same lexer everything else does -- there is no second
/// text-to-`Digit` path in this project -- and then rejects anything that
/// is not exactly one integer literal: an expression, a signed value or a
/// Humphrey point all describe something a single circuit run cannot
/// represent.
fn integer_operand(
    alphabet: &DisplayAlphabet,
    text: &str,
    which: &str,
) -> Result<Vec<Digit>, ApiError> {
    let text = &alphabet.to_canonical(text)?;
    let tokens = lex(text).map_err(|e| ApiError::Parse(dz_parse::ExprParseError::Lex(e)))?;
    match tokens.as_slice() {
        [Token::Number { int_digits, frac_digits }] if frac_digits.is_empty() => {
            // The lexer yields the integer part most-significant first; the
            // circuits are little-endian. Reversing a `Vec` is bookkeeping.
            let mut little_endian = int_digits.clone();
            little_endian.reverse();
            if little_endian.is_empty() {
                little_endian.push(Digit::ZERO);
            }
            Ok(little_endian)
        }
        _ => Err(ApiError::Trace(format!(
            "the circuit viewer traces one concrete run, so operand {} must be a plain \
             non-negative dozenal integer (no expression, sign or Humphrey point): got '{}'",
            which, text
        ))),
    }
}

/// Run one circuit with the requested backend and record every step.
pub fn trace(req: &TraceRequest) -> Result<Trace, ApiError> {
    let a = integer_operand(&req.alphabet, &req.a, "A")?;
    let b = integer_operand(&req.alphabet, &req.b, "B")?;
    match req.backend {
        BackendKind::Mvl => trace_with(&MvlGates::new(), req.kind, &a, &b),
        BackendKind::Crt => trace_with(&CrtGates::new(), req.kind, &a, &b),
        BackendKind::Table => trace_with(&TableGates::new(), req.kind, &a, &b),
    }
}

/// The same run as `trace`, plus what it cost in gates (Phase 9).
///
/// Added rather than folded into `trace` so no existing caller changes.
/// Only the five counters come back, **not** a depth: the traced circuits
/// are instantiated with `Values` (`dz_gates::circuits::trace`), which is
/// the instantiation that does not carry arrival levels, so there is no
/// honest depth to report for this pass. An interface that wants depth for
/// the same arithmetic asks for the ordinary call, where `execute`
/// measures both in one pass.
pub fn trace_measured(req: &TraceRequest) -> Result<(Trace, GateCount), ApiError> {
    let a = integer_operand(&req.alphabet, &req.a, "A")?;
    let b = integer_operand(&req.alphabet, &req.b, "B")?;
    match req.backend {
        BackendKind::Mvl => measured_trace(&MvlGates::new(), req.kind, &a, &b),
        BackendKind::Crt => measured_trace(&CrtGates::new(), req.kind, &a, &b),
        BackendKind::Table => measured_trace(&TableGates::new(), req.kind, &a, &b),
    }
}

fn measured_trace<G: DozGates>(
    g: &G,
    kind: TraceKind,
    a: &[Digit],
    b: &[Digit],
) -> Result<(Trace, GateCount), ApiError> {
    g.reset_cost();
    let trace = trace_with(g, kind, a, b)?;
    Ok((trace, g.cost()))
}

fn trace_with<G: DozGates>(
    g: &G,
    kind: TraceKind,
    a: &[Digit],
    b: &[Digit],
) -> Result<Trace, ApiError> {
    Ok(match kind {
        TraceKind::Add => {
            let (sum, steps) = ripple_add_trace(g, a, b);
            Trace::Add(AddTrace {
                a: render(a),
                b: render(b),
                steps: steps_of(&steps),
                result: render(&sum),
            })
        }
        TraceKind::Sub => {
            let t = subtract_trace(g, a, b);
            let mut result = render(&t.result);
            if t.borrowed {
                // |a| < |b|: the magnitude is right, the sign is not.
                result.insert(0, '-');
            }
            Trace::Sub(SubTrace {
                a: render(&t.a),
                b: render(&t.b),
                ones_complement: render(&t.ones_complement),
                complement_steps: steps_of(&t.complement_steps),
                radix_complement: render(&t.radix_complement),
                add_steps: steps_of(&t.add_steps),
                raw_sum: render(&t.raw_sum),
                borrowed: t.borrowed,
                decode_ones_complement: render(&t.decode_ones_complement),
                decode_steps: steps_of(&t.decode_steps),
                result,
            })
        }
        TraceKind::Mul => {
            let t = multiply_trace(g, a, b);
            Trace::Mul(MulTrace {
                a: render(&t.a),
                b: render(&t.b),
                rows: t
                    .rows
                    .iter()
                    .map(|row| MulRow {
                        index: row.index,
                        multiplier_digit: digit_to_char(row.multiplier_digit),
                        row: render(&row.row),
                        shifted: render(&row.shifted),
                        accumulate_steps: steps_of(&row.accumulate),
                        accumulated: render(&row.accumulated),
                    })
                    .collect(),
                result: render(&t.product),
            })
        }
    })
}

/// One backend's cost for one operation: the five primitive counters, the
/// circuit depth of the same run, and the reconstructions that run needed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BackendCost {
    pub backend: BackendKind,
    pub gate_count: GateCount,
    pub depth: u32,
    /// Internal-representation -> tag reconstructions
    /// (`DozGates::reconstruction_ops`). Always `0` for MVL and for the
    /// table oracle; the CRT column is the whole reason this table can
    /// separate backends at all, since the five counters cannot.
    pub reconstruction_ops: u64,
}

impl BackendCost {
    /// Whether this row is the truth-table oracle rather than a
    /// production backend. Kept here so both interfaces ask the same
    /// question of the same authority instead of each re-deriving it.
    pub fn is_oracle(&self) -> bool {
        self.backend == BackendKind::Table
    }

    /// The ranking key, lexicographic and in this order: primitive
    /// invocations, then depth, then reconstructions.
    ///
    /// No weighted formula. There is no measured exchange rate between a
    /// primitive invocation and a reconstruction, so inventing a
    /// coefficient would manufacture precision the experiment does not
    /// have. Ordering lexicographically says exactly what is known: the
    /// headline cost decides when it differs, and reconstructions break
    /// the ties it leaves -- which, for these circuits, is every tie
    /// (`docs/02-crt-vs-mvl.md`).
    fn rank_key(&self) -> (u64, u32, u64) {
        (self.gate_count.total(), self.depth, self.reconstruction_ops)
    }
}

/// The one-sentence reading of a comparison table, composed here so the
/// CLI and the GUI cannot drift apart (`docs/06-fase9-paridad-cli.md`
/// section 7): both receive it already written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Verdict {
    /// The cheapest *production* backend. The table oracle is excluded
    /// from the ranking (`docs/05` section 5, `docs/01`): its numbers are
    /// real, but it indexes memory instead of composing gates, so
    /// "cheaper than the oracle" is not a research answer.
    pub winner: BackendKind,
    /// True when the ranked backends tie on all five primitive counters
    /// and on depth, so the decision came from the reconstruction column
    /// alone. For every circuit in this repo it is true, and saying so is
    /// the point: a bare "mvl wins" would hide the finding.
    pub decided_by_reconstructions: bool,
    /// The cost model, spelled out. Shown next to the verdict so the
    /// reader can see what was ranked instead of trusting a number.
    pub model: String,
    /// The full verdict as prose, ready to print.
    pub summary: String,
}

/// Rank the production backends of a comparison and write the verdict.
///
/// Returns `None` only if the comparison holds no production backend,
/// which `compare_backends` never produces.
pub fn verdict(costs: &[BackendCost]) -> Option<Verdict> {
    let mut ranked: Vec<&BackendCost> = costs.iter().filter(|c| !c.is_oracle()).collect();
    ranked.sort_by_key(|c| c.rank_key());
    let winner = *ranked.first()?;
    let model = "primitive invocations + internal->tag reconstructions, \
                 ranked lexicographically by (total, depth, reconstructions)"
        .to_string();

    let tied: Vec<&&BackendCost> = ranked
        .iter()
        .filter(|c| {
            c.gate_count.total() == winner.gate_count.total() && c.depth == winner.depth
        })
        .collect();
    let decided_by_reconstructions = tied.len() > 1;

    let mut summary = String::new();
    if decided_by_reconstructions {
        let names: Vec<&str> = tied.iter().map(|c| c.backend.label()).collect();
        summary.push_str(&format!(
            "verdict: {} is cheapest. {} tie on all five primitive counters ({}) and on depth ({}), \
             so the five headline numbers cannot separate them -- they are equal by construction, \
             since every backend implements the same primitives for the same circuit. \
             The tie is broken by reconstructions: ",
            winner.backend.label(),
            names.join(" and "),
            winner.gate_count.total(),
            winner.depth
        ));
        for (i, c) in tied.iter().enumerate() {
            if i > 0 {
                summary.push_str(", ");
            }
            summary.push_str(&format!("{} {}", c.backend.label(), c.reconstruction_ops));
        }
        summary.push('.');
    } else {
        summary.push_str(&format!(
            "verdict: {} is cheapest, at {} primitive invocations, depth {}, {} reconstructions.",
            winner.backend.label(),
            winner.gate_count.total(),
            winner.depth,
            winner.reconstruction_ops
        ));
    }
    summary.push_str(&format!(" Cost model: {}.", model));
    if costs.iter().any(|c| c.is_oracle()) {
        summary.push_str(
            " The table backend is listed but not ranked: it is the truth-table oracle, \
             not a production backend.",
        );
    }
    Some(Verdict {
        winner: winner.backend,
        decided_by_reconstructions,
        model,
        summary,
    })
}

/// The same operation measured on all three backends.
///
/// The *value* is asserted equal across backends: a mismatch would be a
/// correctness bug, not a research finding, so it is returned once.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackendComparison {
    pub display: String,
    pub costs: Vec<BackendCost>,
    /// The reading of the table, written here rather than in an
    /// interface. One authority, exactly like the CSV and the JSON: the
    /// terminal and the GUI print the same sentence because they are
    /// handed the same sentence.
    pub verdict: Option<Verdict>,
}

/// Run `op` against MVL, CRT and Table and report each one's cost.
///
/// Always all three (`docs/05` section 5): the set is small and fixed, and
/// making it configurable would only invite comparing two backends and
/// calling it a result. Note that `Table` is the oracle, not a production
/// backend -- its numbers are real but are not a "which is cheaper"
/// answer, and an interface showing this table should label it as such.
///
/// `alphabet` is the presentation alphabet `op`'s operand *text* is
/// written in, for the same reason `Request` carries one: every `dz-api`
/// entry point that accepts user text also accepts the alphabet that text
/// is in. The `display` that comes back is canonical.
pub fn compare_backends(
    op: &Operation,
    mode: NumberMode,
    scale: usize,
    alphabet: DisplayAlphabet,
) -> Result<BackendComparison, ApiError> {
    let mut costs = Vec::with_capacity(BackendKind::ALL.len());
    let mut display: Option<String> = None;
    for backend in BackendKind::ALL {
        let request = Request {
            backend,
            mode,
            scale,
            alphabet,
            op: op.clone(),
        };
        let response = crate::execute(&request)?;
        match &display {
            Some(first) => debug_assert_eq!(
                first, &response.display,
                "backends disagree -- correctness bug, not a research result"
            ),
            None => display = Some(response.display.clone()),
        }
        costs.push(BackendCost {
            backend,
            gate_count: response
                .gate_count
                .expect("a successful call always counts gates"),
            depth: response.depth.unwrap_or(0),
            reconstruction_ops: response.reconstruction_ops.unwrap_or(0),
        });
    }
    let verdict = verdict(&costs);
    Ok(BackendComparison {
        display: display.unwrap_or_default(),
        costs,
        verdict,
    })
}

/// The gate/depth table as CSV: one row per backend.
///
/// Used for both "this operation on one backend" (one row) and the
/// three-backend comparison (three rows).
pub fn gate_count_csv(rows: &[BackendCost]) -> String {
    let mut out =
        String::from("backend,cycle,min,max,lit,thr,total,depth,reconstruction_ops,oracle\n");
    for row in rows {
        out.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            row.backend.label(),
            row.gate_count.cycle,
            row.gate_count.min,
            row.gate_count.max,
            row.gate_count.lit,
            row.gate_count.thr,
            row.gate_count.total(),
            row.depth,
            row.reconstruction_ops,
            row.is_oracle()
        ));
    }
    out
}

fn push_steps(out: &mut String, kind: &str, stage: &str, group: usize, steps: &[TraceStep]) {
    for step in steps {
        out.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            kind, stage, group, step.position, step.a, step.b, step.carry_in, step.sum,
            step.carry_out
        ));
    }
}

/// A trace as CSV, one row per digit position.
///
/// One schema covers all three trace shapes: `stage` says which circuit
/// stage a row belongs to (`add`, `complement`, `decode`, `accumulate`)
/// and `group` says which partial row of a multiplication it came from
/// (always `0` for addition and subtraction). Widening one schema beats
/// three incompatible files when the point is to compare them in a
/// spreadsheet.
pub fn trace_csv(trace: &Trace) -> String {
    let mut out = String::from("kind,stage,group,position,a,b,carry_in,sum,carry_out\n");
    match trace {
        Trace::Add(t) => push_steps(&mut out, "add", "add", 0, &t.steps),
        Trace::Sub(t) => {
            push_steps(&mut out, "sub", "complement", 0, &t.complement_steps);
            push_steps(&mut out, "sub", "add", 0, &t.add_steps);
            push_steps(&mut out, "sub", "decode", 0, &t.decode_steps);
        }
        Trace::Mul(t) => {
            for row in &t.rows {
                push_steps(&mut out, "mul", "accumulate", row.index, &row.accumulate_steps);
            }
        }
    }
    out
}

/// The non-step context of a trace, as a second small CSV: the operands,
/// the intermediate words a viewer shows above the step table, and the
/// result. Kept separate from `trace_csv` rather than bolted on as
/// optional columns, following the same discipline `docs/05` section 4
/// states for a future depth CSV.
pub fn trace_summary_csv(trace: &Trace) -> String {
    let mut out = String::from("kind,field,value\n");
    let kind = trace.kind().label();
    let mut row = |field: &str, value: &str| {
        out.push_str(&format!("{},{},{}\n", kind, field, value));
    };
    match trace {
        Trace::Add(t) => {
            row("a", &t.a);
            row("b", &t.b);
            row("result", &t.result);
        }
        Trace::Sub(t) => {
            row("a", &t.a);
            row("b", &t.b);
            row("ones_complement", &t.ones_complement);
            row("radix_complement", &t.radix_complement);
            row("raw_sum", &t.raw_sum);
            row("borrowed", if t.borrowed { "true" } else { "false" });
            row("decode_ones_complement", &t.decode_ones_complement);
            row("result", &t.result);
        }
        Trace::Mul(t) => {
            row("a", &t.a);
            row("b", &t.b);
            for r in &t.rows {
                row(&format!("row{}_digit", r.index), &r.multiplier_digit.to_string());
                row(&format!("row{}_partial", r.index), &r.row);
                row(&format!("row{}_shifted", r.index), &r.shifted);
                row(&format!("row{}_accumulated", r.index), &r.accumulated);
            }
            row("result", &t.result);
        }
    }
    out
}

// ------------------------------------------------------------------- JSON

/// A trace as one line of JSON.
///
/// Hand-written for the same reason `to_json_line` is (`lib.rs`): `serde`
/// stays out of `dz-api`, and the only frontier of this repository that
/// takes it on is `app/src-tauri`. These structures are shallow -- a list
/// of records of `usize` and single glyphs -- so the escaping the
/// hand-written path needs is the one `escape_json` already does.
///
/// Every dozenal value in the output is text produced by `dz-parse`, so a
/// reader (an LLM, a script) can display it but cannot do dozenal
/// arithmetic with it. That is the purity rule at a process boundary, the
/// same property the struct definitions above carry.
pub fn trace_json_line(trace: &Trace) -> String {
    let mut out = String::from("{\"ok\":true,\"kind\":\"");
    out.push_str(trace.kind().label());
    out.push('"');
    match trace {
        Trace::Add(t) => {
            push_field(&mut out, "a", &t.a);
            push_field(&mut out, "b", &t.b);
            out.push_str(",\"steps\":");
            push_steps_json(&mut out, &t.steps);
            push_field(&mut out, "result", &t.result);
        }
        Trace::Sub(t) => {
            push_field(&mut out, "a", &t.a);
            push_field(&mut out, "b", &t.b);
            push_field(&mut out, "ones_complement", &t.ones_complement);
            out.push_str(",\"complement_steps\":");
            push_steps_json(&mut out, &t.complement_steps);
            push_field(&mut out, "radix_complement", &t.radix_complement);
            out.push_str(",\"add_steps\":");
            push_steps_json(&mut out, &t.add_steps);
            push_field(&mut out, "raw_sum", &t.raw_sum);
            out.push_str(&format!(",\"borrowed\":{}", t.borrowed));
            push_field(&mut out, "decode_ones_complement", &t.decode_ones_complement);
            out.push_str(",\"decode_steps\":");
            push_steps_json(&mut out, &t.decode_steps);
            push_field(&mut out, "result", &t.result);
        }
        Trace::Mul(t) => {
            push_field(&mut out, "a", &t.a);
            push_field(&mut out, "b", &t.b);
            out.push_str(",\"rows\":[");
            for (i, row) in t.rows.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push_str(&format!("{{\"index\":{}", row.index));
                push_field(&mut out, "multiplier_digit", &row.multiplier_digit.to_string());
                push_field(&mut out, "row", &row.row);
                push_field(&mut out, "shifted", &row.shifted);
                out.push_str(",\"accumulate_steps\":");
                push_steps_json(&mut out, &row.accumulate_steps);
                push_field(&mut out, "accumulated", &row.accumulated);
                out.push('}');
            }
            out.push(']');
            push_field(&mut out, "result", &t.result);
        }
    }
    out.push('}');
    out
}

/// The three-backend comparison as one line of JSON.
///
/// `table` is reported like the other two but is the oracle, not a
/// production backend (`compare_backends`); the `oracle` flag says so in
/// the document itself, so a reader that never saw `docs/02-crt-vs-mvl.md`
/// cannot read the row as a "which backend wins" answer.
pub fn comparison_json_line(comparison: &BackendComparison) -> String {
    let mut out = String::from("{\"ok\":true");
    push_field(&mut out, "display", &comparison.display);
    out.push_str(",\"costs\":[");
    for (i, cost) in comparison.costs.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            "{{\"backend\":\"{}\",\"oracle\":{},\"gate_count\":{},\"depth\":{},\"reconstruction_ops\":{}}}",
            cost.backend.label(),
            cost.is_oracle(),
            crate::gate_count_json(&cost.gate_count),
            cost.depth,
            cost.reconstruction_ops
        ));
    }
    out.push(']');
    match &comparison.verdict {
        Some(v) => {
            out.push_str(&format!(
                ",\"verdict\":{{\"winner\":\"{}\",\"decided_by_reconstructions\":{}",
                v.winner.label(),
                v.decided_by_reconstructions
            ));
            push_field(&mut out, "model", &v.model);
            push_field(&mut out, "summary", &v.summary);
            out.push('}');
        }
        None => out.push_str(",\"verdict\":null"),
    }
    out.push('}');
    out
}

fn push_field(out: &mut String, name: &str, value: &str) {
    out.push_str(&format!(
        ",\"{}\":\"{}\"",
        name,
        crate::escape_json(value)
    ));
}

fn push_steps_json(out: &mut String, steps: &[TraceStep]) {
    out.push('[');
    for (i, step) in steps.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            "{{\"position\":{},\"a\":\"{}\",\"b\":\"{}\",\"carry_in\":\"{}\",\"sum\":\"{}\",\"carry_out\":\"{}\"}}",
            step.position,
            crate::escape_json(&step.a.to_string()),
            crate::escape_json(&step.b.to_string()),
            crate::escape_json(&step.carry_in.to_string()),
            crate::escape_json(&step.sum.to_string()),
            crate::escape_json(&step.carry_out.to_string())
        ));
    }
    out.push(']');
}
