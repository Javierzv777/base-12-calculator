---
name: dz-cli
description: >
  Drive the dozenal (base 12) calculator from the terminal: evaluate
  expressions, count gates and circuit depth, trace a ripple-carry addition,
  subtraction or multiplication digit by digit, compare the three gate
  backends, and export any of it as CSV or JSON. Use whenever a question in
  this repo is about what a dozenal computation *costs* or *how it runs*,
  rather than about the source code.
---

# dz-cli

The terminal interface to this repository's dozenal calculator. Everything
below was run against the built binary; the outputs are copied from real
runs, not reconstructed.

## 1. How to invoke it

`cargo` is not on the PATH:

```powershell
$env:PATH += ";$env:USERPROFILE\.cargo\bin"     # powershell
export PATH="$PATH:/c/Users/ADMIN/.cargo/bin"   # bash
```

Then either of:

```bash
cargo run -p dz-cli --release -- <args>
target/release/dz-cli <args>          # after one build
```

**Use `--release`.** In a debug build a transcendental function takes
minutes; in release `sin(1)` at scale 12 returns in well under a second.

With no arguments the binary starts an interactive REPL instead. An agent
wants the one-shot form.

## 2. The alphabet

- Digits are `0`-`9` plus `X` (ten) and `E` (eleven). `T` is an accepted
  alias for ten.
- `;` is the Humphrey point: `4;6` is four and a half.
- Output uses the ASCII aliases by default (a pipe or a CI log is exactly
  where U+218A/U+218B fails to render); `--unicode` prints the glyphs
  `↊`/`↋`. **`--json` and `--csv` always carry the canonical glyphs**,
  because those documents are built by `dz-api` and the CLI never rewrites
  them.

Traps to avoid:

- `e` is **not** Euler's number — it is the digit eleven. Write `euler`.
  For the same reason `x` and `t` cannot be function names.
- Inside a function call the comma separates arguments, so write `10000`,
  never the grouped form `1,0000`. Outside a call, `1,0000` is the
  four-digit grouping separator.
- `^` takes non-negative integer exponents only. Anything else is written
  `exp(y * ln(x))`.

### `--alphabet <XY>`: choosing the two symbols

`--alphabet` picks the pair of characters that stand for ten and eleven,
for typing *and* for printing. It changes presentation only — the
calculation, the JSON and the CSV are untouched.

```
$ dz-cli eval "ab" --alphabet ab --mode fixed
ab

$ dz-cli eval "abs(-3)" --alphabet ab
3

$ dz-cli eval "b" --alphabet ab --mode fixed --json
{"ok":true,"display":"↋","periodic_note":false,"approximate":false,"gate_count":{"cycle":3696,"min":6764,"max":4996,"lit":3396,"thr":840,"total":19692},"depth":254,"reconstruction_ops":0}
```

Note the third one: the terminal shows `b`, the JSON shows `↋`. Exchange
documents keep the canonical alphabet on purpose, so an export never
depends on who ran it. The same holds for `--csv`.

`abs` still works because the pair is applied with the lexer's own rule: a
maximal run of letters is a *number* only if every letter is a digit, so
`ab` is ten-eleven and `abs` is a function name.

A pair that would break a catalog name is refused, and the message says
which name:

```
$ dz-cli eval "1+1" --alphabet pi
usage error: the pair 'p'/'i' would make 'pi' read as a number instead of a name, so the function 'pi' could never be called
try 'dz-cli --help'
```

With a pair configured, the legacy aliases stop being digits — the
canonical glyphs never do:

```
$ dz-cli eval "E" --alphabet ab
error: 'E' is not a number in the current alphabet: ten is 'a', eleven is 'b' (the canonical glyphs ↊ and ↋ are always accepted too)

$ dz-cli eval "↊+↋" --alphabet ab --mode fixed
19
```

In the REPL the same thing is `:alphabet ab`; bare `:alphabet` reports the
current pair. Changing it repaints `:history` and `:mem`, because those
hold canonical text.

## 3. Recipes

Every command below was executed; the output shown is what it printed.

### "What is 4;6 x 3?"

```
$ dz-cli eval "4;6 * 3" --mode fixed
11;6
```

