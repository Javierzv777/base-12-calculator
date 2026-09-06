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
Capa 5  UI Tauri (teclado dozenal, visor de circuito, historial)
Capa 4  Motor de evaluación (AST, precedencia, funciones extensibles)
Capa 3  Lexer/Parser dozenal (nunca pasa por i64/f64)
Capa 2  Números: punto fijo, racionales exactos, float dozenal (DFP)
Capa 1  Enteros dozenales (Dz): suma, resta, mult, div, comparación
Capa 0  Puertas lógicas 12-valuadas (backends intercambiables)
```

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
- Trait `DozOp { fn arity(); fn apply(&[Value]) -> Result<Value> }` → registro de operaciones. **Este es el punto de extensión.** Añadir una función nueva = implementar el trait y registrarla, cero cambios en el core.
- Operaciones v1: `+ − × ÷ mod ^ neg` y paréntesis.
- Manejo de errores como valores (división por cero, overflow de precisión).

---

## 7. Capa 5 — UI Tauri

- Teclado de 12 dígitos + operadores. Fuente con soporte ↊↋ (o SVG propios si falla el render — probable en Windows).
- Display dozenal, historial, memoria.
- **Panel de investigación** (lo que distingue esta app):
  - Selector de backend en caliente (MVL / CRT / tabla).
  - Contador de puertas y profundidad de circuito de la última operación.
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
│  ├─ dz-parse/          # Capa 3
│  ├─ dz-eval/           # Capa 4
│  └─ dz-cli/            # REPL — primer entregable usable
├─ app/                  # Tauri (Capa 5)
├─ docs/                 # notas de investigación, conteos de puertas, benchmarks
│  ├─ 00-fundamentos.md
│  ├─ 01-diseno-sumador.md
│  ├─ 02-crt-vs-mvl.md
│  └─ 03-formato-dfp.md
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
| **6. GUI** | App Tauri | Pendiente |
| **7. Panel investigación** | Conteo de puertas + visor de circuito | Pendiente |
| **8. Extensiones** | Carry-lookahead, multiplicador v2, `DozOp` | Parcial — v2 y `lookahead_add` hechos, `DozOp` en uso |

Estado: 65 tests, clippy limpio, lint de pureza con 4 reglas. El DFP se adelantó
de la Fase 8 a la 5; el multiplicador v2 se adelantó de la 8 a antes de la GUI
porque su coste bloqueaba los property tests de racionales.

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
- **Pendiente derivado:** el pliegue lineal de 12 `max` del `half_adder` es
  ahora el cuello de botella de profundidad (15 de los 17-25 niveles). No se
  rebalanceó a propósito: convertiría el 151 publicado en 150 y eso hay que
  decidirlo aparte. `dz-int` tampoco adoptó `lookahead_add` todavía.

### B. GUI Tauri (Fase 6) — el salto grande
- Teclado de 12 dígitos, display, historial, memoria.
- Reutiliza `dz-eval` tal cual; el núcleo no se toca.
- **Riesgo real:** el render de `↊`/`↋` (U+218A/218B) falla en muchas fuentes de
  Windows. Decidir pronto entre SVG propios o alias ASCII por defecto. No dejarlo
  para el final.
- Prerrequisitos ya instalados (ver Sección 11).

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

### E. Fuera de alcance por ahora
Funciones trascendentes (log, trig). Requieren CORDIC dozenal — es un proyecto
propio, no una extensión. No empezarlo antes de que la Fase 7 esté cerrada.

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

- **Render de ↊ ↋:** sigue abierto y es el riesgo principal de la Fase 6. Poco
  soporte tipográfico en Windows. Decidir entre SVG propios o alias ASCII por
  defecto antes de construir el teclado, no después.
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
