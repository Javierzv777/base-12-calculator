// Cases for the separate base-10 input helper (`docs/08-conversion-base10.md`
// section 5). The project has no frontend test runner, so this is a plain
// script: `node app/tests/base10.test.ts` (Node strips the types).
//
// It lives outside `app/src` on purpose. Everything under `src` is compiled by
// `tsc --noEmit` during `npm run build`, and importing a `.ts` path -- which is
// what Node's type stripping needs -- would have required turning on
// `allowImportingTsExtensions` in the shared tsconfig. Keeping the test out of
// `src` costs nothing and touches no existing configuration.
//
// Like the module it exercises, this file imports nothing from the calculator.

import {
  base10CacheSize,
  clearBase10Cache,
  convertBase10Text,
  convertBase12Text,
  terminatesInBase10,
  terminatesInBase12,
  type Base10Style,
} from "../src/base10.ts";

// The mode and the scale arrive as arguments, exactly like the symbols: the
// module never imports anything to find them out (section 1.2).
const CANONICAL: Base10Style = { ten: "↊", eleven: "↋", mode: "rational", scale: 12 };
const FIXED: Base10Style = { ...CANONICAL, mode: "fixed" };
const DFP: Base10Style = { ...CANONICAL, mode: "dfp" };
const FIXED_4: Base10Style = { ...CANONICAL, mode: "fixed", scale: 4 };

let failures = 0;
let checks = 0;

function check(label: string, actual: unknown, expected: unknown): void {
  checks += 1;
  const a = JSON.stringify(actual);
  const b = JSON.stringify(expected);
  if (a === b) {
    console.log(`ok   ${label}`);
  } else {
    failures += 1;
    console.log(`FAIL ${label}\n       expected ${b}\n       actual   ${a}`);
  }
}

/** The conversion, or null when it would change nothing. */
function suggestion(text: string, symbols: Base10Style = CANONICAL): string | null {
  const out = convertBase10Text(text, symbols);
  return out.changed ? out.text : null;
}


/** What the preview shows, which may carry the preview-only `…`. */
function preview(text: string, symbols: Base10Style = CANONICAL): string | null {
  return convertBase10Text(text, symbols).preview;
}

/** What the always-available button would produce, with no heuristic at all. */
function button(text: string, symbols: Base10Style = CANONICAL): string {
  return convertBase10Text(text, symbols).text;
}

// --- the case from the request -------------------------------------------
check("12 + 11", suggestion("12 + 11"), "10 + ↋");

// --- unchanged text stays quiet: the filter that buys the discretion ------
check("5 + 3 stays quiet", suggestion("5 + 3"), null);
check("7*6 stays quiet", suggestion("7*6"), null);

// --- plain integers -------------------------------------------------------
check("144", suggestion("144"), "100");
check("11", suggestion("11"), "↋");
check("10", suggestion("10"), "↊");
check("0", suggestion("0"), null);
check("007 keeps its zeros", button("007"), "007");

// Beyond Number.MAX_SAFE_INTEGER (2^53 + 1): BigInt is exact at any size.
check("huge integer", button("9007199254740993"), "702273685↋77↊29");

// --- decimals that terminate (section 3.2) -------------------------------
check("0.5", suggestion("0.5"), "0;6");
check("0.25", suggestion("0.25"), "0;3");
check("0.75", suggestion("0.75"), "0;9");
check("1.5", suggestion("1.5"), "1;6");

// --- decimals that do not terminate: the shape depends on the mode -------
// rational: the exact fraction.
check("0.1 rational", suggestion("0.1"), "1/↊");
check("0.3 rational", suggestion("0.3"), "3/↊");
// Inside a larger expression the fraction is parenthesised, or `1/0.1` would
// become `1/1/↊`, which the parser reads as `(1/1)/↊`.
check("1/0.1 rational", suggestion("1/0.1"), "1/(1/↊)");
check("2 + 0.3 rational", suggestion("2 + 0.3"), "2 + (3/↊)");

// fixed: truncated to the session scale. dfp goes the same way.
check("0.1 fixed", suggestion("0.1", FIXED), "0;124972497249");
check("0.3 fixed", suggestion("0.3", FIXED), "0;372497249724");
check("0.1 dfp", suggestion("0.1", DFP), "0;124972497249");
// The same case at another scale: the scale is what decides, not the tool.
check("0.1 fixed scale 4", suggestion("0.1", FIXED_4), "0;1249");
check("1.1 fixed scale 4", suggestion("1.1", FIXED_4), "1;1249");
// No parentheses are needed around a Humphrey number.
check("1/0.1 fixed", suggestion("1/0.1", FIXED), "1/0;124972497249");

// The ellipsis is preview-only: the lexer does not accept `…`, so it must
// never reach the input box.
check("preview shows the ellipsis", preview("0.1", FIXED), "0;124972497249…");
check("converted has no ellipsis", suggestion("0.1", FIXED)?.includes("…"), false);
check("button text has no ellipsis", button("0.1", FIXED).includes("…"), false);
check("exact preview has no ellipsis", preview("0.5", FIXED), "0;6");
check("fraction preview has no ellipsis", preview("0.1", CANONICAL), "1/↊");

