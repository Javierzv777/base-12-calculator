# Traspaso — exponente de irracionalidad de π con la base 12 como enfoque

Documento de traspaso entre equipos (2026-10-11). Lo dejó la sesión del equipo
original para que una sesión en un equipo más rápido termine el cálculo
pesado y redacte el hallazgo `14`. **Este fichero no es el hallazgo**: es el
contexto, lo ya medido y la lista de pendientes. Cuando el hallazgo `14`
exista, este fichero puede borrarse o quedarse como registro.

Antes de nada, leer `CLAUDE.md` (reglas del repo), la memoria del proyecto si
existe en el otro equipo, `Investigations/99-sintesis.md` §4.12–§4.13 y
`Investigations/13-hallazgo-riemann-base12.md` (el documento hermano: mismo
protocolo, misma estructura de casillas I–IV, predicciones antes de medir,
"lo que no funcionó", tabla de correcciones).

---

## 1. La fuente que se está analizando

- Repositorio: <https://github.com/openai/math> — 719 manuscritos generados
  por un modelo interno de OpenAI, agrupados en 372 familias; ~42 % de los
  resultados principales con formalización en Lean. El propio README avisa de
  que algunos resultados no formalizados "could have issues".
- Familia **017**: *The irrationality exponent of π is 2* (preprint fechado el
  24 de septiembre de 2026, autor "OpenAI").
  - Paper: `preprints/The-irrationality-exponent-of-pi-is-2-September-24-2026/paper.pdf`
  - Fuente LaTeX: misma carpeta, `build/main.tex`, `build/sections/{interpolation,determinant,conclusion,series}.tex`,
    `build/references.bib`.
  - Traza de razonamiento resumida: `reasoning_traces/irrationality-exponent-of-pi.pdf`
  - Alcance de Lean: `lean/docs/017.md`. Enunciado del reto:
    `lean/ComparatorChallenges/PiExponent.lean` (con `sorry`, es el enunciado)
    y `PiExponent.json` (solución `OAI.NumberTheory.PiExponent.Main`, axiomas
    permitidos `propext`, `Quot.sound`, `Classical.choice`). La prueba formal
    vive en `lean/OAI/NumberTheory/PiExponent/` (Ampleness, Analysis,
    Approximation, Cohomology, Geometry, Jets, LocalAlgebra, Polynomials,
    Main.lean, Statement.lean).
  - **No se verificó** la formalización en este equipo (no se instaló Lean ni
    `comparator`). El corolario de Flint–Hills queda **fuera** del alcance
    formalizado según `017.md`.

### Qué afirma

`μ(π) = 2`: para todo `ν > 2` existe `Q(ν)` tal que `|π − p/q| ≥ q^{−ν}` para
todo `p ∈ ℤ` y todo `q ≥ Q(ν)` (fracciones reducidas o no). Antes, el mejor
resultado publicado era `μ(π) ≤ 7,103205…` (Zeilberger–Zudilin 2020; Bai 2026
afirma 7,10186…). Corolarios: la serie de Flint–Hills `Σ 1/(n³ sin² n)`
converge, y `Σ 1/(nᵃ |sin n|ᵇ)` converge si y solo si `a > max{1, b}`.

### Cómo lo prueba (resumen fiel de `main.tex` y `conclusion.tex`)

- Método de **determinantes de interpolación** (línea Laurent/Roth), no las
  formas lineales con denominadores `lcm(1..n)` de Hata/Salikhov/Zeilberger–Zudilin.
- Coordenadas `Y, X₁…X_m` con pesos; monomios bajo una desigualdad de grado
  ponderado (símplices completos); jets en centros `(1, c_{j1}, …, c_{jm})`
  a lo largo de curvas logarítmicas `Y = 1+t, Xᵢ = c_{ji} + uᵢ + log(1+t)`.
  Teorema de interpolación multicentro probado con geometría algebraica
  (blowups, nef + amplio, Serre, estimaciones de ceros).
- Si hubiese aproximaciones `|π − p/q| ≤ q^{−ν}` con `q` no acotado, se toman
  `m` de ellas sucesivas, centros `c_{ji} = 2i·j·pᵢ/qᵢ` que aproximan los
  periodos `2πi·j`, y un menor no nulo de una matriz sobre `ℚ(i)`: cota
  inferior aritmética contra cota superior analítica ("collision saving"
  cuadrático por grupos de filas con el mismo índice transversal). Contradicción.
- Parámetros (Lemma `par:constants`): racionales `θ, A, B, C` con
  `0<θ<A<B<1`, `ν(A−θ) > 1−θ`, `C>1`, `B<1/C`, `B<Cθ<1`; dimensión `m`,
  `K = ⌊Cᵐ⌋`, `w₀ = B^{−m}`, `v₀ = 2Kθᵐw₀`, `Λ = 4 log 2`, `c = (log 2)/4`.
- **No efectivo**: el texto dice expresamente que `Q(ν)` no se obtiene de forma
  efectiva.
