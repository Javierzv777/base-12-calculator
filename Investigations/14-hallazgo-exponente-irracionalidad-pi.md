# Hallazgo — El exponente de irracionalidad de π leído con la base 12: lo que un cálculo finito puede mostrar de `μ(π) = 2`

Sexta extensión posterior al cierre. La abre un preprint del repositorio
<https://github.com/openai/math> (familia 017, *The irrationality exponent of π
is 2*, fechado el 2026-09-24), traído por el usuario con la pregunta de siempre:
*¿qué puede decir de esto el instrumento en base 12?* El trabajo empezó en otro
equipo (2026-10-11), que dejó contexto, primeras medidas y predicciones en
`14-pi-exponent/HANDOFF.md`; la sesión que redacta este documento terminó el
cálculo (P1–P4 del traspaso) en el mismo día.

Protocolo de `00-plan-investigacion.md`: toda cifra dozenal sale de
`target/release/dz-cli.exe` (backend MVL) con el guion al lado; los literales
sin más son **dozenales**. Los guiones están en `14-pi-exponent/` (§11). Donde
se usó un **oráculo externo** (Node con `BigInt`, π por Machin a 300 cifras
decimales) se dice expresamente; ninguna cifra dozenal del documento sale de él.
Las cifras decimales que acompañan a las dozenales son traducción para leer, no
medida.

**Recordatorio no negociable.** `μ(π) = 2` no se verifica aquí ni puede
verificarse con un cálculo finito (§2). Tampoco se verificó el preprint: ni su
formalización en Lean ni su argumento. Su resumen en §1 viene del traspaso, que
lo extrajo de la fuente LaTeX; esta sesión no lo releyó. Lo que el documento
hace es separar qué parte del enunciado toca la base 12 (casilla III), qué parte
no se traslada (IV), y medir las imágenes finitas del enunciado que el
instrumento puede producir.

Casillas como en `11`–`13`: solo (III) "efecto de la representación" y (IV) "la
pregunta no se traslada"; (I) y (II) hablan de métrica y aquí no aplican.

**Numeración.** El traspaso pedía resumir esto en §4.14 de la síntesis. Ese
número lo tomó, al integrar el trabajo de otra máquina, la frontera de la bola
L₁ (`15`); este documento va en **§4.16**.

---

## 0. Resumen del veredicto (para quien no lea el resto)

| pregunta | veredicto | casilla |
|---|---|---|
| ¿Ayuda la base 12 a la prueba? | **No.** La prueba vive en ℂ y en geometría algebraica (determinantes de interpolación, blowups, estimaciones de ceros); no aparecen primos, `lcm` ni denominadores `2ᵃ3ᵇ`. | (IV) |
| ¿Puede el instrumento confirmarlo o refutarlo? | **Ninguna de las dos.** `∀ν ∃Q ∀q ≥ Q` con `Q` no efectivo: una aproximación excepcional sola no lo refuta y ninguna lista finita lo confirma. Más fuerte que la HR de `13`, que al menos es refutable. | (IV) |
| Fracción continua de π, certificada por intervalo | **104 cocientes** (`a0`–`a103`) desde las 99 cifras dozenales de π del instrumento, 0 discrepancias con el oráculo. En `a104` el intervalo deja de decidir. | — |
| Exponente local `λₙ` de las convergentes | 95 convergentes medidas (error ≤ 4,2·10⁻¹² frente al oráculo). Todas `> 2`, como debe ser; el máximo es **22/7 (3,43)**, no 355/113 (3,20); de `n = 41` en adelante, `λ ∈ [2,0007; 2,056]`. Es la imagen finita de `limsup λₙ = 2`. | — |
| Rachas de cifras en el desarrollo de π | Lo único ligado a la base: el teorema acota las rachas de `0`/`↋`. En 99 cifras no hay ninguna racha de 3; `max r/k = 0,083` (de `0`/`↋`), `0,154` (cualquier cifra); en base 10, `0,047` y `0,087`. El teorema no muerde a esta escala y las dos bases se comportan igual. | (III), compartida con toda base |
| Flint–Hills `Σ 1/(n³ sin² n)` | Sumas parciales por puertas hasta `n = 2000`: `S(2000) = 26;3841E08₁₂ ≈ 30,30796`. El salto de `n = 355` (`20;7217₁₂ ≈ 24,598`) es el 81 % de la suma; los picos posteriores en numeradores de convergentes son `≤ 10⁻⁵`. La convergencia es lo que afirma el corolario; no es comprobable numéricamente. | (IV) |

