# 06 — Fase 9: paridad CLI y skill para LLM

Estado de partida: rama `feat/phase7-research-panel`, Fase 7 cerrada y validada
en pantalla (`docs/05-fase7-panel-investigacion.md` §8.6). Todo lo que hace el
panel de investigación vive ya en `dz-api` (capa 5), que es **sin estado** y no
sabe nada de Tauri. La GUI llega a esas funciones; el terminal no.

Este documento diseña la Fase 9: llevar al terminal **todo** lo que hoy solo
existe en la GUI, y publicar un *skill* en Markdown para que un LLM (Claude Code
u otro agente) pueda manejar la calculadora sin abrir una ventana.

---

## 1. Inventario verificado: qué expone cada interfaz hoy

`dz-api` es la única fuente; las dos interfaces son clientes suyos. La columna
«dz-api» dice si la capacidad ya está construida en la capa 5.

| Capacidad | dz-api | GUI (Tauri) | dz-cli | Hueco |
|---|---|---|---|---|
| Evaluar operación / expresión | `execute` (`lib.rs:244`) | `calc` (`commands.rs:127`) | one-shot + REPL | — |
| Conteo de puertas | `Response.gate_count` | `paintGates` | `--gates`, `:gates` | — |
| Profundidad | `Response.depth` | fila en la tabla | en la línea `gates:` | — |
| Catálogo de funciones | `functions()` (`lib.rs:236`) | `list_functions` (`commands.rs:225`) | `--functions`, `:funcs` | — |
| Backend / modo / escala | `Request` | `set_backend`/`set_mode`/`set_scale` | flags + `:backend`/`:mode`/`:scale` | — |
| **Traza paso a paso** (suma/resta/mult.) | `research::trace` (`research.rs:244`) | `trace_operation` (`research.rs:167`) | **no existe** | **1** |
| **Comparar los tres backends** | `research::compare_backends` (`research.rs:340`) | `compare_backends` (`research.rs:190`) | **no existe** | **2** |
| **Export CSV** (3 documentos) | `gate_count_csv` / `trace_csv` / `trace_summary_csv` (`research.rs:380-439`) | `export_csv` + diálogo nativo | **no existe** | **3** |
| **Memoria** (M+/MR/MC) | — (vive en la interfaz) | `memory_store` / `memory_clear` (`commands.rs:209`, `:216`) | **no existe** | **4** |
| **Historial** | — (vive en la interfaz) | `SessionDto.history` / `clear_history` (`commands.rs:202`) | **no existe** | **5** |
| **Volcado de sesión** | — | `get_session` (`commands.rs:196`) | **no existe** | **6** |
| **JSON de salida** | `to_json_line` (`lib.rs:400`) | serde en cada DTO | `--json`, **solo de `Response`** | **7** |

Los huecos 1-3 son capacidades de `dz-api` que el CLI simplemente nunca llamó:
fontanería, cero lógica dozenal nueva. Los huecos 4-6 son estado de sesión, que
por diseño **no** vive en `dz-api` (`CLAUDE.md`: «Sin estado: la sesión vive en
cada interfaz») — el REPL tendrá que llevar el suyo, igual que `AppState` lleva
el de la GUI. El hueco 7 es lo que convierte todo esto en algo que un LLM puede
consumir.

**Regla que no cambia:** `dz-cli` sigue sin contener lógica dozenal. Construye
peticiones de `dz-api` e imprime lo que vuelve. Ningún `Digit`, ningún `.tag()`,
ningún operador nativo sobre un valor dozenal. `pwsh tools/lint-purity.ps1` debe
seguir en verde.

---

## 2. Diseño: los tres subcomandos nuevos

Se añaden como operaciones de primer nivel en `build_operation`
(`crates/dz-cli/src/main.rs:207`), pero **no** son `Operation` de `dz-api` — son
otro despacho, porque `research::trace` y `compare_backends` tienen su propia
petición y su propia respuesta. Eso quiere decir un `enum Command` nuevo en el
CLI por encima del actual `Operation`:

```rust
enum Command {
    /// The existing path: build a dz_api::Operation and call `execute`.
    Calc(Operation),
    /// dz_api::research::trace
    Trace { kind: TraceKind, a: String, b: String },
    /// dz_api::research::compare_backends
    Compare(Operation),
}
```

`split_arguments` no cambia de forma; `run_command` gana dos ramas.

