//! Integer number-theory surface of `dz-api`: `divrem`, `valuation`,
//! `divisible`, and bounded `forall`/`exists`
//! (`Investigations/12-hallazgo-godel-incompletitud.md`, sections 6 and 10).
//!
//! These return *several* named values (a quotient **and** a remainder), so
//! they do not fit `Response.display`, which is one formatted number. The
//! response is a list of named text fields plus the same measurements
//! `Response` carries. As everywhere in this crate, every dozenal figure
//! crosses the boundary as text already formatted by `dz-parse`.
//!
//! # The bounded quantifiers
//!
//! `forall`/`exists` evaluate a body for a bound variable running over
//! `0, 1, ..., N-1`. They stay **bounded, hence decidable**: the loop
//! always ends after at most `N` candidates. There is deliberately no
//! unbounded form (that would be the mu-operator, the ingredient that makes
//! Goedel's theorems apply -- section 2.3 of the investigation).
//!
//! * The counter is a `Dz` advanced by gate-composed `add`; the loop ends
//!   when a gate-composed `compare` against `N` says so. No native integer
//!   ever stands for a dozenal quantity.
//! * Each candidate is the body's syntax tree with the variable replaced by
//!   the counter's digits (a literal node), then evaluated exactly in
//!   rational mode by the ordinary evaluator. An approximate body is an
//!   error: a quantifier needs a decision, and a truncated `sin` is not one
//!   (investigation section 2.2).
//! * The body is an expression that **holds when it is nonzero**, or a
//!   single relation `lhs = rhs`, `!=`, `<`, `<=`, `>`, `>=` between two
//!   expressions, decided by gate-composed comparison.
//! * One backend, one cost window: `gate_count` is the aggregate over every
//!   candidate that ran, short-circuiting included.
//! * `depth` is the **largest depth of a single candidate** (each candidate
//!   gets its own `DepthScope`, and the counter enters it as a primary
//!   input). That is the depth of the loop unrolled in parallel; the
//!   AND/OR tree that would combine the `N` verdicts is not counted.

use crate::{operand, ApiError, BackendKind, GateCount};
use dz_eval::{EvalConfig, EvalError};
use dz_gates::circuits::DepthScope;
use dz_gates::{CrtGates, Digit, DozGates, MvlGates, TableGates};
use dz_int::{DivisibilityMethod, Dz};
use dz_num::DzRational;
use dz_parse::{format_rational, lex, DisplayAlphabet, Expr, InputBase, Token};
use std::cmp::Ordering;

/// The default name of the bound variable. A single letter that is not a
/// digit alias (`x`, `t`, `e` and `X`, `T`, `E` are digits), so the lexer
/// reads it as a name, and that is not a catalog function.
pub const DEFAULT_BOUND_VAR: &str = "i";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quantifier {
    Forall,
    Exists,
}

