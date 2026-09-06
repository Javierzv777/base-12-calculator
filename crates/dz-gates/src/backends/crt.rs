//! Backend B: Chinese Remainder Theorem decomposition, `Z12 ≅ Z4 × Z3`.
//!
//! A digit's tag is treated internally as a pair `(q, t)` with `q = x mod 4`
//! (`Z4`) and `t = x mod 3` (`Z3`). Like `backends::mvl`, this module is a
//! sanctioned exception to the "no raw arithmetic on digit tags" rule: it
//! *is* the definition of the CRT backend under investigation, so it must
//! read and combine tags with native `%`/`+` to do so. Nothing outside
//! `dz-gates::backends` is allowed to do this.
//!
//! ## Why min/max/thr are expensive here
//!
//! Addition and multiplication decompose cleanly component-wise (`Z4` and
//! `Z3` are independent rings), which is the whole appeal of CRT. But the
//! *order* on `Z12` is not a product order on `(Z4, Z3)` -- there is no way
//! to know whether `x >= j` by looking at `q` and `t` separately without a
//! reconstruction step. So `min`, `max`, `lit`, `thr` (and therefore
//! `cycle`, which is a successor test near the wraparound) all pay for a
//! full CRT->tag reconstruction (`reconstruct`, a 12-entry lookup) before
//! they can compare anything. This module counts those reconstructions
//! (`recon_ops`) separately from the five headline gate counts so the cost
//! can be quantified; see `docs/02-crt-vs-mvl.md` for the measured numbers
//! and conclusion.

use crate::digit::{Digit, RADIX};
use crate::gatecount::{Counter, GateCount};
use crate::traits::DozGates;
use std::cell::Cell;

/// Decompose a tag into its `(mod 4, mod 3)` residues.
const fn decompose(x: u8) -> (u8, u8) {
    (x % 4, x % 3)
}

/// Reconstruction table: `RECON[q][t] = the unique x in 0..12 with x%4==q, x%3==t`.
const fn build_recon_table() -> [[u8; 3]; 4] {
    let mut table = [[0u8; 3]; 4];
    let mut x: u8 = 0;
    while x < RADIX {
        let (q, t) = decompose(x);
        table[q as usize][t as usize] = x;
        x += 1;
    }
    table
}
const RECON: [[u8; 3]; 4] = build_recon_table();

fn reconstruct(q: u8, t: u8) -> u8 {
    RECON[q as usize][t as usize]
}

#[derive(Debug, Default)]
pub struct CrtGates {
    counter: Counter,
    /// Extra instrumentation: number of full CRT->tag reconstructions
    /// performed. Not part of `GateCount` (which stays backend-agnostic at
    /// the 5-primitive level) but exposed through
    /// `DozGates::reconstruction_ops`, which is what carries it up to
    /// `dz-api` and from there to both interfaces.
    recon_ops: Cell<u64>,
}

impl CrtGates {
    pub fn new() -> Self {
        CrtGates::default()
    }

    fn bump_recon(&self) {
        self.recon_ops.set(self.recon_ops.get() + 1);
    }
}

impl DozGates for CrtGates {
    fn cycle(&self, x: Digit) -> Digit {
        self.counter.bump_cycle();
        // Component-wise successor: cheap, no reconstruction needed for the
        // *component* update, but we still need a tag to hand back to the
        // caller (Digit is always stored/compared by tag at the API
        // boundary), so we reconstruct once.
        let (q, t) = decompose(x.tag());
        let q2 = (q + 1) % 4;
        let t2 = (t + 1) % 3;
        self.bump_recon();
        Digit::new(reconstruct(q2, t2)).expect("reconstruction table is total")
    }

    fn min(&self, x: Digit, y: Digit) -> Digit {
        self.counter.bump_min();
        // Order is not a product order on (Z4, Z3): must reconstruct both
        // tags before native comparison is even meaningful.
        self.bump_recon();
        self.bump_recon();
        if x.tag() <= y.tag() {
            x
        } else {
            y
        }
    }

    fn max(&self, x: Digit, y: Digit) -> Digit {
        self.counter.bump_max();
        self.bump_recon();
        self.bump_recon();
        if x.tag() >= y.tag() {
            x
        } else {
            y
        }
    }

    fn lit(&self, j: u8, x: Digit) -> Digit {
        self.counter.bump_lit();
        // Equality *is* expressible component-wise: x == j iff both residues
        // match. No reconstruction of x needed, only of the answer (MAX/ZERO,
        // which are constants, not reconstructed from x).
        let (qx, tx) = decompose(x.tag());
        let (qj, tj) = decompose(j);
        if qx == qj && tx == tj {
            Digit::MAX
        } else {
            Digit::ZERO
        }
    }

    fn thr(&self, j: u8, x: Digit) -> Digit {
        self.counter.bump_thr();
        // >= is fundamentally an order predicate: reconstruct.
        self.bump_recon();
        if x.tag() >= j {
            Digit::MAX
        } else {
            Digit::ZERO
        }
    }

    fn cost(&self) -> GateCount {
        self.counter.snapshot()
    }

    /// The CRT-only overhead, as `DozGates::reconstruction_ops` promises:
    /// one per order comparison rebuilt from residues.
    fn reconstruction_ops(&self) -> u64 {
        self.recon_ops.get()
    }

    fn reset_cost(&self) {
        self.counter.reset();
        self.recon_ops.set(0);
    }

    fn name(&self) -> &'static str {
        "crt"
    }
}
