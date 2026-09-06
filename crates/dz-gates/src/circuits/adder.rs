//! Adders built purely by composing `DozGates` primitives.
//!
//! No arithmetic is performed on digit tags in this file. `j` below is a
//! plain `u8` loop counter driving a decoder (as in PLAN.md Section 2's
//! `MAX_j MIN(...)` formulas) and `RADIX - j` is computing that loop
//! counter's complementary threshold parameter -- never a digit's tag.
//!
//! # One circuit, two instantiations
//!
//! Every adder here is written once, generic over `circuits::depth::GateNet`,
//! and exposed twice: through `Values` (signals are plain `Digit`s, so the
//! public entry points cost and behave exactly as they always have) and
//! through `Levels` (signals additionally carry a depth, powering the
//! `*_depth` helpers). There is deliberately no second copy of any circuit:
//! two copies would drift.

use crate::circuits::depth::{max_level, GateNet, Levels, Values};
use crate::digit::{Digit, RADIX};
use crate::traits::DozGates;

/// Apply `cycle` to `x`, `j` times (`j` is a loop counter, not a digit).
fn cycle_pow_net<N: GateNet>(net: &N, x: N::S, j: u8) -> N::S {
    let mut acc = x;
    let mut i: u8 = 0;
    while i < j {
        acc = net.cycle(acc);
        i += 1;
    }
    acc
}

/// The half-adder core, with the carry left as a raw **MVL boolean**
/// (`Digit::ZERO` for false, `Digit::MAX` for true) rather than as the
/// numeric digit 1.
///
/// `s = MAX_{j=0..=11} MIN( lit(j, y), cycle^j(x) )`
/// `c = MAX_{j=0..=11} MIN( lit(j, y), thr(12-j, x) )`
///
/// `half_adder` digitizes that flag; the carry-lookahead network wants it in
/// boolean form (it *is* the generate signal `G`), so both callers share this
/// one body instead of each growing their own copy of the decoder.
///
/// The `MAX` fold here is deliberately linear, not a balanced tree: the
/// published half-adder cost of 151 gates is a measured research result, and
/// rebalancing would silently turn it into 150. Its depth cost (twelve
/// levels) is measured and reported honestly instead.
fn half_adder_raw_net<N: GateNet>(net: &N, x: N::S, y: N::S) -> (N::S, N::S) {
    let mut sum = net.lift(Digit::ZERO);
    let mut carry = net.lift(Digit::ZERO);
    let mut j: u8 = 0;
    while j < RADIX {
        let branch_s = net.min(net.lit(j, y), cycle_pow_net(net, x, j));
        sum = net.max(sum, branch_s);

        let branch_c = net.min(net.lit(j, y), net.thr(RADIX - j, x));
        carry = net.max(carry, branch_c);
        j += 1;
    }
    (sum, carry)
}

fn half_adder_net<N: GateNet>(net: &N, x: N::S, y: N::S) -> (N::S, N::S) {
    let (sum, flag) = half_adder_raw_net(net, x, y);
    // `flag` is an MVL-boolean (`Digit::ZERO` for false, `Digit::MAX` for
    // true) -- `lit`/`thr` both signal "true" as the top symbol, not as the
    // numeric digit 1. A dozenal add carry has *weight* 1 (it is added into
    // the next position), so it must be digitized from the boolean domain
    // into the numeric domain: `MIN(flag, ONE)` yields `ZERO` when flag is
    // `ZERO` and `ONE` when flag is `MAX` (since `min(MAX, ONE) == ONE`).
    // This is the same "MIN as a mux" trick used for `branch_s` above, just
    // gating a constant instead of a variable.
    let carry = net.min(flag, net.lift(Digit::ONE));
    (sum, carry)
}

fn full_adder_net<N: GateNet>(net: &N, x: N::S, y: N::S, cin: N::S) -> (N::S, N::S) {
    let (s1, c1) = half_adder_net(net, x, y);
    let (s2, c2) = half_adder_net(net, s1, cin);
    let cout = net.max(c1, c2);
    (s2, cout)
}

fn ripple_add_net<N: GateNet>(net: &N, a: &[N::S], b: &[N::S]) -> Vec<N::S> {
    let n = if a.len() >= b.len() { a.len() } else { b.len() };
    let zero = net.lift(Digit::ZERO);
    let mut out = Vec::with_capacity(n + 1);
    let mut carry = zero;
    let mut i: usize = 0;
    while i < n {
        let da = if i < a.len() { a[i] } else { zero };
        let db = if i < b.len() { b[i] } else { zero };
        let (s, c) = full_adder_net(net, da, db, carry);
        out.push(s);
        carry = c;
        i += 1;
    }
    // Equality against the ZERO symbol (derived `PartialEq`), not arithmetic
    // on the tag: deciding whether to grow the `Vec` by one limb.
    if N::value(carry) != Digit::ZERO {
        out.push(carry);
    }
    out
}

