//! `dz-cli` -- the terminal interface to the dozenal calculator.
//!
//! Two modes in one binary (`docs/04-fase6-interfaces.md`, section 3):
//!
//!   - **no arguments: the REPL**, unchanged in behaviour since Phase 5 --
//!     dozenal glyphs, `:backend`/`:mode` session state, exact rationals by
//!     default;
//!   - **arguments: one-shot**, `dz-cli sum 1 3`, which evaluates, prints
//!     one line and exits, with ASCII digit aliases by default (a pipe or
//!     a CI log is exactly where U+218A/U+218B fails to render) and an
//!     exit code a script can branch on.
//!
//! Neither mode contains dozenal logic: both build a `dz_api::Request` and
//! print what comes back. The function keypad, the one-shot subcommands
//! and the REPL's `:funcs` listing all come from the same registry
//! catalog, so a function registered in `dz-eval` shows up in all three
//! without touching this file.

use dz_api::{
    compare_backends, comparison_json_line, error_json_line, execute, functions, gate_count_csv,
    to_ascii, to_json_line, trace_csv, trace_json_line, trace_measured, trace_summary_csv,
    ApiError, BackendComparison, BackendKind, DisplayAlphabet, GateCount, InputBase, NumberMode,
    Operation, Request, Response, Trace, TraceKind, TraceRequest,
};
use std::io::{self, BufRead, Write};

const USAGE_EXIT: i32 = 2;
const ERROR_EXIT: i32 = 1;

// ------------------------------------------------------------------ shared

/// Render canonical text the way this invocation should show it: through
/// the configured presentation alphabet first (`docs/07`), then the ASCII
/// fallback for terminals that cannot draw U+218A/U+218B.
///
/// Order matters and only in one way: with the default alphabet
/// `from_canonical` is the identity, so `--unicode` and the ASCII default
/// behave exactly as they always did. With a configured pair there are no
/// canonical glyphs left for `to_ascii` to touch.
fn shown(text: &str, alphabet: &DisplayAlphabet, unicode: bool) -> String {
    let text = alphabet.from_canonical(text);
    if unicode {
        text
    } else {
        to_ascii(&text)
    }
}

/// The plain-text rendering of a response, including the periodicity note
/// the REPL has always appended.
fn plain_output(
    response: &Response,
    mode: NumberMode,
    unicode: bool,
    alphabet: &DisplayAlphabet,
) -> String {
    let mut out = response.display.clone();
    if response.periodic_note {
        match mode {
            NumberMode::Fixed => out.push_str(" (periodica en base 12, valor truncado)"),
            _ => out.push_str(" (periodica en base 12)"),
        }
    }
    shown(&out, alphabet, unicode)
}

fn gate_line(response: &Response) -> Option<String> {
    response.gate_count.as_ref().map(|count| {
        // Depth rides along on the same line: it is measured in the same
        // pass as the counters (Phase 7) and reading one without the other
        // is how "cheaper" gets confused with "shallower".
        let depth = match response.depth {
            Some(levels) => levels.to_string(),
            None => "?".to_string(),
        };
        // `recon` is the backend-specific overhead the five counters
        // cannot see: 0 on MVL and on the table oracle, non-zero only on
        // CRT (`docs/02-crt-vs-mvl.md`).
        format!(
            "gates: cycle={} min={} max={} lit={} thr={} total={} depth={} recon={}",
            count.cycle,
            count.min,
            count.max,
            count.lit,
            count.thr,
            count.total(),
            depth,
            response.reconstruction_ops.unwrap_or(0)
        )
    })
}

fn function_listing() -> String {
    let mut out = String::new();
    let catalog = functions();
    let mut current = "";
    for info in catalog {
        let label = info.category.label();
        if label != current {
            out.push_str(&format!("  [{}]\n", label));
            current = label;
        }
        let arguments = if info.arity.max == 0 {
            String::new()
        } else if info.arity.min == info.arity.max {
            format!("({} arg)", info.arity.min)
        } else {
            format!("({}-{} args)", info.arity.min, info.arity.max)
        };
        let exactness = if info.always_exact { "exact" } else { "approx" };
        out.push_str(&format!(
            "    {:<7} {:<10} {:<7} {}\n",
            info.name, arguments, exactness, info.description
        ));
    }
    out
}

// -------------------------------------------------------------- one-shot

/// Which of the two CSV documents `--csv`/`--csv-summary` asks for.
///
/// Two documents, not one overloaded schema: `trace_csv` is the step
/// table and `trace_summary_csv` is the context above it
/// (`docs/05-fase7-panel-investigacion.md`, 8.4).
#[derive(Clone, Copy, PartialEq, Eq)]
enum CsvDocument {
    Steps,
    Summary,
}

/// A CSV request: which document, and where it goes. `None` is stdout,
/// which is what a pipe or an agent wants.
struct CsvTarget {
    document: CsvDocument,
    path: Option<String>,
}

