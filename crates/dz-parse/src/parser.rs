//! Recursive-descent / precedence-climbing parser over `Token`s produced by
//! `crate::lexer`. Precedence, low to high: `+ -`, `* / %`, unary `-`, `^`
//! (right-associative), parens/number/call.
//!
//! Function calls (`sin(x)`, `log(2, 8)`) and constants (`pi`) both parse
//! into `Expr::Call`; the evaluator resolves the name against its function
//! registry, so adding a function never touches this file.

use crate::ast::Expr;
use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error: {}", self.message)
    }
}
impl std::error::Error for ParseError {}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let t = self.tokens.get(self.pos);
        self.pos += 1;
        t
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expr::Add(Box::new(left), Box::new(right));
                }
                Some(Token::Minus) => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expr::Sub(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Mul(Box::new(left), Box::new(right));
                }
                Some(Token::Slash) => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Div(Box::new(left), Box::new(right));
                }
                Some(Token::Percent) => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Mod(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(Token::Minus) = self.peek() {
            self.advance();
            let inner = self.parse_unary()?;
            return Ok(Expr::Neg(Box::new(inner)));
        }
        self.parse_power()
    }

    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let base = self.parse_primary()?;
        if let Some(Token::Caret) = self.peek() {
            self.advance();
            let exponent = self.parse_unary()?; // right-associative
            return Ok(Expr::Pow(Box::new(base), Box::new(exponent)));
        }
        Ok(base)
    }

    /// `name` or `name(arg, arg, ...)`. A bare identifier is a
    /// zero-argument call, which is how constants (`pi`) reach the
    /// evaluator's function registry without needing a second syntax.
    fn parse_call(&mut self, name: String) -> Result<Expr, ParseError> {
        if !matches!(self.peek(), Some(Token::LParen)) {
            return Ok(Expr::Call { name, args: Vec::new() });
        }
        self.advance(); // '('
        let mut args = Vec::new();
        if matches!(self.peek(), Some(Token::RParen)) {
            self.advance();
            return Ok(Expr::Call { name, args });
        }
        loop {
            args.push(self.parse_expr()?);
            match self.advance().cloned() {
                Some(Token::Comma) => continue,
                Some(Token::RParen) => break,
                Some(other) => {
                    return Err(ParseError {
                        message: format!(
                            "expected ',' or ')' in call to '{}', found {:?}",
                            name, other
                        ),
                    })
                }
                None => {
                    return Err(ParseError {
                        message: format!("unclosed argument list in call to '{}'", name),
                    })
                }
            }
        }
        Ok(Expr::Call { name, args })
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.advance().cloned() {
            Some(Token::Number { int_digits, frac_digits }) => {
                Ok(Expr::Number { int_digits, frac_digits })
            }
            Some(Token::Ident(name)) => self.parse_call(name),
            Some(Token::LParen) => {
                let inner = self.parse_expr()?;
                match self.advance() {
                    Some(Token::RParen) => Ok(inner),
                    _ => Err(ParseError { message: "expected ')'".into() }),
                }
            }
            Some(other) => Err(ParseError { message: format!("unexpected token {:?}", other) }),
            None => Err(ParseError { message: "unexpected end of input".into() }),
        }
    }
}

pub fn parse(tokens: &[Token]) -> Result<Expr, ParseError> {
    let mut p = Parser { tokens, pos: 0 };
    let expr = p.parse_expr()?;
    if p.pos != tokens.len() {
        return Err(ParseError {
            message: format!("unexpected trailing token {:?}", tokens[p.pos]),
        });
    }
    Ok(expr)
}
