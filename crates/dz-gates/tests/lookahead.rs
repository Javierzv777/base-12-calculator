//! Phase 8: carry-lookahead adder.
//!
//! Two things are pinned here:
//!
//! 1. **Equivalence.** `lookahead_add` must agree with `ripple_add` on the
//!    returned digits *and* on the returned `Vec`'s length, for every input
//!    -- it is meant to be a drop-in replacement, not merely "numerically
//!    equal up to leading zeros".
//! 2. **The depth model.** `circuits::depth` is instrumentation, so its
//!    numbers are only meaningful if they are stable. The tests below fix
//!    the measured depths, and the published gate counts (151, 1212), so a
//!    future refactor of the shared generic adder body cannot silently
//!    change either.

use dz_gates::backends::{oracle_add, CrtGates, MvlGates, TableGates};
use dz_gates::circuits::{
    generate_propagate, half_adder, half_adder_balanced, half_adder_balanced_depth,
    half_adder_depth, lookahead_add, lookahead_add_balanced, lookahead_add_balanced_depth,
    lookahead_add_depth, ripple_add, ripple_add_depth,
};
use dz_gates::{Digit, DozGates};
use proptest::prelude::*;

fn all_digits() -> Vec<Digit> {
    Digit::all().to_vec()
}

fn d(tag: u8) -> Digit {
    Digit::new(tag).unwrap()
}

// ---------------------------------------------------------------------
// (a) Exhaustive single digit, all three backends, against both
//     `ripple_add` and the table oracle.
// ---------------------------------------------------------------------

fn check_single_digit_exhaustive<G: DozGates>(name: &str, g: &G) {
    let mut count = 0;
    for x in all_digits() {
        for y in all_digits() {
            let ripple = ripple_add(g, &[x], &[y]);
            let look = lookahead_add(g, &[x], &[y]);
            assert_eq!(
                look, ripple,
                "[{}] lookahead != ripple at ({:?},{:?})",
                name, x, y
            );

            let (os, oc) = oracle_add(x, y);
            let expected = if oc == Digit::ZERO {
                vec![os]
            } else {
                vec![os, oc]
            };
            assert_eq!(
                look, expected,
                "[{}] lookahead != oracle at ({:?},{:?})",
                name, x, y
            );
            count += 1;
        }
    }
    assert_eq!(count, 144);
}

#[test]
fn lookahead_single_digit_exhaustive_all_backends() {
    check_single_digit_exhaustive("mvl", &MvlGates::new());
    check_single_digit_exhaustive("crt", &CrtGates::new());
    check_single_digit_exhaustive("table", &TableGates::new());
}

// ---------------------------------------------------------------------
// (b) Exhaustive two digits: 12^4 = 20736 pairs of two-digit numbers.
//     One backend only (MVL, the production one); the cross-backend
//     agreement is already covered exhaustively above and in
//     `tests/differential.rs`.
// ---------------------------------------------------------------------

#[test]
fn lookahead_two_digit_exhaustive_matches_ripple() {
    let g = MvlGates::new();
    let mut count = 0u32;
    for a1 in all_digits() {
        for a0 in all_digits() {
            let a = [a0, a1];
            for b1 in all_digits() {
                for b0 in all_digits() {
                    let b = [b0, b1];
                    assert_eq!(
                        lookahead_add(&g, &a, &b),
                        ripple_add(&g, &a, &b),
                        "mismatch at {:?} + {:?}",
                        a,
                        b
                    );
                    count += 1;
                }
            }
        }
    }
    assert_eq!(count, 20736);
}

// ---------------------------------------------------------------------
// (c) Mismatched operand lengths, in both directions.
// ---------------------------------------------------------------------

#[test]
fn lookahead_handles_unequal_lengths() {
    let g = MvlGates::new();
    let long = vec![d(11), d(11), d(11), d(5), d(7)];
    let short = vec![d(1)];
    let zero = vec![Digit::ZERO];

    for (a, b) in [
        (long.clone(), short.clone()),
        (short.clone(), long.clone()),
        (long.clone(), zero.clone()),
        (zero.clone(), long.clone()),
        (zero.clone(), zero.clone()),
        (long.clone(), vec![d(2), d(3)]),
        (vec![d(2), d(3)], long.clone()),
    ] {
        assert_eq!(
            lookahead_add(&g, &a, &b),
            ripple_add(&g, &a, &b),
            "mismatch at {:?} + {:?}",
            a,
            b
        );
    }
}

