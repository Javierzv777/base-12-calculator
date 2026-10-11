//! End-to-end checks of the one-shot CLI mode: what it prints, and the
//! exit code a script would branch on (`docs/04-fase6-interfaces.md`, 3.2).
//!
//! These run the real binary, so they also cover the argument parsing that
//! no library test can reach. Every case here is cheap on purpose -- the
//! cost of the scientific functions is exercised in `dz-sci`'s own tests,
//! not by paying for it again in a subprocess.

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_dz-cli"))
        .args(args)
        .output()
        .expect("the CLI binary runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn named_operations_print_one_line_and_succeed() {
    let output = run(&["sum", "1", "3"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "4");

    assert_eq!(stdout(&run(&["mul", "4;6", "3", "--mode", "fixed"])), "11;6");
    assert_eq!(stdout(&run(&["eval", "4;6 * 3", "--mode", "fixed"])), "11;6");
    assert_eq!(stdout(&run(&["neg", "3"])), "-3");
}

#[test]
fn a_registered_function_is_also_a_subcommand() {
    // `sqrt 9` is exact, so it prints `3` with no approximation marker.
    let output = run(&["sqrt", "9", "--scale", "4"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "3");
}

#[test]
fn rational_mode_reports_a_periodic_fraction() {
    let output = run(&["div", "1", "5"]);
    assert!(output.status.success());
    assert!(stdout(&output).starts_with("1/5"));
    assert!(stdout(&output).contains("periodica"));
}

#[test]
fn digits_are_ascii_by_default_and_glyphs_on_request() {
    // A pipe or a CI log is exactly where U+218A/U+218B fails to render.
    assert_eq!(stdout(&run(&["eval", "X", "--mode", "fixed"])), "X");
    assert_eq!(
        stdout(&run(&["eval", "X", "--mode", "fixed", "--unicode"])),
        "\u{218A}"
    );
}

#[test]
fn json_mode_is_one_line_and_machine_readable() {
    let line = stdout(&run(&["sum", "1", "3", "--json"]));
    assert!(line.starts_with('{') && line.ends_with('}'), "{}", line);
    assert!(line.contains("\"ok\":true"));
    assert!(line.contains("\"display\":\"4\""));
    assert!(line.contains("\"total\":"));
}

#[test]
fn gate_counts_are_available_per_call() {
    let output = run(&["sum", "1", "3", "--gates"]);
    let text = stdout(&output);
    assert!(text.contains("gates: cycle="), "{}", text);
    assert!(text.contains("total="), "{}", text);
}

#[test]
fn every_backend_answers_the_same() {
    for backend in ["mvl", "crt", "table"] {
        let output = run(&["mul", "4;6", "3", "--mode", "fixed", "--backend", backend]);
        assert_eq!(stdout(&output), "11;6", "backend {}", backend);
    }
}

#[test]
fn a_calculation_error_exits_one_and_a_usage_error_exits_two() {
    let failed = run(&["div", "1", "0"]);
    assert_eq!(failed.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&failed.stderr).contains("division by zero"));

    for bad in [
        vec!["frobnicate", "1"],
        vec!["sum", "1"],
        vec!["sum", "1", "3", "--mode", "octal"],
        vec!["sum", "1", "3", "--nonsense"],
    ] {
        let output = run(&bad);
        assert_eq!(output.status.code(), Some(2), "{:?}", bad);
    }
}

#[test]
fn help_and_the_function_catalog_exit_zero() {
    let help = run(&["--help"]);
    assert!(help.status.success());
    assert!(stdout(&help).contains("dozenal"));

    let catalog = run(&["--functions"]);
    assert!(catalog.status.success());
    let text = stdout(&catalog);
    for expected in ["sin", "sqrt", "ln", "pi", "trigonometry"] {
        assert!(text.contains(expected), "catalog is missing {}", expected);
    }
}

/// First `n` fractional digits of 1/7 in base twelve by schoolbook long
/// division, as ASCII (`X` = ten, `E` = eleven). Test oracle only.
fn one_seventh_digits(n: usize) -> String {
    let mut remainder = 1u32;
    let mut out = String::new();
    for _ in 0..n {
        remainder *= 12;
        out.push(char::from_digit(remainder / 7, 12).unwrap().to_ascii_uppercase());
        remainder %= 7;
    }
    out.replace('A', "X").replace('B', "E")
}

fn fractional_part(line: &str) -> String {
    line.split(';')
        .nth(1)
        .unwrap_or("")
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string()
}

#[test]
fn fixed_division_keeps_24_digits_by_default() {
    let out = stdout(&run(&["eval", "1/7", "--mode", "fixed"]));
    assert_eq!(fractional_part(&out), one_seventh_digits(24));
}

#[test]
fn fixed_division_follows_scale_when_it_exceeds_24() {
    let out = stdout(&run(&["eval", "1/7", "--mode", "fixed", "--scale", "60"]));
    assert_eq!(fractional_part(&out), one_seventh_digits(60));
    // A scale below 24 never shrinks division.
    let out = stdout(&run(&["eval", "1/7", "--mode", "fixed", "--scale", "6"]));
    assert_eq!(fractional_part(&out), one_seventh_digits(24));
}