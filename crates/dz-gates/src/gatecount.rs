//! Instrumentation: counts how many times each gate primitive is invoked.
//!
//! Every `DozGates` backend carries a `GateCount` and increments it on every
//! primitive call. This is metadata about circuit cost, not a computation on
//! dozenal values -- summing these counters is ordinary bookkeeping, exempt
//! from the purity restriction (which is about the *value* of digits).

use std::cell::Cell;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GateCount {
    pub cycle: u64,
    pub min: u64,
    pub max: u64,
    pub lit: u64,
    pub thr: u64,
}

impl GateCount {
    pub fn total(&self) -> u64 {
        self.cycle + self.min + self.max + self.lit + self.thr
    }

    pub fn zero() -> GateCount {
        GateCount::default()
    }
}

impl std::ops::Add for GateCount {
    type Output = GateCount;
    fn add(self, rhs: GateCount) -> GateCount {
        GateCount {
            cycle: self.cycle + rhs.cycle,
            min: self.min + rhs.min,
            max: self.max + rhs.max,
            lit: self.lit + rhs.lit,
            thr: self.thr + rhs.thr,
        }
    }
}

/// Interior-mutable counter cell shared by a backend instance.
#[derive(Debug, Default)]
pub struct Counter(Cell<GateCount>);

impl Counter {
    pub fn new() -> Self {
        Counter(Cell::new(GateCount::zero()))
    }
    pub fn bump_cycle(&self) {
        let mut c = self.0.get();
        c.cycle += 1;
        self.0.set(c);
    }
    pub fn bump_min(&self) {
        let mut c = self.0.get();
        c.min += 1;
        self.0.set(c);
    }
    pub fn bump_max(&self) {
        let mut c = self.0.get();
        c.max += 1;
        self.0.set(c);
    }
    pub fn bump_lit(&self) {
        let mut c = self.0.get();
        c.lit += 1;
        self.0.set(c);
    }
    pub fn bump_thr(&self) {
        let mut c = self.0.get();
        c.thr += 1;
        self.0.set(c);
    }
    pub fn snapshot(&self) -> GateCount {
        self.0.get()
    }
    pub fn reset(&self) {
        self.0.set(GateCount::zero());
    }
}
