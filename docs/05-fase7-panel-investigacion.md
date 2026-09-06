# 05 — Fase 7: panel de investigación

Estado de partida (rama `feat/balanced-half-adder`, commit `9b72211`):
Fase 6 completa y más amplia de lo diseñado — `dz-api`, la GUI Tauri, un
crate `dz-sci` de funciones científicas y un modo CLI de una invocación en
`dz-cli`. Todos los tests en verde (≈108, `cargo test --workspace
--release`), clippy limpio, lint de pureza OK. Este documento diseña la
Fase 7 (PLAN.md §7 y §10bis-C) sin implementarla, verificando contra el
código real qué ya existe y qué falta.

---

## 1. Estado real verificado

### 1.1 Selector de backend en caliente — ya existe, completo

`app/src-tauri/src/commands.rs:170-176`, comando `set_backend`, y
`app/src/main.ts:279-281` lo conecta a un `<select id="backend">`
(`app/index.html:43-44`), poblado por `SessionDto.backends` (`commands.rs:96`,
`dz_api::BackendKind::ALL`). El comentario en el propio código
(`commands.rs:168-169`) ya lo dice: *"already what the Phase 7 research
panel needs, so that phase adds a chart, not a command."* No falta nada
del lado de la mecánica de cambio de backend. Lo que falta es lo que un
panel de *investigación* añade por encima de un simple `<select>`: ver
§5 (comparación de los tres backends para la misma operación en una sola
vista, hoy no existe ningún comando ni UI que ejecute los tres a la vez).

### 1.2 Contador de puertas de la última operación — ya existe el desglose, falta la profundidad y la comparación

- `dz_api::Response.gate_count: Option<GateCount>` (`crates/dz-api/src/lib.rs:172`),
  poblado siempre en éxito (`lib.rs:314`, `g.reset_cost()` en `lib.rs:265`
  antes de despachar).
- `GateCountDto` (`commands.rs:65-73`) y `ResponseDto.gate_count`
  (`commands.rs:83`) ya cruzan el puente completos, con los cinco
  contadores (`cycle/min/max/lit/thr`) más `total`.
- El frontend **ya pinta el desglose**, no solo el total:
  `app/src/main.ts:151-164` (`paintGates`) escribe en
  `#g-cycle/#g-min/#g-max/#g-lit/#g-thr/#g-total`
  (`app/index.html:56-61`), con formato regional (`toLocaleString("es")`).
  Es decir: el punto 2 del pedido de PLAN.md ya está construido en su
  forma más básica.

**Lo que falta** es lo que convierte esa tabla en instrumento de
investigación:
- **Profundidad de circuito.** `dz-api`/`ResponseDto` no exponen
  profundidad en absoluto — solo `GateCount`. La instrumentación existe en
  `dz-gates` (`circuits/depth.rs`, `Levels`, `Sig`, `max_level`, y en
  `circuits/adder.rs:432-503`: `half_adder_depth`, `ripple_add_depth`,
  `lookahead_add_depth` + sus variantes `_balanced`), pero es una API de
  **circuitos concretos de la Fase 8** (medir un `half_adder`/`ripple_add`
  aislado, en programas de ejemplo/tests), no de "la última operación del
  usuario" a través de `dz-eval`/`dz-api`. Una expresión de usuario típica
  encadena `ripple_add`, `multiply`, `div_rem`, etc., y hoy no hay ninguna
  función que devuelva "la profundidad total del circuito que evaluó esta
  expresión" — hace falta diseñarla (§2).
- **Comparación entre backends para la misma operación.** `dz-api::execute`
  ya puede correr una `Request` con cualquiera de los tres `BackendKind`
  (de hecho el test `all_three_backends_agree_on_the_answer`,
  `crates/dz-api/src/lib.rs:477-488`, ya lo hace), pero no hay ningún
  comando Tauri ni vista que ejecute los tres de una vez y muestre los
  tres `gate_count` lado a lado. Es "fruta al alcance de la mano", como
  dice el pedido — diseño en §5.

### 1.3 Visor paso a paso del ripple-carry — no existe, es lo que más diseño requiere

No hay ninguna función en `dz-gates`, `dz-api` ni en el frontend que
devuelva el estado intermedio dígito a dígito de una suma. Lo que existe:

- `ripple_add`/`ripple_add_net` (`circuits/adder.rs:187-205`) calculan el
  resultado dígito a dígito internamente (`full_adder_net` por posición,
  con `carry` fluyendo de una iteración a la siguiente), pero **descartan
  el estado intermedio**: la función solo devuelve el `Vec<Digit>` final.
- `lookahead_add`/`lookahead_add_net` calculan cada posición mediante el
  árbol de Kogge-Stone (`sum_gen_prop_net` + prefijos), así que "acarreo
  propagándose dígito a dígito" ni siquiera es una descripción fiel de su
  ejecución interna — es un circuito de otra forma. **El visor paso a paso
  tiene que ser sobre `ripple_add` específicamente**, que es el que
  PLAN.md §7 pide ("ver el acarreo propagarse dígito a dígito") y el único
  de los dos cuya ejecución es literalmente secuencial.
- El patrón `GateNet`/`Values`/`Levels` de `circuits/depth.rs` es
  exactamente la plantilla a reutilizar para no escribir un tercer
  `ripple_add` (ver §2): una instanciación nueva de `GateNet` que, además
  de calcular el valor, acumula una traza, sin tocar el circuito en sí.

### 1.4 Exportar traza a CSV — no existe, depende del punto anterior

Sin datos de traza no hay nada que exportar. Diseño en §4, condicionado al
tipo de traza de §2.

### 1.5 `dz-sci` y el panel de investigación

