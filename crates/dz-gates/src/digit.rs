//! `Digit`: an opaque symbol of the dozenal alphabet.
//!
//! # Purity contract
//!
//! `Digit` deliberately does **not** implement `Add`, `Sub`, `Mul`, `Div`,
//! `Ord` (ordering by native comparison), `PartialOrd`, or `From<u32>`/`From<u8>`.
//! The wrapped `u8` is a *tag* identifying which of the twelve symbols this
//! value is -- it is not a quantity that native Rust arithmetic may act on.
//!
//! The only code in this whole workspace allowed to read `.tag()` is the
//! gate primitive implementations in `crate::backends::*`, because those
//! implementations *are* the truth-table definition of what each symbol
//! means. Everything above `dz-gates` (adders, multipliers, `dz-int`,
//! `dz-parse`, `dz-eval`, `dz-cli`) may only build values by composing
//! `DozGates` primitives (`cycle`, `min`, `max`, `lit`, `thr`) -- never by
//! touching `.tag()` and doing `+`, `-`, `*`, `/`, `<`, `>` on it.
//!
//! `Digit::new` / `Digit::tag` exist because *something* has to convert
//! between the outside world (a keyboard alphabet, a display glyph table)
//! and the symbol space. That conversion is a table lookup (index into an
//! alphabet array), not arithmetic on a dozenal value, so it is exempted by
//! the purity lint (see `tools/lint-purity.ps1`) -- but only in the specific
//! narrow spots that need it (alphabet mapping in `dz-parse`, loop counters
//! for circuit construction). Nothing may use `.tag()` to compute a sum,
//! difference, product or comparison result directly.

pub const RADIX: u8 = 12;

/// One symbol of the base-12 alphabet, represented as an opaque tag `0..=11`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Digit(u8);

impl Digit {
    /// The symbol representing zero. Used as a starting/identity value; not
    /// the result of any arithmetic.
    pub const ZERO: Digit = Digit(0);
    /// The symbol representing one (successor of zero under `cycle`, but
    /// provided directly since some circuits need a literal "one" constant).
    pub const ONE: Digit = Digit(1);
    /// The highest symbol, `↋` = eleven.
    pub const MAX: Digit = Digit(11);

    /// Construct a digit from a raw tag `0..=11`. Returns `None` out of range.
    ///
    /// This is a *symbol table lookup*, analogous to constructing a `char`
    /// from a codepoint -- it is not computing a dozenal quantity.
    pub const fn new(tag: u8) -> Option<Digit> {
        if tag < RADIX {
            Some(Digit(tag))
        } else {
            None
        }
    }

    /// Extract the raw tag. Reserved for gate primitive implementations and
    /// symbol-table (alphabet <-> digit) code. See module docs.
    pub const fn tag(self) -> u8 {
        self.0
    }

    /// All twelve digits in tag order `0..=11`, for building decoder
    /// circuits (`MAX_j MIN(lit(j, y), ...)`) and for exhaustive tests.
    /// Iterating `0..RADIX` here is a loop counter, explicitly permitted.
    pub fn all() -> [Digit; RADIX as usize] {
        let mut out = [Digit::ZERO; RADIX as usize];
        let mut j: u8 = 0;
        while j < RADIX {
            out[j as usize] = Digit(j);
            j += 1;
        }
        out
    }
}
