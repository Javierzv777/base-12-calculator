//! `dz-api` -- Layer 5: stable, interface-agnostic entry point over
//! `dz-eval` (`docs/04-fase6-interfaces.md`, section 2).
//!
//! Every consumer that is not the evaluator itself -- the REPL, the
//! one-shot CLI mode, the Tauri backend -- talks to the calculator through
//! this crate, never by naming `dz_eval::evaluate*` or `dz_num` types
//! directly. That is what lets three interfaces share one implementation
//! of "what a dozenal operation is" instead of duplicating dispatch logic
//! three times.
//!
//! Two properties are deliberate and load-bearing:
//!
//!   - **Stateless.** Every `Request` carries its own backend, display
//!     mode and working scale. Session state (which backend is "active",
//!     history, memory) is a user-interface concept and lives in the
//!     interface -- a one-shot CLI invocation has no session at all.
//!   - **Strings out, never numbers.** A `Response` carries text already
//!     formatted by `dz-parse`, plus structured metadata. Nothing that
//!     crosses this boundary invites arithmetic on the other side, which
//!     is the purity rule applied to a process (or language) boundary.

pub mod research;
pub mod theory;

pub use theory::{
    execute_theory, theory_json_line, Field, Quantifier, TheoryOp, TheoryRequest, TheoryResponse,
    DEFAULT_BOUND_VAR,
};

pub use research::{
    compare_backends, comparison_json_line, gate_count_csv, trace, trace_csv, trace_json_line,
    trace_measured, trace_summary_csv, verdict, AddTrace, BackendComparison, BackendCost, MulRow,
    MulTrace, SubTrace, Trace, TraceKind, TraceRequest, TraceStep, Verdict,
};

use dz_eval::{EvalConfig, EvalError, FnInfo};
use dz_gates::circuits::DepthScope;
use dz_gates::{CrtGates, DozGates, MvlGates, TableGates};

/// Re-exported so consumers can name the type in `Response.gate_count`
/// without depending on Layer 0 themselves.
pub use dz_gates::GateCount;
use dz_parse::{format_dfp, format_fixed, format_rational, parse_str, Expr, ExprParseError};

/// The presentation alphabet (`docs/07-alfabeto-configurable.md`).
///
/// Re-exported so an interface never reaches into `dz-parse` itself, and
/// so the rule that decides "name or number" has exactly one home. Build
/// one with [`display_alphabet`], which supplies the function catalog the
/// validation needs.
pub use dz_parse::{AlphabetError, DisplayAlphabet};

/// The input base (`docs/10-entrada-base-10.md`).
///
/// Re-exported for the same reason [`DisplayAlphabet`] is: an interface
/// names the setting, never the `dz-parse` module that implements it.
/// Unlike the alphabet, this one is not free -- see [`Request::input_base`].
pub use dz_parse::{from_dozenal_text, to_dozenal_text, InputBase, InputBaseError};

/// Rewrite a formatted, canonical dozenal result back into `base`, for
/// result chaining (`docs/09-encadenado-de-resultado.md`).
///
/// This is the entry point an interface calls, so that no interface names
/// `dz-parse` or has to build a backend of its own: the conversion is gate
/// work, and the count of that work comes back alongside the text so the
/// interface can show what chaining cost.
///
/// It is a separate function rather than a field on `Response` because
/// chaining is a *later* user gesture -- the result is displayed first,
/// and only converted if the user types an operator next. Charging every
/// call for a conversion nobody asked for would put a cost in
/// `Response.gate_count` that no arithmetic in the request incurred.
pub fn chain_operand(
    backend: BackendKind,
    base: InputBase,
    display: &str,
) -> Result<(String, GateCount), ApiError> {
    fn convert<G: DozGates>(
        g: &G,
        base: InputBase,
        display: &str,
    ) -> Result<(String, GateCount), ApiError> {
        g.reset_cost();
        let text = from_dozenal_text(g, base, display)?;
        Ok((text, g.cost()))
    }
    match backend {
        BackendKind::Mvl => convert(&MvlGates::new(), base, display),
        BackendKind::Crt => convert(&CrtGates::new(), base, display),
        BackendKind::Table => convert(&TableGates::new(), base, display),
    }
}

