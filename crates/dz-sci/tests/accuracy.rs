//! Accuracy checks for the scientific functions, against `f64` as an
//! external oracle.
//!
//! PLAN.md Section 8.3 allows tests -- and only tests -- to reach outside
//! the dozenal world for a reference value, exactly as the integer tests do
//! with `num-bigint`. The production code never converts a dozenal value to
//! a native float; this file does, in one place (`to_f64`), so a series
//! implementation that is subtly wrong fails here instead of silently
//! shipping.

use dz_gates::{Digit, DozGates, MvlGates};
use dz_num::DzFixed;
use dz_sci::{acos, asin, atan, constants, cos, exp, ln, log12, sin, sqrt, tan};

/// Test-only bridge to a native float. Walks the little-endian digits and
/// weights them by powers of twelve.
fn to_f64(v: &DzFixed) -> f64 {
    let mut total = 0.0f64;
    let mut weight = 1.0f64;
    let digits = v.magnitude_digits();
    let mut i = 0usize;
    while i < digits.len() {
        total += (digits[i].tag() as f64) * weight;
        weight *= 12.0;
        i += 1;
    }
    let mut scale = 0usize;
    while scale < v.scale {
        total /= 12.0;
        scale += 1;
    }
    if v.is_negative() {
        -total
    } else {
        total
    }
}

/// Build a `DzFixed` from a small integer, without native arithmetic
/// leaking into the value under test.
fn int<G: DozGates>(g: &G, k: usize) -> DzFixed {
    DzFixed::from_int(dz_int::small_usize_to_dz(g, k))
}

/// The dozenal literal `0;6` (one half) and `0;3` (one quarter).
fn half() -> DzFixed {
    DzFixed::from_digits(false, &[Digit::ZERO], &[Digit::new(6).unwrap()])
}
fn quarter() -> DzFixed {
    DzFixed::from_digits(false, &[Digit::ZERO], &[Digit::new(3).unwrap()])
}

const SCALE: usize = 10;
/// `12^-10` is about `2e-11`; allow a few units in the last place.
const TOLERANCE: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64, what: &str) {
    let difference = (actual - expected).abs();
    assert!(
        difference < TOLERANCE,
        "{}: got {}, expected {}, difference {}",
        what,
        actual,
        expected,
        difference
    );
}

#[test]
fn pi_matches_the_oracle() {
    assert_close(to_f64(&constants::pi(SCALE)), std::f64::consts::PI, "pi");
    assert_close(
        to_f64(&constants::half_pi(SCALE)),
        std::f64::consts::FRAC_PI_2,
        "pi/2",
    );
    assert_close(to_f64(&constants::euler(SCALE)), std::f64::consts::E, "e");
    assert_close(
        to_f64(&constants::ln12(SCALE)),
        12.0f64.ln(),
        "ln 12",
    );
}

#[test]
fn sqrt_matches_the_oracle_and_reports_exactness() {
    let g = MvlGates::new();
    let (two_root, exact) = sqrt(&g, &int(&g, 2), SCALE).unwrap();
    assert!(!exact, "sqrt(2) is irrational, must not be reported exact");
    assert_close(to_f64(&two_root), 2.0f64.sqrt(), "sqrt 2");

    let (nine_root, exact) = sqrt(&g, &int(&g, 9), SCALE).unwrap();
    assert!(exact, "sqrt(9) is exactly 3");
    assert_close(to_f64(&nine_root), 3.0, "sqrt 9");

    let (small, _) = sqrt(&g, &half(), SCALE).unwrap();
    assert_close(to_f64(&small), 0.5f64.sqrt(), "sqrt 1/2");

    let (big, _) = sqrt(&g, &int(&g, 1000), SCALE).unwrap();
    assert_close(to_f64(&big), 1000.0f64.sqrt(), "sqrt 1000");

    assert!(sqrt(&g, &int(&g, 1).neg(), SCALE).is_err());
}

