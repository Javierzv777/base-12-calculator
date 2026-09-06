//! `DynGates` -- a `Sized` adapter around `&dyn DozGates`.
//!
//! Every arithmetic API in the stack is generic over `G: DozGates` (a
//! monomorphised backend, so the gate calls inline). A *registry* of
//! calculator functions, on the other hand, has to be a
//! `Vec<Box<dyn DozFunction>>`, which forces its methods to take the
//! backend as a trait object. `DynGates` bridges the two: it is a plain
//! sized struct that forwards each of the five primitives (plus the
//! instrumentation hooks) to the borrowed trait object, so
//! `DynGates(g)` can be handed to any `G: DozGates` API.
//!
//! It adds no logic and no gates of its own: every method is a one-line
//! delegation, and the counters it reports are the borrowed backend's own.

use crate::digit::Digit;
use crate::gatecount::GateCount;
use crate::traits::DozGates;

pub struct DynGates<'a>(pub &'a dyn DozGates);

impl DozGates for DynGates<'_> {
    fn cycle(&self, x: Digit) -> Digit {
        self.0.cycle(x)
    }
    fn min(&self, x: Digit, y: Digit) -> Digit {
        self.0.min(x, y)
    }
    fn max(&self, x: Digit, y: Digit) -> Digit {
        self.0.max(x, y)
    }
    fn lit(&self, j: u8, x: Digit) -> Digit {
        self.0.lit(j, x)
    }
    fn thr(&self, j: u8, x: Digit) -> Digit {
        self.0.thr(j, x)
    }
    fn cost(&self) -> GateCount {
        self.0.cost()
    }
    fn reset_cost(&self) {
        self.0.reset_cost()
    }
    fn name(&self) -> &'static str {
        self.0.name()
    }
}
