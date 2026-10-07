//! `valuation` and `divisible` against a native oracle (tests only).

use dz_gates::circuits::DepthScope;
use dz_gates::{Digit, DozGates, MvlGates};
use dz_int::{
    divisible, last_digit_divisible, valuation, DivisibilityMethod, Dz, ValuationError, ZeroDivisor,
};
use proptest::prelude::*;

fn dz(value: i64) -> Dz {
    let negative = value < 0;
    let mut v = value.unsigned_abs();
    let mut out = Vec::new();
    if v == 0 {
        out.push(Digit::ZERO);
    }
    while v > 0 {
        out.push(Digit::new((v % 12) as u8).unwrap());
        v /= 12;
    }
    Dz::from_magnitude(negative, out)
}

fn native(d: &Dz) -> u64 {
    let mut acc: u64 = 0;
    for &digit in d.magnitude_digits().iter().rev() {
        acc = acc * 12 + digit.tag() as u64;
    }
    acc
}

fn oracle_valuation(mut n: u64, p: u64) -> (u64, u64) {
    let mut k = 0;
    while n.is_multiple_of(p) {
        n /= p;
        k += 1;
    }
    (k, n)
}

#[test]
fn valuation_small_known_values() {
    let g = MvlGates::new();
    // 15000 (dec) = 2^3 * 3 * 5^4
    let n = dz(15000);
    for (p, want) in [(2, 3), (3, 1), (5, 4), (7, 0), (11, 0)] {
        let v = valuation(&g, &n, &dz(p)).unwrap();
        assert_eq!(native(&v.exponent), want, "p = {}", p);
    }
    // composite p: 12 divides 144 twice, 6 divides 144 twice (36 | 144)
    assert_eq!(native(&valuation(&g, &dz(144), &dz(12)).unwrap().exponent), 2);
    assert_eq!(native(&valuation(&g, &dz(144), &dz(6)).unwrap().exponent), 2);
}

#[test]
fn valuation_returns_the_cofactor() {
    let g = MvlGates::new();
    let v = valuation(&g, &dz(15000), &dz(5)).unwrap();
    assert_eq!(native(&v.cofactor), 24);
    assert_eq!(native(&v.exponent), 4);
}

#[test]
fn valuation_ignores_the_sign_of_n() {
    let g = MvlGates::new();
    let v = valuation(&g, &dz(-48), &dz(2)).unwrap();
    assert_eq!(native(&v.exponent), 4);
    assert_eq!(native(&v.cofactor), 3);
}

#[test]
fn valuation_of_one_is_zero_with_one_division() {
    let g = MvlGates::new();
    let v = valuation(&g, &dz(1), &dz(7)).unwrap();
    assert!(v.exponent.is_zero());
    assert_eq!(native(&v.cofactor), 1);
}

#[test]
fn valuation_error_paths() {
    let g = MvlGates::new();
    assert_eq!(valuation(&g, &dz(0), &dz(2)).unwrap_err(), ValuationError::ZeroNumber);
    for p in [-3, 0, 1] {
        assert_eq!(
            valuation(&g, &dz(24), &dz(p)).unwrap_err(),
            ValuationError::DivisorTooSmall,
            "p = {}",
            p
        );
    }
    // p is checked first: v_1(0) reports the divisor, not the zero.
    assert_eq!(valuation(&g, &dz(0), &dz(1)).unwrap_err(), ValuationError::DivisorTooSmall);
}

#[test]
fn last_digit_rule_matches_native_for_divisors_of_twelve() {
    let g = MvlGates::new();
    for d in [2i64, 3, 4, 6, 12] {
        for n in [-144i64, -13, 0, 1, 5, 8, 9, 12, 25, 30, 36, 143, 144, 1000, 1728, 1735] {
            let got = last_digit_divisible(&g, &dz(n), &dz(d)).expect("d divides twelve");
            assert_eq!(got, n % d == 0, "n = {}, d = {}", n, d);
        }
    }
}

#[test]
fn last_digit_rule_declines_other_divisors() {
    let g = MvlGates::new();
    for d in [0i64, 1, 5, 7, 8, 9, 10, 11, 13, 24, 144] {
        assert_eq!(last_digit_divisible(&g, &dz(30), &dz(d)), None, "d = {}", d);
    }
}

#[test]
fn divisible_falls_back_to_div_rem_and_says_so() {
    let g = MvlGates::new();
    let a = divisible(&g, &dz(35), &dz(7)).unwrap();
    assert!(a.divisible);
    assert_eq!(a.method, DivisibilityMethod::DivRem);
    let b = divisible(&g, &dz(36), &dz(4)).unwrap();
    assert!(b.divisible);
    assert_eq!(b.method, DivisibilityMethod::LastDigit);
    assert_eq!(divisible(&g, &dz(36), &dz(0)).unwrap_err(), ZeroDivisor);
}

#[test]
fn last_digit_rule_is_much_cheaper_than_div_rem() {
    let n = dz(1_234_567);
    let g1 = MvlGates::new();
    g1.reset_cost();
    let _ = last_digit_divisible(&g1, &n, &dz(3));
    let rule = g1.cost().total();
    let g2 = MvlGates::new();
    g2.reset_cost();
    let _ = n.div_rem(&g2, &dz(3)).unwrap();
    let division = g2.cost().total();
    assert!(rule * 100 < division, "rule {} vs div_rem {}", rule, division);
}

#[test]
fn depth_is_measured_inside_a_scope_and_counts_do_not_change() {
    let n = dz(1000);
    let d = dz(4);
    let plain = MvlGates::new();
    plain.reset_cost();
    let a = last_digit_divisible(&plain, &n, &d);
    let plain_cost = plain.cost();

    let tracked = MvlGates::new();
    tracked.reset_cost();
    let scope = DepthScope::enter();
    let b = last_digit_divisible(&tracked, &n, &d);
    let depth = scope.measured();
    assert_eq!(a, b);
    assert_eq!(plain_cost, tracked.cost(), "measuring depth must not change the count");
    assert!(depth >= 13, "depth {}", depth);
}

proptest! {
    #[test]
    fn valuation_matches_oracle(n in 1u64..1_000_000, p in 2u64..40) {
        let g = MvlGates::new();
        let v = valuation(&g, &dz(n as i64), &dz(p as i64)).unwrap();
        let (k, rest) = oracle_valuation(n, p);
        prop_assert_eq!(native(&v.exponent), k);
        prop_assert_eq!(native(&v.cofactor), rest);
    }

    #[test]
    fn divisible_matches_oracle(n in 0u64..1_000_000, d in 1u64..40) {
        let g = MvlGates::new();
        let r = divisible(&g, &dz(n as i64), &dz(d as i64)).unwrap();
        prop_assert_eq!(r.divisible, n % d == 0);
    }
}
