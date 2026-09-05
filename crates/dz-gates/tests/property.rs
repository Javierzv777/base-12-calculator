use dz_gates::backends::{oracle_add, MvlGates};
use dz_gates::circuits::half_adder;
use dz_gates::{Digit, DozGates};
use proptest::prelude::*;

fn arb_digit() -> impl Strategy<Value = Digit> {
    (0u8..12).prop_map(|t| Digit::new(t).unwrap())
}

proptest! {
    #[test]
    fn min_is_commutative(x in arb_digit(), y in arb_digit()) {
        let g = MvlGates::new();
        prop_assert_eq!(g.min(x, y), g.min(y, x));
    }

    #[test]
    fn max_is_commutative(x in arb_digit(), y in arb_digit()) {
        let g = MvlGates::new();
        prop_assert_eq!(g.max(x, y), g.max(y, x));
    }

    #[test]
    fn half_adder_sum_is_commutative(x in arb_digit(), y in arb_digit()) {
        let g = MvlGates::new();
        prop_assert_eq!(half_adder(&g, x, y), half_adder(&g, y, x));
    }

    #[test]
    fn half_adder_matches_oracle(x in arb_digit(), y in arb_digit()) {
        let g = MvlGates::new();
        prop_assert_eq!(half_adder(&g, x, y), oracle_add(x, y));
    }

    #[test]
    fn cycle_is_a_bijection_over_12_applications(x in arb_digit()) {
        // Applying cycle 12 times must return to the start (it's a single
        // 12-cycle permutation), checked without ever inspecting the tag.
        let g = MvlGates::new();
        let mut acc = x;
        for _ in 0..12 {
            acc = g.cycle(acc);
        }
        prop_assert_eq!(acc, x);
    }
}