### 2.1 `dz-cli trace <add|sub|mul> <a> <b>`

Mapea 1:1 a `dz_api::research::trace(&TraceRequest { backend, kind, a, b })`.
`TraceKind::parse` ya existe (`research.rs:89`), así que el CLI no inventa
nombres: reusa esa función, igual que hace con `BackendKind::parse`.

Salida por defecto, texto plano tabulado, una fila por paso, en el mismo orden
que el visor de la GUI (de la menos significativa a la más significativa):

```
$ dz-cli trace add EEE 1 --gates
trace: add  EEE + 1 = 1000
  pos  a  b  cin  sum  cout
    0  E  1    0    0     1
    1  E  0    1    0     1
    2  E  0    1    0     1
result: 1000
gates: cycle=... total=... depth=...
```

Para `sub`, las cinco etapas reales que ya modela `SubTrace` (complemento a 11,
`+1`, suma, suma cruda, decodificación), **etiquetadas**, no escondidas — misma
decisión que tomó la Fase 7 (§8.3 de `docs/05`). Para `mul`, una sub-tabla por
`MulRow` (fila parcial, desplazamiento, sub-traza de acumulación, acumulado).

La honestidad que ya lleva la GUI viaja al terminal: si el usuario pide
`trace div`, el error dice que no existe traza de división y por qué (la
división restauradora no tiene un acarreo que avance), no un «operación
desconocida» genérico.

### 2.2 `dz-cli compare <operación> <args...>`

Toma exactamente los mismos argumentos que la operación normal, así que
`build_operation` se reutiliza tal cual para el resto de la línea:

```
$ dz-cli compare mul 25 34
result: 808
  backend  cycle  min  max  lit  thr  total  depth
  mvl      ...
  crt      ...
  table    ...          (oráculo, no es un backend de producción)
```

La etiqueta de «oráculo» sobre `table` es obligatoria, por la misma razón que la
lleva el panel (`docs/05` §5): sin ella la tabla invita a leer «Table gana» como
un resultado de investigación, y no lo es.

### 2.3 `--csv [ruta]` sobre `trace` y `compare`

Sin `--csv`, salida legible. Con `--csv` y sin ruta, el documento va a *stdout*
(que es lo que un pipe o un LLM quiere); con ruta, se escribe al fichero. El
documento lo arma **siempre** `dz-api` (`gate_count_csv`, `trace_csv`,
`trace_summary_csv`) — el CLI no formatea CSV, igual que el frontend tampoco lo
hace. Ésa es la regla que ya fijó la Fase 7: Rust es la única autoridad del
formato.

`trace --csv` emite `trace_csv`; para el documento de contexto se usa
`--csv-summary`, que emite `trace_summary_csv`. Dos documentos distintos, no un
esquema sobrecargado — la misma disciplina del §8.4 de `docs/05`.

---

## 3. Diseño: sesión en el REPL (huecos 4-6)

La GUI tiene `AppState` (`app/src-tauri/src/state.rs`) con backend, modo,
escala, historial y memoria. El REPL tiene hoy solo los tres primeros, en
variables locales de su bucle. Se completa **en el CLI**, sin tocar `dz-api`:

| Comando REPL | Equivalente GUI | Qué hace |
|---|---|---|
| `:mem` | recuadro de memoria | imprime el valor guardado |
| `:mem+` | `memory_store` | guarda el último resultado |
| `:memc` | `memory_clear` | borra la memoria |
| `:history` | lista del historial | imprime las entradas con índice |
| `:clear` | `clear_history` | vacía el historial |
| `:session` | `get_session` | vuelca backend, modo, escala, memoria, tamaño del historial |

El valor de memoria se guarda como **el string ya formateado** que devolvió
`dz-api`, no como un número reconstruido — el CLI no puede recomponer un valor
dozenal, exactamente igual que el frontend. Recuperarlo es reinyectar ese texto
como operando, que vuelve a pasar por el lexer de `dz-parse`.

El historial guarda `(entrada, salida)` en texto. Es memoria del proceso; no se
persiste a disco. Que el REPL muera y se lleve el historial es también el
comportamiento de la GUI (`AppState` es de proceso), así que no se introduce una
divergencia nueva.

---

## 4. Diseño: `--json` para todo (hueco 7, el que habilita al LLM)

