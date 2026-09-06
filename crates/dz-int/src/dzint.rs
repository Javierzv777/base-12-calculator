//! Layer 1: arbitrary-precision dozenal integers.
//!
//! # One body, two instantiations (and why `Dz` carries a level)
//!
//! Every operation here is written **once**, generic over
//! `dz_gates::circuits::GateNet`, exactly as the circuits in `dz-gates`
//! are. The public methods (`add`, `sub`, `mul`, `div_rem`, `compare`,
//! `is_even`) pick the instantiation at run time:
//!
//! * `Values` -- signals are plain `Digit`s. This is what every caller has
//!   always got, so the published gate counts are untouched.
//! * `Levels` -- signals additionally carry the depth of the gate chain
//!   that produced them. `Levels` invokes the backend exactly once per
//!   primitive, with the same values, so control flow and `GateCount` are
//!   identical; only `u32` bookkeeping rides alongside.
//!
//! The switch is `dz_gates::circuits::depth_tracking()`, turned on for the
//! duration of an evaluation by a `DepthScope` (`dz-api` does this).
//!
//! `Dz` therefore carries a `level`: the depth at which its digits became
//! available. When a `Dz` re-enters a circuit its digits are injected with
//! `GateNet::lift_at` instead of `lift`, so the measured depth of `a * b +
//! c` is the depth of one composed data-flow graph rather than the sum of
//! three unrelated measurements. **This is the whole reason
//! `dz-num`/`dz-sci`/`dz-eval` needed no changes to report depth**: they
//! compose `Dz` operations, and the level rides inside the `Dz` they pass
//! around. The `level` field is private, ignored by `PartialEq` (two
//! integers are equal when they denote the same number, whatever circuit
//! produced them) and invisible in `Debug`.
//!
//! Purity: `level` is `u32` circuit bookkeeping about *shape*, never a
//! dozenal quantity, so native arithmetic on it is the sanctioned kind --
//! the same status `GateCount`'s counters have.

use dz_gates::circuits::{
    depth_tracking, flag_to_bool, ge_net, is_even_digit_net, multiply_net, ones_complement_net,
    record_level, ripple_add_net, ripple_add_net_traced, GateNet, Levels, Values,
};
use dz_gates::{Digit, DozGates};
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sign {
    Zero,
    Pos,
    Neg,
}

/// Arbitrary-precision dozenal integer: sign + little-endian magnitude
/// digits, normalized (no leading `Digit::ZERO` beyond a single digit for
/// the value zero). Internally, subtraction is done via 12's-complement
/// addition (`sub_magnitude_net`, built from
/// `dz_gates::circuits::ones_complement` and `ripple_add`), per PLAN.md
/// Section 3.
#[derive(Clone)]
pub struct Dz {
    sign: Sign,
    magnitude: Vec<Digit>, // little-endian, normalized
    /// Depth at which these digits became available. See the module docs.
    level: u32,
}

impl fmt::Debug for Dz {
    /// Deliberately identical to what `#[derive(Debug)]` printed before
    /// `level` existed: the depth is instrumentation, not part of the
    /// value, and test failure messages should not sprout a field that
    /// means nothing unless a `DepthScope` is active.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dz")
            .field("sign", &self.sign)
            .field("magnitude", &self.magnitude)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DivRemError {
    DivisionByZero,
}

impl fmt::Display for DivRemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DivRemError::DivisionByZero => write!(f, "division by zero"),
        }
    }
}
impl std::error::Error for DivRemError {}

fn normalize(mut digits: Vec<Digit>) -> Vec<Digit> {
    // Trimming trailing (most-significant) zero digits is `Vec` length
    // bookkeeping driven by equality against the ZERO symbol, not
    // arithmetic on a digit's value.
    while digits.len() > 1 && *digits.last().unwrap() == Digit::ZERO {
        digits.pop();
    }
    if digits.is_empty() {
        digits.push(Digit::ZERO);
    }
    digits
}

/// Signal-space counterpart of `normalize`.
fn normalize_net<N: GateNet>(net: &N, mut digits: Vec<N::S>) -> Vec<N::S> {
    while digits.len() > 1 && N::value(*digits.last().unwrap()) == Digit::ZERO {
        digits.pop();
    }
    if digits.is_empty() {
        digits.push(net.lift(Digit::ZERO));
    }
    digits
}

