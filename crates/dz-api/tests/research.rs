//! Phase 7: the research-panel surface of `dz-api`.

use dz_api::{
    compare_backends, comparison_json_line, execute, gate_count_csv, trace, trace_csv,
    trace_json_line, trace_summary_csv, ApiError, BackendKind, NumberMode, Operation, Request,
    DisplayAlphabet, Trace, TraceKind, TraceRequest,
};

fn request(kind: TraceKind, a: &str, b: &str) -> TraceRequest {
    TraceRequest {
        backend: BackendKind::Mvl,
        kind,
        a: a.to_string(),
        b: b.to_string(),
        alphabet: DisplayAlphabet::canonical(),
    }
}

// ------------------------------------------------------------------
// Depth for any user expression.
// ------------------------------------------------------------------

#[test]
fn every_successful_call_reports_a_depth() {
    for text in ["1 + 1", "4;6 * 3", "1/3", "↋↋↋ + 1"] {
        let response = execute(&Request::new(Operation::Eval { expr: text.into() }))
            .unwrap_or_else(|e| panic!("{} should evaluate: {}", text, e));
        let depth = response.depth.expect("a successful call reports depth");
        assert!(depth > 0, "{} should have a nonzero depth", text);
    }
}

/// The invariant the whole depth design rests on: measuring depth does not
/// change the gate count of the operation being measured. `execute` always
/// measures, so this compares against the published headline figure that
/// the untraced circuit produces on its own.
#[test]
fn measuring_depth_does_not_change_the_published_gate_counts() {
    use dz_gates::circuits::{multiply, ripple_add};
    use dz_gates::{Digit, DozGates, MvlGates};

    // Four-digit ripple add: 1212 gates (CLAUDE.md).
    let g = MvlGates::new();
    let a = vec![Digit::ZERO, Digit::ZERO, Digit::ZERO, Digit::ONE];
    let b = vec![Digit::MAX, Digit::MAX, Digit::MAX, Digit::ZERO];
    let _ = ripple_add(&g, &a, &b);
    assert_eq!(g.cost().total(), 1212);

    // 3x2 digit multiply: 9795 gates (CLAUDE.md).
    let g = MvlGates::new();
    let x = vec![Digit::MAX; 3];
    let y = vec![Digit::MAX; 2];
    let _ = multiply(&g, &x, &y);
    assert_eq!(g.cost().total(), 9795);
}

/// Depth composes across the expression: a longer chain of operations is
/// deeper than a single one, even though every layer between `dz-api` and
/// `dz-gates` is unaware that depth exists.
#[test]
fn a_longer_expression_is_deeper() {
    let one = execute(&Request::new(Operation::Eval { expr: "1000 + 1000".into() }))
        .expect("evaluates")
        .depth
        .expect("depth");
    let three = execute(&Request::new(Operation::Eval {
        expr: "1000 + 1000 + 1000 + 1000".into(),
    }))
    .expect("evaluates")
    .depth
    .expect("depth");
    assert!(three > one, "chained depth {} should exceed {}", three, one);
}

#[test]
fn depth_appears_in_the_json_line() {
    let response = execute(&Request::new(Operation::Eval { expr: "1 + 1".into() })).expect("ok");
    let line = dz_api::to_json_line(&response);
    assert!(line.contains("\"depth\":"), "{}", line);
}

// ------------------------------------------------------------------
// Traces.
// ------------------------------------------------------------------

#[test]
fn an_addition_trace_reconstructs_the_sum() {
    let t = trace(&request(TraceKind::Add, "↋↋↋", "1")).expect("traces");
    match t {
        Trace::Add(add) => {
            assert_eq!(add.a, "↋↋↋");
            assert_eq!(add.b, "1", "operands are shown as given; the steps carry positions");
            assert_eq!(add.result, "1000");
            assert_eq!(add.steps.len(), 3);
            // The carry visibly propagates through all three positions.
            assert_eq!(add.steps[0].carry_in, '0');
            for step in &add.steps {
                assert_eq!(step.sum, '0');
                assert_eq!(step.carry_out, '1');
            }
        }
        other => panic!("expected an addition trace, got {:?}", other),
    }
}

#[test]
fn a_subtraction_trace_shows_the_complement_stages() {
    let t = trace(&request(TraceKind::Sub, "1000", "1")).expect("traces");
    match t {
        Trace::Sub(sub) => {
            assert_eq!(sub.result, "↋↋↋");
            assert!(!sub.borrowed);
            // Subtraction really is complement-then-add, and the trace says so.
            assert!(!sub.ones_complement.is_empty());
            assert!(!sub.radix_complement.is_empty());
            assert!(!sub.complement_steps.is_empty());
            assert!(!sub.add_steps.is_empty());
            assert!(sub.decode_steps.is_empty(), "no borrow, no decoding stage");
        }
        other => panic!("expected a subtraction trace, got {:?}", other),
    }
}