#[test]
fn lookahead_empty_operands_match_ripple() {
    let g = MvlGates::new();
    let empty: [Digit; 0] = [];
    assert_eq!(lookahead_add(&g, &empty, &empty), ripple_add(&g, &empty, &empty));
    assert_eq!(lookahead_add(&g, &[d(4)], &empty), ripple_add(&g, &[d(4)], &empty));
    assert_eq!(lookahead_add(&g, &empty, &[d(4)]), ripple_add(&g, &empty, &[d(4)]));
}

// ---------------------------------------------------------------------
// (d) Worst-case carry propagation: the whole width propagates.
// ---------------------------------------------------------------------

#[test]
fn lookahead_full_length_carry_chain() {
    let g = MvlGates::new();

    // [11,11,...,11] + [1]: every position propagates, and the carry has to
    // reach the top and grow the Vec by one limb.
    for n in 1usize..=16 {
        let a = vec![Digit::MAX; n];
        let b = vec![Digit::ONE];
        let look = lookahead_add(&g, &a, &b);
        assert_eq!(look, ripple_add(&g, &a, &b), "n={}", n);

        let mut expected = vec![Digit::ZERO; n];
        expected.push(Digit::ONE);
        assert_eq!(look, expected, "n={}: expected a full rollover", n);
    }

    // [11;8] + [11;8]: every position generates (11+11 = 22 = 1*12 + 10),
    // and every position above the lowest also receives that carry
    // (22 + 1 = 23 = 1*12 + 11), so the result is 10 then seven 11s then a
    // new top limb.
    let a = vec![Digit::MAX; 8];
    let look = lookahead_add(&g, &a, &a);
    assert_eq!(look, ripple_add(&g, &a, &a));
    let mut expected = vec![Digit::MAX; 8];
    expected[0] = d(10);
    expected.push(Digit::ONE);
    assert_eq!(look.len(), 9);
    assert_eq!(look, expected);

    // No extra limb when the top position neither generates nor propagates.
    let a = vec![Digit::MAX, Digit::MAX, d(3)];
    let b = vec![Digit::ONE];
    let look = lookahead_add(&g, &a, &b);
    assert_eq!(look, ripple_add(&g, &a, &b));
    assert_eq!(look, vec![Digit::ZERO, Digit::ZERO, d(4)]);
    assert_eq!(look.len(), 3);
}

// ---------------------------------------------------------------------
// Generate / propagate signals, as MVL booleans.
// ---------------------------------------------------------------------

#[test]
fn generate_and_propagate_are_mutually_exclusive_144() {
    let g = MvlGates::new();
    for x in all_digits() {
        for y in all_digits() {
            let (gen, prop) = generate_propagate(&g, x, y);
            // Both are MVL booleans: ZERO or MAX, nothing in between.
            assert!(gen == Digit::ZERO || gen == Digit::MAX, "G not boolean");
            assert!(prop == Digit::ZERO || prop == Digit::MAX, "P not boolean");
            // Never both true -- that would need x + y == 23.
            assert!(
                !(gen == Digit::MAX && prop == Digit::MAX),
                "G and P both set at ({:?},{:?})",
                x,
                y
            );
            // G agrees with the half-adder's own carry.
            let (sum, carry) = half_adder(&g, x, y);
            let gen_expected = if carry == Digit::ZERO {
                Digit::ZERO
            } else {
                Digit::MAX
            };
            assert_eq!(gen, gen_expected, "G disagrees with half_adder carry");
            // P is exactly "the half-sum is the top symbol".
            let prop_expected = if sum == Digit::MAX {
                Digit::MAX
            } else {
                Digit::ZERO
            };
            assert_eq!(prop, prop_expected, "P disagrees with lit(11, s)");
        }
    }
}

// ---------------------------------------------------------------------
// (f) The depth model itself, and the published gate counts.
// ---------------------------------------------------------------------

#[test]
fn half_adder_depth_model_is_pinned() {
    let g = MvlGates::new();
    // Derivation under the model in `circuits::depth`:
    //   lit(j, y)                     -> level 1
    //   cycle^j(x)                    -> level j
    //   branch_c = min(lit, thr)      -> level 2
    //   the linear MAX fold over the twelve carry branches adds one level
    //   per term: 3, 4, ..., 14
    //   digitizing with min(flag, ONE) -> 15
    // The sum output tops out one level lower (14), so the circuit depth is
    // the carry's 15. Twelve of those fifteen levels are the linear fold --
    // which is exactly the cost `lookahead_add` cannot remove, since it
    // reuses the same decoder.
    assert_eq!(half_adder_depth(&g, d(7), d(9)), 15);
    // Depth is a property of the circuit, not of the operand values.
    for x in all_digits() {
        for y in all_digits() {
            assert_eq!(half_adder_depth(&g, x, y), 15);
        }
    }
}