### "What does `sin(1)` cost?"

```
$ dz-cli eval "sin(1)" --mode fixed --scale 12 --gates
~0;X1208XX92234
gates: cycle=5743188 min=9039834 max=6706775 lit=4594776 thr=1054968 total=27139541 depth=18397 recon=0
```

### "How does the carry propagate through `EEE + 1`?"

```
$ dz-cli trace add EEE 1
trace: add  EEE + 1 = 1000
  pos  a  b  cin  sum  cout
    0  E  1   0    0     1
    1  E  0   1    0     1
    2  E  0   1    0     1
result: 1000
```

### "How is a subtraction actually performed?"

There is no subtractor circuit; `sub` shows the twelve's-complement
strategy that really runs, stage by stage.

```
$ dz-cli trace sub 1 1000
trace: sub  1 - 1000 = -EEE
  (there is no subtractor circuit: this is the twelve's-complement strategy)
  stage 1  elevens' complement of b = XEEE
  stage 2  add 1 to it:
    pos  a  b  cin  sum  cout
      0  E  1   0    0     1
      1  E  0   1    0     1
      2  E  0   1    0     1
      3  X  0   1    E     0
  stage 2  twelve's complement of b = E000
  stage 3  a + twelve's complement of b:
    pos  a  b  cin  sum  cout
      0  1  0   0    1     0
      1  0  0   0    0     0
      2  0  0   0    0     0
      3  0  E   0    E     0
  stage 3  raw sum = E001
  borrowed: yes (a < b, so the raw sum is decoded)
  stage 4  elevens' complement of the raw sum = 0EEX
  stage 5  add 1 to it:
    pos  a  b  cin  sum  cout
      0  X  1   0    E     0
      1  E  0   0    E     0
      2  E  0   0    E     0
      3  0  0   0    0     0
result: -EEE
```

### "How does schoolbook multiplication run?"

```
$ dz-cli trace mul 25 34
trace: mul  25 * 34 = 808
  row 0  multiplier digit 4  partial 98  shifted 98
    pos  a  b  cin  sum  cout
      0  0  8   0    8     0
      1  0  9   0    9     0
    accumulated = 98
  row 1  multiplier digit 3  partial 73  shifted 730
    pos  a  b  cin  sum  cout
      0  8  0   0    8     0
      1  9  3   0    0     1
      2  0  7   1    8     0
    accumulated = 808
result: 808
```

### "Which backend is cheaper?"

```
$ dz-cli compare mul 25 34
result: 808
  backend     cycle      min      max      lit      thr      total    depth    recon
  mvl         16896    28156    20923    14376     3576      83927      441        0
  crt         16896    28156    20923    14376     3576      83927      441   118630
  table       16896    28156    20923    14376     3576      83927      441        0
  (table is the truth-table oracle, not a production backend)
  verdict: mvl is cheapest. mvl and crt tie on all five primitive counters
  (83927) and on depth (441), so the five headline numbers cannot separate
  them -- they are equal by construction, since every backend implements the
  same primitives for the same circuit. The tie is broken by
  reconstructions: mvl 0, crt 118630. Cost model: primitive invocations +
  internal->tag reconstructions, ranked lexicographically by (total, depth,
  reconstructions). The table backend is listed but not ranked: it is the
  truth-table oracle, not a production backend.
```

The five primitive columns are identical across the three rows, and that
is a property of the design rather than a measurement: `GateCount` counts
*invocations of the `DozGates` primitives*, and the three backends
implement the same primitives for the same circuit. **Never report that as
"the three backends cost the same."** The column that separates them is
`recon` — the digit reconstructions CRT needs before every order
comparison, because `Z12 = Z4 x Z3` destroys the order
(`docs/02-crt-vs-mvl.md`). MVL and the oracle store the tag itself, so
their `0` is the truth, not a missing number.

The `verdict:` line is written by `dz-api` (`dz_api::verdict`), not by the
CLI, and the GUI's research panel prints the same sentence. Quote it
rather than composing your own reading. What it does and does not claim:

- It ranks **production backends only**. `table` is listed but excluded:
  it is the truth-table oracle and indexes memory instead of composing
  gates, so "cheaper than the oracle" is not a research answer.