#[test]
fn a_borrowing_subtraction_trace_is_marked_and_signed() {
    let t = trace(&request(TraceKind::Sub, "1", "1000")).expect("traces");
    match t {
        Trace::Sub(sub) => {
            assert!(sub.borrowed);
            assert!(sub.result.starts_with('-'), "got {}", sub.result);
            assert!(!sub.decode_steps.is_empty());
        }
        other => panic!("expected a subtraction trace, got {:?}", other),
    }
}

#[test]
fn a_multiplication_trace_is_nested_and_reconstructs_the_product() {
    let t = trace(&request(TraceKind::Mul, "↋↋", "↋↋")).expect("traces");
    match t {
        Trace::Mul(mul) => {
            assert_eq!(mul.rows.len(), 2, "one group per multiplier digit");
            for (i, row) in mul.rows.iter().enumerate() {
                assert_eq!(row.index, i);
                assert_eq!(row.multiplier_digit, '↋');
                assert!(!row.accumulate_steps.is_empty(), "each row has its own sub-trace");
            }
            // The last row's accumulator is the product: ↋↋ x ↋↋ = ↋↊01.
            assert_eq!(mul.rows.last().unwrap().accumulated, mul.result);
            assert_eq!(mul.result, "↋↊01");
        }
        other => panic!("expected a multiplication trace, got {:?}", other),
    }
}

/// The traced result must agree with what the ordinary calculator says, or
/// the viewer is showing a different computation from the one the user ran.
#[test]
fn traces_agree_with_the_ordinary_evaluator() {
    let cases = [
        (TraceKind::Add, "↋↋↋", "1", Operation::Sum { a: "↋↋↋".into(), b: "1".into() }),
        (TraceKind::Sub, "1000", "1", Operation::Sub { a: "1000".into(), b: "1".into() }),
        (TraceKind::Mul, "↋↋", "↋↋", Operation::Mul { a: "↋↋".into(), b: "↋↋".into() }),
    ];
    for (kind, a, b, op) in cases {
        let traced = trace(&request(kind, a, b)).expect("traces");
        let evaluated = execute(&Request::new(op).with_mode(NumberMode::Fixed)).expect("evaluates");
        // The trace keeps every digit position (so rows line up); the
        // formatter trims leading zeros. Compare after trimming.
        let trimmed = traced.result().trim_start_matches('0');
        let trimmed = if trimmed.is_empty() { "0" } else { trimmed };
        assert_eq!(trimmed, evaluated.display, "{:?} {} {}", kind, a, b);
    }
}

#[test]
fn traces_are_identical_across_all_three_backends() {
    for kind in TraceKind::ALL {
        let mut reference: Option<Trace> = None;
        for backend in BackendKind::ALL {
            let mut req = request(kind, "1728", "↋");
            req.backend = backend;
            let t = trace(&req).expect("traces");
            match &reference {
                Some(first) => assert_eq!(first, &t, "{:?} on {}", kind, backend.label()),
                None => reference = Some(t),
            }
        }
    }
}

#[test]
fn the_viewer_refuses_what_a_single_circuit_run_cannot_represent() {
    for (a, b) in [("1;6", "2"), ("1+1", "2"), ("sin(1)", "2"), ("", "2")] {
        let error = trace(&request(TraceKind::Add, a, b))
            .expect_err("non-integer operands must be rejected");
        assert!(
            matches!(error, ApiError::Trace(_) | ApiError::Parse(_)),
            "unexpected error for '{}': {:?}",
            a,
            error
        );
    }
}

// ------------------------------------------------------------------
// Backend comparison and CSV.
// ------------------------------------------------------------------

#[test]
fn comparing_backends_runs_all_three_and_agrees_on_the_answer() {
    let comparison = compare_backends(
        &Operation::Mul { a: "4;6".into(), b: "3".into() },
        NumberMode::Fixed,
        dz_api::default_scale(),
        DisplayAlphabet::canonical(),
    )
    .expect("compares");
    assert_eq!(comparison.display, "11;6");
    assert_eq!(comparison.costs.len(), 3);
    for (row, expected) in comparison.costs.iter().zip(BackendKind::ALL) {
        assert_eq!(row.backend, expected);
        assert!(row.gate_count.total() > 0);
        assert!(row.depth > 0);
    }
}