`crates/dz-sci` (constants.rs, elementary.rs, exact.rs, trig.rs, util.rs)
implementa funciones aproximadas (`sin`, `cos`, `ln`, `sqrt`, ...) sobre
`DzFixed`, con la disciplina de pureza documentada en su propio
`lib.rs:1-40`: toda la aritmética pasa por `DozGates`, así que **cada
llamada a una función científica también hace un número de invocaciones de
puerta perfectamente real y medible** — no hay una categoría "las
científicas quedan fuera del conteo". De hecho `dz_api::execute` ya cuenta
esto sin distinción: una `Request` con `Operation::Call { name: "sin", .. }`
pasa por el mismo `g.reset_cost()`/`g.cost()` que una suma
(`crates/dz-api/src/lib.rs:255-316`), y el test
`approximate_results_are_marked_and_shown_as_digits` (`lib.rs:441-454`)
confirma que el camino se ejecuta con normalidad.

**Decisión para la Fase 7: el panel de investigación mide lo que
`gate_count`/profundidad ya miden, sin distinguir "función científica" de
"operación aritmética".** No hace falta ninguna categoría especial. La
única consecuencia práctica es que el conteo de una función como `sin`
será mucho mayor (itera una serie con varias multiplicaciones/divisiones
`DzFixed` por término) — eso es información de investigación legítima
("cuánto cuesta `sin` en puertas", una pregunta que el panel debería poder
responder tal cual, sin trabajo adicional), no un caso especial a filtrar.
Lo único que el panel de profundidad (§2) *no* puede hacer todavía es
descomponer el costo de `sin` en el árbol de circuitos que lo compone
(cuántos `ripple_add`, cuántos `multiply`), porque eso requeriría
instrumentar `dz-sci`/`dz-int`/`dz-num` con el mismo patrón `GateNet` que
hoy solo cubre `dz-gates::circuits::adder` — se deja fuera de esta fase
explícitamente (ver §7 Riesgos).

---

## 2. Extensión de API para el visor paso a paso

### 2.1 Dónde vive la instrumentación de traza: `dz-gates`, patrón `GateNet`

Igual que la profundidad, la traza **no puede vivir fuera de `dz-gates`**:
solo el circuito mismo sabe, dígito a dígito, qué `x_i`/`y_i`/`carry_in`
alimentó a qué `full_adder`. Reconstruirla desde fuera (p. ej. llamando a
`ripple_add` repetidamente con prefijos crecientes) cambiaría qué se mide
y sería frágil ante cualquier cambio futuro del circuito. Sigue,
literalmente, la nota que ya deja `circuits/depth.rs:1-47`: escribir el
circuito una sola vez, genérico sobre `GateNet`, e instanciarlo con una
tercera implementación en vez de duplicar `ripple_add_net`.

**Nueva instanciación en `crates/dz-gates/src/circuits/depth.rs`** (o un
módulo hermano `trace.rs` si se prefiere no mezclar los dos conceptos —
recomendado, ver más abajo), junto a `Values`/`Levels`:

```rust
// crates/dz-gates/src/circuits/trace.rs
//! Per-position trace instrumentation for `ripple_add`, via the same
//! `GateNet` abstraction `circuits::depth` uses for circuit depth. A third
//! instantiation, not a third copy of the circuit: `ripple_add_net` in
//! `circuits::adder` is written once and this module supplies the signal
//! type that makes a `ripple_add_net` run also record one `StepRecord` per
//! digit position, with zero effect on `Values`-based (production) calls.

use crate::digit::Digit;
use crate::traits::DozGates;
use std::cell::RefCell;

/// One digit position's worth of what the step-by-step viewer shows:
/// the two input digits, the carry that flowed in, the sum digit and the
/// carry that flows out. All fields are already-resolved `Digit`s -- the
/// viewer (frontend) never recomputes anything from them, it only
/// displays them in sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StepRecord {
    pub position: usize, // usize bookkeeping: which digit place, not a value
    pub a: Digit,
    pub b: Digit,
    pub carry_in: Digit,
    pub sum: Digit,
    pub carry_out: Digit,
}

/// Trace-recording instantiation: signals are plain `Digit`s (so values
/// and gate counts against `g` are byte-for-byte identical to `Values`),
/// but every full-adder application also appends one `StepRecord` to an
/// interior-mutable log. `RefCell`, not `Cell`, because the recorded value
/// is a `Vec`, not `Copy` -- the same category of interior mutability
/// `gatecount::Counter` already uses for its own bookkeeping, just for a
/// log instead of a running tally.
pub struct Traced<'g, G: DozGates> {
    inner: Values<'g, G>,
    log: RefCell<Vec<StepRecord>>,
}

impl<'g, G: DozGates> Traced<'g, G> {
    pub fn new(g: &'g G) -> Self {
        Traced { inner: Values(g), log: RefCell::new(Vec::new()) }
    }
    pub fn into_log(self) -> Vec<StepRecord> {
        self.log.into_inner()
    }
}

impl<G: DozGates> GateNet for Traced<'_, G> {
    type S = Digit;
    // cycle/min/max/lit/thr/lift/value: delegate straight to `self.inner`,
    // identical to `Values` -- recording happens at a higher level (see
    // below), not inside the primitive gates themselves.
    ...
}
```

**Punto de diseño clave: la traza no se graba dentro de `GateNet`, sino en
una variante de `ripple_add` que conoce la posición.** `GateNet` no tiene
noción de "posición dentro de una suma multi-dígito" — esa estructura vive
en `ripple_add_net` (el `while i < n` de `circuits/adder.rs:187-205`), así
que la forma correcta de no duplicar el circuito es parametrizar
`ripple_add_net` sobre un *sumidero de traza* opcional, no sobre un
`GateNet` de traza:

```rust
// circuits/adder.rs -- no new copy of the loop, an optional callback.
fn ripple_add_net_traced<N: GateNet>(
    net: &N,
    a: &[N::S],
    b: &[N::S],
    mut on_step: impl FnMut(usize, N::S, N::S, N::S, N::S, N::S),
    // (position, a_i, b_i, carry_in, sum, carry_out)
) -> Vec<N::S> {
    // identical body to ripple_add_net, plus one on_step(...) call per
    // iteration of the existing while loop.
}

fn ripple_add_net<N: GateNet>(net: &N, a: &[N::S], b: &[N::S]) -> Vec<N::S> {
    ripple_add_net_traced(net, a, b, |_, _, _, _, _, _| {})
}
```

Esto es estrictamente menos invasivo que una tercera instanciación de
`GateNet`: **una función existente gana un parámetro con un callback
vacío por defecto**, y el circuito sigue existiendo en un solo lugar. La
entrada pública nueva:

```rust
/// Step-by-step trace of `ripple_add`: same result as `ripple_add`, plus
/// one `StepRecord` per digit position, in order from least significant
/// to most significant. Built for the Phase 7 step-by-step viewer.
///
/// Deliberately `ripple_add`-only: `lookahead_add`'s Kogge-Stone network
/// has no "carry crawls through position i" step to show -- tracing it
/// would mean exposing the prefix-doubling stages instead, a different UI
/// concept, out of scope here (see docs/05, section 7).
pub fn ripple_add_trace<G: DozGates>(g: &G, a: &[Digit], b: &[Digit]) -> (Vec<Digit>, Vec<StepRecord>) {
    let mut steps = Vec::new();
    let sum = ripple_add_net_traced(&Values(g), a, b, |position, a_i, b_i, cin, s, cout| {
        steps.push(StepRecord { position, a: a_i, b: b_i, carry_in: cin, sum: s, carry_out: cout });
    });
    (sum, steps)
}
```

### 2.2 No contaminación (mismo rigor que la Fase 8)

- `ripple_add` público **no cambia**: sigue siendo `ripple_add_net_traced`
  con un callback vacío, así que su `GateCount` medido es idéntico
  (verificable con el mismo test de regresión de `ripple_add(4,4) = 1212`
  que ya existe).
- Grabar un `StepRecord` es empujar a un `Vec` y leer valores `Digit` ya
  calculados por las primitivas — no invoca ninguna primitiva adicional,
  así que `ripple_add_trace` cuesta exactamente lo mismo en `GateCount`
  que `ripple_add` sobre las mismas entradas. Un test de regresión debe
  fijar esto explícitamente: `g.cost()` después de `ripple_add_trace`
  debe ser igual a `g.cost()` después de `ripple_add` con las mismas
  entradas y un backend reseteado.
- `StepRecord` no lleva ninguna aritmética nativa sobre los valores: los
  campos `a/b/carry_in/sum/carry_out` son `Digit` opacos, tal como llegan
  de `full_adder_net`; `position` es el mismo `i: usize` que ya recorre
  el bucle existente, aritmética de índice sancionada.
- **No se mezcla con `Levels`/profundidad.** Un `Traced` no calcula
  niveles y un `Levels` no graba pasos — son instrumentaciones
  ortogonales sobre el mismo circuito, cada una activada explícitamente
  por quien la necesita, igual que hoy conteo y profundidad se miden en
  pasadas separadas.

### 2.3 Cómo cruza `dz-api` → Tauri → frontend

`dz-api` gana una función nueva, **no un campo nuevo en `Response`**: la
traza es un dato pesado y específico de una operación (una suma binaria
concreta), mientras que `Response` es el resultado general de *cualquier*
operación (`sum`, `sin`, `eval` de una expresión completa). Forzar
`Response.trace: Option<Vec<StepDto>>` obligaría a que toda la maquinaria
de `evaluate_rational_with`/`evaluate_fixed_with` supiera producir una
traza para expresiones arbitrarias (¿qué es la traza de `4;6 * 3 + sin(1)`?
— no hay una respuesta única, y no es el problema que este visor resuelve).

En vez de eso:

```rust
// crates/dz-api/src/lib.rs -- new, narrow entry point alongside `execute`.

/// One step of a ripple-carry addition, already resolved to display
/// glyphs by `dz-parse` -- nothing here is a number the frontend could
/// compute with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceStep {
    pub position: usize,
    pub a: char,
    pub b: char,
    pub carry_in: char,
    pub sum: char,
    pub carry_out: char,
}

/// A request for the two operands of a *plain integer* ripple-carry
/// addition -- deliberately narrower than `Request`: the step-by-step
/// viewer is defined over `dz-int::Dz` addition, not over the general
/// expression language, an approximation, or the other three arithmetic
/// operations. `Sub`/`Mul`/`Div` build on top of add/complement/repeated
/// add in ways that would each need their own trace shape (see section 7).
pub struct TraceRequest {
    pub backend: BackendKind,
    pub a: String,
    pub b: String,
}

pub fn trace_add(req: &TraceRequest) -> Result<Vec<TraceStep>, ApiError> {
    // 1. Lex `a`/`b` with `dz_parse::lex` into digit vectors (same lexer,
    //    no second text-to-Digit path), reject anything that isn't a
    //    plain non-negative integer literal (no expressions here: a trace
    //    over "1+2 add 3" is not a well-defined single ripple-carry run).
    // 2. Dispatch on `req.backend` to a fresh `MvlGates`/`CrtGates`/`TableGates`,
    //    call `dz_gates::circuits::ripple_add_trace`.
    // 3. Map each `StepRecord` to a `TraceStep` via `dz_parse::alphabet::digit_to_char`
    //    -- the same, single formatting authority `format_fixed` already uses.
}
```

