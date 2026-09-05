//! `DzFloat` (DFP) tests: round-trip for exact values, half-even tie
//! resolution, rounding monotonicity, and overflow/underflow to the
//! reserved special values.

use dz_gates::{Digit, MvlGates};
use dz_int::Dz;
use dz_num::{DfpKind, DzFixed, DzFloat};
use proptest::prelude::*;
use std::cmp::Ordering;

fn digits_le(mut v: u32) -> Vec<Digit> {
    if v == 0 {
        return vec![Digit::ZERO];
    }
    let mut out = Vec::new();
    while v > 0 {
        out.push(Digit::new((v % 12) as u8).unwrap());
        v /= 12;
    }
    out
}

fn fixed_from_i64(n: i64, scale: usize) -> DzFixed {
    DzFixed { magnitude: Dz::from_magnitude(n < 0, digits_le(n.unsigned_abs() as u32)), scale }
}

#[test]
fn zero_round_trips() {
    let g = MvlGates::new();
    let z = DzFixed::zero();
    let f = DzFloat::from_dzfixed(&g, &z);
    assert_eq!(f.kind, DfpKind::Zero);
    let back = f.to_dzfixed(&g).unwrap();
    assert_eq!(back, z);
}

#[test]
fn small_exact_values_round_trip() {
    let g = MvlGates::new();
    for &(n, scale) in &[(1i64, 0usize), (-1, 0), (54, 0), (46, 1), (4, 1), (-100, 2)] {
        let fx = fixed_from_i64(n, scale);
        let f = DzFloat::from_dzfixed(&g, &fx);
        assert_eq!(f.kind, DfpKind::Normal, "value {}/{} should be Normal", n, scale);
        let back = f.to_dzfixed(&g).unwrap();
        assert_eq!(back.compare(&g, &fx), Ordering::Equal, "round-trip mismatch for {}e-{}", n, scale);
    }
}

/// Exactly 12 significant dozenal digits: fits the mantissa exactly, no
/// rounding needed.
#[test]
fn twelve_digit_value_is_exact() {
    let g = MvlGates::new();
    // 12^11 (a 1 followed by 11 zeros) has exactly 12 significant digits.
    let mut digits = vec![Digit::ZERO; 11];
    digits.push(Digit::ONE);
    let fx = DzFixed { magnitude: Dz::from_magnitude(false, digits), scale: 0 };
    let f = DzFloat::from_dzfixed(&g, &fx);
    assert_eq!(f.kind, DfpKind::Normal);
    let back = f.to_dzfixed(&g).unwrap();
    assert_eq!(back.compare(&g, &fx), Ordering::Equal);
}

/// A value with 13 significant digits (little-endian indices 0..=12,
/// index 0 = least significant = the digit that gets dropped when
/// rounding to a 12-digit mantissa, index 12 = most significant) where the
/// dropped digit is exactly `6` (the one true tie: `2*6 == 12`). It must
/// round to whichever neighboring mantissa is *even*.
#[test]
fn half_even_tie_rounds_to_even_mantissa() {
    let g = MvlGates::new();

    // index 0 = 6 (dropped, the tie), index 1 = 0 (becomes the retained
    // mantissa's new LSD after the shift -- already even), index 12 = 2
    // (nonzero MSD, keeps the number at 13 significant digits).
    let mut digits_even = vec![Digit::ZERO; 13];
    digits_even[0] = Digit::new(6).unwrap();
    digits_even[12] = Digit::new(2).unwrap();
    let fx_even = DzFixed { magnitude: Dz::from_magnitude(false, digits_even), scale: 0 };
    let f_even = DzFloat::from_dzfixed(&g, &fx_even);
    assert_eq!(f_even.kind, DfpKind::Normal);
    assert!(f_even.mantissa.is_even(&g), "already-even retained mantissa must not change on a tie");

    // Same, but index 1 = 1 (odd): the tie must round up to make it even.
    let mut digits_odd = vec![Digit::ZERO; 13];
    digits_odd[0] = Digit::new(6).unwrap();
    digits_odd[1] = Digit::new(1).unwrap();
    digits_odd[12] = Digit::new(2).unwrap();
    let fx_odd = DzFixed { magnitude: Dz::from_magnitude(false, digits_odd), scale: 0 };
    let f_odd = DzFloat::from_dzfixed(&g, &fx_odd);
    assert_eq!(f_odd.kind, DfpKind::Normal);
    assert!(f_odd.mantissa.is_even(&g), "odd retained mantissa + exact tie must round up to even");
}

