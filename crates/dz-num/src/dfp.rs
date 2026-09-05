//! `DzFloat` -- dozenal floating point (DFP), PLAN.md Section 4 / Phase 5.
//!
//! See `docs/03-formato-dfp.md` for the full design writeup and the
//! half-even rounding correction. Summary of the format:
//!
//! - Sign: one bit (native `bool` -- not a dozenal quantity).
//! - Biased exponent: conceptually two dozenal digits (raw range
//!   `0..=143`), bias `71`. Reserved raw values: `0` means the value is
//!   zero (mantissa ignored); `143` (`"↋↋"`, all-elevens -- the dozenal
//!   analogue of IEEE-754's all-ones exponent) means Infinity (mantissa
//!   zero) or NaN (mantissa nonzero).
//! - Mantissa: twelve dozenal digits, stored as a `Dz` integer `M` with
//!   `12^11 <= M < 12^12` when normalized (top digit nonzero).
//! - Value when normal: `(-1)^sign * M * 12^(unbiased_exponent - 11)`.
//!
//! Both the biased exponent and the mantissa are stored and manipulated as
//! `Dz` values, operated on exclusively through `Dz`'s own gate-composed
//! `add`/`sub`/`mul`/`div_rem`/`compare`/`is_even` -- never through native
//! arithmetic on a digit's value. The *positions* things live at (how many
//! digits long a `Vec` is, how many times a loop runs) are native
//! `usize`/`i64` bookkeeping, exactly as PLAN.md Section 0 allows; no digit
//! tag is ever read outside `dz-gates::backends` / `dz-parse::alphabet`
//! (see `tools/lint-purity.ps1`, unchanged by this file).

use crate::fixed::{pow12, trim};
use crate::DzFixed;
use dz_gates::{Digit, DozGates};
use dz_int::{dz_to_small_usize, small_usize_to_dz, Dz, Sign};
use std::cmp::Ordering;

pub const MANTISSA_DIGITS: usize = 12;
pub const BIAS: i64 = 71;
pub const MAX_RAW_EXP: i64 = 143; // 11*12+11 = "\u{218B}\u{218B}", all-elevens

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DfpKind {
    Zero,
    Infinity,
    NaN,
    Normal,
}

#[derive(Clone, Debug)]
pub struct DzFloat {
    pub sign: bool,
    pub kind: DfpKind,
    /// Meaningful only when `kind == Normal`: raw biased exponent, `1..=142`.
    pub biased_exponent: Dz,
    /// Meaningful only when `kind == Normal`: `12^11 <= mantissa < 12^12`.
    pub mantissa: Dz,
}

fn one() -> Dz {
    Dz::from_magnitude(false, vec![Digit::ONE])
}

fn twelve() -> Dz {
    pow12(1)
}

impl DzFloat {
    pub fn zero(sign: bool) -> DzFloat {
        DzFloat { sign, kind: DfpKind::Zero, biased_exponent: Dz::zero(), mantissa: Dz::zero() }
    }

    pub fn infinity(sign: bool) -> DzFloat {
        DzFloat { sign, kind: DfpKind::Infinity, biased_exponent: Dz::zero(), mantissa: Dz::zero() }
    }

    pub fn nan() -> DzFloat {
        DzFloat { sign: false, kind: DfpKind::NaN, biased_exponent: Dz::zero(), mantissa: Dz::zero() }
    }

    pub fn is_nan(&self) -> bool {
        self.kind == DfpKind::NaN
    }
    pub fn is_infinite(&self) -> bool {
        self.kind == DfpKind::Infinity
    }
    pub fn is_zero(&self) -> bool {
        self.kind == DfpKind::Zero
    }

    /// Encode an exact `DzFixed` value into DFP, rounding the mantissa to
    /// `MANTISSA_DIGITS` dozenal digits with round-half-to-even if it has
    /// more significant digits than that. See `docs/03-formato-dfp.md` for
    /// the derivation of the exponent/rounding arithmetic below.
    pub fn from_dzfixed<G: DozGates>(g: &G, value: &DzFixed) -> DzFloat {
        if value.is_zero() {
            return DzFloat::zero(false);
        }
        let sign = value.magnitude.sign() == Sign::Neg;
        let digits = value.magnitude_digits(); // little-endian, non-negative
        let l = digits.len() as i64; // length bookkeeping: sanctioned native arithmetic
        let scale = value.scale as i64;
        let e = l - 1 - scale; // position (weight 12^e) of the most-significant digit

        let abs_val = Dz::from_magnitude(false, digits.to_vec());

        let (mantissa, exponent_adjust) = if (l as usize) <= MANTISSA_DIGITS {
            // Fewer than 12 significant digits: exact, just left-pad with
            // zero digits (multiply by a power of 12 -- Vec construction,
            // no rounding needed).
            let shift = MANTISSA_DIGITS - l as usize;
            (abs_val.mul(g, &pow12(shift)), 0i64)
        } else {
            // More than 12 significant digits: divide out the excess and
            // round half-to-even.
            let shift = l as usize - MANTISSA_DIGITS;
            let divisor = pow12(shift);
            let (q, r) = abs_val.div_rem(g, &divisor).expect("divisor is never zero");
            let two_r = r.add(g, &r);
            let mut q = q;
            match two_r.compare(g, &divisor) {
                Ordering::Less => {}
                Ordering::Greater => q = q.add(g, &one()),
                Ordering::Equal => {
                    if !q.is_even(g) {
                        q = q.add(g, &one());
                    }
                }
            }
            // Rounding up an all-elevens mantissa overflows to 13 digits
            // (exactly 12^12); renormalize by dropping one digit and
            // bumping the exponent, matching standard FP rounding overflow.
            let mut adjust = 0i64;
            if q.magnitude_digits().len() > MANTISSA_DIGITS {
                q = q.div_rem(g, &twelve()).expect("12 is never zero").0;
                adjust = 1;
            }
            (q, adjust)
        };

        let unbiased = e + exponent_adjust;
        let biased = unbiased + BIAS;
        if biased <= 0 {
            // Underflow: flush to zero. No subnormals in this format (a
            // documented simplification, see docs/03-formato-dfp.md).
            return DzFloat::zero(sign);
        }
        if biased >= MAX_RAW_EXP {
            return DzFloat::infinity(sign);
        }
        let biased_exponent = small_usize_to_dz(g, biased as usize);
        DzFloat { sign, kind: DfpKind::Normal, biased_exponent, mantissa }
    }

