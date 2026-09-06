# 04 — Fase 6: motor reutilizable, CLI de una invocación y GUI Tauri

Estado de partida (rama `feat/balanced-half-adder`): Fases 0–5 y 8 completas,
69 tests, clippy limpio, lint de pureza en verde. Este documento diseña la
Fase 6 (PLAN.md §7 y §10bis-B) sin implementarla. Ninguna decisión de aquí
debe requerir tocar `dz-gates`, `dz-int`, `dz-num` o `dz-parse`: la Fase 6 es
estrictamente una capa nueva por encima de `dz-eval`, más dos interfaces
(CLI de comando único, Tauri) que la consumen.

---

## 1. Decisión de arquitectura de la capa de API

### 1.1 El problema concreto

Hoy `crates/dz-cli/src/main.rs` es dueño de tres cosas a la vez: el bucle de
entrada (`stdin`), el estado de sesión (`backend: Backend`, `mode: Mode`), y
la traducción entre texto de usuario y llamadas a `dz_eval::{evaluate,
evaluate_rational, evaluate_dfp}` + `dz_parse::{parse_str, format_*}`. Una
GUI Tauri y un modo `base12 sum 1 3` necesitan exactamente esa traducción,
pero ninguna de las dos quiere el bucle de `stdin` ni quiere reimplementar
"cómo se arma un `DzFixed` desde una operación nombrada en vez de desde un
AST parseado".

`dz-eval` ya tiene la mitad del trabajo hecho: `EvalError` (enum, no
strings), y `DozOp`/`OpRegistry` como punto de extensión para funciones
nuevas dentro del lenguaje de expresiones (`abs`, futuras `gcd`, etc.). Pero
`DozOp` opera sobre `&[DzFixed]` — está atado al modo `fixed` y a la
evaluación *dentro* de una expresión ya parseada. No es el punto de
extensión correcto para "una interfaz externa quiere pedir una suma sin
pasar por el lexer".

### 1.2 Alternativas consideradas

**(a) Extender `dz-eval` in situ con un tipo `Request`/`Response`.**
Descartada. `dz-eval` es Capa 4 en la jerarquía del proyecto y sus
dependencias (`dz-gates`, `dz-int`, `dz-num`, `dz-parse`) están fijadas por
lo que la evaluación de expresiones necesita. Meter ahí serialización
(`serde`, pensando en JSON para Tauri/CLI) y un tipo de sesión mezclaría
"cómo se evalúa una expresión" con "cómo una interfaz externa habla con la
calculadora" — dos responsabilidades con tasas de cambio distintas (añadir
un operador al lenguaje vs. añadir un campo a la respuesta para la Fase 7).
También forzaría que todo lo que ya depende de `dz-eval` (nada aún, pero es
la capa que consumirán `dz-cli` y Tauri) cargue con una dependencia de
`serde` que hoy no necesita.

**(b) Un crate nuevo `dz-api`, capa 5, entre `dz-eval` y las interfaces.**
**Elegida.** Encaja en la cadena de capas ya documentada en CLAUDE.md
(`dz-cli → dz-eval → dz-parse → dz-num → dz-int → dz-gates`, cada capa
llamando solo hacia abajo): `dz-api` depende de `dz-eval`, `dz-parse`,
`dz-num`, `dz-gates` (para `DozGates`/`GateCount`/los tres backends) y
`dz-int` (indirectamente, vía tipos reexportados), y todo lo que hoy sería
"interfaz" (`dz-cli`, un futuro `dz-cli-cmd`, la lib de Tauri) depende
únicamente de `dz-api`, nunca de `dz-eval` directamente. Es la misma jugada
que ya se hizo con `DzRational::num_digits()`/`den_digits()` en `dz-num`
para que `dz-parse` no necesite nombrar `dz_int::Dz` — aislar a la capa de
arriba de los tipos internos de la de abajo.

**(c) Trait `DozOp` extendido para cubrir operaciones binarias con nombre.**
Descartada como solución completa, pero se conserva **sin cambios** como lo
que ya es: el punto de extensión del *lenguaje de expresiones* (funciones
invocables dentro de una expresión parseada, tipo `abs(x)`). `dz-api` no lo
reemplaza ni lo envuelve; simplemente lo usa cuando la operación pedida es
"evaluar una expresión completa" (ver 2.2, variante `Eval`), y para el resto
de operaciones nombradas (`Sum`, `Sub`, ...) despacha directamente a los
métodos ya existentes de `DzFixed`/`DzRational`/`DzFloat` — no hay necesidad
de forzar `sum`/`sub`/`mul` a pasar por `OpRegistry`, que está pensado para
funciones *nuevas*, no para las cuatro operaciones aritméticas base que ya
tienen su propio método en cada tipo numérico.

**(d) Serialización de dominio (`serde` sobre `Dz`/`DzFixed` directamente).**
Descartada como interfaz pública. Serializar la representación interna
(`Vec<Digit>` + signo) filtraría un detalle de implementación al frontend y
tentaría a alguien a hacer aritmética con esos números del lado JS — que es
justo lo que la regla de pureza prohíbe cruzar la frontera del proceso, ni
hablar de cruzar a otro lenguaje. `dz-api` serializa **strings ya
formateados** (ver 1.4) y, cuando haga falta el valor exacto para depuración
avanzada (no en la Fase 6), listas de dígitos como `u8` opacos, nunca un
tipo que invite a `+`/`*` en el receptor.

