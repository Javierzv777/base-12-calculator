//! Phase 10: base-ten input, converted with gates
//! (`docs/10-entrada-base-10.md`).
//!
//! The point of the phase is not that base ten works -- it is that it
//! *costs*, and that the cost is visible in the same `gate_count` every
//! other call reports. These tests pin both halves: the values are right,
//! and the price is neither zero nor hidden.

use dz_api::{
    chain_operand, execute, BackendKind, InputBase, NumberMode, Operation, Request, Response,
};

fn eval(expr: &str, base: InputBase, mode: NumberMode) -> Response {
    execute(
        &Request::new(Operation::Eval { expr: expr.to_string() })
            .with_mode(mode)
            .with_input_base(base),
    )
    .expect("evaluates")
}

fn display(expr: &str, base: InputBase) -> String {
    eval(expr, base, NumberMode::Rational).display
}

fn cost(expr: &str, base: InputBase) -> u64 {
    eval(expr, base, NumberMode::Rational)
        .gate_count
        .expect("a successful call always counts gates")
        .total()
}

// ------------------------------------------------------------ the values

#[test]
fn a_hundred_and_forty_four_in_base_ten_is_the_same_value_as_100_in_base_twelve() {
    assert_eq!(display("144", InputBase::Ten), "100");
    assert_eq!(display("144", InputBase::Ten), display("100", InputBase::Twelve));
}

#[test]
fn the_small_cases_are_right() {
    assert_eq!(display("11", InputBase::Ten), "\u{218B}");
    assert_eq!(display("10", InputBase::Ten), "\u{218A}");
    assert_eq!(display("0", InputBase::Ten), "0");
    assert_eq!(display("9", InputBase::Ten), "9");
}

#[test]
fn a_long_integer_survives() {
    // 987654321 decimal, read digit by digit through Horner. Checked by
    // round trip rather than against a dozenal literal typed by hand: the
    // hand-typed literal is exactly the thing the project does not trust.
    let dozenal = display("987654321", InputBase::Ten);
    let (back, _) = chain_operand(BackendKind::Mvl, InputBase::Ten, &dozenal).expect("converts");
    assert_eq!(back, "987654321");
}

#[test]
fn a_decimal_fraction_stays_exact_in_rational_mode() {
    // 0.5 becomes 5/10, which DzRational reduces to one half.
    assert_eq!(display("0.5", InputBase::Ten), "1/2");
    // 0.1 has no finite dozenal form; as a rational it is exactly 1/ten.
    assert_eq!(display("0.1", InputBase::Ten), "1/\u{218A}");
    assert_eq!(display("1.25", InputBase::Ten), "5/4");
}

#[test]
fn arithmetic_composes_across_the_rewrite() {
    // 155 decimal = 1*144 + 0*12 + 11.
    assert_eq!(display("144 + 11", InputBase::Ten), "10\u{218B}");
    assert_eq!(display("12 * 12", InputBase::Ten), "100");
    // Outside a call the comma is the DECIMAL separator, so it composes
    // with the arithmetic exactly as the point does.
    assert_eq!(
        display("3,14 + 0,01", InputBase::Ten),
        display("3.14 + 0.01", InputBase::Ten)
    );
}

/// Base ten spends its comma on the fractional part, which is what a
/// Spanish or German numeric keypad emits, so it has none left over for
/// grouping: `1,000,000` is refused rather than silently read as a million.
/// Reading it as grouping is what made `3,14` mean `314`.
#[test]
fn grouping_a_base_ten_literal_is_refused_rather_than_guessed() {
    let error = execute(
        &Request::new(Operation::Eval {
            expr: "1,000,000".to_string(),
        })
        .with_input_base(InputBase::Ten),
    )
    .expect_err("two decimal separators in one literal");
    assert!(
        error.to_string().contains("second decimal separator"),
        "{}",
        error
    );
}

#[test]
fn function_names_still_work_in_base_ten() {
    assert_eq!(display("abs(3)", InputBase::Ten), "3");
    assert_eq!(display("abs(0 - 11)", InputBase::Ten), "\u{218B}");
    // A name whose letters include digit aliases (`e`, `x`, `t`) is still
    // a name: the identifier rule is the lexer's, not a new one.
    assert!(!display("exp(0)", InputBase::Ten).is_empty());
}

#[test]
fn a_dozenal_only_digit_is_refused_with_its_own_message() {
    let error = execute(
        &Request::new(Operation::Eval { expr: "\u{218A}".into() })
            .with_input_base(InputBase::Ten),
    )
    .expect_err("ten has no base-ten spelling");
    let message = error.to_string();
    assert!(message.contains("not a base-ten digit"), "{}", message);

    let humphrey = execute(
        &Request::new(Operation::Eval { expr: "4;6".into() }).with_input_base(InputBase::Ten),
    )
    .expect_err("the Humphrey point is dozenal");
    assert!(humphrey.to_string().contains("write '.'"), "{}", humphrey);
}