- It states the **cost model** out loud — primitive invocations plus
  internal->tag reconstructions, ordered lexicographically by
  `(total, depth, reconstructions)`. There is **no weighted formula**:
  no measured exchange rate exists between a gate invocation and a
  reconstruction.
- When the ranked backends tie on all five counters and on depth (which,
  for every circuit in this repo, they do) it says so and names the
  reconstructions as the tiebreak.

A plain call reports the same number, so you can ask about one backend
without running a comparison:

```
$ dz-cli mul 25 34 --mode fixed --gates --backend crt
808
gates: cycle=18084 min=30334 max=22512 lit=15432 thr=3792 total=90154 depth=441 recon=127568
```

### "Give me the numbers as a file"

```
$ dz-cli trace mul 25 34 --csv
kind,stage,group,position,a,b,carry_in,sum,carry_out
mul,accumulate,0,0,0,8,0,8,0
mul,accumulate,0,1,0,9,0,9,0
mul,accumulate,1,0,8,0,0,8,0
mul,accumulate,1,1,9,3,0,0,1
mul,accumulate,1,2,0,7,1,8,0
```

```
$ dz-cli trace add EEE 1 --csv-summary
kind,field,value
add,a,↋↋↋
add,b,1
add,result,1000
```

`--csv` on a `compare` emits the gate table instead:

```
$ dz-cli compare sum 1 3 --csv
backend,cycle,min,max,lit,thr,total,depth,reconstruction_ops,oracle
mvl,6204,9910,7398,5124,1308,29944,441,0,false
crt,6204,9910,7398,5124,1308,29944,441,42128,false
table,6204,9910,7398,5124,1308,29944,441,0,true
```

Add a path to write a file instead of printing: `--csv=out.csv`,
`--csv-summary=context.csv`. The attached-value spelling is required —
the operands are positional, so `--csv 5` would be ambiguous.

### "What can it compute?"

```
$ dz-cli --functions
Callable functions and constants:
  [constant]
    pi                 approx  ratio of a circle's circumference to its diameter
    tau                approx  a full turn, two pi
    euler              approx  base of the natural logarithm (spelled out: 'e' is the digit eleven)
  [rounding]
    abs     (1 arg)    exact   absolute value
    ...
```

Any listed name is also a subcommand: `dz-cli sqrt 2 --mode fixed`.

## 4. Use `--json` when the output is for you

Plain text is for a human reading a terminal. When an agent will parse the
result, add `--json`: one line, no trailing prose.

```
$ dz-cli sum 1 3 --json
{"ok":true,"display":"4","periodic_note":false,"approximate":false,"gate_count":{"cycle":6204,"min":9910,"max":7398,"lit":5124,"thr":1308,"total":29944},"depth":441,"reconstruction_ops":0}
```

```
$ dz-cli trace add EEE 1 --json
{"ok":true,"kind":"add","a":"↋↋↋","b":"1","steps":[{"position":0,"a":"↋","b":"1","carry_in":"0","sum":"0","carry_out":"1"},{"position":1,"a":"↋","b":"0","carry_in":"1","sum":"0","carry_out":"1"},{"position":2,"a":"↋","b":"0","carry_in":"1","sum":"0","carry_out":"1"}],"result":"1000"}
```

```
$ dz-cli compare mul 25 34 --json
{"ok":true,"display":"808","costs":[{"backend":"mvl","oracle":false,"gate_count":{"cycle":16896,"min":28156,"max":20923,"lit":14376,"thr":3576,"total":83927},"depth":441,"reconstruction_ops":0},{"backend":"crt","oracle":false,"gate_count":{"cycle":16896,"min":28156,"max":20923,"lit":14376,"thr":3576,"total":83927},"depth":441,"reconstruction_ops":118630},{"backend":"table","oracle":true,"gate_count":{"cycle":16896,"min":28156,"max":20923,"lit":14376,"thr":3576,"total":83927},"depth":441,"reconstruction_ops":0}],"verdict":{"winner":"mvl","decided_by_reconstructions":true,"model":"primitive invocations + internal->tag reconstructions, ranked lexicographically by (total, depth, reconstructions)","summary":"verdict: mvl is cheapest. mvl and crt tie on all five primitive counters (83927) and on depth (441), so the five headline numbers cannot separate them -- they are equal by construction, since every backend implements the same primitives for the same circuit. The tie is broken by reconstructions: mvl 0, crt 118630. Cost model: primitive invocations + internal->tag reconstructions, ranked lexicographically by (total, depth, reconstructions). The table backend is listed but not ranked: it is the truth-table oracle, not a production backend."}}
```