### 1.3 Por qué esto no bloquea la Fase 7

El panel de investigación (Fase 7) necesita el conteo de puertas de la
*última operación*. `GateCount` (`crates/dz-gates/src/gatecount.rs`) ya se
lee con `g.cost()` después de una llamada y se resetea con `g.reset_cost()`
antes. `dz-api` reserva un campo `gate_count: Option<GateCountDto>` en la
respuesta (ver 2.2) desde el día uno, poblado siempre (es casi gratis:
`reset_cost()` antes de despachar, `cost()` después), aunque ninguna
interfaz de la Fase 6 lo muestre todavía. Así la Fase 7 no toca `dz-api` en
absoluto — solo la interfaz (CLI o Tauri) empieza a leer un campo que ya
estaba ahí. Lo mismo aplica al futuro visor paso a paso de ripple-carry: no
lo diseñamos aquí, pero al no atar la respuesta de `dz-api` a "solo el
resultado final", queda espacio para añadir `trace: Option<Vec<TraceStep>>`
sin romper la firma pública (es un campo `Option` nuevo, no un cambio de
tipo).

### 1.4 Estado de sesión: vive en la interfaz, no en `dz-api`

`dz-api` es **sin estado** entre llamadas: cada `Request` lleva su propio
`mode` y `backend` explícitos (ver 2.1). La sesión (qué backend/modo está
activo "ahora", historial de operaciones, memoria tipo M+/MR) vive en cada
interfaz:

- El REPL de `dz-cli` ya tiene esto (`enum Backend`, `enum Mode` en
  `main.rs`) — se conserva tal cual, solo que en vez de llamar a
  `dz_eval::evaluate*` directamente, construye un `Request` con el
  `mode`/`backend` de la sesión actual y llama a `dz_api::execute`.
- El modo CLI de una invocación (§3) no tiene sesión: cada proceso es una
  llamada; el "estado" son los flags de esa invocación.
- Tauri (§4) mantiene el estado de sesión (backend elegido, historial) en
  `tauri::State`, del lado Rust — nunca en el frontend, porque el frontend
  no debe decidir *nada* sobre semántica dozenal, solo mostrar lo que Rust
  ya formateó.

Razón para no meter la sesión en `dz-api`: el historial y la memoria son
conceptos de *interfaz de usuario* (qué se le muestra a alguien y en qué
orden), no de *motor de cálculo*. Meterlos en `dz-api` obligaría a decidir
ahí cosas como "¿cuántas entradas de historial se retienen?" que no tienen
nada que ver con lo que este proyecto investiga, y acoplaría el motor a un
modelo de UI concreto que la GUI y el CLI no necesariamente comparten (un
CLI de una invocación no tiene historial en absoluto).

---

## 2. Especificación de la API (`crates/dz-api`)

### 2.1 Cargo.toml (nuevo crate)

```toml
[package]
name = "dz-api"
version.workspace = true
edition.workspace = true
authors.workspace = true

[dependencies]
dz-gates = { path = "../dz-gates" }
dz-int = { path = "../dz-int" }
dz-num = { path = "../dz-num" }
dz-parse = { path = "../dz-parse" }
dz-eval = { path = "../dz-eval" }
```

Nota deliberada: **sin `serde` en esta fase.** JSON para el modo `--json`
del CLI y para el puente Tauri se sirve mejor con una serialización manual
mínima (ver §3.3) hasta que haya un segundo consumidor real que la
justifique — es la misma disciplina de "no construir la capa de abstracción
antes de tener dos usos concretos" que ya sigue el resto del repo (`DozOp`
se añadió cuando hizo falta extender, no antes). Si en la Fase 7 el panel
de Tauri necesita estructuras anidadas complejas (trazas paso a paso),
`serde` se añade entonces a `dz-api`, no antes.

### 2.2 Tipos centrales

