//! Measures gate-invocation counts for representative operations, for
//! `docs/00-fundamentos.md` and `docs/01-diseno-sumador.md`. Run with:
//!   cargo run -p dz-gates --example gate_counts

use dz_gates::circuits::{digit_mul, half_adder, multiply, ripple_add};
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

    // Single-digit multiply.
    let g = MvlGates::new();
    let _ = digit_mul(&g, Digit::new(7).unwrap(), Digit::new(9).unwrap());
    report("digit_mul(7,9) [MVL]", &g);

    // Multi-digit ripple-add: two 4-digit numbers.
    let g = MvlGates::new();
    let a = digits(1728); // "1000" base12
    let b = digits(1727); // "0ЕЕЕ"-ish, just some 4-digit number
    let _ = ripple_add(&g, &a, &b);
    report("ripple_add(4-digit, 4-digit) [MVL]", &g);

    // Multi-digit multiply: 3-digit x 2-digit.
    let g = MvlGates::new();
    let a = digits(1000);
    let b = digits(99);
    let _ = multiply(&g, &a, &b);
    report("multiply(3-digit x 2-digit, shift-add) [MVL]", &g);

    let g = CrtGates::new();
    let a = digits(1000);
    let b = digits(99);
    let _ = multiply(&g, &a, &b);
    report("multiply(3-digit x 2-digit, shift-add) [CRT]", &g);
    println!(
        "  (CRT reconstruction ops for this call: {})",
        g.reconstruction_ops()
    );
}
