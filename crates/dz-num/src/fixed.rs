use dz_gates::{Digit, DozGates};
use dz_int::{Dz, DivRemError, Sign};

/// Default number of fractional digits produced by `DzFixed::div` when the
/// division doesn't terminate exactly. A real Phase 5 (exact rationals)
/// would avoid this entirely; this fixed-point approximation is the
/// minimal Layer-2 support needed to make Phase 4's REPL usable for `/`.
pub const DEFAULT_DIV_SCALE: usize = 24;

/// `magnitude / 12^scale`. `scale` is `usize` bookkeeping (how many digits
/// follow the Humphrey point), never a computation on a digit's value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DzFixed {
    pub magnitude: Dz,
    pub scale: usize,
}

/// `12^k` as a `Dz`, built by pushing `k` zero digits then a `ONE` -- pure
/// `Vec` construction (`k` is a loop counter), no arithmetic on any digit's
/// value. Shared with `crate::rational` and `crate::dfp`.
pub(crate) fn pow12(k: usize) -> Dz {
    let mut v = Vec::with_capacity(k + 1);
    let mut i = 0usize;
    while i < k {
        v.push(Digit::ZERO);
        i += 1;
    }
    v.push(Digit::ONE);
    Dz::from_magnitude(false, v)
}

/// Strip trailing (least-significant / fractional) zero digits, reducing
/// `scale` accordingly, so `4;60` displays as `4;6` and exact integers
/// display with `scale == 0`. Vec/index manipulation only.
pub(crate) fn trim(m: Dz, scale: usize) -> (Dz, usize) {
    if m.is_zero() {
        return (Dz::zero(), 0);
    }
    let mut digits = m.magnitude_digits().to_vec();
    let mut s = scale;
    while s > 0 && digits.len() > 1 && digits[0] == Digit::ZERO {
        digits.remove(0);
        s -= 1;
    }
    let negative = m.sign() == Sign::Neg;
    (Dz::from_magnitude(negative, digits), s)
}

impl DzFixed {
    pub fn from_int(magnitude: Dz) -> DzFixed {
        DzFixed { magnitude, scale: 0 }
    }

    /// Build directly from digits as read left-to-right by a lexer:
    /// `int_digits` (most-significant first) and `frac_digits` (the digits
    /// after the Humphrey point, most-significant-fractional first). This
    /// lets `dz-parse` (Layer 3) construct a `DzFixed` (Layer 2) without
    /// depending on `dz-int` (Layer 1) directly, keeping the layer
    /// adjacency rule in PLAN.md Section 1 intact. No native arithmetic is
    /// performed here: concatenation and reversal are `Vec` operations, and
    /// `Dz::from_magnitude` does the little-endian construction.
    pub fn from_digits(negative: bool, int_digits: &[Digit], frac_digits: &[Digit]) -> DzFixed {
        let mut msd_first = Vec::with_capacity(int_digits.len() + frac_digits.len());
        msd_first.extend_from_slice(int_digits);
        msd_first.extend_from_slice(frac_digits);
        msd_first.reverse(); // -> little-endian
        let magnitude = Dz::from_magnitude(negative, msd_first);
        let (m, s) = trim(magnitude, frac_digits.len());
        DzFixed { magnitude: m, scale: s }
    }

    /// Little-endian magnitude digits, exposed for display formatting
    /// (`dz-parse`'s formatter needs to walk them to place the Humphrey
    /// point -- a `Vec`-indexing operation, not digit-value arithmetic).
    pub fn magnitude_digits(&self) -> &[Digit] {
        self.magnitude.magnitude_digits()
    }

    pub fn is_negative(&self) -> bool {
        self.magnitude.sign() == Sign::Neg
    }

    pub fn zero() -> DzFixed {
        DzFixed { magnitude: Dz::zero(), scale: 0 }
    }

    pub fn is_zero(&self) -> bool {
        self.magnitude.is_zero()
    }

    fn align<G: DozGates>(g: &G, a: &DzFixed, b: &DzFixed) -> (Dz, Dz, usize) {
        let common = a.scale.max(b.scale);
        let a_scaled = if a.scale < common {
            a.magnitude.mul(g, &pow12(common - a.scale))
        } else {
            a.magnitude.clone()
        };
        let b_scaled = if b.scale < common {
            b.magnitude.mul(g, &pow12(common - b.scale))
        } else {
            b.magnitude.clone()
        };
        (a_scaled, b_scaled, common)
    }

    pub fn add<G: DozGates>(&self, g: &G, other: &DzFixed) -> DzFixed {
        let (am, bm, scale) = Self::align(g, self, other);
        let (m, s) = trim(am.add(g, &bm), scale);
        DzFixed { magnitude: m, scale: s }
    }

    pub fn sub<G: DozGates>(&self, g: &G, other: &DzFixed) -> DzFixed {
        let (am, bm, scale) = Self::align(g, self, other);
        let (m, s) = trim(am.sub(g, &bm), scale);
        DzFixed { magnitude: m, scale: s }
    }

    pub fn neg(&self) -> DzFixed {
        DzFixed { magnitude: self.magnitude.negate(), scale: self.scale }
    }

    pub fn mul<G: DozGates>(&self, g: &G, other: &DzFixed) -> DzFixed {
        let mag = self.magnitude.mul(g, &other.magnitude);
        let (m, s) = trim(mag, self.scale + other.scale);
        DzFixed { magnitude: m, scale: s }
    }

    /// Division at a fixed result scale (truncating, not rounding). See
    /// `DEFAULT_DIV_SCALE`; a true Phase 5 would replace this with exact
    /// rational arithmetic.
    pub fn div<G: DozGates>(
        &self,
        g: &G,
        other: &DzFixed,
        result_scale: usize,
    ) -> Result<DzFixed, DivRemError> {
        if other.is_zero() {
            return Err(DivRemError::DivisionByZero);
        }
        let numerator = self.magnitude.mul(g, &pow12(other.scale + result_scale));
        let denominator = other.magnitude.mul(g, &pow12(self.scale));
        let (q, _r) = numerator.div_rem(g, &denominator)?;
        let (m, s) = trim(q, result_scale);
        Ok(DzFixed { magnitude: m, scale: s })
    }

    pub fn rem<G: DozGates>(&self, g: &G, other: &DzFixed) -> Result<DzFixed, DivRemError> {
        if other.is_zero() {
            return Err(DivRemError::DivisionByZero);
        }
        let (am, bm, scale) = Self::align(g, self, other);
        let (_q, r) = am.div_rem(g, &bm)?;
        let (m, s) = trim(r, scale);
        Ok(DzFixed { magnitude: m, scale: s })
    }

    /// Non-negative integer exponent power via repeated multiplication.
    pub fn pow<G: DozGates>(&self, g: &G, exponent: u32) -> DzFixed {
        let mut acc = DzFixed::from_int(Dz::from_magnitude(false, vec![Digit::ONE]));
        let mut i: u32 = 0;
        while i < exponent {
            acc = acc.mul(g, self);
            i += 1;
        }
        acc
    }

    pub fn compare<G: DozGates>(&self, g: &G, other: &DzFixed) -> std::cmp::Ordering {
        let (am, bm, _scale) = Self::align(g, self, other);
        am.compare(g, &bm)
    }
}
