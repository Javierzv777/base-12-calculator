//! Measures gate-invocation counts for representative operations, for
//! `docs/00-fundamentos.md` and `docs/01-diseno-sumador.md`. Run with:
//!   cargo run -p dz-gates --example gate_counts

use dz_gates::circuits::{
    digit_mul_v1, digit_mul_v2, half_adder, half_adder_balanced, half_adder_balanced_depth,
    half_adder_depth, lookahead_add, lookahead_add_balanced, lookahead_add_balanced_depth,
    lookahead_add_depth, multiply, ripple_add, ripple_add_depth,
};
use dz_gates::{CrtGates, Digit, DozGates, MvlGates};

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

fn report(label: &str, g: &impl DozGates) {
    let c = g.cost();
    println!(
        "{:<45} cycle={:<5} min={:<5} max={:<5} lit={:<5} thr={:<5} total={}",
        label, c.cycle, c.min, c.max, c.lit, c.thr, c.total()
    );
}

fn main() {
    // Single-digit half-adder.
    let g = MvlGates::new();
    let _ = half_adder(&g, Digit::new(7).unwrap(), Digit::new(9).unwrap());
    report("half_adder(7,9) [MVL]", &g);

    let g = CrtGates::new();
    let _ = half_adder(&g, Digit::new(7).unwrap(), Digit::new(9).unwrap());
    report("half_adder(7,9) [CRT]", &g);
    println!(
        "  (CRT reconstruction ops for this call: {})",
        g.reconstruction_ops()
    );

    // Single-digit multiply: v1 (shift-add) vs v2 (SOP decoder+mux).
    let g = MvlGates::new();
    let _ = digit_mul_v1(&g, Digit::new(7).unwrap(), Digit::new(9).unwrap());
    report("digit_mul_v1(7,9) shift-add [MVL]", &g);

    let g = MvlGates::new();
    let _ = digit_mul_v2(&g, Digit::new(7).unwrap(), Digit::new(9).unwrap());
    report("digit_mul_v2(7,9) SOP [MVL]", &g);

    // Same comparison at the two extremes (0 and 11*11, the largest
    // product) to check v2's cost is truly constant regardless of operand
    // value, unlike v1's (which depends on y via its repeated-addition loop).
    let g = MvlGates::new();
    let _ = digit_mul_v1(&g, Digit::new(11).unwrap(), Digit::new(11).unwrap());
    report("digit_mul_v1(11,11) shift-add [MVL]", &g);

    let g = MvlGates::new();
    let _ = digit_mul_v2(&g, Digit::new(11).unwrap(), Digit::new(11).unwrap());
    report("digit_mul_v2(11,11) SOP [MVL]", &g);

    let g = MvlGates::new();
    let _ = digit_mul_v1(&g, Digit::new(0).unwrap(), Digit::new(0).unwrap());
    report("digit_mul_v1(0,0) shift-add [MVL]", &g);

    let g = MvlGates::new();
    let _ = digit_mul_v2(&g, Digit::new(0).unwrap(), Digit::new(0).unwrap());
    report("digit_mul_v2(0,0) SOP [MVL]", &g);

    // Multi-digit ripple-add: two 4-digit numbers.
    let g = MvlGates::new();
    let a = digits(1728); // "1000" base12
    let b = digits(1727); // some 4-digit number
    let _ = ripple_add(&g, &a, &b);
    report("ripple_add(4-digit, 4-digit) [MVL]", &g);

    // Multi-digit multiply: 3-digit x 2-digit, using the default digit
    // multiplier (currently v2).
    let g = MvlGates::new();
    let a = digits(1000);
    let b = digits(99);
    let _ = multiply(&g, &a, &b);
    report("multiply(3-digit x 2-digit, default=v2) [MVL]", &g);

    let g = CrtGates::new();
    let a = digits(1000);
    let b = digits(99);
    let _ = multiply(&g, &a, &b);
    report("multiply(3-digit x 2-digit, default=v2) [CRT]", &g);
    println!(
        "  (CRT reconstruction ops for this call: {})",
        g.reconstruction_ops()
    );

    // ---------------------------------------------------------------
    // Phase 8: ripple-carry vs carry-lookahead, gates AND circuit depth.
    //
    // Depth is measured in a *separate pass* with a *fresh, throwaway*
    // backend whose `GateCount` is never read, exactly as `lo_hi_table()`
    // does for its synthesis backend: the depth-instrumented run still
    // drives the gates to compute values, so mixing the two passes would
    // contaminate the published counts above.
    // ---------------------------------------------------------------
    println!();
    println!("Phase 8 -- ripple-carry vs carry-lookahead (MVL backend)");
    println!(
        "{:<6} {:>12} {:>8} {:>12} {:>8}",
        "n", "ripple gts", "depth", "lookahd gts", "depth"
    );
    for n in [1usize, 2, 4, 8, 16] {
        // Worst case for a carry chain: all-eleven plus one, so the carry
        // has to travel the whole width.
        let a = vec![Digit::MAX; n];
        let b = {
            let mut v = vec![Digit::ZERO; n];
            v[0] = Digit::ONE;
            v
        };

        let g = MvlGates::new();
        let _ = ripple_add(&g, &a, &b);
        let ripple_gates = g.cost().total();

        let g = MvlGates::new();
        let _ = lookahead_add(&g, &a, &b);
        let look_gates = g.cost().total();

        let g = MvlGates::new();
        let ripple_depth = ripple_add_depth(&g, &a, &b);
        let g = MvlGates::new();
        let look_depth = lookahead_add_depth(&g, &a, &b);

        println!(
            "{:<6} {:>12} {:>8} {:>12} {:>8}",
            n, ripple_gates, ripple_depth, look_gates, look_depth
        );
    }

    let g = MvlGates::new();
    println!(
        "half_adder depth (linear MAX fold, 12 levels): {}",
        half_adder_depth(&g, Digit::new(7).unwrap(), Digit::new(9).unwrap())
    );

    // ---------------------------------------------------------------
    // Rebalancing experiment (PLAN.md Section 10bis, part A): what happens
    // if the shared decoder's 12-leaf MAX fold is a balanced binary tree
    // instead of the linear fold above. See `half_adder_raw_net_balanced`
    // in `circuits/adder.rs` for why this is not simply "150 gates, 4
    // levels" -- the sum fold's branches are not uniform depth to begin
    // with (`cycle_pow(x,j)` is itself `j` levels deep).
    // ---------------------------------------------------------------
    println!();
    println!("Rebalancing experiment -- linear fold vs balanced tree (MVL backend)");

    let g = MvlGates::new();
    let _ = half_adder(&g, Digit::new(7).unwrap(), Digit::new(9).unwrap());
    let linear_gates = g.cost().total();
    let g = MvlGates::new();
    let linear_depth = half_adder_depth(&g, Digit::new(7).unwrap(), Digit::new(9).unwrap());

    let g = MvlGates::new();
    let _ = half_adder_balanced(&g, Digit::new(7).unwrap(), Digit::new(9).unwrap());
    let balanced_gates = g.cost().total();
    let g = MvlGates::new();
    let balanced_depth = half_adder_balanced_depth(&g, Digit::new(7).unwrap(), Digit::new(9).unwrap());

    println!(
        "half_adder(7,9)   gates: linear={:<6} balanced={:<6} | depth: linear={:<3} balanced={}",
        linear_gates, balanced_gates, linear_depth, balanced_depth
    );

    println!(
        "{:<6} {:>12} {:>10} {:>8} {:>10}",
        "n", "look gts (L)", "(B)", "depth (L)", "(B)"
    );
    for n in [1usize, 2, 4, 8, 16] {
        let a = vec![Digit::MAX; n];
        let b = {
            let mut v = vec![Digit::ZERO; n];
            v[0] = Digit::ONE;
            v
        };

        let g = MvlGates::new();
        let _ = lookahead_add(&g, &a, &b);
        let look_gates = g.cost().total();

        let g = MvlGates::new();
        let _ = lookahead_add_balanced(&g, &a, &b);
        let look_gates_bal = g.cost().total();

        let g = MvlGates::new();
        let look_depth = lookahead_add_depth(&g, &a, &b);
        let g = MvlGates::new();
        let look_depth_bal = lookahead_add_balanced_depth(&g, &a, &b);

        println!(
            "{:<6} {:>12} {:>10} {:>8} {:>10}",
            n, look_gates, look_gates_bal, look_depth, look_depth_bal
        );
    }
}
