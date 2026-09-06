//! The presentation alphabet as an interface sees it
//! (`docs/07-alfabeto-configurable.md`).
//!
//! `dz-api` is the layer that owns *when* to translate; `dz-parse` owns
//! *how*. These tests pin the "when": input translated before the lexer,
//! output left canonical, JSON and CSV untouched.

use dz_api::{
    InputBase,
    display_alphabet, execute, gate_count_csv, to_json_line, trace, trace_csv, BackendKind,
    DisplayAlphabet, NumberMode, Operation, Request, TraceKind, TraceRequest,
};

fn ab() -> DisplayAlphabet {
    display_alphabet('a', 'b').expect("a/b is a valid pair against the real catalog")
}

fn eval(text: &str, alphabet: DisplayAlphabet) -> String {
    execute(
        &Request::new(Operation::Eval { expr: text.to_string() })
            .with_mode(NumberMode::Fixed)
            .with_alphabet(alphabet),
    )
    .expect("expression evaluates")
    .display
}

#[test]
fn a_over_b_is_accepted_and_p_over_i_is_not() {
    assert!(display_alphabet('a', 'b').is_ok());
    let rejected = display_alphabet('p', 'i').expect_err("p/i swallows 'pi'");
    assert!(rejected.to_string().contains("'pi'"), "{}", rejected);
}

#[test]
fn abs_still_works_when_a_and_b_are_digits() {
    // The trap of `docs/07` section 2: a naive character substitution
    // turns `abs(3)` into a call to a function that does not exist.
    assert_eq!(eval("abs(-3)", ab()), "3");
    // ...while the same two letters on their own are the number
    // ten-eleven, which is what makes it a trap in the first place.
    assert_eq!(eval("ab", ab()), eval("\u{218A}\u{218B}", DisplayAlphabet::canonical()));
}

#[test]
fn output_stays_canonical_so_the_interface_decides_when_to_repaint() {
    // `execute` translates input only. `display` is canonical whatever
    // alphabet went in, which is what lets the GUI keep one history and
    // repaint it (`docs/07` section 3.5).
    let response = execute(
        &Request::new(Operation::Eval { expr: "b".to_string() })
            .with_mode(NumberMode::Fixed)
            .with_alphabet(ab()),
    )
    .expect("evaluates");
    assert!(response.display.contains('\u{218B}'), "{}", response.display);
    assert_eq!(ab().from_canonical(&response.display), "b");
}

#[test]
fn the_json_line_is_not_translated() {
    // An exchange document must not depend on a visual preference: the
    // JSON is built from the canonical `display`.
    let response = execute(
        &Request::new(Operation::Eval { expr: "b".to_string() })
            .with_mode(NumberMode::Fixed)
            .with_alphabet(ab()),
    )
    .expect("evaluates");
    let line = to_json_line(&response);
    assert!(line.contains('\u{218B}'), "{}", line);
    assert!(!line.contains("\"b;"), "{}", line);
}

#[test]
fn a_trace_reads_its_operands_in_the_alphabet_but_exports_canonical_csv() {
    let request = TraceRequest {
        backend: BackendKind::Mvl,
        kind: TraceKind::Add,
        a: "bbb".to_string(),
        b: "1".to_string(),
        alphabet: ab(),
        input_base: InputBase::Twelve,
    };
    let typed = trace(&request).expect("traces");
    let canonical = trace(&TraceRequest {
        alphabet: DisplayAlphabet::canonical(),
        a: "\u{218B}\u{218B}\u{218B}".to_string(),
        ..request
    })
    .expect("traces");
    // Same run, spelled two ways on the way in.
    assert_eq!(typed, canonical);
    assert!(trace_csv(&typed).contains('\u{218B}'));
}

#[test]
fn the_gate_csv_is_unaffected_by_the_alphabet() {
    let op = Operation::Sum { a: "b".into(), b: "1".into() };
    let comparison = dz_api::compare_backends(&op, NumberMode::Fixed, 4, ab(), InputBase::Twelve).expect("compares");
    let csv = gate_count_csv(&comparison.costs);
    // The gate table carries no dozenal digits at all; the check is that
    // it still builds and still has one row per backend when a
    // non-canonical alphabet was used to type the operands.
    assert_eq!(csv.lines().count(), 1 + BackendKind::ALL.len());
}

#[test]
fn a_legacy_alias_is_a_syntax_error_under_a_configured_pair() {
    // `docs/07` section 3.3: with `a`/`b` chosen, `E` is not eleven.
    let error = execute(
        &Request::new(Operation::Eval { expr: "E".to_string() }).with_alphabet(ab()),
    )
    .expect_err("E is no longer a digit");
    assert!(matches!(error, dz_api::ApiError::Alphabet(_)), "{:?}", error);
    // ...but the canonical glyphs always are.
    assert_eq!(eval("\u{218B}", ab()), eval("\u{218B}", DisplayAlphabet::canonical()));
}

#[test]
fn the_default_alphabet_changes_nothing_at_all() {
    for text in ["1+1", "XE", "abs(3)", "exp(1)", "4;6*3", "1,0000"] {
        assert_eq!(
            eval(text, DisplayAlphabet::canonical()),
            execute(
                &Request::new(Operation::Eval { expr: text.to_string() })
                    .with_mode(NumberMode::Fixed)
            )
            .expect("evaluates")
            .display,
            "input {}",
            text
        );
    }
}