Hoy `--json` solo sabe serializar `Response` (`to_json_line`, `lib.rs:400`) y
`ApiError` (`error_json_line`, `lib.rs:420`). Se extiende a las salidas nuevas
con funciones hermanas en `dz-api`, **no** en el CLI:

```rust
// crates/dz-api/src/research.rs
pub fn trace_json_line(trace: &Trace) -> String;
pub fn comparison_json_line(comparison: &BackendComparison) -> String;
// the session dump stays in dz-cli: the session lives in the interface.
```

Motivo de ponerlas en `dz-api` y no en el CLI: la misma razón por la que el CSV
lo arma Rust. Una sola autoridad de formato. Si mañana la GUI quiere exportar
JSON, no hay un segundo serializador que diverja.

**`serde` no entra en `dz-api`.** Hoy la única frontera del repo con `serde` es
`app/src-tauri` (`CLAUDE.md`), y `to_json_line` ya escribe JSON a mano. Las
funciones nuevas siguen ese patrón — escapado explícito de comillas, nada más.
Es más código que un `derive`, pero mantiene la dependencia donde estaba y el
JSON de estas estructuras es plano.

Todos los campos de esos JSON son texto ya formateado o contadores `usize`;
ninguno es un valor dozenal recomponible. Un LLM que lea la salida puede razonar
sobre ella pero no puede hacer aritmética dozenal con ella, que es justo la
propiedad que el proyecto quiere.

### 4.1 Códigos de salida

Se conservan los tres de siempre (0 ok, 1 error de cálculo, 2 error de uso) y se
extienden a los subcomandos nuevos sin añadir códigos: un `trace div` es error de
uso (2); un `trace` con un operando no entero es error de cálculo (1).

---

## 5. El skill para LLM

Se crea `.claude/skills/dz-cli/SKILL.md` (hoy el repo no tiene directorio
`.claude/`). Es un skill de repo: cualquier sesión de Claude Code abierta en
este proyecto lo ve.

Frontmatter:

```markdown
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
```

Cuerpo, en inglés (convención del repo para código y para lo que consume una
herramienta; los documentos de investigación siguen en español):

1. **Cómo invocar**: `cargo` no está en el PATH — `$env:PATH +=
   ";$env:USERPROFILE\.cargo\bin"`, y `cargo run -p dz-cli --release -- <args>`.
   El `--release` importa: en debug una trascendente tarda minutos.
2. **El alfabeto**: dígitos `0-9` más `X` (diez) y `E` (once); `;` es el punto
   Humphrey; `--unicode` para ↊/↋. La trampa que un LLM va a pisar: `e` **no**
   es el número de Euler, se escribe `euler`; `x` y `t` tampoco son nombres.
   Dentro de una llamada la coma separa argumentos, así que ahí `10000`, nunca
   `1,0000`.
3. **Tabla de recetas**: una fila por pregunta típica → la línea de comando
   exacta. «¿Cuánto cuesta `sin(1)`?» → `eval "sin(1)" --mode fixed --scale 12
   --gates`. «¿Cómo se propaga el acarreo?» → `trace add EEE 1`. «¿Qué backend
   es más barato?» → `compare mul 25 34`.
4. **Siempre `--json` cuando la salida la va a leer el agente**, texto plano
   cuando la va a leer una persona.
5. **Cómo leer los números**: puertas y profundidad son cosas distintas y se
   miden en la misma pasada; `pi` marca 0 puertas a propósito (constantes
   memoizadas); `table` no es un backend de producción.
6. **Lo que el CLI no hace**, para que el agente no lo intente: no hay traza de
   división ni de `lookahead_add`; `^` solo acepta exponentes enteros no
   negativos (`exp(y * ln(x))` para lo demás); `1/5` no termina en base 12 y se
   avisa en vez de truncar en silencio.
7. **La regla de pureza del repo**, en una línea, con el puntero a `CLAUDE.md`:
   si el agente va a *editar* código, ningún operador aritmético nativo toca un
   valor dozenal.

El skill describe la herramienta; **no** duplica la arquitectura del repo, que
ya está en `CLAUDE.md`. Un puntero, no una copia — dos copias divergen.

---

## 6. Orden de implementación

1. **`dz-api`**: `trace_json_line`, `comparison_json_line` (§4), con tests que
   fijen el formato. No toca nada existente; es sumar dos funciones.
2. **`dz-cli`, subcomandos**: `enum Command`, `trace`, `compare`, `--csv`,
   `--csv-summary` (§2). Es el grueso.
