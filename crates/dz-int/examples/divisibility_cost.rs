//! Measured cost of deciding `d | n`: the last-digit decoder against the
//! restoring division it replaces
//! (`Investigations/12-hallazgo-godel-incompletitud.md`, section 10).
//!
//! Run with `cargo run -p dz-int --release --example divisibility_cost`.
//!
//! Every count is a fresh `MvlGates` between `reset_cost` calls, so the
//! numbers are the circuit alone: no operand reading, no evaluator. The
//! operands are built digit by digit from a fixed pattern with the last
//! digit zero (so `d | n` holds for every `d` below); only `Vec` indices
//! are computed.

use dz_gates::{Digit, DozGates, MvlGates};
use dz_int::{last_digit_divisible, Dz};

const PATTERN: [u8; 11] = [0, 3, 8, 5, 1, 7, 2, 9, 4, 10, 11];

fn number_with_digits(count: usize) -> Dz {
    let mut digits = Vec::with_capacity(count);
    let mut i: usize = 0;
    while i < count {
        let tag = PATTERN[i % PATTERN.len()];
        // Keep the most significant digit nonzero so the length is `count`.
        let tag = if i + 1 == count && tag == 0 { 1 } else { tag };
        digits.push(Digit::new(tag).expect("pattern entries are dozenal digits"));
        i += 1;
    }
    Dz::from_magnitude(false, digits)
}

fn divisor(tag: u8) -> Dz {
    Dz::from_magnitude(false, vec![Digit::new(tag).expect("dozenal digit")])
}

fn twelve() -> Dz {
    Dz::from_magnitude(false, vec![Digit::ZERO, Digit::ONE])
}

fn measure<F: FnOnce(&MvlGates)>(f: F) -> u64 {
    let g = MvlGates::new();
    g.reset_cost();
    f(&g);
    g.cost().total()
}

fn main() {
    println!("digits  d   last-digit  div_rem   ratio(div_rem / last-digit)");
    for &count in &[2usize, 4, 8, 16, 32] {
        let n = number_with_digits(count);
        let cases: [(&str, Dz); 5] = [
            ("2", divisor(2)),
            ("3", divisor(3)),
            ("4", divisor(4)),
            ("6", divisor(6)),
            ("10", twelve()),
        ];
        for (label, d) in cases.iter() {
            let rule = measure(|g| {
                assert_eq!(last_digit_divisible(g, &n, d), Some(true));
            });
            let division = measure(|g| {
                let (_, r) = n.div_rem(g, d).expect("nonzero divisor");
                assert!(r.is_zero());
            });
            println!(
                "{:>6}  {:<2}  {:>10}  {:>8}   {:.0}",
                count,
                label,
                rule,
                division,
                division as f64 / rule as f64
            );
        }
    }
}