// ------------------------------------------------------------- the price

/// The result of the phase: base twelve reads a literal for free because
/// the digit's position already is its weight; base ten has to compute
/// every weight, and the bill grows with the number of digits.
#[test]
fn reading_base_ten_costs_gates_and_the_cost_grows_with_the_digits() {
    // Each pair is the *same value* spelled in the two bases, so whatever
    // the evaluator spends on it cancels and the difference is purely the
    // reading. Powers of twelve make the pairing obvious.
    let pairs = [
        ("12", "10"),
        ("144", "100"),
        ("1728", "1000"),
        ("20736", "10000"),
        ("248832", "100000"),
    ];
    let mut previous = 0u64;
    for (decimal, dozenal) in pairs {
        assert_eq!(display(decimal, InputBase::Ten), dozenal);
        let read_as_ten = cost(decimal, InputBase::Ten);
        let read_as_twelve = cost(dozenal, InputBase::Twelve);
        assert!(
            read_as_ten > read_as_twelve,
            "{}: base ten cost {} is not above base twelve's {}",
            decimal,
            read_as_ten,
            read_as_twelve
        );
        let reading = read_as_ten - read_as_twelve;
        assert!(
            reading > previous,
            "{}: reading cost {} <= {}",
            decimal,
            reading,
            previous
        );
        previous = reading;
    }
}

#[test]
fn the_default_request_is_base_twelve_and_unchanged() {
    let request = Request::new(Operation::Sum { a: "1".into(), b: "1".into() });
    assert_eq!(request.input_base, InputBase::Twelve);
    let response = execute(&request.with_mode(NumberMode::Fixed)).expect("evaluates");
    // The control of the whole phase: moving the measurement window to
    // before the parse changed no published number.
    assert_eq!(
        response.gate_count.expect("counts").total(),
        24_083,
        "1+1 is still 24,083 gates"
    );
    assert_eq!(response.depth.expect("depth"), 441);
}

// ----------------------------------------------------------- the way back

/// The chaining bug: `144 + 11` shows `103`, and seeding `103` into a
/// base-ten input box would silently mean one hundred and three.
#[test]
fn chaining_converts_the_result_back_instead_of_reusing_the_dozenal_text() {
    let shown = display("144 + 11", InputBase::Ten);
    // 155 decimal, which is `10↋` dozenal. Seeding `10↋` into a base-ten
    // box is not even typeable; seeding the digits of a shorter dozenal
    // answer would be worse still, because it would read as a *valid*
    // decimal and continue from the wrong number in silence.
    assert_eq!(shown, "10\u{218B}");
    let (chained, count) = chain_operand(BackendKind::Mvl, InputBase::Ten, &shown).expect("converts");
    assert_eq!(chained, "155");
    assert_ne!(chained, shown);
    assert!(count.total() > 0, "writing a decimal is gate work");
    // And feeding it straight back in reproduces the same value.
    assert_eq!(display(&chained, InputBase::Ten), shown);
}

#[test]
fn chaining_in_base_twelve_is_the_identity_and_free() {
    let (chained, count) =
        chain_operand(BackendKind::Mvl, InputBase::Twelve, "103").expect("converts");
    assert_eq!(chained, "103");
    assert_eq!(count.total(), 0);
}

#[test]
fn chaining_a_rational_converts_both_integers_exactly() {
    let shown = display("1 / 11", InputBase::Ten);
    assert_eq!(shown, "1/\u{218B}");
    let (chained, _) = chain_operand(BackendKind::Mvl, InputBase::Ten, &shown).expect("converts");
    assert_eq!(chained, "1/11");
    assert_eq!(display(&chained, InputBase::Ten), shown);
}

/// A fixed-point value need not have a finite decimal form, so the way
/// back emits the exact fraction rather than truncating in silence -- the
/// same call `docs/08` section 3.2 made in the other direction.
#[test]
fn chaining_a_fixed_value_emits_an_exact_fraction() {
    let response = eval("1 / 3", InputBase::Ten, NumberMode::Fixed);
    assert!(response.display.starts_with("0;4"), "{}", response.display);
    let (chained, _) =
        chain_operand(BackendKind::Mvl, InputBase::Ten, &response.display).expect("converts");
    let (numerator, denominator) = chained.split_once('/').expect("an exact fraction");
    assert!(!numerator.is_empty() && !denominator.is_empty());
    // Reading it back in base ten reproduces the same fixed-point value.
    assert_eq!(
        eval(&chained, InputBase::Ten, NumberMode::Fixed).display,
        response.display
    );
}