3. **`dz-cli`, sesión**: memoria, historial, `:session` (§3).
4. **`dz-cli`, `--json` en las rutas nuevas** y actualización de `print_usage` y
   de `:help` — los dos textos de ayuda tienen que cubrir todo lo nuevo, o el
   comando existe y nadie lo encuentra.
5. **Skill** `.claude/skills/dz-cli/SKILL.md` (§5), escrito **contra el binario
   ya construido**: cada línea de comando de la tabla de recetas se ejecuta y se
   pega su salida real. Un skill con ejemplos inventados es peor que no tenerlo.
6. **Tests**: `crates/dz-cli/tests/` (hoy no existe) con los subcomandos nuevos
   ejercitados de punta a punta, más una prueba de que el CSV que emite el CLI es
   byte a byte el que emite `dz-api` (la GUI ya tiene esa garantía por
   construcción; el CLI debe tenerla por test).
7. **Docs**: §8 de este documento con lo implementado y lo que se desvió;
   `CLAUDE.md` en la sección de `dz-cli` y en Comandos.

---

## 7. Riesgos

- **Divergencia GUI/CLI.** Al terminar, las dos interfaces deben ofrecer el mismo
  conjunto de capacidades. Lo que evita que se separen otra vez es que ambas
  llamen a `dz-api` y que ninguna formatee nada por su cuenta: si el CLI formatea
  su propio CSV o su propio JSON, la divergencia vuelve.
- **Superficie de la línea de comandos.** `trace` y `compare` como subcomandos de
  primer nivel conviven con «cualquier función registrada es un subcomando»
  (`build_operation`, rama `other`). Si alguien registrase una función llamada
  `trace` o `compare` en `dz-eval`, quedaría inalcanzable. Es improbable, pero el
  despacho debe reservar esos dos nombres explícitamente y decirlo en un
  comentario, no dejarlo al azar del orden del `match`.
- **El skill envejece.** Los ejemplos se copian de una corrida real, así que un
  cambio futuro en el formato de salida los deja mintiendo. Mitigación barata:
  los tests del paso 6 cubren esos mismos comandos, así que un cambio de formato
  rompe un test antes de que el skill mienta.

---

## 8. Lo implementado (rama `feat/phase7-research-panel`)

El diseño de arriba se deja tal cual estaba; aquí se anota qué se construyó, en
qué se desvió y qué no se hizo, con la misma convención honesta de
`docs/04-fase6-interfaces.md` §9 y `docs/05-fase7-panel-investigacion.md` §8.

### 8.1 Los siete huecos, cerrados

| Hueco | Dónde quedó |
|---|---|
| 1. Traza paso a paso | `dz-cli trace add\|sub\|mul a b` — `main.rs:379` (`build_command`), `main.rs:427` (`render_trace`), `main.rs:550` (`run_trace`) |
| 2. Comparar los tres backends | `dz-cli compare <op> <args>` — `main.rs:492` (`render_comparison`), `main.rs:587` (`run_compare`) |
| 3. Export CSV | `--csv[=ruta]` y `--csv-summary[=ruta]` — `main.rs:105` (`CsvDocument`), `main.rs:531` (`emit_csv`) |
| 4. Memoria | `:mem` / `:mem+` / `:memc` en el REPL — `main.rs:709` (`Session`), bucle del REPL |
| 5. Historial | `:history` / `:clear`, mismo sitio |
| 6. Volcado de sesión | `:session`, mismo sitio |
| 7. `--json` de las salidas nuevas | `dz_api::research::trace_json_line` (`research.rs:520`) y `comparison_json_line` (`research.rs:579`) |

`dz-cli` sigue sin lógica dozenal: construye peticiones de `dz-api`, imprime lo
que vuelve y, en el caso del CSV, escribe los bytes exactos que `dz-api` armó.
`pwsh tools/lint-purity.ps1` en verde.

### 8.2 Desviaciones del diseño

**`--csv <ruta>` se escribe `--csv=<ruta>`.** El §2.3 pedía un valor opcional
separado por espacio. No se puede hacer sin ambigüedad: los operandos también
son posicionales, así que en `trace add EEE 1 --csv 5` no hay forma de saber si
`5` es una ruta o un operando extra. La forma con `=` es inequívoca y el parser
la resuelve en `split_arguments` (`main.rs:158-180`); sin `=`, el documento va a
stdout, que es el caso que un pipe o un agente usa.

