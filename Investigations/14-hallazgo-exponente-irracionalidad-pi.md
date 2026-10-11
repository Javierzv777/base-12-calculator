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
| Fracción continua de π, certificada por intervalo | **104 cocientes** (`a0`–`a103`) desde las 99 cifras dozenales de π del instrumento, **429** desde 399 cifras y **434** (`a0`–`a433`) desde 409 (P3, §4.3), con 0 discrepancias con el oráculo en las tres. La de 409 certifica **`a431 = 10034₁₂ = 20776`**, el cociente más grande de π en sus primeros miles. | — |
| Exponente local `λₙ` de las convergentes | **430 convergentes** (`n = 1…422` con 399 cifras, `420…430` con 409; error ≤ 1,75·10⁻¹² frente al oráculo salvo la última fila, 3,9·10⁻⁹). Todas `> 2`, como debe ser; el máximo sigue siendo **22/7 (3,43)**, no 355/113 (3,20). El exceso medio sobre 2 baja de 0,016 (`n = 41–100`) a 0,0031 (`n = 301–422`), aproximadamente como `1/n`. El cociente gigante solo da **`λ₄₃₀ = 2,0200`** (`2;02↊6↊↊62₁₂`). Es la imagen finita de `limsup λₙ = 2`. | — |
| Rachas de cifras en el desarrollo de π | Lo único ligado a la base: el teorema acota las rachas de `0`/`↋`. En 399 cifras hay 34 rachas de 2 o más (33 esperadas al azar) y una sola de 3 (`↋↋↋` tras la cifra 105); `max r/k = 0,083` (de `0`/`↋`), `0,154` (cualquier cifra), los mismos que con 99. En base 10 (431 cifras), `0,047` y `0,087`. El teorema no muerde a esta escala y las dos bases se comportan igual. | (III), compartida con toda base |
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

**P3 (más cifras).** Con el tope de `--scale` subido (§7),
`dz-cli eval "pi" --mode fixed --scale 400` tarda **686 s** y da **399 cifras
correctas** (`pi-scale400.out`; `pi-digits-oracle.mjs`, Machin en `BigInt`, lo
contrasta cifra a cifra). A escala 200, 89 s y 199 cifras. En esas tres escalas
la regla es la misma: escala `s` → `s − 1` cifras correctas, con la última
perdida por truncado. A escala 410 (`pi-scale410.out`, 741 s) salen las 410
correctas: esta vez el truncado acierta también la última. Aun así §4.3 usa
solo 409, la regla propia del instrumento, para que la certificación no
dependa del oráculo en la última cifra. El tiempo crece como ~`s³` (×3,4 de 100 a 200, ×7,7 de 200
a 400): las series de Machin necesitan `O(s)` términos y cada división
restauradora a escala `s` cuesta `O(s²)`.

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

| | base 12, 99 cifras | base 10, 107 cifras | base 12, 399 cifras | base 10, 431 cifras |
|---|--:|--:|--:|--:|
| rachas de longitud ≥ 2 (esperadas al azar) | 8 (8,2) | 6 (10,6) | 34 (33,2) | 37 (43,0) |
| racha más larga | 2 | 2 | 3 (`↋↋↋` tras la cifra 105) | 3 |
| `max r/k`, `k ≥ 10`, cualquier cifra | 0,154 (`66` tras la cifra 13) | 0,087 (`33` tras la 23) | 0,154 | 0,087 |
| `max r/k`, `k ≥ 10`, cifra `0`/última | 0,083 (`↋↋` tras la 24) | 0,047 (`99` tras la 43) | 0,083 | 0,047 |

Las columnas de base 12 salen del instrumento (`runs.mjs`); las de base 10, del
oráculo (`runs-base10.mjs`), con tantas cifras decimales como información llevan
las dozenales (`log 12 / log 10 ≈ 1,079`). El número esperado de rachas de 2 o
más en `N` cifras aleatorias de base `b` es `(N − 1)/b`, y π queda cerca en las
cuatro columnas. Nada distingue sus cifras de cifras al azar a esta escala, en
ninguna de las dos bases. Con 399 cifras el máximo de `r/k` no cambia: lo siguen
fijando las rachas de las primeras cifras, porque una racha de 3 tras la cifra
105 solo da `r/k = 0,029`. El punto de Feynman (`999999` en la cifra decimal 762)
sigue fuera.

