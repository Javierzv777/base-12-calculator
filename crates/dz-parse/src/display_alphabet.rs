//! The **presentation** alphabet: which two characters the user sees and
//! types for ten and eleven (`docs/07-alfabeto-configurable.md`).
//!
//! This is not the dozenal alphabet. `alphabet::GLYPHS` and
//! `alphabet::char_to_digit` are, and neither is touched here: a
//! `DisplayAlphabet` translates *text* into canonical text just before the
//! lexer runs, and canonical text back into user text just for display.
//! Nothing below `dz-parse` ever sees a configured symbol, so no test,
//! circuit or gate count anywhere else changes because of one.
//!
//! ## Why this lives here and not one layer up (or in JavaScript)
//!
//! Translating is *not* character substitution. With the pair `a`/`b` a
//! naive `replace` turns `abs(3)` into `↊↋s(3)` and destroys the function
//! call. The lexer already knows the right rule -- a maximal run of ASCII
//! letters is an identifier if and only if one of its letters is not a
//! digit alias -- so the translation applies **that same rule**, through
//! the very same function (`lexer::is_identifier_run_with`), with the
//! configured pair as the alias set. `ab` is the number ten-eleven; `abs`
//! is still a name.
//!
//! That rule is dozenal semantics, so it may not be re-derived in the
//! frontend (`CLAUDE.md`: "el frontend nunca decide semántica dozenal")
//! nor copied into `dz-api`. It belongs next to its owner, the lexer.

use crate::alphabet::POINT;
use crate::lexer::{is_identifier_run, is_identifier_run_with, letter_run};

/// U+218A DEK, the canonical ten.
pub const CANONICAL_TEN: char = '\u{218A}';
/// U+218B EL, the canonical eleven.
pub const CANONICAL_ELEVEN: char = '\u{218B}';

/// Whether a letter run that the *configured* alphabet reads as a name is
/// rejected when the canonical lexer would still read it as a number.
///
/// `true` is what `docs/07` section 3.3 asks for ("limitar"): with the
/// pair `a`/`b`, typing `E` is a syntax error rather than a silent eleven,
/// because ten and eleven are now spelled `a` and `b` and nothing else.
/// The legacy aliases `X`/`T`/`E` therefore only work when they *are* the
/// configured pair.
///
/// Flipping this one constant to `false` restores "the legacy aliases are
/// always accepted" -- deliberately a single line, because the choice is a
/// policy call and not a structural one. The canonical alphabet is
/// unaffected either way: [`DisplayAlphabet::canonical`] short-circuits to
/// the identity, so the default build accepts exactly what it always did.
const REJECT_LEGACY_ALIASES: bool = true;

/// Why a pair was refused, or why a piece of text could not be translated.
///
/// One variant per rule, each carrying the specific symbol or name at
/// fault: "invalid symbol" on its own would leave the user guessing which
/// of four rules they tripped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlphabetError {
    /// The symbol already means something in the expression language.
    Reserved(char),
    /// Ten and eleven would be spelled the same (case-insensitively).
    Duplicate(char),
    /// A canonical glyph used for the other digit: `↊` may only ever mean
    /// ten and `↋` only ever eleven.
    WrongGlyphSlot(char),
    /// The pair would turn a catalog name into a number literal, making
    /// the function or constant unreachable (`p`/`i` swallows `pi`).
    SwallowsName {
        name: String,
        ten: char,
        eleven: char,
    },
    /// Typed text spelled a digit with a legacy alias that is not part of
    /// the configured pair. See [`REJECT_LEGACY_ALIASES`].
    LegacyAlias {
        run: String,
        ten: char,
        eleven: char,
    },
}

