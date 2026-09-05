# CLAUDE.md

Calculadora dozenal (base 12) construida desde puertas lógicas 12-valuadas.
Proyecto de investigación. `PLAN.md` tiene el plan completo por fases.

## Regla que gobierna todo el repo

**Ningún operador aritmético nativo de Rust se aplica jamás a un valor dozenal.**

- `Digit` (`crates/dz-gates/src/digit.rs`) es `struct Digit(u8)` y **no implementa**
  `Add`, `Sub`, `Mul`, `Div`, `Ord` ni `From<u32>`. El `u8` interno es una etiqueta
  de símbolo, no una cantidad.
- Solo pueden leer `.tag()`: los backends (`backends/*.rs`, que son la definición
  por tabla de verdad de las primitivas) y `dz-parse/src/alphabet.rs` (mapeo
  símbolo ↔ carácter). Nada más.
- Se permite aritmética nativa sobre índices de `Vec`, contadores de bucle y
  longitudes `usize`. Nunca sobre el *valor* de un dígito.
- Distinción clave: las tablas de `cycle`/`min`/`max` en los backends son
  **axiomas** — definen qué son los símbolos. Suma, multiplicación, división y
  comparación son **operaciones derivadas** y deben salir de las primitivas.
  Derivar una operación con `*` nativo es exactamente lo que el proyecto existe
  para evitar (ver commit `18565f8`, que arregla justo ese fallo).

Antes de dar por bueno un cambio: `pwsh tools/lint-purity.ps1`.

## Comandos

`cargo` no está en el PATH. Está en `C:\Users\ADMIN\.cargo\bin`.

```bash
export PATH="$PATH:/c/Users/ADMIN/.cargo/bin"   # bash
$env:PATH += ";$env:USERPROFILE\.cargo\bin"     # powershell

cargo test --workspace              # 48 tests, ~2 min en debug
cargo test --workspace --release    # mucho más rápido (~segundos)
cargo clippy --workspace --all-targets
cargo run -p dz-cli                 # REPL
cargo run -p dz-gates --example gate_counts   # conteos de puertas
pwsh tools/lint-purity.ps1          # lint de pureza (4 reglas)
```

La suite lenta es `dz-num/tests/rational.rs` (~106 s en debug): `DzRational::new`
corre Euclides en cada operación y cada `div_rem` hace multiplicaciones de tanteo.
No está colgado, solo es caro. En `--release` baja a ~1 s.

## Dónde está implementado qué

Las capas solo llaman hacia abajo: `dz-cli` → `dz-eval` → `dz-parse` → `dz-num`
→ `dz-int` → `dz-gates`.

### `crates/dz-gates` — Capa 0, las puertas

| Qué | Dónde |
|---|---|
| `Digit`, `RADIX`, contrato de pureza | `src/digit.rs` |
| Trait `DozGates` (`cycle`/`min`/`max`/`lit`/`thr`/`cost`) | `src/traits.rs` |
| Instrumentación `GateCount`, `Counter` | `src/gatecount.rs` |
| Backend MVL (Post-álgebra, el de producción) | `src/backends/mvl.rs` |
| Backend CRT (Z12 ≅ Z4×Z3) + `reconstruction_ops()` | `src/backends/crt.rs` |
| Oráculo por tabla, `oracle_add`, `oracle_mul` — **solo para tests** | `src/backends/table.rs` |
| `half_adder`, `full_adder`, `ripple_add` | `src/circuits/adder.rs` |
| `digit_mul_v1` (shift-add), `digit_mul_v2` (SOP, por defecto), `multiply` | `src/circuits/multiplier.rs` |
| `ge`, `flag_to_bool` | `src/circuits/comparator.rs` |
| `ones_complement`, `radix_complement` | `src/circuits/complement.rs` |
| `is_even_digit` | `src/circuits/parity.rs` |

Fórmulas del sumador de un dígito (`adder.rs`):
```
s = MAX_j MIN( lit(j,y), cycle^j(x) )
c = MAX_j MIN( lit(j,y), thr(12-j, x) )
```
Del multiplicador SOP (`multiplier.rs`):
```
lo/hi = MAX_{j,k} MIN( lit(j,x), lit(k,y), LO/HI[j][k] )
```
`LO_HI` se sintetiza en `lo_hi_table()` llamando a `digit_mul_v1` sobre los 144
pares, memoizado en `OnceLock`, con un `MvlGates` desechable cuyo `GateCount`
nunca se lee — así el coste de inicialización no contamina la medición del
usuario. **No rellenes esa tabla con `j * k`**; es el bug que ya se arregló una vez.