// --- the termination test on its own, for several k ----------------------
check("5^1 | 5", terminatesInBase12(5n, 1), true);
check("5^1 | 1", terminatesInBase12(1n, 1), false);
check("5^2 | 25", terminatesInBase12(25n, 2), true);
check("5^2 | 75", terminatesInBase12(75n, 2), true);
check("5^2 | 50", terminatesInBase12(50n, 2), true); // 0.50 is 1/2
check("5^2 | 20", terminatesInBase12(20n, 2), false); // 0.20 is 1/5
check("5^3 | 125", terminatesInBase12(125n, 3), true);
check("5^3 | 375", terminatesInBase12(375n, 3), true);
check("5^3 | 625", terminatesInBase12(625n, 3), true);
check("5^3 | 100", terminatesInBase12(100n, 3), false);
check("0.125", suggestion("0.125"), "0;16");
check("0.375", suggestion("0.375"), "0;46");

// --- text that already carries dozenal glyphs ----------------------------
// The button converts what it can and leaves the rest; the popup does not fire.
check("↊ + 12 by button", button("↊ + 12"), "↊ + 10");
check("4;6 * 3 stays quiet", suggestion("4;6 * 3"), null);
check("↊ + 12 kept dozenal", convertBase10Text("↊ + 12", CANONICAL).keptDozenal, true);
check("configured symbol by button", button("12 + B", { ...CANONICAL, ten: "A", eleven: "B" }), "10 + B");

// --- digits inside an identifier are never rewritten ---------------------
check("identifier digits", button("log12(144)"), "log12(100)");
check("call still suggests", suggestion("sqrt(144)"), "sqrt(100)");

// --- prose is not an expression ------------------------------------------

// --- three-digit thousands grouping is a base-10 tell --------------------
check("thousands value", suggestion("1,000"), "6↋4");
// Dozenal grouping is four digits: leave it alone.
check("dozenal grouping stays quiet", suggestion("1,0000"), null);

// --- the configured symbol pair is honoured in the output ----------------
check("configured pair in output", suggestion("11", { ...CANONICAL, ten: "X", eleven: "E" }), "E");
check("configured pair, larger", suggestion("143", { ...CANONICAL, ten: "X", eleven: "E" }), "EE");
check("configured pair in a fraction", button("0.1", { ...CANONICAL, ten: "X", eleven: "E" }), "1/X");

// --- the button reports what it did --------------------------------------
check("nothing to convert", convertBase10Text("5 + 3", CANONICAL).changed, false);
check("fraction flag", convertBase10Text("0.1", CANONICAL).madeFraction, true);
check("humphrey flag", convertBase10Text("0.5", CANONICAL).madeHumphrey, true);

// --- the conversion cache (section 2.2) -----------------------------------
// The cache is an optimisation and never a source of truth, so every case here
// compares the cached route against a recompute from an empty cache. If the two
// ever disagree the cache is wrong, whatever it saved.

/** The answer with a guaranteed-cold cache: the reference every hit must match. */
function cold(text: string, symbols: Base10Style): unknown {
  clearBase10Cache();
  const fresh = convertBase10Text(text, symbols);
  clearBase10Cache();
  return fresh;
}

/** The answer on a hit: the same call made twice, with the cache left warm. */
function warm(text: string, symbols: Base10Style): unknown {
  convertBase10Text(text, symbols);
  return convertBase10Text(text, symbols);
}

for (const [label, text, symbols] of [
  ["integers", "12 + 11", CANONICAL],
  ["fraction", "0.1", CANONICAL],
  ["truncated", "0.1", FIXED],
  ["truncated, other scale", "0.1", FIXED_4],
  ["humphrey", "0.25", CANONICAL],
  ["unchanged text", "5 + 3", CANONICAL],
  ["dozenal left alone", "↊ + 12", CANONICAL],
] as Array<[string, string, Base10Style]>) {
  check(`cache hit equals recompute: ${label}`, warm(text, symbols), cold(text, symbols));
}

// The trap the key exists for: the same text under another configuration must
// never be served the first configuration's answer.
clearBase10Cache();
check("cache: rational first", convertBase10Text("0.1", CANONICAL).text, "1/↊");
check("cache: fixed is not the rational entry", convertBase10Text("0.1", FIXED).text, "0;124972497249");
check("cache: scale 4 is not the scale 12 entry", convertBase10Text("0.1", FIXED_4).text, "0;1249");
check("cache: dfp is not the rational entry", convertBase10Text("0.1", DFP).text, "0;124972497249");
check(
  "cache: another alphabet is not the canonical entry",
  convertBase10Text("11", { ...CANONICAL, ten: "X", eleven: "E" }).text,
  "E",
);
check("cache: the canonical alphabet is untouched by it", convertBase10Text("11", CANONICAL).text, "↋");
check("cache: one text, six distinct entries", base10CacheSize(), 6);

