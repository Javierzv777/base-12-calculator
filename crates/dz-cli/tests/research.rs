//! End-to-end checks of the Phase 9 research subcommands
//! (`docs/06-fase9-paridad-cli.md`): `trace`, `compare`, `--csv`,
//! `--csv-summary` and `--json`.
//!
//! The load-bearing test here is `the_csv_the_cli_prints_is_byte_for_byte
//! _the_one_dz_api_builds`: the GUI cannot drift from `dz-api`'s format
//! because it never sees the text, and this is how the CLI gets the same
//! guarantee. If someone ever "tidies up" a column in `dz-cli`, this fails.

use dz_api::{
    InputBase,
    compare_backends, gate_count_csv, trace, trace_csv, trace_summary_csv, BackendKind, NumberMode,
    DisplayAlphabet, Operation, TraceKind, TraceRequest,
};
use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_dz-cli"))
        .args(args)
        .output()
        .expect("the CLI binary runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

// ---------------------------------------------------------------- trace

#[test]
fn an_addition_trace_prints_one_row_per_digit_position() {
    let output = run(&["trace", "add", "EEE", "1"]);
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.starts_with("trace: add  EEE + 1 = 1000"), "{}", text);
    assert!(text.contains("pos  a  b  cin  sum  cout"), "{}", text);
    // Three positions, least significant first, then the result line.
    assert_eq!(
        text.lines().filter(|l| l.trim_start().starts_with(['0', '1', '2'])).count(),
        3,
        "{}",
        text
    );
    assert!(text.trim_end().ends_with("result: 1000"), "{}", text);
}

#[test]
fn a_subtraction_trace_names_the_complement_stages_instead_of_faking_a_borrow() {
    let text = stdout(&run(&["trace", "sub", "1", "1000"]));
    for expected in [
        "there is no subtractor circuit",
        "elevens' complement of b",
        "twelve's complement of b",
        "raw sum",
        "borrowed: yes",
    ] {
        assert!(text.contains(expected), "{} missing from {}", expected, text);
    }
    assert!(text.contains("result: -EEE"), "{}", text);
}

#[test]
fn a_multiplication_trace_is_one_group_per_multiplier_digit() {
    let text = stdout(&run(&["trace", "mul", "25", "34"]));
    assert!(text.contains("row 0  multiplier digit 4"), "{}", text);
    assert!(text.contains("row 1  multiplier digit 3"), "{}", text);
    assert!(text.contains("accumulated = 808"), "{}", text);
    assert!(text.contains("result: 808"), "{}", text);
}

#[test]
fn digits_are_ascii_by_default_and_glyphs_on_request() {
    assert!(stdout(&run(&["trace", "add", "EEE", "1"])).contains("EEE + 1"));
    assert!(stdout(&run(&["trace", "add", "EEE", "1", "--unicode"]))
        .contains("\u{218B}\u{218B}\u{218B} + 1"));
}

#[test]
fn a_traced_run_reports_gates_but_says_it_has_no_depth() {
    // The traced circuits run the plain instantiation, which carries no
    // arrival levels: reporting a number there would be inventing one.
    let text = stdout(&run(&["trace", "add", "EEE", "1", "--gates"]));
    assert!(text.contains("gates: cycle="), "{}", text);
    assert!(text.contains("depth=n/a"), "{}", text);
}

#[test]
fn there_is_no_division_trace_and_the_error_says_why() {
    let output = run(&["trace", "div", "1", "2"]);
    assert_eq!(output.status.code(), Some(2));
    let error = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(error.contains("no division trace"), "{}", error);
    assert!(error.contains("add, sub, mul"), "{}", error);
}

#[test]
fn a_trace_operand_must_be_a_plain_integer_and_that_is_a_calculation_error() {
    // Usage errors are the ones this binary detects before calling
    // `dz-api`; a well-formed but untraceable operand is exit 1.
    let output = run(&["trace", "add", "4;6", "1"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("plain"));
}

// -------------------------------------------------------------- compare

#[test]
fn compare_reports_all_three_backends_and_labels_the_oracle() {
    let output = run(&["compare", "mul", "25", "34", "--mode", "fixed"]);
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.starts_with("result: 808"), "{}", text);
    for backend in ["mvl", "crt", "table"] {
        assert!(text.contains(backend), "{} missing from {}", backend, text);
    }
    assert!(text.contains("oracle"), "{}", text);
}

