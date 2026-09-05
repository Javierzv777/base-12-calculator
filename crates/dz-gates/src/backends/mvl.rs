//! Backend A: direct 12-valued (Post algebra) truth tables.
//!
//! This module is the one sanctioned place in the entire workspace where a
//! dozenal symbol's tag is read and compared/added as a plain `u8`. That is
//! not a purity violation: this code *is* the definition of what `cycle`,
//! `min`, `max`, `lit`, `thr` mean. A truth table has to be written down
//! somewhere as the ground truth; everything built on top of `DozGates`
//! (adders, multipliers, `dz-int`, the parser, the evaluator) never does
//! this -- it only calls these five named gates. `tools/lint-purity.ps1`
//! excludes `backends/*.rs` from the "no raw arithmetic on digit tags" scan
//! for exactly this reason, and nowhere else.

use crate::digit::{Digit, RADIX};
use crate::gatecount::{Counter, GateCount};
use crate::traits::DozGates;

/// Precomputed successor table: `CYCLE[x] = (x + 1) mod 12`.
const fn build_cycle_table() -> [u8; RADIX as usize] {
    let mut t = [0u8; RADIX as usize];
    let mut x: u8 = 0;
    while x < RADIX {
        t[x as usize] = (x + 1) % RADIX;
        x += 1;
    }
    t
}
const CYCLE_TABLE: [u8; RADIX as usize] = build_cycle_table();

#[derive(Debug, Default)]
pub struct MvlGates {
    counter: Counter,
}

impl MvlGates {
    pub fn new() -> Self {
        MvlGates {
            counter: Counter::new(),
        }
    }
}

impl DozGates for MvlGates {
    fn cycle(&self, x: Digit) -> Digit {
        self.counter.bump_cycle();
        Digit::new(CYCLE_TABLE[x.tag() as usize]).expect("table entries are always in-range")
    }

    fn min(&self, x: Digit, y: Digit) -> Digit {
        self.counter.bump_min();
        if x.tag() <= y.tag() {
            x
        } else {
            y
        }
    }

    fn max(&self, x: Digit, y: Digit) -> Digit {
        self.counter.bump_max();
        if x.tag() >= y.tag() {
            x
        } else {
            y
        }
    }

    fn lit(&self, j: u8, x: Digit) -> Digit {
        self.counter.bump_lit();
        if x.tag() == j {
            Digit::MAX
        } else {
            Digit::ZERO
        }
    }

    fn thr(&self, j: u8, x: Digit) -> Digit {
        self.counter.bump_thr();
        if x.tag() >= j {
            Digit::MAX
        } else {
            Digit::ZERO
        }
    }

    fn cost(&self) -> GateCount {
        self.counter.snapshot()
    }

    fn reset_cost(&self) {
        self.counter.reset();
    }

    fn name(&self) -> &'static str {
        "mvl"
    }
}
