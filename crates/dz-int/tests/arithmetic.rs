//! `dz-int` correctness tests. The oracle here (`num-bigint`) is used only
//! in test code, per PLAN.md Section 8.3 -- production `dz-int` code never
//! sees it.

use dz_gates::{Digit, MvlGates};
use dz_int::Dz;
use num_bigint::BigInt;
use proptest::prelude::*;

fn digits_le(value: u32) -> Vec<Digit> {
    // Test-only helper: converts a native u32 into dozenal digits so tests
    // can build `Dz` values from ordinary numbers. This lives in test code,
    // which PLAN.md explicitly allows to use an external oracle/conversion;
    // production code (`dz-int`, `dz-parse`) never does this.
    if value == 0 {
        return vec![Digit::ZERO];
    }
    let mut v = value;
    let mut out = Vec::new();
    while v > 0 {
        out.push(Digit::new((v % 12) as u8).unwrap());
        v /= 12;
    }
    out
}

fn dz_from_i64(n: i64) -> Dz {
    let negative = n < 0;
    let mag = digits_le(n.unsigned_abs() as u32);
    Dz::from_magnitude(negative, mag)
}

fn dz_to_bigint(d: &Dz) -> BigInt {
    let mut acc = BigInt::from(0);
    let base = BigInt::from(12);
    for &digit in d.magnitude_digits().iter().rev() {
        acc = acc * &base + BigInt::from(digit_to_u8_test_only(digit));
    }
    match d.sign() {
        dz_int::Sign::Neg => -acc,
        _ => acc,
    }
}

// Test-only: extracting a digit's tag to compare against an external
// oracle is exactly what PLAN.md Section 8.3 sanctions ("el test convierte
// a base 12 desde fuera para comprobar; el código de producción no lo ve").
fn digit_to_u8_test_only(d: Digit) -> u8 {
    d.tag()
}

#[test]
fn basic_addition() {
    let g = MvlGates::new();
    // 4;6 (base12 "46") + ... simple sanity: 8 + 5 = 13(dec) = "11" in base12 (1*12+1)
    let a = dz_from_i64(8);
    let b = dz_from_i64(5);
    let sum = a.add(&g, &b);
    assert_eq!(dz_to_bigint(&sum), BigInt::from(13));
}

#[test]
fn basic_multiplication_4_6_times_3() {
    // From PLAN.md: "4;6 x 3 = 11;6" -- integer part check: 4*3=12=10(base12)
    // full fixed-point check happens in dz-eval/dz-cli tests; here just the
    // integer semantics: 46 (base12, no point) * 3.
    let g = MvlGates::new();
    let a = dz_from_i64(4 * 12 + 6); // "46" base12 = 4*12+6 = 54
    let b = dz_from_i64(3);
    let prod = a.mul(&g, &b);
    assert_eq!(dz_to_bigint(&prod), BigInt::from(54 * 3));
}

#[test]
fn division_and_remainder() {
    let g = MvlGates::new();
    let a = dz_from_i64(100);
    let b = dz_from_i64(7);
    let (q, r) = a.div_rem(&g, &b).unwrap();
    assert_eq!(dz_to_bigint(&q), BigInt::from(100 / 7));
    assert_eq!(dz_to_bigint(&r), BigInt::from(100 % 7));
}

#[test]
fn division_by_zero_errors() {
    let g = MvlGates::new();
    let a = dz_from_i64(10);
    let z = Dz::zero();
    assert!(a.div_rem(&g, &z).is_err());
}

fn arb_small_i64() -> impl Strategy<Value = i64> {
    -5000i64..5000i64
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(300))]

    #[test]
    fn add_matches_bigint(a in arb_small_i64(), b in arb_small_i64()) {
        let g = MvlGates::new();
        let da = dz_from_i64(a);
        let db = dz_from_i64(b);
        let sum = da.add(&g, &db);
        prop_assert_eq!(dz_to_bigint(&sum), BigInt::from(a) + BigInt::from(b));
    }

    #[test]
    fn sub_matches_bigint(a in arb_small_i64(), b in arb_small_i64()) {
        let g = MvlGates::new();
        let da = dz_from_i64(a);
        let db = dz_from_i64(b);
        let diff = da.sub(&g, &db);
        prop_assert_eq!(dz_to_bigint(&diff), BigInt::from(a) - BigInt::from(b));
    }

    #[test]
    fn mul_matches_bigint(a in -300i64..300, b in -300i64..300) {
        let g = MvlGates::new();
        let da = dz_from_i64(a);
        let db = dz_from_i64(b);
        let prod = da.mul(&g, &db);
        prop_assert_eq!(dz_to_bigint(&prod), BigInt::from(a) * BigInt::from(b));
    }

    #[test]
    fn addition_is_commutative(a in arb_small_i64(), b in arb_small_i64()) {
        let g = MvlGates::new();
        let da = dz_from_i64(a);
        let db = dz_from_i64(b);
        prop_assert_eq!(da.add(&g, &db), db.add(&g, &da));
    }

    #[test]
    fn addition_is_associative(a in arb_small_i64(), b in arb_small_i64(), c in arb_small_i64()) {
        let g = MvlGates::new();
        let da = dz_from_i64(a);
        let db = dz_from_i64(b);
        let dc = dz_from_i64(c);
        let left = da.add(&g, &db).add(&g, &dc);
        let right = da.add(&g, &db.add(&g, &dc));
        prop_assert_eq!(left, right);
    }

    #[test]
    fn multiplication_distributes_over_addition(a in -200i64..200, b in -200i64..200, c in -200i64..200) {
        let g = MvlGates::new();
        let da = dz_from_i64(a);
        let db = dz_from_i64(b);
        let dc = dz_from_i64(c);
        let left = da.mul(&g, &db.add(&g, &dc));
        let right = da.mul(&g, &db).add(&g, &da.mul(&g, &dc));
        prop_assert_eq!(left, right);
    }

    #[test]
    fn add_then_sub_roundtrips(a in arb_small_i64(), b in arb_small_i64()) {
        let g = MvlGates::new();
        let da = dz_from_i64(a);
        let db = dz_from_i64(b);
        let result = da.add(&g, &db).sub(&g, &db);
        prop_assert_eq!(result, da);
    }

    #[test]
    fn mul_then_div_roundtrips(a in -500i64..500, b in 1i64..50) {
        let g = MvlGates::new();
        let da = dz_from_i64(a);
        let db = dz_from_i64(b);
        let prod = da.mul(&g, &db);
        let (q, _r) = prod.div_rem(&g, &db).unwrap();
        prop_assert_eq!(q, da);
    }

    #[test]
    fn compare_matches_bigint(a in arb_small_i64(), b in arb_small_i64()) {
        let g = MvlGates::new();
        let da = dz_from_i64(a);
        let db = dz_from_i64(b);
        let ord = da.compare(&g, &db);
        prop_assert_eq!(ord, a.cmp(&b));
    }
}