struct Flags {
    mode: NumberMode,
    backend: BackendKind,
    scale: usize,
    json: bool,
    unicode: bool,
    gates: bool,
    csv: Option<CsvTarget>,
    /// The presentation alphabet for this invocation (`--alphabet ab`).
    /// Default: the canonical glyphs, i.e. exactly the old behaviour.
    alphabet: DisplayAlphabet,
    /// Which base the typed operands are in (`--input-base 10`).
    /// Default: twelve, i.e. exactly the old behaviour -- and free, which
    /// base ten is not (`docs/10-entrada-base-10.md`).
    input_base: InputBase,
}

impl Default for Flags {
    fn default() -> Flags {
        Flags {
            mode: NumberMode::Rational,
            backend: BackendKind::Mvl,
            scale: dz_api::default_scale(),
            json: false,
            unicode: false,
            gates: false,
            csv: None,
            alphabet: DisplayAlphabet::canonical(),
            input_base: InputBase::Twelve,
        }
    }
}

/// Read `--alphabet ab` / `:alphabet ab`: exactly two characters, ten then
/// eleven, validated by `dz-api` against the live function catalog.
///
/// The error text is `dz-api`'s, verbatim: the rule that rejects a pair
/// lives in `dz-parse` and there is no second wording of it here.
fn parse_alphabet(value: &str) -> Result<DisplayAlphabet, String> {
    let chars: Vec<char> = value.chars().collect();
    match chars.as_slice() {
        [ten, eleven] => dz_api::display_alphabet(*ten, *eleven).map_err(|e| e.to_string()),
        _ => Err(format!(
            "the alphabet is exactly two characters, ten then eleven (e.g. 'ab'): got '{}'",
            value
        )),
    }
}

fn print_usage() {
    println!("dz-cli -- dozenal (base 12) calculator");
    println!();
    println!("  dz-cli                      start the interactive REPL");
    println!("  dz-cli <operation> <args>   evaluate once and exit");
    println!();
    println!("Operations:");
    println!("  sum a b        sub a b        mul a b");
    println!("  div a b        mod a b        pow base exp");
    println!("  neg a          eval \"<expression>\"");
    println!("  <function> <args...>   any registered function, e.g. 'dz-cli sin 1'");
    println!();
    println!("Research:");
    println!("  trace add|sub|mul a b       step-by-step circuit run (integers only)");
    println!("  compare <operation> <args>  the same call on all three gate backends");
    println!("  chain <display>             convert a formatted dozenal result back to");
    println!("                              --input-base (the reverse of reading base 10);");
    println!("                              reads --backend and --input-base like any call");
    println!();
    println!("Flags:");
    println!("  --mode rational|fixed|dfp   display mode (default: rational)");
    println!("  --backend mvl|crt|table     gate backend (default: mvl)");
    println!("  --scale <n>                 working digits for approximate functions");
    println!("  --json                      one line of JSON instead of plain text");
    println!("  --unicode                   dozenal glyphs instead of the X/E aliases");
    println!("  --alphabet <XY>             presentation symbols for ten and eleven,");
    println!("                              e.g. '--alphabet ab' (default: \u{218A}\u{218B});");
    println!("                              affects typed input and printed output only,");
    println!("                              never the CSV or the JSON");
    println!("  --input-base 10|12          base of the operands you type (default: 12);");
    println!("                              output is always dozenal. Base 10 is read");
    println!("                              with gates, so '--gates' includes its cost");
    println!("  --gates                     also print the gate count of the call");
    println!("  --functions                 list the callable functions and constants");
    println!("  --csv[=<path>]              CSV instead of text: the step table of a");
    println!("                              'trace', the gate table of a 'compare';");
    println!("                              no path means stdout");
    println!("  --csv-summary[=<path>]      the other trace document: operands, partial");
    println!("                              rows and result ('trace' only)");
    println!("  --help, -h                  this text");
    println!();
    println!("REPL session commands: :mem  :mem+  :memc  :history  :clear  :session");
    println!();
    println!("Notes:");
    println!("  Digits are 0-9 plus X (ten) and E (eleven); ';' is the Humphrey point,");
    println!("  so 4;6 is four and a half. Inside a function call the comma separates");
    println!("  arguments, so write 10000 rather than the grouped form 1,0000 there.");
    println!("  Exit codes: 0 success, 1 calculation error, 2 usage error.");
}

