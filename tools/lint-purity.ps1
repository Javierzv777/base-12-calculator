#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Purity lint for the dozenal calculator (PLAN.md Section 0 / Section 8.4).

.DESCRIPTION
    Scans crates/dz-*/src for the two things PLAN.md's central invariant
    forbids:

      1. A native arithmetic/ordering trait ever implemented on `Digit`
         (`Add`, `Sub`, `Mul`, `Div`, `Rem`, `Ord`, `PartialOrd`,
         `From<u32>`, `From<u8>`). This must never exist anywhere, in any
         file -- there is no sanctioned exception for this one.

      2. `.tag()` -- the one function allowed to read a `Digit`'s raw `u8`
         -- being called outside the narrow set of files that are the
         *definition* of what a symbol means (a truth table has to be
         written down somewhere) or a *symbol-table* lookup (mapping the
         outside alphabet onto the tag space). Everywhere else, arithmetic
         on a digit's value must be expressed as composed `DozGates` calls.

      3. `str::parse::<...>()` / `.parse::<...>()` used inside `dz-parse`'s
         lexer, which would smuggle a value through native integer/float
         parsing instead of building it digit-by-digit (PLAN.md Section 5).

      4. Native `*`, `%`, or `/` operators anywhere in `crates/dz-*/src`
         outside `backends/` -- i.e. computing a product, remainder, or
         quotient natively instead of deriving it from `DozGates`
         primitives (`Dz::mul`, `Dz::div_rem`, ...). This is the rule that
         would have caught, on its own, an actual regression that shipped
         and was later fixed: `digit_mul_v2`'s SOP constant table
         (`LO[j][k]`/`HI[j][k]`) was originally computed with native
         `j * k; prod % RADIX; prod / RADIX` in
         `dz-gates/src/circuits/multiplier.rs` -- outside `backends/`, so
         rule #2 (`.tag()`) didn't catch it (no `.tag()` was called), but
         it was still native arithmetic silently encoding the meaning of
         multiplication. See `docs/01-diseno-sumador.md` ("objecion
         planteada, y resuelta") for the full story and the fix (the table
         is now synthesized by calling the already gate-composed
         `digit_mul_v1`, memoized once). This rule exists so the *next*
         version of this mistake gets caught by CI instead of by manual
         review.

.WHAT COUNTS AS ACCEPTABLE NATIVE ARITHMETIC (not flagged by this script)
    - Arithmetic on `usize`/loop counters/`Vec` indices/lengths (e.g. `i += 1`,
      `RADIX - j`, `v.len() - 1`). PLAN.md Section 0 explicitly allows this:
      "Se permite aritmetica nativa SOLO para: indices de Vec, contadores de
      bucle, y usize de longitudes." This script does not (and structurally
      cannot, via plain text matching) try to distinguish "arithmetic on a
      loop counter" from "arithmetic on a digit's value" in general -- that
      distinction is enforced by *never calling `.tag()`* outside the
      sanctioned files, which is exactly rule #2 above. If `.tag()` isn't
      called, there is no digit-value `u8` in scope to do forbidden
      arithmetic on in the first place.
    - Arithmetic inside `dz-gates/src/backends/{mvl,crt,table}.rs`: these
      files *are* the truth-table/oracle definitions of what each gate
      primitive and (for the table oracle) each digit-add/-multiply means.
      Something has to write down "successor of x is (x+1) mod 12"; this is
      the one sanctioned place. Nothing built on top of `DozGates`
      (adders, multipliers, `dz-int`, `dz-parse`, `dz-eval`, `dz-cli`) may
      do this.
    - `lo_hi_table()` in `dz-gates/src/circuits/multiplier.rs` (digit
      multiplier v2, SOP) used to compute its constant table with native
      `j * k` -- that was the bug rule #4 above exists to catch, and it has
      since been fixed to synthesize the table by calling `digit_mul_v1`
      (gate-composed) instead. It is *not* listed here as an accepted
      exception; it is exempt from rule #4 below only in the sense that it
      no longer contains the pattern rule #4 forbids. See
      `docs/01-diseno-sumador.md`.
    - `dz-parse/src/alphabet.rs`: converting between a `char` (the outside
      alphabet) and a `Digit` (the tag space) is a symbol-table lookup,
      analogous to `char::from_u32`, not a computation on a dozenal
      quantity.
    - Test code (`crates/*/tests/**`): PLAN.md Section 8.3 explicitly
      allows tests to reach into an external oracle (`num-bigint`) and to
      read digit tags to compare against it. Production code never does
      this, which is what this script checks (`src/`, not `tests/`).

