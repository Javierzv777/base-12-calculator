//! The **input** base: which base the number literals a user types are
//! meant in (`docs/10-entrada-base-10.md`).
//!
//! Sibling of [`crate::display_alphabet`] and deliberately the same shape,
//! including the pair of directions: [`to_dozenal_text`] runs user text
//! into canonical dozenal text just before the lexer, and
//! [`from_dozenal_text`] runs a formatted result back out, which the
//! result-chaining feature needs so it does not silently reinterpret a
//! dozenal answer as a decimal one. Nothing below `dz-parse` ever sees a
//! base-ten character, so no circuit, backend or published gate count
//! changes because of either.
//!
//! Unlike the alphabet's pair, neither direction here is a substitution.
//! Both are arithmetic, and they are not even the same arithmetic: the way
//! in is multiplications (`dz_int::from_base10_digits`), the way out is
//! divisions (`dz_int::to_base10_digits`), and the restoring divider is
//! the most expensive circuit in the project.
//!
//! ## What makes this different from the presentation alphabet
//!
//! Choosing other symbols for ten and eleven is a relabelling: it costs
//! nothing, because the value each position carries is unchanged. Choosing
//! base ten is **not**. In a dozenal magnitude a digit's position *is* its
//! weight, so reading `100` costs zero gates -- the `Vec` already is the
//! number. In base ten no position means a power of twelve, so every
//! weight has to be computed, and this module computes it the only way the
//! repository allows: `dz_int::from_base10_digits`, a Horner loop of
//! gate-composed `Dz::mul`/`Dz::add`.
//!
//! That is why this function takes a `&G` when [`crate::DisplayAlphabet`]
//! does not, and why `dz-api` starts its measurement *before* calling it:
//! in base ten, reading the input is arithmetic, and the whole point of
//! the phase is that its price shows up in `gate_count`.
//!
//! ## What is reused rather than re-derived
//!
//! Two lexer rules are borrowed outright instead of copied:
//!
//!   - [`crate::lexer::is_identifier_run`] decides name vs. number. A run
//!     of letters that the canonical lexer would read as a *number*
//!     (`E`, `XE`) is exactly the run base ten has to reject; a run it
//!     reads as a name (`abs`, `exp`, `tan`) is passed through untouched.
//!   - [`crate::lexer::CallParens`] decides what a comma means. Inside a
//!     call it separates arguments and therefore ends a literal; outside
//!     one it is the cosmetic grouping separator and is dropped, so
//!     `1,000,000` reads as one seven-digit decimal number.
//!
//! ## The one simplification, stated as such
//!
//! [`crate::DisplayAlphabet::to_canonical`] is **not** applied to base-ten
//! text. In base ten no letter is a digit, so a letter run is
//! unambiguously a name and there is nothing to translate: `abs` is `abs`
//! whatever pair the user configured for ten and eleven. This is a real
//! simplification of the pipeline, not an oversight -- it is also the
//! reason the two settings compose without either having to know about the
//! other.

use crate::alphabet::{char_to_digit, digit_to_char, POINT};
use crate::display_alphabet::{CANONICAL_ELEVEN, CANONICAL_TEN};
use crate::lexer::{is_identifier_run, letter_run, CallParens};
use dz_gates::{Digit, DozGates};
use dz_int::{from_base10_digits, pow10, to_base10_digits, Dz};

/// Which base the *typed* number literals are in.
///
/// `Twelve` is the default and the identity, exactly as
/// [`crate::DisplayAlphabet::canonical`] is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputBase {
    #[default]
    Twelve,
    Ten,
}

impl InputBase {
    /// Read the setting from a flag value: `"10"`/`"ten"`, `"12"`/`"twelve"`.
    pub fn parse(name: &str) -> Option<InputBase> {
        match name {
            "12" | "twelve" | "dozenal" => Some(InputBase::Twelve),
            "10" | "ten" | "decimal" => Some(InputBase::Ten),
            _ => None,
        }
    }

    /// The flag value that names this base, for echoing a session back.
    pub fn label(self) -> &'static str {
        match self {
            InputBase::Twelve => "12",
            InputBase::Ten => "10",
        }
    }

    /// Whether this is the untouched default, in which case the rewrite is
    /// the identity and costs nothing.
    pub fn is_dozenal(self) -> bool {
        matches!(self, InputBase::Twelve)
    }
}