/// Parse flags out of the argument list, returning the positional
/// arguments and the flag values. A usage error is `Err(message)`.
fn split_arguments(args: &[String]) -> Result<(Vec<String>, Flags), String> {
    let mut positional = Vec::new();
    let mut flags = Flags::default();
    let mut i = 0usize;
    while i < args.len() {
        let argument = args[i].clone();
        if let Some(name) = argument.strip_prefix("--") {
            // `--csv` and `--csv-summary` take an *optional* path. It is
            // spelled `--csv=out.csv` rather than `--csv out.csv` because
            // the operands are positional too: `trace add EEE 1 --csv 5`
            // would otherwise be ambiguous between a path and an operand.
            let (name, attached) = match name.split_once('=') {
                Some((head, value)) => (head, Some(value.to_string())),
                None => (name, None),
            };
            if name == "csv" || name == "csv-summary" {
                flags.csv = Some(CsvTarget {
                    document: if name == "csv" {
                        CsvDocument::Steps
                    } else {
                        CsvDocument::Summary
                    },
                    path: attached,
                });
                i += 1;
                continue;
            }
            if attached.is_some() {
                return Err(format!("'--{}' does not take an attached value", name));
            }
            match name {
                "json" => flags.json = true,
                "unicode" => flags.unicode = true,
                "gates" => flags.gates = true,
                "help" => return Err(String::new()),
                "mode" | "backend" | "scale" | "alphabet" | "input-base" => {
                    i += 1;
                    if i >= args.len() {
                        return Err(format!("--{} needs a value", name));
                    }
                    let value = args[i].clone();
                    match name {
                        "mode" => {
                            flags.mode = NumberMode::parse(&value)
                                .ok_or(format!("unknown mode '{}'", value))?
                        }
                        "backend" => {
                            flags.backend = BackendKind::parse(&value)
                                .ok_or(format!("unknown backend '{}'", value))?
                        }
                        "alphabet" => flags.alphabet = parse_alphabet(&value)?,
                        "input-base" => {
                            flags.input_base = InputBase::parse(&value)
                                .ok_or(format!("unknown input base '{}' (use 10 or 12)", value))?
                        }
                        _ => {
                            flags.scale = parse_count(&value)
                                .ok_or(format!("--scale needs a small number, got '{}'", value))?
                        }
                    }
                }
                other => return Err(format!("unknown flag '--{}'", other)),
            }
            i += 1;
            continue;
        }
        if argument == "-h" {
            return Err(String::new());
        }
        positional.push(argument);
        i += 1;
    }
    Ok((positional, flags))
}

/// A digit count, not a dozenal value: how many fractional digits to work
/// to. Reading it as a native decimal number is the same category of
/// bookkeeping as a `Vec` length (PLAN.md Section 0), and it never becomes
/// part of a computed value.
fn parse_count(text: &str) -> Option<usize> {
    let count: usize = text.parse().ok()?;
    if count == 0 || count > 100 {
        return None;
    }
    Some(count)
}

fn build_operation(positional: &[String]) -> Result<Operation, String> {
    let name = positional[0].as_str();
    let rest = &positional[1..];
    let binary = |expected: usize| -> Result<(), String> {
        if rest.len() == expected {
            Ok(())
        } else {
            Err(format!(
                "'{}' takes {} argument(s), got {}",
                name,
                expected,
                rest.len()
            ))
        }
    };
    match name {
        "sum" | "add" => {
            binary(2)?;
            Ok(Operation::Sum { a: rest[0].clone(), b: rest[1].clone() })
        }
        "sub" => {
            binary(2)?;
            Ok(Operation::Sub { a: rest[0].clone(), b: rest[1].clone() })
        }
        "mul" => {
            binary(2)?;
            Ok(Operation::Mul { a: rest[0].clone(), b: rest[1].clone() })
        }
        "div" => {
            binary(2)?;
            Ok(Operation::Div { a: rest[0].clone(), b: rest[1].clone() })
        }
        "mod" => {
            binary(2)?;
            Ok(Operation::Mod { a: rest[0].clone(), b: rest[1].clone() })
        }
        "pow" => {
            binary(2)?;
            Ok(Operation::Pow { base: rest[0].clone(), exponent: rest[1].clone() })
        }
        "neg" => {
            binary(1)?;
            Ok(Operation::Neg { a: rest[0].clone() })
        }
        "eval" => {
            binary(1)?;
            Ok(Operation::Eval { expr: rest[0].clone() })
        }
        // Reserved names. `trace` and `compare` are top-level commands
        // (see `Command`), not operations, and the `other` arm below
        // turns *any* registered function name into a subcommand. Naming
        // them here explicitly means that registering a function called
        // `trace` or `compare` in `dz-eval` cannot silently shadow, or be
        // shadowed by, the research subcommands: the clash is reported
        // instead of being decided by the order of this `match`
        // (`docs/06-fase9-paridad-cli.md`, section 7).
        "trace" | "compare" => Err(format!(
            "'{0}' is a top-level command, not an operation: write \
             'dz-cli {0} ...' (see 'dz-cli --help')",
            name
        )),
        other => {
            // Any registered function is also a subcommand: `dz-cli sin 1`.
            // The catalog decides, so a new function needs no change here.
            let info = functions()
                .into_iter()
                .find(|f| f.name == other)
                .ok_or(format!("unknown operation '{}'", other))?;
            if !info.arity.accepts(rest.len()) {
                return Err(format!(
                    "'{}' takes {} argument(s), got {}",
                    other,
                    info.arity.min,
                    rest.len()
                ));
            }
            Ok(Operation::Call { name: other.to_string(), args: rest.to_vec() })
        }
    }
}

