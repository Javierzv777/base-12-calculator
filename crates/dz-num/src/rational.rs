//! `DzRational` -- exact dozenal rationals, `num/den` as two `Dz` values,
//! `den` always positive, sign carried in `num`. Reduced to canonical form
//! (`gcd(|num|, den) == 1`, `den > 0`, and `0/1` for zero) after every
//! operation, using only `Dz`'s own gate-composed division (Euclid's
//! algorithm needs nothing but repeated `div_rem`). PLAN.md Section 4: "el
//! MCD por Euclides usando solo la division de Capa 1."

use crate::fixed::{pow12, trim};
use crate::DzFixed;
use dz_gates::{Digit, DozGates};
use dz_int::{DivRemError, Dz, Sign};
use std::cmp::Ordering;

fn one() -> Dz {
    Dz::from_magnitude(false, vec![Digit::ONE])
}

fn two() -> Dz {
    Dz::from_magnitude(false, vec![Digit::new(2).unwrap()])
}

fn three() -> Dz {
    Dz::from_magnitude(false, vec![Digit::new(3).unwrap()])
}

fn abs(n: &Dz) -> Dz {
    if n.sign() == Sign::Neg {
        n.negate()
    } else {
        n.clone()
    }
}

/// Euclid's algorithm using only `Dz::div_rem`. `a`, `b` must be
/// non-negative (callers pass absolute values).
fn gcd<G: DozGates>(g: &G, a: &Dz, b: &Dz) -> Dz {
    let mut a = a.clone();
    let mut b = b.clone();
    while !b.is_zero() {
        let (_q, r) = a.div_rem(g, &b).expect("b is non-zero by the loop condition");
        a = b;
        b = r;
    }
    a
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RationalError {
    DivisionByZero,
}

impl std::fmt::Display for RationalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RationalError::DivisionByZero => write!(f, "division by zero"),
        }
    }
}
impl std::error::Error for RationalError {}

impl From<DivRemError> for RationalError {
    fn from(_: DivRemError) -> Self {
        RationalError::DivisionByZero
    }
}

/// `num / den`, always kept in canonical form: `den` positive and nonzero,
/// `gcd(|num|, den) == 1`, and zero is always `0/1`.
#[derive(Clone, Debug)]
pub struct DzRational {
    pub num: Dz,
    pub den: Dz,
}

impl DzRational {
    /// Build and reduce to canonical form. `den == 0` is an error.
    pub fn new<G: DozGates>(g: &G, num: Dz, den: Dz) -> Result<DzRational, RationalError> {
        if den.is_zero() {
            return Err(RationalError::DivisionByZero);
        }
        let (mut num, mut den) = if den.sign() == Sign::Neg {
            (num.negate(), den.negate())
        } else {
            (num, den)
        };
        if num.is_zero() {
            return Ok(DzRational { num: Dz::zero(), den: one() });
        }
        let d = gcd(g, &abs(&num), &den);
        if d.compare(g, &one()) != Ordering::Equal {
            num = num.div_rem(g, &d)?.0;
            den = den.div_rem(g, &d)?.0;
        }
        Ok(DzRational { num, den })
    }

    pub fn from_int(n: Dz) -> DzRational {
        DzRational { num: n, den: one() }
    }

    pub fn zero() -> DzRational {
        DzRational { num: Dz::zero(), den: one() }
    }

    pub fn is_zero(&self) -> bool {
        self.num.is_zero()
    }

    pub fn is_negative(&self) -> bool {
        self.num.sign() == Sign::Neg
    }

    /// Little-endian numerator/denominator digits, exposed so `dz-parse`'s
    /// formatter can render `num/den` without needing to name `dz_int::Dz`
    /// directly (keeping the layer-adjacency rule in PLAN.md Section 1:
    /// `dz-parse` depends on `dz-num`, not on `dz-int` two layers down).
    pub fn num_digits(&self) -> &[Digit] {
        self.num.magnitude_digits()
    }
    pub fn den_digits(&self) -> &[Digit] {
        self.den.magnitude_digits()
    }

    /// Build directly from digits as read by the lexer (integer part +
    /// fractional part after the Humphrey point), mirroring
    /// `DzFixed::from_digits`: `int;frac` = `(int*12^len(frac) + frac) /
    /// 12^len(frac)`, reduced.
    pub fn from_digits<G: DozGates>(
        g: &G,
        negative: bool,
        int_digits: &[Digit],
        frac_digits: &[Digit],
    ) -> DzRational {
        let mut msd_first = Vec::with_capacity(int_digits.len() + frac_digits.len());
        msd_first.extend_from_slice(int_digits);
        msd_first.extend_from_slice(frac_digits);
        msd_first.reverse();
        let numerator = Dz::from_magnitude(negative, msd_first);
        let denominator = pow12(frac_digits.len());
        DzRational::new(g, numerator, denominator).expect("12^k is never zero")
    }

