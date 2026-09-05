use dz_gates::circuits::{
    flag_to_bool, ge, is_even_digit, multiply as gate_multiply, ones_complement, ripple_add,
};
use dz_gates::{Digit, DozGates};
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sign {
    Zero,
    Pos,
    Neg,
}

/// Arbitrary-precision dozenal integer: sign + little-endian magnitude
/// digits, normalized (no leading `Digit::ZERO` beyond a single digit for
/// the value zero). Internally, subtraction is done via 12's-complement
/// addition (`sub_magnitude`, built from `dz_gates::circuits::ones_complement`
/// and `ripple_add`), per PLAN.md Section 3.
#[derive(Clone, Debug)]
pub struct Dz {
    sign: Sign,
    magnitude: Vec<Digit>, // little-endian, normalized
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DivRemError {
    DivisionByZero,
}

impl fmt::Display for DivRemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DivRemError::DivisionByZero => write!(f, "division by zero"),
        }
    }
}
impl std::error::Error for DivRemError {}

fn normalize(mut digits: Vec<Digit>) -> Vec<Digit> {
    // Trimming trailing (most-significant) zero digits is `Vec` length
    // bookkeeping driven by equality against the ZERO symbol, not
    // arithmetic on a digit's value.
    while digits.len() > 1 && *digits.last().unwrap() == Digit::ZERO {
        digits.pop();
    }
    if digits.is_empty() {
        digits.push(Digit::ZERO);
    }
    digits
}

fn is_zero_mag(digits: &[Digit]) -> bool {
    digits.iter().all(|&d| d == Digit::ZERO)
}

fn pad(digits: &[Digit], width: usize) -> Vec<Digit> {
    let mut out = digits.to_vec();
    while out.len() < width {
        out.push(Digit::ZERO);
    }
    out
}