.PHASE 5 ADDITION: dz_int::{small_usize_to_dz, dz_to_small_usize}
    Phase 5 (`DzRational`, `DzFloat`/DFP) needed to convert between a
    native `usize` (a DFP exponent, itself derived from `Vec` lengths --
    sanctioned bookkeeping) and the small `Dz` that represents it, without
    ever computing a digit's tag from that `usize` via `%`/`/` (which would
    just be redoing, in native arithmetic, exactly the job the gates exist
    to do). Both directions are implemented by *counting*, never by
    computing:
      - `small_usize_to_dz(g, k)`: adds `Dz::from_magnitude(.., [ONE])` to
        an accumulator `k` times via `Dz::add` (itself gate-composed).
      - `dz_to_small_usize(g, n, max)`: counts a probe `Dz` up from zero,
        comparing with `Dz::eq_value` (gate-composed) at each step, until
        it matches `n` or exceeds `max`.
    Neither function calls `.tag()` or applies a native operator to a
    digit's value -- `k`/`max`/the loop counter are plain `usize`, exactly
    the category PLAN.md Section 0 permits. This script doesn't need a new
    rule for this pattern (it doesn't call `.tag()`, so rule #2 already
    finds nothing to flag); this note exists so a reviewer auditing new
    Phase 5 code knows *why* a "convert usize to Dz" helper is safe instead
    of having to re-derive it. `DzFloat`'s biased exponent and mantissa are
    likewise stored and operated on as `Dz` (via its own `add`/`sub`/`mul`/
    `div_rem`/`compare`/`is_even`), never as native integers -- see
    `crates/dz-num/src/dfp.rs` and `docs/03-formato-dfp.md`.

.EXIT CODE
    0 if clean, 1 if any violation was found (suitable for CI).
#>

$ErrorActionPreference = 'Stop'

$RepoRoot = Split-Path -Parent $PSScriptRoot
$CratesRoot = Join-Path $RepoRoot 'crates'

# Files where `.tag()` is the sanctioned truth-table / symbol-table
# definition, not a purity violation. Paths are relative to $CratesRoot,
# forward-slash normalized.
$TagExemptFiles = @(
    'dz-gates/src/backends/mvl.rs',
    'dz-gates/src/backends/crt.rs',
    'dz-gates/src/backends/table.rs'
    'dz-parse/src/alphabet.rs'
)

# Directory (relative to $CratesRoot, forward-slash) exempt from rule #4
# (native `* % /`): the backend truth-table/oracle definitions. This is
# the *only* exemption for rule #4 -- unlike rule #2, there is no
# "symbol-table lookup" analog for arithmetic operators, since converting
# a char to a Digit (alphabet.rs) never needs to multiply/divide/mod.
$ArithmeticExemptDirPrefix = 'dz-gates/src/backends/'

function Normalize-RelPath([string]$FullPath) {
    $rel = $FullPath.Substring($CratesRoot.Length + 1)
    return $rel -replace '\\', '/'
}

$violations = New-Object System.Collections.Generic.List[string]

$srcFiles = Get-ChildItem -Path $CratesRoot -Recurse -Filter '*.rs' |
    Where-Object { $_.FullName -replace '\\','/' -match '/src/' }

# --- Rule 1: forbidden trait impls on Digit, anywhere, no exceptions. ---
$forbiddenImplPattern = 'impl\s+(std::ops::)?(Add|Sub|Mul|Div|Rem)(<[^>]*>)?\s+for\s+Digit\b|impl\s+(std::cmp::)?(Ord|PartialOrd)\s+for\s+Digit\b|impl\s+From<u32>\s+for\s+Digit\b|impl\s+From<u8>\s+for\s+Digit\b'