La conclusión de fondo repite la de `11`–`13`: **el exponente de irracionalidad
no tiene base.** Un desarrollo posicional, en cualquier base, aproxima con
exponente ≈ 1; las convergentes de la fracción continua, que no tiene base,
aproximan con exponente 2. Lo que el teorema dice sobre las cifras de π lo dice
igual en base 10.

---

## 1. La fuente y qué afirma

- Repositorio: <https://github.com/openai/math> — 719 manuscritos generados por
  un modelo interno de OpenAI, en 372 familias; ~42 % de los resultados
  principales con formalización en Lean. El README avisa de que algunos
  resultados no formalizados "could have issues".
- Familia 017: `preprints/The-irrationality-exponent-of-pi-is-2-September-24-2026/`
  (paper, `build/main.tex` y secciones). Alcance formal en `lean/docs/017.md`;
  enunciado del reto en `lean/ComparatorChallenges/PiExponent.lean`. El corolario
  de Flint–Hills está **fuera** del alcance formalizado.

**Enunciado.** `μ(π) = 2`: para todo `ν > 2` existe `Q(ν)` tal que
`|π − p/q| ≥ q^{−ν}` para todo `p` y todo `q ≥ Q(ν)`. La mejor cota publicada
antes era `μ(π) ≤ 7,103205…` (Zeilberger–Zudilin 2020). Corolarios: `Σ 1/(n³ sin² n)`
converge, y `Σ 1/(nᵃ |sin n|ᵇ)` converge si y solo si `a > max{1, b}`.

**Método** (resumen del traspaso, no releído aquí). Determinantes de
interpolación en la línea Laurent/Roth, no las formas lineales con `lcm(1..n)`
de Hata/Salikhov/Zeilberger–Zudilin. Jets en centros `(1, c_{j1}, …, c_{jm})`
a lo largo de curvas logarítmicas; si existieran aproximaciones
`|π − p/q| ≤ q^{−ν}` con `q` no acotado, los centros `c_{ji} = 2i·j·pᵢ/qᵢ`
aproximarían los periodos `2πi·j` y un menor no nulo sobre `ℚ(i)` daría una
contradicción entre cota aritmética y analítica. El texto dice expresamente que
`Q(ν)` **no es efectivo**.

Este documento no evalúa si la prueba es correcta. Que un resultado de este
tamaño (de 7,10 a 2) venga de un preprint generado por un modelo, con
formalización parcial no verificada aquí, es razón para tratarlo como
afirmación y no como teorema. Nada de lo medido abajo depende de que sea cierto:
las medidas son las mismas con `μ(π) = 2` o con `μ(π) = 7`.

---

## 2. ¿Puede un cálculo decir algo de `μ(π)`? (IV)

La HR (`13` §0) es Π₁: un contraejemplo sería un cálculo finito. `μ(π) = 2` es
`∀ν > 2 ∃Q ∀q ≥ Q: |π − p/q| ≥ q^{−ν}`:

- **No es refutable en finito.** Una aproximación con `λ = 5` no lo contradice:
  el enunciado tolera cualquier cantidad *finita* de excepciones, y sin `Q`
  efectivo no hay ningún `q` a partir del cual una excepción cuente.
- **No es confirmable en finito.** Que las primeras 95 convergentes tengan
  `λ ≤ 3,43` es compatible con `μ(π) = 7`: basta que la primera excepción esté
  más allá.
- **Lo que sí da el cálculo** es la imagen a escala finita: cómo se ve
  `limsup λₙ = 2` en las convergentes que caben en 99 cifras, y cuánto de la
  suma de Flint–Hills está dentro de la ventana medida.

`PE1` (registrada en el traspaso): se cumple por construcción.

---

## 3. π en base 12 y las rachas de cifras (III), compartida

### 3.1 La medida

`dz-cli eval "pi" --mode fixed --scale 100` (26 s) da 99 cifras dozenales
correctas (`pi-scale100.out`, contrastado en el otro equipo con Machin en
`BigInt`); la cifra 100 difiere por truncado:

```
3;184809493E918664573X6211EE151551X05729290X7809X492742140X60X55256X0661X03753X3XX54805646880181X36831
```