impl std::fmt::Display for AlphabetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlphabetError::Reserved(c) => write!(
                f,
                "'{}' cannot be a digit symbol: digits 0-9, the Humphrey point ';', \
                 the comma, parentheses and the operators already have a meaning",
                c
            ),
            AlphabetError::Duplicate(c) => write!(
                f,
                "ten and eleven cannot both be '{}': they must be different symbols",
                c
            ),
            AlphabetError::WrongGlyphSlot(c) => write!(
                f,
                "'{}' is a canonical dozenal glyph and cannot stand for the other digit \
                 ({} is ten, {} is eleven)",
                c, CANONICAL_TEN, CANONICAL_ELEVEN
            ),
            AlphabetError::SwallowsName { name, ten, eleven } => write!(
                f,
                "the pair '{}'/'{}' would make '{}' read as a number instead of a name, \
                 so the function '{}' could never be called",
                ten, eleven, name, name
            ),
            AlphabetError::LegacyAlias { run, ten, eleven } => write!(
                f,
                "'{}' is not a number in the current alphabet: ten is '{}', eleven is '{}' \
                 (the canonical glyphs {} and {} are always accepted too)",
                run, ten, eleven, CANONICAL_TEN, CANONICAL_ELEVEN
            ),
        }
    }
}
impl std::error::Error for AlphabetError {}

/// The two symbols a user sees for ten and eleven.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayAlphabet {
    ten: char,
    eleven: char,
}

impl Default for DisplayAlphabet {
    fn default() -> DisplayAlphabet {
        DisplayAlphabet::canonical()
    }
}

/// Two symbols count as the same when they differ only in ASCII case: the
/// pair is accepted in both cases on input, so `a` and `A` cannot mean
/// different digits.
fn same_symbol(a: char, b: char) -> bool {
    a.eq_ignore_ascii_case(&b)
}

/// Characters the expression language already spends: redefining any of
/// them makes text ambiguous rather than merely unfamiliar.
///
/// `~` and `/` are here because they appear in *output* (the approximation
/// marker and the fraction bar), and `=` and `.` because they read as
/// arithmetic to anyone glancing at the display.
fn is_reserved(c: char) -> bool {
    c.is_ascii_digit()
        || c.is_whitespace()
        || c.is_control()
        || matches!(
            c,
            POINT
                | ','
                | '('
                | ')'
                | '+'
                | '-'
                | '*'
                | '/'
                | '%'
                | '^'
                | '~'
                | '='
                | '.'
                | '\u{00D7}'
                | '\u{00F7}'
        )
}

impl DisplayAlphabet {
    /// The default: U+218A / U+218B, i.e. exactly today's behaviour --
    /// both translations are the identity and the legacy `X`/`T`/`E`
    /// aliases keep working through the lexer as they always have.
    pub fn canonical() -> DisplayAlphabet {
        DisplayAlphabet {
            ten: CANONICAL_TEN,
            eleven: CANONICAL_ELEVEN,
        }
    }

    /// Build a pair, refusing the four cases of [`AlphabetError`].
    ///
    /// `reserved_names` is the catalog of callable names that must keep
    /// lexing as identifiers. It is a parameter rather than a lookup
    /// because the registry lives in `dz-eval`, two layers *above* this
    /// crate; `dz_api::display_alphabet` is the caller that fills it in
    /// from `dz_eval::default_registry()`.
    pub fn new(
        ten: char,
        eleven: char,
        reserved_names: &[String],
    ) -> Result<DisplayAlphabet, AlphabetError> {
        if is_reserved(ten) {
            return Err(AlphabetError::Reserved(ten));
        }
        if is_reserved(eleven) {
            return Err(AlphabetError::Reserved(eleven));
        }
        if same_symbol(ten, eleven) {
            return Err(AlphabetError::Duplicate(ten));
        }
        if ten == CANONICAL_ELEVEN {
            return Err(AlphabetError::WrongGlyphSlot(ten));
        }
        if eleven == CANONICAL_TEN {
            return Err(AlphabetError::WrongGlyphSlot(eleven));
        }
        let candidate = DisplayAlphabet { ten, eleven };
        for name in reserved_names {
            // A name made entirely of the pair's letters is a maximal
            // letter run with no non-alias letter in it, which is the
            // lexer's definition of a number: `pi` under the pair `p`/`i`
            // would be the number ten-eleven and the constant would be
            // unreachable.
            if !name.is_empty() && name.chars().all(|c| candidate.is_display_digit(c)) {
                return Err(AlphabetError::SwallowsName {
                    name: name.clone(),
                    ten,
                    eleven,
                });
            }
        }
        Ok(candidate)
    }