/// A dropped remainder strictly more than half (here, dropped digit `7`,
/// so `2*7=14 > 12`) always rounds up, unambiguously -- not a tie. Before
/// rounding, the retained mantissa's new LSD (index 1 of the 13-digit
/// input, left at `0`) would be `0`; after rounding up it must become `1`.
#[test]
fn more_than_half_always_rounds_up() {
    let g = MvlGates::new();
    let mut digits = vec![Digit::ZERO; 13];
    digits[0] = Digit::new(7).unwrap(); // dropped digit = 7 > 6: unambiguous round up
    digits[12] = Digit::new(2).unwrap();
    let fx = DzFixed { magnitude: Dz::from_magnitude(false, digits), scale: 0 };
    let f = DzFloat::from_dzfixed(&g, &fx);
    assert_eq!(f.kind, DfpKind::Normal);
    assert_eq!(f.mantissa.magnitude_digits()[0], Digit::ONE, "7/12 > half must round up");
}

/// Overflow: a value whose exponent would exceed the representable range
/// flushes to Infinity.
#[test]
fn overflow_flushes_to_infinity() {
    let g = MvlGates::new();
    // 12^200 has far too large an exponent to represent (max unbiased ~71).
    let mut digits = vec![Digit::ZERO; 200];
    digits.push(Digit::ONE);
    let fx = DzFixed { magnitude: Dz::from_magnitude(false, digits), scale: 0 };
    let f = DzFloat::from_dzfixed(&g, &fx);
    assert_eq!(f.kind, DfpKind::Infinity);
    assert!(!f.sign);
}

/// Underflow: an extremely small value flushes to zero (no subnormals).
#[test]
fn underflow_flushes_to_zero() {
    let g = MvlGates::new();
    let fx = DzFixed { magnitude: Dz::from_magnitude(false, vec![Digit::ONE]), scale: 200 };
    let f = DzFloat::from_dzfixed(&g, &fx);
    assert_eq!(f.kind, DfpKind::Zero);
}

#[test]
fn special_value_arithmetic() {
    let g = MvlGates::new();
    let inf = DzFloat::infinity(false);
    let neg_inf = DzFloat::infinity(true);
    let nan = DzFloat::nan();
    let one = DzFloat::from_dzfixed(&g, &fixed_from_i64(1, 0));
    let zero = DzFloat::zero(false);

    assert!(inf.add(&g, &neg_inf).is_nan());
    assert!(inf.add(&g, &one).is_infinite());
    assert!(zero.mul(&g, &inf).is_nan());
    assert!(one.div(&g, &zero).is_infinite());
    assert!(zero.div(&g, &zero).is_nan());
    assert!(nan.add(&g, &one).is_nan());
    assert_eq!(inf.compare(&g, &one), Some(Ordering::Greater));
    assert_eq!(nan.compare(&g, &one), None);
}

fn arb_small_i64() -> impl Strategy<Value = i64> {
    -100_000i64..100_000i64
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(60))]

    /// Values that fit within the mantissa exactly must round-trip exactly.
    #[test]
    fn exact_roundtrip_for_small_values(n in arb_small_i64(), scale in 0usize..6) {
        let g = MvlGates::new();
        let fx = fixed_from_i64(n, scale);
        let f = DzFloat::from_dzfixed(&g, &fx);
        if n == 0 {
            prop_assert_eq!(f.kind, DfpKind::Zero);
        } else {
            prop_assert_eq!(f.kind, DfpKind::Normal);
            let back = f.to_dzfixed(&g).unwrap();
            prop_assert_eq!(back.compare(&g, &fx), Ordering::Equal);
        }
    }

    /// Rounding to the DFP mantissa preserves order: if a <= b exactly,
    /// then round(a) <= round(b). This is the standard monotonicity
    /// property any correctly-rounded format must satisfy.
    #[test]
    fn rounding_is_monotonic(a in arb_small_i64(), b in arb_small_i64(), scale in 0usize..4) {
        let g = MvlGates::new();
        let fa = fixed_from_i64(a, scale);
        let fb = fixed_from_i64(b, scale);
        let exact_order = fa.compare(&g, &fb);

        let da = DzFloat::from_dzfixed(&g, &fa);
        let db = DzFloat::from_dzfixed(&g, &fb);
        let rounded_order = da.compare(&g, &db).unwrap();

        match exact_order {
            Ordering::Less => prop_assert_ne!(rounded_order, Ordering::Greater),
            Ordering::Greater => prop_assert_ne!(rounded_order, Ordering::Less),
            Ordering::Equal => prop_assert_eq!(rounded_order, Ordering::Equal),
        }
    }

    /// `add` composed of exact small values that stay within mantissa
    /// precision must match plain integer addition exactly.
    #[test]
    fn add_is_exact_for_small_values(a in -1000i64..1000, b in -1000i64..1000) {
        let g = MvlGates::new();
        let fa = DzFloat::from_dzfixed(&g, &fixed_from_i64(a, 0));
        let fb = DzFloat::from_dzfixed(&g, &fixed_from_i64(b, 0));
        let sum = fa.add(&g, &fb);
        let expected = DzFloat::from_dzfixed(&g, &fixed_from_i64(a + b, 0));
        prop_assert_eq!(sum.compare(&g, &expected), Some(Ordering::Equal));
    }
}