impl Quantifier {
    pub fn label(&self) -> &'static str {
        match self {
            Quantifier::Forall => "forall",
            Quantifier::Exists => "exists",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TheoryOp {
    /// Quotient and remainder from one `div_rem` pass (truncating: the
    /// remainder takes the sign of the dividend).
    DivRem { a: String, b: String },
    /// `v_p(n)`: the largest `k` with `p^k | n`.
    Valuation { n: String, p: String },
    /// `d | n`, by the last digit when `d` divides twelve, else by `div_rem`.
    Divisible { n: String, d: String },
    /// `forall`/`exists var < bound . body`.
    Bounded { quantifier: Quantifier, bound: String, var: String, body: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TheoryRequest {
    pub backend: BackendKind,
    pub alphabet: DisplayAlphabet,
    pub input_base: InputBase,
    pub op: TheoryOp,
}

impl TheoryRequest {
    pub fn new(op: TheoryOp) -> TheoryRequest {
        TheoryRequest {
            backend: BackendKind::Mvl,
            alphabet: DisplayAlphabet::canonical(),
            input_base: InputBase::Twelve,
            op,
        }
    }
    pub fn with_backend(mut self, backend: BackendKind) -> TheoryRequest {
        self.backend = backend;
        self
    }
    pub fn with_alphabet(mut self, alphabet: DisplayAlphabet) -> TheoryRequest {
        self.alphabet = alphabet;
        self
    }
    pub fn with_input_base(mut self, input_base: InputBase) -> TheoryRequest {
        self.input_base = input_base;
        self
    }
}

/// One named output. `raw` marks a JSON literal (a boolean) as opposed to
/// a string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub name: &'static str,
    pub value: String,
    pub raw: bool,
}

impl Field {
    fn text(name: &'static str, value: String) -> Field {
        Field { name, value, raw: false }
    }
    fn flag(name: &'static str, value: bool) -> Field {
        Field { name, value: value.to_string(), raw: true }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TheoryResponse {
    /// Canonical dozenal text for numbers, `true`/`false` for flags.
    pub fields: Vec<Field>,
    pub gate_count: GateCount,
    pub depth: u32,
    pub reconstruction_ops: u64,
}

impl TheoryResponse {
    pub fn field(&self, name: &str) -> Option<&str> {
        self.fields.iter().find(|f| f.name == name).map(|f| f.value.as_str())
    }
}

/// Run one request on a freshly built backend.
pub fn execute_theory(req: &TheoryRequest) -> Result<TheoryResponse, ApiError> {
    match req.backend {
        BackendKind::Mvl => run(&MvlGates::new(), req),
        BackendKind::Crt => run(&CrtGates::new(), req),
        BackendKind::Table => run(&TableGates::new(), req),
    }
}

fn int_text(n: &Dz) -> String {
    format_rational(&DzRational::from_int(n.clone()))
}

/// An operand that must denote an exact integer.
fn integer_operand<G: DozGates>(
    g: &G,
    req: &TheoryRequest,
    text: &str,
    what: &str,
) -> Result<Dz, ApiError> {
    let expr = operand(g, &req.alphabet, req.input_base, text)?;
    let out = dz_eval::evaluate_rational_with(g, &expr, &EvalConfig::defaults())?;
    if out.approximate {
        return Err(ApiError::Theory(format!("{} is approximate; an integer is needed", what)));
    }
    if out.value.den_digits() != [Digit::ONE] {
        return Err(ApiError::Theory(format!("{} must be an integer", what)));
    }
    Ok(out.value.num)
}

fn run<G: DozGates>(g: &G, req: &TheoryRequest) -> Result<TheoryResponse, ApiError> {
    // Same window rule as `execute_with`: open it before the text is read,
    // so reading a base-ten operand is billed.
    g.reset_cost();
    match &req.op {
        TheoryOp::Bounded { quantifier, bound, var, body } => {
            run_bounded(g, req, *quantifier, bound, var, body)
        }
        other => {
            let scope = DepthScope::enter();
            let fields = match other {
                TheoryOp::DivRem { a, b } => {
                    let a = integer_operand(g, req, a, "the dividend")?;
                    let b = integer_operand(g, req, b, "the divisor")?;
                    let (q, r) = a.div_rem(g, &b).map_err(EvalError::from)?;
                    vec![
                        Field::text("quotient", int_text(&q)),
                        Field::text("remainder", int_text(&r)),
                    ]
                }
                TheoryOp::Valuation { n, p } => {
                    let n = integer_operand(g, req, n, "the number")?;
                    let p = integer_operand(g, req, p, "the divisor")?;
                    let v = dz_int::valuation(g, &n, &p)
                        .map_err(|e| ApiError::Theory(e.to_string()))?;
                    vec![
                        Field::text("exponent", int_text(&v.exponent)),
                        Field::text("cofactor", int_text(&v.cofactor)),
                    ]
                }
                TheoryOp::Divisible { n, d } => {
                    let n = integer_operand(g, req, n, "the number")?;
                    let d = integer_operand(g, req, d, "the divisor")?;
                    let r = dz_int::divisible(g, &n, &d)
                        .map_err(|e| ApiError::Theory(e.to_string()))?;
                    let method = match r.method {
                        DivisibilityMethod::LastDigit => "last-digit",
                        DivisibilityMethod::DivRem => "div_rem",
                    };
                    vec![
                        Field::flag("divisible", r.divisible),
                        Field::text("method", method.to_string()),
                    ]
                }
                TheoryOp::Bounded { .. } => unreachable!("handled above"),
            };
            Ok(TheoryResponse {
                fields,
                gate_count: g.cost(),
                depth: scope.measured(),
                reconstruction_ops: g.reconstruction_ops(),
            })
        }
    }
}

// ------------------------------------------------------------ the quantifier

#[derive(Clone, Copy, PartialEq, Eq)]
enum Relation {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/// Split `body` at its single relation, if any. The characters `= < > !`
/// are not part of the expression grammar, so the first one found is
/// unambiguous.
fn split_relation(body: &str) -> Result<(String, Option<(Relation, String)>), ApiError> {
    let chars: Vec<char> = body.chars().collect();
    let mut i: usize = 0;
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        let found = match (c, next) {
            ('<', Some('=')) => Some((Relation::Le, 2usize)),
            ('>', Some('=')) => Some((Relation::Ge, 2)),
            ('!', Some('=')) => Some((Relation::Ne, 2)),
            ('<', _) => Some((Relation::Lt, 1)),
            ('>', _) => Some((Relation::Gt, 1)),
            ('=', _) => Some((Relation::Eq, 1)),
            _ => None,
        };
        if let Some((relation, width)) = found {
            let left: String = chars[..i].iter().collect();
            let right: String = chars[i + width..].iter().collect();
            if right.chars().any(|ch| matches!(ch, '=' | '<' | '>' | '!')) {
                return Err(ApiError::Theory(
                    "a body may contain only one relation".to_string(),
                ));
            }
            if left.trim().is_empty() || right.trim().is_empty() {
                return Err(ApiError::Theory(
                    "a relation needs an expression on both sides".to_string(),
                ));
            }
            return Ok((left, Some((relation, right))));
        }
        i += 1;
    }
    Ok((body.to_string(), None))
}

/// The variable must lex as exactly one name, survive the configured
/// presentation alphabet untouched, and not be a catalog function.
fn check_variable(req: &TheoryRequest, var: &str) -> Result<(), ApiError> {
    let reject = |why: &str| ApiError::Theory(format!("bad variable name '{}': {}", var, why));
    let tokens = lex(var).map_err(|_| reject("not a plain name"))?;
    if !matches!(tokens.as_slice(), [Token::Ident(name)] if name == var) {
        return Err(reject(
            "it must be a run of letters that is a name, not a number (x, t, e, X, T, E are digits)",
        ));
    }
    if req.input_base.is_dozenal() {
        let canonical = req.alphabet.to_canonical(var)?;
        if canonical != var {
            return Err(reject("the configured alphabet reads it as digits"));
        }
    }
    if crate::functions().iter().any(|f| f.name == var) {
        return Err(reject("it is a function or constant name"));
    }
    Ok(())
}

/// The body's tree with every bare `var` replaced by the counter's digits.
/// A pure `Vec` rewrite; `int_digits` run most significant first.
fn substitute(expr: &Expr, var: &str, digits_msd: &[Digit]) -> Expr {
    let go = |e: &Expr| Box::new(substitute(e, var, digits_msd));
    match expr {
        Expr::Call { name, args } if name == var && args.is_empty() => Expr::Number {
            int_digits: digits_msd.to_vec(),
            frac_digits: Vec::new(),
        },
        Expr::Call { name, args } => Expr::Call {
            name: name.clone(),
            args: args.iter().map(|a| substitute(a, var, digits_msd)).collect(),
        },
        Expr::Number { .. } => expr.clone(),
        Expr::Neg(a) => Expr::Neg(go(a)),
        Expr::Add(a, b) => Expr::Add(go(a), go(b)),
        Expr::Sub(a, b) => Expr::Sub(go(a), go(b)),
        Expr::Mul(a, b) => Expr::Mul(go(a), go(b)),
        Expr::Div(a, b) => Expr::Div(go(a), go(b)),
        Expr::Mod(a, b) => Expr::Mod(go(a), go(b)),
        Expr::Pow(a, b) => Expr::Pow(go(a), go(b)),
    }
}

fn exact_value<G: DozGates>(g: &G, expr: &Expr) -> Result<DzRational, ApiError> {
    let out = dz_eval::evaluate_rational_with(g, expr, &EvalConfig::defaults())?;
    if out.approximate {
        return Err(ApiError::Theory(
            "the body is approximate (a truncated series); a quantifier needs an exact decision"
                .to_string(),
        ));
    }
    Ok(out.value)
}

fn run_bounded<G: DozGates>(
    g: &G,
    req: &TheoryRequest,
    quantifier: Quantifier,
    bound: &str,
    var: &str,
    body: &str,
) -> Result<TheoryResponse, ApiError> {
    check_variable(req, var)?;
    let bound = integer_operand(g, req, bound, "the bound")?;
    if bound.sign() == dz_int::Sign::Neg {
        return Err(ApiError::Theory("the bound must not be negative".to_string()));
    }
    let (left_text, right) = split_relation(body)?;
    let left = operand(g, &req.alphabet, req.input_base, &left_text)?;
    let right = match right {
        Some((relation, text)) => {
            Some((relation, operand(g, &req.alphabet, req.input_base, &text)?))
        }
        None => None,
    };

    let one = Dz::from_magnitude(false, vec![Digit::ONE]);
    let mut counter = Dz::zero();
    let mut max_depth: u32 = 0;
    let mut decided = false;
    // `counter < bound`, decided by gates; the counter is a `Dz` advanced
    // by gate-composed `add`.
    while counter.compare(g, &bound) == Ordering::Less {
        let mut digits_msd = counter.magnitude_digits().to_vec();
        digits_msd.reverse();
        let scope = DepthScope::enter();
        let l = substitute(&left, var, &digits_msd);
        let holds = match &right {
            None => !exact_value(g, &l)?.is_zero(),
            Some((relation, r)) => {
                let r = substitute(r, var, &digits_msd);
                let order = exact_value(g, &l)?.compare(g, &exact_value(g, &r)?);
                match relation {
                    Relation::Eq => order == Ordering::Equal,
                    Relation::Ne => order != Ordering::Equal,
                    Relation::Lt => order == Ordering::Less,
                    Relation::Le => order != Ordering::Greater,
                    Relation::Gt => order == Ordering::Greater,
                    Relation::Ge => order != Ordering::Less,
                }
            }
        };
        let this_depth = scope.measured();
        drop(scope);
        if this_depth > max_depth {
            max_depth = this_depth;
        }
        let stop = match quantifier {
            Quantifier::Forall => !holds,
            Quantifier::Exists => holds,
        };
        if stop {
            decided = true;
            break;
        }
        counter = counter.add(g, &one);
    }

    // `decided`: the loop stopped at `counter`, so `counter + 1` candidates
    // ran; otherwise it ran out of candidates and `counter == bound` ran.
    let verdict = match quantifier {
        Quantifier::Forall => !decided,
        Quantifier::Exists => decided,
    };
    let mut fields = vec![
        Field::text("quantifier", quantifier.label().to_string()),
        Field::flag("holds", verdict),
    ];
    let checked = if decided {
        let name = match quantifier {
            Quantifier::Forall => "counterexample",
            Quantifier::Exists => "witness",
        };
        fields.push(Field::text(name, int_text(&counter)));
        counter.add(g, &one)
    } else {
        counter
    };
    fields.push(Field::text("checked", int_text(&checked)));
    Ok(TheoryResponse {
        fields,
        gate_count: g.cost(),
        depth: max_depth,
        reconstruction_ops: g.reconstruction_ops(),
    })
}

// --------------------------------------------------------------------- JSON

/// A successful response as one line of JSON, written by hand like
/// `to_json_line`: the fields first, then the shared measurements.
pub fn theory_json_line(response: &TheoryResponse) -> String {
    let mut out = String::from("{\"ok\":true");
    for field in &response.fields {
        if field.raw {
            out.push_str(&format!(",\"{}\":{}", field.name, field.value));
        } else {
            out.push_str(&format!(
                ",\"{}\":\"{}\"",
                field.name,
                crate::escape_json(&field.value)
            ));
        }
    }
    out.push_str(&format!(
        ",\"gate_count\":{},\"depth\":{},\"reconstruction_ops\":{}}}",
        crate::gate_count_json(&response.gate_count),
        response.depth,
        response.reconstruction_ops
    ));
    out
}
