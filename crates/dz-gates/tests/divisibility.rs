//! The last-digit divisibility decoder against a native oracle (tests may
//! read tags and use `%`; production code may not), plus its measured cost.

use dz_gates::circuits::{
    divisor_row, last_digit_divisible, last_digit_divisible_net, max_level, GateNet, Levels,
    SMALL_DIVISOR_TAGS,
};
use dz_gates::{Digit, DozGates, GateCount, MvlGates};

fn digit(t: u8) -> Digit {
    Digit::new(t).unwrap()
}

#[test]
fn rows_are_the_multiples_for_every_divisor_of_twelve() {
    let mg = MvlGates::new();
    // 2, 3, 4, 6 and twelve itself (keyed by digit zero).
    let mut cases: Vec<(u8, u8)> = SMALL_DIVISOR_TAGS.iter().map(|&d| (d, d)).collect();
    cases.push((0, 12));
    for (key, divisor) in cases {
        for x in 0u8..12 {
            let got = last_digit_divisible(&mg, digit(key), digit(x));
            let want = x % divisor == 0;
            assert_eq!(got != Digit::ZERO, want, "divisor {} last digit {}", divisor, x);
            // An MVL boolean: exactly ZERO or MAX.
            assert!(got == Digit::ZERO || got == Digit::MAX);
        }
    }
}

#[test]
fn row_for_two_matches_the_even_digits() {
    let row = divisor_row(digit(2));
    for x in 0u8..12 {
        assert_eq!(row[x as usize] != Digit::ZERO, x % 2 == 0);
    }
}

#[test]
fn cost_is_thirty_five_primitives_and_depth_thirteen_for_every_divisor() {
    for key in [2u8, 3, 4, 6, 0] {
        let mg = MvlGates::new();
        mg.reset_cost();
        let _ = last_digit_divisible(&mg, digit(key), digit(7));
        let c: GateCount = mg.cost();
        assert_eq!((c.lit, c.min, c.max, c.cycle, c.thr), (12, 12, 11, 0, 0));
        assert_eq!(c.total(), 35);

        let mg2 = MvlGates::new();
        let net = Levels(&mg2);
        let out = last_digit_divisible_net(&net, &divisor_row(digit(key)), net.lift(digit(7)));
        assert_eq!(max_level(&[out]), 13);
    }
}

#[test]
fn building_the_rows_is_not_billed_to_the_caller() {
    // The table is synthesized with a throwaway backend, so the first call
    // costs the same as the hundredth.
    let mg = MvlGates::new();
    mg.reset_cost();
    let _ = last_digit_divisible(&mg, digit(3), digit(9));
    let first = mg.cost().total();
    mg.reset_cost();
    let _ = last_digit_divisible(&mg, digit(3), digit(9));
    assert_eq!(first, mg.cost().total());
}