```rust
//! `dz-api` -- Layer 5: stable, interface-agnostic entry point over
//! `dz-eval`. Every consumer that isn't `dz-eval` itself (the REPL, the
//! one-shot CLI mode, the Tauri backend) talks to the calculator through
//! this crate, never by naming `dz_eval::evaluate*` or `dz_num::*` types
//! directly. This is what lets three interfaces share one implementation
//! of "what a dozenal operation is" without duplicating dispatch logic.

use dz_gates::{CrtGates, DozGates, GateCount, MvlGates, TableGates};
use dz_parse::ExprParseError;

/// Which of the three `DozGates` backends to use for a single call.
/// Selecting a backend is per-request, not global state -- session state
/// (which backend a REPL or GUI currently has "active") lives in the
/// caller, not here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackendKind {
    Mvl,
    Crt,
    Table,
}

/// Which numeric representation the result is rendered in, mirroring
/// `dz-cli`'s existing `:mode rational|fixed|dfp`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumberMode {
    Rational,
    Fixed,
    Dfp,
}

/// A single request: one operation, fully self-describing. No hidden
/// session state -- `backend`/`mode` travel with every call.
#[derive(Clone, Debug)]
pub struct Request {
    pub backend: BackendKind,
    pub mode: NumberMode,
    pub op: Operation,
}

/// The four base arithmetic operations get named variants so callers don't
/// need to build an `Expr` AST just to add two numbers (`base12 sum 1 3`
/// shouldn't go through the lexer). `Eval` is the escape hatch for full
/// expressions, reusing `dz_parse::parse_str` exactly as `dz-cli` does
/// today.
///
/// Operands are plain dozenal text (e.g. `"4;6"`, `"-3"`), parsed the same
/// way a REPL number literal is: digit-by-digit, never through a native
/// number parser. `dz-api` re-lexes each operand with `dz_parse::lex` to
/// get `int_digits`/`frac_digits`, exactly what `Expr::Number` already
/// carries, so there is exactly one code path (`dz-parse`'s lexer) that
/// ever turns text into `Digit`s -- `dz-api` does not add a second one.
#[derive(Clone, Debug)]
pub enum Operation {
    Sum { a: String, b: String },
    Sub { a: String, b: String },
    Mul { a: String, b: String },
    Div { a: String, b: String },
    Mod { a: String, b: String },
    Pow { base: String, exponent: String },
    Neg { a: String },
    /// Full expression, e.g. `"4;6 * 3"` or `"(1+2)^3"`.
    Eval { expr: String },
}

/// Everything a caller needs, in one value: the formatted result (already
/// in dozenal glyphs -- see `dz-parse::format`), enough metadata to know
/// whether it's exact, and gate-count instrumentation for the Phase 7
/// research panel, populated unconditionally (cheap: `reset_cost` before
/// dispatch, `cost` after) even though no Phase 6 interface displays it
/// yet.
#[derive(Clone, Debug)]
pub struct Response {
    /// Formatted value, dozenal glyphs (↊/↋), via `dz_parse::format_*`.
    /// Interfaces that need ASCII output (the one-shot CLI's default, see
    /// doc 3) re-map glyphs to `X`/`E` themselves -- `dz-api` always
    /// produces the canonical glyph form, exactly like `dz-parse` does
    /// today, so there is one formatting authority, not two.
    pub display: String,
    /// Set when `mode == Rational` and the exact value doesn't terminate
    /// in base 12 (mirrors `dz-cli`'s "(periodica en base 12)" note) --
    /// structured, so a GUI can render its own badge instead of parsing a
    /// substring out of `display`.
    pub periodic_note: bool,
    /// Gate-invocation counts for this call's backend instance, reset
    /// immediately before dispatch. `None` only if the operation errored
    /// before any gate call could occur (e.g. a lex error on the operand
    /// text, which never touches `DozGates`).
    pub gate_count: Option<GateCount>,
}

/// Structured errors -- never a bare `String` -- so a GUI can localize or
/// style them and a CLI can map them to distinct process exit codes (see
/// doc 3.1). Wraps `dz_eval::EvalError` and `dz_parse::ExprParseError`
/// rather than re-declaring their variants, so a new `EvalError` variant
/// (e.g. Phase 8's future non-integer-exponent case) doesn't need a
/// matching edit here.
#[derive(Debug)]
pub enum ApiError {
    Parse(ExprParseError),
    Eval(dz_eval::EvalError),
}

impl std::fmt::Display for ApiError { /* delegates to inner Display */ }
impl std::error::Error for ApiError {}

/// The one entry point. Constructs the requested backend fresh per call
/// (matching `dz-cli`'s current `:backend` semantics: switching backends
/// resets gate-count history, which is correct -- a research panel
/// comparing backends wants each call's cost in isolation, not a running
/// total across backend switches), dispatches `req.op`, formats per
/// `req.mode`, and returns gate cost alongside the result.
pub fn execute(req: &Request) -> Result<Response, ApiError> {
    match req.backend {
        BackendKind::Mvl => execute_with(&MvlGates::new(), req),
        BackendKind::Crt => execute_with(&CrtGates::new(), req),
        BackendKind::Table => execute_with(&TableGates::new(), req),
    }
}

fn execute_with<G: DozGates>(g: &G, req: &Request) -> Result<Response, ApiError> {
    // 1. Build an `Expr` from `req.op` (named ops become synthetic
    //    `Expr::Add(Number, Number)` etc.; `Eval` calls `dz_parse::parse_str`
    //    directly) -- one AST-construction path either way.
    // 2. g.reset_cost().
    // 3. Dispatch to `dz_eval::evaluate` / `evaluate_rational` / `evaluate_dfp`
    //    per `req.mode`, exactly as `dz-cli::eval_line` does today.
    // 4. Format with `dz_parse::format_fixed` / `format_rational` / `format_dfp`.
    // 5. Response { display, periodic_note, gate_count: Some(g.cost()) }.
    todo!()
}
```

Puntos de diseño explícitos:

- **`Operation` con variantes nombradas, no un `name: &str` + `args: Vec<String>`
  genérico.** Un `enum` cerrado hace que el compilador obligue a manejar cada
  operación en `execute_with`, y evita que el CLI/Tauri tengan que validar
  arity a mano (ese trabajo ya lo hace `OpRegistry::call` para las funciones
  extensibles del lenguaje; para las operaciones base no hace falta
  reinventarlo, el `enum` ya es la validación de arity).
- **Los operandos son `String`, no `DzFixed`/`Dz`.** Mantiene a `dz-api`
  ajeno a qué tipo numérico interno usa cada modo (una `Sum` en modo
  `Rational` construye `DzRational`s; la misma `Sum` en modo `Fixed`
  construye `DzFixed`s) y evita que el CLI tenga que saber construir un
  `Digit` — sigue habiendo un único lugar (`dz-parse::lexer`) que convierte
  texto de usuario en dígitos.
- **`execute` es una función libre, no un objeto con estado.** No hay
  `struct Engine` que guardar entre llamadas porque no hay estado que
  guardar (§1.4). Esto simplifica tanto el CLI (`execute(&req)` y listo)
  como Tauri (cada `#[tauri::command]` puede ser un `execute` directo; el
  `tauri::State` solo guarda qué `backend`/`mode` usar en la *próxima*
  petición, no nada del motor).

### 2.3 Qué prueba necesita `dz-api`

- Diferencial contra `dz-cli::eval_line` actual: para un conjunto de
  expresiones representativas (los mismos casos que ya cubre
  `dz-parse/tests/roundtrip.rs` y los ejemplos de CLAUDE.md como
  `4;6 × 3 = 11;6`), `execute` con `Operation::Eval` debe producir el mismo
  `display` que el REPL produce hoy para esa misma línea y modo. Esto es lo
  que garantiza que extraer la capa no cambió el comportamiento observable.
- Un test por variante de `Operation` (`Sum`, `Sub`, ...) en cada uno de los
  tres `NumberMode`, comprobando que equivale a construir el `Expr` a mano
  (`Expr::Add(Number, Number)`) y llamar a `evaluate*` directamente.
- `gate_count` no es `None` en el camino feliz, y `ApiError::Parse` no trae
  `gate_count` (nunca se llegó a construir un backend).

---

## 3. Modo CLI de una invocación

### 3.1 Dónde vive

**En el mismo binario `dz-cli`, no en uno nuevo.** `main()` inspecciona
`std::env::args()`: si hay argumentos además del nombre del programa,
bifurca a modo comando (parsea, ejecuta, imprime, sale); si no hay
argumentos, arranca el REPL exactamente como hoy. Esto es lo que hacen la
mayoría de herramientas CLI de Unix (`git` sin subcomando vs. con
subcomando no es análogo perfecto, pero `psql`/`sqlite3` sin argumentos
= REPL, con argumentos = ejecutar y salir, sí lo es) y evita mantener dos
`Cargo.toml`/binarios sincronizados a mano. Coste: `main.rs` gana una
bifurcación al principio; el REPL existente (`run_repl()`, extraído de la
actual `main()`) no cambia una línea.

No se necesita ninguna dependencia de parseo de argumentos (`clap`, etc.)
para el vocabulario reducido de abajo — un `match` sobre `args[1]` más
`strip_prefix("--")` a mano es suficiente y consistente con la filosofía
del repo de minimizar dependencias externas al núcleo. Si el vocabulario de
flags crece mucho en el futuro, revisar esta decisión, pero no
preoptimizar ahora.

### 3.2 Gramática

```
base12 <operación> <args...> [flags]
base12 eval "<expresión>" [flags]

operación ::= sum | sub | mul | div | mod | pow | neg
  sum a b        -- a + b
  sub a b        -- a - b
  mul a b        -- a * b
  div a b        -- a / b
  mod a b        -- a % b
  pow base exp   -- base ^ exp
  neg a          -- -a

flags (aplican a ambas formas):
  --mode rational|fixed|dfp     (por defecto: rational, igual que el REPL)
  --backend mvl|crt|table       (por defecto: mvl)
  --json                        (salida estructurada, ver 3.3)
  --unicode                     (usa glifos ↊/↋ en vez de ASCII X/E, ver 3.4)
  --help, -h                    (uso; código de salida 0)
```

Ejemplos:

```
$ base12 sum 1 3
4

$ base12 eval "4;6 * 3"
11;6

$ base12 div 1 3 --mode fixed
0;4000000000000000000000000 (periodica en base 12, valor truncado)

$ base12 div 1 3 --mode rational
1/3

$ base12 sum 1 3 --json
{"ok":true,"display":"4","periodic_note":false,"gate_count":{"cycle":11,"min":4,"max":4,"lit":4,"thr":2,"total":25}}

$ base12 div 1 0
error: division by zero
$ echo $?
1
```

Notas de diseño:

- **Posicional vs. `eval` libre, ambos soportados, sin ambigüedad**: los
  nombres de operación (`sum`, `sub`, ...) son palabras reservadas que
  nunca son un literal dozenal válido por sí solas, así que `argv[1]`
  decide sin heurísticas. `eval` es explícito precisamente para que
  `base12 eval "1 + 3"` no se confunda con intentar interpretar `"1 + 3"`
  como si fuera el primer operando de `sum`.