/// Half-sum plus the two carry-lookahead control signals for one position.
///
/// * `G` ("generate") = "x + y >= 12, regardless of any carry-in". PLAN.md
///   writes this `thr(12-y, x)`, which is shorthand: `thr` takes a
///   *constant* threshold index, not a `Digit`. The real circuit is the
///   decoder the half-adder's carry already uses,
///   `MAX_j MIN( lit(j, y), thr(12-j, x) )`, kept here in raw MVL-boolean
///   form (`ZERO`/`MAX`) because the lookahead network is pure boolean logic.
/// * `P` ("propagate") = "x + y == 11", i.e. a carry-in would push this
///   position to exactly 12 and roll it over: `lit(11, s)` on the half-sum.
///
/// `G` and `P` are mutually exclusive, which is why the lookahead recurrence
/// may use `max` as a plain OR with no ambiguity: `P` holds only when
/// `x + y == 11 < 12`, so `G` is false there; and `G` together with `P` would
/// require `x + y == 23`, impossible for two digits whose largest sum is 22.
fn sum_gen_prop_net<N: GateNet>(net: &N, x: N::S, y: N::S) -> (N::S, N::S, N::S) {
    let (sum, gen) = half_adder_raw_net(net, x, y);
    let prop = net.lit(RADIX - 1, sum);
    (sum, gen, prop)
}

/// Carry-lookahead addition (PLAN.md Phase 8).
///
/// # The recurrence and why it is flattened
///
/// Ripple-carry evaluates `c_{i+1} = G_i OR (P_i AND c_i)` literally, so the
/// carry crawls through one adder per position and depth grows linearly.
/// Expanding the recurrence gives the flat lookahead form
///
/// ```text
/// c_i = G_{i-1} + P_{i-1}G_{i-2} + P_{i-1}P_{i-2}G_{i-3} + ...
/// ```
///
/// which has no sequential dependency at all -- but written naively it
/// recomputes every propagate prefix `P_{i-1}P_{i-2}...` from scratch at
/// every position, and a left `fold` over the terms would put the depth
/// straight back at O(n), defeating the whole exercise.
///
/// The implementation below is therefore the recursive-doubling
/// (Kogge-Stone) evaluation of exactly that flat expansion: it *is* the
/// balanced binary reduction of the `MAX` over terms and of the `MIN` over
/// each propagate prefix, with every prefix shared by all the positions that
/// need it. The block operator
///
/// ```text
/// (G_hi, P_hi) o (G_lo, P_lo) = ( G_hi OR (P_hi AND G_lo),  P_hi AND P_lo )
/// ```
///
/// is associative, so after `ceil(log2 n)` doubling stages `gen[i]` holds the
/// group-generate of the whole block `0..=i`, which *is* the carry out of
/// position `i`. Depth is `O(log n)` stages of two gate levels each, instead
/// of `O(n)` chained full adders.
///
/// # Final sum digit
///
/// The carry stays an MVL boolean the whole way, so the sum digit is not
/// re-added with a second half-adder (which would cost 151 gates and about
/// 27 levels per position). It is a five-gate mux on the half-sum `s_i`:
///
/// ```text
/// out_i = MAX( MIN( lit(0, c_i), s_i ),  MIN( c_i, cycle(s_i) ) )
/// ```
///
/// `lit(0, c)` is the boolean complement of `c` (true exactly when `c` is
/// `ZERO`), so this selects `s_i` when no carry arrives and `cycle(s_i)` --
/// the successor primitive, i.e. "+1" -- when one does. The wrap case
/// `s_i == 11` is already accounted for: that is precisely `P_i`, and the
/// prefix network has already forwarded the carry past this position.
///
/// Observable semantics are identical to `ripple_add`: same `Digit::ZERO`
/// padding of the shorter operand, same output-length rule (one extra limb
/// only when the final carry is non-zero, which is why the final group
/// generate is digitized with `MIN(., ONE)` before that test).
fn lookahead_add_net<N: GateNet>(net: &N, a: &[N::S], b: &[N::S]) -> Vec<N::S> {
    let n = if a.len() >= b.len() { a.len() } else { b.len() };
    if n == 0 {
        return Vec::new();
    }
    let zero = net.lift(Digit::ZERO);

    // Stage 1: per-position half-sum and (G, P) control signals.
    let mut sums = Vec::with_capacity(n);
    let mut gen = Vec::with_capacity(n);
    let mut prop = Vec::with_capacity(n);
    let mut i: usize = 0;
    while i < n {
        let da = if i < a.len() { a[i] } else { zero };
        let db = if i < b.len() { b[i] } else { zero };
        let (s, gi, pi) = sum_gen_prop_net(net, da, db);
        sums.push(s);
        gen.push(gi);
        prop.push(pi);
        i += 1;
    }

    // Stage 2: balanced parallel prefix over the (G, P) block operator.
    // `dist` doubles by self-addition; it is a `Vec` index stride, never a
    // dozenal value.
    let mut dist: usize = 1;
    while dist < n {
        let mut next_gen = Vec::with_capacity(n);
        let mut next_prop = Vec::with_capacity(n);
        let mut i: usize = 0;
        while i < n {
            if i < dist {
                // Nothing further right within reach: block carried forward.
                next_gen.push(gen[i]);
                next_prop.push(prop[i]);
            } else {
                let k = i - dist;
                next_gen.push(net.max(gen[i], net.min(prop[i], gen[k])));
                next_prop.push(net.min(prop[i], prop[k]));
            }
            i += 1;
        }
        gen = next_gen;
        prop = next_prop;
        dist = dist + dist;
    }

    // Stage 3: apply each incoming carry to its half-sum. `c_0` is the
    // constant zero (there is no carry into the least significant position);
    // `c_i` is `gen[i-1]`, the group generate of everything below `i`.
    let mut out = Vec::with_capacity(n + 1);
    let mut i: usize = 0;
    while i < n {
        let cin = if i == 0 { zero } else { gen[i - 1] };
        let not_cin = net.lit(0, cin);
        let unchanged = net.min(not_cin, sums[i]);
        let bumped = net.min(cin, net.cycle(sums[i]));
        out.push(net.max(unchanged, bumped));
        i += 1;
    }

    let carry_out = net.min(gen[n - 1], net.lift(Digit::ONE));
    if N::value(carry_out) != Digit::ZERO {
        out.push(carry_out);
    }
    out
}