#[test]
fn the_gate_csv_has_a_header_and_one_row_per_backend() {
    let comparison = compare_backends(
        &Operation::Sum { a: "1".into(), b: "3".into() },
        NumberMode::Fixed,
        dz_api::default_scale(),
        DisplayAlphabet::canonical(),
    )
    .expect("compares");
    let csv = gate_count_csv(&comparison.costs);
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines[0],
        "backend,cycle,min,max,lit,thr,total,depth,reconstruction_ops,oracle"
    );
    assert_eq!(lines.len(), 4);
    assert!(lines[1].starts_with("mvl,"));
    assert!(lines[2].starts_with("crt,"));
    assert!(lines[3].starts_with("table,"));
    // The two columns the five counters cannot supply: only CRT
    // reconstructs, and only `table` is the oracle.
    assert!(lines[1].ends_with(",0,false"), "{}", lines[1]);
    assert!(lines[3].ends_with(",0,true"), "{}", lines[3]);
    let crt: Vec<&str> = lines[2].split(',').collect();
    assert_ne!(crt[crt.len() - 2], "0", "{}", lines[2]);
    assert_eq!(crt[crt.len() - 1], "false");
}

// ------------------------------------------------------------------
// Reconstructions and the verdict.
// ------------------------------------------------------------------

/// The measurement the whole verdict rests on: the five primitive
/// counters are equal across backends by construction, and the
/// reconstruction column is the one that is not
/// (`docs/02-crt-vs-mvl.md`).
#[test]
fn only_crt_reports_reconstructions() {
    for op in [
        Operation::Mul { a: "25".into(), b: "34".into() },
        Operation::Sum { a: "1".into(), b: "1".into() },
    ] {
        let comparison =
            compare_backends(&op, NumberMode::Fixed, dz_api::default_scale(), DisplayAlphabet::canonical()).expect("compares");
        for row in &comparison.costs {
            match row.backend {
                BackendKind::Crt => assert!(
                    row.reconstruction_ops > 0,
                    "CRT rebuilds a digit for every order comparison"
                ),
                _ => assert_eq!(
                    row.reconstruction_ops, 0,
                    "{} stores the tag itself; zero is the truth, not a gap",
                    row.backend.label()
                ),
            }
        }
    }
}

#[test]
fn a_plain_response_carries_the_same_reconstruction_count() {
    // `Response.reconstruction_ops` and the comparison row must agree:
    // they are the same measurement taken through two entry points.
    let op = Operation::Mul { a: "25".into(), b: "34".into() };
    let comparison =
        compare_backends(&op, NumberMode::Fixed, dz_api::default_scale(), DisplayAlphabet::canonical()).expect("compares");
    for backend in BackendKind::ALL {
        let response = execute(
            &dz_api::Request::new(op.clone())
                .with_mode(NumberMode::Fixed)
                .with_backend(backend),
        )
        .expect("evaluates");
        let row = comparison
            .costs
            .iter()
            .find(|c| c.backend == backend)
            .expect("every backend is in the comparison");
        assert_eq!(response.reconstruction_ops, Some(row.reconstruction_ops));
    }
}

#[test]
fn the_verdict_names_mvl_and_says_the_tie_was_broken_by_reconstructions() {
    for op in [
        Operation::Mul { a: "25".into(), b: "34".into() },
        Operation::Sum { a: "1".into(), b: "1".into() },
    ] {
        let comparison =
            compare_backends(&op, NumberMode::Fixed, dz_api::default_scale(), DisplayAlphabet::canonical()).expect("compares");
        let verdict = comparison.verdict.as_ref().expect("a verdict is written");
        // Consistent with docs/02: CRT never wins for these circuits.
        assert_eq!(verdict.winner, BackendKind::Mvl);
        assert!(
            verdict.decided_by_reconstructions,
            "the five counters tie, so the reconstructions decide"
        );
        // The cost model must be visible in the prose, not implied by a
        // number; and the oracle must be excluded out loud.
        assert!(verdict.summary.contains(&verdict.model), "{}", verdict.summary);
        assert!(verdict.summary.contains("reconstructions"), "{}", verdict.summary);
        assert!(
            verdict.summary.contains("not ranked"),
            "the oracle row is shown but excluded: {}",
            verdict.summary
        );
    }
}

/// The oracle never wins, even if its numbers were the smallest: it is
/// not a production backend, so it is not in the ranking at all.
#[test]
fn the_verdict_never_names_the_oracle() {
    let comparison = compare_backends(
        &Operation::Sum { a: "1".into(), b: "1".into() },
        NumberMode::Fixed,
        dz_api::default_scale(),
        DisplayAlphabet::canonical(),
    )
    .expect("compares");
    let mut costs = comparison.costs.clone();
    // Make the oracle cheapest on every key; it must still lose.
    for row in costs.iter_mut() {
        if row.is_oracle() {
            row.gate_count = dz_api::GateCount::default();
            row.depth = 0;
        }
    }
    let verdict = dz_api::verdict(&costs).expect("a verdict is written");
    assert_eq!(verdict.winner, BackendKind::Mvl);
}