// ---------------------------------------------------------------------
// (h) Balanced-tree rebalancing experiment (PLAN.md Section 10bis, part A).
//
// `half_adder_balanced`/`lookahead_add_balanced` are a second, independent
// circuit -- the linear-fold production functions above are untouched, so
// the 151/1212/15 numbers pinned above stay pinned. These tests instead
// pin what the balanced tree actually measures: same results for every
// input, a small gate-count win, and (contrary to the naive "12 levels
// become 4" intuition) *no* depth win for `half_adder` and a one-level
// *loss* for `lookahead_add`. See `half_adder_raw_net_balanced`'s doc
// comment in `circuits/adder.rs` and `docs/01-diseno-sumador.md` for why.
// ---------------------------------------------------------------------

#[test]
fn half_adder_balanced_matches_half_adder_144() {
    let g = MvlGates::new();
    let mut count = 0;
    for x in all_digits() {
        for y in all_digits() {
            assert_eq!(
                half_adder_balanced(&g, x, y),
                half_adder(&g, x, y),
                "half_adder_balanced disagrees with half_adder at ({:?},{:?})",
                x,
                y
            );
            count += 1;
        }
    }
    assert_eq!(count, 144);
}

#[test]
fn half_adder_balanced_gate_count_and_depth_are_pinned() {
    let g = MvlGates::new();
    let _ = half_adder_balanced(&g, d(7), d(9));
    let c = g.cost();
    assert_eq!(
        (c.cycle, c.min, c.max, c.lit, c.thr),
        (66, 25, 22, 24, 12),
        "half_adder_balanced primitive breakdown changed"
    );
    // Two fewer `max` invocations than the linear fold's 151 (one per
    // 12-leaf fold: a balanced tree over 12 leaves needs 11 combines, the
    // zero-seeded linear accumulator needs 12).
    assert_eq!(c.total(), 149, "half_adder_balanced total changed");

    // Depth does NOT improve: the carry fold collapses from 14 to 6 raw
    // levels as expected of a uniform-depth balanced tree, but the sum
    // fold's branches are not uniform (cycle_pow(x,j) is itself j levels
    // deep), so positionally pairing them costs sum a level (14 -> 15) and
    // that becomes the new bottleneck. Net effect: still 15.
    assert_eq!(half_adder_balanced_depth(&g, d(7), d(9)), 15);
    for x in all_digits() {
        for y in all_digits() {
            assert_eq!(half_adder_balanced_depth(&g, x, y), 15);
        }
    }
}

#[test]
fn lookahead_add_balanced_matches_lookahead_add() {
    let g = MvlGates::new();
    for n in 1usize..=16 {
        let (a, b) = worst_case_operands(n);
        assert_eq!(
            lookahead_add_balanced(&g, &a, &b),
            lookahead_add(&g, &a, &b),
            "n={}",
            n
        );
    }
    // Also the exhaustive single-digit case, all 144 pairs.
    let mut count = 0;
    for x in all_digits() {
        for y in all_digits() {
            assert_eq!(
                lookahead_add_balanced(&g, &[x], &[y]),
                lookahead_add(&g, &[x], &[y]),
                "n=1 at ({:?},{:?})",
                x,
                y
            );
            count += 1;
        }
    }
    assert_eq!(count, 144);
}

#[test]
fn lookahead_add_balanced_depth_is_pinned() {
    let g = MvlGates::new();
    // One level worse than the linear-core lookahead at every width: the
    // shared decoder's sum output is a level deeper under the balanced
    // tree (see `half_adder_balanced_gate_count_and_depth_are_pinned`), and
    // that extra level rides through P = lit(11, sum) and the final mux
    // straight to the output.
    for (n, expected) in [(1usize, 18u32), (2, 19), (4, 22), (8, 24), (16, 26)] {
        let (a, b) = worst_case_operands(n);
        assert_eq!(
            lookahead_add_balanced_depth(&g, &a, &b),
            expected,
            "lookahead_add_balanced n={}",
            n
        );
        // Confirm it is exactly one level worse than the production path.
        assert_eq!(
            lookahead_add_balanced_depth(&g, &a, &b),
            lookahead_add_depth(&g, &a, &b) + 1,
            "n={}",
            n
        );
    }
}

