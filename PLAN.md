# Plan: Calculadora Dozenal Nativa (Base 12 desde puertas lógicas)

**Stack:** Rust (núcleo) + Tauri (UI escritorio Windows)
**Lógica base:** doble backend intercambiable — MVL Post-álgebra 12-valuada + descomposición CRT 4×3, con oráculo por tablas para tests diferenciales.

---

## 0. Principio rector (y su honestidad epistemológica)

La máquina física es binaria. No se puede evitar. Lo que **sí** se puede garantizar, y es el objeto de la investigación, es:

> Ninguna operación aritmética del lenguaje anfitrión se usa jamás sobre valores dozenales. Un dígito es un **símbolo opaco**, no un número. Todas las operaciones se definen exclusivamente como composición de puertas lógicas 12-valuadas.

Concretamente:
- `Digit` es un newtype `struct Digit(u8)` que **no implementa** `Add`, `Sub`, `Mul`, `Ord` aritmético ni `From<u32>`.
- El `u8` interno es una etiqueta de símbolo (índice en el alfabeto `0..↋`), no una cantidad.
- Las únicas funciones que tocan ese `u8` son las **primitivas de puerta** en `dz-gates`, definidas por tabla de verdad.
- Un lint de CI prohíbe `+ - * / %` sobre tipos numéricos dentro de los crates del núcleo. Hacer trampa debe ser *imposible*, no solo desaconsejado.

Esto es lo que separa este proyecto de "calculadora que convierte a base 10 y muestra base 12".

---

## 1. Arquitectura por capas

```
Capa 6   Interfaces: REPL, CLI de una invocación, GUI Tauri
Capa 5   dz-api — entrada única, sin estado, la misma para las tres interfaces
Capa 4   Motor de evaluación (AST, precedencia, registro de funciones)
Capa 3   Lexer/Parser dozenal (nunca pasa por i64/f64)
Capa 2.5 dz-sci — funciones científicas (series sobre DzFixed)
Capa 2   Números: punto fijo, racionales exactos, float dozenal (DFP)
Capa 1   Enteros dozenales (Dz): suma, resta, mult, div, comparación
Capa 0   Puertas lógicas 12-valuadas (backends intercambiables)
```

(Las capas 5, 6 y 2.5 se añadieron en la Fase 6; el plan original tenía la UI
hablando directamente con la capa 4.)

Regla: cada capa solo puede llamar a la inmediatamente inferior. Verificado por `cargo-deny` / grafo de dependencias del workspace.

---

## 2. Capa 0 — Puertas dozenales

### Trait común

```rust
pub trait DozGates {
    fn cycle(x: Digit) -> Digit;              // sucesor mod 12
    fn min(x: Digit, y: Digit) -> Digit;
    fn max(x: Digit, y: Digit) -> Digit;
    fn lit(j: u8, x: Digit) -> Digit;         // ↋ si x == j, si no 0
    fn thr(j: u8, x: Digit) -> Digit;         // ↋ si x >= j, si no 0
    fn cost() -> GateCount;                   // instrumentación
}
```

Estas cinco primitivas son **funcionalmente completas** para lógica 12-valuada (Post). Cualquier `f: Z12^n → Z12` se expresa en forma canónica MAX-de-MIN-de-literales.

### Backend A — MVL directo

Cada primitiva es una tabla de verdad 12 (o 144) entradas. Estructural, contable, es "la puerta" en sentido literal.

### Backend B — CRT 4×3

`Z12 ≅ Z4 × Z3` (anillos, porque gcd(4,3)=1). Un dígito = par `(q, t)` con `q ∈ Z4` (2 bits), `t ∈ Z3` (1 trit).
- Suma y multiplicación mod 12 se descomponen componente a componente. Muy barato.
- **Problema de diseño real:** la CRT destruye el orden. Detectar acarreo requiere `x + y >= 12`, que no es expresable componente a componente. Se necesita un circuito de reconstrucción de orden (decodificador CRT→umbral). Cuantificar ese coste es un resultado de investigación por sí mismo.

### Backend C — Oráculo por tabla

Tabla completa 144 entradas para suma y multiplicación. **No es un backend de producción** — existe solo como referencia para tests diferenciales.

### Sumador dozenal (construido, no tabulado)