/// The column and the sentence that make the table answer its own
/// question. Both are written by `dz-api`; this pins that the terminal
/// actually shows them.
#[test]
fn compare_shows_the_reconstruction_column_and_the_verdict() {
    for args in [
        ["compare", "mul", "25", "34", "--mode", "fixed"],
        ["compare", "sum", "1", "1", "--mode", "fixed"],
    ] {
        let printed = stdout(&run(&args));
        assert!(printed.contains("recon"), "{}", printed);
        // The verdict is soft-wrapped for the terminal, so match against
        // the flattened text rather than against a particular line break.
        let text: String = printed.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(text.contains("verdict: mvl is cheapest"), "{}", printed);
        // The cost model has to be on screen, not implied by a number.
        assert!(
            text.contains("Cost model: primitive invocations + internal->tag reconstructions"),
            "{}",
            printed
        );
        // The interesting half: equal counters, unequal cost.
        assert!(text.contains("tie on all five primitive counters"), "{}", printed);
        assert!(text.contains("broken by reconstructions"), "{}", printed);
        assert!(text.contains("not ranked"), "{}", printed);
    }
}

/// The CLI must not compose the verdict itself: what it prints is the
/// string `dz-api` wrote, wrapped for the terminal and nothing else.
#[test]
fn the_verdict_the_cli_prints_is_the_one_dz_api_wrote() {
    let comparison = compare_backends(
        &Operation::Mul { a: "25".into(), b: "34".into() },
        NumberMode::Fixed,
        dz_api::default_scale(),
        DisplayAlphabet::canonical(),
        InputBase::Twelve,
    )
    .expect("compares");
    let summary = comparison.verdict.expect("a verdict is written").summary;
    let printed = stdout(&run(&["compare", "mul", "25", "34", "--mode", "fixed"]));
    let flattened: String = printed.split_whitespace().collect::<Vec<_>>().join(" ");
    let expected: String = summary.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(flattened.contains(&expected), "{}", printed);
}

#[test]
fn compare_json_carries_the_reconstructions_and_the_verdict() {
    let line = stdout(&run(&["compare", "mul", "25", "34", "--mode", "fixed", "--json"]));
    assert!(
        line.contains("\"backend\":\"mvl\",\"oracle\":false"),
        "{}",
        line
    );
    assert!(line.contains("\"reconstruction_ops\":0"), "{}", line);
    assert!(
        line.contains("\"verdict\":{\"winner\":\"mvl\",\"decided_by_reconstructions\":true"),
        "{}",
        line
    );
}

/// A plain call reports its own reconstructions too, so `--backend crt`
/// is honest without asking for a comparison.
#[test]
fn a_plain_gates_line_reports_reconstructions_per_backend() {
    let mvl = stdout(&run(&["mul", "25", "34", "--mode", "fixed", "--gates"]));
    assert!(mvl.contains("recon=0"), "{}", mvl);
    let crt = stdout(&run(&[
        "mul", "25", "34", "--mode", "fixed", "--gates", "--backend", "crt",
    ]));
    assert!(!crt.contains("recon=0"), "CRT reconstructs: {}", crt);
    assert!(crt.contains("recon="), "{}", crt);
}

#[test]
fn compare_takes_the_same_arguments_as_the_operation_it_measures() {
    let plain = stdout(&run(&["mul", "25", "34", "--mode", "fixed"]));
    let compared = stdout(&run(&["compare", "mul", "25", "34", "--mode", "fixed"]));
    assert!(compared.contains(plain.trim()), "{} / {}", plain, compared);
}

// ------------------------------------------------------------------ CSV

#[test]
fn the_csv_the_cli_prints_is_byte_for_byte_the_one_dz_api_builds() {
    let request = TraceRequest {
        backend: BackendKind::Mvl,
        kind: TraceKind::Add,
        a: "EEE".to_string(),
        b: "1".to_string(),
        alphabet: DisplayAlphabet::canonical(),
        input_base: InputBase::Twelve,
    };
    let expected = trace_csv(&trace(&request).expect("traces"));
    assert_eq!(stdout(&run(&["trace", "add", "EEE", "1", "--csv"])), expected);

    let summary = trace_summary_csv(&trace(&request).expect("traces"));
    assert_eq!(
        stdout(&run(&["trace", "add", "EEE", "1", "--csv-summary"])),
        summary
    );

    let comparison = compare_backends(
        &Operation::Sum { a: "1".into(), b: "3".into() },
        NumberMode::Rational,
        dz_api::default_scale(),
        DisplayAlphabet::canonical(),
        InputBase::Twelve,
    )
    .expect("compares");
    assert_eq!(
        stdout(&run(&["compare", "sum", "1", "3", "--csv"])),
        gate_count_csv(&comparison.costs)
    );
}

