// ---------------------------------------------------------------------------
// SEPARATE TOOL -- NOT PART OF THE CALCULATOR.
//
// This module is a text-editing helper for the input box: a string goes in, a
// string comes out. It rewrites runs of decimal digits as dozenal digits so a
// user who pasted `12 + 11` from somewhere base-10 can offer it to the real
// engine as `10 + E`. Design and rationale: `docs/08-conversion-base10.md`.
//
// It does its arithmetic with native `*`, `%` and `/` on `BigInt`. **If this
// were calculator code that would be a violation of the rule that governs the
// whole repository** (`CLAUDE.md`: no native arithmetic operator is ever applied
// to a dozenal value). It is not calculator code. Its output is never a result:
// it is text the user still has to evaluate with the gate engine, exactly like
// text they typed by hand. It is a find-and-replace, not a second way to
// compute.
//
// The no-mixing rule (`docs/08` section 1.2), which outranks every other
// consideration here:
//
//   * This file imports NOTHING. Not `api.ts`, not `alphabet.ts`, not
//     `glyphs.ts`, not any type that came from Rust. The pair of symbols the
//     interface is currently showing for ten and eleven arrives as an
//     ARGUMENT, never by importing the alphabet module.
//   * Nothing in the calculator may call into this module, and this module may
//     not call into anything in the calculator. The only link that exists is a
//     `paste` handler and a button in `main.ts` that hand it a string and, if
//     the user clicks, write the answer into the input's `value`. That link is
//     UI, not semantics.
//   * If some day this conversion is needed *inside* a computation, the correct
//     answer is not to import this file: it is to build it out of gates in
//     `dz-int`, the way the project's principle demands.
// ---------------------------------------------------------------------------

/**
 * The three number modes the session can be in. Plain strings, chosen to match
 * what the session happens to call them -- the module has no opinion about the
 * calculator's modes and holds no reference to its types.
 */
export type Base10Mode = "fixed" | "rational" | "dfp";

/**
 * How the interface is spelling numbers right now. Everything the module needs
 * to know about the calculator arrives here, as an ARGUMENT (`docs/08` section
 * 1.2): the two symbols, the session's mode, and its working scale.
 */
export interface Base10Style {
  /** The characters the interface is currently drawing for ten and eleven. */
  ten: string;
  eleven: string;
  /** The session's number mode; decides the shape of a non-terminating decimal. */
  mode: Base10Mode;
  /** Digits after the Humphrey point when the mode truncates. */
  scale: number;
}

/** What a conversion did, so the button can say it out loud. */
export interface Base10Conversion {
  /**
   * The whole input text, rewritten. This is what goes into the input box, and
   * it never contains an ellipsis.
   */
  text: string;
  /**
   * The same thing for showing in the popup, with `…` after a truncated
   * expansion. `…` is not a character the lexer accepts, so it must never
   * reach the input -- it exists only to show that the number goes on.
   */
  preview: string;
  /** False when the rewrite is character-for-character the original. */
  changed: boolean;
  /** The source carried dozenal-only symbols, which were left untouched. */
  keptDozenal: boolean;
  /** Some decimal terminated in base twelve and became a Humphrey number. */
  madeHumphrey: boolean;
  /** Some decimal did not terminate and became an exact fraction. */
  madeFraction: boolean;
  /** Some decimal did not terminate and was truncated to the session's scale. */
  madeTruncated: boolean;
}


const DECIMAL_DIGITS = "0123456789";


/**
 * A decimal integer, optionally written with three-digit thousands groups.
 * Dozenal groups by four, so the three-group shape is a base-10 tell.
 */
const GROUPED_NUMBER = /^[0-9]{1,3}(?:,[0-9]{3})+/;

/** A maximal run of letters and digits that starts with a letter. */
const IDENTIFIER = /^[A-Za-z][A-Za-z0-9]*/;

/**
 * One non-negative integer, decimal to dozenal, by repeated division.
 *
 * Native `BigInt` arithmetic: exact at any size, and out of scope of the
 * repository's purity rule for the reason given in the header.
 */
function digitsOf(value: bigint, symbols: Base10Style): string {
  const alphabet = DECIMAL_DIGITS + symbols.ten + symbols.eleven;
  if (value === 0n) {
    return "0";
  }
  let rest = value;
  let out = "";
  while (rest > 0n) {
    out = alphabet[Number(rest % 12n)] + out;
    rest = rest / 12n;
  }
  return out;
}

function gcd(a: bigint, b: bigint): bigint {
  let x = a;
  let y = b;
  while (y !== 0n) {
    const t = x % y;
    x = y;
    y = t;
  }
  return x;
}