    /// Decode back to an exact `DzFixed`. `None` for Infinity/NaN (no exact
    /// finite value).
    pub fn to_dzfixed<G: DozGates>(&self, g: &G) -> Option<DzFixed> {
        match self.kind {
            DfpKind::Zero => Some(DzFixed::zero()),
            DfpKind::Infinity | DfpKind::NaN => None,
            DfpKind::Normal => {
                let biased = dz_to_small_usize(g, &self.biased_exponent, MAX_RAW_EXP as usize)
                    .expect("biased_exponent is always in range for Normal values");
                let unbiased = biased as i64 - BIAS;
                let shift = unbiased - (MANTISSA_DIGITS as i64 - 1);
                let (mag, scale) = if shift >= 0 {
                    (self.mantissa.mul(g, &pow12(shift as usize)), 0usize)
                } else {
                    (self.mantissa.clone(), (-shift) as usize)
                };
                let signed = if self.sign { mag.negate() } else { mag };
                let (m, s) = trim(signed, scale);
                Some(DzFixed { magnitude: m, scale: s })
            }
        }
    }

    pub fn neg(&self) -> DzFloat {
        let mut out = self.clone();
        out.sign = !out.sign;
        out
    }

    pub fn add<G: DozGates>(&self, g: &G, other: &DzFloat) -> DzFloat {
        match (self.kind, other.kind) {
            (DfpKind::NaN, _) | (_, DfpKind::NaN) => DzFloat::nan(),
            (DfpKind::Infinity, DfpKind::Infinity) => {
                if self.sign == other.sign {
                    self.clone()
                } else {
                    DzFloat::nan() // inf + (-inf)
                }
            }
            (DfpKind::Infinity, _) => self.clone(),
            (_, DfpKind::Infinity) => other.clone(),
            _ => {
                let a = self.to_dzfixed(g).expect("finite");
                let b = other.to_dzfixed(g).expect("finite");
                DzFloat::from_dzfixed(g, &a.add(g, &b))
            }
        }
    }

    pub fn sub<G: DozGates>(&self, g: &G, other: &DzFloat) -> DzFloat {
        self.add(g, &other.neg())
    }

    pub fn mul<G: DozGates>(&self, g: &G, other: &DzFloat) -> DzFloat {
        if self.is_nan() || other.is_nan() {
            return DzFloat::nan();
        }
        let result_sign = self.sign != other.sign;
        if self.is_infinite() || other.is_infinite() {
            if self.is_zero() || other.is_zero() {
                return DzFloat::nan(); // 0 * inf
            }
            return DzFloat::infinity(result_sign);
        }
        let a = self.to_dzfixed(g).expect("finite");
        let b = other.to_dzfixed(g).expect("finite");
        DzFloat::from_dzfixed(g, &a.mul(g, &b))
    }

    pub fn div<G: DozGates>(&self, g: &G, other: &DzFloat) -> DzFloat {
        if self.is_nan() || other.is_nan() {
            return DzFloat::nan();
        }
        let result_sign = self.sign != other.sign;
        if other.is_zero() {
            return if self.is_zero() { DzFloat::nan() } else { DzFloat::infinity(result_sign) };
        }
        if other.is_infinite() {
            return if self.is_infinite() { DzFloat::nan() } else { DzFloat::zero(result_sign) };
        }
        if self.is_infinite() {
            return DzFloat::infinity(result_sign);
        }
        let a = self.to_dzfixed(g).expect("finite");
        let b = other.to_dzfixed(g).expect("finite");
        // Extra guard digits before rounding down to the DFP mantissa width.
        let extra_scale = MANTISSA_DIGITS + 4;
        match a.div(g, &b, a.scale.max(b.scale) + extra_scale) {
            Ok(q) => DzFloat::from_dzfixed(g, &q),
            Err(_) => DzFloat::infinity(result_sign),
        }
    }

    /// `None` when either operand is NaN (unordered), matching IEEE-style
    /// float comparison semantics.
    pub fn compare<G: DozGates>(&self, g: &G, other: &DzFloat) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            return None;
        }
        match (self.kind, other.kind) {
            (DfpKind::Zero, DfpKind::Zero) => Some(Ordering::Equal),
            (DfpKind::Infinity, DfpKind::Infinity) => {
                Some(self.sign.cmp(&other.sign).reverse()) // +inf > -inf, equal signs => Equal
            }
            (DfpKind::Infinity, _) => Some(if self.sign { Ordering::Less } else { Ordering::Greater }),
            (_, DfpKind::Infinity) => Some(if other.sign { Ordering::Greater } else { Ordering::Less }),
            _ => {
                let a = self.to_dzfixed(g).expect("finite");
                let b = other.to_dzfixed(g).expect("finite");
                Some(a.compare(g, &b))
            }
        }
    }
}