    pub fn ten(&self) -> char {
        self.ten
    }

    pub fn eleven(&self) -> char {
        self.eleven
    }

    /// Whether this is the untouched default, in which case both
    /// translations are the identity.
    pub fn is_canonical(&self) -> bool {
        self.ten == CANONICAL_TEN && self.eleven == CANONICAL_ELEVEN
    }

    /// The canonical glyph this character stands for, if any: the pair in
    /// either case, plus the canonical glyphs, which are always accepted
    /// because they are ambiguous with nothing.
    fn canonical_digit(&self, c: char) -> Option<char> {
        if same_symbol(c, self.ten) {
            Some(CANONICAL_TEN)
        } else if same_symbol(c, self.eleven) {
            Some(CANONICAL_ELEVEN)
        } else if c == CANONICAL_TEN || c == CANONICAL_ELEVEN {
            Some(c)
        } else {
            None
        }
    }

    fn is_display_digit(&self, c: char) -> bool {
        self.canonical_digit(c).is_some()
    }

    /// User text -> canonical text, ready for [`crate::lex`].
    ///
    /// Letter runs are classified with the lexer's own rule under the
    /// configured alias set, so `abs` stays a name when `a`/`b` are the
    /// pair. Everything else is copied through untouched; in particular
    /// the Humphrey point, the operators and the grouping comma are never
    /// candidates for translation.
    pub fn to_canonical(&self, text: &str) -> Result<String, AlphabetError> {
        if self.is_canonical() {
            return Ok(text.to_string());
        }
        let chars: Vec<char> = text.chars().collect();
        let mut out = String::with_capacity(text.len());
        let mut i = 0usize;
        while i < chars.len() {
            let c = chars[i];
            if c.is_ascii_alphabetic() {
                let run = letter_run(&chars, i);
                if !is_identifier_run_with(&chars, i, run, |c| self.is_display_digit(c)) {
                    // Every letter is a configured digit: a number.
                    let mut k = 0usize;
                    while k < run {
                        // `canonical_digit` is `Some` for every letter of
                        // the run -- that is what the branch just tested.
                        out.push(
                            self.canonical_digit(chars[i + k])
                                .expect("run is all digits"),
                        );
                        k += 1;
                    }
                    i += run;
                    continue;
                }
                // A name under this alphabet. It must also be a name to
                // the canonical lexer, or a legacy alias would sneak in a
                // digit the user did not ask for (`E` under the pair
                // `a`/`b`).
                if REJECT_LEGACY_ALIASES && !is_identifier_run(&chars, i, run) {
                    return Err(AlphabetError::LegacyAlias {
                        run: chars[i..i + run].iter().collect(),
                        ten: self.ten,
                        eleven: self.eleven,
                    });
                }
                let mut k = 0usize;
                while k < run {
                    out.push(chars[i + k]);
                    k += 1;
                }
                i += run;
                continue;
            }
            match self.canonical_digit(c) {
                Some(glyph) => out.push(glyph),
                None => out.push(c),
            }
            i += 1;
        }
        Ok(out)
    }