Semi-sumador de un dígito:
- **Suma:** decodificador 1-de-12 sobre `y` (con `lit`), 12 ramas de `cycle^j` precompuesto, multiplexadas con `MAX`.
  `s = MAX_{j=0..↋} MIN( lit(j, y), cycle^j(x) )`
- **Acarreo:** `c = MAX_{j=0..↋} MIN( lit(j, y), thr(12−j, x) )`

Sumador completo = dos semi-sumadores + `MAX` de acarreos (el acarreo nunca excede 1).

Multi-dígito:
1. **Ripple-carry** primero (simple, correcto, lento: O(n)).
2. **Carry-lookahead dozenal** después: generate `G = thr(12−y, x)`, propagate `P = lit(↋, s)` (propaga si la suma del dígito es exactamente ↋). Comparar conteo de puertas y profundidad contra ripple → primer gráfico del paper.

### Multiplicador
- v1: sumas desplazadas (shift-add en base 12) — solo sumador + desplazamiento posicional.
- v2: tabla dígito×dígito sintetizada en SOP MVL.
- Comparar coste. Segundo resultado medible.

---

## 3. Capa 1 — Enteros dozenales

- `Dz { digits: Vec<Digit>, /* little-endian */ }` de precisión arbitraria.
- **Representación de signo:** complemento a 12 (radix complement). La resta se vuelve suma, sin caminos de código separados. Alternativa documentada: complemento a 11 (diminished radix) con acarreo circular.
- **Comparación:** vía `thr`, nunca vía `Ord` de Rust.
- **División:** restauradora en base 12; luego no-restauradora; luego estimación de cociente estilo Knuth algoritmo D adaptado a radix 12.
- Normalización de ceros a la izquierda, longitud canónica.

---

## 4. Capa 2 — Fracciones y punto flotante

Aquí es donde la base 12 justifica su existencia:

- **Punto fijo dozenal:** N dígitos fraccionarios exactos. `1/3 = 0;4`, `1/4 = 0;3`, `1/6 = 0;2` — todos terminan. Contraste directo con base 10.
- **Racionales exactos:** `num/den` como par de `Dz`, con MCD por Euclides usando solo la división de Capa 1. Modo por defecto de la calculadora → resultados exactos.
- **Float dozenal (DFP):** signo + exponente dozenal sesgado + mantisa dozenal. Redondeo half-even en base 12. Definir el formato es una decisión de diseño propia (no existe estándar) → documentar como contribución.
- Tabla comparativa de periodicidad: qué fracciones terminan en base 10 vs 12 vs 16.

---

## 5. Capa 3 — Lexer / Parser dozenal

- **Alfabeto:** `0 1 2 3 4 5 6 7 8 9 ↊ ↋` (U+218A dek, U+218B el). Alias configurable: `X/E` (Dozenal Society GB), `A/B` (informático), `T/E`.
- **Separador fraccionario:** punto Humphrey `;` (`1;6` = 1.5 decimal). Configurable a `.`.
- **Agrupación:** por cuatro dígitos, `1,0000` (una gruesa-gruesa).
- Crítico: el lexer construye un `Dz` empujando `Digit` uno a uno. **Nunca** `str::parse::<i64>()`. La salida se formatea mapeando `Digit → char`. Ningún número cruza la frontera hacia binario/decimal nativo.

---

## 6. Capa 4 — Motor de evaluación

- AST + parser de precedencia (Pratt).
- Trait `DozFunction` (`arity`, `category`, `apply_fixed`, `apply_rational`) + `FnRegistry` → registro de operaciones. **Este es el punto de extensión.** Añadir una función nueva = implementar el trait y registrarla, cero cambios en el core, y aparece sola en el REPL, en el CLI y en el teclado de la GUI. (Sustituyó a `DozOp`/`OpRegistry`, que no podía ejecutarse porque no existía `Expr::Call`; ver `docs/04-fase6-interfaces.md` §9.2.2.)
- Operaciones v1: `+ − × ÷ mod ^ neg` y paréntesis.
- Manejo de errores como valores (división por cero, overflow de precisión).

---

## 7. Capa 6 — UI Tauri

