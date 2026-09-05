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

## 10. Fases

| Fase | Entregable | Criterio de "hecho" |
|---|---|---|
| **0. Setup** | Toolchain + workspace vacío que compila | `rustup` instalado, `cargo test` verde |
| **1. Puertas** | `dz-gates` con 3 backends | 144/144 diferenciales pasan |
| **2. Sumador** | Semi/full adder + ripple-carry | Suma de N dígitos correcta vs oráculo |
| **3. Enteros** | `dz-int`: ± × ÷ comparación, complemento a 12 | Property tests verdes |
| **4. REPL** | `dz-cli` — **primera cosa usable** | `4;6 × 3 = 11;6` en terminal |
| **5. Fraccionarios** | Racionales exactos + punto fijo | `1÷3` da `0;4` exacto |
| **6. GUI** | App Tauri con teclado y display | Ejecutable Windows funciona |
| **7. Panel investigación** | Conteo de puertas + visor de circuito | Export CSV de trazas |
| **8. Extensiones** | Carry-lookahead, DFP, funciones nuevas vía `DozOp` | Gráficos comparativos en `docs/` |

Las fases 0–4 son el camino crítico. Fase 4 ya te da una calculadora dozenal real y auditable. Todo lo demás se apila encima sin tocar el núcleo.

---

## 11. Prerrequisitos a instalar

- **rustup** (no está instalado): https://rustup.rs → toolchain `stable-x86_64-pc-windows-msvc`
- **Visual Studio Build Tools** con "Desktop development with C++" (linker MSVC)
- **WebView2** — ya viene en Windows 11, no requiere acción
- **Tauri CLI:** `cargo install tauri-cli` (Node ya está instalado para el frontend)

---

## 12. Riesgos y decisiones abiertas

- **Render de ↊ ↋:** poco soporte tipográfico en Windows. Mitigación: SVG propios como fallback, y alias ASCII configurable desde el día 1.
- **Detección de acarreo en CRT:** puede resultar tan cara que anule la ventaja del backend CRT. Es un resultado válido de la investigación, no un fallo — documentarlo.
- **Rendimiento del backend MVL:** simular puertas es lento. Aceptable para una calculadora; si molesta, cachear tablas sintetizadas (sin romper la semántica).
- **Formato DFP:** no hay estándar. Hay que definirlo y justificarlo.
- **Alcance:** resistir la tentación de meter funciones trascendentes antes de la Fase 6. Log y trig en base 12 requieren CORDIC dozenal — proyecto propio.