### 3.2 Por qué las rachas son lo único ligado a la base

Una racha de `r` cifras `0` que empieza tras la cifra fraccionaria `k` da
`|π − p/12ᵏ| < 12^{−(k+r)}`, es decir, una aproximación de exponente
`1 + r/k` con denominador `12ᵏ`. Lo mismo con `↋` (redondeando hacia arriba) y,
con denominador `11·12ᵏ`, con cualquier cifra repetida. `μ(π) = 2` implica
`r ≤ (1 + ε)k + O(1)`; con la cota previa 7,103 solo se tenía `r ≤ 6,103k`.

Esto vale igual en cualquier base `b` con `bᵏ` en lugar de `12ᵏ`. Por eso es
(III) **compartida**: el teorema dice algo de las cifras de π en base 12, y lo
mismo de sus cifras en base 10.

### 3.3 Lo medido (`runs.mjs`, `runs-base10.mjs`)

| | base 12 (99 cifras, instrumento) | base 10 (107 cifras, oráculo) |
|---|--:|--:|
| rachas de longitud ≥ 2 | 8 | 6 |
| racha más larga | 2 | 2 |
| `max r/k`, `k ≥ 10`, cualquier cifra | 0,154 (`66` tras la cifra 13) | 0,087 (`33` tras la 23) |
| `max r/k`, `k ≥ 10`, cifra `0`/última | 0,083 (`↋↋` tras la 24) | 0,047 (`99` tras la 43) |

El número esperado de rachas de 2 en 99 cifras aleatorias de base 12 es
`98/12 ≈ 8`; salen 8. Nada distingue las cifras de π de cifras al azar a esta
escala, en ninguna de las dos bases. Las 107 cifras decimales equivalen en
información a las 99 dozenales (`99·log 12 / log 10 ≈ 106,8`), y el punto de
Feynman (`999999` en la cifra 762) queda muy fuera.

**`PE4`** (`max r/k < 0,5` para `k ≥ 10`, sin diferencia entre bases): se
cumple con margen; el teorema necesitaría `r/k` cerca de 1 para morder.

---

## 4. Fracción continua de π, certificada por intervalo (P1)

### 4.1 El método

π está en `[L, U]` con `L` = las 99 cifras de §3.1 truncadas y
`U = L + 12⁻⁹⁹`. `cf-resume.sh` desarrolla en modo **racional exacto**, dentro
del instrumento, las dos fracciones continuas (`floor` y `1/(x − a)`) y acepta
un cociente solo si `L` y `U` dan el mismo. El bucle vive en bash; cada valor es
del binario. A diferencia de `cf.sh` (otro equipo, se detuvo en `a75`),
guarda `x`, `y` y el índice tras cada cociente, así que una corrida cortada se
reanuda.

### 4.2 El resultado (`cf-resume.out`)

**104 cocientes certificados** (`a0`–`a103`). En el índice 104 `L` da `2` y
`U` da `3`: el intervalo ya no decide (el verdadero es `2`). Contraste con
`cf-oracle.mjs` (CF de π desde Machin a 300 cifras): **0 discrepancias** en 104.
Los 76 primeros coinciden además con el `cf.out` del otro equipo.

Cocientes grandes, en dozenal y decimal:

| índice | dozenal | decimal |
|--:|--:|--:|
| `a4` | `204` | 292 |
| `a21` | `70` | 84 |
| `a33` | `83` | 99 |
| `a79` | `115` | 161 |
| `a80` | `39` | 45 |

Coste: un paso `1/(x − a)` al final de la corrida, con numerador y denominador
de ~50 cifras, cuesta 901 M de puertas (0,5 s). 99 cifras dozenales dan 104
cocientes, que es lo que predice la constante de Lévy: `ln qₙ ≈ n·π²/(12 ln 2)`
y cada convergente fija `2 ln qₙ` nats, así que 99 cifras (`99 ln 12 ≈ 246`
nats) alcanzan para `246 / (2·1,1866) ≈ 103,7` cocientes. Es la ley de Lochs
para un número "típico"; π no se aparta de ella a esta escala.

**`PE2`** (entre 90 y 105 cocientes, todos iguales a los conocidos): se cumple.

---

## 5. Exponentes locales de las convergentes (P2)

### 5.1 El método