/// Why a piece of base-ten text could not be read.
///
/// One variant per rule, each carrying what the user actually typed: a
/// bare "syntax error" would leave them guessing which of three rules they
/// tripped, and two of the three have an obvious fix worth naming.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputBaseError {
    /// A dozenal-only digit (`↊`, `↋`, or one of the ASCII aliases) typed
    /// while the input base is ten.
    NotDecimal { symbol: String, position: usize },
    /// The Humphrey point, which is the dozenal fractional separator.
    HumphreyPoint { position: usize },
    /// Two decimal points in one literal.
    SecondPoint { position: usize },
}

impl std::fmt::Display for InputBaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputBaseError::NotDecimal { symbol, position } => write!(
                f,
                "base-ten input at {}: '{}' is not a base-ten digit -- base ten has only \
                 0-9, and there is no decimal spelling of ten ({}) or eleven ({}); \
                 switch the input base to 12 to type them",
                position, symbol, CANONICAL_TEN, CANONICAL_ELEVEN
            ),
            InputBaseError::HumphreyPoint { position } => write!(
                f,
                "base-ten input at {}: '{}' is the dozenal fractional separator; \
                 in base ten write '.' instead",
                position, POINT
            ),
            InputBaseError::SecondPoint { position } => write!(
                f,
                "base-ten input at {}: unexpected second decimal point",
                position
            ),
        }
    }
}
impl std::error::Error for InputBaseError {}

/// One dozenal magnitude, most significant digit first, as text.
///
/// `digit_to_char` is the sanctioned symbol-table lookup; the reversal is
/// `Vec` bookkeeping (magnitudes are little-endian, text is not).
fn dozenal_text(value: &Dz) -> String {
    value
        .magnitude_digits()
        .iter()
        .rev()
        .map(|&d| digit_to_char(d))
        .collect()
}

/// User text -> canonical dozenal text, ready for [`crate::lex`].
///
/// For [`InputBase::Twelve`] this is the identity and touches no gate. For
/// [`InputBase::Ten`] every number literal is converted with
/// `dz_int::from_base10_digits`, so `g` is charged for the reading.
///
/// A literal with a fractional part becomes a parenthesised division
/// `(n/10^k)` rather than a dozenal fraction, because most decimal
/// fractions do not terminate in base twelve: `0.5` is `(5/↊)`, which
/// `DzRational` reduces to exactly one half, and `0.1` is `(1/↊)`, which
/// stays exact in rational mode and truncates to the working scale in the
/// other two. Writing a truncated dozenal literal here instead would throw
/// the exactness away before the evaluator ever saw the value. The
/// denominator comes from `dz_int::pow10`, which is itself a chain of
/// gate-composed multiplications -- there is no free power of ten.
pub fn to_dozenal_text<G: DozGates>(
    g: &G,
    base: InputBase,
    text: &str,
) -> Result<String, InputBaseError> {
    if base.is_dozenal() {
        return Ok(text.to_string());
    }
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut parens = CallParens::default();
    // Whether the last thing emitted was a name, which is what makes the
    // next '(' a call's argument list rather than a grouping paren.
    let mut after_ident = false;
    let mut i = 0usize;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            out.push(c);
            i += 1;
            continue;
        }
        if c.is_ascii_alphabetic() {
            let run = letter_run(&chars, i);
            if !is_identifier_run(&chars, i, run) {
                // Every letter is a canonical digit alias, so this run is
                // a dozenal number (`E`, `XE`) and has no base-ten reading.
                return Err(InputBaseError::NotDecimal {
                    symbol: chars[i..i + run].iter().collect(),
                    position: i,
                });
            }
            let mut k = 0usize;
            while k < run {
                out.push(chars[i + k]);
                k += 1;
            }
            i += run;
            after_ident = true;
            continue;
        }
        if c == CANONICAL_TEN || c == CANONICAL_ELEVEN {
            return Err(InputBaseError::NotDecimal {
                symbol: c.to_string(),
                position: i,
            });
        }
        if c == POINT {
            return Err(InputBaseError::HumphreyPoint { position: i });
        }
        if c.is_ascii_digit() || c == '.' {
            let in_call = parens.in_call();
            let start = i;
            let mut int_digits = Vec::new();
            let mut frac_digits = Vec::new();
            let mut seen_point = false;
            while i < chars.len() {
                let cc = chars[i];
                if cc == ',' {
                    if in_call {
                        break; // argument separator, not grouping
                    }
                    i += 1;
                    continue;
                }
                if cc == '.' {
                    if seen_point {
                        return Err(InputBaseError::SecondPoint { position: i });
                    }
                    seen_point = true;
                    i += 1;
                    continue;
                }
                if cc.is_ascii_digit() {
                    // `char_to_digit` is the project's single character ->
                    // symbol table; `0`-`9` mean the same symbol in both
                    // bases, which is exactly why only the *weights* need
                    // recomputing.
                    let d = char_to_digit(cc).expect("an ASCII digit is a dozenal digit");
                    if seen_point {
                        frac_digits.push(d);
                    } else {
                        int_digits.push(d);
                    }
                    i += 1;
                    continue;
                }
                if cc.is_ascii_alphabetic() || cc == CANONICAL_TEN || cc == CANONICAL_ELEVEN {
                    // A dozenal digit glued to a decimal literal (`1E`):
                    // report it here rather than silently ending the
                    // number and tripping over it on the next pass.
                    let run = letter_run(&chars, i);
                    if run == 0 || !is_identifier_run(&chars, i, run) {
                        return Err(InputBaseError::NotDecimal {
                            symbol: if run == 0 {
                                cc.to_string()
                            } else {
                                chars[i..i + run].iter().collect()
                            },
                            position: i,
                        });
                    }
                }
                break;
            }
            if int_digits.is_empty() && frac_digits.is_empty() {
                // A lone '.', which the dozenal lexer will reject with its
                // own "unexpected character" message.
                out.push('.');
                i = start + 1;
                after_ident = false;
                continue;
            }
            if frac_digits.is_empty() {
                let value = from_base10_digits(g, &int_digits);
                out.push_str(&dozenal_text(&value));
            } else {
                // n / 10^k, with k the number of decimal places.
                let places = frac_digits.len();
                let mut all = int_digits;
                all.extend_from_slice(&frac_digits);
                let numerator = from_base10_digits(g, &all);
                let denominator = pow10(g, places);
                out.push('(');
                out.push_str(&dozenal_text(&numerator));
                out.push('/');
                out.push_str(&dozenal_text(&denominator));
                out.push(')');
            }
            after_ident = false;
            continue;
        }
        if c == '(' {
            parens.open(after_ident);
        } else if c == ')' {
            parens.close();
        }
        out.push(c);
        after_ident = false;
        i += 1;
    }
    Ok(out)
}