- **Sin flag corta para operación**: cada operación es su propio
  subcomando (no `base12 --op sum 1 3`), porque así se lee como una frase
  y es lo que un usuario de terminal espera de una calculadora de línea de
  comandos (comparar con `expr 1 + 3` de POSIX, pero con nombres en vez de
  símbolos para no chocar con la expansión de shell de `*`).
- **Código de salida**: `0` en éxito, `1` en cualquier `ApiError` (parseo o
  evaluación), `2` en error de uso (flags mal formadas, operación
  desconocida, arity incorrecta) — la distinción entre `1` y `2` es
  estándar en herramientas Unix y le permite a un script distinguir "el
  cálculo falló" de "invocaste mal el programa".
- **`--mode`/`--backend` por defecto igual que el REPL** (`rational`,
  `mvl`) para que el comportamiento no sorprenda a quien ya conoce
  `dz-cli` interactivo.

### 3.3 Formato de salida

- **Por defecto: texto plano**, una línea, sin adornos — pensado para
  `$(base12 sum 1 3)` en scripts. La nota de periodicidad (`"(periodica en
  base 12)"`) se **anexa** en texto plano exactamente como hoy en el REPL,
  porque es información que el usuario interactivo quiere ver, pero un
  script que necesite el valor puro puede combinarlo con `--json` y leer
  `display` sin la nota.
- **`--json`**: un objeto de una sola línea, serializado a mano (sin
  `serde_json` en esta fase — ver 2.1; el objeto es tan simple, con cuatro
  campos de tipos primitivos, que una función `to_json_line(&Response) ->
  String` de un puñado de líneas es más barata que la dependencia). Campos:
  `ok` (bool), `display`, `periodic_note` (bool), `gate_count` (objeto con
  los cinco contadores + `total`, o `null`); en error: `{"ok":false,
  "error":"division by zero"}`.
- Si el vocabulario JSON crece (Fase 7 quiere volcar trazas anidadas), esa
  es la señal para introducir `serde`/`serde_json` como dependencia real de
  `dz-api`, no antes.

### 3.4 Glifos ↊/↋ vs. alias ASCII

**Decisión: ASCII (`X`/`E`) por defecto en modo comando; `--unicode` activa
los glifos.** Justificación:

- El riesgo de renderizado de `↊`/`↋` (U+218A/U+218B) en fuentes de Windows
  ya está documentado como abierto en PLAN.md §12, y una terminal (más aún
  redirigida a un archivo, un pipe, o un log de CI) es exactamente el
  entorno donde no se controla la fuente ni el locale de salida — al
  contrario que un REPL interactivo, donde el usuario ya está mirando la
  terminal y puede constatar si su fuente soporta los glifos antes de
  seguir.
- El REPL interactivo (`dz-cli` sin argumentos) **no cambia**: sigue
  usando los glifos Unicode por defecto, como hoy — este ajuste es
  exclusivo del modo comando de una invocación, donde "pensado para
  scripting" pesa más que "fiel a la investigación tipográfica".
- `dz_parse::alphabet::digit_to_char` sigue siendo la única fuente de
  verdad para el glifo canónico; el modo comando no inventa una segunda
  tabla, solo aplica un mapa de sustitución `↊→X, ↋→E` sobre el `String` ya
  formateado por `dz-api`, análogo al alias de entrada que
  `char_to_digit` ya acepta en la dirección contraria.

---

## 4. Comandos Tauri

### 4.1 Mapeo 1:1 a `dz-api`

```rust
// src-tauri/src/commands.rs -- thin wrappers, zero dozenal logic here.
// Every command borrows `AppState` for the session's current
// backend/mode, builds a `dz_api::Request`, and returns what `dz_api`
// already produced -- formatted strings and plain-data structs, nothing
// that invites arithmetic on the JS side.

#[tauri::command]
fn calc(state: tauri::State<AppState>, op: OperationDto) -> Result<ResponseDto, String>;
// OperationDto mirrors dz_api::Operation, (de)serialized via serde for
// the IPC boundary only -- serde lives in the Tauri crate, not in dz-api.

#[tauri::command]
fn set_backend(state: tauri::State<AppState>, backend: String) -> Result<(), String>;

#[tauri::command]
fn set_mode(state: tauri::State<AppState>, mode: String) -> Result<(), String>;

#[tauri::command]
fn get_session(state: tauri::State<AppState>) -> SessionDto;
// current backend/mode/history, for the frontend to render on load/reload.
```

`AppState` (un `Mutex<Session>` gestionado por Tauri) es donde vive el
estado de sesión de la GUI (backend activo, modo activo, historial de
`(input, display)` para el listado de la calculadora) — coherente con §1.4:
el estado de UI vive en la interfaz, `dz-api` sigue sin estado.

`OperationDto`/`ResponseDto` son structs `#[derive(Serialize, Deserialize)]`
que viven en el crate de Tauri (`src-tauri`), no en `dz-api`, y son
conversiones directas campo a campo desde/hacia `dz_api::Operation`/
`Response` — es la única frontera del repo donde `serde` entra en juego,
y es intencional que sea ahí y no más abajo.