function power(base: bigint, exponent: number): bigint {
  let out = 1n;
  for (let i = 0; i < exponent; i += 1) {
    out = out * base;
  }
  return out;
}

/**
 * `n / 10^k` terminates in base twelve if and only if `5^k` divides `n`.
 *
 * `10^k = 2^k · 5^k` and `12 = 2²·3` carries no five, so the only way the
 * denominator's fives can disappear is for the numerator to already contain
 * them. Exact and cheap -- no expansion is attempted to find out.
 */
export function terminatesInBase12(numerator: bigint, tenthsExponent: number): boolean {
  return numerator % power(5n, tenthsExponent) === 0n;
}

/** What one decimal literal turned into, and by which of the three routes. */
type DecimalOutcome = {
  text: string;
  kind: "exact" | "humphrey" | "fraction" | "truncated";
};

/**
 * `n / 10^k` written out in base twelve, cut after `scale` places.
 *
 * The expansion is the schoolbook one: multiply the remainder by twelve, take
 * the quotient as the next digit. Truncated, not rounded -- the fixed mode of
 * the calculator truncates to its scale too, and this text is going to be fed
 * back into it.
 */
function expandTruncated(n: bigint, d: bigint, scale: number, symbols: Base10Style): string {
  const alphabet = DECIMAL_DIGITS + symbols.ten + symbols.eleven;
  let body = digitsOf(n / d, symbols);
  let rest = n % d;
  const places: string[] = [];
  for (let i = 0; i < scale && rest !== 0n; i += 1) {
    rest = rest * 12n;
    places.push(alphabet[Number(rest / d)]);
    rest = rest % d;
  }
  if (places.length > 0) {
    body = `${body};${places.join("")}`;
  }
  return body;
}

/**
 * A decimal literal, rewritten. Three outcomes:
 *
 *   * it terminates in base twelve -> the dozenal number with its Humphrey
 *     point (`0.5` is `0;6`), exact, whatever the mode. The finite division is
 *     done by multiplying by powers of twelve until the denominator divides
 *     cleanly;
 *   * it does not, and the mode is `rational` -> the exact fraction (`0.1` is
 *     `1/↊`). Still a valid expression, and the calculator evaluates it with
 *     gates without losing anything;
 *   * it does not, and the mode is `fixed` or `dfp` -> the expansion truncated
 *     to the session's scale (`0.1` is `0;124972497249`). Those modes already
 *     truncate to that scale everywhere else in the interface; being exact here
 *     and truncated on evaluation would be the incoherent choice.
 *
 * `docs/08` section 3.2. The `…` that shows the number goes on is added by the
 * caller, for the popup only -- the lexer does not accept that character.
 */
function convertDecimal(
  integerPart: string,
  fractionPart: string,
  symbols: Base10Style,
): DecimalOutcome {
  const k = fractionPart.length;
  const n = BigInt(integerPart + fractionPart);
  const d = power(10n, k);

  if (!terminatesInBase12(n, k)) {
    if (symbols.mode === "rational") {
      const factor = gcd(n, d);
      return {
        text: `${digitsOf(n / factor, symbols)}/${digitsOf(d / factor, symbols)}`,
        kind: "fraction",
      };
    }
    return { text: expandTruncated(n, d, symbols.scale, symbols), kind: "truncated" };
  }

  // Smallest m with `d | n · 12^m`. It always exists and never exceeds k:
  // the fives are gone by hypothesis and `2^k` divides `12^k`.
  let scaled = n;
  let places = 0;
  while (scaled % d !== 0n) {
    scaled = scaled * 12n;
    places += 1;
  }
  let body = digitsOf(scaled / d, symbols);
  if (places === 0) {
    return { text: body, kind: "exact" };
  }
  // The symbols may be multi-character, so count symbols, not code units.
  const glyphs = [...body];
  while (glyphs.length <= places) {
    glyphs.unshift("0");
  }
  body = glyphs.slice(0, glyphs.length - places).join("");
  return { text: `${body};${glyphs.slice(glyphs.length - places).join("")}`, kind: "humphrey" };
}

/** True when the text carries a symbol that only base twelve uses. */
function hasDozenalSymbol(text: string, symbols: Base10Style): boolean {
  for (const marker of ["↊", "↋", ";", symbols.ten, symbols.eleven]) {
    if (marker.length > 0 && text.includes(marker)) {
      return true;
    }
  }
  return false;
}