/// Which of the three `DozGates` backends a single call uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackendKind {
    Mvl,
    Crt,
    Table,
}

impl BackendKind {
    pub fn parse(name: &str) -> Option<BackendKind> {
        match name {
            "mvl" => Some(BackendKind::Mvl),
            "crt" => Some(BackendKind::Crt),
            "table" => Some(BackendKind::Table),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            BackendKind::Mvl => "mvl",
            BackendKind::Crt => "crt",
            BackendKind::Table => "table",
        }
    }

    pub const ALL: [BackendKind; 3] = [BackendKind::Mvl, BackendKind::Crt, BackendKind::Table];
}

/// Which numeric representation the result is rendered in, mirroring
/// `dz-cli`'s `:mode rational|fixed|dfp`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumberMode {
    Rational,
    Fixed,
    Dfp,
}

impl NumberMode {
    pub fn parse(name: &str) -> Option<NumberMode> {
        match name {
            "rational" => Some(NumberMode::Rational),
            "fixed" => Some(NumberMode::Fixed),
            "dfp" => Some(NumberMode::Dfp),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            NumberMode::Rational => "rational",
            NumberMode::Fixed => "fixed",
            NumberMode::Dfp => "dfp",
        }
    }

    pub const ALL: [NumberMode; 3] = [NumberMode::Rational, NumberMode::Fixed, NumberMode::Dfp];
}

/// The named operations an interface can ask for without building an AST.
///
/// The four arithmetic operations get their own variants so `base12 sum 1 3`
/// does not have to go through the expression grammar; `Call` reaches the
/// function registry by name (this is how a GUI's `sin` key works); `Eval`
/// is the escape hatch for a full expression, reusing `dz_parse::parse_str`
/// exactly as the REPL does.
///
/// Operands are text, not numbers: they are parsed by `dz-parse`, the one
/// component allowed to turn characters into digits. Any expression is
/// accepted where an operand is expected, so `sum "1+1" "2"` works -- a
/// superset of "a literal", and one code path either way.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operation {
    Sum { a: String, b: String },
    Sub { a: String, b: String },
    Mul { a: String, b: String },
    Div { a: String, b: String },
    Mod { a: String, b: String },
    Pow { base: String, exponent: String },
    Neg { a: String },
    /// A registry function or constant: `Call { name: "sin", args: ["1"] }`.
    Call { name: String, args: Vec<String> },
    /// A full expression, e.g. `"4;6 * 3"` or `"sin(pi/4)"`.
    Eval { expr: String },
}

/// One request: self-describing, with no hidden session state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub backend: BackendKind,
    pub mode: NumberMode,
    /// Working precision for functions that cannot be exact (`sin`, `ln`).
    /// Ignored by the exact ones.
    pub scale: usize,
    /// Which presentation alphabet the operand *text* of `op` is written
    /// in. Purely about the incoming characters: it is applied before the
    /// lexer and never reaches any layer below `dz-parse`.
    ///
    /// `Response.display` comes back in the **canonical** alphabet
    /// regardless; an interface renders it through
    /// [`DisplayAlphabet::from_canonical`] at paint time. That split is
    /// deliberate: history, CSV and JSON keep canonical text, so changing
    /// the alphabet repaints old results instead of leaving a mixed list
    /// (`docs/07` sections 3.4 and 3.5).
    pub alphabet: DisplayAlphabet,
    /// Which base the operand *text* of `op` is written in. Input only, in
    /// exactly the sense `alphabet` is: `Response.display` is always
    /// dozenal, because converting back would be a second, unmeasured
    /// arithmetic path and the project has none.
    ///
    /// Unlike the alphabet, this setting is **not free**. A dozenal
    /// literal costs nothing to read -- the digit's position in the
    /// magnitude already is its weight -- while a base-ten literal has to
    /// have its weights multiplied out with gates
    /// (`dz_int::from_base10_digits`). That cost is deliberately inside
    /// the measured window, so `Response.gate_count` includes it.
    pub input_base: InputBase,
    pub op: Operation,
}

