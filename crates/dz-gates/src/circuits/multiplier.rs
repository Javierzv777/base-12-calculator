//! Two single-digit multipliers, both built only from `DozGates`
//! primitives (never a runtime table lookup keyed by an operand's tag),
//! kept side by side so their gate costs can be compared -- that
//! comparison is itself a research result (PLAN.md Section 2 v1/v2).
//!
//! - `digit_mul_v1`: shift-add (repeated addition), the original
//!   implementation.
//! - `digit_mul_v2`: two-level SOP (sum-of-products) decoder over both
//!   operands, 144 branches, `MAX`-multiplexed. Its constant table
//!   (`lo_hi_table`) is synthesized by calling `digit_mul_v1` -- gate
//!   composed, not computed with native `*`/`%`/`/` -- see the doc comment
//!   on `lo_hi_table` and the "is this the table backend in disguise?"
//!   discussion in `docs/01-diseno-sumador.md` for the full history (an
//!   earlier version of this file did compute the table with native `j*k`,
//!   which was identified and fixed).
//!
//! `digit_mul` is an alias for whichever is the default (currently v2,
//! since it is ~30x cheaper -- see `docs/01-diseno-sumador.md` for the
//! measured counts). `multiply`/`scale_by_digit` (the multi-digit
//! entry points used by `dz-int`) use the default.
//!
//! # One circuit, two instantiations
//!
//! As in `circuits::adder`, every circuit here has a single body generic
//! over `circuits::depth::GateNet`. The public `digit_mul*`/`multiply`/
//! `scale_by_digit` entry points are the `Values` instantiation, so their
//! published gate counts are unchanged; the `*_net` forms let a caller run
//! the same circuit under `Levels` (depth) or feed a trace sink
//! (`circuits::trace`) without a second copy of the circuit existing.

use crate::backends::MvlGates;
use crate::circuits::adder::{half_adder_net, ripple_add_net, ripple_add_net_traced};
use crate::circuits::depth::{GateNet, Values};
use crate::digit::{Digit, RADIX};
use crate::traits::DozGates;
use std::sync::OnceLock;

/// v1: single-digit multiply via repeated addition, decoded over all
/// possible values of `y`. For every candidate value `j` of `y` (`j` is a
/// `u8` loop counter, `0..12`, never a digit tag), compute what `x * j`
/// would be by *repeated addition* (`j` applications of `ripple_add`,
/// again counted by the loop variable, not by inspecting a digit), then
/// `MAX`-select the branch gated by `lit(j, y)`. This is "sumas
/// desplazadas" (shift-add) from PLAN.md Section 2 v1.
///
/// Returns `(low_digit, high_digit)` such that `x * y = high_digit * 12 +
/// low_digit`.
pub fn digit_mul_v1_net<N: GateNet>(net: &N, x: N::S, y: N::S) -> (N::S, N::S) {
    let zero = net.lift(Digit::ZERO);
    let mut lo = zero;
    let mut hi = zero;
    let mut j: u8 = 0;
    while j < RADIX {
        // acc represents x*j in little-endian digits, built by adding x to
        // itself j times. j and k are loop counters, not digit tags.
        let mut acc: Vec<N::S> = vec![zero];
        let mut k: u8 = 0;
        while k < j {
            acc = ripple_add_net(net, &acc, &[x]);
            k += 1;
        }
        let acc_lo = if !acc.is_empty() { acc[0] } else { zero };
        let acc_hi = if acc.len() > 1 { acc[1] } else { zero };

        let sel = net.lit(j, y);
        lo = net.max(lo, net.min(sel, acc_lo));
        hi = net.max(hi, net.min(sel, acc_hi));
        j += 1;
    }
    (lo, hi)
}

/// v1 shift-add single-digit multiply. See `digit_mul_v1_net`.
pub fn digit_mul_v1<G: DozGates>(g: &G, x: Digit, y: Digit) -> (Digit, Digit) {
    digit_mul_v1_net(&Values(g), x, y)
}