/**
 * Rewrite every standalone decimal number in `text` as a dozenal one.
 *
 * Left untouched: digits inside an identifier (`log12` would break silently if
 * its `12` were rewritten -- no catalog name has digits today, this is for the
 * day one does), every dozenal glyph already present, and every operator,
 * parenthesis and space, copied verbatim. Leading zeros are preserved so `007`
 * is not "changed" into `7`.
 *
 * This is the button's whole job: no heuristic, no conditions. It converts what
 * it can and reports what it did.
 */
function computeBase10Text(text: string, symbols: Base10Style): Base10Conversion {
  // A bare literal keeps the documented shape (`0.1` -> `1/↊`); inside a larger
  // expression the fraction is parenthesised, because `1/0.1` must not become
  // `1/1/↊`, which the parser reads left to right as `(1/1)/↊`.
  const bare = /^\s*[0-9]+\.[0-9]+\s*$/.test(text);
  let out = "";
  // Built in step with `out` and differing from it only where an expansion was
  // cut: `…` shows the reader the number goes on and must never reach the box.
  let preview = "";
  let index = 0;
  let madeHumphrey = false;
  let madeFraction = false;
  let madeTruncated = false;

  /** Append the same text to both, which is what almost everything does. */
  function emit(piece: string): void {
    out += piece;
    preview += piece;
  }

  while (index < text.length) {
    const rest = text.slice(index);

    const identifier = IDENTIFIER.exec(rest);
    if (identifier) {
      emit(identifier[0]);
      index += identifier[0].length;
      continue;
    }

    const character = text[index];
    if (character >= "0" && character <= "9") {
      const grouped = GROUPED_NUMBER.exec(rest);
      let literal: string;
      if (grouped && !/^[0-9]/.test(rest.slice(grouped[0].length))) {
        literal = grouped[0];
      } else {
        literal = /^[0-9]+/.exec(rest)![0];
      }
      index += literal.length;
      const plain = literal.split(",").join("");

      const fraction = /^\.([0-9]+)/.exec(text.slice(index));
      if (fraction) {
        index += fraction[0].length;
        const converted = convertDecimal(plain, fraction[1], symbols);
        if (converted.kind === "fraction") {
          madeFraction = true;
          // A bare literal keeps the documented shape; inside a larger
          // expression the fraction needs its parentheses.
          emit(bare ? converted.text : `(${converted.text})`);
        } else if (converted.kind === "truncated") {
          madeTruncated = true;
          out += converted.text;
          preview += `${converted.text}…`;
        } else {
          madeHumphrey = madeHumphrey || converted.kind === "humphrey";
          emit(converted.text);
        }
        continue;
      }

      // A run of zeros in front carries no value in either base; converting it
      // would rewrite `007` as `7` and make an unchanged text look changed.
      const zeros = /^0*/.exec(plain)![0];
      const significant = plain.slice(zeros.length);
      emit(
        significant.length === 0
          ? zeros
          : zeros + digitsOf(BigInt(significant), symbols),
      );
      continue;
    }

    emit(character);
    index += 1;
  }

  return {
    text: out,
    preview,
    changed: out !== text,
    keptDozenal: hasDozenalSymbol(text, symbols),
    madeHumphrey,
    madeFraction,
    madeTruncated,
  };
}

// ---------------------------------------------------------------------------
// The other direction: base twelve -> base ten (`docs/08` section 2.4).
//
// Same tool, same rules, mirrored. It exists because the swap control next to
// the button flips which way the helper reads the box, the way a translator
// swaps its two languages. The output is still text in the input box and never
// a result -- more plainly so than before, in fact: a base-ten expression is
// not something this calculator can evaluate, so this direction is purely a
// reading aid.
// ---------------------------------------------------------------------------

/**
 * The value of one dozenal glyph, or null when the character is not one.
 *
 * Both the configured pair and the canonical glyphs are accepted: the box may
 * hold text written before the alphabet was changed, and being generous about
 * reading a digit costs nothing. Writing is not generous -- output always uses
 * the configured pair.
 */
function dozenalValue(character: string | undefined, symbols: Base10Style): number | null {
  if (character === undefined) {
    return null;
  }
  const decimal = DECIMAL_DIGITS.indexOf(character);
  if (decimal >= 0) {
    return decimal;
  }
  if (character === symbols.ten || character === "↊") {
    return 10;
  }
  if (character === symbols.eleven || character === "↋") {
    return 11;
  }
  return null;
}

/** A run of dozenal glyphs read as one non-negative integer. */
function dozenalNumber(digits: string, symbols: Base10Style): bigint {
  let value = 0n;
  for (const character of digits) {
    value = value * 12n + BigInt(dozenalValue(character, symbols) ?? 0);
  }
  return value;
}