**`PE4`** (`max r/k < 0,5` para `k ≥ 10`, sin diferencia entre bases): se
cumple con margen, con 99 y con 399 cifras; el teorema necesitaría `r/k` cerca
de 1 para morder.

---

## 4. Fracción continua de π, certificada por intervalo (P1, P3)

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
`cf-oracle.mjs` (CF de π desde Machin a 600 cifras decimales): **0 discrepancias** en 104.
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

### 4.3 Con 399 cifras: el mismo intervalo, sin fracciones (P3)

`cf-resume.sh` no escala. Lleva el cociente completo `x_k` como fracción, y
`DzRational::new` corre Euclides en cada operación. Con 99 cifras eso son ~10
min para la corrida entera; con 399, el tamaño de los números ×4 y el número de
pasos ×4 la habrían llevado a decenas de horas. `cf-int.sh` desarrolla la misma
fracción continua del mismo intervalo **solo con enteros**. Sea `T = 12ᴺ` y
`Λ = L·T` (las `N + 1` primeras cifras de π leídas como un entero). Entonces

```
x_k = |q_{k−2}·Λ − p_{k−2}·T| / |q_{k−1}·Λ − p_{k−1}·T|
```

así que cada `a_k` es el cociente de **un** `divrem` entero (con cociente
pequeño, barato), y `p_k`, `q_k` avanzan con `mul` y `add` enteros. `U` es lo
mismo con `Λ + 1`, y mientras las dos fracciones continuas coinciden comparten
`p` y `q`. Como control, con `N = 99` su salida es **idéntica byte a byte** a
`cf-resume.out`, en 33 s frente a ~10 min.

Con `N = 399` (`cf-int.out`, ~7 min): **429 cocientes certificados**
(`a0`–`a428`). En el índice 429 `L` da `2` y `U` da `1` (el verdadero es `1`).
`cf-oracle.mjs`, con Machin a 600 cifras decimales: **0 discrepancias** en 429,
y los 104 primeros son los de `cf-resume.out`. La ley de Lochs predice
`399 ln 12 / (2·1,1866) ≈ 417,8`; salen 429. Cada producto `q·Λ` cuesta ~93 M
de puertas al llegar a `n ≈ 36`, y algo más después.

Cocientes nuevos de más de 100 en el tramo `a104`–`a428`: `a196 = ↊0₁₂ = 120`,
`a222 = ↊7₁₂ = 127` y `a307 = 304₁₂ = 436`. El oráculo dice que los tres
siguientes son `1, 4, 20776`. **`a431 = 20776`**, el cociente más grande de π en
sus primeros miles, quedaba dos índices fuera del rango certificado.

**Con 409 cifras: `a431` certificado.** Antes de gastar la corrida,
`cf-reach.mjs` (oráculo, solo para planificar) simuló el mismo método para cada
`N`. Reproduce los 429 de `N = 399`, y da 430 para `N = 400–403`, 431 para
`404–407`, 432 para `408` (el primero que incluye `a431`) y 434 para `409`. Se
subió el tope a 410 (§7) y se corrió `cf-int.sh` con `DIGITS=409` sobre
`pi-scale410.out` (`cf-int-409.out`, 7,5 min): **434 cocientes** (`a0`–`a433`),
exactamente los simulados, 0 discrepancias con el oráculo, y los 429 primeros
idénticos a los de 399 cifras. En el índice 434 `L` da `1` y `U` da `2`. El
cociente certificado:

```
a429 = 1   a430 = 4   a431 = 10034₁₂ = 20736 + 36 + 4 = 20776   a432 = 1   a433 = 1
```

---

## 5. Exponentes locales de las convergentes (P2, P3)

### 5.1 El método

`convergents.sh` construye `pₙ/qₙ` con la recurrencia
`pₙ = aₙpₙ₋₁ + pₙ₋₂` en aritmética entera exacta y mide
`λₙ = −log|π − pₙ/qₙ| / log qₙ`. Hay dos versiones.