foreach ($file in $srcFiles) {
    $lines = Get-Content -LiteralPath $file.FullName
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match $forbiddenImplPattern) {
            $rel = Normalize-RelPath $file.FullName
            $violations.Add("[forbidden-trait-impl] ${rel}:$($i+1): $($lines[$i].Trim())")
        }
    }
}

# --- Rule 2: `.tag()` calls outside the sanctioned files. ---
foreach ($file in $srcFiles) {
    $rel = Normalize-RelPath $file.FullName
    if ($TagExemptFiles -contains $rel) {
        continue
    }
    $lines = Get-Content -LiteralPath $file.FullName
    for ($i = 0; $i -lt $lines.Length; $i++) {
        # Skip doc/comment lines mentioning `.tag()` as documentation, not a call.
        $trimmed = $lines[$i].Trim()
        if ($trimmed.StartsWith('//')) { continue }
        if ($lines[$i] -match '\.tag\(\)') {
            $violations.Add("[tag-outside-sanctioned-file] ${rel}:$($i+1): $($lines[$i].Trim())")
        }
    }
}

# --- Rule 3: native number parsing inside the dozenal lexer. ---
$lexerFile = Join-Path $CratesRoot 'dz-parse/src/lexer.rs'
if (Test-Path $lexerFile) {
    $lines = Get-Content -LiteralPath $lexerFile
    for ($i = 0; $i -lt $lines.Length; $i++) {
        $trimmed = $lines[$i].Trim()
        if ($trimmed.StartsWith('//')) { continue }
        if ($lines[$i] -match '\.parse::<\s*(i|u|f)(8|16|32|64|128|size)\s*>' -or
            $lines[$i] -match '\bstr::parse\b') {
            $violations.Add("[native-parse-in-lexer] crates/dz-parse/src/lexer.rs:$($i+1): $($lines[$i].Trim())")
        }
    }
}

# --- Rule 4: native `*`, `%`, `/` outside backends/. ---
# A binary operator needs an operand-like character (alnum, `)`, `]`) on
# both sides to count -- this excludes glob imports (`use foo::*;`, `*`
# preceded by `:`), char/string literals (`'/'`, preceded/followed by a
# quote), and doc-comment prose. Two things are stripped from each line
# before matching:
#   - trailing `// ...` comments (this codebase uses no `/* */` block
#     comments, verified by hand when this rule was added), so prose like
#     "// 0 * inf" on a real code line doesn't false-positive;
#   - double-quoted string literals (e.g. `println!("... 6/12 ...")`,
#     `"* (or x) /"` in `dz-cli`'s help text), so user-facing text
#     describing operators doesn't false-positive either.
$arithmeticPattern = '[A-Za-z0-9_)\]]\s*[*%/]\s*[A-Za-z0-9_(]'
$stringLiteralPattern = '"(?:[^"\\]|\\.)*"'

foreach ($file in $srcFiles) {
    $rel = Normalize-RelPath $file.FullName
    if ($rel.StartsWith($ArithmeticExemptDirPrefix)) {
        continue
    }
    $lines = Get-Content -LiteralPath $file.FullName
    for ($i = 0; $i -lt $lines.Length; $i++) {
        $line = $lines[$i]
        $commentIdx = $line.IndexOf('//')
        $code = if ($commentIdx -ge 0) { $line.Substring(0, $commentIdx) } else { $line }
        $code = [regex]::Replace($code, $stringLiteralPattern, '""')
        if ($code -match $arithmeticPattern) {
            $violations.Add("[native-arithmetic-outside-backends] ${rel}:$($i+1): $($line.Trim())")
        }
    }
}

if ($violations.Count -gt 0) {
    Write-Host "Purity lint FAILED: $($violations.Count) violation(s) found." -ForegroundColor Red
    foreach ($v in $violations) {
        Write-Host "  $v" -ForegroundColor Red
    }
    exit 1
}

Write-Host "Purity lint OK: no forbidden arithmetic on Digit values, no stray .tag() calls, no native parsing in the lexer, no native *, %, or / outside backends/." -ForegroundColor Green
exit 0