/// What one invocation asks for.
///
/// `Calc` is the path that has existed since Phase 6: build a
/// `dz_api::Operation` and call `execute`. The other two are the research
/// surface of `dz-api`, which has its own request and response shapes and
/// therefore cannot be an `Operation` (`docs/06-fase9-paridad-cli.md`,
/// section 2).
enum Command {
    Calc(Operation),
    Trace {
        kind: TraceKind,
        a: String,
        b: String,
    },
    Compare(Operation),
    /// The reverse of `--input-base 10`: rewrite an already-formatted
    /// dozenal result back into base ten (`dz_api::chain_operand`,
    /// `docs/09-encadenado-de-resultado.md`). One-shot only, same as
    /// `trace`/`compare`: there is no session state to chain onto here.
    Chain(String),
}

fn trace_kind_error(name: &str) -> String {
    if name == "div" {
        // The honesty the research panel already carries travels to the
        // terminal: say why there is no such trace, not "unknown".
        "there is no division trace: restoring division has no single carry that advances \
         position by position, so a step viewer for it would be showing something that does \
         not run. Traceable circuits: add, sub, mul"
            .to_string()
    } else {
        format!(
            "unknown trace kind '{}': the traceable circuits are add, sub and mul",
            name
        )
    }
}

fn build_command(positional: &[String]) -> Result<Command, String> {
    match positional[0].as_str() {
        "trace" => {
            let rest = &positional[1..];
            if rest.len() != 3 {
                return Err(format!(
                    "'trace' takes <add|sub|mul> <a> <b>, got {} argument(s)",
                    rest.len()
                ));
            }
            // `TraceKind::parse` is `dz-api`'s, so the CLI invents no
            // names of its own -- same discipline as `BackendKind::parse`.
            let kind = TraceKind::parse(&rest[0]).ok_or_else(|| trace_kind_error(&rest[0]))?;
            Ok(Command::Trace {
                kind,
                a: rest[1].clone(),
                b: rest[2].clone(),
            })
        }
        "compare" => {
            let rest = &positional[1..];
            if rest.is_empty() {
                return Err(
                    "'compare' takes an operation and its arguments, e.g. 'compare mul 25 34'"
                        .to_string(),
                );
            }
            Ok(Command::Compare(build_operation(rest)?))
        }
        "chain" => {
            let rest = &positional[1..];
            if rest.len() != 1 {
                return Err(format!(
                    "'chain' takes a single formatted dozenal result, e.g. 'chain 11;6 \
                     --input-base 10', got {} argument(s)",
                    rest.len()
                ));
            }
            Ok(Command::Chain(rest[0].clone()))
        }
        _ => Ok(Command::Calc(build_operation(positional)?)),
    }
}

// ------------------------------------------------- research rendering

fn step_table(steps: &[dz_api::TraceStep], indent: &str) -> String {
    let mut out = format!("{}pos  a  b  cin  sum  cout\n", indent);
    for step in steps {
        out.push_str(&format!(
            "{}{:>3}  {}  {}   {}    {}     {}\n",
            indent, step.position, step.a, step.b, step.carry_in, step.sum, step.carry_out
        ));
    }
    out
}

/// The plain-text rendering of a trace: one row per digit position, least
/// significant first, exactly the order the GUI viewer steps through.
fn render_trace(trace: &Trace) -> String {
    let mut out = String::new();
    match trace {
        Trace::Add(t) => {
            out.push_str(&format!(
                "trace: add  {} + {} = {}\n",
                t.a, t.b, t.result
            ));
            out.push_str(&step_table(&t.steps, "  "));
        }
        Trace::Sub(t) => {
            // Five labelled stages, because there is no subtractor: the
            // complement strategy is what actually runs (docs/05, 8.3).
            out.push_str(&format!(
                "trace: sub  {} - {} = {}\n",
                t.a, t.b, t.result
            ));
            out.push_str("  (there is no subtractor circuit: this is the twelve's-complement strategy)\n");
            out.push_str(&format!(
                "  stage 1  elevens' complement of b = {}\n",
                t.ones_complement
            ));
            out.push_str("  stage 2  add 1 to it:\n");
            out.push_str(&step_table(&t.complement_steps, "    "));
            out.push_str(&format!(
                "  stage 2  twelve's complement of b = {}\n",
                t.radix_complement
            ));
            out.push_str("  stage 3  a + twelve's complement of b:\n");
            out.push_str(&step_table(&t.add_steps, "    "));
            out.push_str(&format!("  stage 3  raw sum = {}\n", t.raw_sum));
            if t.borrowed {
                out.push_str("  borrowed: yes (a < b, so the raw sum is decoded)\n");
                out.push_str(&format!(
                    "  stage 4  elevens' complement of the raw sum = {}\n",
                    t.decode_ones_complement
                ));
                out.push_str("  stage 5  add 1 to it:\n");
                out.push_str(&step_table(&t.decode_steps, "    "));
            } else {
                out.push_str("  borrowed: no (a >= b, the raw sum is the answer)\n");
            }
        }
        Trace::Mul(t) => {
            out.push_str(&format!(
                "trace: mul  {} * {} = {}\n",
                t.a, t.b, t.result
            ));
            for row in &t.rows {
                out.push_str(&format!(
                    "  row {}  multiplier digit {}  partial {}  shifted {}\n",
                    row.index, row.multiplier_digit, row.row, row.shifted
                ));
                out.push_str(&step_table(&row.accumulate_steps, "    "));
                out.push_str(&format!("    accumulated = {}\n", row.accumulated));
            }
        }
    }
    out.push_str(&format!("result: {}\n", trace.result()));
    out
}