**Primera versión (99 cifras, `convergents.out`).** El error `d = |L − pₙ/qₙ|`
se tomaba en modo fijo a escala 100 (4.488 M de puertas por convergente; era la
parte cara) y se partía como `d = 12^{−z}·m`, donde `z` es el número de ceros
tras `;` (un conteo de posiciones, leído a dozenal con `--input-base 10`, es
decir, con puertas) y `m = 0;<cifras>`. Así `log` trabaja dentro de su escala de
12 cifras: `λ = (z − log m) / log q` (1.378 M de puertas). Las filas con
`z > 91` se descartaban porque `L` solo está a `12⁻⁹⁹` de π; la corrida paró en
`n = 96`.

**Versión actual (P3, sin división).** La división `p/q` a escala 400 no
escala. Con `T = 12ᴺ` y `Λ = L·T` como en §4.3, `E = |q·Λ − p·T|` es un entero
exacto y `|L − p/q| = E/(q·T)`. Si `E` tiene `len` cifras,
`E/T = 12^{−z}·m` con `z = N − len` y `m = 0;<cifras de E>`, y entonces

```
λ = 1 + (z − log m) / log q
```

**Ojo: la columna `z` cambia de significado.** En `convergents.out` (primera
versión) `z` son los ceros de `|L − p/q|`; en `convergents-399.out` son los de
`|q·L − p|`, que tiene unas `len(q)` cifras menos de ceros. El criterio de
fiabilidad cambia con ella. `E/T` arrastra un error de hasta `q·12⁻ᴺ`, así que
la fila es fiable mientras `z + len(q) ≤ N − 8`; con `N = 99` es casi el mismo
corte de antes (para en `n = 95` en vez de `96`). Control a 99 cifras: los 94 `λ`
comunes coinciden con la primera versión hasta la 12.ª cifra dozenal, y la
diferencia con el oráculo es ≤ 2,8·10⁻¹².

Dos variables de entorno añadidas para la corrida de 409 cifras, las dos con
valor por defecto que deja el comportamiento de siempre:

- **`START`** (por defecto 0). Para `n < START` solo avanza la recurrencia de
  `p/q` (dos `eval` enteros baratos por fila) y no se imprime nada; no se
  calculan `E` ni los logaritmos. Así una corrida con más cifras mide solo sus
  filas nuevas, en vez de repetir ~40 min de filas que no cambian.
- **`RELIABLE`** (por defecto 8): el número `R` de cifras fiables que se exige
  en `E`, es decir, el corte `z + len(q) ≤ N − R`. Ocho es mucho más de lo que
  necesita `λ`: con `R` cifras en `E`, el error de `λ` queda por debajo de
  ~`12^{−R} / (len(q)·ln 12)`.

### 5.2 El resultado (`convergents.out`, `convergents-399.out`, `convergents-409*.out`)

Con 99 cifras: 95 convergentes (`n = 1…95`), diferencia máxima con
`lambda-oracle.mjs` **4,2·10⁻¹²**. Con 399 (`DIGITS=399`, ~34 min):
**422 convergentes** (`n = 1…422`; para en `n = 423`), diferencia máxima
**1,75·10⁻¹²**. Las filas comunes dan el mismo `λ`.

Con 409 (`START=420 DIGITS=409`, sobre `cf-int-409.out`, 1,5 min;
`convergents-409.out`): `n = 420…429`, diferencia máxima con el oráculo
**3,8·10⁻¹⁴**, y las tres filas solapadas con la corrida de 399 (420–422)
coinciden con diferencias ≤ 1,75·10⁻¹². Con `R = 8` la corrida para justo en
**`n = 430`**, la convergente previa a `a431`: su error ocupa
`z + len(q) ≈ 404` de las 409 cifras y le quedan unas 5 fiables. Esa fila se midió
aparte con `START=430 RELIABLE=4` (`convergents-409-r4.out`), y está marcada:

| `n` | `aₙ₊₁` | `λₙ` (dozenal) | `λₙ` | oráculo | diferencia |
|--:|--:|---|--:|--:|--:|
| 430 | `a431 = 20776` | `2;02↊6↊↊6233608` | **2,020009** | 2,020009 | 3,9·10⁻⁹ |

