//! The configurable presentation alphabet, checked against the *real*
//! function catalog rather than a hand-written name list.
//!
//! `dz-eval` is a dev-dependency of `dz-parse` (it already was, for the
//! call-syntax round trips), which is what lets this test use
//! `default_registry()` without `dz-parse` depending upwards at build
//! time. Production code gets the catalog through `dz_api::display_alphabet`.

use dz_parse::{lex, parse_str, DisplayAlphabet, Token};

fn catalog() -> Vec<String> {
    dz_eval::default_registry()
        .catalog()
        .into_iter()
        .map(|info| info.name.to_string())
        .collect()
}

#[test]
fn a_over_b_passes_against_the_real_catalog() {
    // `abs`, `acos` and `atan` all contain letters outside the pair, so
    // none of them turns into a number literal.
    let alphabet = DisplayAlphabet::new('a', 'b', &catalog()).expect("a/b must be a usable pair");
    assert_eq!(alphabet.to_canonical("ab").unwrap(), "\u{218A}\u{218B}");
    assert_eq!(alphabet.to_canonical("abs(3)").unwrap(), "abs(3)");
}

#[test]
fn p_over_i_fails_against_the_real_catalog_naming_pi() {
    let error = DisplayAlphabet::new('p', 'i', &catalog())
        .expect_err("p/i makes the constant pi unreachable");
    let message = error.to_string();
    assert!(
        message.contains("'pi'"),
        "the error must name the broken constant: {}",
        message
    );
}

#[test]
fn every_catalog_name_still_lexes_as_an_identifier_under_a_over_b() {
    // The end-to-end version of the `abs(3)` trap: translate each name
    // through the pair and check the lexer still sees a name.
    let alphabet = DisplayAlphabet::new('a', 'b', &catalog()).expect("valid pair");
    for name in catalog() {
        let translated = alphabet
            .to_canonical(&name)
            .unwrap_or_else(|e| panic!("'{}' should translate, got {}", name, e));
        let tokens = lex(&translated).unwrap_or_else(|e| panic!("'{}': {}", name, e));
        assert_eq!(
            tokens,
            vec![Token::Ident(name.clone())],
            "'{}' must stay one identifier",
            name
        );
    }
}

#[test]
fn a_translated_expression_parses_to_the_same_ast_as_the_canonical_one() {
    let alphabet = DisplayAlphabet::new('a', 'b', &catalog()).expect("valid pair");
    for (typed, canonical) in [
        ("a + b", "\u{218A} + \u{218B}"),
        ("ab;a", "\u{218A}\u{218B};\u{218A}"),
        ("abs(-b)", "abs(-\u{218B})"),
        ("sin(a/2)", "sin(\u{218A}/2)"),
    ] {
        let translated = alphabet.to_canonical(typed).expect("translates");
        assert_eq!(translated, canonical, "input {}", typed);
        assert_eq!(
            parse_str(&translated).map_err(|e| e.to_string()),
            parse_str(canonical).map_err(|e| e.to_string()),
            "input {}",
            typed
        );
    }
}