impl Request {
    /// Defaults matching the REPL: MVL backend, rational mode, the
    /// standard scientific working scale.
    pub fn new(op: Operation) -> Request {
        Request {
            backend: BackendKind::Mvl,
            mode: NumberMode::Rational,
            scale: dz_sci::DEFAULT_SCI_SCALE,
            alphabet: DisplayAlphabet::canonical(),
            input_base: InputBase::Twelve,
            op,
        }
    }

    pub fn with_backend(mut self, backend: BackendKind) -> Request {
        self.backend = backend;
        self
    }

    pub fn with_mode(mut self, mode: NumberMode) -> Request {
        self.mode = mode;
        self
    }

    pub fn with_scale(mut self, scale: usize) -> Request {
        self.scale = scale;
        self
    }

    pub fn with_alphabet(mut self, alphabet: DisplayAlphabet) -> Request {
        self.alphabet = alphabet;
        self
    }

    pub fn with_input_base(mut self, input_base: InputBase) -> Request {
        self.input_base = input_base;
        self
    }
}

/// Build a presentation alphabet, validated against the live function
/// catalog.
///
/// The catalog check (`docs/07` section 3.2, rule 3) is why this wrapper
/// exists: `DisplayAlphabet::new` deliberately takes the reserved names as
/// a parameter, because the registry lives in `dz-eval`, two layers above
/// `dz-parse`. This is the layer where both are in scope, so this is the
/// constructor every interface should call.
pub fn display_alphabet(ten: char, eleven: char) -> Result<DisplayAlphabet, AlphabetError> {
    let names: Vec<String> = functions().into_iter().map(|f| f.name.to_string()).collect();
    DisplayAlphabet::new(ten, eleven, &names)
}

/// Everything a caller needs, in one value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    /// The formatted value, in canonical dozenal glyphs (`X`/`E` shown as
    /// U+218A/U+218B). Interfaces that need ASCII call [`to_ascii`] --
    /// `dz-parse` stays the single formatting authority.
    pub display: String,
    /// Set in `Rational` mode when the exact value does not terminate in
    /// base twelve. Structured, so a GUI renders its own badge instead of
    /// scraping a substring out of `display`.
    pub periodic_note: bool,
    /// Set when any part of the expression was computed by an approximate
    /// function (`sin`, `ln`, an irrational `sqrt`). `display` is then
    /// prefixed with `~`, and in rational mode the value is shown in
    /// fixed-point form -- printing a twelve-digit fraction for `sin(1)`
    /// would claim an exactness that is not there.
    pub approximate: bool,
    /// Gate invocations for this call, counted from a backend reset
    /// immediately before dispatch. Populated on every successful call,
    /// for the Phase 7 research panel; `None` when the operation failed
    /// before any gate ran (a parse error).
    pub gate_count: Option<GateCount>,
    /// Circuit depth of the same run: the longest chain of primitive
    /// invocations from a primary input to any output, across the *whole*
    /// expression rather than one isolated circuit.
    ///
    /// Measured in the same pass as `gate_count` and without changing it:
    /// the depth-carrying instantiation invokes the backend once per
    /// primitive, exactly as the plain one does. See
    /// `dz_gates::circuits::depth` for the model and `dz_int::dzint` for
    /// how a `Dz` carries its own arrival level, which is what makes depth
    /// compose across `dz-num`/`dz-sci` without those crates knowing.
    pub depth: Option<u32>,
    /// Backend-specific overhead the five primitive counters cannot see:
    /// full internal-representation -> tag reconstructions
    /// (`DozGates::reconstruction_ops`). `0` for MVL and for the table
    /// oracle -- that is the truth, not a missing measurement -- and
    /// non-zero only for CRT, whose every order comparison rebuilds the
    /// digit from its residues (`docs/02-crt-vs-mvl.md`).
    ///
    /// A sibling field rather than a sixth `GateCount` counter, on
    /// purpose: `GateCount` is what stays equal across backends by
    /// construction, and this is what explains why equal counters are not
    /// equal cost.
    pub reconstruction_ops: Option<u64>,
}