Errors keep the same shape: `{"ok":false,"error":"..."}`. Exit codes are
`0` success, `1` calculation error, `2` usage error.

Every value in these documents is text already formatted by `dz-parse`.
You can read and quote it; you cannot do dozenal arithmetic with it. Ask
the binary instead — that is the point.

## 5. How to read the numbers

- **Gates and depth are different measurements** taken in the same pass.
  `total` is how many primitive gate invocations the call made; `depth` is
  the longest chain of primitives, the model in
  `dz_gates::circuits::depth` (input = level 0, each primitive =
  `1 + max(inputs)`). Cheaper is not shallower.
- **`pi` reports 0 gates on purpose.** Constants are synthesized once in a
  throwaway backend and memoized, so a user's measurement never depends on
  whether their call happened to be the first. Depth is still reported:

  ```
  $ dz-cli eval "pi" --mode fixed --scale 6 --gates
  ~3;184809
  gates: cycle=0 min=0 max=0 lit=0 thr=0 total=0 depth=14568 recon=0
  ```
- **`recon` is a third measurement, not part of `total`.** It counts full
  internal-representation-to-tag reconstructions
  (`DozGates::reconstruction_ops`). It is `0` on `mvl` and on `table`
  because their internal representation already *is* the tag — that zero
  is the truth, not an unmeasured quantity — and non-zero only on `crt`.
  It is the only number that distinguishes the backends, since the five
  primitive counters are equal across them by construction.
- **`table` is the oracle**, a truth-table definition used to check the
  others. Its numbers are real but they are not a "which backend to build"
  answer, and `compare`'s verdict excludes it from the ranking for exactly
  that reason.
- **A leading `~` means approximate**: a truncated series at the working
  scale, not an exact value. `--scale` sets the number of fractional
  dozenal digits.
- **A trace reports gates but no depth** (`depth=n/a`): the traced
  circuits run the instantiation that does not carry arrival levels. For
  depth, ask for the ordinary call with `--gates`.

## 6. What the CLI does not do

Do not try these; they fail by design, with an explanation.

- **No division trace.** `trace div ...` is a usage error: restoring
  division has no single carry advancing position by position.
- **No `lookahead_add` trace.** Its Kogge-Stone prefix network does not
  run position by position; a faithful viewer would be a different
  visualization.
- **`trace` takes plain non-negative integers only** — no expression, no
  sign, no Humphrey point. `trace add 4;6 1` is a calculation error.
- **`1/5` does not terminate in base 12.** The CLI says so rather than
  truncating in silence:

  ```
  $ dz-cli div 1 5
  1/5 (periodica en base 12)
  ```
- **`^` rejects fractional and negative exponents**; use
  `exp(y * ln(x))`.
- **The REPL's session is process memory.** `:mem`/`:mem+`/`:memc`,
  `:history`/`:clear`, `:session` exist in the interactive mode only, and
  nothing is persisted to disk when it exits.
- **`--alphabet` does not persist either**, and it does not translate the
  JSON or the CSV. It is per invocation (or per REPL session), never a
  stored setting: the GUI has its own preference, in its own browser
  storage.

## 7. If you are going to edit this repository

One rule governs the whole codebase: **no native Rust arithmetic operator
is ever applied to a dozenal value.** `Digit` is a symbol tag, not a
quantity; only `crates/dz-gates/src/backends/*.rs` and
`crates/dz-parse/src/alphabet.rs` may read `.tag()`. Native arithmetic on
`Vec` indices, loop counters and `usize` lengths is fine.

Read `CLAUDE.md` before changing code — it holds the layering, the crate
map and the measured results — and run `pwsh tools/lint-purity.ps1` before
calling a change good. This skill describes the tool only; it deliberately
does not restate the architecture, because two copies diverge.
