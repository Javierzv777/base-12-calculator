//! Dozenal lexer. Builds number literals by pushing `Digit`s one at a time
//! as characters are read -- **never** `str::parse::<i64>()` or similar
//! (PLAN.md Section 5's central requirement for this layer).
//!
//! Two token kinds beyond the arithmetic operators exist so the expression
//! language can name functions (`sin(1;6)`, `sqrt(4)`, `pi`): `Ident` and
//! `Comma`. Both need a rule to coexist with the dozenal alphabet, because
//! the ASCII digit aliases (`X`/`T` = ten, `E` = eleven) are letters and
//! the four-digit grouping separator is a comma:
//!
//!   - **Identifier vs. number:** a maximal run of ASCII letters is an
//!     identifier if and only if at least one of its letters is *not* a
//!     digit alias. `E` and `XE` stay numbers (eleven, ten-eleven);
//!     `exp`, `tan` and `pi` are identifiers even though `e`, `x` and `t`
//!     are aliases on their own. This keeps every literal that lexed as a
//!     number before this feature existing lexing as one.
//!   - **Comma:** inside a function's argument list it separates
//!     arguments; anywhere else it stays the cosmetic four-digit grouping
//!     separator (`1,0000`). The lexer tracks that with a stack of
//!     parenthesis kinds, so the rule is positional and exact, not a
//!     heuristic on what follows the comma. Consequence, documented for
//!     users: inside a call, write `10000`, not `1,0000`.

use crate::alphabet::{char_to_digit, POINT};
use dz_gates::Digit;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number { int_digits: Vec<Digit>, frac_digits: Vec<Digit> },
    /// A function or constant name, lowercased (`sin`, `pi`, `sqrt`).
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
    /// Argument separator inside a function call's parentheses.
    Comma,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexError {
    pub message: String,
    pub position: usize,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lex error at {}: {}", self.position, self.message)
    }
}
impl std::error::Error for LexError {}

/// Length of the maximal run of ASCII letters starting at `i`.
///
/// `pub(crate)` so `display_alphabet` can walk letter runs the same way
/// this lexer does instead of growing a second copy of the rule.
pub(crate) fn letter_run(chars: &[char], i: usize) -> usize {
    let mut n = 0usize;
    while i + n < chars.len() && chars[i + n].is_ascii_alphabetic() {
        n += 1;
    }
    n
}

/// The identifier-vs-number rule, parameterised by *which* alias set is in
/// force: true as soon as one letter of the run is not a digit alias (see
/// the module docs for why this is the rule).
///
/// The lexer always passes the canonical alias set. `display_alphabet`
/// passes the user's configured pair, so a presentation alphabet decides
/// "name or number" by exactly this function rather than by a second
/// implementation of it in another module -- or, worse, another language.
pub(crate) fn is_identifier_run_with<F>(chars: &[char], i: usize, len: usize, is_alias: F) -> bool
where
    F: Fn(char) -> bool,
{
    let mut k = 0usize;
    while k < len {
        if !is_alias(chars[i + k]) {
            return true;
        }
        k += 1;
    }
    false
}

/// Whether a run of letters should be read as an identifier rather than as
/// a run of dozenal digit aliases, under the canonical alphabet.
pub(crate) fn is_identifier_run(chars: &[char], i: usize, len: usize) -> bool {
    is_identifier_run_with(chars, i, len, |c| char_to_digit(c).is_some())
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    // One entry per open parenthesis: true if it opened a function call's
    // argument list (i.e. the previous token was an identifier).
    let mut paren_kinds: Vec<bool> = Vec::new();
    let mut i: usize = 0;
    while i < chars.len() {
        let c = chars[i];
        let in_call = *paren_kinds.last().unwrap_or(&false);
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c == ',' && !in_call {
            // The four-digit grouping separator (PLAN.md Section 5,
            // "agrupacion por cuatro digitos"); purely cosmetic, skipped.
            i += 1;
            continue;
        }
        if c == ',' {
            tokens.push(Token::Comma);
            i += 1;
            continue;
        }
        if c.is_ascii_alphabetic() {
            let run = letter_run(&chars, i);
            if is_identifier_run(&chars, i, run) {
                let mut name = String::with_capacity(run);
                let mut k = 0usize;
                while k < run {
                    name.push(chars[i + k].to_ascii_lowercase());
                    k += 1;
                }
                tokens.push(Token::Ident(name));
                i += run;
                continue;
            }
        }
        if char_to_digit(c).is_some() || c == POINT {
            // Consume one full number literal: digits, optional point,
            // more digits. Each digit character is pushed as a `Digit`
            // immediately -- no substring is ever handed to a native
            // integer parser.
            let mut int_digits: Vec<Digit> = Vec::new();
            let mut frac_digits: Vec<Digit> = Vec::new();
            let mut seen_point = false;
            while i < chars.len() {
                let cc = chars[i];
                if cc == ',' {
                    if in_call {
                        break; // argument separator, not grouping
                    }
                    i += 1;
                    continue;
                }
                if cc == POINT {
                    if seen_point {
                        return Err(LexError {
                            message: "unexpected second Humphrey point".into(),
                            position: i,
                        });
                    }
                    seen_point = true;
                    i += 1;
                    continue;
                }
                if let Some(d) = char_to_digit(cc) {
                    if seen_point {
                        frac_digits.push(d);
                    } else {
                        int_digits.push(d);
                    }
                    i += 1;
                } else {
                    break;
                }
            }
            if int_digits.is_empty() && frac_digits.is_empty() {
                return Err(LexError {
                    message: "empty number literal".into(),
                    position: i,
                });
            }
            if int_digits.is_empty() {
                int_digits.push(Digit::ZERO);
            }
            tokens.push(Token::Number { int_digits, frac_digits });
            continue;
        }
        let tok = match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' | '\u{00D7}' => Token::Star, // '*' or unicode '×'
            '/' | '\u{00F7}' => Token::Slash, // '/' or unicode '÷'
            '%' => Token::Percent,
            '^' => Token::Caret,
            '(' => {
                let opens_call = matches!(tokens.last(), Some(Token::Ident(_)));
                paren_kinds.push(opens_call);
                Token::LParen
            }
            ')' => {
                paren_kinds.pop();
                Token::RParen
            }
            other => {
                return Err(LexError {
                    message: format!("unexpected character '{}'", other),
                    position: i,
                })
            }
        };
        tokens.push(tok);
        i += 1;
    }
    Ok(tokens)
}