/// Structured errors -- never a bare string -- so a GUI can style them and
/// a CLI can map them to distinct exit codes.
#[derive(Debug)]
pub enum ApiError {
    Parse(ExprParseError),
    Eval(EvalError),
    /// A research-panel request that is well-formed text but not something
    /// a single circuit run can represent (see `research::trace`).
    Trace(String),
    /// The request's text could not be read in the configured
    /// presentation alphabet (`docs/07`): a legacy alias that this
    /// alphabet does not spell, most often.
    Alphabet(AlphabetError),
    /// The request's text could not be read in the configured input base
    /// (`docs/10`): a dozenal-only digit typed while the base is ten, most
    /// often.
    InputBase(InputBaseError),
    /// A number-theory request (`theory`) that is well-formed text but not
    /// something it can answer: a non-integer operand, `valuation` of zero,
    /// a bad bound-variable name, an approximate quantifier body.
    Theory(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Parse(e) => write!(f, "{}", e),
            ApiError::Eval(e) => write!(f, "{}", e),
            ApiError::Trace(message) => write!(f, "{}", message),
            ApiError::Alphabet(e) => write!(f, "{}", e),
            ApiError::InputBase(e) => write!(f, "{}", e),
            ApiError::Theory(message) => write!(f, "{}", message),
        }
    }
}
impl std::error::Error for ApiError {}

impl From<AlphabetError> for ApiError {
    fn from(e: AlphabetError) -> ApiError {
        ApiError::Alphabet(e)
    }
}

impl From<InputBaseError> for ApiError {
    fn from(e: InputBaseError) -> ApiError {
        ApiError::InputBase(e)
    }
}

impl From<ExprParseError> for ApiError {
    fn from(e: ExprParseError) -> ApiError {
        ApiError::Parse(e)
    }
}

impl From<EvalError> for ApiError {
    fn from(e: EvalError) -> ApiError {
        ApiError::Eval(e)
    }
}

/// The catalog of callable functions and constants: name, arity, category
/// and description. Interfaces build their keypads and help text from
/// this, so registering a function in `dz-eval` is enough to make it
/// appear everywhere.
pub fn functions() -> Vec<FnInfo> {
    dz_eval::default_registry().catalog()
}

/// Fractional digits kept by division for a requested working `scale`:
/// `max(DEFAULT_DIV_SCALE, scale)`. Division follows `--scale` only when the
/// user asks for more than the historical 24 digits, so every number
/// published for the default scale is unchanged.
pub fn division_scale(scale: usize) -> usize {
    scale.max(dz_num::DEFAULT_DIV_SCALE)
}

/// The one entry point. Builds the requested backend fresh (so a call's
/// gate cost is measured in isolation, which is what a backend comparison
/// wants), dispatches, formats per `mode`, and returns the cost alongside
/// the result.
pub fn execute(req: &Request) -> Result<Response, ApiError> {
    match req.backend {
        BackendKind::Mvl => execute_with(&MvlGates::new(), req),
        BackendKind::Crt => execute_with(&CrtGates::new(), req),
        BackendKind::Table => execute_with(&TableGates::new(), req),
    }
}

/// Parse one piece of operand text, first rewriting it out of the user's
/// input base and then out of their presentation alphabet.
///
/// Both translations are `dz-parse` calls, not rules re-derived here: this
/// layer knows *when* to translate, `dz-parse` knows *how*.
///
/// The two settings do not compose in the obvious way, and the asymmetry
/// is deliberate. In base ten no letter is a digit, so a run of letters is
/// unambiguously a name and there is nothing for the presentation alphabet
/// to translate -- applying it would only give the user a second, useless
/// way to spell digits that base ten does not have. In base twelve the
/// input-base rewrite is the identity and costs no gate, so the pipeline
/// is exactly what it was.
fn operand<G: DozGates>(
    g: &G,
    alphabet: &DisplayAlphabet,
    input_base: InputBase,
    text: &str,
) -> Result<Expr, ApiError> {
    let text = to_dozenal_text(g, input_base, text)?;
    let canonical = if input_base.is_dozenal() {
        alphabet.to_canonical(&text)?
    } else {
        text
    };
    Ok(parse_str(&canonical)?)
}

