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
    error_json_line, execute, functions, to_ascii, to_json_line, ApiError, BackendKind, NumberMode,
    Operation, Request, Response,
};
use std::io::{self, BufRead, Write};

const USAGE_EXIT: i32 = 2;
const ERROR_EXIT: i32 = 1;

// ------------------------------------------------------------------ shared

/// The plain-text rendering of a response, including the periodicity note
/// the REPL has always appended.
fn plain_output(response: &Response, mode: NumberMode, unicode: bool) -> String {
    let mut out = response.display.clone();
    if response.periodic_note {
        match mode {
            NumberMode::Fixed => out.push_str(" (periodica en base 12, valor truncado)"),
            _ => out.push_str(" (periodica en base 12)"),
        }
    }
    if unicode {
        out
    } else {
        to_ascii(&out)
    }
}

fn gate_line(response: &Response) -> Option<String> {
    response.gate_count.as_ref().map(|count| {
        format!(
            "gates: cycle={} min={} max={} lit={} thr={} total={}",
            count.cycle,
            count.min,
            count.max,
            count.lit,
            count.thr,
            count.total()
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

struct Flags {
    mode: NumberMode,
    backend: BackendKind,
    scale: usize,
    json: bool,
    unicode: bool,
    gates: bool,
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
        }
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
    println!("Flags:");
    println!("  --mode rational|fixed|dfp   display mode (default: rational)");
    println!("  --backend mvl|crt|table     gate backend (default: mvl)");
    println!("  --scale <n>                 working digits for approximate functions");
    println!("  --json                      one line of JSON instead of plain text");
    println!("  --unicode                   dozenal glyphs instead of the X/E aliases");
    println!("  --gates                     also print the gate count of the call");
    println!("  --functions                 list the callable functions and constants");
    println!("  --help, -h                  this text");
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
            match name {
                "json" => flags.json = true,
                "unicode" => flags.unicode = true,
                "gates" => flags.gates = true,
                "help" => return Err(String::new()),
                "mode" | "backend" | "scale" => {
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
    let operation = match build_operation(&positional) {
        Ok(op) => op,
        Err(message) => {
            eprintln!("usage error: {}", message);
            eprintln!("try 'dz-cli --help'");
            return USAGE_EXIT;
        }
    };

    let request = Request::new(operation)
        .with_mode(flags.mode)
        .with_backend(flags.backend)
        .with_scale(flags.scale);
    match execute(&request) {
        Ok(response) => {
            if flags.json {
                println!("{}", to_json_line(&response));
            } else {
                println!("{}", plain_output(&response, flags.mode, flags.unicode));
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

fn report_exit_code(_error: &ApiError) -> i32 {
    // Both parse and evaluation failures are "the calculation did not
    // happen"; usage errors (exit 2) are the ones this binary detects
    // before ever calling `dz-api`.
    ERROR_EXIT
}

// ------------------------------------------------------------------- REPL

struct Session {
    backend: BackendKind,
    mode: NumberMode,
    scale: usize,
    show_gates: bool,
}

fn print_repl_help() {
    println!("dz-cli -- dozenal (base 12) calculator");
    println!("  Alphabet: 0-9, X/T = ten (dek, \u{218A}), E = eleven (el, \u{218B})");
    println!("  Fractional separator: ';' (Humphrey point), e.g. 4;6 = 4 + 6/12");
    println!("  Operators: + - * (or x) / (or \u{00f7}) % ^  and parentheses");
    println!("  Functions: sin(1), sqrt(9), log(100), pi -- ':funcs' lists them all");
    println!("  Commands: :backend mvl|crt|table   :mode rational|fixed|dfp");
    println!("            :scale <n>   :gates   :funcs   :help   :quit");
}

fn run_repl() {
    let mut session = Session {
        backend: BackendKind::Mvl,
        mode: NumberMode::Rational,
        scale: dz_api::default_scale(),
        show_gates: false,
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

        let request = Request::new(Operation::Eval { expr: line.to_string() })
            .with_backend(session.backend)
            .with_mode(session.mode)
            .with_scale(session.scale);
        match execute(&request) {
            Ok(response) => {
                // The interactive REPL keeps the canonical glyphs: the user
                // is looking at the terminal and can see whether their font
                // renders them (docs/04-fase6-interfaces.md, 3.4).
                println!("{}", plain_output(&response, session.mode, true));
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