### 4.2 Preparación para la Fase 7 sin implementarla

- `ResponseDto` incluye `gate_count` desde el día uno (viene gratis de
  `dz_api::Response`, ver 2.2) aunque el frontend de la Fase 6 no lo
  muestre. La Fase 7 añade un componente de UI que lee un campo que ya
  está en el DTO — cero cambios en Rust.
- `set_backend` ya es "selector de backend en caliente": la Fase 7 lo
  reutiliza tal cual para su selector con contador de puertas; no hace
  falta diseñar un comando nuevo.
- No se diseña aquí el visor paso a paso de ripple-carry (eso es
  explícitamente Fase 7) ni el export CSV — mencionados solo para constatar
  que la forma de `Response` (campos `Option` añadibles) no los bloquea.

---

## 5. Renderizado de glifos dozenales en la UI

**Decisión: fuente embebida (web font) con `X`/`E` como *fallback* visual
automático vía CSS, no como alias por defecto de los datos.**

Razonamiento:

- A diferencia del modo CLI (§3.4), aquí el "cliente" es un navegador
  embebido (WebView2 en Windows, vía Tauri) que la propia app controla:
  se puede empotrar una fuente TTF/WOFF2 con cobertura garantizada de
  U+218A/U+218B (p. ej. Noto Sans tiene ambos glifos) como recurso
  estático de la app, sin depender de qué tenga instalado el sistema. Esto
  es estrictamente mejor que SVGs propios por glifo: una fuente embebida
  permite que el texto siga siendo texto seleccionable/copiable (import
  para el "exportar traza a CSV" de la Fase 7, y para que el historial se
  pueda copiar-pegar), mientras que SVGs requieren maquetación manual de
  cada dígito, texto no seleccionable, y complican el layout del teclado
  (que necesita 12 teclas con label, dos de las cuales serían imágenes en
  vez de texto).
- Los datos que Rust manda al frontend siempre usan los glifos canónicos
  (igual que el REPL); el frontend nunca decide "voy a mandar X en vez de
  ↋" porque el frontend no toca dígitos, solo muestra strings (regla de
  pureza, ver §6). Si en algún momento se quisiera un modo de accesibilidad
  ASCII dentro de la GUI, sería una preferencia de *presentación* pura:
  una hoja de estilos o un flag de UI que le pide a Rust (vía `set_*`
  futuro) que formatee con alias en vez de con glifos — la lógica de "qué
  alfabeto usar" sigue centralizada en `dz_parse::alphabet`, nunca
  duplicada en CSS/JS.
- Riesgo residual documentado, a validar en la implementación real (ver
  §8): confirmar que la fuente elegida efectivamente cubre ambos glifos y
  se ve bien a los tamaños del teclado antes de comprometerse; si no,
  fallback a un segundo candidato tipográfico antes de considerar SVGs.

---

## 6. Stack de frontend recomendado

**Vanilla TypeScript + Vite, sin framework de componentes (ni React, ni
Vue, ni Svelte).**

Justificación frente al alcance real: la Fase 6 pide un teclado de 12
dígitos + operadores, un display, historial y memoria — cuatro
componentes visuales sin estado compartido complejo ni routing ni
formularios. Node v24 y `cargo-tauri` 2.11.4 ya están instalados (PLAN.md
§11); Tauri 2.x se integra igual de bien con vanilla JS/TS que con
cualquier framework (su plantilla oficial `create-tauri-app` ofrece
"vanilla-ts" como opción de primera clase), y Vite da recarga en caliente
y *bundling* sin configuración adicional.

Meter React/Vue para esto sería sobre-ingeniería: no hay listas grandes que
virtualizar, no hay árboles de componentes profundos, y el propio
principio de pureza del proyecto empuja a que el frontend sea lo más
"tonto" posible (recibe strings, dispara eventos, invoca comandos Tauri) —
justo el tipo de UI donde un framework de gestión de estado no paga su
complejidad. Si la Fase 7 (panel de investigación con visor paso a paso de
circuito) resulta necesitar visualizaciones más ricas (animar la
propagación del acarreo dígito a dígito), esa es la fase en la que
reevaluar si hace falta algo más — no antes.

Estructura sugerida:

```
app/
├─ src-tauri/          # Rust: comandos, AppState, Cargo.toml con dz-api
├─ src/                # TS: keypad.ts, display.ts, history.ts, main.ts
├─ index.html
├─ vite.config.ts
└─ package.json
```

Sin librería de estilos (CSS plano o, como mucho, variables CSS para el
tema) — de nuevo, cuatro componentes no justifican un sistema de diseño.

---

## 7. Orden de implementación sugerido

1. **`dz-api`** (crate nuevo, §2). Es la dependencia de todo lo demás; sin
   esto ni el CLI ni Tauri tienen a qué llamar. Incluye sus tests
   diferenciales contra el `dz-cli::eval_line` actual — esto es también la
   validación de que la extracción no cambió comportamiento.
2. **Modo CLI de una invocación** (§3), en paralelo con el paso 3 una vez
   que `dz-api` esté estable: es el consumidor más simple de `dz-api` (sin
   estado de sesión, sin IPC, sin frontend) y sirve como *smoke test* de la
   API antes de invertir en la GUI. Bloquea solo en `dz-api`.