`convergents.sh` construye `pₙ/qₙ` con la recurrencia
`pₙ = aₙpₙ₋₁ + pₙ₋₂` en modo racional exacto y mide
`λₙ = −log|π − pₙ/qₙ| / log qₙ`. El error `d = |L − pₙ/qₙ|` se toma en modo
fijo a escala 100 (4.488 M de puertas por convergente; es la parte cara) y se
parte como `d = 12^{−z}·m`, donde `z` es el número de ceros tras `;` (un conteo
de posiciones, leído a dozenal con `--input-base 10`, es decir, con puertas) y
`m = 0;<cifras>`. Así `log` trabaja dentro de su escala de 12 cifras:
`λ = (z − log m) / log q` (1.378 M de puertas). Las filas con `z > 91` se
descartan porque `L` solo está a `12⁻⁹⁹` de π; la corrida para en `n = 96`.

### 5.2 El resultado (`convergents.out`)

95 convergentes (`n = 1…95`). Contraste con `lambda-oracle.mjs`: diferencia
máxima **4,2·10⁻¹²**.

| `n` | `pₙ/qₙ` (dozenal) | decimal | `λₙ` (dozenal) | `λₙ` |
|--:|---|---|---|--:|
| 1 | `1X/7` | 22/7 | `3;5199` | **3,4293** |
| 2 | `239/8X` | 333/106 | `2;020X` | 2,0144 |
| 3 | `257/95` | 355/113 | `3;250E` | **3,2020** |
| 4 | `50221/171X6` | 103993/33102 | `2;063X` | 2,0439 |
| 20 | — | (antes de `a21 = 84`) | `2;2434` | 2,1964 |
| 32 | — | (antes de `a33 = 99`) | `2;1408` | 2,1115 |
| 78 | — | (antes de `a79 = 161`) | — | 2,0564 |
| 80 | — | | — | 2,0007 (mínimo) |

Por tramos: `n = 1–10`, máximo 3,429 y media 2,313; `11–40`, 2,196 y 2,051;
`41–70`, 2,040 y 2,017; `71–95`, 2,056 y 2,015.

### 5.3 Lectura

- **Todas `λₙ > 2`.** No es un hallazgo: toda convergente cumple
  `|π − p/q| < 1/q²`. Lo que mide el exponente de irracionalidad es cuánto
  *por encima* de 2 quedan, y eso lo deciden los cocientes grandes:
  `λₙ ≈ 2 + log aₙ₊₁ / log qₙ`. Los picos están justo antes de `a4`, `a21`,
  `a33`, `a79`.
- **El exceso decrece** porque `log qₙ` crece linealmente en `n` y los
  cocientes de π, a esta escala, no crecen. Un `aₙ₊₁` del tamaño de
  `qₙ^{5}` haría `λ ≈ 7`; nada parecido aparece. `μ(π) = 2` afirma que el
  exceso tiende a 0; `μ(π) = 7` permitiría excepciones infinitamente raras. Las
  dos son compatibles con la tabla.
- **El máximo es el de 22/7, no el de 355/113.** 355/113 es la aproximación
  famosa porque es buena *para su tamaño de numerador*, pero 22/7 tiene `q = 7`
  y un logaritmo tan pequeño que su exponente queda más alto.

**`PE3`** (λ en `[2; 3,3]` con el máximo en 355/113): **falla**. La cota
inferior se cumple (es un teorema), pero 22/7 da 3,43 y es el máximo. Se
advirtió antes de correr P2 (con un cálculo a mano, no con el instrumento) y no
se cambió la predicción.

---

## 6. Flint–Hills (P4)

### 6.1 Las sumas parciales

`flint-hills-resume.sh` suma `1/(n³ sin² n)` por puertas, modo fijo, escala 12,
con `n` leído en base 10 (`--input-base 10`). La corrida del otro equipo paró
en `n = 246`; esta sesión la llevó a 400 (`flint-hills-400.out`) y luego a 2000
(`flint-hills-2000.out`, que empieza copiando la de 400).

| `n` | dozenal | `S(n)` (dozenal) | decimal | oráculo |
|--:|--:|---|--:|--:|
| 100 | `84` | `4;9802X453X67` | 4,80569368208 | 4,80569368209 |
| 355 | `257` | `25;4X4E0667X78` | 29,4056252662 | 29,4056253575 |
| 400 | `294` | `25;4X551447341` | 29,4059179098 | 29,4059180011 |
| 1000 | `6E4` | `26;2120544447X` | 30,1747900738 | 30,1747901659 |
| 2000 | `11X8` | `26;3841E083X61` | 30,3079630345 | 30,3079631267 |