### `crates/dz-int` — Capa 1, enteros

Todo en `src/dzint.rs`: `Dz` (precisión arbitraria, signo+magnitud), `add`, `sub`
(vía complemento a 12), `mul`, `div_rem` (división restauradora), `compare`,
`is_even`. También `small_usize_to_dz`/`dz_to_small_usize`, que convierten por
**conteo repetido** con puertas, nunca con `% 12`.

### `crates/dz-num` — Capa 2, fraccionarios

| Qué | Dónde |
|---|---|
| `DzFixed` punto fijo, `DEFAULT_DIV_SCALE = 24` | `src/fixed.rs` |
| `DzRational` exacto, `terminates_in_base12()`, `to_fixed()` | `src/rational.rs` |
| `DzFloat` (DFP), `MANTISSA_DIGITS = 12`, `BIAS = 71` | `src/dfp.rs` |

Redondeo half-even en `dfp.rs::from_dzfixed`: compara `2r` contra el divisor para
detectar el empate exacto sin dividir, y desempata con `is_even`. No lo
"simplifiques" — es la forma correcta.

### `crates/dz-parse` — Capa 3

`alphabet.rs` (glifos `0-9↊↋`, alias ASCII `X`/`T`/`E`, punto Humphrey `;`),
`lexer.rs` (construye `Dz` dígito a dígito, **nunca** `str::parse`), `parser.rs`
(precedencia), `ast.rs`, `format.rs` (`format_fixed`, `format_rational`, `format_dfp`).

### `crates/dz-eval` — Capa 4

`src/lib.rs`: `evaluate` (fixed), `evaluate_rational`, `evaluate_dfp`. Trait
`DozOp` + `OpRegistry` son el punto de extensión para funciones nuevas: se
implementa y se registra, sin tocar el núcleo.

### `crates/dz-cli` — Capa 5 (provisional)

`src/main.rs`: REPL. `:mode rational|fixed|dfp`, `:backend mvl|crt|table`, `:help`,
`:quit`. Modo por defecto: `rational`.

### Docs e instrumentación

- `docs/00-fundamentos.md` — principio de pureza, estado, limitaciones
- `docs/01-diseno-sumador.md` — sumador, multiplicador v1 vs v2, y la discusión
  "¿el SOP es el backend de tabla disfrazado?"
- `docs/02-crt-vs-mvl.md` — hallazgo: CRT no gana
- `docs/03-formato-dfp.md` — diseño del formato flotante
- `tools/lint-purity.ps1` — 4 reglas; la 4 detecta `*`/`%`/`/` nativos fuera de
  `backends/`, que es la clase de fallo que las otras tres no veían

## Resultados medidos

`digit_mul` por multiplicación de un dígito, backend MVL:

| | v1 shift-add | v2 SOP |
|---|--:|--:|
| `(7,9)` y `(↋,↋)` | 33.693 | 876 |
| `(0,0)` | 20.058 | 876 |
| `multiply` 3×2 dígitos | 201.243 | 9.795 |

v2 es de coste constante; v1 dependía del operando. `half_adder` = 151.

**CRT no compensa.** Misma topología que MVL pero necesita reconstruir el dígito
para toda comparación de orden (`min`/`max`/`thr`): 176 reconstrucciones en un
`half_adder`, 13.938 en un `multiply` 3×2. La ventaja de descomponer nunca llega.

## Limitaciones conocidas (no son bugs por descubrir)

- DFP sin subnormales ni cero con signo (`Dz` cero no tiene signo por construcción).
- `evaluate_dfp` redondea una vez al final de la expresión, no tras cada operación,
  así que `1/0` dentro de una expresión falla en la etapa racional en vez de dar `Inf`.
- Sin carry-lookahead (Fase 8) ni GUI Tauri (Fase 6).
- `1/5` no termina en base 12; el REPL lo avisa en vez de truncar en silencio.

## Convenciones

- Documentación y comentarios de código en inglés; docs de investigación en español.
- No hacer commits salvo que se pida.
- Los tests pueden usar `num-bigint` como oráculo; el código de producción no.
