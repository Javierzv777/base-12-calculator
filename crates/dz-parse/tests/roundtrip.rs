use dz_gates::{Digit, MvlGates};
use dz_num::DzFixed;
use dz_parse::{format_fixed, parse_str};
use proptest::prelude::*;

fn arb_digit() -> impl Strategy<Value = Digit> {
    (0u8..12).prop_map(|t| Digit::new(t).unwrap())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(300))]

    /// Round-trip: format a value, parse the resulting text back through
    /// the full lexer/parser/evaluator pipeline, and check we get the same
    /// value back. PLAN.md Section 8.2 requires exactly this.
    #[test]
    fn parse_format_roundtrip(
        negative in any::<bool>(),
        int_digits in prop::collection::vec(arb_digit(), 1..5),
        frac_digits in prop::collection::vec(arb_digit(), 0..4),
    ) {
        let value = DzFixed::from_digits(negative, &int_digits, &frac_digits);
        let text = format_fixed(&value);
        let g = MvlGates::new();
        let expr = parse_str(&text).expect("formatted output must re-parse");
        let reparsed = dz_eval::evaluate(&g, &expr).expect("evaluation must succeed");
        prop_assert_eq!(reparsed, value);
    }
}

#[test]
fn basic_examples() {
    assert_eq!(
        format_fixed(&DzFixed::from_digits(false, &[Digit::new(4).unwrap()], &[Digit::new(6).unwrap()])),
        "4;6"
    );
}