Error relativo máximo frente al oráculo (`flint-hills-oracle.mjs`):
**3,1·10⁻⁹, en n = 355**, y de ahí se arrastra a todas las filas siguientes. La
causa es la escala: `sin 355 ≈ −3,0·10⁻⁵` ocupa cuatro de las doce cifras
dozenales en ceros y le quedan ~8 significativas. A escala 24 el mismo término
sale `20;7217X7679656821…` frente a `20;7217X734621…` a escala 12. Coste:
1,13·10¹¹ puertas en las 2000 filas, 56,5 M de media por término.

### 6.2 Los picos

Los términos grandes están en los numeradores `p = pₙ` de las convergentes,
donde `|sin p| = |sin(p − qπ)| ≈ 1/(aₙ₊₁ q)`. `flint-hills-spikes.sh 24` (escala 24) los evalúa sueltos; los
12 coinciden con el oráculo con error relativo ≤ 7·10⁻¹⁶:

| `p` | término |
|--:|--:|
| 3 | 1,8598 |
| 22 | 1,1987 |
| 333 | 3,48·10⁻⁴ |
| **355** | **24,598** |
| 103993 | 2,43·10⁻⁶ |
| 104348 | 7,25·10⁻⁶ |
| 208341 | 1,68·10⁻⁶ |
| 312689 | 3,89·10⁻⁶ |
| 833719 | 3,23·10⁻⁷ |
| 1146408 | 1,92·10⁻⁶ |
| 4272943 | 4,24·10⁻⁸ |
| 5419351 | 4,31·10⁻⁶ |

En el pico, `1/(p³ sin² p) ≈ aₙ₊₁² q²/p³ ≈ aₙ₊₁²/(π² p)`: los picos
**decaen como 1/p** salvo que el cociente siguiente sea grande. Para 355,
`a₄ = 292` da `292²/(π²·355) ≈ 24,3` (medido: 24,598); esa sola fila aporta
24,6 de los 30,3 de `S(2000)`. La
pregunta de Flint–Hills es exactamente si aparecen infinitas aproximaciones
con `aₙ₊₁² / pₙ` que no sume, es decir, cocientes del orden de `√pₙ` o
mayores infinitas veces; `μ(π) < 2,5` bastaría para descartarlo (Alekseyev
2011, citado de memoria, no comprobado aquí), y el preprint afirma `μ(π) = 2`.

**`PE5`** (salto de ≈ 24,6 en `n = 355` y ninguno comparable antes de
`n = 103993`): se cumple, y en forma más fuerte. No hay salto comparable ni
siquiera en 103993 (`2,4·10⁻⁶`) ni en ningún pico medido: los cocientes que
siguen a `a₄` son pequeños frente a `√pₙ`. En `n ≤ 2000`, tras 355 vienen
`n = 3` (1,86), `n = 1` (1,41) y `n = 22` (1,20); el mayor con `n > 355` es
`n = 710 = 2·355` (`0;9283₁₂ ≈ 0,77`), el eco de la misma aproximación.

---

## 7. El instrumento

- **Cambio de código para esta investigación** (otro equipo, commit `f126e03`):
  la escala de división en modo fijo pasa a ser `max(24, --scale)`
  (`dz_api::division_scale`). Antes, `|L − p/q|` a escala 100 se habría
  truncado a 24 cifras y P2 no habría sido posible. 276 tests en verde con el
  binario recompilado en esta sesión.
- **El binario del árbol estaba viejo** (2026-09-14, anterior a `f126e03` y a
  `isqrt`). Ninguna medida de esta sesión usa ese binario; hubo que recompilar
  antes de empezar.
- **Costes de referencia** (MVL): π por `4*atan(1)` a escala 12, 24 y 48:
  352,7 M, 1.053,8 M y 4.268,6 M de puertas (otro equipo). Paso de CF con
  ~50 cifras: 901 M. `|L − p/q|` a escala 100: 4.488 M. `λ` a escala 12:
  1.378 M. Término de Flint–Hills a escala 12: 56,5 M de media.
- Ninguna medida de este documento usa una propiedad de la base 12 que no tenga
  cualquier otra base: el instrumento hace de calculadora exacta de precisión
  arbitraria y nada más. Es el resultado esperado para un enunciado (IV).

