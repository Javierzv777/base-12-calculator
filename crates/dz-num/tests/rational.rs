//! `DzRational` correctness tests. `num-bigint`/`num-rational` are used
//! only here, as the external oracle (PLAN.md Section 8.3) -- never in
//! `dz-num`'s production code.

use dz_gates::{Digit, MvlGates};
use dz_int::Dz;
use dz_num::DzRational;
use num_bigint::BigInt;
use num_rational::BigRational;
use proptest::prelude::*;

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

fn dz_from_i64(n: i64) -> Dz {
    Dz::from_magnitude(n < 0, digits_le(n.unsigned_abs() as u32))
}

fn rat_from_i64s(g: &MvlGates, num: i64, den: i64) -> DzRational {
    DzRational::new(g, dz_from_i64(num), dz_from_i64(den)).unwrap()
}

fn digit_to_u8_test_only(d: Digit) -> u8 {
    d.tag()
}

fn dz_to_bigint(d: &Dz) -> BigInt {
    let mut acc = BigInt::from(0);
    let base = BigInt::from(12);
    for &digit in d.magnitude_digits().iter().rev() {
        acc = acc * &base + BigInt::from(digit_to_u8_test_only(digit));
    }
    if d.sign() == dz_int::Sign::Neg {
        -acc
    } else {
        acc
    }
}

fn rat_to_bigrational(r: &DzRational) -> BigRational {
    BigRational::new(dz_to_bigint(&r.num), dz_to_bigint(&r.den))
}

// Matches `dz-int`'s property test range now that `Dz` multiplication uses
// the SOP multiplier v2 (876 gate calls per single-digit multiply, ~38.5x
// cheaper than v1's ~34k -- see docs/01-diseno-sumador.md) instead of v1's
// shift-add. `DzRational` normalizes with Euclid's algorithm on every
// operation (gcd via repeated `Dz::div_rem`, each doing several `Dz::mul`
// trial multiplications), so this remains the more expensive of the two
// test suites, but v2 made ±5000/300 cases practical (~1 minute; see the
// measured time reported alongside this file in the final report).
fn arb_small_i64() -> impl Strategy<Value = i64> {
    -5000i64..5000i64
}
fn arb_nonzero_small_i64() -> impl Strategy<Value = i64> {
    (1i64..5000i64).prop_flat_map(|x| prop_oneof![Just(x), Just(-x)])
}

#[test]
fn basic_reduction() {
    let g = MvlGates::new();
    let r = rat_from_i64s(&g, 4, 8);
    assert_eq!(dz_to_bigint(&r.num), BigInt::from(1));
    assert_eq!(dz_to_bigint(&r.den), BigInt::from(2));
}

#[test]
fn negative_denominator_normalizes() {
    let g = MvlGates::new();
    let r = rat_from_i64s(&g, 1, -3);
    assert_eq!(dz_to_bigint(&r.num), BigInt::from(-1));
    assert_eq!(dz_to_bigint(&r.den), BigInt::from(3));
}

#[test]
fn one_third_terminates_is_false_one_quarter_is_true() {
    let g = MvlGates::new();
    let one_third = rat_from_i64s(&g, 1, 3);
    let one_quarter = rat_from_i64s(&g, 1, 4);
    let one_fifth = rat_from_i64s(&g, 1, 5);
    assert!(one_third.terminates_in_base12(&g));
    assert!(one_quarter.terminates_in_base12(&g));
    assert!(!one_fifth.terminates_in_base12(&g));
}

#[test]
fn to_fixed_one_third_is_exact_0_4() {
    let g = MvlGates::new();
    let one_third = rat_from_i64s(&g, 1, 3);
    let fx = one_third.to_fixed(&g, 8);
    // 1/3 = 0;4 exactly, so trimmed scale should be 1 and magnitude digit 4.
    assert_eq!(fx.scale, 1);
    assert_eq!(dz_to_bigint(&fx.magnitude), BigInt::from(4));
}

