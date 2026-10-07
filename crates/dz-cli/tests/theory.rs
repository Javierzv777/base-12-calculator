//! End-to-end checks of the number-theory subcommands of the one-shot CLI:
//! what they print, the JSON shape and the exit codes.

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_dz-cli"))
        .args(args)
        .output()
        .expect("the CLI binary runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n")
}

#[test]
fn divrem_prints_both_values_and_the_gate_line_on_request() {
    let plain = run(&["divrem", "8820", "7"]);
    assert!(plain.status.success());
    assert_eq!(stdout(&plain), "quotient: 12X6\nremainder: 6\n");
    let gated = run(&["divrem", "8820", "7", "--gates"]);
    assert!(stdout(&gated).contains("total=113920 depth=294 recon=0"), "{}", stdout(&gated));
}

#[test]
fn valuation_and_divisible_print_named_fields() {
    assert_eq!(stdout(&run(&["valuation", "8820", "5"])), "exponent: 4\ncofactor: 20\n");
    assert_eq!(
        stdout(&run(&["divisible", "8820", "4"])),
        "divisible: true\nmethod: last-digit\n"
    );
    assert_eq!(
        stdout(&run(&["divisible", "8820", "7"])),
        "divisible: false\nmethod: div_rem\n"
    );
}

#[test]
fn quantifiers_print_the_verdict_and_the_witness() {
    let found = run(&["exists", "1000", "i*i = 8909"]);
    assert!(found.status.success());
    assert_eq!(stdout(&found), "quantifier: exists\nholds: true\nwitness: X3\nchecked: X4\n");
    let all = run(&["forall", "20", "i*i >= i"]);
    assert_eq!(stdout(&all), "quantifier: forall\nholds: true\nchecked: 20\n");
}

#[test]
fn the_variable_can_be_renamed_and_a_bad_name_is_a_calculation_error() {
    let ok = run(&["exists", "10", "k*k = 4", "--var", "k"]);
    assert!(stdout(&ok).contains("witness: 2"));
    let bad = run(&["exists", "10", "x*x = 4", "--var", "x"]);
    assert_eq!(bad.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&bad.stderr).contains("bad variable name"));
}

#[test]
fn json_output_and_error_paths() {
    let json = stdout(&run(&["valuation", "8820", "5", "--json"]));
    assert!(json.starts_with("{\"ok\":true,\"exponent\":\"4\",\"cofactor\":\"20\",\"gate_count\""), "{}", json);

    let zero = run(&["valuation", "0", "2"]);
    assert_eq!(zero.status.code(), Some(1));
    let json_error = stdout(&run(&["valuation", "5", "1", "--json"]));
    assert!(json_error.starts_with("{\"ok\":false,\"error\":"), "{}", json_error);

    let usage = run(&["divrem", "1"]);
    assert_eq!(usage.status.code(), Some(2));
    let usage = run(&["forall", "3"]);
    assert_eq!(usage.status.code(), Some(2));
}

#[test]
fn the_quantifier_names_are_reserved_commands_not_operations() {
    let output = run(&["compare", "forall", "3", "i"]);
    assert_eq!(output.status.code(), Some(2));
}