La diferencia de 3,9·10⁻⁹ cabe en la cota de `R = 4` (~10⁻⁷). El oráculo
había estimado 2,0200 antes de medir. El cociente más grande de π en sus
primeros miles da un exceso sobre 2 de solo 0,020, la décima parte del que daba
`a21 = 84` en `n = 20` (0,196). `λ − 2 ≈ ln aₙ₊₁ / ln qₙ`: `ln 20776` es solo
2,2 veces `ln 84`, mientras que `ln q₄₃₀ ≈ 497` (`q₄₃₀ ≈ 10²¹⁶`) es unas 22
veces `ln q₂₀`. Es el mecanismo de §5.3 en su caso extremo: un cociente 250
veces mayor llega 410 convergentes tarde y pesa diez veces menos.

| `n` | `pₙ/qₙ` (dozenal) | decimal | `λₙ` (dozenal) | `λₙ` |
|--:|---|---|---|--:|
| 1 | `1X/7` | 22/7 | `3;5199` | **3,4293** |
| 2 | `239/8X` | 333/106 | `2;020X` | 2,0144 |
| 3 | `257/95` | 355/113 | `3;250E` | **3,2020** |
| 4 | `50221/171X6` | 103993/33102 | `2;063X` | 2,0439 |
| 20 | — | (antes de `a21 = 84`) | `2;2434` | 2,1964 |
| 32 | — | (antes de `a33 = 99`) | `2;1408` | 2,1115 |
| 78 | — | (antes de `a79 = 161`) | `2;0816` | 2,0564 |
| 80 | — | | — | 2,0007 (mínimo con 99 cifras) |
| 195 | — | (antes de `a196 = 120`) | — | 2,0208 |
| 221 | — | (antes de `a222 = 127`) | `2;0279` | 2,0184 |
| 306 | — | (antes de `a307 = 436`) | `2;0257` | 2,0171 |
| 338 | — | | — | 2,0004 (mínimo con 399 cifras) |
| 430 | — | (antes de `a431 = 20776`) | `2;02↊6` | 2,0200 (409 cifras, `R = 4`) |

Por tramos, con 399 cifras:

| `n` | máximo de `λ` (en) | media de `λ` | exceso medio sobre 2 |
|--:|--:|--:|--:|
| 1–10 | 3,4293 (1) | 2,3130 | 0,3130 |
| 11–40 | 2,1964 (20) | 2,0509 | 0,0509 |
| 41–100 | 2,0564 (78) | 2,0158 | 0,0158 |
| 101–200 | 2,0272 (117) | 2,0077 | 0,0077 |
| 201–300 | 2,0184 (221) | 2,0044 | 0,0044 |
| 301–422 | 2,0171 (306) | 2,0031 | 0,0031 |

### 5.3 Lectura

- **Todas `λₙ > 2`.** No es un hallazgo: toda convergente cumple
  `|π − p/q| < 1/q²`. Lo que mide el exponente de irracionalidad es cuánto
  *por encima* de 2 quedan, y eso lo deciden los cocientes grandes:
  `λₙ ≈ 2 + log aₙ₊₁ / log qₙ`. Los picos están justo antes de `a4`, `a21`,
  `a33`, `a79`.
- **El exceso decrece** porque `log qₙ` crece linealmente en `n` y los
  cocientes de π, a esta escala, no crecen. Con la constante de Lévy
  (`ln qₙ ≈ 1,187 n`) y la de Khinchin (media de `ln aₙ` ≈ 0,988 para un número
  típico), el exceso medio esperado es del orden de `0,83/n`. Los tramos de la
  tabla dan entre 1,1 y 1,4 veces eso (0,0158 frente a ~0,012 en torno a
  `n = 70`; 0,0031 frente a ~0,0023 en torno a `n = 360`). Mismo orden y misma
  forma `1/n`; la diferencia la ponen los picos, que suben las medias. Esa
  estimación es un cálculo aparte, no una medida. Un `aₙ₊₁` del tamaño de
  `qₙ^{5}` haría `λ ≈ 7`; nada parecido aparece, y el mayor cociente del tramo
  nuevo (`a307 = 436`) solo da 2,017. `μ(π) = 2` afirma que el
  exceso tiende a 0; `μ(π) = 7` permitiría excepciones infinitamente raras. Las
  dos son compatibles con la tabla.
