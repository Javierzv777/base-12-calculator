//! Transcendental constants, synthesized from gate-composed arithmetic and
//! memoized per requested scale.
//!
//! # Why a throwaway backend, and why memoizing is not cheating
//!
//! These constants are *circuit outputs*, not literals: `pi` is computed
//! from Machin's formula with the same `DzFixed` operations everything
//! else uses, and `ln12`/`euler` from the series in `crate::elementary`.
//! Nothing is transcribed from a decimal table.
//!
//! They are computed once per scale on a throwaway `MvlGates` whose
//! `GateCount` is never read, and cached. That is exactly the pattern
//! `dz_gates::circuits::multiplier::lo_hi_table` established for the SOP
//! multiplier's constant table, for the same two reasons: the user's gate
//! measurement should not be polluted by one-off constant synthesis, and
//! it should not depend on whether this call happens to be the first one.
//! Caching a *result* is fine; what the project forbids -- and what is not
//! happening here -- is importing the *meaning* of an operation from
//! native arithmetic (see CLAUDE.md, "cachear resultados si; importar
//! significado de aritmetica nativa no").

use crate::elementary::{exp, ln};
use crate::util::{div, one, round_to_scale, small_int};
use crate::{SciError, GUARD_DIGITS};
use dz_gates::{DozGates, MvlGates};
use dz_num::DzFixed;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

const MAX_TERMS: usize = 400;

fn cache() -> &'static Mutex<HashMap<(&'static str, usize), DzFixed>> {
    static CACHE: OnceLock<Mutex<HashMap<(&'static str, usize), DzFixed>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn memoized(name: &'static str, scale: usize, compute: impl FnOnce() -> DzFixed) -> DzFixed {
    if let Some(hit) = cache()
        .lock()
        .expect("constant cache mutex is never poisoned")
        .get(&(name, scale))
    {
        return hit.clone();
    }
    let value = compute();
    cache()
        .lock()
        .expect("constant cache mutex is never poisoned")
        .insert((name, scale), value.clone());
    value
}

/// `atan(1/n)` for a small integer `n`, by the Maclaurin series
/// `1/n - 1/(3n^3) + 1/(5n^5) - ...`.
///
/// Each step divides the running power by `n^2` and then by the next odd
/// integer, so no power is ever formed explicitly and every quantity stays
/// small. Both the odd divisor and `n^2` are dozenal values advanced by
/// gate-composed addition/multiplication.
fn atan_reciprocal<G: DozGates>(g: &G, n: usize, gs: usize) -> Result<DzFixed, SciError> {
    let unit = one();
    let two = unit.add(g, &unit);
    let n_value = small_int(g, n);
    let n_squared = n_value.mul(g, &n_value);

    let mut power = div(g, &unit, &n_value, gs)?;
    let mut sum = power.clone();
    let mut denominator = unit.clone(); // steps to 3, 5, 7, ...
    let mut subtract = true;
    let mut steps = 0usize;
    loop {
        power = div(g, &power, &n_squared, gs)?;
        denominator = denominator.add(g, &two);
        if power.is_zero() {
            break;
        }
        let piece = div(g, &power, &denominator, gs)?;
        if piece.is_zero() {
            break;
        }
        sum = if subtract {
            sum.sub(g, &piece)
        } else {
            sum.add(g, &piece)
        };
        subtract = !subtract;
        steps += 1;
        if steps > MAX_TERMS {
            break;
        }
    }
    Ok(sum)
}

/// Pi to `scale` fractional dozenal digits, via Machin's formula
/// `pi = 16*atan(1/5) - 4*atan(1/239)`.
///
/// Machin's identity is used rather than the plain Leibniz series because
/// the two arctangents converge geometrically (by `1/25` and `1/57121` per
/// term), which is what makes a few dozen dozenal digits affordable when
/// every multiplication is a gate circuit.
pub fn pi(scale: usize) -> DzFixed {
    memoized("pi", scale, || {
        let g = MvlGates::new();
        let gs = scale + GUARD_DIGITS;
        let first = atan_reciprocal(&g, 5, gs).expect("no division by zero in Machin's formula");
        let second =
            atan_reciprocal(&g, 239, gs).expect("no division by zero in Machin's formula");
        let sixteen = small_int(&g, 16);
        let four = small_int(&g, 4);
        let value = first.mul(&g, &sixteen).sub(&g, &second.mul(&g, &four));
        round_to_scale(&g, &value, scale)
    })
}

/// `2*pi`.
pub fn two_pi(scale: usize) -> DzFixed {
    memoized("two_pi", scale, || {
        let g = MvlGates::new();
        let p = pi(scale + GUARD_DIGITS);
        round_to_scale(&g, &p.add(&g, &p), scale)
    })
}

/// `pi/2`.
pub fn half_pi(scale: usize) -> DzFixed {
    memoized("half_pi", scale, || {
        let g = MvlGates::new();
        let p = pi(scale + GUARD_DIGITS);
        round_to_scale(&g, &p.mul(&g, &crate::util::half()), scale)
    })
}

/// The natural logarithm of twelve -- the constant that turns the free,
/// exact radix split of a dozenal number (`x = m * 12^e`) into a
/// logarithm, and therefore the one constant this base needs that base ten
/// would spell `ln 10`.
pub fn ln12(scale: usize) -> DzFixed {
    memoized("ln12", scale, || {
        let g = MvlGates::new();
        let gs = scale + GUARD_DIGITS;
        // Asking `ln` for twelve directly would recurse: its radix split
        // sends any argument of twelve or more back here for this very
        // constant. Splitting twelve into two factors that are both inside
        // `[1,12)` breaks the cycle -- each one takes the series path with
        // a zero exponent, so neither needs a constant at all.
        let two = small_int(&g, 2);
        let six = small_int(&g, 6);
        let ln_two = ln(&g, &two, gs).expect("2 is positive");
        let ln_six = ln(&g, &six, gs).expect("6 is positive");
        round_to_scale(&g, &ln_two.add(&g, &ln_six), scale)
    })
}

/// Euler's number, as `e^1`.
///
/// It is spelled `euler` and not `e` in the expression language for a
/// reason that is specific to this project: `e` is the ASCII alias for the
/// digit eleven (`dz_parse::alphabet`), so a bare `e` is a *number* here,
/// not a name.
pub fn euler(scale: usize) -> DzFixed {
    memoized("euler", scale, || {
        let g = MvlGates::new();
        let gs = scale + GUARD_DIGITS;
        let value = exp(&g, &one(), gs).expect("exp(1) is in range");
        round_to_scale(&g, &value, scale)
    })
}