- Teclado de 12 dígitos + operadores, más un pad científico generado desde el catálogo del registro de funciones. Sobre los glifos ↊↋: el frontend mide si la fuente los tiene y cae a los alias `X`/`E` si no (queda pendiente empotrar una fuente).
- Display dozenal, historial, memoria. **Hecho** — ver `app/` y `docs/04-fase6-interfaces.md` §9.
- **Panel de investigación** (lo que distingue esta app):
  - Selector de backend en caliente (MVL / CRT / tabla). **Hecho.**
  - Contador de puertas de la última operación. **Hecho** (la profundidad todavía no; se mide en pasada aparte, ver `circuits/depth.rs`).
  - Visor paso a paso del sumador ripple: ver el acarreo propagarse dígito a dígito.
  - Exportar traza a CSV para el análisis.

---

## 8. Testing y verificación

1. **Diferencial:** los 3 backends deben coincidir en las 144 combinaciones de suma y multiplicación de un dígito. Exhaustivo, no muestreado.
2. **Property-based (`proptest`):** conmutatividad, asociatividad, distributividad, `(a+b)−b == a`, `(a×b)÷b == a`, round-trip `parse(format(x)) == x`.
3. **Oráculo externo:** contra `num-bigint` **solo en los tests**, nunca en el core. El test convierte a base 12 desde fuera para comprobar; el código de producción no lo ve.
4. **Lint de pureza:** script en CI que falla si aparece aritmética nativa en `crates/dz-*`. Es el guardián del principio de la Sección 0.
5. **Fuzzing** del parser.

---

## 9. Estructura del repositorio

```
InverstigacionBase12/
├─ Cargo.toml            # workspace
├─ crates/
│  ├─ dz-gates/          # Capa 0 — trait + backends mvl/ crt/ table/
│  ├─ dz-int/            # Capa 1
│  ├─ dz-num/            # Capa 2
│  ├─ dz-sci/            # Capa 2.5 — sqrt, exp, ln, trigonometría
│  ├─ dz-parse/          # Capa 3
│  ├─ dz-eval/           # Capa 4 — AST + registro de funciones
│  ├─ dz-api/            # Capa 5 — entrada única y sin estado de las interfaces
│  └─ dz-cli/            # Capa 6 — REPL + modo de una invocación
├─ app/                  # Capa 6 — GUI Tauri (workspace propio)
│  ├─ src-tauri/         # comandos, AppState, DTOs con serde
│  └─ src/               # frontend vanilla TS + Vite
├─ docs/                 # notas de investigación, conteos de puertas, benchmarks
│  ├─ 00-fundamentos.md
│  ├─ 01-diseno-sumador.md
│  ├─ 02-crt-vs-mvl.md
│  ├─ 03-formato-dfp.md
│  ├─ 04-fase6-interfaces.md
│  └─ 05-funciones-cientificas.md
└─ tools/lint-purity.ps1
```

---

## 10. Fases — estado

Actualizado tras los commits `ad31650`, `47bb0e9`, `18565f8`, `1bb7b80`.
Ver `CLAUDE.md` para rutas y comandos.

| Fase | Entregable | Estado |
|---|---|---|
| **0. Setup** | Workspace de 6 crates que compila | **Hecha** |
| **1. Puertas** | `dz-gates`, 3 backends | **Hecha** — 144/144 diferenciales |
| **2. Sumador** | Semi/full adder + ripple-carry | **Hecha** — `half_adder` = 151 puertas |
| **3. Enteros** | `dz-int`, complemento a 12, división restauradora | **Hecha** |
| **4. REPL** | `dz-cli` | **Hecha** — `4;6 × 3 = 11;6` |
| **5. Fraccionarios** | Punto fijo, racionales exactos, DFP | **Hecha** — `1÷3` da `1/3` exacto |
| **6. GUI** | App Tauri | **Hecha** — `dz-api` + CLI de una invocación + GUI Tauri con teclado dozenal, historial y memoria |
| **7. Panel investigación** | Conteo de puertas + visor de circuito | Parcial — el conteo por operación ya se muestra en la GUI y en `--gates`; falta el visor paso a paso y el export CSV |
| **8. Extensiones** | Carry-lookahead, multiplicador v2, `DozOp` | **Hecha** — v2, `lookahead_add` y el experimento de rebalanceo cerrados; el registro de funciones (antes `DozOp`) en uso real |