/// Turn a named operation into the AST the evaluator already understands,
/// so there is exactly one evaluation path regardless of how the request
/// arrived.
///
/// A `Call`'s *name* is never translated: it comes from the registry
/// catalog, not from the user's keyboard, and is always spelled the way
/// `dz-eval` registered it.
fn build_expr<G: DozGates>(
    g: &G,
    alphabet: &DisplayAlphabet,
    input_base: InputBase,
    op: &Operation,
) -> Result<Expr, ApiError> {
    let expr = match op {
        Operation::Sum { a, b } => Expr::Add(
            Box::new(operand(g, alphabet, input_base, a)?),
            Box::new(operand(g, alphabet, input_base, b)?),
        ),
        Operation::Sub { a, b } => Expr::Sub(
            Box::new(operand(g, alphabet, input_base, a)?),
            Box::new(operand(g, alphabet, input_base, b)?),
        ),
        Operation::Mul { a, b } => Expr::Mul(
            Box::new(operand(g, alphabet, input_base, a)?),
            Box::new(operand(g, alphabet, input_base, b)?),
        ),
        Operation::Div { a, b } => Expr::Div(
            Box::new(operand(g, alphabet, input_base, a)?),
            Box::new(operand(g, alphabet, input_base, b)?),
        ),
        Operation::Mod { a, b } => Expr::Mod(
            Box::new(operand(g, alphabet, input_base, a)?),
            Box::new(operand(g, alphabet, input_base, b)?),
        ),
        Operation::Pow { base, exponent } => Expr::Pow(
            Box::new(operand(g, alphabet, input_base, base)?),
            Box::new(operand(g, alphabet, input_base, exponent)?),
        ),
        Operation::Neg { a } => Expr::Neg(Box::new(operand(g, alphabet, input_base, a)?)),
        Operation::Call { name, args } => {
            let mut parsed = Vec::with_capacity(args.len());
            for argument in args {
                parsed.push(operand(g, alphabet, input_base, argument)?);
            }
            Expr::Call {
                name: name.clone(),
                args: parsed,
            }
        }
        Operation::Eval { expr } => operand(g, alphabet, input_base, expr)?,
    };
    Ok(expr)
}

fn execute_with<G: DozGates>(g: &G, req: &Request) -> Result<Response, ApiError> {
    // The measurement window opens *before* the text is read, and that
    // placement is the whole point of the base-ten phase.
    //
    // It used to open after `build_expr`, on the reasoning that a caller
    // reading `gate_count` wants the cost of the arithmetic and not of
    // parsing. That reasoning assumed parsing is not arithmetic -- true
    // in base twelve, where a literal's digits are already its magnitude
    // and the lexer is a `Vec::push`, and false in base ten, where every
    // weight has to be multiplied out with real gates
    // (`dz_int::from_base10_digits`). Leaving the window where it was
    // would have reported the interesting cost as zero.
    //
    // Moving it changes no published number, and that is checked rather
    // than asserted: in base twelve `build_expr` still invokes no
    // primitive, so `1+1` stays at 24,083 gates and depth 441, and
    // `examples/gate_counts` prints byte-identical output.
    //
    // Depth is measured in the *same* pass, not a second one. The
    // depth-carrying instantiation of the circuits invokes the backend
    // once per primitive with the same values, so `GateCount` and control
    // flow are bit-identical to a run with tracking off; only `u32`
    // bookkeeping rides alongside. `dz-api/tests/depth.rs` pins that.
    let scope = DepthScope::enter();
    g.reset_cost();

    let expr = build_expr(g, &req.alphabet, req.input_base, &req.op)?;
    let config = EvalConfig {
        registry: dz_eval::default_registry(),
        scale: req.scale,
        div_scale: division_scale(req.scale),
    };

    let (display, periodic_note, approximate) = match req.mode {
        NumberMode::Rational => {
            let out = dz_eval::evaluate_rational_with(g, &expr, &config)?;
            if out.approximate {
                // An approximation has no meaningful exact fraction: show
                // the fixed-point form of what was actually computed.
                let fixed = out.value.to_fixed(g, req.scale);
                (format_fixed(&fixed), false, true)
            } else {
                let periodic = !out.value.is_zero() && !out.value.terminates_in_base12(g);
                (format_rational(&out.value), periodic, false)
            }
        }
        NumberMode::Fixed => {
            let out = dz_eval::evaluate_fixed_with(g, &expr, &config)?;
            // Termination is decided on the *exact* value, so the warning
            // is never a guess from the truncated output. An approximate
            // expression has no exact value to ask about.
            let periodic = if out.approximate {
                false
            } else {
                match dz_eval::evaluate_rational_with(g, &expr, &config) {
                    Ok(exact) => !exact.value.is_zero() && !exact.value.terminates_in_base12(g),
                    Err(_) => false,
                }
            };
            (format_fixed(&out.value), periodic, out.approximate)
        }
        NumberMode::Dfp => {
            let out = dz_eval::evaluate_dfp_with(g, &expr, &config)?;
            (format_dfp(g, &out.value), false, out.approximate)
        }
    };

    let display = if approximate {
        let mut marked = String::with_capacity(display.len() + 1);
        marked.push('~');
        marked.push_str(&display);
        marked
    } else {
        display
    };

    Ok(Response {
        display,
        periodic_note,
        approximate,
        gate_count: Some(g.cost()),
        depth: Some(scope.measured()),
        reconstruction_ops: Some(g.reconstruction_ops()),
    })
}