/**
 * `n / 12^k` terminates in base ten if and only if `3^k` divides `n`.
 *
 * The mirror of `terminatesInBase12`, true for the mirror reason:
 * `12^k = 2^2k * 3^k` and `10 = 2*5` carries no three, so the only way the
 * denominator's threes can disappear is for the numerator to contain them.
 * `0;6` is `0.5`, but `0;4` is one third and never terminates.
 */
export function terminatesInBase10(numerator: bigint, twelfthsExponent: number): boolean {
  return numerator % power(3n, twelfthsExponent) === 0n;
}

/** `n / d` in base ten, cut after `scale` places. Truncated, never rounded. */
function expandTruncatedBase10(n: bigint, d: bigint, scale: number): string {
  let body = (n / d).toString(10);
  let rest = n % d;
  const places: string[] = [];
  for (let i = 0; i < scale && rest !== 0n; i += 1) {
    rest = rest * 10n;
    places.push((rest / d).toString(10));
    rest = rest % d;
  }
  if (places.length > 0) {
    body = `${body}.${places.join("")}`;
  }
  return body;
}

/**
 * One dozenal literal, rewritten in base ten. The three outcomes of section 3.2
 * again, mirrored: an exact decimal point, an exact fraction in rational mode,
 * or the expansion truncated to the session's scale.
 */
function convertDozenal(
  integerPart: string,
  fractionPart: string,
  symbols: Base10Style,
): DecimalOutcome {
  const k = fractionPart.length;
  const n = dozenalNumber(integerPart + fractionPart, symbols);
  const d = power(12n, k);

  if (!terminatesInBase10(n, k)) {
    if (symbols.mode === "rational") {
      const factor = gcd(n, d);
      return {
        text: `${(n / factor).toString(10)}/${(d / factor).toString(10)}`,
        kind: "fraction",
      };
    }
    return { text: expandTruncatedBase10(n, d, symbols.scale), kind: "truncated" };
  }

  // Smallest m with `d | n * 10^m`. It exists and never exceeds 2k: the threes
  // are gone by hypothesis and `2^2k` divides `10^2k`.
  let scaled = n;
  let places = 0;
  while (scaled % d !== 0n) {
    scaled = scaled * 10n;
    places += 1;
  }
  let body = (scaled / d).toString(10);
  if (places === 0) {
    return { text: body, kind: "exact" };
  }
  while (body.length <= places) {
    body = `0${body}`;
  }
  return {
    text: `${body.slice(0, body.length - places)}.${body.slice(body.length - places)}`,
    kind: "humphrey",
  };
}

/**
 * Rewrite every standalone dozenal number in `text` in base ten.
 *
 * The mirror of `computeBase10Text`, with the same exclusions: digits inside an
 * identifier stay put, operators and spaces are copied verbatim, leading zeros
 * are preserved. Identifiers follow the lexer's own rule (`CLAUDE.md`): a
 * maximal run of letters is a name only if some letter of it is NOT a digit
 * alias, so with the `X`/`E` pair `XE` is a number and `exp` is a name.
 */
function computeBase12Text(text: string, symbols: Base10Style): Base10Conversion {
  const isDigit = (character: string | undefined): boolean =>
    dozenalValue(character, symbols) !== null;
  // A bare literal keeps the documented shape; inside a larger expression a
  // fraction is parenthesised, for the reason given on the way out.
  const trimmed = text.trim();
  const bare =
    trimmed.split(";").length === 2 &&
    [...trimmed].every((character) => character === ";" || isDigit(character));

  let out = "";
  let preview = "";
  let index = 0;
  let madeHumphrey = false;
  let madeFraction = false;
  let madeTruncated = false;

  function emit(piece: string): void {
    out += piece;
    preview += piece;
  }

  while (index < text.length) {
    const identifier = IDENTIFIER.exec(text.slice(index));
    // A run of letters is a name only when one of its letters is not a digit
    // alias -- which is what makes `XE` a number when the pair is `X`/`E`.
    if (identifier && ![...identifier[0]].every(isDigit)) {
      emit(identifier[0]);
      index += identifier[0].length;
      continue;
    }

    if (isDigit(text[index])) {
      let literal = "";
      while (isDigit(text[index + literal.length])) {
        literal += text[index + literal.length];
      }
      index += literal.length;

      let fraction = "";
      if (text[index] === ";" && isDigit(text[index + 1])) {
        index += 1;
        while (isDigit(text[index + fraction.length])) {
          fraction += text[index + fraction.length];
        }
        index += fraction.length;
      }

      if (fraction.length > 0) {
        const converted = convertDozenal(literal, fraction, symbols);
        if (converted.kind === "fraction") {
          madeFraction = true;
          emit(bare ? converted.text : `(${converted.text})`);
        } else if (converted.kind === "truncated") {
          madeTruncated = true;
          out += converted.text;
          preview += `${converted.text}…`;
        } else {
          madeHumphrey = madeHumphrey || converted.kind === "humphrey";
          emit(converted.text);
        }
        continue;
      }

      const zeros = /^0*/.exec(literal)![0];
      const significant = literal.slice(zeros.length);
      emit(
        significant.length === 0
          ? zeros
          : zeros + dozenalNumber(significant, symbols).toString(10),
      );
      continue;
    }

    emit(text[index]);
    index += 1;
  }

  return {
    text: out,
    preview,
    changed: out !== text,
    // Only meaningful the other way round: nothing here has to be left behind
    // for being unreadable in base ten.
    keptDozenal: false,
    madeHumphrey,
    madeFraction,
    madeTruncated,
  };
}