#[test]
fn the_trace_csv_has_one_row_per_step_for_every_kind() {
    let add = trace(&request(TraceKind::Add, "↋↋↋", "1")).expect("traces");
    let csv = trace_csv(&add);
    assert_eq!(
        csv.lines().next().unwrap(),
        "kind,stage,group,position,a,b,carry_in,sum,carry_out"
    );
    assert_eq!(csv.lines().count(), 4, "header plus three positions");

    let sub = trace(&request(TraceKind::Sub, "1", "1000")).expect("traces");
    let csv = trace_csv(&sub);
    assert!(csv.contains(",complement,"));
    assert!(csv.contains(",add,"));
    assert!(csv.contains(",decode,"));

    let mul = trace(&request(TraceKind::Mul, "↋↋", "↋↋")).expect("traces");
    let csv = trace_csv(&mul);
    assert!(csv.contains("mul,accumulate,0,"));
    assert!(csv.contains("mul,accumulate,1,"));
}

#[test]
fn the_trace_summary_csv_carries_the_context_the_step_table_lacks() {
    let mul = trace(&request(TraceKind::Mul, "↋↋", "↋↋")).expect("traces");
    let csv = trace_summary_csv(&mul);
    assert_eq!(csv.lines().next().unwrap(), "kind,field,value");
    assert!(csv.contains("mul,row0_partial,"));
    assert!(csv.contains("mul,result,↋↊01"));

    let sub = trace(&request(TraceKind::Sub, "1", "1000")).expect("traces");
    let csv = trace_summary_csv(&sub);
    assert!(csv.contains("sub,borrowed,true"));
    assert!(csv.contains("sub,radix_complement,"));
}

// ------------------------------------------------------------------
// Phase 9: the JSON documents. Same rule as the CSV -- one formatting
// authority, so an interface prints these bytes and never rebuilds them.
// ------------------------------------------------------------------

#[test]
fn the_trace_json_is_one_line_and_carries_every_stage() {
    let add = trace_json_line(&trace(&request(TraceKind::Add, "↋↋↋", "1")).expect("traces"));
    assert!(!add.contains('\n'), "one line: {}", add);
    assert!(add.starts_with("{\"ok\":true,\"kind\":\"add\""), "{}", add);
    assert!(add.contains("\"result\":\"1000\""), "{}", add);
    assert!(add.contains("\"steps\":[{\"position\":0,"), "{}", add);

    let sub = trace_json_line(&trace(&request(TraceKind::Sub, "1", "1000")).expect("traces"));
    // The five real stages are named, not hidden behind a fake borrow.
    for field in [
        "\"kind\":\"sub\"",
        "\"ones_complement\"",
        "\"complement_steps\"",
        "\"radix_complement\"",
        "\"add_steps\"",
        "\"raw_sum\"",
        "\"borrowed\":true",
        "\"decode_steps\"",
    ] {
        assert!(sub.contains(field), "{} missing from {}", field, sub);
    }

    let mul = trace_json_line(&trace(&request(TraceKind::Mul, "↋↋", "↋↋")).expect("traces"));
    assert!(mul.contains("\"rows\":[{\"index\":0,"), "{}", mul);
    assert!(mul.contains("\"accumulate_steps\":"), "{}", mul);
    assert!(mul.contains("\"result\":\"↋↊01\""), "{}", mul);
}

#[test]
fn the_comparison_json_labels_the_oracle() {
    let comparison = compare_backends(
        &Operation::Mul { a: "25".into(), b: "34".into() },
        NumberMode::Fixed,
        dz_api::default_scale(),
        DisplayAlphabet::canonical(),
    )
    .expect("compares");
    let line = comparison_json_line(&comparison);
    assert!(!line.contains('\n'), "one line: {}", line);
    assert!(line.starts_with("{\"ok\":true,\"display\":\""), "{}", line);
    // `table` is the oracle; the document says so itself, so a reader that
    // never saw docs/02 cannot read its row as "table wins".
    assert!(
        line.contains("\"backend\":\"mvl\",\"oracle\":false"),
        "{}",
        line
    );
    assert!(
        line.contains("\"backend\":\"table\",\"oracle\":true"),
        "{}",
        line
    );
    assert!(line.contains("\"depth\":"), "{}", line);
}

#[test]
fn the_json_documents_escape_what_needs_escaping() {
    // Every dozenal glyph passes through untouched; the escaping that
    // exists is the one `escape_json` already performs for quotes and
    // control characters, shared with `to_json_line`.
    let add = trace_json_line(&trace(&request(TraceKind::Add, "↊↋", "1")).expect("traces"));
    assert!(add.contains("\"a\":\"↊↋\""), "{}", add);
    assert!(!add.contains("\\u"), "{}", add);
}