    /// Canonical output -> user text, for display only.
    ///
    /// A plain substitution is correct in this direction: formatted output
    /// contains no identifiers, only digits, the Humphrey point, a sign, a
    /// fraction bar and the `~` marker.
    pub fn from_canonical(&self, text: &str) -> String {
        if self.is_canonical() {
            return text.to_string();
        }
        text.chars()
            .map(|c| match c {
                CANONICAL_TEN => self.ten,
                CANONICAL_ELEVEN => self.eleven,
                other => other,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    fn ab() -> DisplayAlphabet {
        DisplayAlphabet::new('a', 'b', &names(&["abs", "acos", "atan", "pi", "sin"]))
            .expect("a/b is a valid pair")
    }

    #[test]
    fn the_default_is_the_identity_in_both_directions() {
        let canonical = DisplayAlphabet::canonical();
        assert!(canonical.is_canonical());
        // Including the legacy aliases: the default build accepts exactly
        // what it accepted before this feature existed.
        for text in ["1+1", "XE", "abs(3)", "exp(E)", "4;6", "1,0000", "~↊;↋"] {
            assert_eq!(canonical.to_canonical(text).unwrap(), text);
            assert_eq!(canonical.from_canonical(text), text);
        }
    }

    #[test]
    fn a_over_b_is_accepted_and_does_not_swallow_abs() {
        let alphabet = ab();
        assert_eq!(alphabet.to_canonical("ab").unwrap(), "↊↋");
        assert_eq!(alphabet.to_canonical("AB").unwrap(), "↊↋");
        // The whole point of the feature.
        assert_eq!(alphabet.to_canonical("abs(3)").unwrap(), "abs(3)");
        assert_eq!(alphabet.to_canonical("atan(a)").unwrap(), "atan(↊)");
        assert_eq!(alphabet.to_canonical("2b;a").unwrap(), "2↋;↊");
    }

    #[test]
    fn p_over_i_is_refused_and_says_which_name_breaks() {
        let error = DisplayAlphabet::new('p', 'i', &names(&["pi", "sin", "abs"]))
            .expect_err("p/i swallows the constant pi");
        assert_eq!(
            error,
            AlphabetError::SwallowsName {
                name: "pi".into(),
                ten: 'p',
                eleven: 'i'
            }
        );
        assert!(error.to_string().contains("'pi'"), "{}", error);
    }

    #[test]
    fn reserved_symbols_and_duplicates_are_refused() {
        let list = names(&["pi"]);
        assert_eq!(
            DisplayAlphabet::new('5', 'b', &list).unwrap_err(),
            AlphabetError::Reserved('5')
        );
        assert_eq!(
            DisplayAlphabet::new('a', ';', &list).unwrap_err(),
            AlphabetError::Reserved(';')
        );
        assert_eq!(
            DisplayAlphabet::new('a', ',', &list).unwrap_err(),
            AlphabetError::Reserved(',')
        );
        assert_eq!(
            DisplayAlphabet::new('(', 'b', &list).unwrap_err(),
            AlphabetError::Reserved('(')
        );
        assert_eq!(
            DisplayAlphabet::new('a', '+', &list).unwrap_err(),
            AlphabetError::Reserved('+')
        );
        assert_eq!(
            DisplayAlphabet::new('a', 'A', &list).unwrap_err(),
            AlphabetError::Duplicate('a')
        );
        assert_eq!(
            DisplayAlphabet::new(CANONICAL_ELEVEN, 'b', &list).unwrap_err(),
            AlphabetError::WrongGlyphSlot(CANONICAL_ELEVEN)
        );
    }

    #[test]
    fn the_canonical_glyphs_stay_typeable_but_the_legacy_aliases_do_not() {
        let alphabet = ab();
        assert_eq!(alphabet.to_canonical("↊+↋").unwrap(), "↊+↋");
        let error = alphabet
            .to_canonical("E")
            .expect_err("E is no longer eleven");
        assert!(
            matches!(error, AlphabetError::LegacyAlias { .. }),
            "{:?}",
            error
        );
        // `exp` survives: `p` is not a canonical alias, so the run is an
        // identifier to both alphabets.
        assert_eq!(alphabet.to_canonical("exp(1)").unwrap(), "exp(1)");
    }

    #[test]
    fn output_translation_is_a_plain_substitution() {
        let alphabet = ab();
        assert_eq!(alphabet.from_canonical("~1↊;↋3"), "~1a;b3");
        assert_eq!(alphabet.from_canonical("-↋/↊"), "-b/a");
    }

    #[test]
    fn a_non_letter_pair_translates_without_touching_names() {
        let alphabet = DisplayAlphabet::new('@', '#', &names(&["pi", "abs"])).expect("valid pair");
        assert_eq!(alphabet.to_canonical("@#+abs(1)").unwrap(), "↊↋+abs(1)");
        assert_eq!(alphabet.from_canonical("↊↋"), "@#");
    }
}