- No aparecen primos, `lcm`, valoraciones ni denominadores de la forma `2ᵃ3ᵇ`:
  **ningún objeto de la prueba depende de la base de numeración.**

---

## 2. Lectura con el criterio de la serie (provisional)

- **(IV) para "¿ayuda la base 12 a la prueba?"**: la prueba vive en ℂ y en
  geometría algebraica; no hay punto de entrada para una base.
- **(IV) más fuerte que en `13`**: la HR es Π₁ (un contraejemplo finito la
  refutaría). `μ(π) = 2` tiene la forma `∀ν ∃Q ∀q ≥ Q`, y además `Q` no es
  efectivo: **ningún cálculo finito puede ni confirmarlo ni refutarlo** (una
  aproximación excepcional sola no lo contradice; harían falta infinitas). El
  instrumento solo puede mostrar comportamiento a escala finita.
- **(III) lo único ligado a la base**: el teorema acota las rachas de cifras
  `0` o `↋` (o cualquier cifra repetida de forma que dé una fracción `p/12ᵏ`
  muy buena) en el desarrollo dozenal de π: una racha de longitud `r` tras la
  cifra `k` da `|π − p/12ᵏ| < 12^{−(k+r)}`, y `μ(π)=2` implica
  `r ≤ (1+ε)k + O(1)`. Con la cota previa 7,103 solo se tenía `r ≤ 6,103k`.
  Vale igual en cualquier base (por eso es III compartida, no ventaja de 12).
- Los desarrollos posicionales son malos aproximantes (exponente ~1); las
  convergentes de la fracción continua (que no tiene base) dan 2.

---

## 3. Lo ya medido en el equipo original

Instrumento: `target/release/dz-cli.exe`, backend MVL. Todos los guiones están
en esta carpeta y se ejecutan desde la raíz del repo.

| qué | resultado | fichero |
|---|---|---|
| π a escala 100 (`dz-cli eval "pi" --mode fixed --scale 100`, 26 s) | **99 cifras dozenales correctas** (contraste con Machin en `BigInt`, oráculo externo); la cifra 100 difiere por truncado | `pi-scale100.out` |
| Rachas de `0`/`↋` en esas 99 cifras | la más larga es `↋↋` (cifras 25–26); exponente local de truncar `1 + r/k ≤ 1,25`, lejos de 2 | (cálculo con node sobre la cadena del instrumento) |
| Fracción continua de π **certificada por intervalo** (`cf.sh`): CF de `L = π₉₉` y de `U = L + 12⁻⁹⁹` en modo racional exacto; se acepta un cociente solo si coinciden | 63+ cocientes en el momento del traspaso, todos coinciden con los conocidos (en dozenal: `a2=13` = 15, `a4=204` = 292, `a21=70` = 84, `a33=83` = 99) | `cf.out` (puede estar incompleto: ver pendientes) |
| Sumas parciales de Flint–Hills (`flint-hills.sh 400`), modo fijo escala 12 | en curso; n = 178 al traspasar, suma `4;981X…` | `flint-hills-400.out` (parcial) |
| Términos "pico" de Flint–Hills (`flint-hills-spikes.sh 24`) | `n = 355 = 257₁₂` da `~20;7217X76…` (≈ 24,6), 276 M puertas | probar entero en el otro equipo |
| Coste de π (con `4*atan(1)`, porque `pi` marca 0 puertas por estar memoizado) | escala 12: 352.713.609 puertas (2 s); 24: 1.053.849.683 (6 s); 48: 4.268.608.794 (26 s) | — |

### Cambios de código hechos para esta investigación (ya en `main`)

- `isqrt` (commit `f0a4dd4`), de la investigación `13`.
- **Escala de división configurable** (este traspaso): `EvalConfig.div_scale`,
  `dz_api::division_scale(scale) = max(24, scale)`. Antes, `a/b` en modo fijo y
  toda fracción leída con `--input-base 10` se truncaban a 24 cifras dozenales
  aunque `--scale` fuese 100. Ahora un π decimal de ~130 cifras leído con
  `--input-base 10 --scale 100` acierta 99 cifras dozenales (antes 24).
  276 tests en verde; `1+1`, `4;6*3`, `sqrt(2)` sin cambios.

### Trampas del instrumento que muerden aquí

- `pi` marca **0 puertas** (memoizado): para medir coste usar `4*atan(1)`.
- `--scale` admite como máximo **100** (`parse_count` en
  `crates/dz-cli/src/main.rs`). Para más cifras de π habría que subir ese tope
  (ver pendiente P3).
- Literales sin `--input-base 10` son **dozenales** (`12` es catorce).
- La salida JSON trae glifos `↊`/`↋` (multibyte); para manipular cadenas en
  bash usar la salida ASCII (`X`/`E`) o tenerlo en cuenta al comparar.
- Una tarea en segundo plano de Claude Code muere a las 2 h y una suspensión
  del equipo la deja colgada: los guiones largos deben ser **reanudables**.
- `cargo build` no puede reemplazar `dz-cli.exe` mientras un guion lo está
  usando: para tests con corridas en marcha, `CARGO_TARGET_DIR` aparte.