**`trace --gates` necesitó una función nueva en `dz-api`.** El §2.1 enseñaba una
línea `gates:` bajo una traza, pero `research::trace` no devuelve coste. Se
añadió `trace_measured` (`crates/dz-api/src/research.rs:263`), que corre la
misma traza con un backend fresco y devuelve `(Trace, GateCount)`. Es una
adición: `trace` no cambió y la GUI no se tocó.

**Esa línea dice `depth=n/a`, no un número.** Los circuitos trazados se
instancian con `Values` (`dz_gates::circuits::trace`), que es justo la
instanciación que **no** lleva niveles de llegada. Reportar una profundidad ahí
sería inventarla. La profundidad de la misma aritmética se pide con la llamada
normal (`dz-cli sum ... --gates`), donde `execute` mide ambas cosas en la misma
pasada.

**`--csv-summary` no aplica a `compare`.** Son dos documentos de traza; una
comparación solo tiene el documento de puertas. Pedirlo es error de uso (2).

**`crates/dz-cli/tests/` sí existía** (el §6 lo daba por inexistente): ya estaba
`oneshot.rs` de la Fase 6. Los tests nuevos van en `tests/research.rs` y
`oneshot.rs` no se tocó.

**El JSON y el CSV salen con los glifos canónicos `↊`/`↋`, no con los alias
ASCII**, aunque el texto plano por defecto sí usa los alias. No es una decisión
nueva: es lo que `to_json_line` ya hacía desde la Fase 6, y aplicar `to_ascii`
encima sería el CLI reformateando un documento de `dz-api`, exactamente lo que
el §7 prohíbe. El texto plano de `trace`/`compare` sí respeta `--unicode`,
porque ese sí lo compone el CLI.

### 8.3 La reserva de nombres del §7

`build_operation` (`crates/dz-cli/src/main.rs:321`) tiene una rama explícita
`"trace" | "compare"` que devuelve un error de uso diciendo que son comandos de
primer nivel, **antes** de la rama `other` que convierte cualquier función
registrada en subcomando. Si mañana alguien registra una función llamada
`trace` en `dz-eval`, el choque se reporta en vez de decidirse por el orden del
`match`. Fijado por `tests/research.rs::trace_and_compare_are_reserved_names_not_operations`.

### 8.4 De limitación a capacidad: la columna `recon` y el veredicto

Este apartado documentaba una limitación: la tabla de `compare` daba **los
mismos cinco contadores para los tres backends** y no había forma, desde
ninguna interfaz, de ver por qué eso no significa «los tres cuestan lo mismo».
La causa nunca fue un bug — `GateCount` cuenta invocaciones de las primitivas de
`DozGates`, y los tres backends implementan las mismas primitivas para el mismo
circuito, así que el empate es una propiedad del diseño. Lo que ese contador no
veía era el sobrecoste real de CRT: las reconstrucciones de dígito que necesita
para toda comparación de orden (`docs/02-crt-vs-mvl.md`).

Ya no es una limitación. La instrumentación existía (`CrtGates` contaba
`recon_ops` desde la Fase 2) pero no cruzaba las capas; ahora sí:

1. **`DozGates::reconstruction_ops()`** (`crates/dz-gates/src/traits.rs`), con
   cuerpo por defecto `0` para que sea una adición no rompedora — el mismo
   patrón que la Fase 7 usó con `GateNet::lift_at`. `CrtGates` lo sobrescribe;
   MVL y `TableGates` se quedan con el 0, que **es la verdad**, no una medición
   ausente: su representación interna ya es el tag. `DynGates` lo reenvía, como
   cualquier otro contador — tomar ahí el valor por defecto habría reportado 0
   para un `CrtGates` prestado.
2. **`Response.reconstruction_ops`** y **`BackendCost.reconstruction_ops`**
   (`crates/dz-api`). Campo hermano de `depth`, **nunca** un sexto contador
   dentro de `GateCount`: esa estructura se mantiene agnóstica del backend al
   nivel de las cinco primitivas a propósito, y es precisamente esa agnosticidad
   la que hace necesario el número de al lado.
3. **`dz_api::verdict(&[BackendCost]) -> Option<Verdict>`**, que viaja dentro de
   `BackendComparison`.

#### Cómo se calcula el veredicto