3. **Andamiaje de Tauri** (`cargo tauri init`, `AppState`, comandos básicos
   `calc`/`set_backend`/`set_mode` de §4) — puede empezar en paralelo con
   el paso 2 (ambos solo dependen de `dz-api`, no entre sí), pero conviene
   que quien lo haga espere a que la forma de `Request`/`Response` esté
   fijada por el paso 1 para no reescribir DTOs a medio camino.
4. **Frontend vanilla TS** (teclado, display, historial) sobre los
   comandos del paso 3. Depende del paso 3 pero no del paso 2.
5. **Selección tipográfica** (§5) puede investigarse en paralelo desde el
   principio (es una decisión de assets, no de código) pero su integración
   real espera a tener el paso 4 con texto en pantalla para probarla.

Los pasos 2 y 3+4 son independientes entre sí una vez completado el 1;
pueden repartirse entre dos personas/sesiones de trabajo sin coordinación
más allá de congelar la forma de `dz_api::{Request, Response, Operation}`
antes de que ambas ramas empiecen.

---

## 8. Riesgos abiertos y decisiones que requieren al usuario

Estas son decisiones que este documento **no** toma por no ser puramente
técnicas, o por depender de una prueba empírica que no se puede hacer sin
escribir código:

1. **Nombre del binario/comando final.** Este documento usa `base12` como
   nombre ilustrativo del modo comando (`base12 sum 1 3`). El binario real
   sigue llamándose `dz-cli` en `Cargo.toml`; hay que decidir si se
   renombra el binario, se añade un alias, o el comando documentado al
   usuario final sigue siendo `dz-cli sum 1 3`. Impacto: cosmético, pero
   visible en toda la documentación de usuario que se escriba después.

2. **Fuente tipográfica concreta para ↊/↋ en la GUI (§5).** Se recomienda
   embeber una fuente con cobertura de U+218A/U+218B, pero cuál
   específicamente (Noto Sans, alguna fuente dozenal dedicada de la
   Dozenal Society) no se ha probado visualmente. Requiere una sesión
   corta de prueba visual en Windows con WebView2 antes de comprometerse,
   idealmente antes del paso 4 del orden de implementación.

3. **Persistencia del historial/memoria de la GUI.** Este documento asume
   que vive en memoria del proceso (`AppState`, se pierde al cerrar la
   app). Si se quiere que sobreviva entre sesiones (archivo local, similar
   a cómo Tauri expone `tauri-plugin-store`), es una decisión de producto
   que añade una dependencia y una superficie de fallos (¿qué pasa si el
   archivo de historial está corrupto?) que no estaba en el alcance
   original de PLAN.md §7 y debería confirmarse antes de implementarla.

4. **Alcance exacto de `--json` en el CLI.** Se especifica un objeto plano
   de cuatro campos (§3.3). Si se anticipa que herramientas externas van a
   depender de este formato como contrato estable, vale la pena decidir
   ahora si se versiona (`{"version":1, ...}`) para no romper consumidores
   cuando la Fase 7 quiera anexar campos de traza más ricos.

5. **¿El modo comando de una invocación necesita soportar `:backend`/`:mode`
   por variable de entorno además de flags** (p. ej. `BASE12_MODE=dfp`),
   para uso cómodo en scripts que hacen muchas llamadas? No está en el
   pedido original; se deja fuera del diseño salvo que se confirme que
   hace falta.

Ninguno de estos cinco puntos bloquea empezar el paso 1 (`dz-api`) del
orden de implementación — todos son decidibles en paralelo o más adelante
sin invalidar el diseño de la API.

---

## 9. Estado de implementación (fase 6 hecha)

Las secciones 1–8 son el diseño previo; esta sección registra lo que se
construyó, dónde se siguió el diseño al pie de la letra y dónde no, y por qué.

### 9.1 Lo que existe

| Pieza | Dónde | Estado |
|---|---|---|
| `dz-api` (capa 5, sin estado) | `crates/dz-api/src/lib.rs` | hecho, 11 tests |
| Modo CLI de una invocación | `crates/dz-cli/src/main.rs` | hecho |
| REPL (mismo binario, sin argumentos) | `crates/dz-cli/src/main.rs` | conservado, ahora sobre `dz-api` |
| Comandos Tauri + `AppState` | `app/src-tauri/src/{commands,state}.rs` | hecho, compila |
| Frontend vanilla TS + Vite | `app/src/*.ts`, `app/index.html` | hecho, `npm run build` en verde |
| Registro de funciones + científicas | `crates/dz-eval/src/{functions,builtins}.rs`, `crates/dz-sci` | hecho, ver `docs/05-funciones-cientificas.md` |

Tests: 108 en el workspace (eran 69), clippy limpio, `tools/lint-purity.ps1` en
verde con las cuatro reglas intactas.

### 9.2 Desviaciones respecto al diseño, y su motivo