// ---------------------------------------------------------------------------
// Conversion cache (`docs/08` section 2.2).
//
// It lives inside this module, not at the connection point, for the same reason
// everything else about the module does: it is self-contained, so every caller
// -- the button, the popup heuristic, the tests -- gets it with no wiring at
// all, and the connection point keeps knowing nothing about how a conversion is
// produced. Keeping it here also keeps the no-mixing rule intact: still zero
// imports, still string in / string out.
//
// It is an OPTIMISATION AND NEVER A SOURCE OF TRUTH: a hit must return exactly
// what recomputing would return, which is what makes the shape of the key the
// whole point of this section.
// ---------------------------------------------------------------------------

/** Session-lived, in memory, never persisted. The oldest entry is evicted. */
const CACHE_LIMIT = 50;

const cache = new Map<string, Base10Conversion>();

/**
 * The cache key is the WHOLE INPUT TUPLE: the direction, the text, and every
 * field of `Base10Style`, without exception.
 *
 * The direction is part of it for the same reason everything else is: `11` is
 * `↋` one way and `13` the other, so the two directions must not be able to
 * see each other's entries.
 *
 * Keying on the text alone would be a poisoned cache, and a silent one: `0.1`
 * is a fraction in rational mode and a truncated expansion in fixed with as
 * many digits as the scale says, and the symbols for ten and eleven are
 * whatever the interface is configured to draw. A text-only key would serve one
 * configuration a correct answer computed for another.
 *
 * The fields are read off the object itself (sorted, so key order cannot
 * matter) instead of being listed by hand, so a field added to `Base10Style`
 * tomorrow joins the key on its own. Forgetting it is not possible.
 */
function cacheKey(direction: string, text: string, symbols: Base10Style): string {
  const fields = (Object.keys(symbols) as Array<keyof Base10Style>).sort();
  return JSON.stringify([direction, text, fields.map((name) => [name, symbols[name]])]);
}

/**
 * Look the answer up, or compute and remember it.
 *
 * The `compute*` functions above are what the rewrites actually are; this adds
 * nothing but the lookup. The stored entry is copied on the way out, so a
 * caller that mutates its result cannot corrupt anyone else's.
 */
function cached(
  direction: string,
  text: string,
  symbols: Base10Style,
  compute: (text: string, symbols: Base10Style) => Base10Conversion,
): Base10Conversion {
  const key = cacheKey(direction, text, symbols);
  const hit = cache.get(key);
  if (hit) {
    // Re-insert so the entry counts as recently used and survives eviction.
    cache.delete(key);
    cache.set(key, hit);
    return { ...hit };
  }
  const conversion = compute(text, symbols);
  cache.set(key, conversion);
  if (cache.size > CACHE_LIMIT) {
    // A Map iterates in insertion order, so the first key is the oldest one.
    const oldest = cache.keys().next().value;
    if (oldest !== undefined) {
      cache.delete(oldest);
    }
  }
  return { ...conversion };
}

export function convertBase10Text(text: string, symbols: Base10Style): Base10Conversion {
  return cached("to12", text, symbols, computeBase10Text);
}

/**
 * The same, the other way round: every dozenal number in `text` rewritten in
 * base ten (`docs/08` section 2.4). Same cache, same key, different direction.
 */
export function convertBase12Text(text: string, symbols: Base10Style): Base10Conversion {
  return cached("to10", text, symbols, computeBase12Text);
}

/** Empties the cache. Exists so the tests can compare a hit against a recompute. */
export function clearBase10Cache(): void {
  cache.clear();
}

/** How many entries the cache is holding. Tests only. */
export function base10CacheSize(): number {
  return cache.size;
}