/// `LO[j][k]`/`HI[j][k]` -- the dozenal multiplication table, playing the
/// role of a *circuit specification* for the SOP decoder below, not a
/// runtime data structure (see the module docs and
/// `docs/01-diseno-sumador.md` for the "is this the table backend in
/// disguise?" discussion this feeds).
///
/// This table is **synthesized by calling `digit_mul_v1`** (already
/// gate-composed and already checked equivalent to the table oracle on all
/// 144 combinations) for every `(j, k)` pair, once, and memoized. No `*`,
/// `%`, or `/` is applied to a digit's value anywhere in this
/// initialization -- an earlier version of this file computed `j * k`
/// directly with native `u8` arithmetic to fill this table, which was
/// exactly the thing the whole project exists to avoid: deriving the
/// *meaning* of multiplication natively instead of from gates. `j`/`k`
/// here are still just loop counters (`0..12`, enumerating "every possible
/// digit value" to build `Digit::new(j)`/`Digit::new(k)` test operands --
/// the same pattern `Digit::all()` uses), but the *product* itself now
/// comes from `digit_mul_v1(init_gates, Digit::new(j), Digit::new(k))`,
/// not from computing `j * k`.
///
/// The backend used to synthesize the table is fixed to `MvlGates`, not
/// generic over the caller's `G`. This is a deliberate, documented choice:
/// multiplication is one mathematical function, and the differential tests
/// (`digit_mul_v1_v2_and_oracle_agree_144_all_backends`) already establish
/// that all three backends compute the identical `(lo, hi)` for every one
/// of the 144 pairs -- so which backend synthesizes the specification
/// cannot change what the specification *is*. A disposable `MvlGates`
/// instance is created solely for this synthesis and discarded; its
/// `GateCount` is never read, so the (one-time, memoized) cost of building
/// this table never contaminates the gate count measured for a caller's
/// own `digit_mul_v2` call (verified by `examples/gate_counts.rs` still
/// reporting exactly 876 per call).
fn lo_hi_table() -> &'static Vec<Vec<(Digit, Digit)>> {
    static LO_HI: OnceLock<Vec<Vec<(Digit, Digit)>>> = OnceLock::new();
    LO_HI.get_or_init(|| {
        let init_gates = MvlGates::new();
        let mut table = Vec::with_capacity(RADIX as usize);
        let mut j: u8 = 0;
        while j < RADIX {
            let xj = Digit::new(j).expect("j is in 0..RADIX");
            let mut row = Vec::with_capacity(RADIX as usize);
            let mut k: u8 = 0;
            while k < RADIX {
                let yk = Digit::new(k).expect("k is in 0..RADIX");
                row.push(digit_mul_v1(&init_gates, xj, yk));
                k += 1;
            }
            table.push(row);
            j += 1;
        }
        table
    })
}

/// v2: two-level sum-of-products (SOP) decoder over both operands.
///
/// `lo = MAX_{j,k} MIN( lit(j,x), lit(k,y), LO[j][k] )`
/// `hi = MAX_{j,k} MIN( lit(j,x), lit(k,y), HI[j][k] )`
///
/// For every one of the 144 `(j, k)` pairs: `MIN(lit(j,x), lit(k,y))` is
/// the MVL-AND of "x is exactly j" and "y is exactly k" (both booleans
/// encoded as `ZERO`/`MAX`); `MIN(that, CONST)` mux-selects the
/// precomputed constant `LO[j][k]`/`HI[j][k]` when the branch matches,
/// `ZERO` otherwise (the same "MIN as mux" trick used throughout this
/// crate); `MAX` across all 144 branches picks out the (at most one)
/// matching one. This is a real decoder+mux circuit of constant depth: at
/// runtime it never reads `x.tag()`/`y.tag()` to index anything -- it
/// decodes `x` and `y` into 24 boolean signals (12 `lit(j,x)` + 12
/// `lit(k,y)`) and combines them combinationally.
pub fn digit_mul_v2_net<N: GateNet>(net: &N, x: N::S, y: N::S) -> (N::S, N::S) {
    let table = lo_hi_table();
    let mut lo = net.lift(Digit::ZERO);
    let mut hi = net.lift(Digit::ZERO);
    let mut j: u8 = 0;
    while j < RADIX {
        let lit_x = net.lit(j, x);
        let mut k: u8 = 0;
        while k < RADIX {
            let sel = net.min(lit_x, net.lit(k, y));
            let (lo_const, hi_const) = table[j as usize][k as usize];
            lo = net.max(lo, net.min(sel, net.lift(lo_const)));
            hi = net.max(hi, net.min(sel, net.lift(hi_const)));
            k += 1;
        }
        j += 1;
    }
    (lo, hi)
}

/// v2 SOP single-digit multiply. See `digit_mul_v2_net`.
pub fn digit_mul_v2<G: DozGates>(g: &G, x: Digit, y: Digit) -> (Digit, Digit) {
    digit_mul_v2_net(&Values(g), x, y)
}

/// The default single-digit multiplier used by `multiply`/`scale_by_digit`
/// (and therefore by `dz-int`). Currently v2: see
/// `docs/01-diseno-sumador.md` for the measured ~30x gate-count reduction
/// over v1 that motivates the switch.
pub fn digit_mul_net<N: GateNet>(net: &N, x: N::S, y: N::S) -> (N::S, N::S) {
    digit_mul_v2_net(net, x, y)
}

/// The default single-digit multiplier. See `digit_mul_net`.
pub fn digit_mul<G: DozGates>(g: &G, x: Digit, y: Digit) -> (Digit, Digit) {
    digit_mul_v2(g, x, y)
}