Este diseño mantiene la regla de "strings/tokens ya resueltos, nunca algo
recomponible": `TraceStep` lleva `char`, no `Digit`, así que ni siquiera
hace falta que el DTO de Tauri sepa qué es un `Digit` — es literalmente
texto de un carácter por campo. El frontend no puede "sumar" un `char`.

Comando Tauri nuevo (`app/src-tauri/src/commands.rs`):

```rust
#[derive(Debug, Serialize)]
pub struct TraceStepDto {
    pub position: usize,
    pub a: String,
    pub b: String,
    pub carry_in: String,
    pub sum: String,
    pub carry_out: String,
}

#[tauri::command]
pub fn trace_add(state: State<AppState>, a: String, b: String) -> Result<Vec<TraceStepDto>, String> {
    let backend = state.0.lock().map_err(|_| "session lock poisoned".to_string())?.backend;
    dz_api::trace_add(&dz_api::TraceRequest { backend, a, b })
        .map(|steps| steps.into_iter().map(|s| TraceStepDto { /* char -> String */ }).collect())
        .map_err(|e| e.to_string())
}
```

No usa el `Session.mode` (racional/fijo/DFP): una traza de ripple-carry es
inherentemente sobre enteros dígito a dígito, independiente del modo de
visualización del resultado final. Si el operando de usuario trae un
punto Humphrey, la interfaz decide si lo rechaza (mensaje: "el visor de
circuito trabaja sobre enteros; usa la parte entera") o si trocea en
parte entera/fraccionaria y llama dos veces — se deja como decisión de UI
en §3, no de API.

---

## 3. Diseño del panel en el frontend

### 3.1 Ubicación: pestaña nueva, no una sección más de la barra lateral

La barra lateral actual (`app/index.html:35-83`) ya tiene tres secciones
(`Sesión`, `Coste de la última operación`, `Memoria`, `Historial`) en un
`<aside>` angosto pensado para una calculadora. El panel de investigación
añade contenido que no cabe ahí sin abrumar la vista principal (una tabla
de comparación de tres backends, y sobre todo un visor de pasos que
necesita espacio horizontal para mostrar 8-12 posiciones dígito a dígito).

**Diseño: una pestaña `Investigación` en el panel lateral**, que reemplaza
el contenido de `<aside class="panel side">` cuando está activa (toggle
con la sección `Sesión`/histórico existente, que sigue siendo la vista por
defecto). Esto evita reestructurar el layout de dos columnas ya construido
y mantiene el frontend "tonto": una pestaña es un `display: none`/`block`
sobre contenedores ya poblados por Rust, no lógica nueva.

Widgets, cada uno mapeado a lo que ya existe o a lo diseñado en §2/§5:

1. **Desglose de puertas de la última operación** — ya existe
   (`#g-cycle`...`#g-total`), se traslada tal cual a la pestaña nueva.
2. **Profundidad de la última operación** — fila nueva `depth` junto a
   `total` en la misma tabla. Requiere que `ResponseDto`/`GateCountDto`
   (o un campo hermano) exponga profundidad — ver nota de alcance en §7:
   esto requiere que `dz-eval`/`dz-api` sepan pedir profundidad además de
   conteo para *cualquier* expresión, no solo para `ripple_add` aislado, lo
   cual hoy no existe (§1.2) y se marca como decisión pendiente.
3. **Tabla comparativa de backends** (§5): tres columnas (MVL/CRT/Table),
   una fila por primitiva + total, para la operación que el usuario acaba
   de evaluar. Se dispara con un botón "Comparar backends" junto al
   resultado, no automáticamente en cada tecla — ejecutar tres backends en
   cada pulsación sería trabajo desperdiciado la mayoría de las veces que
   alguien solo quiere el resultado.
4. **Visor paso a paso del ripple-carry**: un formulario propio (dos
   campos de entrada para los operandos enteros, un botón "Trazar suma"),
   independiente del display principal de la calculadora — no reutiliza
   `#expression`, porque una traza es sobre una suma binaria concreta, no
   sobre una expresión general (§2.3). Al recibir los `TraceStepDto`,
   renderiza una fila por posición (de la menos significativa a la más
   significativa, o invertido para lectura izquierda-a-derecha — decisión
   de UI menor) mostrando `a_i, b_i, acarreo-entrante → suma_i,
   acarreo-saliente`, y un control de reproducción simple
   (anterior/siguiente/todo) que solo cambia qué fila está resaltada — la
   secuencia completa ya llegó de Rust en un solo `invoke`, no hay
   *streaming* ni estado a reconstruir del lado JS.
5. **Botón "Exportar CSV"** junto a cada uno de los tres widgets de datos
   (gate count, comparación de backends, traza) — ver §4.

### 3.2 Cuándo se piden los datos a Tauri

- Desglose de puertas: ya llega gratis con cada `calc()` (sin cambios).
- Profundidad: mismo ciclo de vida que el desglose, si se implementa como
  parte de `ResponseDto` (ver alcance en §7).
- Comparación de backends: bajo demanda (clic en "Comparar backends"),
  usando el último `input`/`OperationDto` evaluado — la interfaz ya
  conserva `lastResult`/la última expresión en `main.ts:125`, basta con
  guardar también la última `OperationDto` para poder re-ejecutarla contra
  los tres backends sin pedirle al usuario que la reescriba.
- Traza de suma: bajo demanda, formulario independiente, sin relación con
  el ciclo de vida de `calc()`.

---

## 4. Diseño del export CSV

**Decisión: Rust arma el CSV completo; el frontend solo dispara la
descarga de un `string` ya recibido.** Coherente con la regla ya aplicada
en todo el proyecto ("el frontend no toca semántica dozenal", y aquí ni
siquiera hace falta tocar semántica: es formato de texto, pero mantener
*una* autoridad de formato evita que mañana alguien "arregle una coma" en
el CSV del lado JS mientras Rust sigue pensando que el formato es otro).
Además, calcular percentiles/ordenar/agregar antes de imprimir (si el CSV
alguna vez agrega algo más que una fila por dato) es más natural en Rust,
que ya tiene los tipos.

```rust
// crates/dz-api/src/lib.rs

/// One row of the gate/depth-comparison CSV: one backend's cost for one
/// operation.
pub fn gate_count_csv(rows: &[(BackendKind, GateCount)]) -> String {
    let mut out = String::from("backend,cycle,min,max,lit,thr,total\n");
    for (backend, count) in rows {
        out.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            backend.label(), count.cycle, count.min, count.max, count.lit, count.thr, count.total()
        ));
    }
    out
}

/// One row per digit position of a ripple-carry trace.
pub fn trace_csv(steps: &[TraceStep]) -> String {
    let mut out = String::from("position,a,b,carry_in,sum,carry_out\n");
    for s in steps {
        out.push_str(&format!("{},{},{},{},{},{}\n", s.position, s.a, s.b, s.carry_in, s.sum, s.carry_out));
    }
    out
}
```

Comando Tauri: `export_gate_csv`/`export_trace_csv` devuelven el `String`
ya armado; el frontend lo recibe y usa el plugin de sistema de archivos de
Tauri (`@tauri-apps/plugin-dialog` + `@tauri-apps/plugin-fs`, o el enlace
nativo `save`) para escribirlo a disco, o simplemente crea un
`Blob`/`URL.createObjectURL` para un enlace de descarga si se prefiere no
añadir el plugin de FS todavía — decisión de implementación menor, sin
impacto en la pureza porque en ningún punto se parsea ni recalcula el CSV
del lado JS, solo se persiste.

Columnas elegidas explícitamente:

- **Gate count**: `backend,cycle,min,max,lit,thr,total` — una fila por
  backend, reutilizable tanto para "esta operación en un solo backend"
  (una fila) como para la comparación de tres (§5, tres filas).
- **Traza**: `position,a,b,carry_in,sum,carry_out` — exactamente los
  campos de `TraceStep`, sin transformación.

Si en el futuro se pide un CSV de profundidad (ver alcance §7), se añade
un `depth_csv` con la misma disciplina, no se sobrecargan las columnas
existentes con un campo opcional.

---

## 5. Comparación de backends en la misma vista

Ya identificado como "fruta al alcance de la mano" porque `dz_api::execute`
acepta cualquier `BackendKind` por request y los tres ya están probados
para coincidir en el resultado (`crates/dz-api/src/lib.rs:477-488`). Diseño:

```rust
// crates/dz-api/src/lib.rs

/// Run the same operation against all three backends and report each
/// one's gate cost, for the research panel's backend-comparison table.
/// The *value* is asserted equal across backends (a mismatch would be a
/// correctness bug, not a research finding) and returned once.
pub struct BackendComparison {
    pub display: String, // shared answer, from whichever backend ran first
    pub costs: [(BackendKind, GateCount); 3],
}

pub fn compare_backends(op: &Operation, mode: NumberMode, scale: usize) -> Result<BackendComparison, ApiError> {
    let mut costs = Vec::with_capacity(3);
    let mut display = None;
    for backend in BackendKind::ALL {
        let req = Request { backend, mode, scale, op: op.clone() };
        let response = execute(&req)?;
        if let Some(d) = &display {
            debug_assert_eq!(d, &response.display, "backends disagree -- correctness bug, not a research result");
        } else {
            display = Some(response.display.clone());
        }
        costs.push((backend, response.gate_count.expect("a successful call always counts gates")));
    }
    Ok(BackendComparison { display: display.unwrap(), costs: costs.try_into().unwrap() })
}
```

Comando Tauri `compare_backends(op: OperationDto) -> Result<ComparisonDto, String>`,
llamado bajo demanda desde el botón descrito en §3.1. La tabla en el
frontend es literalmente tres columnas de la misma tabla de desglose que
ya existe (`#g-cycle` etc.), una por backend — reutiliza el mismo
componente de render, parametrizado por una lista de `(label, GateCountDto)`
en vez de uno solo.

Nota honesta: para el backend `Table` (oráculo, no de producción), la
comparación no es interesante en sí misma para "¿cuál es más barato?"
porque su costo por diseño es cero puertas en el sentido MVL (indexa
memoria directamente — ver `docs/01-diseno-sumador.md`); mostrarlo en la
tabla sigue siendo correcto (es el dato real) pero el panel debería
etiquetarlo como "no es un backend de producción" para no invitar a leer
"Table gana" como una conclusión de investigación válida — mismo cuidado
que ya tiene `docs/02-crt-vs-mvl.md` con CRT.

---

## 6. Orden de implementación sugerido

1. **`ripple_add_net_traced` + `ripple_add_trace` en `dz-gates`** (§2.1),
   con su test de no-contaminación de `GateCount` (§2.2) y tests
   diferenciales de que la traza reconstruye exactamente el mismo `Vec<Digit>`
   que `ripple_add`. Es la pieza más delicada (toca `dz-gates`, requiere el
   mismo rigor de pureza que la Fase 8) y todo lo demás depende de ella.
2. **`dz_api::{TraceStep, TraceRequest, trace_add, compare_backends,
   gate_count_csv, trace_csv}`** (§2.3, §4, §5) — depende solo del paso 1
   y de tipos que ya existen (`BackendKind`, `GateCount`). Puede empezar en
   paralelo con el paso 3 de la Fase 6 en su momento; aquí no hay
   paralelismo real porque es el único bloqueante de los comandos Tauri.
3. **Comandos Tauri** (`trace_add`, `compare_backends`, `export_*_csv`) y
   sus DTOs — mecánico una vez fijados los tipos del paso 2.
4. **Frontend**: pestaña "Investigación", tabla de comparación de
   backends, visor paso a paso, botones de exportación — puede
   desarrollarse con datos de prueba fijos (*mocks*) en paralelo a los
   pasos 1-3 si se fija primero la forma de los DTOs (igual que se hizo
   para la Fase 6), y conectarse al final.
5. **Profundidad de "la última operación" general** (no solo de
   `ripple_add`/`lookahead_add` aislados) — deliberadamente al final,
   porque depende de una decisión de alcance no resuelta (§7, punto 1):
   si se resuelve que sí entra en esta fase, es una extensión de
   `dz-eval`/`dz-api` más grande que el resto de este documento junto.

---

## 7. Riesgos y decisiones que requieren al usuario

1. **¿La profundidad debe medirse para *cualquier* expresión de usuario, o
   solo para `ripple_add`/`lookahead_add` invocados explícitamente desde
   el panel?** Este es el riesgo de diseño más grande de la Fase 7. Hoy
   `half_adder_depth`/`ripple_add_depth`/`lookahead_add_depth` son
   funciones de investigación de la Fase 8 que miden **un circuito
   aislado con entradas concretas** (`crates/dz-gates/src/circuits/adder.rs:432-460`).
   `dz-int::Dz::add`/`mul`/`div_rem` y `dz-sci` **no** están instrumentados
   con `GateNet`/`Levels` — solo llaman a las funciones `Values`-based de
   `dz-gates`. Extender la profundidad a "la última operación evaluada por
   `dz-eval`" (una suma de enteros de precisión arbitraria, o peor, un
   `sin(x)` con decenas de multiplicaciones internas) exigiría instrumentar
   `dz-int::dzint.rs` y `dz-num`/`dz-sci` con el mismo patrón `GateNet`
   punto por punto — una obra considerablemente mayor que exponer `ripple_add_trace`,
   y con su propio riesgo de contaminación en cada capa. **Se necesita que
   el usuario decida** si la Fase 7 se entrega con profundidad solo para
   circuitos de un dígito/una llamada aislada (alcance chico, ya casi
   listo) o si se compromete a instrumentar las capas de enteros/racionales
   también (alcance grande, probablemente su propia sub-fase).

