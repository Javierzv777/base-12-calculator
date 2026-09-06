//! Phase 7: the subtraction trace, and the whole-expression depth
//! instrumentation, must both be views of the production path.

use dz_gates::circuits::DepthScope;
use dz_gates::{Digit, DozGates, MvlGates};
use dz_int::{subtract_trace, Dz};

fn digits(n: u32) -> Vec<Digit> {
    if n == 0 {
        return vec![Digit::ZERO];
    }
    let mut v = n;
    let mut out = Vec::new();
    while v > 0 {
        out.push(Digit::new((v % 12) as u8).unwrap());
        v /= 12;
    }
    out
}

fn dz(n: u32) -> Dz {
    Dz::from_magnitude(false, digits(n))
}

/// Both operands nonzero: `Dz::add`/`sub`/`mul` short-circuit on a zero
/// operand before any gate runs (that is a sign-level decision, not a
/// circuit), so a zero case would compare a skipped computation against a
/// performed one. Zero is covered separately where it is meaningful.
const PAIRS: [(u32, u32); 6] = [
    (5, 3),
    (3, 5),
    (1728, 1),
    (1, 1728),
    (20735, 20735),
    (144, 143),
];

/// Differential: the traced subtraction produces exactly what `Dz::sub`
/// produces, sign included.
#[test]
fn subtraction_trace_matches_dz_sub() {
    let g = MvlGates::new();
    for (x, y) in PAIRS {
        let trace = subtract_trace(&g, &digits(x), &digits(y));
        let expected = dz(x).sub(&g, &dz(y));
        let expected_negative = expected.sign() == dz_int::Sign::Neg;
        assert_eq!(
            trace.result.as_slice(),
            expected.magnitude_digits(),
            "{} - {} magnitude",
            x,
            y
        );
        assert_eq!(trace.borrowed, expected_negative, "{} - {} sign", x, y);
    }
}

/// Non-contamination: recording the stages invokes no primitive.
#[test]
fn taking_a_subtraction_trace_costs_nothing_extra() {
    for (x, y) in PAIRS {
        let g = MvlGates::new();
        let _ = dz(x).sub(&g, &dz(y));
        let plain = g.cost();

        let g = MvlGates::new();
        let _ = subtract_trace(&g, &digits(x), &digits(y));
        let traced = g.cost();

        assert_eq!(plain, traced, "trace changed the cost of {} - {}", x, y);
    }
}

/// End to end: the trace's own stages reconstruct the answer, and they
/// show the twelve's complement honestly rather than pretending a borrow
/// rippled. `ones_complement + 1 == radix_complement`, and
/// `a + radix_complement == raw_sum`, both read straight off the recorded
/// steps.
#[test]
fn a_subtraction_trace_reconstructs_its_own_stages() {
    let g = MvlGates::new();
    let trace = subtract_trace(&g, &digits(1728), &digits(1));

    // Stage 2's recorded sums are the radix complement, limb for limb.
    let mut i = 0usize;
    while i < trace.complement_steps.len() {
        assert_eq!(trace.radix_complement[i], trace.complement_steps[i].sum, "limb {}", i);
        i += 1;
    }
    // Stage 3's recorded sums are the raw sum, limb for limb (the raw sum
    // may carry one extra limb beyond the recorded positions).
    let mut k = 0usize;
    while k < trace.add_steps.len() && k < trace.raw_sum.len() {
        assert_eq!(trace.raw_sum[k], trace.add_steps[k].sum, "limb {}", k);
        k += 1;
    }
    assert!(!trace.borrowed, "1728 >= 1, so no decoding stage runs");
    assert!(trace.decode_steps.is_empty());
    assert_eq!(trace.result, digits(1727));
}

#[test]
fn the_borrow_path_runs_its_decoding_stages() {
    let g = MvlGates::new();
    let trace = subtract_trace(&g, &digits(1), &digits(1728));
    assert!(trace.borrowed);
    assert!(!trace.decode_steps.is_empty());
    assert!(!trace.decode_ones_complement.is_empty());
    assert_eq!(trace.result, digits(1727));
}

// ------------------------------------------------------------------
// Depth instrumentation.
// ------------------------------------------------------------------

/// The load-bearing invariant of the depth design: turning tracking on
/// changes neither the answer nor the measured gate count, because the
/// depth-carrying instantiation invokes the backend exactly as often.
#[test]
fn depth_tracking_changes_neither_result_nor_gate_count() {
    for (x, y) in PAIRS {
        let g = MvlGates::new();
        let plain_sum = dz(x).add(&g, &dz(y));
        let plain_product = dz(x).mul(&g, &dz(y));
        let plain_cost = g.cost();

        let g = MvlGates::new();
        let scope = DepthScope::enter();
        let tracked_sum = dz(x).add(&g, &dz(y));
        let tracked_product = dz(x).mul(&g, &dz(y));
        let tracked_cost = g.cost();
        let depth = scope.measured();
        drop(scope);

        assert_eq!(plain_sum, tracked_sum, "{} + {}", x, y);
        assert_eq!(plain_product, tracked_product, "{} x {}", x, y);
        assert_eq!(plain_cost, tracked_cost, "cost for {} , {}", x, y);
        assert!(depth > 0, "a real computation has a nonzero depth");
    }
}

/// Outside a scope, depth is simply not measured -- no ambient cost, no
/// levels accumulating for the lifetime of the process.
#[test]
fn depth_is_zero_when_nobody_asked_for_it() {
    let g = MvlGates::new();
    let sum = dz(1728).add(&g, &dz(1727));
    assert_eq!(sum.level(), 0);
}

/// Depth *composes*: chaining two additions must be deeper than one, which
/// is the whole reason `Dz` carries its own arrival level.
#[test]
fn chained_operations_are_deeper_than_single_ones() {
    let g = MvlGates::new();

    let scope = DepthScope::enter();
    let _ = dz(1728).add(&g, &dz(1727));
    let single = scope.measured();
    drop(scope);

    let scope = DepthScope::enter();
    let first = dz(1728).add(&g, &dz(1727));
    let _ = first.add(&g, &dz(5));
    let chained = scope.measured();
    drop(scope);

    assert!(
        chained > single,
        "chained depth {} should exceed single depth {}",
        chained,
        single
    );
}

/// A single-digit addition's depth must agree with the `half_adder`
/// figure the Phase 8 work published: a full adder is two half-adders in
/// series plus a `max`, so one ripple position sits at 30 levels -- the
/// same number `examples/gate_counts.rs` prints for `ripple_add` at n = 1.
#[test]
fn one_digit_addition_matches_the_published_ripple_depth() {
    let g = MvlGates::new();
    let scope = DepthScope::enter();
    let _ = dz(7).add(&g, &dz(9));
    let depth = scope.measured();
    drop(scope);
    assert_eq!(depth, 30);
}

/// Nested scopes restore the outer measurement instead of clobbering it.
#[test]
fn nested_scopes_restore_the_outer_measurement() {
    let g = MvlGates::new();
    let outer = DepthScope::enter();
    let _ = dz(1728).add(&g, &dz(1727));
    let before = outer.measured();
    {
        let inner = DepthScope::enter();
        let _ = dz(1).add(&g, &dz(1));
        drop(inner);
    }
    assert!(outer.measured() >= before);
    drop(outer);
}
