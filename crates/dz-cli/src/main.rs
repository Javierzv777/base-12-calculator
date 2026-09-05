//! `dz-cli` -- the first usable deliverable (PLAN.md Phase 4/5): a REPL
//! that evaluates dozenal expressions end-to-end, never touching native
//! binary/decimal arithmetic for the values themselves.
//!
//! Three display modes (`:mode rational|fixed|dfp`), default `rational`
//! (PLAN.md Phase 5: "este debe ser el modo por defecto del REPL"):
//!   - `rational`: exact `num/den`, e.g. `1/3` prints `1/3`.
//!   - `fixed`: truncated fixed-point, with an explicit warning when the
//!     exact value doesn't terminate in base 12 (never a silent truncation).
//!   - `dfp`: dozenal floating point (12-digit mantissa, half-even rounded).

use dz_eval::{evaluate, evaluate_dfp, evaluate_rational};
use dz_gates::{CrtGates, DozGates, MvlGates, TableGates};
use dz_parse::{format_dfp, format_fixed, format_rational, parse_str};
use std::io::{self, BufRead, Write};

enum Backend {
    Mvl(MvlGates),
    Crt(CrtGates),
    Table(TableGates),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Rational,
    Fixed,
    Dfp,
}

fn eval_line<G: DozGates>(g: &G, mode: Mode, line: &str) -> String {
    let expr = match parse_str(line) {
        Ok(e) => e,
        Err(e) => return format!("error: {}", e),
    };
    match mode {
        Mode::Rational => match evaluate_rational(g, &expr) {
            Ok(value) => {
                let mut out = format_rational(&value);
                if !value.is_zero() && !value.terminates_in_base12(g) {
                    out.push_str(" (periodica en base 12)");
                }
                out
            }
            Err(e) => format!("error: {}", e),
        },
        Mode::Fixed => {
            // Check termination against the *exact* value first, so the
            // warning is never a guess based on the truncated output.
            let periodic_note = match evaluate_rational(g, &expr) {
                Ok(r) if !r.is_zero() && !r.terminates_in_base12(g) => {
                    Some(" (periodica en base 12, valor truncado)".to_string())
                }
                _ => None,
            };
            match evaluate(g, &expr) {
                Ok(value) => {
                    let mut out = format_fixed(&value);
                    if let Some(note) = periodic_note {
                        out.push_str(&note);
                    }
                    out
                }
                Err(e) => format!("error: {}", e),
            }
        }
        Mode::Dfp => match evaluate_dfp(g, &expr) {
            Ok(value) => format_dfp(g, &value),
            Err(e) => format!("error: {}", e),
        },
    }
}

fn print_help() {
    println!("dz-cli -- dozenal (base 12) calculator");
    println!("  Alphabet: 0-9, X/T = ten (dek, \u{218A}), E = eleven (el, \u{218B})");
    println!("  Fractional separator: ';' (Humphrey point), e.g. 4;6 = 4 + 6/12");
    println!("  Operators: + - * (or x) / (or \u{00f7}) % ^  and parentheses");
    println!("  Commands: :backend mvl|crt|table   :mode rational|fixed|dfp   :help   :quit");
}

fn main() {
    let mut backend = Backend::Mvl(MvlGates::new());
    let mut mode = Mode::Rational;
    let stdin = io::stdin();
    println!("dz-cli -- dozenal calculator (Phase 5). Default mode: rational. Type :help for usage, :quit to exit.");
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
            print_help();
            continue;
        }
        if let Some(rest) = line.strip_prefix(":backend ") {
            match rest.trim() {
                "mvl" => {
                    backend = Backend::Mvl(MvlGates::new());
                    println!("backend = mvl");
                }
                "crt" => {
                    backend = Backend::Crt(CrtGates::new());
                    println!("backend = crt");
                }
                "table" => {
                    backend = Backend::Table(TableGates::new());
                    println!("backend = table");
                }
                other => println!("unknown backend '{}' (use mvl, crt, or table)", other),
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix(":mode ") {
            match rest.trim() {
                "rational" => {
                    mode = Mode::Rational;
                    println!("mode = rational");
                }
                "fixed" => {
                    mode = Mode::Fixed;
                    println!("mode = fixed");
                }
                "dfp" => {
                    mode = Mode::Dfp;
                    println!("mode = dfp");
                }
                other => println!("unknown mode '{}' (use rational, fixed, or dfp)", other),
            }
            continue;
        }

        let result = match &backend {
            Backend::Mvl(g) => eval_line(g, mode, line),
            Backend::Crt(g) => eval_line(g, mode, line),
            Backend::Table(g) => eval_line(g, mode, line),
        };
        println!("{}", result);
    }
}