// Every field of `Base10Style` has to be part of the key. This walks the object
// instead of naming the fields, so a field added tomorrow is covered here too:
// the loop fails as soon as a new field is left out of the key.
for (const field of Object.keys(CANONICAL) as Array<keyof Base10Style>) {
  const variant: Base10Style = { ...CANONICAL };
  if (field === "ten") {
    variant.ten = "A";
  } else if (field === "eleven") {
    variant.eleven = "B";
  } else if (field === "mode") {
    variant.mode = "fixed";
  } else if (field === "scale") {
    variant.scale = 4;
  }
  const text = "11 + 0.1";
  const reference = cold(text, variant);
  convertBase10Text(text, CANONICAL); // poison the cache with the other config
  check(`the key covers field: ${field}`, convertBase10Text(text, variant), reference);
}

// A bounded, session-lived cache: fifty entries, the oldest one discarded.
clearBase10Cache();
for (let i = 0; i < 200; i += 1) {
  convertBase10Text(String(i), CANONICAL);
}
check("cache is bounded", base10CacheSize() <= 50, true);
check("cache kept the newest", convertBase10Text("199", CANONICAL).text, "147");

// A caller that mutates its result must not corrupt the stored entry.
clearBase10Cache();
const mine = convertBase10Text("12", CANONICAL);
mine.text = "tampered";
check("cache entry is not aliased", convertBase10Text("12", CANONICAL).text, "10");
clearBase10Cache();


// --- the other direction: base twelve -> base ten (section 2.4) -----------

/** What the swapped button would produce. */
function back(text: string, symbols: Base10Style = CANONICAL): string {
  return convertBase12Text(text, symbols).text;
}

check("back: the request, reversed", back("10 + ↋"), "12 + 11");
check("back: 100", back("100"), "144");
check("back: ↋", back("↋"), "11");
check("back: ↊", back("↊"), "10");
check("back: nothing to do", convertBase12Text("5 + 3", CANONICAL).changed, false);
// Beyond Number.MAX_SAFE_INTEGER, both ways.
check("back: round trip of a huge integer", back(button("9007199254740993", CANONICAL)), "9007199254740993");

// Fractions: `n / 12^k` terminates in base ten iff 3^k divides n.
check("back: 0;6", back("0;6"), "0.5");
check("back: 0;3", back("0;3"), "0.25");
check("back: 0;9", back("0;9"), "0.75");
check("back: 1;6", back("1;6"), "1.5");
check("back: 0;16", back("0;16"), "0.125");
check("back: terminates test, k=1", terminatesInBase10(6n, 1), true);
check("back: terminates test, k=1 no", terminatesInBase10(4n, 1), false);
check("back: terminates test, k=2", terminatesInBase10(4n, 2), false);
check("back: terminates test, k=2 yes", terminatesInBase10(9n * 5n, 2), true);
// A third does not terminate in base ten: the mode decides the shape.
check("back: a third, rational", back("0;4"), "1/3");
check("back: a third, fixed", back("0;4", FIXED), "0.333333333333");
check("back: a third, scale 4", back("0;4", FIXED_4), "0.3333");
check("back: a third, dfp is like fixed", back("0;4", DFP), "0.333333333333");
check(
  "back: the ellipsis is preview-only",
  convertBase12Text("0;4", FIXED).preview,
  "0.333333333333…",
);
check("back: a fraction inside an expression is parenthesised", back("1/0;4"), "1/(1/3)");
// Identifiers: a run of letters is a name unless every letter is a digit alias.
check("back: digits inside an identifier stay put", back("log12(100)"), "log12(144)");
check("back: a name is not a number", back("exp(10)", { ...CANONICAL, ten: "X", eleven: "E" }), "exp(12)");
check("back: all-alias letters are a number", back("XE", { ...CANONICAL, ten: "X", eleven: "E" }), "131");
check("back: leading zeros are preserved", back("007"), "007");
// The canonical glyphs are read even when another pair is configured.
check("back: canonical glyphs are always readable", back("↋", { ...CANONICAL, ten: "X", eleven: "E" }), "11");

// The two directions must not see each other's cache entries.
clearBase10Cache();
check("directions do not share entries: to12", convertBase10Text("11", CANONICAL).text, "↋");
check("directions do not share entries: to10", convertBase12Text("11", CANONICAL).text, "13");
check("directions do not share entries: to12 again", convertBase10Text("11", CANONICAL).text, "↋");
check("two directions, two entries", base10CacheSize(), 2);
for (const [label, text, symbols] of [
  ["integers", "10 + ↋", CANONICAL],
  ["fraction", "0;4", CANONICAL],
  ["truncated", "0;4", FIXED],
] as Array<[string, string, Base10Style]>) {
  clearBase10Cache();
  const reference = convertBase12Text(text, symbols);
  clearBase10Cache();
  convertBase12Text(text, symbols);
  check(`back: cache hit equals recompute: ${label}`, convertBase12Text(text, symbols), reference);
}
clearBase10Cache();

console.log(`\n${checks - failures}/${checks} passed`);
if (failures > 0) {
  throw new Error(`${failures} failing case(s)`);
}