2. **Alcance de la traza: ¿solo suma, o también resta/multiplicación?**
   Este documento decide (§2.3) que la traza cubre únicamente
   `ripple_add` sobre enteros, siguiendo la letra de PLAN.md §7 ("ver el
   acarreo propagarse"). `Dz::sub` es complemento a 12 + una suma
   (`dz-int/src/dzint.rs`, `sub_magnitude`), así que técnicamente podría
   reutilizar `ripple_add_trace` sobre el complementado, pero mostrarle
   eso a un usuario sin contexto ("¿por qué mi resta se traza como una
   suma de un número raro?") necesita una explicación de UI que este
   documento no diseña. La multiplicación no tiene una noción de "un
   acarreo que se propaga dígito a dígito" (es `scale_by_digit` +
   `ripple_add` por cada dígito del multiplicador) — trazarla sería un
   visor distinto, de otra forma. **Confirmar con el usuario si la Fase 7
   se cierra con solo el visor de suma**, dejando resta/multiplicación
   para una fase posterior, o si se espera algo más ahora.

3. **¿El comando `compare_backends` corre siempre los tres backends, o
   deja elegir cuáles?** El diseño de §5 asume "siempre los tres" (son
   solo 12 valores dozenales de peor caso, nunca lento). Si en el futuro
   se añaden más backends experimentales, esta decisión debería
   revisarse, pero no bloquea esta fase.

4. **Persistencia del CSV**: §4 deja abierto si se usa el plugin de
   sistema de archivos de Tauri (requiere añadir una dependencia:
   `@tauri-apps/plugin-fs`/`plugin-dialog`, hoy no instalada según
   `app/package.json`) o una descarga vía Blob del navegador embebido (sin
   dependencias nuevas, pero peor integrada con el diálogo nativo "Guardar
   como"). Es una decisión de producto menor pero requiere que el usuario
   apruebe añadir la dependencia si se prefiere la primera opción.

5. **Nombre y ubicación de la pestaña "Investigación"** (§3.1): este
   documento asume que reemplaza el contenido del `<aside>` en vez de
   abrir una ventana/panel separado de Tauri. Si se prefiere una segunda
   ventana (más fiel a "panel de investigación" como algo aparte de la
   calculadora), es un cambio de alcance de la Fase 6 (multi-ventana en
   Tauri) que vale la pena decidir antes de escribir la pestaña.

Ninguno de estos cinco puntos bloquea el paso 1 del orden de
implementación (§6): `ripple_add_trace` en `dz-gates` es necesario bajo
cualquier respuesta a las preguntas 1-5.

---

## 8. Lo implementado (rama `feat/phase7-research-panel`)

Esta sección se añade **después** de construir la fase. El diseño de arriba se
deja tal cual estaba; aquí se anota qué se hizo y en qué difiere, siguiendo la
misma convención que `docs/04-fase6-interfaces.md` §9.

### 8.1 Las cinco decisiones abiertas del §7, resueltas

| §7 | Decisión |
|---|---|
| 1. ¿Profundidad de cualquier expresión? | **Sí, alcance grande.** Ver §8.2. |
| 2. ¿Traza solo de suma? | **No: suma, resta y multiplicación.** Ver §8.3. |
| 3. ¿`compare_backends` siempre los tres? | **Siempre los tres**, no configurable. |
| 4. ¿Pestaña o ventana? | **Pestaña de nivel superior**, dos vistas que el usuario alterna. «Calculadora» sigue siendo la de arranque. La barra lateral de la calculadora no se toca. |
| 5. ¿CSV con plugin de FS o `Blob`? | **Plugin.** Diálogo nativo «Guardar como» de `@tauri-apps/plugin-dialog`; la escritura la hace Rust. |

### 8.2 Profundidad de cualquier expresión: cómo, y qué se descartó

El documento avisaba (§7.1) de que esto era "una obra considerablemente mayor
que exponer `ripple_add_trace`". Lo era, y la forma de hacerla sin que lo fuese
tanto resultó ser una sola idea.

**Descartado — instrumentación paralela por capa.** Escribir `Dz::add_depth`,
`DzFixed::mul_depth`, `dz_sci::sin_depth`… duplicaría la orquestación de cada
operación (los `while`/`if` que deciden qué circuito llamar) en dos copias
destinadas a divergir. Es exactamente el error que `circuits/depth.rs` existe
para no cometer, un nivel más arriba.

**Descartado — hacer genérico todo el árbol de tipos.** `Dz<S>`, `DzFixed<S>`,
`DzRational<S>`, `DzFloat<S>` propagando un parámetro de señal hasta `dz-eval`
cambiaría todas las firmas públicas del proyecto, que era una restricción dura.

**Hecho — el nivel viaja dentro del valor.**

1. Los circuitos de `dz-gates` que aún no eran genéricos sobre `GateNet`
   (`comparator`, `complement`, `parity`, `multiplier`) lo son ahora, con la
   función pública como instanciación `Values`.
2. `GateNet` gana `lift_at(digit, level)` y `level(signal)`, ambos con cuerpo
   por defecto (adición no rompedora).
3. `dz-int` escribe `add`/`sub`/`mul`/`div_rem`/`compare`/`is_even` **una sola
   vez**, genéricos sobre `GateNet`; los métodos públicos conservan su firma y
   eligen `Values` o `Levels` según haya un `DepthScope` activo.
4. `Dz` lleva un `level` privado y re-entra a los circuitos por `lift_at`.
5. `dz-api::execute` abre un `DepthScope` alrededor de la misma pasada en que
   cuenta puertas, y `Response` gana `depth`.

**`dz-num`, `dz-sci` y `dz-eval` no cambiaron de lógica en absoluto.** Componen
operaciones de `Dz` y el nivel viaja dentro del valor. Los únicos cuatro toques
en esas capas son llamadas a `Dz::at_level` en sitios que reordenan dígitos sin
invocar puertas (`DzFixed::trim`, `DzFixed::trunc`, `dz_sci::util::truncate`, el
desempaque de `DzFloat::from_dzfixed`), para que el reordenamiento no reinicie
la cadena.

**Por qué no contamina.** El documento asumía que medir profundidad obliga a una
segunda pasada. No es así: `Levels` invoca al backend exactamente una vez por
primitiva, con los mismos valores y por tanto el mismo flujo de control, que
`Values`. Ejecutar `Levels` *en vez de* `Values` da el mismo `GateCount`.
Fijado por regresión en `dz-int/tests/trace.rs` y
`dz-api/tests/research.rs`, y comprobado contra la salida completa de
`examples/gate_counts.rs`, que es idéntica dígito a dígito a la de antes.

**El trade-off, dicho claro.** Una operación nueva en `dz-int` escrita con el
patrón `*_net` genérico obtiene profundidad gratis; una escrita directamente
contra `Values` será correcta pero invisible para la medición. Y un
reordenamiento nuevo en `dz-num`/`dz-sci` que olvide `at_level` hará que la
profundidad quede *por debajo* de la real, en silencio.

### 8.3 Traza: tres formas, no una

El §2.3 diseñaba solo la suma. La forma final:

- **Suma** — como se diseñó: `ripple_add_net_traced` con un callback, y
  `ripple_add_net` es esa misma función con un callback vacío. Un solo bucle.
- **Resta** — vive en `dz-int` (`subtract_trace`), no en `dz-gates`, porque en
  la capa 0 no existe ningún restador. La traza expone las cinco etapas reales
  (complemento a 11 → `+1` con sus acarreos → suma de verdad con los suyos →
  suma cruda → decodificación si `a < b`) en vez de fingir un préstamo. El §7.2
  temía que enseñar eso confundiera; la respuesta del panel es etiquetar cada
  etapa, no esconderla.
- **Multiplicación** — traza anidada, como se pedía. Un `PartialRow` por dígito
  del multiplicador, con la fila parcial, su desplazamiento, la sub-traza de
  suma de su acumulación y el acumulado resultante. Se recoge con un *sink* de
  métodos vacíos por defecto (`MulTraceSink`) en vez de un `Vec` incondicional,
  para que `multiply` no asigne nada extra.

El §2.1 proponía una tercera instanciación de `GateNet` llamada `Traced`. **No
se construyó**, y el propio documento ya lo anticipaba dos párrafos más abajo:
`GateNet` no tiene noción de posición dentro de una suma multi-dígito, así que
la traza tenía que salir del bucle que sí la tiene. El callback/sink es esa
solución.

### 8.4 CSV

Tres documentos, no dos:

- `gate_count_csv` — `backend,cycle,min,max,lit,thr,total,depth`. El diseño no
  incluía `depth` porque la profundidad estaba sin decidir; se añadió como
  columna del mismo documento en vez de crear un `depth_csv` aparte, porque es
  el mismo dato de la misma corrida (el §4 reservaba `depth_csv` para *otro*
  concepto, una tabla de profundidad por sí sola, que no existe).
- `trace_csv` — un esquema para las tres formas:
  `kind,stage,group,position,a,b,carry_in,sum,carry_out`. `stage` distingue
  `complement`/`add`/`decode`/`accumulate` y `group` dice de qué fila parcial
  viene. Tres archivos incompatibles serían peores para comparar en una hoja de
  cálculo.
- `trace_summary_csv` — el contexto que la tabla de pasos no lleva (operandos,
  complementos, filas parciales, resultado), como documento aparte, siguiendo
  la disciplina que el §4 pedía.

Guardado: el frontend abre el diálogo nativo con `save()`
(`@tauri-apps/plugin-dialog`) y pasa la ruta al comando `export_csv`, que arma
el documento con `dz-api` y lo escribe. **El texto del CSV no llega nunca a
JavaScript**, que es más fuerte que lo que pedía el §4.

### 8.5 Lo que no se hizo

- **Sin traza de división.** No se pidió y la división restauradora no tiene una
  noción de "un acarreo avanzando" que enseñar.
- **Sin traza de `lookahead_add`.** Como decía el §2.1: su red de Kogge-Stone no
  ejecuta posición a posición. Un visor fiel sería otra visualización (etapas de
  prefijo), y sería una fase propia.
- **Sin descomposición del coste de `sin` en su árbol de circuitos** (§1.5). La
  profundidad y el conteo de `sin(1)` sí se miden ahora de punta a punta
  (27.139.541 puertas, 18.397 niveles); lo que sigue sin existir es el desglose
  "cuántos `ripple_add`, cuántos `multiply`".
Los tres puntos de arriba son alcance descartado a propósito, no deuda. El
cuarto punto que había aquí — «la UI no se ha visto correr» — ya no aplica:
se validó en pantalla, y lo observado está en §8.6.

### 8.6 Validación en pantalla

La ventana se abrió y se manejó de verdad (Windows 11, WebView2, `npm run
tauri dev`), no solo se compiló. Lo comprobado:

- **Vista Calculadora.** Arranca por defecto. Teclado dozenal y pad científico
  (generado desde el catálogo: constantes, redondeo, raíces, exponenciales,
  trigonometría) se pintan completos. `1+1` en modo fijo, escala 12, backend
  MVL da `2` con **24.083 puertas y profundidad 441** — los mismos números
  publicados en `CLAUDE.md`, medidos esta vez desde la GUI. `↋4;6*3` en modo
  racional da `583/2` (= 819/2, correcto). Historial, memoria y los tres
  selectores de sesión responden.
- **Vista Investigación.** El desglose y la profundidad se repintan con cada
  cálculo. «Comparar backends» ejecuta los tres y los muestra con conteo y
  profundidad, con la etiqueta «oráculo» sobre TABLE.
- **Visor paso a paso.** Las tres formas: `↋↋↋ + 1 = 1000` (3 pasos, el
  acarreo recorriendo las tres posiciones); `5 − ↊ = −5` con sus cinco etapas
  (complemento a 11 → +1 → suma → suma cruda → decodificación); `25 × 34 =
  808` con una fila parcial por dígito del multiplicador. La reproducción
  (anterior/siguiente/todo) resalta el paso actual y atenúa los pendientes.
  Los alias ASCII (`X` por ↊) se aceptan también en los campos del visor.
- **Export CSV.** El diálogo nativo «Guardar CSV» abre con su filtro `*.csv`,
  y Rust escribe el archivo en la ruta elegida. Verificados los **tres**
  documentos en disco: `gate_count_csv` (una fila por backend con `depth`),
  `trace_csv` (`sub,complement/add/decode`) y `trace_summary_csv` (los cuatro
  campos por fila parcial de la multiplicación).
- **Glifos ↊/↋.** En esta máquina la medición de `glyphs.ts` **sí** encuentra
  glifo: el pad, el visor y los resultados muestran los caracteres reales
  U+218A/U+218B, no el respaldo `X`/`E`. El camino de respaldo, por tanto, no
  se ejerció aquí; lo que sí se corrigió son dos sitios donde ese respaldo
  habría quedado a medias (ver abajo).

**Defectos encontrados y corregidos** (todos de frontend; ninguno tocó
semántica dozenal ni el formato del CSV, que sigue siendo autoridad de Rust):

1. `trace_summary_csv` **no era alcanzable desde la GUI**. El comando
   `export_csv` ya aceptaba `"trace-summary"` desde el principio, pero no había
   ningún botón que lo pidiera: el tercer documento del §8.4 existía en Rust y
   era inaccesible. Ahora el bloque del visor tiene dos botones, «Exportar
   pasos CSV» y «Exportar resumen CSV», cada uno con su propio diálogo.
2. **La barra de pestañas se iba con el scroll.** Al tamaño de ventana por
   defecto la calculadora es más alta que el viewport, así que bajar a los
   botones científicos se llevaba por delante el único acceso a la vista
   Investigación. Ahora es `position: sticky` sobre un `body` en columna.
3. **Jerarquía de títulos invertida** en la vista Investigación: los `h2` de
   cada bloque salían con el tamaño por defecto del navegador (1,5em) y se
   veían más grandes que el `h1` del panel. Ahora siguen la misma convención
   de versalitas que la barra lateral de la calculadora.
4. **Los campos A y B del visor salían con el fondo claro del sistema** sobre
   el tema oscuro: la regla global de formularios cubría `select` y
   `input[type=number]`, no los de texto.
5. **Los botones deshabilitados parecían pulsables** (exportar y reproducción,
   antes de que haya nada que exportar). Ahora se atenúan.
6. Dos sitios donde el respaldo ASCII de `glyphs.ts` quedaba a medias: el
   teclado insertaba `↊` aunque la tecla mostrara `X`, y el operando de
   ejemplo del visor venía escrito con glifos directamente en el HTML. Ambos
   pasan ya por `display()`. **No verificado en pantalla**, porque esta máquina
   tiene los glifos y el respaldo no llega a activarse.
