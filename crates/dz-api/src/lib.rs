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

use dz_eval::{EvalConfig, EvalError, FnInfo};
use dz_gates::{CrtGates, DozGates, GateCount, MvlGates, TableGates};
use dz_parse::{format_dfp, format_fixed, format_rational, parse_str, Expr, ExprParseError};

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
}

/// Structured errors -- never a bare string -- so a GUI can style them and
/// a CLI can map them to distinct exit codes.
#[derive(Debug)]
pub enum ApiError {
    Parse(ExprParseError),
    Eval(EvalError),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Parse(e) => write!(f, "{}", e),
            ApiError::Eval(e) => write!(f, "{}", e),
        }
    }
}
impl std::error::Error for ApiError {}

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

fn operand(text: &str) -> Result<Expr, ApiError> {
    Ok(parse_str(text)?)
}

/// Turn a named operation into the AST the evaluator already understands,
/// so there is exactly one evaluation path regardless of how the request
/// arrived.
fn build_expr(op: &Operation) -> Result<Expr, ApiError> {
    let expr = match op {
        Operation::Sum { a, b } => Expr::Add(Box::new(operand(a)?), Box::new(operand(b)?)),
        Operation::Sub { a, b } => Expr::Sub(Box::new(operand(a)?), Box::new(operand(b)?)),
        Operation::Mul { a, b } => Expr::Mul(Box::new(operand(a)?), Box::new(operand(b)?)),
        Operation::Div { a, b } => Expr::Div(Box::new(operand(a)?), Box::new(operand(b)?)),
        Operation::Mod { a, b } => Expr::Mod(Box::new(operand(a)?), Box::new(operand(b)?)),
        Operation::Pow { base, exponent } => {
            Expr::Pow(Box::new(operand(base)?), Box::new(operand(exponent)?))
        }
        Operation::Neg { a } => Expr::Neg(Box::new(operand(a)?)),
        Operation::Call { name, args } => {
            let mut parsed = Vec::with_capacity(args.len());
            for argument in args {
                parsed.push(operand(argument)?);
            }
            Expr::Call { name: name.clone(), args: parsed }
        }
        Operation::Eval { expr } => operand(expr)?,
    };
    Ok(expr)
}

fn execute_with<G: DozGates>(g: &G, req: &Request) -> Result<Response, ApiError> {
    let expr = build_expr(&req.op)?;
    let config = EvalConfig {
        registry: dz_eval::default_registry(),
        scale: req.scale,
    };

    // Everything before this point is text handling; the measurement
    // starts here so a caller reading `gate_count` sees the cost of the
    // arithmetic, not of parsing.
    g.reset_cost();

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

fn escape_json(text: &str) -> String {
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

fn gate_count_json(count: &GateCount) -> String {
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
    format!(
        "{{\"ok\":true,\"display\":\"{}\",\"periodic_note\":{},\"approximate\":{},\"gate_count\":{}}}",
        escape_json(&response.display),
        response.periodic_note,
        response.approximate,
        gates
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