    pub fn neg(&self) -> DzRational {
        DzRational { num: self.num.negate(), den: self.den.clone() }
    }

    pub fn add<G: DozGates>(&self, g: &G, other: &DzRational) -> Result<DzRational, RationalError> {
        let num = self
            .num
            .mul(g, &other.den)
            .add(g, &other.num.mul(g, &self.den));
        let den = self.den.mul(g, &other.den);
        DzRational::new(g, num, den)
    }

    pub fn sub<G: DozGates>(&self, g: &G, other: &DzRational) -> Result<DzRational, RationalError> {
        self.add(g, &other.neg())
    }

    pub fn mul<G: DozGates>(&self, g: &G, other: &DzRational) -> Result<DzRational, RationalError> {
        let num = self.num.mul(g, &other.num);
        let den = self.den.mul(g, &other.den);
        DzRational::new(g, num, den)
    }

    pub fn div<G: DozGates>(&self, g: &G, other: &DzRational) -> Result<DzRational, RationalError> {
        if other.is_zero() {
            return Err(RationalError::DivisionByZero);
        }
        let num = self.num.mul(g, &other.den);
        let den = self.den.mul(g, &other.num);
        // `den` may come out negative if `other.num` is negative;
        // `DzRational::new` re-normalizes the sign into the numerator.
        DzRational::new(g, num, den)
    }

    pub fn rem<G: DozGates>(&self, g: &G, other: &DzRational) -> Result<DzRational, RationalError> {
        if other.is_zero() {
            return Err(RationalError::DivisionByZero);
        }
        // a mod b = a - trunc(a/b) * b, trunc toward zero (matches Rust's
        // native `%`), computed with exact rational arithmetic throughout.
        let q = self.div(g, other)?;
        let trunc_num = q.num.div_rem(g, &q.den)?.0; // integer part, truncated
        let trunc = DzRational::from_int(trunc_num);
        let prod = trunc.mul(g, other)?;
        self.sub(g, &prod)
    }

    pub fn pow<G: DozGates>(&self, g: &G, exponent: u32) -> Result<DzRational, RationalError> {
        let mut acc = DzRational::from_int(one());
        let mut i: u32 = 0;
        while i < exponent {
            acc = acc.mul(g, self)?;
            i += 1;
        }
        Ok(acc)
    }

    /// Cross-multiplication comparison (`den`s are always positive, so this
    /// is sign-safe): `a/b cmp c/d <=> a*d cmp c*b`.
    pub fn compare<G: DozGates>(&self, g: &G, other: &DzRational) -> Ordering {
        let left = self.num.mul(g, &other.den);
        let right = other.num.mul(g, &self.den);
        left.compare(g, &right)
    }

    /// Whether the reduced denominator's only prime factors are 2 and 3,
    /// i.e. whether this fraction terminates in base 12 (PLAN.md Section 4:
    /// "tabla comparativa de periodicidad"). Uses only `Dz::div_rem`.
    pub fn terminates_in_base12<G: DozGates>(&self, g: &G) -> bool {
        let mut d = self.den.clone();
        loop {
            let (q, r) = d.div_rem(g, &two()).expect("2 is never zero");
            if r.is_zero() {
                d = q;
            } else {
                break;
            }
        }
        loop {
            let (q, r) = d.div_rem(g, &three()).expect("3 is never zero");
            if r.is_zero() {
                d = q;
            } else {
                break;
            }
        }
        d.compare(g, &one()) == Ordering::Equal
    }

    /// Convert to a `DzFixed` with exactly `scale` fractional digits,
    /// truncating (not rounding) if the fraction doesn't terminate within
    /// that many digits.
    pub fn to_fixed<G: DozGates>(&self, g: &G, scale: usize) -> DzFixed {
        let numerator = self.num.mul(g, &pow12(scale));
        let (q, _r) = numerator
            .div_rem(g, &self.den)
            .expect("den is never zero in canonical form");
        let (m, s) = trim(q, scale);
        DzFixed { magnitude: m, scale: s }
    }
}

impl PartialEq for DzRational {
    fn eq(&self, other: &Self) -> bool {
        // Canonical form is unique, so structural equality of the reduced
        // (num, den) pair is exactly value equality.
        self.num == other.num && self.den == other.den
    }
}
impl Eq for DzRational {}
