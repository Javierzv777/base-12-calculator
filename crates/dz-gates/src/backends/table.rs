//! Backend C: flat lookup-table oracle.
//!
//! Per PLAN.md Section 2, this is *not* a production backend: it exists so
//! tests have a third, structurally independent implementation to
//! differential-test MVL and CRT against, and so the composite adder/
//! multiplier built in `crate::circuits` can be checked against a full
//! 144-entry digit-add/digit-multiply oracle built directly (not via the
//! ripple/shift-add circuits).
//!
//! Like `backends::mvl` and `backends::crt`, this module is a sanctioned
//! exception to the raw-arithmetic-on-tags rule: building an oracle table
//! *is* native arithmetic by definition, that's the point of an oracle.
//! `oracle_add`/`oracle_mul` are used exclusively from test code.

use crate::digit::{Digit, RADIX};
use crate::gatecount::{Counter, GateCount};
use crate::traits::DozGates;

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

const fn build_min_table() -> [[u8; RADIX as usize]; RADIX as usize] {
    let mut t = [[0u8; RADIX as usize]; RADIX as usize];
    let mut x: u8 = 0;
    while x < RADIX {
        let mut y: u8 = 0;
        while y < RADIX {
            t[x as usize][y as usize] = if x <= y { x } else { y };
            y += 1;
        }
        x += 1;
    }
    t
}
const MIN_TABLE: [[u8; RADIX as usize]; RADIX as usize] = build_min_table();

const fn build_max_table() -> [[u8; RADIX as usize]; RADIX as usize] {
    let mut t = [[0u8; RADIX as usize]; RADIX as usize];
    let mut x: u8 = 0;
    while x < RADIX {
        let mut y: u8 = 0;
        while y < RADIX {
            t[x as usize][y as usize] = if x >= y { x } else { y };
            y += 1;
        }
        x += 1;
    }
    t
}
const MAX_TABLE: [[u8; RADIX as usize]; RADIX as usize] = build_max_table();

/// `ADD_TABLE[x][y] = (sum_digit, carry)` for the 144 combinations, the
/// full oracle used by differential tests against the composite adder.
const fn build_add_table() -> [[(u8, u8); RADIX as usize]; RADIX as usize] {
    let mut t = [[(0u8, 0u8); RADIX as usize]; RADIX as usize];
    let mut x: u8 = 0;
    while x < RADIX {
        let mut y: u8 = 0;
        while y < RADIX {
            let sum = x + y;
            t[x as usize][y as usize] = (sum % RADIX, sum / RADIX);
            y += 1;
        }
        x += 1;
    }
    t
}
const ADD_TABLE: [[(u8, u8); RADIX as usize]; RADIX as usize] = build_add_table();

/// `MUL_TABLE[x][y] = (low_digit, high_digit)` for the 144 combinations.
const fn build_mul_table() -> [[(u8, u8); RADIX as usize]; RADIX as usize] {
    let mut t = [[(0u8, 0u8); RADIX as usize]; RADIX as usize];
    let mut x: u8 = 0;
    while x < RADIX {
        let mut y: u8 = 0;
        while y < RADIX {
            let prod = x * y;
            t[x as usize][y as usize] = (prod % RADIX, prod / RADIX);
            y += 1;
        }
        x += 1;
    }
    t
}
const MUL_TABLE: [[(u8, u8); RADIX as usize]; RADIX as usize] = build_mul_table();

/// Full-oracle single-digit addition: `x + y = carry*12 + sum`.
/// **Test-only.** Never call from production code paths.
pub fn oracle_add(x: Digit, y: Digit) -> (Digit, Digit) {
    let (s, c) = ADD_TABLE[x.tag() as usize][y.tag() as usize];
    (
        Digit::new(s).expect("table in range"),
        Digit::new(c).expect("carry is 0 or 1, valid digit"),
    )
}

/// Full-oracle single-digit multiplication: `x * y = high*12 + low`.
/// **Test-only.** Never call from production code paths.
pub fn oracle_mul(x: Digit, y: Digit) -> (Digit, Digit) {
    let (lo, hi) = MUL_TABLE[x.tag() as usize][y.tag() as usize];
    (
        Digit::new(lo).expect("table in range"),
        Digit::new(hi).expect("max product 11*11=121=a1_12, high digit < 12"),
    )
}

#[derive(Debug, Default)]
pub struct TableGates {
    counter: Counter,
}

impl TableGates {
    pub fn new() -> Self {
        TableGates::default()
    }
}

impl DozGates for TableGates {
    fn cycle(&self, x: Digit) -> Digit {
        self.counter.bump_cycle();
        Digit::new(CYCLE_TABLE[x.tag() as usize]).expect("table in range")
    }

    fn min(&self, x: Digit, y: Digit) -> Digit {
        self.counter.bump_min();
        Digit::new(MIN_TABLE[x.tag() as usize][y.tag() as usize]).expect("table in range")
    }

    fn max(&self, x: Digit, y: Digit) -> Digit {
        self.counter.bump_max();
        Digit::new(MAX_TABLE[x.tag() as usize][y.tag() as usize]).expect("table in range")
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
        "table"
    }
}