/// Replace the dozenal glyphs with their ASCII aliases (`X` for ten, `E`
/// for eleven), for terminals, pipes and log files where U+218A/U+218B may
/// not render. Input parsing already accepts both spellings
/// (`dz_parse::alphabet`), so this is the mirror of an alias that exists.
pub fn to_ascii(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '\u{218A}' => 'X',
            '\u{218B}' => 'E',
            other => other,
        })
        .collect()
}

pub(crate) fn escape_json(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out
}

pub(crate) fn gate_count_json(count: &GateCount) -> String {
    format!(
        "{{\"cycle\":{},\"min\":{},\"max\":{},\"lit\":{},\"thr\":{},\"total\":{}}}",
        count.cycle,
        count.min,
        count.max,
        count.lit,
        count.thr,
        count.total()
    )
}

/// A successful response as one line of JSON.
///
/// Serialized by hand on purpose: the object has five fields of primitive
/// types, and `docs/04-fase6-interfaces.md` section 3.3 records the
/// decision not to take on `serde` until a consumer needs nested
/// structures (the Phase 7 trace viewer would be that consumer).
pub fn to_json_line(response: &Response) -> String {
    let gates = match &response.gate_count {
        Some(count) => gate_count_json(count),
        None => "null".to_string(),
    };
    let depth = match response.depth {
        Some(levels) => levels.to_string(),
        None => "null".to_string(),
    };
    let reconstructions = match response.reconstruction_ops {
        Some(ops) => ops.to_string(),
        None => "null".to_string(),
    };
    format!(
        "{{\"ok\":true,\"display\":\"{}\",\"periodic_note\":{},\"approximate\":{},\"gate_count\":{},\"depth\":{},\"reconstruction_ops\":{}}}",
        escape_json(&response.display),
        response.periodic_note,
        response.approximate,
        gates,
        depth,
        reconstructions
    )
}

/// An error as one line of JSON, in the same shape.
pub fn error_json_line(error: &ApiError) -> String {
    format!(
        "{{\"ok\":false,\"error\":\"{}\"}}",
        escape_json(&error.to_string())
    )
}

