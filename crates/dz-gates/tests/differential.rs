//! Exhaustive 144/144 differential tests: the three backends (MVL, CRT,
//! Table) and the gate-composed adder/multiplier circuits must all agree,
//! for every one of the 12*12 digit-pair combinations, on both addition and
//! multiplication.

use dz_gates::backends::{oracle_add, oracle_mul, CrtGates, MvlGates, TableGates};
use dz_gates::circuits::{digit_mul, digit_mul_v1, digit_mul_v2, half_adder};
use dz_gates::{Digit, DozGates};

fn all_digits() -> Vec<Digit> {
    Digit::all().to_vec()
}

#[test]
fn primitives_agree_across_backends_144() {
    let mvl = MvlGates::new();
    let crt = CrtGates::new();
    let table = TableGates::new();

    for x in all_digits() {
        // cycle is unary; check once per x inside the loop below too.
        assert_eq!(mvl.cycle(x), crt.cycle(x), "cycle mismatch mvl/crt at {:?}", x);
        assert_eq!(mvl.cycle(x), table.cycle(x), "cycle mismatch mvl/table at {:?}", x);

        for y in all_digits() {
            assert_eq!(mvl.min(x, y), crt.min(x, y), "min mismatch at ({:?},{:?})", x, y);
            assert_eq!(mvl.min(x, y), table.min(x, y), "min mismatch at ({:?},{:?})", x, y);

            assert_eq!(mvl.max(x, y), crt.max(x, y), "max mismatch at ({:?},{:?})", x, y);
            assert_eq!(mvl.max(x, y), table.max(x, y), "max mismatch at ({:?},{:?})", x, y);
        }
    }

    for j in 0u8..12 {
        for x in all_digits() {
            assert_eq!(mvl.lit(j, x), crt.lit(j, x), "lit mismatch at ({}, {:?})", j, x);
            assert_eq!(mvl.lit(j, x), table.lit(j, x), "lit mismatch at ({}, {:?})", j, x);

            assert_eq!(mvl.thr(j, x), crt.thr(j, x), "thr mismatch at ({}, {:?})", j, x);
            assert_eq!(mvl.thr(j, x), table.thr(j, x), "thr mismatch at ({}, {:?})", j, x);
        }
    }
}

#[test]
fn half_adder_matches_oracle_144_all_backends() {
    let mvl = MvlGates::new();
    let crt = CrtGates::new();
    let table = TableGates::new();

    let mut count = 0;
    for x in all_digits() {
        for y in all_digits() {
            let (os, oc) = oracle_add(x, y);

            let (s_mvl, c_mvl) = half_adder(&mvl, x, y);
            assert_eq!((s_mvl, c_mvl), (os, oc), "mvl add mismatch at ({:?},{:?})", x, y);

            let (s_crt, c_crt) = half_adder(&crt, x, y);
            assert_eq!((s_crt, c_crt), (os, oc), "crt add mismatch at ({:?},{:?})", x, y);

            let (s_tab, c_tab) = half_adder(&table, x, y);
            assert_eq!((s_tab, c_tab), (os, oc), "table add mismatch at ({:?},{:?})", x, y);

            count += 1;
        }
    }
    assert_eq!(count, 144);
}

#[test]
fn digit_mul_matches_oracle_144_all_backends() {
    // The current default (`digit_mul`, currently v2) against the oracle,
    // on all three `DozGates` backends.
    let mvl = MvlGates::new();
    let crt = CrtGates::new();
    let table = TableGates::new();

    let mut count = 0;
    for x in all_digits() {
        for y in all_digits() {
            let (olo, ohi) = oracle_mul(x, y);

            let (lo_mvl, hi_mvl) = digit_mul(&mvl, x, y);
            assert_eq!((lo_mvl, hi_mvl), (olo, ohi), "mvl mul mismatch at ({:?},{:?})", x, y);

            let (lo_crt, hi_crt) = digit_mul(&crt, x, y);
            assert_eq!((lo_crt, hi_crt), (olo, ohi), "crt mul mismatch at ({:?},{:?})", x, y);

            let (lo_tab, hi_tab) = digit_mul(&table, x, y);
            assert_eq!((lo_tab, hi_tab), (olo, ohi), "table mul mismatch at ({:?},{:?})", x, y);

            count += 1;
        }
    }
    assert_eq!(count, 144);
}

/// v1 (shift-add) vs v2 (SOP) vs the table oracle, all three `DozGates`
/// backends: 144 combinations x 2 multiplier implementations x 3 backends
/// = 864 comparisons, all must agree.
fn check_v1_v2_against_oracle<G: DozGates>(name: &str, g: &G) {
    let mut count = 0;
    for x in all_digits() {
        for y in all_digits() {
            let expected = oracle_mul(x, y);

            let v1 = digit_mul_v1(g, x, y);
            assert_eq!(v1, expected, "v1[{}] mismatch at ({:?},{:?})", name, x, y);

            let v2 = digit_mul_v2(g, x, y);
            assert_eq!(v2, expected, "v2[{}] mismatch at ({:?},{:?})", name, x, y);

            assert_eq!(v1, v2, "v1/v2[{}] disagree at ({:?},{:?})", name, x, y);
            count += 1;
        }
    }
    assert_eq!(count, 144);
}

#[test]
fn digit_mul_v1_v2_and_oracle_agree_144_all_backends() {
    check_v1_v2_against_oracle("mvl", &MvlGates::new());
    check_v1_v2_against_oracle("crt", &CrtGates::new());
    check_v1_v2_against_oracle("table", &TableGates::new());
}
