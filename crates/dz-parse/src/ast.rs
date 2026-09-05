use dz_gates::Digit;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Number { int_digits: Vec<Digit>, frac_digits: Vec<Digit> },
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}