#[test]
fn exp_and_ln_match_the_oracle() {
    let g = MvlGates::new();
    assert_close(to_f64(&exp(&g, &int(&g, 1), SCALE).unwrap()), 1.0f64.exp(), "exp 1");
    assert_close(to_f64(&exp(&g, &half(), SCALE).unwrap()), 0.5f64.exp(), "exp 1/2");
    assert_close(to_f64(&exp(&g, &int(&g, 5), SCALE).unwrap()), 5.0f64.exp(), "exp 5");
    assert_close(
        to_f64(&exp(&g, &int(&g, 3).neg(), SCALE).unwrap()),
        (-3.0f64).exp(),
        "exp -3",
    );

    assert_close(to_f64(&ln(&g, &int(&g, 2), SCALE).unwrap()), 2.0f64.ln(), "ln 2");
    assert_close(to_f64(&ln(&g, &int(&g, 100), SCALE).unwrap()), 100.0f64.ln(), "ln 100");
    assert_close(to_f64(&ln(&g, &half(), SCALE).unwrap()), 0.5f64.ln(), "ln 1/2");
    assert!(ln(&g, &DzFixed::zero(), SCALE).is_err());

    // exp and ln are inverses to within the working precision.
    let round_trip = ln(&g, &exp(&g, &int(&g, 4), SCALE + 2).unwrap(), SCALE).unwrap();
    assert_close(to_f64(&round_trip), 4.0, "ln(exp(4))");
}

#[test]
fn log12_is_exact_on_powers_of_twelve() {
    let g = MvlGates::new();
    // 144 decimal is `100` dozenal: a gross, so its base-12 log is 2.
    let (value, exact) = log12(&g, &int(&g, 144), SCALE).unwrap();
    assert!(exact, "log12 of a power of twelve must be exact");
    assert_close(to_f64(&value), 2.0, "log12 144");

    let (value, exact) = log12(&g, &int(&g, 2), SCALE).unwrap();
    assert!(!exact);
    assert_close(to_f64(&value), 2.0f64.ln() / 12.0f64.ln(), "log12 2");
}

#[test]
fn trig_matches_the_oracle() {
    let g = MvlGates::new();
    assert_close(to_f64(&sin(&g, &int(&g, 1), SCALE).unwrap()), 1.0f64.sin(), "sin 1");
    assert_close(to_f64(&cos(&g, &int(&g, 1), SCALE).unwrap()), 1.0f64.cos(), "cos 1");
    assert_close(to_f64(&tan(&g, &int(&g, 1), SCALE).unwrap()), 1.0f64.tan(), "tan 1");

    // Angle reduction: sin of an angle well past a full turn.
    assert_close(to_f64(&sin(&g, &int(&g, 20), SCALE).unwrap()), 20.0f64.sin(), "sin 20");
    assert_close(
        to_f64(&cos(&g, &int(&g, 7).neg(), SCALE).unwrap()),
        (-7.0f64).cos(),
        "cos -7",
    );

    // sin(pi) is zero to within the working precision.
    let at_pi = sin(&g, &constants::pi(SCALE + 4), SCALE).unwrap();
    assert!(to_f64(&at_pi).abs() < TOLERANCE, "sin(pi) = {}", to_f64(&at_pi));
}

#[test]
fn inverse_trig_matches_the_oracle() {
    let g = MvlGates::new();
    assert_close(to_f64(&atan(&g, &int(&g, 1), SCALE).unwrap()), 1.0f64.atan(), "atan 1");
    assert_close(to_f64(&atan(&g, &int(&g, 5), SCALE).unwrap()), 5.0f64.atan(), "atan 5");
    assert_close(
        to_f64(&atan(&g, &half().neg(), SCALE).unwrap()),
        (-0.5f64).atan(),
        "atan -1/2",
    );
    assert_close(to_f64(&asin(&g, &half(), SCALE).unwrap()), 0.5f64.asin(), "asin 1/2");
    assert_close(
        to_f64(&asin(&g, &int(&g, 1), SCALE).unwrap()),
        std::f64::consts::FRAC_PI_2,
        "asin 1",
    );
    assert_close(to_f64(&acos(&g, &quarter(), SCALE).unwrap()), 0.25f64.acos(), "acos 1/4");
    assert!(asin(&g, &int(&g, 2), SCALE).is_err());
}

#[test]
fn every_backend_agrees() {
    // The functions are backend-agnostic by construction (they only call
    // `DzFixed`), but this is the cheap differential check that nothing
    // slipped a backend-specific assumption in.
    let mvl = MvlGates::new();
    let crt = dz_gates::CrtGates::new();
    let a = sin(&mvl, &int(&mvl, 2), 6).unwrap();
    let b = sin(&crt, &int(&crt, 2), 6).unwrap();
    assert_eq!(a.compare(&mvl, &b), std::cmp::Ordering::Equal);
}