---

## 4. Pendientes para el equipo con más capacidad

En orden de prioridad. Cada uno dice qué correr y qué entregar.

**P1. Terminar la fracción continua certificada.** Si `cf.out` no termina en
una línea `stop: L gives …, U gives … at index k`, relanzar
`bash Investigations/14-pi-exponent/cf.sh > Investigations/14-pi-exponent/cf.out`
(no es reanudable; tarda del orden de una hora con 99 cifras). Contrastar con la
CF conocida de π (oráculo externo permitido para contrastar). Esperado: ~95–100
cocientes certificados con 99 cifras dozenales.

**P2. Exponente local de cada convergente.** Escribir `convergents.sh`: a partir
de `cf.out`, construir `pₙ/qₙ` en modo racional exacto con el instrumento
(`p_{n} = a_n p_{n−1} + p_{n−2}`, ídem `q`), y medir
`λₙ = −ln|π − pₙ/qₙ| / ln qₙ` con `dz-cli eval ... --mode fixed --scale 100`
(usar `π₉₉` como literal; ojo: `|π − pₙ/qₙ|` debe ser ≫ 12⁻⁹⁹ para que el
valor sea fiable: descartar las últimas convergentes). Entregar la tabla
`n, aₙ, pₙ/qₙ en dozenal, λₙ`. Esperado: `λ` alrededor de 2, con picos donde
`a_{n+1}` es grande (`355/113`: `λ ≈ 3,2`, por `a₄ = 292`). Es la imagen finita
de `limsup λₙ = μ(π) = 2`, que el teorema afirma y ningún cálculo finito decide.

**P3 (opcional, pesado). Más cifras de π.** Subir el tope de `--scale` en
`parse_count` (p. ej. a 400) con un test, y calcular `pi` a escala 300–400.
Coste: crece algo más que cuadráticamente (×3–4 al duplicar la escala); a 100
fueron 26 s. Con más cifras: más cocientes certificados (P1) y estadística de
rachas de cifras (§2) sobre cientos de cifras, comparando `max r/k` contra la
cota `(1+ε)` del teorema y contra la anterior `6,103`.

**P4. Flint–Hills.** (a) Terminar las sumas parciales hasta 400 si no acabaron
aquí (`flint-hills-400.out`; el guion original no es reanudable — usar
`flint-hills-resume.sh 400 Investigations/14-pi-exponent/flint-hills-400.out`,
que continúa desde la última fila). (b) Extender a `N = 2000` o más con el
mismo guion reanudable. (c) Correr `flint-hills-spikes.sh 24` (términos en
`n = 3, 22, 333, 355, 103993, 104348, 208341, 312689, 833719, …`, los
numeradores de convergentes): son unas pocas llamadas y muestran que los picos
son los que deciden la convergencia; comprobar que el `sin` conserva cifras
significativas (si no, subir la escala). Entregar: sumas parciales en dozenal
en hitos (100, 355, 400, 1000, 2000) y la tabla de picos. Recordar que la
convergencia **no** es comprobable numéricamente: es lo que afirma el
corolario (fuera del alcance formalizado).

**P5. Redactar `Investigations/14-hallazgo-exponente-irracionalidad-pi.md`**
con el protocolo de la serie: predicciones antes de medir (las de abajo ya
están registradas aquí, antes de P1–P4), resumen de veredicto con casillas,
"lo que no funcionó", "lo que no se hizo", correcciones, reproducción. Añadir
§4.14 a `99-sintesis.md` y la entrada en la memoria del proyecto. Commit y push
cuando el usuario lo pida.

### Predicciones registradas en este traspaso (antes de P1–P4)

- **PE1.** Ninguna medida dirá nada sobre la verdad de `μ(π) = 2`; el enunciado
  no es decidible por cálculo finito (ni siquiera refutable, a diferencia de la
  HR).
- **PE2.** La CF certificada con 99 cifras dozenales dará entre 90 y 105
  cocientes, todos iguales a los conocidos.
- **PE3.** Los exponentes locales `λₙ` de las convergentes estarán en `[2, 3,3]`
  y el máximo será el de `355/113`.
- **PE4.** Las rachas de cifras repetidas en el desarrollo dozenal de π tendrán
  `max r/k < 0,5` para `k ≥ 10` en todas las cifras disponibles: el teorema no
  muerde a esta escala, y la base 12 no se comporta distinto de la 10.
- **PE5.** La suma parcial de Flint–Hills salta en `n = 355` por ≈ 24,6
  (≈ `20;7₁₂`) y el siguiente salto comparable no aparece antes de
  `n = 103993`.

---

## 5. Cómo arrancar en el otro equipo

```bash
git pull
export PATH="$PATH:$HOME/.cargo/bin"
cargo build -p dz-cli --release
cargo test --workspace --release      # 276 tests
# luego P1–P5 desde la raíz del repo
```

Para lanzar un agente: darle este fichero como contexto de entrada y pedirle
P1–P5 en orden, con corridas largas en segundo plano y guiones reanudables.