#[test]
fn a_csv_path_writes_the_same_document_to_a_file() {
    let path = std::env::temp_dir().join("dz-cli-phase9-trace.csv");
    let argument = format!("--csv={}", path.display());
    let output = run(&["trace", "add", "EEE", "1", &argument]);
    assert!(output.status.success(), "{}", stdout(&output));
    let written = std::fs::read_to_string(&path).expect("the file was written");
    assert_eq!(written, stdout(&run(&["trace", "add", "EEE", "1", "--csv"])));
    let _ = std::fs::remove_file(&path);
}

#[test]
fn csv_does_not_apply_to_a_plain_call() {
    let output = run(&["sum", "1", "3", "--csv"]);
    assert_eq!(output.status.code(), Some(2));
    // ... and the trace-only document does not apply to `compare`.
    assert_eq!(
        run(&["compare", "sum", "1", "3", "--csv-summary"]).status.code(),
        Some(2)
    );
}

// ----------------------------------------------------------------- JSON

#[test]
fn json_covers_the_new_outputs_in_one_line_each() {
    let line = stdout(&run(&["trace", "add", "EEE", "1", "--json"]));
    assert_eq!(line.lines().count(), 1, "{}", line);
    assert!(line.contains("\"kind\":\"add\""), "{}", line);
    assert!(line.contains("\"steps\":["), "{}", line);

    let line = stdout(&run(&["compare", "mul", "25", "34", "--mode", "fixed", "--json"]));
    assert_eq!(line.lines().count(), 1, "{}", line);
    assert!(line.contains("\"backend\":\"table\",\"oracle\":true"), "{}", line);

    // Errors keep the shape `--json` has always had.
    let output = run(&["trace", "div", "1", "2", "--json"]);
    assert_eq!(output.status.code(), Some(2));
}

// ------------------------------------------------------- reserved names

#[test]
fn trace_and_compare_are_reserved_names_not_operations() {
    // `dz-cli <function>` turns any registered function into a
    // subcommand, so these two names are claimed explicitly rather than
    // left to the order of a `match` (`docs/06`, section 7).
    for name in ["trace", "compare"] {
        let output = run(&["compare", name, "1", "2"]);
        assert_eq!(output.status.code(), Some(2), "{}", name);
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        assert!(error.contains("top-level command"), "{}", error);
    }
}

// -------------------------------------------------------- REPL session

fn repl(script: &str) -> String {
    use std::io::Write;
    use std::process::Stdio;
    let mut child = Command::new(env!("CARGO_BIN_EXE_dz-cli"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("the REPL starts");
    child
        .stdin
        .as_mut()
        .expect("stdin is piped")
        .write_all(script.as_bytes())
        .expect("the script is written");
    let output = child.wait_with_output().expect("the REPL exits");
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn the_repl_keeps_a_history_and_a_memory_register() {
    let text = repl("1+1\n:mem+\n:history\n:session\n:quit\n");
    assert!(text.contains("memory = 2"), "{}", text);
    assert!(text.contains("1  1+1  =  2"), "{}", text);
    assert!(text.contains("history = 1 entries"), "{}", text);
    assert!(text.contains("backend = mvl"), "{}", text);
    assert!(text.contains("scale = 12 fractional digits"), "{}", text);
}

#[test]
fn clearing_the_memory_and_the_history_empties_them() {
    let text = repl("1+1\n:mem+\n:memc\n:mem\n:clear\n:history\n:quit\n");
    assert!(text.contains("memory is empty"), "{}", text);
    assert!(text.contains("history is empty"), "{}", text);
}

#[test]
fn a_recalled_memory_value_is_text_that_parses_back() {
    // The memory holds the formatted string, so recalling it is feeding
    // that text through `dz-parse` again -- the CLI never recomposes a
    // dozenal value itself.
    let text = repl("X+2\n:mem+\n:mem\n:quit\n");
    assert!(text.contains("memory = 10"), "{}", text);
    assert_eq!(stdout(&run(&["eval", "10"])).trim(), "10");
}

#[test]
fn the_help_text_documents_everything_new() {
    // A command nobody can find is a command that does not exist.
    let text = stdout(&run(&["--help"]));
    for expected in [
        "trace add|sub|mul",
        "compare <operation>",
        "--csv",
        "--csv-summary",
        ":mem",
        ":history",
        ":session",
    ] {
        assert!(text.contains(expected), "{} missing from help", expected);
    }
}