---

## 8. Lo que no funcionó

- **`cf.sh` no era reanudable** y el otro equipo lo perdió en `a75` (una tarea
  en segundo plano muere a las 2 h). Se reescribió como `cf-resume.sh`; la
  corrida completa no llegó a cortarse, pero ahora podría.
- **`PE3` estaba mal planteada**: confundía "la mejor aproximación conocida"
  con "el mayor exponente local". El exponente premia los denominadores
  pequeños.
- **Escala 12 en Flint–Hills** pierde ~4 cifras en el término de 355 y ese
  error (3·10⁻⁹ relativo) se arrastra a todas las sumas posteriores. No cambia
  ninguna conclusión, pero una corrida limpia debería usar escala 24 en los
  `n` cercanos a numeradores de convergentes.

---

## 9. Lo que no se hizo

- **P3** (subir el tope de `--scale` a 300–400 y calcular más cifras de π): se
  dejó para después. Daría ~300 cocientes certificados y estadística de rachas
  sobre cientos de cifras; no cambiaría ningún veredicto (§2).
- No se verificó la formalización Lean ni se releyó el preprint; §1 es el
  resumen del traspaso.
- No se comparó con las medidas de irracionalidad de otras constantes
  (`e`, con `μ(e) = 2` demostrado y efectivo, sería el control natural:
  cocientes con patrón, `λₙ` explícitos).
- Flint–Hills no se llevó más allá de `n = 2000` término a término; los picos
  sí se evaluaron hasta `p = 5419351`.
- No se buscó en la literatura si la relación `λₙ ≈ 2 + log aₙ₊₁ / log qₙ` y el
  decaimiento `1/(π² p)` de los picos están ya escritos así (es casi seguro que
  sí; son consecuencias directas de la teoría clásica de fracciones continuas).

---

## 10. Predicciones contra resultados

| | predicción (traspaso, antes de P1–P4) | resultado |
|---|---|---|
| PE1 | ninguna medida dice nada sobre `μ(π) = 2` | **se cumple** (por construcción, §2) |
| PE2 | 90–105 cocientes certificados, todos los conocidos | **se cumple**: 104, 0 discrepancias |
| PE3 | `λₙ ∈ [2; 3,3]`, máximo en 355/113 | **falla**: máximo 3,43 en 22/7 |
| PE4 | `max r/k < 0,5` para `k ≥ 10`, sin diferencia entre bases | **se cumple**: 0,154 / 0,087 |
| PE5 | salto de ≈ 24,6 en 355, ninguno comparable antes de 103993 | **se cumple**, más fuerte: ninguno comparable en ningún pico medido |

---

## Correcciones

- `HANDOFF.md` §4 P5 decía "añadir §4.14 a `99-sintesis.md`". Al integrar el
  trabajo de otra máquina, §4.14 y §4.15 pasaron a ser la frontera de la bola L₁
  (`15`) y el paseo D₄/D₆ (`16`); este documento va en §4.16.
- `HANDOFF.md` §3 da las "rachas de `0`/`↋`" como `↋↋` en las cifras 25–26: es
  lo mismo que `k = 24` aquí (24 cifras antes de la racha).

---

## 11. Reproducción

Desde la raíz del repo, con `target/release/dz-cli.exe` compilado de `main`
(`cargo build -p dz-cli --release`):

```bash
D=Investigations/14-pi-exponent
bash $D/cf-resume.sh                                  # P1 -> cf-resume.out (reanudable, ~1 h)
node $D/cf-oracle.mjs $D/cf-resume.out                # contraste
bash $D/convergents.sh > $D/convergents.out           # P2 (~15 min)
node $D/lambda-oracle.mjs $D/convergents.out          # contraste
bash $D/flint-hills-resume.sh 2000 $D/flint-hills-2000.out   # P4 (reanudable, ~40 min)
node $D/flint-hills-oracle.mjs $D/flint-hills-2000.out
bash $D/flint-hills-spikes.sh 24 > $D/flint-hills-spikes-24.out
node $D/spikes-oracle.mjs $D/flint-hills-spikes-24.out
cd $D && node runs.mjs && node runs-base10.mjs        # rachas, base 12 y control en base 10
```

`cf.sh`, `flint-hills.sh` y `cf.out` son del otro equipo y se conservan como
registro; `cf-resume.out` los reemplaza.