- **El máximo es el de 22/7, no el de 355/113.** 355/113 es la aproximación
  famosa porque es buena *para su tamaño de numerador*, pero 22/7 tiene `q = 7`
  y un logaritmo tan pequeño que su exponente queda más alto.

**`PE3`** (λ en `[2; 3,3]` con el máximo en 355/113): **falla**. La cota
inferior se cumple (es un teorema), pero 22/7 da 3,43 y es el máximo, también en
las 422 convergentes de 399 cifras. Se
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
- **Tope de `--scale` subido de 100 a 410** (P3; primero a 400, luego a 410
  para certificar `a431`): `MAX_SCALE` en `crates/dz-cli/src/main.rs` (antes,
  un `100` literal en `parse_count`). Vale para la línea de órdenes y para el
  `:scale` del REPL, y los mensajes de error dan el rango. Test nuevo en
  `crates/dz-cli/tests/oneshot.rs` (`scale_accepts_up_to_410_and_rejects_more`:
  acepta 410 y rechaza 411 y 0 con código 2). **277 tests en verde**
  (`cargo test --workspace --release`), lint de pureza OK y clippy sin avisos.
  El tope solo existe para que un error al teclear no lance una corrida de
  horas: π a escala 410 ya tarda 12 min. La GUI tiene su propio
  límite de 60 (`app/src-tauri/src/commands.rs`) y no se tocó.
- **El binario del árbol estaba viejo** (2026-09-14, anterior a `f126e03` y a
  `isqrt`). Ninguna medida de esta sesión usa ese binario; hubo que recompilar
  antes de empezar.
- **Costes de referencia** (MVL): π por `4*atan(1)` a escala 12, 24 y 48:
  352,7 M, 1.053,8 M y 4.268,6 M de puertas (otro equipo). `pi` a escala 100,
  200, 400 y 410: 26 s, 89 s, 686 s y 741 s (marca 0 puertas por memoizado). Paso de CF con
  ~50 cifras (`cf-resume.sh`): 901 M. Producto `q·Λ` con 399 cifras
  (`cf-int.sh`, `convergents.sh`): ~93 M. `|L − p/q|` a escala 100: 4.488 M. `λ`
  a escala 12: 1.378 M. Término de Flint–Hills a escala 12: 56,5 M de media.
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
- **La fracción continua en racionales no escala** (P3). `cf-resume.sh` y la
  primera versión de `convergents.sh` hacían que el instrumento normalizara
  fracciones (Euclides en cada `DzRational::new`) o dividiera a escala completa.
  Con 99 cifras se notaba poco; con 399 las dos rutas eran inviables. La salida
  fue reformular las dos medidas en enteros (`cf-int.sh`, §4.3; nuevo
  `convergents.sh`, §5.1): ninguna división grande, un `divrem` de cociente
  pequeño por paso. Es la misma lección de `05-funciones-cientificas.md`: la
  división restauradora domina todo el coste.
- **Una estimación de tiempo mal hecha.** A mitad de la corrida de `cf-int.sh`
  se anunció que le quedaban ~50 min; terminó en ~7. El ritmo de los primeros
  cocientes no servía para extrapolar.
- **Un proceso huérfano mezcló dos salidas.** Se paró una corrida completa de
  `convergents.sh` con 409 cifras para sustituirla por la de `START=420`, pero
  sus `bash` hijos siguieron vivos y escribían en el mismo fichero: la primera
  `convergents-409.out` salió con filas 38–45 intercaladas. Se terminaron esos
  procesos y se repitió la corrida limpia; el fichero actual es el limpio. Para
  cortar una de estas corridas hay que matar el árbol de procesos entero, no
  solo la tarea.
- **`R = 8` corta justo la fila que importaba.** Con 409 cifras el criterio por
  defecto se para en `n = 430`, la convergente previa a `a431`; hizo falta
  `RELIABLE=4` para esa fila (§5.2). El oráculo de planificación
  (`cf-reach.mjs`) no lo avisó porque simula la fracción continua, no el corte
  de `convergents.sh`.