fn is_zero_mag(digits: &[Digit]) -> bool {
    digits.iter().all(|&d| d == Digit::ZERO)
}

fn pad_net<N: GateNet>(net: &N, digits: &[N::S], width: usize) -> Vec<N::S> {
    let mut out = digits.to_vec();
    while out.len() < width {
        out.push(net.lift(Digit::ZERO));
    }
    out
}

/// Inject a `Dz`'s digits into a circuit at the level they arrived from
/// the circuit that produced them (see the module docs on composition).
fn lift_mag<N: GateNet>(net: &N, value: &Dz) -> Vec<N::S> {
    value
        .magnitude
        .iter()
        .map(|&d| net.lift_at(d, value.level))
        .collect()
}

/// Recover plain digits plus the level the deepest of them arrived at, and
/// report that level to the ambient `DepthScope` (a no-op when tracking is
/// off).
fn lower<N: GateNet>(sigs: &[N::S]) -> (Vec<Digit>, u32) {
    let mut digits = Vec::with_capacity(sigs.len());
    let mut level: u32 = 0;
    for &s in sigs {
        digits.push(N::value(s));
        let l = N::level(s);
        if l > level {
            level = l;
        }
    }
    record_level(level);
    (digits, level)
}

/// Compare two normalized-or-padded little-endian magnitudes, most
/// significant digit first, using the gate-composed `ge` comparator plus
/// equality (never native `<`/`>` on a digit tag).
fn compare_magnitude_net<N: GateNet>(net: &N, a: &[N::S], b: &[N::S]) -> Ordering {
    let n = a.len().max(b.len());
    let ap = pad_net(net, a, n);
    let bp = pad_net(net, b, n);
    let mut i = n;
    while i > 0 {
        i -= 1;
        if N::value(ap[i]) != N::value(bp[i]) {
            let flag = ge_net(net, ap[i], bp[i]);
            record_level(N::level(flag));
            return if flag_to_bool(N::value(flag)) {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        }
    }
    Ordering::Equal
}

/// Observer for `sub_magnitude_net`, so the Phase 7 viewer can show what a
/// dozenal subtraction *actually* runs -- a complement followed by an
/// addition -- without a second copy of `sub_magnitude_net` existing.
///
/// Every method has an empty default body; the production path passes
/// `NoSubTrace`, which compiles away.
#[allow(unused_variables)]
pub trait SubTraceSink<S> {
    /// The elevens' complement of `b`, digit by digit (no carry chain).
    fn ones_complement(&mut self, comp: &[S]) {}
    /// One carry step of turning that into the twelve's complement (`+1`).
    #[allow(clippy::too_many_arguments)]
    fn complement_step(&mut self, position: usize, a: S, b: S, carry_in: S, sum: S, carry_out: S) {}
    /// The finished twelve's complement of `b`.
    fn radix_complement(&mut self, rc: &[S]) {}
    /// One carry step of the real addition `a + rc_b`.
    #[allow(clippy::too_many_arguments)]
    fn add_step(&mut self, position: usize, a: S, b: S, carry_in: S, sum: S, carry_out: S) {}
    /// The sum before it is interpreted; `borrowed` is `a < b`.
    fn raw_sum(&mut self, sum: &[S], borrowed: bool) {}
    /// Only when `borrowed`: the elevens' complement taken to decode the
    /// result back out of complement form.
    fn decode_ones_complement(&mut self, comp: &[S]) {}
    /// Only when `borrowed`: one carry step of that decoding's `+1`.
    #[allow(clippy::too_many_arguments)]
    fn decode_step(&mut self, position: usize, a: S, b: S, carry_in: S, sum: S, carry_out: S) {}
}

/// The production sink: records nothing.
pub struct NoSubTrace;

impl<S> SubTraceSink<S> for NoSubTrace {}

/// `a - b` for two non-negative magnitudes, via 12's-complement addition.
/// Returns `(|a-b|, a < b)`.
///
/// Let `n = max(len(a), len(b))`, so both `a` and `b` fit exactly in `n`
/// digits. Compute the *exact* (not truncated) radix complement
/// `rc_b = 12^n - b` -- this is `n` digits when `b > 0`, or `n+1` digits
/// in the edge case `b == 0` (where `12^n - 0 == 12^n` genuinely needs one
/// more digit). Then `sum = a + rc_b = 12^n + (a - b)` exactly, with no
/// wraparound at all (unlike a fixed-width register). Since `0 <= a,b <
/// 12^n`, that sum needs `n+1` digits (top digit exactly `1`) iff `a >= b`,
/// and at most `n` digits iff `a < b` (in which case it's already the
/// complement representation of `b - a`, decoded by complementing once
/// more). Crucially, `rc_b` must *not* be truncated to `n` digits before
/// this addition -- doing so (an earlier bug) discards exactly the
/// information this test depends on.
fn sub_magnitude_net<N: GateNet, T: SubTraceSink<N::S>>(
    net: &N,
    a: &[N::S],
    b: &[N::S],
    sink: &mut T,
) -> (Vec<N::S>, bool) {
    let n = a.len().max(b.len());
    let a_pad = pad_net(net, a, n);
    let b_pad = pad_net(net, b, n);
    let comp1_b = ones_complement_net(net, &b_pad);
    sink.ones_complement(&comp1_b);
    let one = net.lift(Digit::ONE);
    // exact 12^n - b, n or n+1 digits
    let rc_b = ripple_add_net_traced(net, &comp1_b, &[one], |p, x, y, cin, s, cout| {
        sink.complement_step(p, x, y, cin, s, cout);
    });
    sink.radix_complement(&rc_b);
    let sum = normalize_net(
        net,
        ripple_add_net_traced(net, &a_pad, &rc_b, |p, x, y, cin, s, cout| {
            sink.add_step(p, x, y, cin, s, cout);
        }),
    );
    let borrowed = sum.len() <= n;
    sink.raw_sum(&sum, borrowed);
    if !borrowed {
        // a >= b: drop the top "1" digit, the remaining n digits are a - b.
        let mut mag = sum;
        mag.truncate(n);
        (normalize_net(net, mag), false)
    } else {
        // a < b: `sum` (padded to n digits) is the 12's complement of
        // (b - a). Complement it once more (exact, same reasoning) to
        // recover the magnitude b - a.
        let sum_p = pad_net(net, &sum, n);
        let comp1_sum = ones_complement_net(net, &sum_p);
        sink.decode_ones_complement(&comp1_sum);
        let mag = normalize_net(
            net,
            ripple_add_net_traced(net, &comp1_sum, &[one], |p, x, y, cin, s, cout| {
                sink.decode_step(p, x, y, cin, s, cout);
            }),
        );
        (mag, true)
    }
}

/// Entry point for `crate::trace`: run the one `sub_magnitude_net` body on
/// plain digits with a recording sink. Values-only, since a trace is about
/// what a normal (production) subtraction does.
pub(crate) fn sub_magnitude_trace_entry<G: DozGates, T: SubTraceSink<Digit>>(
    g: &G,
    a: &[Digit],
    b: &[Digit],
    sink: &mut T,
) -> (Vec<Digit>, bool) {
    sub_magnitude_net(&Values(g), a, b, sink)
}

fn add_net<N: GateNet>(net: &N, lhs: &Dz, rhs: &Dz) -> Dz {
    match (lhs.sign, rhs.sign) {
        (Sign::Zero, _) => rhs.clone(),
        (_, Sign::Zero) => lhs.clone(),
        (Sign::Pos, Sign::Pos) | (Sign::Neg, Sign::Neg) => {
            let sum = ripple_add_net(net, &lift_mag(net, lhs), &lift_mag(net, rhs));
            let (digits, level) = lower::<N>(&sum);
            Dz::rebuild(lhs.sign, normalize(digits), level)
        }
        (Sign::Pos, Sign::Neg) => {
            // lhs = +A, rhs = -B; sum = A - B, negative iff A < B.
            let (mag, is_negative) = sub_magnitude_net(
                net,
                &lift_mag(net, lhs),
                &lift_mag(net, rhs),
                &mut NoSubTrace,
            );
            let (digits, level) = lower::<N>(&mag);
            Dz::from_signed_magnitude(digits, is_negative, level)
        }
        (Sign::Neg, Sign::Pos) => {
            // lhs = -A, rhs = +B; sum = B - A. sub_magnitude reports
            // is_negative = (B < A), which is exactly when the result
            // B - A is negative -- no extra negation needed.
            let (mag, is_negative) = sub_magnitude_net(
                net,
                &lift_mag(net, rhs),
                &lift_mag(net, lhs),
                &mut NoSubTrace,
            );
            let (digits, level) = lower::<N>(&mag);
            Dz::from_signed_magnitude(digits, is_negative, level)
        }
    }
}

fn mul_net<N: GateNet>(net: &N, lhs: &Dz, rhs: &Dz) -> Dz {
    if lhs.is_zero() || rhs.is_zero() {
        return Dz::zero();
    }
    let product = multiply_net(net, &lift_mag(net, lhs), &lift_mag(net, rhs));
    let (digits, level) = lower::<N>(&product);
    let negative = lhs.sign != rhs.sign;
    Dz::rebuild(
        if negative { Sign::Neg } else { Sign::Pos },
        normalize(digits),
        level,
    )
}

fn compare_net<N: GateNet>(net: &N, lhs: &Dz, rhs: &Dz) -> Ordering {
    use Sign::*;
    match (lhs.sign, rhs.sign) {
        (Zero, Zero) => Ordering::Equal,
        (Zero, Pos) | (Neg, Pos) | (Neg, Zero) => Ordering::Less,
        (Zero, Neg) | (Pos, Neg) | (Pos, Zero) => Ordering::Greater,
        (Pos, Pos) => compare_magnitude_net(net, &lift_mag(net, lhs), &lift_mag(net, rhs)),
        (Neg, Neg) => compare_magnitude_net(net, &lift_mag(net, rhs), &lift_mag(net, lhs)),
    }
}

fn is_even_net<N: GateNet>(net: &N, value: &Dz) -> bool {
    // index 0 = least significant digit
    let lsd = net.lift_at(value.magnitude[0], value.level);
    let flag = is_even_digit_net(net, lsd);
    record_level(N::level(flag));
    flag_to_bool(N::value(flag))
}

/// Restoring long division (schoolbook, base 12).
///
/// Depth note: the trial quotient digit is *selected* by native control
/// flow (the `if` that keeps the largest `j` whose `b*j` still fits), which
/// has been true of this algorithm since Phase 3. The selected digit is
/// therefore injected at the level of the candidate product that justified
/// it, so the chain of comparisons and multiplications that produced the
/// decision is accounted for rather than silently restarting at zero.
fn div_rem_net<N: GateNet>(net: &N, lhs: &Dz, rhs: &Dz) -> Result<(Dz, Dz), DivRemError> {
    if rhs.is_zero() {
        return Err(DivRemError::DivisionByZero);
    }
    if lhs.is_zero() {
        return Ok((Dz::zero(), Dz::zero()));
    }

    let a = lift_mag(net, lhs); // MSD-first iteration built below
    let b = lift_mag(net, rhs);

    if compare_magnitude_net(net, &a, &b) == Ordering::Less {
        // |a| < |b|: quotient 0, remainder is a itself (sign of a).
        return Ok((Dz::zero(), lhs.clone()));
    }

    let zero = net.lift(Digit::ZERO);
    let mut quotient_msd_first: Vec<N::S> = Vec::with_capacity(a.len());
    let mut remainder: Vec<N::S> = vec![zero];

    let mut i = a.len();
    while i > 0 {
        i -= 1;
        let a_i = a[i];
        // remainder = remainder * 12 + a_i
        let mut shifted = vec![zero];
        shifted.extend_from_slice(&remainder);
        remainder = normalize_net(net, ripple_add_net(net, &shifted, &[a_i]));

        // Trial quotient digit: largest q in 0..12 with b*q <= remainder.
        let mut q: u8 = 0;
        let mut best_product: Vec<N::S> = vec![zero];
        let mut j: u8 = 0;
        while j < dz_gates::RADIX {
            let jd = net.lift(Digit::new(j).expect("j in 0..RADIX"));
            let candidate = normalize_net(net, multiply_net(net, &b, &[jd]));
            if compare_magnitude_net(net, &candidate, &remainder) != Ordering::Greater {
                q = j;
                best_product = candidate;
            } else {
                break;
            }
            j += 1;
        }
        let mut decision_level: u32 = 0;
        for &s in best_product.iter() {
            let l = N::level(s);
            if l > decision_level {
                decision_level = l;
            }
        }
        let qd = net.lift_at(Digit::new(q).expect("q in 0..RADIX"), decision_level);
        let (new_rem, _neg) = sub_magnitude_net(net, &remainder, &best_product, &mut NoSubTrace);
        remainder = new_rem;
        quotient_msd_first.push(qd);
    }

    quotient_msd_first.reverse();
    let (quotient_digits, quotient_level) = lower::<N>(&quotient_msd_first);
    let (remainder_digits, remainder_level) = lower::<N>(&remainder);
    let quotient_mag = normalize(quotient_digits);
    let remainder_mag = normalize(remainder_digits);

    let quotient_negative = lhs.sign != rhs.sign;
    let quotient = if is_zero_mag(&quotient_mag) {
        Dz::zero()
    } else {
        Dz::rebuild(
            if quotient_negative { Sign::Neg } else { Sign::Pos },
            quotient_mag,
            quotient_level,
        )
    };
    // Remainder takes the sign of the dividend (truncating division),
    // the conventional choice matching Rust's native integer `%`.
    let remainder = if is_zero_mag(&remainder_mag) {
        Dz::zero()
    } else {
        Dz::rebuild(lhs.sign, remainder_mag, remainder_level)
    };
    Ok((quotient, remainder))
}

impl Dz {
    pub fn zero() -> Dz {
        Dz {
            sign: Sign::Zero,
            magnitude: vec![Digit::ZERO],
            level: 0,
        }
    }

    /// Build from an explicit sign and little-endian magnitude digits
    /// (e.g. produced by the lexer, pushing one `Digit` at a time -- see
    /// `dz-parse`). Normalizes leading zeros and canonicalizes the sign.
    ///
    /// Digits arriving this way are *primary inputs*: they were read from
    /// text or assembled positionally, not produced by a gate, so their
    /// level is zero. A layer that rearranges the digits of an existing
    /// value (rescaling, trimming) should use [`Dz::at_level`] to keep the
    /// depth chain intact -- rewiring costs no gates but does not reset
    /// how deep its inputs already were.
    pub fn from_magnitude(sign_negative: bool, digits: Vec<Digit>) -> Dz {
        let mag = normalize(digits);
        let sign = if is_zero_mag(&mag) {
            Sign::Zero
        } else if sign_negative {
            Sign::Neg
        } else {
            Sign::Pos
        };
        Dz {
            sign,
            magnitude: mag,
            level: 0,
        }
    }

    fn rebuild(sign: Sign, magnitude: Vec<Digit>, level: u32) -> Dz {
        let sign = if is_zero_mag(&magnitude) { Sign::Zero } else { sign };
        Dz { sign, magnitude, level }
    }

    /// The depth at which this value's digits became available. Meaningful
    /// only inside a `dz_gates::circuits::DepthScope`; zero otherwise.
    pub fn level(&self) -> u32 {
        self.level
    }

    /// The same value, declared to arrive at `level`.
    ///
    /// For gate-free rewiring done by the layers above (shifting the
    /// Humphrey point, trimming fractional zeros, splitting a mantissa):
    /// those operations invoke no primitive, so they must not *reset* the
    /// depth of the digits they move around either.
    pub fn at_level(mut self, level: u32) -> Dz {
        self.level = level;
        self
    }

    pub fn sign(&self) -> Sign {
        self.sign
    }

    /// Little-endian magnitude digits (normalized).
    pub fn magnitude_digits(&self) -> &[Digit] {
        &self.magnitude
    }

    pub fn is_zero(&self) -> bool {
        matches!(self.sign, Sign::Zero)
    }

    pub fn negate(&self) -> Dz {
        let sign = match self.sign {
            Sign::Zero => Sign::Zero,
            Sign::Pos => Sign::Neg,
            Sign::Neg => Sign::Pos,
        };
        Dz {
            sign,
            magnitude: self.magnitude.clone(),
            level: self.level,
        }
    }

    pub fn add<G: DozGates>(&self, g: &G, other: &Dz) -> Dz {
        if depth_tracking() {
            add_net(&Levels(g), self, other)
        } else {
            add_net(&Values(g), self, other)
        }
    }

    fn from_signed_magnitude(mag: Vec<Digit>, negative: bool, level: u32) -> Dz {
        let mag = normalize(mag);
        if is_zero_mag(&mag) {
            Dz::zero()
        } else {
            Dz {
                sign: if negative { Sign::Neg } else { Sign::Pos },
                magnitude: mag,
                level,
            }
        }
    }

    pub fn sub<G: DozGates>(&self, g: &G, other: &Dz) -> Dz {
        self.add(g, &other.negate())
    }

    pub fn mul<G: DozGates>(&self, g: &G, other: &Dz) -> Dz {
        if depth_tracking() {
            mul_net(&Levels(g), self, other)
        } else {
            mul_net(&Values(g), self, other)
        }
    }

    /// Compare (ordering), never via native `Ord` on digit tags: sign first,
    /// then gate-composed magnitude comparison.
    pub fn compare<G: DozGates>(&self, g: &G, other: &Dz) -> Ordering {
        if depth_tracking() {
            compare_net(&Levels(g), self, other)
        } else {
            compare_net(&Values(g), self, other)
        }
    }

    pub fn eq_value<G: DozGates>(&self, g: &G, other: &Dz) -> bool {
        self.compare(g, other) == Ordering::Equal
    }

    /// Restoring long division (schoolbook, base 12): returns `(quotient, remainder)`.
    pub fn div_rem<G: DozGates>(&self, g: &G, other: &Dz) -> Result<(Dz, Dz), DivRemError> {
        if depth_tracking() {
            div_rem_net(&Levels(g), self, other)
        } else {
            div_rem_net(&Values(g), self, other)
        }
    }

    /// Whether this integer is even. Since 12 is even, parity is read off
    /// the least-significant digit alone (same as base 10) -- via the
    /// gate-composed `is_even_digit`, never by inspecting the digit's tag.
    pub fn is_even<G: DozGates>(&self, g: &G) -> bool {
        if depth_tracking() {
            is_even_net(&Levels(g), self)
        } else {
            is_even_net(&Values(g), self)
        }
    }
}

/// Convert a small non-negative native `usize` into the `Dz` it denotes, by
/// counting up from zero with `Dz::add` -- i.e. purely through gate-composed
/// addition, never by computing a digit's tag from `k` via `%`/`/`. `k`
/// itself is ordinary loop-counter/length bookkeeping (PLAN.md Section 0),
/// so driving the loop this many times is allowed; what's *not* allowed
/// (and not done here) is deriving a digit's value directly from `k` with
/// native arithmetic.
pub fn small_usize_to_dz<G: DozGates>(g: &G, k: usize) -> Dz {
    let mut acc = Dz::zero();
    let one = Dz::from_magnitude(false, vec![Digit::ONE]);
    let mut i: usize = 0;
    while i < k {
        acc = acc.add(g, &one);
        i += 1;
    }
    acc
}

/// The inverse: recover a native `usize` from a small non-negative `Dz`, by
/// counting up and gate-comparing (`eq_value`) at each step, bounded by
/// `max`. Returns `None` if `n` is negative or exceeds `max`. This is the
/// same "extract a native loop bound from a dozenal value without ever
/// reading a digit's tag" technique used in `dz-eval`'s exponent handling.
pub fn dz_to_small_usize<G: DozGates>(g: &G, n: &Dz, max: usize) -> Option<usize> {
    if n.sign() == Sign::Neg {
        return None;
    }
    let mut probe = Dz::zero();
    let one = Dz::from_magnitude(false, vec![Digit::ONE]);
    let mut i: usize = 0;
    while i <= max {
        if probe.eq_value(g, n) {
            return Some(i);
        }
        probe = probe.add(g, &one);
        i += 1;
    }
    None
}

impl PartialEq for Dz {
    /// Value equality. `level` is instrumentation about the circuit that
    /// produced the value, not part of the value.
    fn eq(&self, other: &Self) -> bool {
        self.sign == other.sign && self.magnitude == other.magnitude
    }
}
impl Eq for Dz {}
