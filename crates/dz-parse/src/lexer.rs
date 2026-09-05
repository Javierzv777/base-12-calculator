//! Dozenal lexer. Builds number literals by pushing `Digit`s one at a time
//! as characters are read -- **never** `str::parse::<i64>()` or similar
//! (PLAN.md Section 5's central requirement for this layer).

use crate::alphabet::{char_to_digit, POINT};
use dz_gates::Digit;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number { int_digits: Vec<Digit>, frac_digits: Vec<Digit> },
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
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

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i: usize = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() || c == ',' {
            // ',' is the four-digit grouping separator (PLAN.md Section 5,
            // "agrupacion por cuatro digitos"); purely cosmetic, skipped.
            i += 1;
            continue;
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
            '(' => Token::LParen,
            ')' => Token::RParen,
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
