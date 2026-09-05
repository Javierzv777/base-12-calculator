//! Dozenal alphabet: `0123456789↊↋` (U+218A DEK, U+218B EL) with ASCII
//! aliases `X`/`x` (ten) and `E`/`e` (eleven), per PLAN.md Section 5.
//!
//! Converting between a `char` and a `Digit` is a symbol-table lookup, the
//! sanctioned exception documented in `dz_gates::digit`: it is how the
//! outside world's alphabet maps onto the opaque tag space, not arithmetic
//! on a dozenal quantity. `digit_to_char` indexes a fixed display table by
//! `d.tag()`; `char_to_digit` is a `match`, not a numeric parse.

use dz_gates::Digit;

/// The canonical display glyphs, indexed by tag `0..=11`.
const GLYPHS: [char; 12] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '\u{218A}', '\u{218B}'];

/// The Humphrey point: the dozenal fractional separator.
pub const POINT: char = ';';

pub fn digit_to_char(d: Digit) -> char {
    GLYPHS[d.tag() as usize]
}

/// Parse one character into a `Digit`, accepting the canonical glyphs and
/// the configurable ASCII aliases (`X`/`x`/`T`/`t` for ten, `E`/`e` for
/// eleven -- both the Dozenal Society of Great Britain's `X`/`E` and the
/// alternate `T`/`E` convention mentioned in PLAN.md Section 5 are accepted
/// on input; output always uses the canonical Unicode glyphs).
pub fn char_to_digit(c: char) -> Option<Digit> {
    match c {
        '0'..='9' => Digit::new((c as u32 - '0' as u32) as u8),
        '\u{218A}' | 'X' | 'x' | 'T' | 't' => Digit::new(10),
        '\u{218B}' | 'E' | 'e' => Digit::new(11),
        _ => None,
    }
}

pub fn is_digit_char(c: char) -> bool {
    char_to_digit(c).is_some()
}