#[test]
fn published_gate_counts_are_unchanged() {
    // These two numbers appear in CLAUDE.md and docs/01-diseno-sumador.md.
    // The Phase 8 refactor made every adder generic over `GateNet`; this
    // test is the tripwire proving that generalization cost nothing.
    let g = MvlGates::new();
    let _ = half_adder(&g, d(7), d(9));
    let c = g.cost();
    assert_eq!(
        (c.cycle, c.min, c.max, c.lit, c.thr),
        (66, 25, 24, 24, 12),
        "half_adder primitive breakdown changed"
    );
    assert_eq!(c.total(), 151, "half_adder total changed");

    let g = MvlGates::new();
    let a = vec![d(0), d(0), d(0), d(1)];
    let b = vec![d(11), d(11), d(11), d(0)];
    let _ = ripple_add(&g, &a, &b);
    let c = g.cost();
    assert_eq!(
        (c.cycle, c.min, c.max, c.lit, c.thr),
        (528, 200, 196, 192, 96),
        "ripple_add 4x4 primitive breakdown changed"
    );
    assert_eq!(c.total(), 1212, "ripple_add 4x4 total changed");
}

// ---------------------------------------------------------------------
// (g) Depth comparison. The lookahead must win, and the ripple depth must
//     grow with n.
// ---------------------------------------------------------------------

fn worst_case_operands(n: usize) -> (Vec<Digit>, Vec<Digit>) {
    let a = vec![Digit::MAX; n];
    let mut b = vec![Digit::ZERO; n];
    b[0] = Digit::ONE;
    (a, b)
}

#[test]
fn lookahead_depth_beats_ripple_from_the_first_digit() {
    let g = MvlGates::new();
    let mut previous_ripple = 0u32;
    for n in 1usize..=16 {
        let (a, b) = worst_case_operands(n);
        let rd = ripple_add_depth(&g, &a, &b);
        let ld = lookahead_add_depth(&g, &a, &b);
        assert!(
            ld < rd,
            "n={}: lookahead depth {} did not beat ripple depth {}",
            n,
            ld,
            rd
        );
        assert!(
            rd > previous_ripple,
            "n={}: ripple depth {} is not strictly increasing (previous {})",
            n,
            rd,
            previous_ripple
        );
        previous_ripple = rd;
    }
}

#[test]
fn measured_depths_are_pinned() {
    let g = MvlGates::new();
    // Ripple is affine in n (16n + 14): one full adder per position, each
    // adding a fixed number of levels to the carry chain.
    for (n, expected) in [(1usize, 30u32), (2, 46), (4, 78), (8, 142), (16, 270)] {
        let (a, b) = worst_case_operands(n);
        assert_eq!(ripple_add_depth(&g, &a, &b), expected, "ripple n={}", n);
    }
    // Lookahead is 15 (the shared decoder) + 2 per prefix stage + the mux.
    for (n, expected) in [(1usize, 17u32), (2, 18), (4, 21), (8, 23), (16, 25)] {
        let (a, b) = worst_case_operands(n);
        assert_eq!(lookahead_add_depth(&g, &a, &b), expected, "lookahead n={}", n);
    }
}

// ---------------------------------------------------------------------
// (e) Property tests.
// ---------------------------------------------------------------------

fn arb_digit() -> impl Strategy<Value = Digit> {
    (0u8..12).prop_map(|t| Digit::new(t).unwrap())
}

fn arb_number() -> impl Strategy<Value = Vec<Digit>> {
    prop::collection::vec(arb_digit(), 1..8)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(300))]

    #[test]
    fn lookahead_equals_ripple(a in arb_number(), b in arb_number()) {
        let g = MvlGates::new();
        prop_assert_eq!(lookahead_add(&g, &a, &b), ripple_add(&g, &a, &b));
    }

    #[test]
    fn lookahead_is_commutative(a in arb_number(), b in arb_number()) {
        let g = MvlGates::new();
        prop_assert_eq!(lookahead_add(&g, &a, &b), lookahead_add(&g, &b, &a));
    }

    #[test]
    fn lookahead_depth_never_exceeds_ripple_depth(a in arb_number(), b in arb_number()) {
        let g = MvlGates::new();
        prop_assert!(lookahead_add_depth(&g, &a, &b) < ripple_add_depth(&g, &a, &b));
    }
}
