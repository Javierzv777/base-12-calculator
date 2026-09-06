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

cargo test --workspace              # 108 tests, ~3,5 min en debug
cargo test --workspace --release    # mucho más rápido (~segundos)
cargo clippy --workspace --all-targets
cargo run -p dz-cli                 # REPL
cargo run -p dz-cli -- sin 1 --gates          # modo de una invocación
cargo run -p dz-cli -- --functions            # catálogo de funciones
cargo run -p dz-gates --example gate_counts   # conteos de puertas
pwsh tools/lint-purity.ps1          # lint de pureza (4 reglas)

cd app && npm install               # una vez (Vite + @tauri-apps/*)
npm run tauri dev                   # GUI en desarrollo
npm run tauri build                 # instalador; --no-bundle para solo el .exe
npm run build                       # typecheck + bundle del frontend a solas
```

La suite lenta es `dz-num/tests/rational.rs` (~106 s en debug): `DzRational::new`
corre Euclides en cada operación y cada `div_rem` hace multiplicaciones de tanteo.
No está colgado, solo es caro. En `--release` baja a ~1 s.

## Dónde está implementado qué

Las capas solo llaman hacia abajo:

```
dz-cli, app/src-tauri  →  dz-api  →  dz-eval  →  dz-sci  →  dz-num  →  dz-int  →  dz-gates
                                        └──────→  dz-parse ──┘
