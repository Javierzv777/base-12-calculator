//! Measures gate-invocation counts for representative operations, for
//! `docs/00-fundamentos.md` and `docs/01-diseno-sumador.md`. Run with:
//!   cargo run -p dz-gates --example gate_counts

use dz_gates::circuits::{
    digit_mul_v1, digit_mul_v2, half_adder, half_adder_depth, lookahead_add, lookahead_add_depth,
    multiply, ripple_add, ripple_add_depth,
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
}
