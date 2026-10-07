//! `dz_api::theory`: divrem, valuation, divisible and the bounded
//! quantifiers, through the public API only. Expected values are dozenal
//! text; none is computed natively.

use dz_api::{
    display_alphabet, execute, execute_theory, functions, theory_json_line, ApiError, InputBase,
    Operation, Quantifier, Request, TheoryOp, TheoryRequest, DEFAULT_BOUND_VAR,
};

fn run(op: TheoryOp) -> Result<dz_api::TheoryResponse, ApiError> {
    execute_theory(&TheoryRequest::new(op))
}

fn bounded(q: Quantifier, bound: &str, body: &str) -> TheoryOp {
    TheoryOp::Bounded {
        quantifier: q,
        bound: bound.to_string(),
        var: DEFAULT_BOUND_VAR.to_string(),
        body: body.to_string(),
    }
}

#[test]
fn divrem_is_one_pass_and_matches_the_separate_operations() {
    // 8820 = 7 * 12X6 + 6 (dozenal), from the CLI example.
    let r = run(TheoryOp::DivRem { a: "8820".into(), b: "7".into() }).unwrap();
    assert_eq!(r.field("quotient"), Some("12\u{218A}6"));
    assert_eq!(r.field("remainder"), Some("6"));
    // One div_rem pass costs less than `div` and `mod` run separately.
    let one_pass = r.gate_count.total();
    let mod_only = execute(&Request::new(Operation::Mod { a: "8820".into(), b: "7".into() }))
        .unwrap()
        .gate_count
        .unwrap()
        .total();
    assert!(one_pass > 0 && one_pass < 2 * mod_only, "{} vs {}", one_pass, mod_only);
}

#[test]
fn divrem_signs_follow_the_dividend_and_zero_divisor_is_an_error() {
    let r = run(TheoryOp::DivRem { a: "-8820".into(), b: "7".into() }).unwrap();
    assert_eq!(r.field("remainder"), Some("-6"));
    let error = run(TheoryOp::DivRem { a: "5".into(), b: "0".into() }).unwrap_err();
    assert!(matches!(error, ApiError::Eval(_)), "{}", error);
}

#[test]
fn operands_must_be_exact_integers() {
    for text in ["1/2", "4;6", "sqrt(2)"] {
        let error = run(TheoryOp::DivRem { a: text.into(), b: "3".into() }).unwrap_err();
        assert!(matches!(error, ApiError::Theory(_)), "{}: {}", text, error);
    }
}

#[test]
fn valuation_reports_exponent_and_cofactor() {
    let r = run(TheoryOp::Valuation { n: "8820".into(), p: "5".into() }).unwrap();
    assert_eq!(r.field("exponent"), Some("4"));
    assert_eq!(r.field("cofactor"), Some("20"));
}

#[test]
fn valuation_error_paths() {
    let zero = run(TheoryOp::Valuation { n: "0".into(), p: "2".into() }).unwrap_err();
    assert!(matches!(zero, ApiError::Theory(_)));
    for p in ["1", "0", "-3"] {
        let e = run(TheoryOp::Valuation { n: "24".into(), p: p.into() }).unwrap_err();
        assert!(matches!(e, ApiError::Theory(_)), "p = {}", p);
    }
}

#[test]
fn divisible_reports_its_method() {
    let a = run(TheoryOp::Divisible { n: "8820".into(), d: "4".into() }).unwrap();
    assert_eq!(a.field("divisible"), Some("true"));
    assert_eq!(a.field("method"), Some("last-digit"));
    let b = run(TheoryOp::Divisible { n: "8820".into(), d: "7".into() }).unwrap();
    assert_eq!(b.field("divisible"), Some("false"));
    assert_eq!(b.field("method"), Some("div_rem"));
    let c = run(TheoryOp::Divisible { n: "8820".into(), d: "10".into() }).unwrap();
    assert_eq!(c.field("method"), Some("last-digit"), "ten (dozenal) is twelve");
    assert!(run(TheoryOp::Divisible { n: "8820".into(), d: "0".into() }).is_err());
}

#[test]
fn exists_finds_the_first_witness_and_short_circuits() {
    // 8909 (dozenal) is 123 squared; 123 is X3.
    let r = run(bounded(Quantifier::Exists, "1000", "i*i = 8909")).unwrap();
    assert_eq!(r.field("holds"), Some("true"));
    assert_eq!(r.field("witness"), Some("\u{218A}3"));
    assert_eq!(r.field("checked"), Some("\u{218A}4"), "witness + 1 candidates ran");
}

#[test]
fn exists_over_an_empty_range_and_a_miss_are_false() {
    let none = run(bounded(Quantifier::Exists, "0", "1")).unwrap();
    assert_eq!(none.field("holds"), Some("false"));
    assert_eq!(none.field("checked"), Some("0"));
    // 8820 (dozenal, 15000 decimal) is not a square.
    let miss = run(bounded(Quantifier::Exists, "100", "i*i = 8820")).unwrap();
    assert_eq!(miss.field("holds"), Some("false"));
    assert_eq!(miss.field("checked"), Some("100"));
    assert_eq!(miss.field("witness"), None);
}

#[test]
fn forall_is_vacuously_true_and_reports_a_counterexample() {
    let vacuous = run(bounded(Quantifier::Forall, "0", "0")).unwrap();
    assert_eq!(vacuous.field("holds"), Some("true"));
    let all = run(bounded(Quantifier::Forall, "20", "i*i >= i")).unwrap();
    assert_eq!(all.field("holds"), Some("true"));
    let broken = run(bounded(Quantifier::Forall, "20", "i < 5")).unwrap();
    assert_eq!(broken.field("holds"), Some("false"));
    assert_eq!(broken.field("counterexample"), Some("5"));
    assert_eq!(broken.field("checked"), Some("6"));
}