/// Compare two normalized-or-padded little-endian magnitudes, most
/// significant digit first, using the gate-composed `ge` comparator plus
/// equality (never native `<`/`>` on a digit tag).
fn compare_magnitude<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> Ordering {
    let n = a.len().max(b.len());
    let ap = pad(a, n);
    let bp = pad(b, n);
    let mut i = n;
    while i > 0 {
        i -= 1;
        if ap[i] != bp[i] {
            return if flag_to_bool(ge(g, ap[i], bp[i])) {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        }
    }
    Ordering::Equal
}

/// `a - b` for two non-negative magnitudes, via 12's-complement addition.
/// Returns `(|a-b|, a < b)`.
///
/// Let `n = max(len(a), len(b))`, so both `a` and `b` fit exactly in `n`
/// digits. Compute the *exact* (not truncated) radix complement
/// `rc_b = 12^n - b` -- this is `n` digits when `b > 0`, or `n+1` digits
/// in the edge case `b == 0` (where `12^n - 0 == 12^n` genuinely needs one
/// more digit). Then `sum = a + rc_b = 12^n + (a - b)` exactly, with no
/// wraparound at all (unlike a fixed-width register). Since `0 <= a,b <
/// 12^n`, that sum needs `n+1` digits (top digit exactly `1`) iff `a >= b`,
/// and at most `n` digits iff `a < b` (in which case it's already the
/// complement representation of `b - a`, decoded by complementing once
/// more). Crucially, `rc_b` must *not* be truncated to `n` digits before
/// this addition -- doing so (an earlier bug) discards exactly the
/// information this test depends on.
fn sub_magnitude<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> (Vec<Digit>, bool) {
    let n = a.len().max(b.len());
    let a_pad = pad(a, n);
    let b_pad = pad(b, n);
    let comp1_b = ones_complement(g, &b_pad);
    let rc_b = ripple_add(g, &comp1_b, &[Digit::ONE]); // exact 12^n - b, n or n+1 digits
    let sum = normalize(ripple_add(g, &a_pad, &rc_b));
    if sum.len() > n {
        // a >= b: drop the top "1" digit, the remaining n digits are a - b.
        let mut mag = sum;
        mag.truncate(n);
        (normalize(mag), false)
    } else {
        // a < b: `sum` (padded to n digits) is the 12's complement of
        // (b - a). Complement it once more (exact, same reasoning) to
        // recover the magnitude b - a.
        let sum_p = pad(&sum, n);
        let comp1_sum = ones_complement(g, &sum_p);
        let mag = normalize(ripple_add(g, &comp1_sum, &[Digit::ONE]));
        (mag, true)
    }
}

impl Dz {
    pub fn zero() -> Dz {
        Dz {
            sign: Sign::Zero,
            magnitude: vec![Digit::ZERO],
        }
    }

    /// Build from an explicit sign and little-endian magnitude digits
    /// (e.g. produced by the lexer, pushing one `Digit` at a time -- see
    /// `dz-parse`). Normalizes leading zeros and canonicalizes the sign.
    pub fn from_magnitude(sign_negative: bool, digits: Vec<Digit>) -> Dz {
        let mag = normalize(digits);
        let sign = if is_zero_mag(&mag) {
            Sign::Zero
        } else if sign_negative {
            Sign::Neg
        } else {
            Sign::Pos
        };
        Dz { sign, magnitude: mag }
    }

    pub fn sign(&self) -> Sign {
        self.sign
    }

    /// Little-endian magnitude digits (normalized).
    pub fn magnitude_digits(&self) -> &[Digit] {
        &self.magnitude
    }

    pub fn is_zero(&self) -> bool {
        matches!(self.sign, Sign::Zero)
    }

    pub fn negate(&self) -> Dz {
        let sign = match self.sign {
            Sign::Zero => Sign::Zero,
            Sign::Pos => Sign::Neg,
            Sign::Neg => Sign::Pos,
        };
        Dz {
            sign,
            magnitude: self.magnitude.clone(),
        }
    }

    pub fn add<G: DozGates>(&self, g: &G, other: &Dz) -> Dz {
        match (self.sign, other.sign) {
            (Sign::Zero, _) => other.clone(),
            (_, Sign::Zero) => self.clone(),
            (Sign::Pos, Sign::Pos) | (Sign::Neg, Sign::Neg) => {
                let mag = normalize(ripple_add(g, &self.magnitude, &other.magnitude));
                Dz { sign: self.sign, magnitude: mag }
            }
            (Sign::Pos, Sign::Neg) => {
                // self = +A, other = -B; sum = A - B, negative iff A < B.
                let (mag, is_negative) = sub_magnitude(g, &self.magnitude, &other.magnitude);
                Dz::from_signed_magnitude(mag, is_negative)
            }
            (Sign::Neg, Sign::Pos) => {
                // self = -A, other = +B; sum = B - A. sub_magnitude(B, A)
                // reports is_negative = (B < A), which is exactly when the
                // result B - A is negative -- no extra negation needed.
                let (mag, is_negative) = sub_magnitude(g, &other.magnitude, &self.magnitude);
                Dz::from_signed_magnitude(mag, is_negative)
            }
        }
    }

    fn from_signed_magnitude(mag: Vec<Digit>, negative: bool) -> Dz {
        let mag = normalize(mag);
        if is_zero_mag(&mag) {
            Dz::zero()
        } else {
            Dz {
                sign: if negative { Sign::Neg } else { Sign::Pos },
                magnitude: mag,
            }
        }
    }

    pub fn sub<G: DozGates>(&self, g: &G, other: &Dz) -> Dz {
        self.add(g, &other.negate())
    }

    pub fn mul<G: DozGates>(&self, g: &G, other: &Dz) -> Dz {
        if self.is_zero() || other.is_zero() {
            return Dz::zero();
        }
        let mag = normalize(gate_multiply(g, &self.magnitude, &other.magnitude));
        let negative = self.sign != other.sign;
        Dz {
            sign: if negative { Sign::Neg } else { Sign::Pos },
            magnitude: mag,
        }
    }

    /// Compare (ordering), never via native `Ord` on digit tags: sign first,
    /// then gate-composed magnitude comparison.
    pub fn compare<G: DozGates>(&self, g: &G, other: &Dz) -> Ordering {
        use Sign::*;
        match (self.sign, other.sign) {
            (Zero, Zero) => Ordering::Equal,
            (Zero, Pos) | (Neg, Pos) | (Neg, Zero) => Ordering::Less,
            (Zero, Neg) | (Pos, Neg) | (Pos, Zero) => Ordering::Greater,
            (Pos, Pos) => compare_magnitude(g, &self.magnitude, &other.magnitude),
            (Neg, Neg) => compare_magnitude(g, &other.magnitude, &self.magnitude),
        }
    }

    pub fn eq_value<G: DozGates>(&self, g: &G, other: &Dz) -> bool {
        self.compare(g, other) == Ordering::Equal
    }

    /// Restoring long division (schoolbook, base 12): returns `(quotient, remainder)`.
    pub fn div_rem<G: DozGates>(&self, g: &G, other: &Dz) -> Result<(Dz, Dz), DivRemError> {
        if other.is_zero() {
            return Err(DivRemError::DivisionByZero);
        }
        if self.is_zero() {
            return Ok((Dz::zero(), Dz::zero()));
        }

        let a = &self.magnitude; // MSD-first iteration built below
        let b = &other.magnitude;

        if compare_magnitude(g, a, b) == Ordering::Less {
            // |a| < |b|: quotient 0, remainder is a itself (sign of a).
            return Ok((Dz::zero(), self.clone()));
        }

        let mut quotient_msd_first: Vec<Digit> = Vec::with_capacity(a.len());
        let mut remainder: Vec<Digit> = vec![Digit::ZERO];

        let mut i = a.len();
        while i > 0 {
            i -= 1;
            let a_i = a[i];
            // remainder = remainder * 12 + a_i
            let mut shifted = vec![Digit::ZERO];
            shifted.extend_from_slice(&remainder);
            remainder = normalize(ripple_add(g, &shifted, &[a_i]));

            // Trial quotient digit: largest q in 0..12 with b*q <= remainder.
            let mut q: u8 = 0;
            let mut best_product: Vec<Digit> = vec![Digit::ZERO];
            let mut j: u8 = 0;
            while j < dz_gates::RADIX {
                let jd = Digit::new(j).expect("j in 0..RADIX");
                let candidate = normalize(gate_multiply(g, b, &[jd]));
                if compare_magnitude(g, &candidate, &remainder) != Ordering::Greater {
                    q = j;
                    best_product = candidate;
                } else {
                    break;
                }
                j += 1;
            }
            let qd = Digit::new(q).expect("q in 0..RADIX");
            let (new_rem, _neg) = sub_magnitude(g, &remainder, &best_product);
            remainder = new_rem;
            quotient_msd_first.push(qd);
        }

        quotient_msd_first.reverse();
        let quotient_mag = normalize(quotient_msd_first);
        let remainder_mag = normalize(remainder);

        let quotient_negative = self.sign != other.sign;
        let quotient = if is_zero_mag(&quotient_mag) {
            Dz::zero()
        } else {
            Dz {
                sign: if quotient_negative { Sign::Neg } else { Sign::Pos },
                magnitude: quotient_mag,
            }
        };
        // Remainder takes the sign of the dividend (truncating division),
        // the conventional choice matching Rust's native integer `%`.
        let remainder = if is_zero_mag(&remainder_mag) {
            Dz::zero()
        } else {
            Dz {
                sign: self.sign,
                magnitude: remainder_mag,
            }
        };
        Ok((quotient, remainder))
    }

    /// Whether this integer is even. Since 12 is even, parity is read off
    /// the least-significant digit alone (same as base 10) -- via the
    /// gate-composed `is_even_digit`, never by inspecting the digit's tag.
    pub fn is_even<G: DozGates>(&self, g: &G) -> bool {
        let lsd = self.magnitude[0]; // index 0 = least significant digit
        flag_to_bool(is_even_digit(g, lsd))
    }
}

/// Convert a small non-negative native `usize` into the `Dz` it denotes, by
/// counting up from zero with `Dz::add` -- i.e. purely through gate-composed
/// addition, never by computing a digit's tag from `k` via `%`/`/`. `k`
/// itself is ordinary loop-counter/length bookkeeping (PLAN.md Section 0),
/// so driving the loop this many times is allowed; what's *not* allowed
/// (and not done here) is deriving a digit's value directly from `k` with
/// native arithmetic.
pub fn small_usize_to_dz<G: DozGates>(g: &G, k: usize) -> Dz {
    let mut acc = Dz::zero();
    let one = Dz::from_magnitude(false, vec![Digit::ONE]);
    let mut i: usize = 0;
    while i < k {
        acc = acc.add(g, &one);
        i += 1;
    }
    acc
}

/// The inverse: recover a native `usize` from a small non-negative `Dz`, by
/// counting up and gate-comparing (`eq_value`) at each step, bounded by
/// `max`. Returns `None` if `n` is negative or exceeds `max`. This is the
/// same "extract a native loop bound from a dozenal value without ever
/// reading a digit's tag" technique used in `dz-eval`'s exponent handling.
pub fn dz_to_small_usize<G: DozGates>(g: &G, n: &Dz, max: usize) -> Option<usize> {
    if n.sign() == Sign::Neg {
        return None;
    }
    let mut probe = Dz::zero();
    let one = Dz::from_magnitude(false, vec![Digit::ONE]);
    let mut i: usize = 0;
    while i <= max {
        if probe.eq_value(g, n) {
            return Some(i);
        }
        probe = probe.add(g, &one);
        i += 1;
    }
    None
}

impl PartialEq for Dz {
    fn eq(&self, other: &Self) -> bool {
        self.sign == other.sign && self.magnitude == other.magnitude
    }
}
impl Eq for Dz {}