Estado: 108 tests, clippy limpio, lint de pureza con 4 reglas. El DFP se adelantó
de la Fase 8 a la 5; el multiplicador v2 se adelantó de la 8 a antes de la GUI
porque su coste bloqueaba los property tests de racionales. Las funciones
trascendentes, que la §10bis-E daba por fuera de alcance, entraron con la Fase 6
(`crates/dz-sci`, `docs/05-funciones-cientificas.md`): lo que faltaba no era
CORDIC sino sintaxis de llamada y una capa donde poner el análisis numérico.

---

## 10bis. Lo que falta, en orden sugerido

### A. Carry-lookahead dozenal (Fase 8) — **hecho**
`lookahead_add` en `dz-gates/src/circuits/adder.rs`, sin tocar capas superiores.
- `G` no es literalmente `thr(12−y, x)` como decía este plan: `thr` toma un
  umbral **constante**, no un `Digit`. El circuito real es el decodificador
  `MAX_j MIN(lit(j,y), thr(12−j,x))`, que es el acarreo del `half_adder` antes
  de digitalizarlo. `P = lit(↋, s)` sí es literal.
- Red de prefijos por doblado recursivo (Kogge-Stone) sobre el operador de
  bloque `(G,P)∘(G',P')`: reducciones binarias balanceadas, prefijos de
  propagación compartidos. Nada de `fold` lineal.
- Profundidad medida con un trait `GateNet` nuevo (`circuits/depth.rs`) y dos
  instanciaciones, `Values` y `Levels`, sobre **un solo** cuerpo de circuito.
  `GateCount` no se tocó.
- **Resultado, y fue una sorpresa:** el lookahead gana en las dos métricas a la
  vez desde n = 1 (no solo en profundidad). 16 dígitos: 4 848 → 2 644 puertas
  (0,55×) y 270 → 25 niveles (0,09×). La causa es que el `full_adder` del ripple
  encadenaba dos `half_adder` por posición y el lookahead usa uno solo más un
  mux de 5 puertas.
- Entregado: filas en `examples/gate_counts.rs`, sección «Carry-lookahead
  dozenal (Fase 8)» en `docs/01-diseno-sumador.md`, 13 tests en
  `dz-gates/tests/lookahead.rs`.
- **Pendiente derivado, ya resuelto:** el pliegue lineal de 12 `max` del
  `half_adder` era el cuello de botella de profundidad (15 de los 17-25
  niveles). Se construyó y midió la variante balanceada
  (`half_adder_balanced`/`lookahead_add_balanced`, función nueva e
  independiente, sin tocar la de producción) y **no ganó**: el pliegue de
  `carry` sí balancea como se esperaba (14 → 6 en crudo), pero el de `sum`
  no, porque sus 12 ramas no tienen la misma profundidad de entrada
  (`cycle_pow(x,j)` ya es `j` niveles hondo) — balancearlo posicionalmente
  cuesta un nivel en vez de ahorrarlo (14 → 15), así que la profundidad de
  `half_adder` se queda en 15 y la de `lookahead_add` en n=16 sube de 25 a
  26 en vez de bajar a ~17. Solo se gana un poco de conteo de puertas (151 →
  149 en `half_adder`). Detalle en `docs/01-diseno-sumador.md` sección
  "Rebalanceo del pliegue lineal (experimento cerrado)". `half_adder`,
  `generate_propagate` y `lookahead_add` siguen siendo el circuito de
  producción sin cambios. `dz-int` tampoco adoptó `lookahead_add` todavía
  (deuda de la parte D, no de esta).

### B. GUI Tauri (Fase 6) — **hecha**
- Teclado de 12 dígitos, display, historial, memoria: `app/`, Tauri 2 + vanilla
  TS/Vite. `app/src-tauri` es un workspace propio, así que el núcleo sigue
  compilando y testeando en segundos sin arrastrar el árbol de Tauri.
- El núcleo no se tocó para esto, pero sí se **añadió una capa**: `dz-api`
  (capa 5, sin estado) es la entrada única de las tres interfaces (REPL, modo de
  una invocación, GUI). El diseño y sus desviaciones están en
  `docs/04-fase6-interfaces.md` §§1–8 (diseño) y §9 (lo construido).
- El modo de una invocación (`dz-cli sum 1 3`, `dz-cli eval "sin(pi/4)"`,
  `--json`, `--gates`, `--scale`) vive en el mismo binario que el REPL.