/// Multiply a multi-digit little-endian number by a single digit, returning
/// a little-endian multi-digit result (length `a.len() + 1` at most).
pub fn scale_by_digit_net<N: GateNet>(net: &N, a: &[N::S], d: N::S) -> Vec<N::S> {
    let mut out = Vec::with_capacity(a.len() + 1);
    let mut carry = net.lift(Digit::ZERO);
    let mut i: usize = 0;
    while i < a.len() {
        let (lo, hi) = digit_mul_net(net, a[i], d);
        let (s, c1) = half_adder_net(net, lo, carry);
        // hi + c1 never overflows a single digit for base-12 schoolbook
        // multiplication (max hi is 10, c1 is 0 or 1); take the sum branch.
        let (carry_out, _c2) = half_adder_net(net, hi, c1);
        out.push(s);
        carry = carry_out;
        i += 1;
    }
    if N::value(carry) != Digit::ZERO {
        out.push(carry);
    }
    out
}

/// Multiply a multi-digit little-endian number by a single digit. See
/// `scale_by_digit_net`.
pub fn scale_by_digit<G: DozGates>(g: &G, a: &[Digit], d: Digit) -> Vec<Digit> {
    scale_by_digit_net(&Values(g), a, d)
}

/// Shift a little-endian number left by `k` digit positions (multiply by
/// `12^k`), by prepending `k` zero digits. `k` is `usize` bookkeeping, not a
/// digit value.
fn shift_left_net<N: GateNet>(net: &N, a: &[N::S], k: usize) -> Vec<N::S> {
    let zero = net.lift(Digit::ZERO);
    let mut out = Vec::with_capacity(a.len() + k);
    let mut i = 0;
    while i < k {
        out.push(zero);
        i += 1;
    }
    out.extend_from_slice(a);
    out
}

/// Full shift-add multiplication of two little-endian multi-digit numbers
/// (the "shift-add" here is positional, at the multi-digit level -- the
/// per-digit products come from the default `digit_mul`, v2).
///
/// `sink` observes the structure a step-by-step viewer needs: for each
/// digit of `b`, the partial row that digit produced and its positional
/// shift, then every carry step of accumulating that row into the running
/// total, then the new accumulator. See `circuits::trace::multiply_trace`.
///
/// The sink is a trait rather than a closure so the production
/// instantiation (`NoMulTrace`) has empty bodies that compile away
/// entirely: `multiply` allocates nothing extra and invokes no extra gate,
/// so the published `multiply` counts are untouched.
pub fn multiply_net_traced<N: GateNet, T: MulTraceSink<N::S>>(
    net: &N,
    a: &[N::S],
    b: &[N::S],
    sink: &mut T,
) -> Vec<N::S> {
    let mut acc: Vec<N::S> = vec![net.lift(Digit::ZERO)];
    let mut k: usize = 0;
    while k < b.len() {
        let row = scale_by_digit_net(net, a, b[k]);
        let shifted = shift_left_net(net, &row, k);
        sink.row(k, b[k], &row, &shifted);
        acc = ripple_add_net_traced(net, &acc, &shifted, |position, x, y, cin, s, cout| {
            sink.accumulate_step(k, position, x, y, cin, s, cout);
        });
        sink.row_done(k, &acc);
        k += 1;
    }
    acc
}

/// Observer for `multiply_net_traced`. Every method has an empty default
/// body, so an implementor writes only the hooks it cares about and the
/// production path (`NoMulTrace`) costs nothing.
#[allow(unused_variables)]
pub trait MulTraceSink<S> {
    /// A partial row was produced for multiplier digit `index`.
    fn row(&mut self, index: usize, multiplier_digit: S, row: &[S], shifted: &[S]) {}
    /// One digit position of accumulating row `index` into the running total.
    #[allow(clippy::too_many_arguments)]
    fn accumulate_step(
        &mut self,
        index: usize,
        position: usize,
        a: S,
        b: S,
        carry_in: S,
        sum: S,
        carry_out: S,
    ) {
    }
    /// Row `index` has been accumulated; `acc` is the new running total.
    fn row_done(&mut self, index: usize, acc: &[S]) {}
}

/// The production sink: records nothing.
pub struct NoMulTrace;

impl<S> MulTraceSink<S> for NoMulTrace {}

/// Full shift-add multiplication, generic over the signal type.
pub fn multiply_net<N: GateNet>(net: &N, a: &[N::S], b: &[N::S]) -> Vec<N::S> {
    multiply_net_traced(net, a, b, &mut NoMulTrace)
}

/// Full shift-add multiplication of two little-endian multi-digit numbers.
/// See `multiply_net_traced`.
pub fn multiply<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> Vec<Digit> {
    multiply_net(&Values(g), a, b)
}