#[test]
fn division_by_zero_errors() {
    let g = MvlGates::new();
    let a = rat_from_i64s(&g, 1, 2);
    let z = DzRational::zero();
    assert!(a.div(&g, &z).is_err());
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(300))]

    #[test]
    fn add_matches_bigrational(
        (an, ad) in (arb_small_i64(), arb_nonzero_small_i64()),
        (bn, bd) in (arb_small_i64(), arb_nonzero_small_i64()),
    ) {
        let g = MvlGates::new();
        let a = rat_from_i64s(&g, an, ad);
        let b = rat_from_i64s(&g, bn, bd);
        let sum = a.add(&g, &b).unwrap();
        let expected = rat_to_bigrational(&a) + rat_to_bigrational(&b);
        prop_assert_eq!(rat_to_bigrational(&sum), expected);
    }

    #[test]
    fn mul_matches_bigrational(
        (an, ad) in (arb_small_i64(), arb_nonzero_small_i64()),
        (bn, bd) in (arb_small_i64(), arb_nonzero_small_i64()),
    ) {
        let g = MvlGates::new();
        let a = rat_from_i64s(&g, an, ad);
        let b = rat_from_i64s(&g, bn, bd);
        let prod = a.mul(&g, &b).unwrap();
        let expected = rat_to_bigrational(&a) * rat_to_bigrational(&b);
        prop_assert_eq!(rat_to_bigrational(&prod), expected);
    }

    #[test]
    fn addition_is_associative(
        (an, ad) in (arb_small_i64(), arb_nonzero_small_i64()),
        (bn, bd) in (arb_small_i64(), arb_nonzero_small_i64()),
        (cn, cd) in (arb_small_i64(), arb_nonzero_small_i64()),
    ) {
        let g = MvlGates::new();
        let a = rat_from_i64s(&g, an, ad);
        let b = rat_from_i64s(&g, bn, bd);
        let c = rat_from_i64s(&g, cn, cd);
        let left = a.add(&g, &b).unwrap().add(&g, &c).unwrap();
        let right = a.add(&g, &b.add(&g, &c).unwrap()).unwrap();
        prop_assert_eq!(left, right);
    }

    #[test]
    fn multiplication_distributes_over_addition(
        (an, ad) in (arb_small_i64(), arb_nonzero_small_i64()),
        (bn, bd) in (arb_small_i64(), arb_nonzero_small_i64()),
        (cn, cd) in (arb_small_i64(), arb_nonzero_small_i64()),
    ) {
        let g = MvlGates::new();
        let a = rat_from_i64s(&g, an, ad);
        let b = rat_from_i64s(&g, bn, bd);
        let c = rat_from_i64s(&g, cn, cd);
        let left = a.mul(&g, &b.add(&g, &c).unwrap()).unwrap();
        let right = a.mul(&g, &b).unwrap().add(&g, &a.mul(&g, &c).unwrap()).unwrap();
        prop_assert_eq!(left, right);
    }

    #[test]
    fn add_then_sub_roundtrips(
        (an, ad) in (arb_small_i64(), arb_nonzero_small_i64()),
        (bn, bd) in (arb_small_i64(), arb_nonzero_small_i64()),
    ) {
        let g = MvlGates::new();
        let a = rat_from_i64s(&g, an, ad);
        let b = rat_from_i64s(&g, bn, bd);
        let result = a.add(&g, &b).unwrap().sub(&g, &b).unwrap();
        prop_assert_eq!(result, a);
    }

    #[test]
    fn div_then_mul_roundtrips(
        (an, ad) in (arb_small_i64(), arb_nonzero_small_i64()),
        (bn, bd) in (arb_nonzero_small_i64(), arb_nonzero_small_i64()),
    ) {
        let g = MvlGates::new();
        let a = rat_from_i64s(&g, an, ad);
        let b = rat_from_i64s(&g, bn, bd);
        let result = a.div(&g, &b).unwrap().mul(&g, &b).unwrap();
        prop_assert_eq!(result, a);
    }

    #[test]
    fn canonical_form_is_unique(
        (an, ad) in (arb_small_i64(), arb_nonzero_small_i64()),
        k in 1i64..20,
    ) {
        // a/d and (a*k)/(d*k) must reduce to bit-for-bit the same (num, den).
        let g = MvlGates::new();
        let a = rat_from_i64s(&g, an, ad);
        let b = rat_from_i64s(&g, an * k, ad * k);
        prop_assert_eq!(a, b);
    }

    #[test]
    fn compare_matches_bigrational(
        (an, ad) in (arb_small_i64(), arb_nonzero_small_i64()),
        (bn, bd) in (arb_small_i64(), arb_nonzero_small_i64()),
    ) {
        let g = MvlGates::new();
        let a = rat_from_i64s(&g, an, ad);
        let b = rat_from_i64s(&g, bn, bd);
        let ord = a.compare(&g, &b);
        let expected = rat_to_bigrational(&a).cmp(&rat_to_bigrational(&b));
        prop_assert_eq!(ord, expected);
    }
}