1. **Hay una capa nueva más, `dz-sci` (capa 2.5).** El diseño asumía que la
   fase 6 no tocaría nada por debajo de `dz-eval`. El pedido de operaciones
   científicas (coseno, seno, …) obligó a decidir dónde vive el análisis
   numérico: meterlo en `dz-eval` habría mezclado "cómo se recorre un AST" con
   "cómo converge una serie de Taylor". `dz-sci` depende solo de `dz-num` y no
   conoce el AST; `dz-eval` la usa, `dz-api` ni la nombra salvo para el valor
   por defecto de la escala.

2. **`DozOp`/`OpRegistry` fueron reemplazados por `DozFunction`/`FnRegistry`,
   no conservados en paralelo.** El §1.2(c) proponía conservar `DozOp` sin
   cambios como punto de extensión del lenguaje de expresiones. En la práctica
   `DozOp` nunca llegaba a ejecutarse: no existía `Expr::Call`, así que
   `evaluate` no tenía forma de invocarlo, y su firma (`&[DzFixed]`) dejaba
   fuera el modo racional, que es el modo por defecto del REPL. Mantener dos
   registros —uno muerto y uno vivo— habría sido peor que migrar. `abs`, el
   ejemplo que ilustraba `DozOp`, sigue existiendo como función registrada.

3. **`Response` tiene un campo más de lo diseñado: `approximate`.** El §2.2
   preveía `display`, `periodic_note` y `gate_count`. Con funciones
   trascendentes hace falta distinguir "1/3 exacto" de "seno truncado a doce
   dígitos": en modo racional un resultado aproximado se muestra en punto fijo
   con prefijo `~` en vez de como una fracción de doce dígitos que afirmaría
   una exactitud inexistente. `Request` gana igualmente `scale`, la precisión
   de trabajo de esas funciones.

4. **Los operandos de las operaciones nombradas se parsean con `parse_str`, no
   re-lexando el literal.** El §2.2 proponía `dz_parse::lex` para obtener
   `int_digits`/`frac_digits`. Usar el parser completo acepta cualquier
   expresión donde se esperaba un literal (`sum "1+1" "2"` funciona) y deja un
   único camino de texto→AST en vez de dos. Sigue sin haber una segunda ruta
   que convierta texto en dígitos: es el lexer de `dz-parse`, como se pedía.

5. **La coma tiene dos significados, resueltos por contexto léxico.** Separador
   de argumentos dentro de la lista de una llamada, separador de agrupación de
   cuatro dígitos fuera (`1,0000`). El lexer lleva una pila de paréntesis y
   sabe en cuál está, así que la regla es posicional y exacta, no heurística.
   Coste documentado para el usuario: dentro de una llamada hay que escribir
   `10000`, no `1,0000`.

6. **Identificadores frente a alias de dígitos.** `x`, `t` y `e` son alias
   ASCII de dígitos, así que `exp` o `tan` chocarían con ellos. Regla: una
   secuencia maximal de letras es un identificador si y solo si **alguna** de
   sus letras no es alias de dígito. `E` y `XE` siguen siendo números; `exp`,
   `tan` y `pi` son nombres. Ningún literal que antes lexaba como número dejó
   de hacerlo.

7. **El CLI de una invocación acepta como subcomando cualquier función del
   registro** (`dz-cli sin 1`), además del vocabulario fijo del §3.2. Sale
   gratis del catálogo y evita mantener una lista paralela. Se añadieron
   `--scale`, `--gates` y `--functions` a las banderas diseñadas.

8. **Glifos en la GUI: detección en tiempo de ejecución en vez de fuente
   empotrada** (§5). El frontend mide en un `canvas` el ancho de `↊`/`↋` contra
   un carácter de uso privado (garantizado sin glifo); si coinciden, no hay
   glifo y toda la presentación cae a los alias `X`/`E`. Los datos que manda
   Rust siguen usando siempre los glifos canónicos, y la sustitución es
   estrictamente de presentación. Empotrar una fuente sigue siendo la mejora
   natural (§8.2 sigue abierta), pero ahora el fallo tipográfico ya no rompe la
   app en una máquina sin fuente adecuada.

### 9.3 Las cinco decisiones abiertas del §8

1. **Nombre del binario**: sigue siendo `dz-cli` (`dz-cli sum 1 3`). El
   documento usaba `base12` como nombre ilustrativo; renombrar es cosmético y
   se deja al usuario.
2. **Fuente para ↊/↋**: sin decidir. Mitigado con el fallback automático
   (9.2.8); la prueba visual sigue pendiente.
3. **Persistencia del historial**: no se implementó. Vive en `AppState` y se
   pierde al cerrar, como asumía el diseño.
4. **Versionado de `--json`**: no se versionó. El objeto ganó el campo
   `approximate`; si alguna herramienta externa llega a depender del formato,
   ese es el momento de añadir `"version"`.
5. **Variables de entorno para modo/backend**: no implementadas, no hicieron
   falta.

### 9.4 Cómo se corre

```bash
cargo run -p dz-cli                      # REPL
cargo run -p dz-cli -- sin 1 --gates     # una invocación
cargo run -p dz-cli -- --functions       # catálogo de funciones

cd app && npm install                    # una vez
npm run tauri dev                        # GUI en desarrollo
npm run tauri build                      # instalador
```