---

## 9. Lo que no se hizo

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
| PE2 | 90–105 cocientes certificados (con 99 cifras), todos los conocidos | **se cumple**: 104, 0 discrepancias (con 399 cifras, 429; con 409, 434) |
| PE3 | `λₙ ∈ [2; 3,3]`, máximo en 355/113 | **falla**: máximo 3,43 en 22/7, también en 430 convergentes |
| PE4 | `max r/k < 0,5` para `k ≥ 10`, sin diferencia entre bases | **se cumple**: 0,154 / 0,087, igual con 99 y con 399 cifras |
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
bash $D/cf-resume.sh                                  # P1 -> cf-resume.out (reanudable, ~10 min)
node $D/cf-oracle.mjs $D/cf-resume.out                # contraste
bash $D/convergents.sh > $D/convergents.out           # P2 (~6 min; versión actual, columna z nueva, §5.1)
node $D/lambda-oracle.mjs $D/convergents.out          # contraste

# P3: 399 cifras
target/release/dz-cli.exe eval "pi" --mode fixed --scale 400 > $D/pi-scale400.out   # ~11 min
node $D/pi-digits-oracle.mjs $D/pi-scale400.out       # 399 cifras correctas
bash $D/cf-int.sh                                     # -> cf-int.out, 429 cocientes (reanudable, ~7 min)
node $D/cf-oracle.mjs $D/cf-int.out
DIGITS=399 PI_FILE=$D/pi-scale400.out bash $D/convergents.sh $D/cf-int.out > $D/convergents-399.out  # ~34 min
node $D/lambda-oracle.mjs $D/convergents-399.out
(cd $D && node runs.mjs pi-scale400.out 399 && node runs-base10.mjs 399)

# P3: 409 cifras, para certificar a431 = 20776
node $D/cf-reach.mjs 399 412                          # planificación (oráculo): cocientes por N
target/release/dz-cli.exe eval "pi" --mode fixed --scale 410 > $D/pi-scale410.out   # ~12 min
node $D/pi-digits-oracle.mjs $D/pi-scale410.out
DIGITS=409 PI_FILE=$D/pi-scale410.out bash $D/cf-int.sh $D/cf-int-409.out $D/cf-int-409.state  # 434, ~7,5 min
node $D/cf-oracle.mjs $D/cf-int-409.out
# START: solo las filas nuevas (para n < START, solo la recurrencia p/q)
START=420 DIGITS=409 PI_FILE=$D/pi-scale410.out bash $D/convergents.sh $D/cf-int-409.out > $D/convergents-409.out   # ~1,5 min
# RELIABLE=4: la fila n = 430, previa a a431, que R = 8 no admite
START=430 RELIABLE=4 DIGITS=409 PI_FILE=$D/pi-scale410.out bash $D/convergents.sh $D/cf-int-409.out > $D/convergents-409-r4.out
node $D/lambda-oracle.mjs $D/convergents-409.out
node $D/lambda-oracle.mjs $D/convergents-409-r4.out

# P4
bash $D/flint-hills-resume.sh 2000 $D/flint-hills-2000.out   # P4 (reanudable, ~40 min)
node $D/flint-hills-oracle.mjs $D/flint-hills-2000.out
bash $D/flint-hills-spikes.sh 24 > $D/flint-hills-spikes-24.out
node $D/spikes-oracle.mjs $D/flint-hills-spikes-24.out
cd $D && node runs.mjs && node runs-base10.mjs        # rachas, base 12 y control en base 10
```

`cf.sh`, `flint-hills.sh` y `cf.out` son del otro equipo y se conservan como
registro; `cf-resume.out` los reemplaza. `convergents.out` lo produjo la primera
versión de `convergents.sh` (columna `z` con el significado antiguo, §5.1);
volver a correr el guion actual da los mismos `λ`, con otra `z`.
`pi-scale400.out` y `pi-scale410.out` llevan delante el tiempo de la corrida
(`(686 s) `, `(741 s) `), igual que `pi-scale100.out`; los guiones lo quitan
solos, y también aceptan la salida sin ese prefijo.