/// A dozenal magnitude read straight out of text, most significant digit
/// first. Free: the characters already are the magnitude, so this is a
/// `Vec` reversal and nothing else.
fn dozenal_value(digits: &[Digit]) -> Dz {
    let mut little_endian = digits.to_vec();
    little_endian.reverse();
    Dz::from_magnitude(false, little_endian)
}

/// Twelve to the `k`, as a magnitude. Zero gates, and that is the point:
/// a power of the base is `k` zeros and a one, while `dz_int::pow10` has
/// to multiply. The asymmetry between the two directions is the same
/// asymmetry the whole phase is about.
fn pow12(k: usize) -> Dz {
    let mut digits = vec![Digit::ZERO; k];
    digits.push(Digit::ONE);
    Dz::from_magnitude(false, digits)
}

/// One magnitude written in base-ten numerals.
fn base10_text<G: DozGates>(g: &G, value: &Dz) -> String {
    to_base10_digits(g, value)
        .into_iter()
        .map(digit_to_char)
        .collect()
}

/// Canonical dozenal output -> text in `base`, the mirror of
/// [`to_dozenal_text`] and the same pair [`crate::DisplayAlphabet`] makes
/// with `to_canonical`/`from_canonical`.
///
/// ## The bug this exists to prevent
///
/// The result-chaining feature (`docs/09-encadenado-de-resultado.md`)
/// seeds the input box with the last result when the user types an
/// operator straight after Enter. In base twelve that is coherent: the box
/// and the result speak the same language. With a base-ten input box it is
/// not. `144 + 11` displays `103`, and seeding `103` into a box that is
/// read as base ten would silently continue the calculation from one
/// hundred and three -- `87` dozenal, a different number, with nothing on
/// screen to say so. Chaining therefore has to convert the result back,
/// and back through gates, for the same reason the way in does.
///
/// ## Exactness, per mode
///
/// The output text is either two integers or one fixed-point value, and
/// the two get different treatment because only one of them can be written
/// as a finite decimal:
///
///   - **Rational** (`n/d`): both parts are integers, so converting each
///     separately is exact, always. The simple face is pinned to rational
///     precisely so it never leaves this case.
///   - **Fixed and DFP** (`i;f`): the value is `magnitude / 12^scale`.
///     Exact as a fraction, but often not as a decimal -- `0;4` is a third
///     and has no finite decimal form. Truncating silently is not an
///     option, so this emits the exact fraction in base-ten numerals
///     (`magnitude` over `12^scale`, both converted). That is the same
///     call `docs/08` section 3.2 already made in the other direction,
///     where `0.1` came back as `1/↊` rather than a truncated dozenal.
///
/// Everything that is not a digit run -- the sign, the fraction bar, the
/// `~` approximation marker, `Inf`/`NaN` -- is copied through. Formatted
/// output contains no identifiers that could be mistaken for digits, which
/// is what makes this direction so much simpler than the way in.
pub fn from_dozenal_text<G: DozGates>(
    g: &G,
    base: InputBase,
    text: &str,
) -> Result<String, InputBaseError> {
    if base.is_dozenal() {
        return Ok(text.to_string());
    }
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0usize;
    while i < chars.len() {
        let c = chars[i];
        if char_to_digit(c).is_none() || c.is_ascii_alphabetic() {
            // Not the start of a number: a sign, a bar, `~`, a space, or a
            // letter of `Inf`/`NaN`. The alphabetic guard matters because
            // `char_to_digit` accepts the legacy `X`/`E` aliases, which
            // formatted output never uses.
            out.push(c);
            i += 1;
            continue;
        }
        let mut int_digits: Vec<Digit> = Vec::new();
        let mut frac_digits: Vec<Digit> = Vec::new();
        let mut seen_point = false;
        while i < chars.len() {
            let cc = chars[i];
            if cc == POINT && !seen_point {
                seen_point = true;
                i += 1;
                continue;
            }
            if cc.is_ascii_alphabetic() {
                break;
            }
            match char_to_digit(cc) {
                Some(d) => {
                    if seen_point {
                        frac_digits.push(d);
                    } else {
                        int_digits.push(d);
                    }
                    i += 1;
                }
                None => break,
            }
        }
        if frac_digits.is_empty() {
            out.push_str(&base10_text(g, &dozenal_value(&int_digits)));
        } else {
            let places = frac_digits.len();
            let mut all = int_digits;
            all.extend_from_slice(&frac_digits);
            out.push_str(&base10_text(g, &dozenal_value(&all)));
            out.push('/');
            out.push_str(&base10_text(g, &pow12(places)));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dz_gates::MvlGates;

    fn ten(text: &str) -> String {
        to_dozenal_text(&MvlGates::new(), InputBase::Ten, text).expect("valid base-ten text")
    }

    fn back(text: &str) -> String {
        from_dozenal_text(&MvlGates::new(), InputBase::Ten, text).expect("formatted output")
    }

    #[test]
    fn base_twelve_is_the_identity_and_costs_nothing() {
        let g = MvlGates::new();
        for text in ["1+1", "XE", "abs(3)", "4;6", "1,0000", "↊;↋"] {
            assert_eq!(
                to_dozenal_text(&g, InputBase::Twelve, text).unwrap(),
                text,
                "{}",
                text
            );
        }
        assert_eq!(g.cost().total(), 0);
    }

    #[test]
    fn integers_are_rewritten_and_names_are_not() {
        assert_eq!(ten("144"), "100");
        assert_eq!(ten("11"), "↋");
        assert_eq!(ten("0"), "0");
        assert_eq!(ten("144 + 11"), "100 + ↋");
        assert_eq!(ten("abs(3)"), "abs(3)");
        assert_eq!(ten("log_base(8, 2)"), "log_base(8, 2)");
    }

    #[test]
    fn the_grouping_comma_survives_but_the_argument_comma_ends_the_number() {
        // Outside a call the comma groups digits, so this is one million.
        assert_eq!(ten("1,000,000"), ten("1000000"));
        // Inside one it separates arguments, so these stay two numbers.
        assert_eq!(ten("log_base(100,10)"), "log_base(84,↊)");
    }

    #[test]
    fn a_fraction_becomes_an_exact_division() {
        assert_eq!(ten("0.5"), "(5/↊)");
        assert_eq!(ten("0.1"), "(1/↊)");
        assert_eq!(ten("1.25"), "(↊5/84)");
    }

    #[test]
    fn dozenal_only_symbols_are_refused_with_their_own_message() {
        let g = MvlGates::new();
        for text in ["↊", "E", "1+↋", "XE"] {
            let error = to_dozenal_text(&g, InputBase::Ten, text)
                .expect_err("dozenal digits have no base-ten reading");
            assert!(
                matches!(error, InputBaseError::NotDecimal { .. }),
                "{:?}",
                error
            );
            assert!(error.to_string().contains("not a base-ten digit"));
        }
        let error = to_dozenal_text(&g, InputBase::Ten, "4;6").expect_err("';' is dozenal");
        assert!(matches!(error, InputBaseError::HumphreyPoint { .. }), "{:?}", error);
    }

    // ---------------------------------------------------------- the way back

    #[test]
    fn base_twelve_is_the_identity_on_the_way_back_too() {
        let g = MvlGates::new();
        for text in ["103", "-4;6", "~1;4972", "23↊/↋", "Inf", "NaN"] {
            assert_eq!(from_dozenal_text(&g, InputBase::Twelve, text).unwrap(), text);
        }
        assert_eq!(g.cost().total(), 0);
    }

    #[test]
    fn a_round_trip_is_the_identity() {
        for decimal in ["0", "7", "11", "144", "1000", "987654321"] {
            assert_eq!(back(&ten(decimal)), decimal, "{}", decimal);
        }
    }

    /// The chaining bug this direction exists to prevent: `144 + 11`
    /// displays `103`, and `103` read as base ten is a different number.
    #[test]
    fn a_dozenal_result_comes_back_as_the_decimal_it_denotes() {
        assert_eq!(ten("144 + 11"), "100 + ↋");
        // 100 + ↋ = 103 dozenal, which is 147 in decimal -- not 103.
        assert_eq!(back("103"), "147");
        assert_ne!(back("103"), "103");
    }

    #[test]
    fn a_rational_display_converts_both_integers_exactly() {
        // Two integers around a bar: no rounding is possible either side.
        assert_eq!(back("23↊/↋"), "334/11");
        assert_eq!(back("-1/2"), "-1/2");
        assert_eq!(back("~↋"), "~11");
    }

    #[test]
    fn a_fixed_value_that_has_no_finite_decimal_comes_back_as_an_exact_fraction() {
        // 0;4 is one third: there is no finite decimal for it, so the
        // exact fraction is emitted rather than a silent truncation.
        assert_eq!(back("0;4"), "4/12");
        assert_eq!(back("11;6"), "162/12");
        assert_eq!(back("-0;06"), "-6/144");
    }

    #[test]
    fn writing_a_decimal_costs_gates_and_the_cost_grows_with_the_digits() {
        let free = MvlGates::new();
        from_dozenal_text(&free, InputBase::Twelve, "123456").unwrap();
        assert_eq!(free.cost().total(), 0);

        let mut previous = 0u64;
        for text in ["↋", "↋↋", "↋↋↋", "↋↋↋↋↋↋"] {
            let g = MvlGates::new();
            from_dozenal_text(&g, InputBase::Ten, text).unwrap();
            let cost = g.cost().total();
            assert!(cost > previous, "{} cost {} <= {}", text, cost, previous);
            previous = cost;
        }
    }

    #[test]
    fn reading_a_decimal_literal_costs_gates_and_the_cost_grows_with_the_digits() {
        // The result of the phase, pinned: base twelve reads a literal for
        // free because the position is the weight; base ten has to pay for
        // every weight it computes.
        let free = MvlGates::new();
        to_dozenal_text(&free, InputBase::Twelve, "123456").unwrap();
        assert_eq!(free.cost().total(), 0);

        // A *single* decimal digit is still free: `0`-`9` denote the same
        // symbol in both bases, and Horner's first step multiplies a zero
        // accumulator, which `Dz::mul` short-circuits. The price starts at
        // the second digit, which is the first one that needs a weight.
        let one_digit = MvlGates::new();
        to_dozenal_text(&one_digit, InputBase::Ten, "7").unwrap();
        assert_eq!(one_digit.cost().total(), 0);

        let mut previous = 0u64;
        for text in ["12", "123", "123456"] {
            let g = MvlGates::new();
            to_dozenal_text(&g, InputBase::Ten, text).unwrap();
            let cost = g.cost().total();
            assert!(cost > previous, "{} cost {} <= {}", text, cost, previous);
            previous = cost;
        }
    }
}
