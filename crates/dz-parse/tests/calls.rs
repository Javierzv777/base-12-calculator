//! Lexing and parsing of function calls, and the two rules that let call
//! syntax coexist with the dozenal alphabet (see `lexer.rs`): letters that
//! are digit aliases, and a comma that means two different things.

use dz_gates::Digit;
use dz_parse::{lex, parse_str, Expr, Token};

fn digits(text: &str) -> Vec<Digit> {
    text.chars()
        .map(|c| dz_parse::alphabet::char_to_digit(c).expect("test uses valid digits"))
        .collect()
}

#[test]
fn letters_that_are_digit_aliases_still_lex_as_numbers() {
    // Regression guard for the feature that introduced identifiers: every
    // literal that lexed as a number before must still do so.
    for source in ["E", "X", "XE", "T", "1X", "E;6"] {
        let tokens = lex(source).expect("alias-only text is a number literal");
        assert!(
            matches!(tokens[0], Token::Number { .. }),
            "{} should lex as a number, got {:?}",
            source,
            tokens
        );
    }
}

#[test]
fn a_run_of_letters_with_a_non_alias_is_an_identifier() {
    for name in ["sin", "cos", "tan", "exp", "pi", "sqrt", "euler"] {
        let tokens = lex(name).expect("identifier lexes");
        assert_eq!(tokens, vec![Token::Ident(name.to_string())], "{}", name);
    }
    // Case-insensitive on input, lowercased in the token.
    assert_eq!(lex("SIN").unwrap(), vec![Token::Ident("sin".to_string())]);
}

#[test]
fn constants_parse_with_and_without_parentheses() {
    let bare = parse_str("pi").expect("bare constant parses");
    let called = parse_str("pi()").expect("constant with empty parens parses");
    assert_eq!(bare, called);
    match bare {
        Expr::Call { name, args } => {
            assert_eq!(name, "pi");
            assert!(args.is_empty());
        }
        other => panic!("expected a call, got {:?}", other),
    }
}

#[test]
fn calls_take_one_or_more_arguments() {
    match parse_str("sin(1)").expect("unary call parses") {
        Expr::Call { name, args } => {
            assert_eq!(name, "sin");
            assert_eq!(args.len(), 1);
        }
        other => panic!("expected a call, got {:?}", other),
    }
    match parse_str("log(2, 8)").expect("binary call parses") {
        Expr::Call { name, args } => {
            assert_eq!(name, "log");
            assert_eq!(args.len(), 2);
        }
        other => panic!("expected a call, got {:?}", other),
    }
}

#[test]
fn the_comma_groups_outside_calls_and_separates_inside_them() {
    // Outside a call it is the four-digit grouping separator, so this is
    // one number of five digits, not two numbers.
    let grouped = parse_str("1,0000").expect("grouped literal parses");
    assert_eq!(
        grouped,
        Expr::Number { int_digits: digits("10000"), frac_digits: vec![] }
    );

    // Inside a call's argument list it separates arguments.
    match parse_str("max(1,2)").expect("call with two arguments parses") {
        Expr::Call { name, args } => {
            assert_eq!(name, "max");
            assert_eq!(args.len(), 2);
        }
        other => panic!("expected a call, got {:?}", other),
    }

    // Parentheses that are not a call keep the grouping meaning.
    let inside_plain_parens = parse_str("(1,0000)").expect("grouped literal in parens parses");
    assert_eq!(
        inside_plain_parens,
        Expr::Number { int_digits: digits("10000"), frac_digits: vec![] }
    );
}

#[test]
fn calls_compose_with_the_rest_of_the_grammar() {
    // Precedence is unchanged: a call is a primary expression.
    let expr = parse_str("2 * sin(1) + 3").expect("mixed expression parses");
    match expr {
        Expr::Add(left, right) => {
            assert!(matches!(*left, Expr::Mul(_, _)));
            assert!(matches!(*right, Expr::Number { .. }));
        }
        other => panic!("expected an addition at the top, got {:?}", other),
    }
    parse_str("sqrt(sqrt(16))").expect("nested calls parse");
    parse_str("-sin(1)^2").expect("call under unary minus and power parses");
}

#[test]
fn malformed_calls_are_errors_not_panics() {
    assert!(parse_str("sin(1").is_err(), "unclosed argument list");
    assert!(parse_str("sin(1 2)").is_err(), "missing separator");
    assert!(parse_str("sin(,)").is_err(), "empty argument");
}