/// The three-backend table. `table` is labelled as the oracle for the
/// same reason the panel labels it (`docs/05`, section 5): without that
/// line the table invites reading "table wins" as a research result.
fn render_comparison(comparison: &BackendComparison) -> String {
    let mut out = format!("result: {}\n", comparison.display);
    out.push_str(
        "  backend     cycle      min      max      lit      thr      total    depth    recon\n",
    );
    for cost in &comparison.costs {
        let count = &cost.gate_count;
        out.push_str(&format!(
            "  {:<8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>10} {:>8} {:>8}\n",
            cost.backend.label(),
            count.cycle,
            count.min,
            count.max,
            count.lit,
            count.thr,
            count.total(),
            cost.depth,
            cost.reconstruction_ops
        ));
    }
    out.push_str("  (table is the truth-table oracle, not a production backend)\n");
    // The verdict is written by `dz-api`, never here: the GUI prints the
    // same sentence, and one authority is what keeps the two interfaces
    // from drifting (`docs/06-fase9-paridad-cli.md`, section 7).
    if let Some(v) = &comparison.verdict {
        out.push_str(&wrap_verdict(&v.summary));
    }
    out
}

/// Soft-wrap the verdict prose at 78 columns so a terminal shows it as a
/// paragraph rather than one very long line. Pure text layout: it splits
/// on spaces and never touches a dozenal value.
fn wrap_verdict(summary: &str) -> String {
    const WIDTH: usize = 76;
    let mut out = String::new();
    let mut line = String::from("  ");
    for word in summary.split_whitespace() {
        if line.chars().count() + word.chars().count() + 1 > WIDTH && !line.trim().is_empty() {
            out.push_str(line.trim_end());
            out.push('\n');
            line = String::from("  ");
        }
        if !line.trim().is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.trim().is_empty() {
        out.push_str(line.trim_end());
        out.push('\n');
    }
    out
}

/// The gate line of a trace. A trace has counters but no depth: the
/// traced circuits run the plain instantiation, which carries no arrival
/// levels (`dz_api::research::trace_measured`).
fn trace_gate_line(count: &GateCount) -> String {
    format!(
        "gates: cycle={} min={} max={} lit={} thr={} total={} depth=n/a (traced pass)",
        count.cycle,
        count.min,
        count.max,
        count.lit,
        count.thr,
        count.total()
    )
}

/// Write a CSV document exactly as `dz-api` produced it. The CLI never
/// reformats it -- one formatting authority is what keeps the terminal and
/// the GUI from drifting apart (`docs/06`, section 7).
fn emit_csv(document: &str, path: &Option<String>) -> i32 {
    match path {
        None => {
            print!("{}", document);
            0
        }
        Some(path) => match std::fs::write(path, document) {
            Ok(()) => {
                println!("wrote {}", path);
                0
            }
            Err(e) => {
                eprintln!("error: could not write '{}': {}", path, e);
                ERROR_EXIT
            }
        },
    }
}

fn run_trace(kind: TraceKind, a: &str, b: &str, flags: &Flags) -> i32 {
    let request = TraceRequest {
        backend: flags.backend,
        kind,
        a: a.to_string(),
        b: b.to_string(),
        alphabet: flags.alphabet,
        input_base: flags.input_base,
    };
    let (trace, count) = match trace_measured(&request) {
        Ok(pair) => pair,
        Err(error) => {
            if flags.json {
                println!("{}", error_json_line(&error));
            } else {
                eprintln!("error: {}", error);
            }
            return report_exit_code(&error);
        }
    };
    if let Some(target) = &flags.csv {
        let document = match target.document {
            CsvDocument::Steps => trace_csv(&trace),
            CsvDocument::Summary => trace_summary_csv(&trace),
        };
        return emit_csv(&document, &target.path);
    }
    if flags.json {
        println!("{}", trace_json_line(&trace));
        return 0;
    }
    let text = render_trace(&trace);
    print!("{}", shown(&text, &flags.alphabet, flags.unicode));
    if flags.gates {
        println!("{}", trace_gate_line(&count));
    }
    0
}

fn run_compare(op: &Operation, flags: &Flags) -> i32 {
    if let Some(target) = &flags.csv {
        if target.document == CsvDocument::Summary {
            eprintln!("usage error: '--csv-summary' is a trace document; 'compare' emits '--csv'");
            eprintln!("try 'dz-cli --help'");
            return USAGE_EXIT;
        }
    }
    let comparison = match compare_backends(op, flags.mode, flags.scale, flags.alphabet, flags.input_base) {
        Ok(comparison) => comparison,
        Err(error) => {
            if flags.json {
                println!("{}", error_json_line(&error));
            } else {
                eprintln!("error: {}", error);
            }
            return report_exit_code(&error);
        }
    };
    if let Some(target) = &flags.csv {
        return emit_csv(&gate_count_csv(&comparison.costs), &target.path);
    }
    if flags.json {
        println!("{}", comparison_json_line(&comparison));
        return 0;
    }
    let text = render_comparison(&comparison);
    print!("{}", shown(&text, &flags.alphabet, flags.unicode));
    0
}

fn run_command(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--functions") {
        println!("Callable functions and constants:");
        print!("{}", function_listing());
        return 0;
    }
    let (positional, flags) = match split_arguments(args) {
        Ok(parsed) => parsed,
        Err(message) => {
            if message.is_empty() {
                print_usage();
                return 0;
            }
            eprintln!("usage error: {}", message);
            eprintln!("try 'dz-cli --help'");
            return USAGE_EXIT;
        }
    };
    if positional.is_empty() {
        eprintln!("usage error: no operation given");
        eprintln!("try 'dz-cli --help'");
        return USAGE_EXIT;
    }
    let command = match build_command(&positional) {
        Ok(command) => command,
        Err(message) => {
            eprintln!("usage error: {}", message);
            eprintln!("try 'dz-cli --help'");
            return USAGE_EXIT;
        }
    };
    match command {
        Command::Trace { kind, a, b } => run_trace(kind, &a, &b, &flags),
        Command::Compare(op) => run_compare(&op, &flags),
        Command::Chain(display) => run_chain(&display, &flags),
        Command::Calc(op) => run_calc(op, &flags),
    }
}