```

`dz-api` (capa 5) es el único punto de entrada de las interfaces: ni el REPL, ni
el modo de una invocación, ni la GUI nombran `dz_eval::evaluate*` ni tipos de
`dz-num`. `dz-sci` (capa 2.5) es el análisis numérico de las funciones
científicas; no conoce el AST.

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
| `lookahead_add`, `generate_propagate`, `*_depth` (Fase 8) | `src/circuits/adder.rs` |
| `half_adder_balanced`, `lookahead_add_balanced`, `*_balanced_depth` (experimento de rebalanceo, no es producción) | `src/circuits/adder.rs` |
| Trait `GateNet` + `Values`/`Levels` (medición de profundidad) | `src/circuits/depth.rs` |
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

### `crates/dz-sci` — Capa 2.5, funciones científicas

| Qué | Dónde |
|---|---|
| `sqrt` (Newton), `exp`, `ln`, `log12`, `log_base` | `src/elementary.rs` |
| `sin`, `cos`, `tan`, `asin`, `acos`, `atan` | `src/trig.rs` |
| `pi` (Machin), `two_pi`, `half_pi`, `ln12`, `euler` — memoizadas | `src/constants.rs` |
| `floor`/`ceil`/`round`/`trunc`/`sign`/`abs`, en fijo **y** en racional | `src/exact.rs` |
| Literales (`0;6` = mitad), truncado a escala, contadores | `src/util.rs` |

Detalles que no hay que "simplificar":

- Los denominadores de las series son **contadores dozenales** (`3, 5, 7…` o
  `(2k)(2k+1)`) que avanzan con `add`/`mul` gate-compuestos. Calcular `2*k+1`
  en `usize` y convertirlo sería importar el significado desde aritmética
  nativa — y el lint lo rechaza.
- Dividir entre dos es multiplicar por `0;6` (exacto, un dígito); escalar por
  `12^k` es mover `scale`, cero puertas.
- Las constantes se sintetizan una vez por escala con un `MvlGates` desechable
  y se memoizan — mismo patrón y misma justificación que `lo_hi_table`. Por eso
  `dz-cli eval "pi" --gates` reporta 0 puertas.
- Diseño completo y números medidos: `docs/05-funciones-cientificas.md`.

### `crates/dz-parse` — Capa 3

`alphabet.rs` (glifos `0-9↊↋`, alias ASCII `X`/`T`/`E`, punto Humphrey `;`),
`lexer.rs` (construye `Dz` dígito a dígito, **nunca** `str::parse`), `parser.rs`
(precedencia), `ast.rs`, `format.rs` (`format_fixed`, `format_rational`, `format_dfp`).

Dos reglas léxicas que conviven con el alfabeto dozenal:

- **Identificador vs. número**: una tira maximal de letras es identificador si y
  solo si alguna de sus letras **no** es alias de dígito. `E` y `XE` siguen
  siendo números; `exp`, `tan`, `pi` son nombres.
- **Coma**: separador de argumentos dentro de una llamada, separador de
  agrupación de cuatro dígitos fuera (`1,0000`). El lexer lleva una pila de
  paréntesis, así que la regla es posicional, no heurística.

`Expr::Call { name, args }` cubre funciones y constantes (una constante es una
llamada de aridad cero, así el evaluador tiene un solo camino).

### `crates/dz-eval` — Capa 4

- `src/lib.rs`: `evaluate` (fixed), `evaluate_rational`, `evaluate_dfp` (firmas
  de siempre) y sus variantes `*_with(g, expr, &EvalConfig)` que devuelven
  `Evaluated { value, approximate }`. La marca de aproximado se propaga por todo
  el árbol: `sin(1) + 1/3` es aproximada, `1/3 + 1` no.
- `src/functions.rs`: trait `DozFunction` + `FnRegistry` + `FnInfo`. **Este es
  el punto de extensión** (sustituye a `DozOp`/`OpRegistry`, que nunca llegaba a
  ejecutarse porque no existía sintaxis de llamada). Una función implementa el
  caso de punto fijo y, si puede ser exacta, sobrescribe el caso racional.
- `src/builtins.rs`: la tabla nombre → algoritmo de `dz-sci`. Añadir una función
  es una fila más; aparece sola en el REPL (`:funcs`), en el CLI (`dz-cli sin 1`)
  y en el teclado de la GUI.

### `crates/dz-api` — Capa 5, punto de entrada de las interfaces

`src/lib.rs`: `Request { backend, mode, scale, op }`, `Operation` (`Sum`…`Pow`,
`Neg`, `Call`, `Eval`), `Response { display, periodic_note, approximate,
gate_count }`, `ApiError`, `execute`, `functions()`, `to_ascii`,
`to_json_line`. **Sin estado**: la sesión (backend activo, historial, memoria)
vive en cada interfaz. `reset_cost()` justo antes de despachar y `cost()`
después, así que toda respuesta trae el coste en puertas de esa llamada.

### `crates/dz-cli` — Capa 6, terminal

`src/main.rs`. Sin argumentos: REPL (`:mode`, `:backend`, `:scale`, `:gates`,
`:funcs`, `:help`, `:quit`; glifos Unicode). Con argumentos: una invocación
(`dz-cli sum 1 3`, `dz-cli eval "sin(pi/4)"`, `dz-cli sqrt 2 --mode fixed`),
alias ASCII por defecto, `--json`, `--unicode`, `--scale`, `--gates`,
`--functions`. Códigos de salida: 0 ok, 1 error de cálculo, 2 error de uso.

### `app/` — Capa 6, GUI Tauri

`src-tauri/src/commands.rs` (comandos `calc`/`set_backend`/`set_mode`/
`set_scale`/`get_session`/`clear_history`/`memory_*`/`list_functions`, DTOs con
`serde` — la única frontera del repo donde entra `serde`), `src-tauri/src/state.rs`
(`AppState`: backend, modo, escala, historial, memoria). Frontend vanilla TS +
Vite en `app/src/`: `api.ts` (invoke tipado), `keypad.ts` (teclado dozenal fijo +
pad científico **generado desde el catálogo**), `glyphs.ts` (detecta si la fuente
tiene ↊/↋ midiendo en un canvas; si no, cae a `X`/`E` solo en presentación),
`history.ts`, `main.ts`. El frontend nunca decide semántica dozenal: recibe
strings ya formateados.

### Docs e instrumentación

- `docs/00-fundamentos.md` — principio de pureza, estado, limitaciones
- `docs/01-diseno-sumador.md` — sumador, multiplicador v1 vs v2, y la discusión
  "¿el SOP es el backend de tabla disfrazado?"
- `docs/02-crt-vs-mvl.md` — hallazgo: CRT no gana
- `docs/03-formato-dfp.md` — diseño del formato flotante
- `docs/04-fase6-interfaces.md` — diseño de la fase 6 y, en §9, lo que se
  implementó y en qué se desvió
- `docs/05-funciones-cientificas.md` — algoritmos, exactitud y coste de `dz-sci`
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

`ripple_add` vs `lookahead_add` (Fase 8), peor caso `[↋]×n + [1]`, backend MVL:

| n | puertas ripple | prof. ripple | puertas lookahead | prof. lookahead |
|--:|--:|--:|--:|--:|
| 1 | 303 | 30 | 157 | 17 |
| 2 | 606 | 46 | 316 | 18 |
| 4 | 1.212 | 78 | 640 | 21 |
| 8 | 2.424 | 142 | 1.300 | 23 |
| 16 | 4.848 | 270 | 2.644 | 25 |

Profundidad de `half_adder` = 15 (12 de esos niveles son el pliegue lineal de
12 `max`). Contra lo esperado, el lookahead **no** sube el conteo total: gana en
puertas y en profundidad desde n = 1, porque el `full_adder` del ripple encadena
dos `half_adder` por posición y el lookahead usa uno solo más un mux de 5
puertas. La profundidad del ripple es afín (`16n + 14`); la del lookahead,
logarítmica. Detalle y lectura honesta en `docs/01-diseno-sumador.md`.

El modelo de profundidad vive en `circuits/depth.rs`: entrada/constante = nivel
0, cada primitiva = `1 + max(niveles de entrada)`. Los circuitos se escriben
**una sola vez**, genéricos sobre `GateNet`, e instanciados con `Values`
(`S = Digit`, coste cero, es lo que usan las funciones públicas) o `Levels`
(`S = Sig { value, level }`). **No se añadió ningún campo a `GateCount`**; una
pasada con `Levels` sí invoca al backend, así que conteo y profundidad se miden
en pasadas separadas con backends frescos.

**CRT no compensa.** Misma topología que MVL pero necesita reconstruir el dígito
para toda comparación de orden (`min`/`max`/`thr`): 176 reconstrucciones en un
`half_adder`, 13.938 en un `multiply` 3×2. La ventaja de descomponer nunca llega.

**Rebalanceo del pliegue lineal — experimento cerrado, no reemplaza al lineal.**
`half_adder_balanced`/`lookahead_add_balanced` (`circuits/adder.rs`) combinan el
mismo decodificador de 12 ramas con un árbol binario (`balanced_max_fold`) en
vez del pliegue lineal, como función nueva e independiente — `half_adder`,
`generate_propagate` y `lookahead_add` (números de arriba) no se tocaron:

| | pliegue lineal (producción) | árbol balanceado |
|---|--:|--:|
| `half_adder(7,9)` puertas | **151** | 149 |
| `half_adder` profundidad | **15** | 15 |
| `lookahead_add` n=16 puertas | **2.644** | 2.612 |
| `lookahead_add` n=16 profundidad | **25** | 26 |

El pliegue de `carry` sí es uniforme (todas sus 12 ramas usan `thr`, que no
depende de `j`) y balancea como se esperaría (14 → 6 en crudo). El de `sum`
no: `cycle_pow(x,j)` ya es `j` niveles hondo por sí solo, así que emparejar
posicionalmente sus 12 ramas no elimina esa profundidad y en este circuito
concreto la empeora un nivel (14 → 15) en vez de mejorarla — y por eso la
profundidad de `half_adder` (el máximo de ambas salidas) no baja, y la de
`lookahead_add` (que reutiliza `sum` para `P` y el mux final) sube. Detalle
completo en `docs/01-diseno-sumador.md` sección "Rebalanceo del pliegue
lineal (experimento cerrado)".

**Coste de las funciones científicas** (backend MVL, modo fijo, escala 12,
`dz-cli eval "<expr>" --mode fixed --scale 12 --gates`):

| expresión | puertas | tiempo (release) |
|---|--:|--:|
| `1+1` | 24.083 | — |
| `4;6*3` | 120.444 | — |
| `sqrt(2)` | 32.657.729 | 141 ms |
| `sin(1)` | 27.139.541 | 557 ms |
| `ln(2)` | 226.222.343 | 401 ms |
| `atan(1)` | 352.673.718 | 588 ms |
| `pi` | 0 | — |

Una trascendente cuesta 2–4 órdenes de magnitud más que una multiplicación, y
lo que domina es la **división restauradora** (`ln` y `atan` pagan las raíces
cuadradas de su reducción de argumento, cada una con su Newton lleno de
divisiones). `pi` marca 0 porque las constantes se sintetizan en un backend
desechable y se memoizan (a propósito: la medición del usuario no debe depender
de si esta llamada fue la primera). Detalle en
`docs/05-funciones-cientificas.md`.

## Limitaciones conocidas (no son bugs por descubrir)

- DFP sin subnormales ni cero con signo (`Dz` cero no tiene signo por construcción).
- `evaluate_dfp` redondea una vez al final de la expresión, no tras cada operación,
  así que `1/0` dentro de una expresión falla en la etapa racional en vez de dar `Inf`.
- El carry-lookahead (Fase 8) ya existe (`lookahead_add`) pero `dz-int` sigue
  usando `ripple_add`; la sustitución es trivial y se dejó fuera para que los
  números publicados de `multiply`/`div_rem` sigan siendo comparables con los de
  fases anteriores.
- `1/5` no termina en base 12; el REPL lo avisa en vez de truncar en silencio.
- `^` solo acepta exponentes enteros no negativos. Un exponente fraccionario se
  escribe `exp(y * ln(x))`; no se cambió el operador para no alterar el
  comportamiento (ni los tests) de la fase 5.
- El número de Euler se llama `euler`, no `e`: `e` es el alias ASCII del dígito
  once. Por la misma razón `x` y `t` tampoco pueden ser nombres.
- Dentro de una llamada la coma separa argumentos, así que ahí hay que escribir
  `10000` y no la forma agrupada `1,0000` (fuera de las llamadas sigue siendo el
  separador de agrupación de siempre).
- Las trascendentes son aproximaciones truncadas a la escala de trabajo y se
  marcan con `~`; en modo racional se muestran en punto fijo en vez de como una
  fracción de doce dígitos.
- La GUI ya existe pero **no se ha validado visualmente en pantalla**: compila,
  arranca y responde, y la elección de fuente para ↊/↋ sigue abierta.
  El visor paso a paso del ripple y el export CSV (Fase 7) no están.

## Convenciones

- Documentación y comentarios de código en inglés; docs de investigación en español.
- No hacer commits salvo que se pida.
- Los tests pueden usar `num-bigint` como oráculo; el código de producción no.