/// The default working scale (fractional dozenal digits) for functions
/// that cannot be exact. Exposed so an interface can show and adjust it
/// without depending on `dz-sci` directly.
pub fn default_scale() -> usize {
    dz_sci::DEFAULT_SCI_SCALE
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(text: &str, mode: NumberMode) -> Response {
        execute(&Request::new(Operation::Eval { expr: text.to_string() }).with_mode(mode))
            .expect("expression evaluates")
    }

    #[test]
    fn named_operations_match_the_equivalent_expression() {
        // The point of the named variants: `sum 1 3` must be the same
        // computation as evaluating "1 + 3", not a parallel one.
        for mode in NumberMode::ALL {
            let named = execute(
                &Request::new(Operation::Sum { a: "4;6".into(), b: "3".into() }).with_mode(mode),
            )
            .expect("sum evaluates");
            let expression = eval("4;6 + 3", mode);
            assert_eq!(named.display, expression.display);
        }
    }

    #[test]
    fn documented_repl_example_still_holds() {
        // CLAUDE.md's headline example: 4;6 x 3 = 11;6.
        let response = execute(
            &Request::new(Operation::Mul { a: "4;6".into(), b: "3".into() })
                .with_mode(NumberMode::Fixed),
        )
        .expect("multiplication evaluates");
        assert_eq!(response.display, "11;6");
        assert!(!response.approximate);
    }

    #[test]
    fn rational_mode_is_exact_and_flags_periodicity() {
        let third = eval("1/3", NumberMode::Rational);
        assert_eq!(third.display, "1/3");
        assert!(!third.periodic_note, "1/3 terminates in base twelve");

        let fifth = eval("1/5", NumberMode::Rational);
        assert_eq!(fifth.display, "1/5");
        assert!(fifth.periodic_note, "1/5 does not terminate in base twelve");
    }

    #[test]
    fn approximate_results_are_marked_and_shown_as_digits() {
        let response = execute(
            &Request::new(Operation::Call { name: "sin".into(), args: vec!["1".into()] })
                .with_scale(6),
        )
        .expect("sin evaluates");
        assert!(response.approximate);
        assert!(response.display.starts_with('~'), "got {}", response.display);
        assert!(
            !response.display.contains('/'),
            "an approximation must not be printed as a fraction: {}",
            response.display
        );
    }

    #[test]
    fn every_call_reports_a_gate_count() {
        let response = eval("1 + 1", NumberMode::Fixed);
        let count = response.gate_count.expect("a successful call always counts gates");
        assert!(count.total() > 0);
    }

    #[test]
    fn a_parse_error_is_structured() {
        let error = execute(&Request::new(Operation::Eval { expr: "1 +".into() }))
            .expect_err("incomplete expression must fail");
        assert!(matches!(error, ApiError::Parse(_)));
    }

    #[test]
    fn division_by_zero_is_an_eval_error() {
        let error = execute(&Request::new(Operation::Div { a: "1".into(), b: "0".into() }))
            .expect_err("division by zero must fail");
        assert!(matches!(error, ApiError::Eval(EvalError::DivisionByZero)));
    }

    #[test]
    fn all_three_backends_agree_on_the_answer() {
        for backend in BackendKind::ALL {
            let response = execute(
                &Request::new(Operation::Eval { expr: "4;6 * 3".into() })
                    .with_backend(backend)
                    .with_mode(NumberMode::Fixed),
            )
            .expect("evaluates");
            assert_eq!(response.display, "11;6", "backend {}", backend.label());
        }
    }

    #[test]
    fn json_output_is_well_formed() {
        let response = eval("1 + 1", NumberMode::Fixed);
        let line = to_json_line(&response);
        assert!(line.starts_with("{\"ok\":true,\"display\":\"2\""), "{}", line);
        assert!(line.contains("\"total\":"));
    }

    #[test]
    fn ascii_aliases_replace_the_glyphs() {
        let response = eval("↊ + ↋", NumberMode::Fixed);
        assert_eq!(response.display, "19");
        let eleven = eval("↋", NumberMode::Fixed);
        assert_eq!(to_ascii(&eleven.display), "E");
    }

    #[test]
    fn the_function_catalog_is_not_empty_and_has_trigonometry() {
        let catalog = functions();
        assert!(catalog.iter().any(|f| f.name == "sin"));
        assert!(catalog.iter().any(|f| f.name == "pi" && f.arity.max == 0));
    }
}