fn lift_slice<N: GateNet>(net: &N, xs: &[Digit]) -> Vec<N::S> {
    xs.iter().copied().map(|d| net.lift(d)).collect()
}

/// Half/semi-adder for one dozenal digit pair.
///
/// Returns `(sum_digit, carry_digit)` where carry is `Digit::ZERO` or
/// `Digit::ONE` (never anything else, since 11+11=22=1*12+10 in dozenal).
pub fn half_adder<G: DozGates>(g: &G, x: Digit, y: Digit) -> (Digit, Digit) {
    half_adder_net(&Values(g), x, y)
}

/// Full adder: two half-adders plus a carry-in, carries combined with `max`
/// (safe because a dozenal digit-add carry is always 0 or 1: `x+y` may carry
/// and `(x+y mod 12)+cin` may carry, but never both at once, so MAX is
/// exactly the OR of two booleans encoded as ZERO/ONE digits).
pub fn full_adder<G: DozGates>(g: &G, x: Digit, y: Digit, cin: Digit) -> (Digit, Digit) {
    full_adder_net(&Values(g), x, y, cin)
}

/// Ripple-carry addition of two little-endian multi-digit numbers.
/// Shorter operand is treated as padded with `Digit::ZERO` (this padding
/// decision, and the final length, are `Vec`/`usize` bookkeeping -- not
/// arithmetic on digit values).
pub fn ripple_add<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> Vec<Digit> {
    ripple_add_net(&Values(g), a, b)
}

/// Carry-lookahead addition. Drop-in equivalent of `ripple_add` (identical
/// results and identical output lengths for every input); see
/// `lookahead_add_net` for the circuit and the reasoning.
pub fn lookahead_add<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> Vec<Digit> {
    lookahead_add_net(&Values(g), a, b)
}

/// The generate/propagate pair for one position, as MVL booleans
/// (`Digit::ZERO` / `Digit::MAX`). See `sum_gen_prop_net`.
pub fn generate_propagate<G: DozGates>(g: &G, x: Digit, y: Digit) -> (Digit, Digit) {
    let (_s, gen, prop) = sum_gen_prop_net(&Values(g), x, y);
    (gen, prop)
}

/// Circuit depth of `half_adder` under the model documented in
/// `circuits::depth`.
///
/// Note: this still drives the backend to compute values, so it does bump
/// `g`'s `GateCount`. Take depth and count measurements in separate passes
/// with separate (or `reset_cost`-ed) backend instances.
pub fn half_adder_depth<G: DozGates>(g: &G, x: Digit, y: Digit) -> u32 {
    let net = Levels(g);
    let (s, c) = half_adder_net(&net, net.lift(x), net.lift(y));
    max_level(&[s, c])
}

/// Circuit depth of `ripple_add`. See the note on `half_adder_depth`.
pub fn ripple_add_depth<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> u32 {
    let net = Levels(g);
    let sa = lift_slice(&net, a);
    let sb = lift_slice(&net, b);
    max_level(&ripple_add_net(&net, &sa, &sb))
}

/// Circuit depth of `lookahead_add`. See the note on `half_adder_depth`.
pub fn lookahead_add_depth<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> u32 {
    let net = Levels(g);
    let sa = lift_slice(&net, a);
    let sb = lift_slice(&net, b);
    max_level(&lookahead_add_net(&net, &sa, &sb))
}