fn run_calc(operation: Operation, flags: &Flags) -> i32 {
    if flags.csv.is_some() {
        eprintln!("usage error: '--csv' applies to 'trace' and 'compare', not to a plain call");
        eprintln!("try 'dz-cli --help'");
        return USAGE_EXIT;
    }
    let request = Request::new(operation)
        .with_mode(flags.mode)
        .with_backend(flags.backend)
        .with_scale(flags.scale)
        .with_alphabet(flags.alphabet)
        .with_input_base(flags.input_base);
    match execute(&request) {
        Ok(response) => {
            if flags.json {
                println!("{}", to_json_line(&response));
            } else {
                println!(
                    "{}",
                    plain_output(&response, flags.mode, flags.unicode, &flags.alphabet)
                );
                if flags.gates {
                    if let Some(line) = gate_line(&response) {
                        println!("{}", line);
                    }
                }
            }
            0
        }
        Err(error) => {
            if flags.json {
                println!("{}", error_json_line(&error));
            } else {
                eprintln!("error: {}", error);
            }
            report_exit_code(&error)
        }
    }
}

/// Escape a string for the hand-written JSON this file emits, the same
/// four characters `dz-api::escape_json` covers. `dz-api`'s version is
/// `pub(crate)`, so a chain result -- which never crosses `dz-api`'s own
/// `to_json_line` -- needs its own copy rather than a visibility change to
/// a function nothing else outside that crate should call.
fn escape_json_local(text: &str) -> String {
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

fn gate_count_json_local(count: &GateCount) -> String {
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

/// `dz-cli chain <display>`: the reverse of `--input-base 10` entry --
/// rewrite an already-formatted dozenal result back into `--input-base`
/// (`dz_api::chain_operand`, `docs/09-encadenado-de-resultado.md`). Unlike
/// `trace`/`compare` this has no request struct of its own: `chain_operand`
/// already takes exactly the backend and base this CLI's flags carry.
///
/// The converted text is printed as-is, never through `shown()`: base ten
/// has no dozenal glyphs to translate, so running it through the
/// presentation alphabet would be a no-op at best.
fn run_chain(display: &str, flags: &Flags) -> i32 {
    match dz_api::chain_operand(flags.backend, flags.input_base, display) {
        Ok((text, count)) => {
            if flags.json {
                println!(
                    "{{\"ok\":true,\"display\":\"{}\",\"gate_count\":{}}}",
                    escape_json_local(&text),
                    gate_count_json_local(&count)
                );
            } else {
                println!("{}", text);
                if flags.gates {
                    println!(
                        "gates: cycle={} min={} max={} lit={} thr={} total={}",
                        count.cycle,
                        count.min,
                        count.max,
                        count.lit,
                        count.thr,
                        count.total()
                    );
                }
            }
            0
        }
        Err(error) => {
            if flags.json {
                println!("{}", error_json_line(&error));
            } else {
                eprintln!("error: {}", error);
            }
            report_exit_code(&error)
        }
    }
}

fn report_exit_code(_error: &ApiError) -> i32 {
    // Both parse and evaluation failures are "the calculation did not
    // happen"; usage errors (exit 2) are the ones this binary detects
    // before ever calling `dz-api`.
    ERROR_EXIT
}

// ------------------------------------------------------------------- REPL

/// The REPL's session state.
///
/// It lives here, not in `dz-api`, because `dz-api` is stateless by
/// design: "which backend is active", the history and the memory register
/// are interface concepts (`CLAUDE.md`, and `AppState` is the GUI's copy
/// of exactly this). Nothing here is a dozenal *value*: the memory holds
/// the **already-formatted string** `dz-api` returned, because the CLI
/// cannot recompose a dozenal number any more than the frontend can.
/// Recalling it means feeding that text back through `dz-parse`.
struct Session {
    backend: BackendKind,
    mode: NumberMode,
    scale: usize,
    show_gates: bool,
    /// The memory register: `None` is empty, as after `:memc`.
    memory: Option<String>,
    /// `(input, output)` pairs, in order. Process memory, like the GUI's.
    /// The output is **canonical** text, so `:alphabet` repaints the whole
    /// list instead of leaving it half in one spelling and half in the
    /// other (`docs/07` section 3.5).
    history: Vec<(String, String)>,
    /// The presentation alphabet, switchable with `:alphabet`.
    alphabet: DisplayAlphabet,
    /// The base typed operands are in, switchable with `:inputbase`.
    input_base: InputBase,
}

fn print_repl_help() {
    println!("dz-cli -- dozenal (base 12) calculator");
    println!("  Alphabet: 0-9, X/T = ten (dek, \u{218A}), E = eleven (el, \u{218B})");
    println!("  Fractional separator: ';' (Humphrey point), e.g. 4;6 = 4 + 6/12");
    println!("  Operators: + - * (or x) / (or \u{00f7}) % ^  and parentheses");
    println!("  Functions: sin(1), sqrt(9), log(100), pi -- ':funcs' lists them all");
    println!("  Commands: :backend mvl|crt|table   :mode rational|fixed|dfp");
    println!("            :scale <n>   :gates   :funcs   :help   :quit");
    println!("            :alphabet <XY> (presentation symbols for ten and eleven,");
    println!("                            e.g. ':alphabet ab'; ':alphabet' shows it)");
    println!("            :inputbase 10|12 (base of what you type; output stays");
    println!("                            dozenal, and base 10 costs gates to read)");
    println!("  Session:  :mem (recall)  :mem+ (store last result)  :memc (clear)");
    println!("            :history   :clear   :session");
    println!("            :chain <display> (convert a formatted result back to :inputbase)");
    println!("  Research: run 'dz-cli trace add EEE 1' or 'dz-cli compare mul 25 34'");
}

fn run_repl() {
    let mut session = Session {
        backend: BackendKind::Mvl,
        mode: NumberMode::Rational,
        scale: dz_api::default_scale(),
        show_gates: false,
        memory: None,
        history: Vec::new(),
        alphabet: DisplayAlphabet::canonical(),
        input_base: InputBase::Twelve,
    };
    let stdin = io::stdin();
    println!("dz-cli -- dozenal calculator. Default mode: rational. Type :help for usage, :quit to exit.");
    loop {
        print!("dz> ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {}
            Err(e) => {
                eprintln!("input error: {}", e);
                break;
            }
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == ":quit" || line == ":q" || line == ":exit" {
            break;
        }
        if line == ":help" {
            print_repl_help();
            continue;
        }
        if line == ":funcs" {
            print!("{}", function_listing());
            continue;
        }
        if line == ":gates" {
            session.show_gates = !session.show_gates;
            println!("gate counts {}", if session.show_gates { "on" } else { "off" });
            continue;
        }
        // Session commands (Phase 9): the terminal's equivalent of the
        // GUI's memory box and history list. The memory holds text, never
        // a number -- recalling it re-parses it with `dz-parse`.
        if line == ":mem" {
            match &session.memory {
                Some(value) => println!("memory = {}", session.alphabet.from_canonical(value)),
                None => println!("memory is empty (':mem+' stores the last result)"),
            }
            continue;
        }
        if line == ":mem+" {
            match session.history.last() {
                Some((_, output)) => {
                    let stored = output.clone();
                    println!("memory = {}", session.alphabet.from_canonical(&stored));
                    session.memory = Some(stored);
                }
                None => println!("nothing to store yet"),
            }
            continue;
        }
        if line == ":memc" {
            session.memory = None;
            println!("memory cleared");
            continue;
        }
        if line == ":history" {
            if session.history.is_empty() {
                println!("history is empty");
            }
            // The index is a counter, not a dozenal value: printing it in
            // decimal is the same bookkeeping as a `Vec` length.
            for (index, (input, output)) in session.history.iter().enumerate() {
                println!(
                    "  {:>3}  {}  =  {}",
                    index + 1,
                    input,
                    session.alphabet.from_canonical(output)
                );
            }
            continue;
        }
        if line == ":clear" {
            session.history.clear();
            println!("history cleared");
            continue;
        }
        if line == ":session" {
            println!("backend = {}", session.backend.label());
            println!("mode = {}", session.mode.label());
            println!("scale = {} fractional digits", session.scale);
            println!("gate counts {}", if session.show_gates { "on" } else { "off" });
            println!(
                "memory = {}",
                session
                    .memory
                    .as_deref()
                    .map(|value| session.alphabet.from_canonical(value))
                    .unwrap_or_else(|| "(empty)".to_string())
            );
            println!(
                "alphabet = {}{}",
                session.alphabet.ten(),
                session.alphabet.eleven()
            );
            println!("input base = {}", session.input_base.label());
            println!("history = {} entries", session.history.len());
            continue;
        }
        // The presentation alphabet (`docs/07`). Changing it repaints the
        // whole session: the history and the memory hold canonical text,
        // and everything is rendered through the alphabet at print time.
        if line == ":alphabet" {
            println!(
                "alphabet: ten = '{}', eleven = '{}'{}",
                session.alphabet.ten(),
                session.alphabet.eleven(),
                if session.alphabet.is_canonical() { " (default)" } else { "" }
            );
            continue;
        }
        if let Some(rest) = line.strip_prefix(":alphabet ") {
            match parse_alphabet(rest.trim()) {
                Ok(alphabet) => {
                    session.alphabet = alphabet;
                    println!(
                        "alphabet: ten = '{}', eleven = '{}'",
                        alphabet.ten(),
                        alphabet.eleven()
                    );
                }
                Err(message) => println!("error: {}", message),
            }
            continue;
        }
        // The input base (`docs/10`). Unlike `:alphabet` this does not
        // repaint anything: it changes how the *next* line is read, and
        // results already in the history were read in whatever base was in
        // force at the time. Output is dozenal either way.
        if line == ":inputbase" {
            println!(
                "input base = {}{}",
                session.input_base.label(),
                if session.input_base.is_dozenal() { " (default, free)" } else { " (costs gates to read)" }
            );
            continue;
        }
        if let Some(rest) = line.strip_prefix(":inputbase ") {
            match InputBase::parse(rest.trim()) {
                Some(base) => {
                    session.input_base = base;
                    println!("input base = {}", base.label());
                }
                None => println!("unknown input base '{}' (use 10 or 12)", rest.trim()),
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix(":backend ") {
            match BackendKind::parse(rest.trim()) {
                Some(backend) => {
                    session.backend = backend;
                    println!("backend = {}", backend.label());
                }
                None => println!("unknown backend '{}' (use mvl, crt, or table)", rest.trim()),
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix(":mode ") {
            match NumberMode::parse(rest.trim()) {
                Some(mode) => {
                    session.mode = mode;
                    println!("mode = {}", mode.label());
                }
                None => println!("unknown mode '{}' (use rational, fixed, or dfp)", rest.trim()),
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix(":scale ") {
            match parse_count(rest.trim()) {
                Some(scale) => {
                    session.scale = scale;
                    println!("scale = {} fractional digits", scale);
                }
                None => println!("scale must be a small positive number of digits"),
            }
            continue;
        }
        // The reverse of `:inputbase 10` reading: rewrite an already
        // formatted dozenal result back into the session's input base
        // (`dz_api::chain_operand`). Session-scoped like `:backend`/
        // `:mode`, not history-scoped -- it takes whatever text is typed
        // after it, not necessarily the last result.
        if let Some(rest) = line.strip_prefix(":chain ") {
            match dz_api::chain_operand(session.backend, session.input_base, rest.trim()) {
                Ok((text, count)) => {
                    println!("{}", text);
                    if session.show_gates {
                        println!(
                            "gates: cycle={} min={} max={} lit={} thr={} total={}",
                            count.cycle,
                            count.min,
                            count.max,
                            count.lit,
                            count.thr,
                            count.total()
                        );
                    }
                }
                Err(error) => println!("error: {}", error),
            }
            continue;
        }

        let request = Request::new(Operation::Eval { expr: line.to_string() })
            .with_backend(session.backend)
            .with_mode(session.mode)
            .with_scale(session.scale)
            .with_alphabet(session.alphabet)
            .with_input_base(session.input_base);
        match execute(&request) {
            Ok(response) => {
                // The interactive REPL keeps the canonical glyphs: the user
                // is looking at the terminal and can see whether their font
                // renders them (docs/04-fase6-interfaces.md, 3.4).
                println!(
                    "{}",
                    plain_output(&response, session.mode, true, &session.alphabet)
                );
                // The history keeps the formatted text, which is all the
                // interface ever holds of a dozenal value.
                session
                    .history
                    .push((line.to_string(), response.display.clone()));
                if session.show_gates {
                    if let Some(line) = gate_line(&response) {
                        println!("{}", line);
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        run_repl();
        return;
    }
    std::process::exit(run_command(&args));
}
