//! Phase 7: the step-by-step traces must be a *view* of the production
//! circuits, never a second implementation of them.
//!
//! Three properties, each pinned here:
//!   1. differential -- the traced run returns exactly what the untraced
//!      one returns, on every input tried;
//!   2. non-contamination -- taking a trace costs the same `GateCount` as
//!      not taking one (the published counts stay published);
//!   3. reconstruction -- the trace's own contents add up to the result,
//!      so a viewer showing them is showing the real computation.

use dz_gates::circuits::{
    multiply, multiply_trace, ripple_add, ripple_add_trace, StepRecord,
};
use dz_gates::{CrtGates, Digit, DozGates, MvlGates, TableGates};

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

const CASES: [(u32, u32); 8] = [
    (0, 0),
    (1, 1),
    (11, 1),
    (143, 1),
    (1728, 1727),
    (100, 44),
    (12, 12),
    (20735, 20735),
];

#[test]
fn ripple_add_trace_matches_ripple_add_on_every_case() {
    let g = MvlGates::new();
    for (x, y) in CASES {
        let a = digits(x);
        let b = digits(y);
        let plain = ripple_add(&g, &a, &b);
        let (traced, _steps) = ripple_add_trace(&g, &a, &b);
        assert_eq!(plain, traced, "{} + {}", x, y);
    }
}

#[test]
fn taking_an_addition_trace_costs_nothing_extra() {
    for (x, y) in CASES {
        let a = digits(x);
        let b = digits(y);

        let g = MvlGates::new();
        let _ = ripple_add(&g, &a, &b);
        let plain = g.cost();

        let g = MvlGates::new();
        let _ = ripple_add_trace(&g, &a, &b);
        let traced = g.cost();

        assert_eq!(plain, traced, "trace changed the measured cost of {} + {}", x, y);
    }
}

/// The published headline number, pinned directly rather than only via
/// equality with the untraced run: `ripple_add` on two four-digit operands
/// is 1212 gates, and taking a trace must not move it.
#[test]
fn the_published_ripple_add_count_survives_tracing() {
    let a = digits(1728);
    let b = digits(1727);

    let g = MvlGates::new();
    let _ = ripple_add(&g, &a, &b);
    assert_eq!(g.cost().total(), 1212);

    let g = MvlGates::new();
    let _ = ripple_add_trace(&g, &a, &b);
    assert_eq!(g.cost().total(), 1212);
}

#[test]
fn an_addition_trace_has_one_step_per_digit_position_in_order() {
    let g = MvlGates::new();
    let a = digits(1728);
    let b = digits(1727);
    let (_sum, steps) = ripple_add_trace(&g, &a, &b);
    assert_eq!(steps.len(), a.len().max(b.len()));
    for (i, step) in steps.iter().enumerate() {
        assert_eq!(step.position, i, "steps must be least-significant first");
    }
}

/// The carry a step reports going out must be the carry the next step
/// reports coming in, and the first step must start from a zero carry.
/// This is what "watch the carry propagate" means; if it did not hold, the
/// viewer would be animating something that never happened.
#[test]
fn carries_chain_from_one_step_to_the_next() {
    let g = MvlGates::new();
    // 11,11,11,11 + 1: the worst case, the carry travels the whole width.
    let a = vec![Digit::MAX; 4];
    let b = vec![Digit::ONE];
    let (sum, steps) = ripple_add_trace(&g, &a, &b);

    assert_eq!(steps[0].carry_in, Digit::ZERO);
    let mut i = 1usize;
    while i < steps.len() {
        assert_eq!(steps[i].carry_in, steps[i - 1].carry_out, "gap at position {}", i);
        i += 1;
    }
    // Every recorded sum digit is the corresponding digit of the result,
    // and the final carry out is the extra limb.
    let mut k = 0usize;
    while k < steps.len() {
        assert_eq!(sum[k], steps[k].sum, "digit {}", k);
        k += 1;
    }
    assert_eq!(sum.len(), steps.len() + 1);
    assert_eq!(sum[steps.len()], steps[steps.len() - 1].carry_out);
}

#[test]
fn addition_traces_agree_across_all_three_backends() {
    let a = digits(1728);
    let b = digits(1727);
    let reference = ripple_add_trace(&MvlGates::new(), &a, &b);
    assert_eq!(reference, ripple_add_trace(&CrtGates::new(), &a, &b));
    assert_eq!(reference, ripple_add_trace(&TableGates::new(), &a, &b));
}

#[test]
fn multiply_trace_matches_multiply_and_costs_the_same() {
    for (x, y) in CASES {
        let a = digits(x);
        let b = digits(y);

        let g = MvlGates::new();
        let plain = multiply(&g, &a, &b);
        let plain_cost = g.cost();

        let g = MvlGates::new();
        let traced = multiply_trace(&g, &a, &b);
        let traced_cost = g.cost();

        assert_eq!(plain, traced.product, "{} x {}", x, y);
        assert_eq!(plain_cost, traced_cost, "trace changed the cost of {} x {}", x, y);
    }
}

/// The published headline number for a 3x2-digit multiply is 9795 gates.
#[test]
fn the_published_multiply_count_survives_tracing() {
    let a = digits(1000);
    let b = digits(99);

    let g = MvlGates::new();
    let _ = multiply(&g, &a, &b);
    assert_eq!(g.cost().total(), 9795);

    let g = MvlGates::new();
    let _ = multiply_trace(&g, &a, &b);
    assert_eq!(g.cost().total(), 9795);
}

/// End to end: the nested multiplication trace reconstructs the product.
/// The last partial row's accumulator *is* the answer, each row's
/// accumulate steps chain their carries, and there is exactly one row per
/// multiplier digit, shifted by its own index.
#[test]
fn a_multiplication_trace_reconstructs_the_product() {
    let g = MvlGates::new();
    let a = digits(1000);
    let b = digits(99);
    let trace = multiply_trace(&g, &a, &b);

    assert_eq!(trace.rows.len(), b.len());
    for (i, row) in trace.rows.iter().enumerate() {
        assert_eq!(row.index, i);
        assert_eq!(row.multiplier_digit, b[i]);
        // The shift is `index` zero digits prepended, nothing else.
        assert_eq!(row.shifted.len(), row.row.len() + i);
        let mut k = 0usize;
        while k < i {
            assert_eq!(row.shifted[k], Digit::ZERO);
            k += 1;
        }
        assert_eq!(&row.shifted[i..], &row.row[..]);
        assert!(!row.accumulate.is_empty());
        assert_carries_chain(&row.accumulate);
    }
    assert_eq!(
        trace.rows.last().unwrap().accumulated,
        trace.product,
        "the last accumulator is the product"
    );
}

fn assert_carries_chain(steps: &[StepRecord]) {
    assert_eq!(steps[0].carry_in, Digit::ZERO);
    let mut i = 1usize;
    while i < steps.len() {
        assert_eq!(steps[i].carry_in, steps[i - 1].carry_out);
        assert_eq!(steps[i].position, steps[i - 1].position + 1);
        i += 1;
    }
}