- **Ranking solo entre backends de producción.** `table` se lista pero no se
  clasifica: es el oráculo por tabla de verdad (`docs/05` §5, `docs/01`), indexa
  memoria en vez de componer puertas, y «más barato que el oráculo» no es una
  respuesta de investigación. La salida lo dice en voz alta.
- **Orden lexicográfico por `(total de primitivas, profundidad,
  reconstrucciones)`.** Sin fórmula ponderada: no hay tipo de cambio medido
  entre una invocación y una reconstrucción, y sumarlas con un coeficiente
  inventaría precisión que el experimento no tiene.
- **El modelo de coste va en la salida**, escrito: «cost model: primitive
  invocations + internal->tag reconstructions, ranked lexicographically by
  (total, depth, reconstructions)». Si el lector no ve el modelo, el veredicto
  es otro oráculo.
- **El empate se nombra.** Cuando dos backends empatan en las cinco primitivas y
  en profundidad — que con estos circuitos es siempre — el veredicto lo declara
  y señala que el desempate viene de las reconstrucciones. Un «mvl gana» a secas
  ocultaría justo lo que hace interesante el resultado.

#### Paridad de las dos interfaces (§7)

- **CLI**: columna `recon` en la tabla de `compare`, línea de veredicto plegada a
  76 columnas, `reconstruction_ops` y `verdict` en `--json`, y las columnas
  `reconstruction_ops,oracle` en `gate_count_csv`. Además `--gates` añade
  `recon=` a la línea de conteo de una llamada cualquiera, así que
  `--backend crt` es honesto sin pedir una comparación.
- **GUI**: fila «reconstrucciones» en la tabla de tres backends y el mismo
  veredicto bajo ella (`app/src/research.ts`, `app/src-tauri/src/research.rs`).
  El frontend recibe la frase **ya redactada por Rust** y la inserta como texto;
  no compone nada en JavaScript, igual que con el CSV.

Salida real, con la que `docs/02-crt-vs-mvl.md` cierra su sección nueva:
`compare mul 25 34` da 118.630 reconstrucciones en CRT y 0 en MVL, con
83.927 invocaciones y profundidad 441 en los tres; `compare sum 1 1` da 33.244
frente a 0, con 23.780 y 441. El veredicto es `mvl` en ambos, consistente con
la conclusión de `docs/02`.

Control de que nada medido se movió: `cargo run -p dz-gates --example
gate_counts --release` da salida **idéntica** (`diff` limpio) y `sin(1)` sigue
en 27.139.541 puertas y profundidad 18.397.

### 8.5 El skill

`.claude/skills/dz-cli/SKILL.md`, con el frontmatter del §5 palabra por palabra
y los siete puntos del cuerpo, en inglés. **Cada línea de comando de la tabla de
recetas se ejecutó contra `target/release/dz-cli.exe` y la salida pegada es la
real**, incluida la de `sin(1)` (27.139.541 puertas, profundidad 18.397 — el
mismo número publicado en `CLAUDE.md`, que es el control de que la Fase 9 no
movió nada medido). El riesgo de que el skill envejezca lo cubren los tests de
`tests/research.rs`, que ejercitan esos mismos comandos.

### 8.6 Lo que no se hizo

- **`trace` y `compare` no existen dentro del REPL.** El §3 solo pedía sesión
  (memoria, historial, volcado) para el modo interactivo, y las dos capacidades
  de investigación son de una invocación por naturaleza: su salida es un
  documento, no una línea de resultado. Se documenta el puntero en `:help`.
- **La memoria no se reinyecta sola.** `:mem` imprime el valor guardado; usarlo
  es copiarlo a la siguiente expresión, que vuelve a pasar por el lexer de
  `dz-parse`. No se añadió un identificador `M` porque una letra suelta choca
  con el alfabeto dozenal, que es la misma razón por la que `e`, `x` y `t` no
  pueden ser nombres.
- **Nada se persiste a disco.** Historial y memoria mueren con el proceso, igual
  que el `AppState` de la GUI. No se introduce una divergencia nueva.
- **No se tocó la GUI.** La paridad se cierra por el lado del terminal; los
  comandos Tauri, sus DTO y el frontend quedan idénticos.
- **Sin traza de división ni de `lookahead_add`**, por las razones de la Fase 7.
  `trace div` responde con esa explicación, no con «operación desconocida».