- **Riesgo del render de `↊`/`↋`**: mitigado sin decidir todavía la tipografía.
  El frontend mide en un canvas si la fuente tiene los glifos y, si no, muestra
  los alias `X`/`E` — solo en presentación; los datos que manda Rust siempre
  llevan los glifos canónicos. Empotrar una fuente sigue siendo la mejora
  natural.
- Extra no planeado, pedido durante la fase: **funciones científicas**
  (`sin`, `cos`, `tan`, `sqrt`, `exp`, `ln`, `log`, inversas, `pi`) en
  `crates/dz-sci`, registradas en `dz-eval` y expuestas solas en las tres
  interfaces por el catálogo del registro. Ver `docs/05-funciones-cientificas.md`.

### C. Panel de investigación (Fase 7) — donde esto deja de ser una calculadora
- Selector de backend en caliente (`DozGates` ya es un trait, está preparado).
- Contador de puertas de la última operación (`GateCount` ya existe).
- Visor paso a paso del ripple-carry: ver el acarreo propagarse dígito a dígito.
- Export CSV de trazas.

### D. Deuda conocida, no urgente
- **DFP:** sin subnormales ni cero con signo. `evaluate_dfp` redondea una vez al
  final de la expresión en vez de tras cada operación, así que `1/0` dentro de una
  expresión falla en la etapa racional en vez de dar `Inf`.
- **Velocidad de racionales:** ~106 s en debug (~1 s en release). La causa es
  Euclides llamando a `div_rem` en cada normalización. Si molesta, cachear o
  implementar GCD binario dozenal.
- **`digit_mul_v1`:** conservado a propósito para la comparación de la
  investigación. No borrarlo.

### E. ~~Fuera de alcance por ahora~~ — hecho, y la premisa era falsa
Las trascendentes (log, trig) están implementadas en `crates/dz-sci` con series
más reducción de argumento, no con CORDIC. Lo que las bloqueaba no era la
matemática: el lenguaje no tenía sintaxis de llamada, `DozOp` nunca llegaba a
ejecutarse, y no existía una capa para el análisis numérico. CORDIC dozenal
sigue siendo interesante, pero como **experimento comparativo de coste** de la
misma familia que "CRT vs MVL" o "ripple vs lookahead" — cambia
multiplicaciones por desplazamientos y sumas, y la pregunta medible es cuánto
baja el conteo de puertas de `sin`. Números actuales y limitaciones en
`docs/05-funciones-cientificas.md`.

---

## 11. Prerrequisitos — instalados

- **rustup / cargo** — rustc 1.98.1. **No está en el PATH**; vive en
  `C:\Users\ADMIN\.cargo\bin`. Ver `CLAUDE.md`.
- **cargo-tauri** 2.11.4 — instalado.
- **Node** v24.13.0 — instalado.
- **WebView2** — incluido en Windows 11.
- **Visual Studio Build Tools** — presente (el workspace enlaza correctamente).

---

## 12. Riesgos y decisiones abiertas

- ~~**Render de ↊ ↋:**~~ **mitigado, no cerrado.** El modo de una invocación del
  CLI usa alias ASCII por defecto (`--unicode` para glifos); el REPL sigue con
  glifos; la GUI detecta en tiempo de ejecución si la fuente los tiene y cae a
  `X`/`E` si no. Falta la prueba visual y la decisión sobre empotrar una fuente
  (`docs/04-fase6-interfaces.md` §5 y §9.2.8).
- ~~**Detección de acarreo en CRT**~~ — **resuelto, y el resultado fue negativo.**
  CRT no compensa: necesita reconstruir el dígito para toda comparación de orden,
  176 reconstrucciones en un `half_adder` y 13.938 en un `multiply` 3×2. La
  ventaja de descomponer nunca llega. Documentado en `docs/02-crt-vs-mvl.md`.
  Es un hallazgo publicable, no un fallo.
- ~~**Formato DFP**~~ — definido en `docs/03-formato-dfp.md`: signo + exponente
  sesgado de 2 dígitos (bias 71) + mantisa de 12 dígitos, half-even correcto.
- **Rendimiento del backend MVL:** sigue siendo lento por diseño. El multiplicador
  v2 lo bajó 38×. Cachear más agresivamente es posible, pero cuidado: cachear la
  *semántica* de una operación derivada es exactamente el bug que arregló
  `18565f8`. Cachear resultados sí; importar significado de aritmética nativa no.
- **Alcance:** resistir meter trascendentes antes de cerrar la Fase 7.