#[test]
fn a_body_without_a_relation_holds_when_nonzero() {
    // i - 3 is zero only at i = 3, so "forall i<10: i-3" fails exactly there.
    let r = run(bounded(Quantifier::Forall, "10", "i - 3")).unwrap();
    assert_eq!(r.field("counterexample"), Some("3"));
}

#[test]
fn every_relation_is_decided() {
    for (body, expected) in [
        ("i != 2", "2"),
        ("i <= 1", "2"),
        ("i > 1", "0"),
        ("i >= 1", "0"),
        ("i = 7", "0"),
    ] {
        let r = run(bounded(Quantifier::Forall, "10", body)).unwrap();
        assert_eq!(r.field("counterexample"), Some(expected), "body {}", body);
    }
}

#[test]
fn the_bound_may_be_an_expression_and_the_variable_may_be_renamed() {
    let r = run(TheoryOp::Bounded {
        quantifier: Quantifier::Exists,
        bound: "2*6".into(),
        var: "k".into(),
        body: "k*k = 9".into(),
    })
    .unwrap();
    assert_eq!(r.field("witness"), Some("3"));
}

#[test]
fn bad_variable_names_are_rejected_with_a_reason() {
    for var in ["x", "t", "e", "X", "xe", "pi", "sin", "1", "a b", ""] {
        let e = run(TheoryOp::Bounded {
            quantifier: Quantifier::Exists,
            bound: "3".into(),
            var: var.into(),
            body: "1".into(),
        })
        .unwrap_err();
        assert!(matches!(e, ApiError::Theory(_)), "var {:?}: {}", var, e);
    }
}

#[test]
fn a_variable_the_alphabet_reads_as_a_digit_is_rejected() {
    // With `i` as the symbol for ten, `i` is a number, not a name.
    let alphabet = display_alphabet('i', 'j').unwrap();
    let e = execute_theory(
        &TheoryRequest::new(bounded(Quantifier::Exists, "3", "i")).with_alphabet(alphabet),
    )
    .unwrap_err();
    assert!(matches!(e, ApiError::Theory(_)), "{}", e);
}

#[test]
fn approximate_or_malformed_bodies_are_errors_not_guesses() {
    let approx = run(bounded(Quantifier::Exists, "3", "sin(i) = 0")).unwrap_err();
    assert!(matches!(approx, ApiError::Theory(_)), "{}", approx);
    for body in ["i = 1 = 2", "= 3", "3 <"] {
        let e = run(bounded(Quantifier::Exists, "3", body)).unwrap_err();
        assert!(matches!(e, ApiError::Theory(_)), "body {:?}", body);
    }
    let negative = run(bounded(Quantifier::Exists, "-1", "1")).unwrap_err();
    assert!(matches!(negative, ApiError::Theory(_)));
}

#[test]
fn the_variable_is_a_real_substitution_not_a_text_replace() {
    // `abs(i)` must see the counter inside the call, and a longer name
    // that merely contains `i` (`pi`) must be left alone.
    let r = run(bounded(Quantifier::Exists, "10", "abs(i) = 4")).unwrap();
    assert_eq!(r.field("witness"), Some("4"));
    let pi = run(bounded(Quantifier::Exists, "2", "pi > 3")).unwrap_err();
    assert!(matches!(pi, ApiError::Theory(_)), "pi is approximate: {}", pi);
}

#[test]
fn cost_is_aggregate_and_roughly_linear_in_the_candidates() {
    let small = run(bounded(Quantifier::Exists, "10", "i*i = 8820")).unwrap();
    let large = run(bounded(Quantifier::Exists, "20", "i*i = 8820")).unwrap();
    let (s, l) = (small.gate_count.total(), large.gate_count.total());
    assert!(l > s && l < 3 * s, "{} then {}", s, l);
    // Depth is that of one candidate, so it does not grow with the bound.
    assert!(large.depth < 2 * small.depth, "{} then {}", small.depth, large.depth);
}

#[test]
fn base_ten_operands_work_and_are_billed() {
    let twelve = run(TheoryOp::Valuation { n: "8820".into(), p: "5".into() }).unwrap();
    let ten = execute_theory(
        &TheoryRequest::new(TheoryOp::Valuation { n: "15000".into(), p: "5".into() })
            .with_input_base(InputBase::Ten),
    )
    .unwrap();
    assert_eq!(ten.field("exponent"), twelve.field("exponent"));
    assert!(ten.gate_count.total() > twelve.gate_count.total());
}

#[test]
fn json_has_the_fields_then_the_measurements() {
    let r = run(TheoryOp::Divisible { n: "8820".into(), d: "4".into() }).unwrap();
    let line = theory_json_line(&r);
    assert!(
        line.starts_with("{\"ok\":true,\"divisible\":true,\"method\":\"last-digit\""),
        "{}",
        line
    );
    assert!(line.contains("\"gate_count\":{") && line.contains("\"depth\":"), "{}", line);
}

#[test]
fn the_functions_are_registered_and_stay_off_the_gui_keypad() {
    for name in ["valuation", "divisible"] {
        let info = functions().into_iter().find(|f| f.name == name).expect(name);
        assert!(info.cli_only, "{} must be CLI only", name);
    }
    let v = execute(&Request::new(Operation::Eval { expr: "valuation(8820, 5)".into() })).unwrap();
    assert_eq!(v.display, "4");
    let d = execute(&Request::new(Operation::Eval {
        expr: "divisible(8820, 6) + divisible(8820, 7)".into(),
    }))
    .unwrap();
    assert_eq!(d.display, "1");
    let bad = execute(&Request::new(Operation::Eval { expr: "valuation(0, 2)".into() }));
    assert!(bad.is_err());
}
