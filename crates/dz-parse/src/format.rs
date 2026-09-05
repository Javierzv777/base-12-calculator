//! Format a `DzFixed` back into dozenal text, mapping `Digit -> char` via
//! the alphabet (PLAN.md Section 5: "la salida se formatea mapeando
//! Digit -> char. Ningun numero cruza la frontera hacia binario/decimal
//! nativo."). Walking `magnitude_digits()` by index and comparing `scale`
//! (a `usize`) is bookkeeping, not arithmetic on a digit's value.

use crate::alphabet::{digit_to_char, POINT};
use dz_gates::{Digit, DozGates};
use dz_num::{DfpKind, DzFixed, DzFloat, DzRational};

pub fn format_fixed(value: &DzFixed) -> String {
    let digits = value.magnitude_digits(); // little-endian
    let scale = value.scale;
    let mut out = String::new();
    if value.is_negative() {
        out.push('-');
    }

    let len = digits.len();
    if len <= scale {
        out.push('0');
    } else {
        // Integer part: indices [scale, len) most-significant first.
        let mut i = len;
        while i > scale {
            i -= 1;
            out.push(digit_to_char(digits[i]));
        }
    }

    if scale > 0 {
        out.push(POINT);
        // Fractional part: indices [0, scale) most-significant-fractional
        // first, i.e. descending from scale-1 to 0.
        let mut i = scale;
        while i > 0 {
            i -= 1;
            let d = if i < digits.len() { digits[i] } else { Digit::ZERO };
            out.push(digit_to_char(d));
        }
    }
    out
}

/// Render little-endian digits most-significant-first, e.g. for an
/// integer numerator/denominator with no Humphrey point.
fn format_digits_msd(digits: &[Digit]) -> String {
    let mut out = String::with_capacity(digits.len());
    let mut i = digits.len();
    while i > 0 {
        i -= 1;
        out.push(digit_to_char(digits[i]));
    }
    out
}

/// Format an exact `DzRational` as `num` (when the denominator is 1) or
/// `num/den` otherwise -- the default REPL mode's display, so `1/3` prints
/// as the exact fraction, never a truncated approximation.
pub fn format_rational(value: &DzRational) -> String {
    let mut out = String::new();
    if value.is_negative() {
        out.push('-');
    }
    out.push_str(&format_digits_msd(value.num_digits()));
    let den = value.den_digits();
    let den_is_one = den.len() == 1 && den[0] == Digit::ONE;
    if !den_is_one {
        out.push('/');
        out.push_str(&format_digits_msd(value.den_digits()));
    }
    out
}

/// Format a `DzFloat` (DFP), decoding Normal values back to an exact
/// `DzFixed` (via the given backend) and reusing `format_fixed`; special
/// values print as `0`/`-0`/`Inf`/`-Inf`/`NaN`.
pub fn format_dfp<G: DozGates>(g: &G, value: &DzFloat) -> String {
    match value.kind {
        DfpKind::NaN => "NaN".to_string(),
        DfpKind::Infinity => {
            if value.sign {
                "-Inf".to_string()
            } else {
                "Inf".to_string()
            }
        }
        DfpKind::Zero => {
            if value.sign {
                "-0".to_string()
            } else {
                "0".to_string()
            }
        }
        DfpKind::Normal => {
            let fx = value.to_dzfixed(g).expect("Normal always decodes");
            format_fixed(&fx)
        }
    }
}
