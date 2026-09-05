//! Two single-digit multipliers, both built only from `DozGates`
//! primitives (never a runtime table lookup keyed by an operand's tag),
//! kept side by side so their gate costs can be compared -- that
//! comparison is itself a research result (PLAN.md Section 2 v1/v2).
//!
//! - `digit_mul_v1`: shift-add (repeated addition), the original
//!   implementation.
//! - `digit_mul_v2`: two-level SOP (sum-of-products) decoder over both
//!   operands, 144 branches, `MAX`-multiplexed. See the "is this the table
//!   backend in disguise?" discussion below and in
//!   `docs/01-diseno-sumador.md` -- short answer: no, but the objection has
//!   a grain of truth worth stating plainly.
//!
//! `digit_mul` is an alias for whichever is the default (currently v2,
//! since it is ~30x cheaper -- see `docs/01-diseno-sumador.md` for the
//! measured counts). `multiply`/`scale_by_digit` (the multi-digit
//! entry points used by `dz-int`) use the default.

use crate::circuits::adder::{half_adder, ripple_add};
use crate::digit::{Digit, RADIX};
use crate::traits::DozGates;

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
pub fn digit_mul_v1<G: DozGates>(g: &G, x: Digit, y: Digit) -> (Digit, Digit) {
    let mut lo = Digit::ZERO;
    let mut hi = Digit::ZERO;
    let mut j: u8 = 0;
    while j < RADIX {
        // acc represents x*j in little-endian digits, built by adding x to
        // itself j times. j and k are loop counters, not digit tags.
        let mut acc: Vec<Digit> = vec![Digit::ZERO];
        let mut k: u8 = 0;
        while k < j {
            acc = ripple_add(g, &acc, &[x]);
            k += 1;
        }
        let acc_lo = if !acc.is_empty() {
            acc[0]
        } else {
            Digit::ZERO
        };
        let acc_hi = if acc.len() > 1 { acc[1] } else { Digit::ZERO };

        let sel = g.lit(j, y);
        lo = g.max(lo, g.min(sel, acc_lo));
        hi = g.max(hi, g.min(sel, acc_hi));
        j += 1;
    }
    (lo, hi)
}

/// `LO[j][k] = (j*k) mod 12`, `HI[j][k] = (j*k) div 12` -- the dozenal
/// multiplication table, but here it plays the role of a *circuit
/// specification*, not a runtime data structure. See the module docs and
/// `docs/01-diseno-sumador.md` for why that distinction matters and is
/// argued honestly rather than asserted. `j`, `k` here are loop counters
/// enumerating "every possible digit value", exactly like `RADIX - j` in
/// `circuits::adder` or `RADIX - 1 - j` in `circuits::complement` -- never
/// a `.tag()` read off a live `Digit`.
const fn build_lo_hi_tables() -> [[(u8, u8); RADIX as usize]; RADIX as usize] {
    let mut t = [[(0u8, 0u8); RADIX as usize]; RADIX as usize];
    let mut j: u8 = 0;
    while j < RADIX {
        let mut k: u8 = 0;
        while k < RADIX {
            let prod = j * k;
            t[j as usize][k as usize] = (prod % RADIX, prod / RADIX);
            k += 1;
        }
        j += 1;
    }
    t
}
const LO_HI: [[(u8, u8); RADIX as usize]; RADIX as usize] = build_lo_hi_tables();

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
pub fn digit_mul_v2<G: DozGates>(g: &G, x: Digit, y: Digit) -> (Digit, Digit) {
    let mut lo = Digit::ZERO;
    let mut hi = Digit::ZERO;
    let mut j: u8 = 0;
    while j < RADIX {
        let lit_x = g.lit(j, x);
        let mut k: u8 = 0;
        while k < RADIX {
            let sel = g.min(lit_x, g.lit(k, y));
            let (lo_c, hi_c) = LO_HI[j as usize][k as usize];
            let lo_const = Digit::new(lo_c).expect("table entries are always in-range");
            let hi_const = Digit::new(hi_c).expect("table entries are always in-range");
            lo = g.max(lo, g.min(sel, lo_const));
            hi = g.max(hi, g.min(sel, hi_const));
            k += 1;
        }
        j += 1;
    }
    (lo, hi)
}

/// The default single-digit multiplier used by `multiply`/`scale_by_digit`
/// (and therefore by `dz-int`). Currently v2: see
/// `docs/01-diseno-sumador.md` for the measured ~30x gate-count reduction
/// over v1 that motivates the switch.
pub fn digit_mul<G: DozGates>(g: &G, x: Digit, y: Digit) -> (Digit, Digit) {
    digit_mul_v2(g, x, y)
}

/// Multiply a multi-digit little-endian number by a single digit, returning
/// a little-endian multi-digit result (length `a.len() + 1` at most).
pub fn scale_by_digit<G: DozGates>(g: &G, a: &[Digit], d: Digit) -> Vec<Digit> {
    let mut out = Vec::with_capacity(a.len() + 1);
    let mut carry = Digit::ZERO;
    let mut i: usize = 0;
    while i < a.len() {
        let (lo, hi) = digit_mul(g, a[i], d);
        let (s, c1) = half_adder(g, lo, carry);
        // hi + c1 never overflows a single digit for base-12 schoolbook
        // multiplication (max hi is 10, c1 is 0 or 1); take the sum branch.
        let (carry_out, _c2) = half_adder(g, hi, c1);
        out.push(s);
        carry = carry_out;
        i += 1;
    }
    if carry != Digit::ZERO {
        out.push(carry);
    }
    out
}

/// Shift a little-endian number left by `k` digit positions (multiply by
/// `12^k`), by prepending `k` zero digits. `k` is `usize` bookkeeping, not a
/// digit value.
fn shift_left(a: &[Digit], k: usize) -> Vec<Digit> {
    let mut out = Vec::with_capacity(a.len() + k);
    let mut i = 0;
    while i < k {
        out.push(Digit::ZERO);
        i += 1;
    }
    out.extend_from_slice(a);
    out
}

/// Full shift-add multiplication of two little-endian multi-digit numbers
/// (the "shift-add" here is positional, at the multi-digit level -- the
/// per-digit products come from the default `digit_mul`, v2).
pub fn multiply<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> Vec<Digit> {
    let mut acc: Vec<Digit> = vec![Digit::ZERO];
    let mut k: usize = 0;
    while k < b.len() {
        let row = scale_by_digit(g, a, b[k]);
        let shifted = shift_left(&row, k);
        acc = ripple_add(g, &acc, &shifted);
        k += 1;
    }
    acc
}
