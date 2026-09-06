//! `dz-parse` -- Layer 3: dozenal lexer/parser. Never converts input text
//! through a native integer/float parser; digits are pushed one at a time
//! into `Digit` values (PLAN.md Section 5).

pub mod alphabet;
pub mod ast;
pub mod display_alphabet;
pub mod format;
pub mod input_base;
pub mod lexer;
pub mod parser;

pub use ast::Expr;
pub use display_alphabet::{AlphabetError, DisplayAlphabet, CANONICAL_ELEVEN, CANONICAL_TEN};
pub use format::{format_dfp, format_fixed, format_rational};
pub use input_base::{from_dozenal_text, to_dozenal_text, InputBase, InputBaseError};
pub use lexer::{lex, LexError, Token};
pub use parser::{parse, ParseError};

#[derive(Debug)]
pub enum ExprParseError {
    Lex(LexError),
    Parse(ParseError),
}

impl std::fmt::Display for ExprParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprParseError::Lex(e) => write!(f, "{}", e),
            ExprParseError::Parse(e) => write!(f, "{}", e),
        }
    }
}
impl std::error::Error for ExprParseError {}

/// Convenience: lex + parse a full expression string.
pub fn parse_str(input: &str) -> Result<Expr, ExprParseError> {
    let tokens = lex(input).map_err(ExprParseError::Lex)?;
    parse(&tokens).map_err(ExprParseError::Parse)
}
