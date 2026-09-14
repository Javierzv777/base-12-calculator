# Hallazgo — Ingrediente 1: cambiar la topología (χ) en vez de la métrica

Primero de una lista ordenada de "ingredientes" candidatos a producir una
geometría que no necesite π, más allá de sustituir la norma (L₁, hexágono,
etc. — ya agotado en `03`–`05`, `07`, `08`). Ingrediente 1: en vez de tocar
la métrica, tocar la **característica de Euler** χ del objeto.

Motivación exacta, de `05` §1.3: el defecto angular total de cualquier
poliedro convexo es `4π` porque `χ=2` para cualquier esfera topológica —
**no** por la norma elegida. La pregunta que abre este documento: si χ deja
de ser 2, ¿desaparece π de la fórmula, o simplemente cambia el múltiplo
entero?

Fecha de apertura: 2026-09-13. Protocolo de `00-plan-investigacion.md`: toda
cifra dozenal sale de `target/release/dz-cli.exe` con el comando al lado;
todo hallazgo se clasifica en casilla (I)/(II)/(III); ningún resultado se
elige por dar el número bonito.

**Recordatorio no negociable** (el mismo de `05`): π es trascendente
(Lindemann–Weierstrass), independiente de la base numeral. Nada de lo que
sigue sugiere que π sea racional. Se investiga si cambiar χ evita
*estructuralmente* que π aparezca en la fórmula del defecto angular total,
o si π sigue apareciendo por otra vía.

---

## 1. El objeto: un toro poliédrico construido, no citado

Para que "χ=0" no sea una afirmación de manual, se construyó un poliedro de
género 1 concreto y se contaron sus vértices, aristas y caras a mano,
exactamente como `05` §1.2 hizo con el octaedro vértice por vértice.

**Construcción**: un anillo de 8 cubos unitarios, dispuestos en una rejilla
3×3 con el cubo central retirado — el "marco de fotos" sugerido en el
enunciado. Cubos en las celdas `(i,j)` con `i,j∈{0,1,2}`, excepto `(1,1)`,
cada uno `[i,i+1]×[j,j+1]×[0,1]`. Es un sólido con un agujero pasante: su
frontera es topológicamente un toro.

### 1.1 Recuento de caras, aristas y vértices

Todas las caras de la frontera son cuadrados unitarios (caras de cubo no
compartidas con otro cubo del sólido):

- **Caras superiores** (`z=1`): una por cada uno de los 8 cubos → 8.
- **Caras inferiores** (`z=0`): igual → 8.
- **Caras laterales expuestas**: recorriendo los 8 cubos y comprobando, para
  cada una de sus 4 caras laterales, si el vecino en esa dirección está
  fuera de la rejilla 3×3 o es la celda hueca `(1,1)` → 16 en total: **12
  exteriores** (perímetro del cuadrado 3×3, `4×3`) y **4 interiores**
  (perímetro del agujero de 1×1, `4×1`).

**F = 8+8+16 = 32.**

Cada cara cuadrada tiene 4 aristas; cada arista la comparten exactamente 2
caras (superficie cerrada, sin borde): **E = 32·4/2 = 64.**

Vértices: por capa (`z=0` o `z=1`) los 16 puntos de la rejilla `4×4` que
delimita el cuadrado 3×3 —incluidos los 4 puntos interiores del agujero,
que siguen siendo esquinas de las caras del anillo— están todos presentes.
**V = 16×2 = 32.**

```
V − E + F = 32 − 64 + 32 = 0
```

```
$ target/release/dz-cli.exe eval "32-64+32" --mode rational
0
```

χ = 0, verificado por conteo directo, no asumido. (Control de consistencia
del método: aplicado a un cubo aislado en vez del anillo, la misma cuenta
da V=8, E=12, F=6, χ=2 — el caso esférico de `05`.)

### 1.2 Defecto angular vértice por vértice — tres clases, no una

A diferencia del octaedro de `05` (un solo tipo de vértice por simetría),
el anillo de cubos tiene **tres clases** de vértice según cuántas caras
cuadradas concurren, y eso es exactamente lo que hace el ejemplo instructivo
en vez de trivial:

| clase | # vértices (total, 2 capas) | caras que concurren | suma de ángulos | defecto `2π − Σ` |
|---|--:|--:|---|---|
| **esquina exterior** (p. ej. `(0,0,0)`) — como una esquina de cubo ordinaria | 8 | 3 (1 tapa + 2 paredes exteriores) | `3·π/2` | **`+π/2`** |
| **punto medio de arista exterior** (p. ej. `(1,0,0)`) — subdivisión de una cara plana, no una esquina real | 16 | 4 (2 tapas + 2 paredes coplanares) | `4·π/2 = 2π` | **`0`** |
| **esquina interior del agujero** (p. ej. `(1,1,0)`) — cóncava, el hueco visto desde dentro | 8 | 5 (3 tapas + 2 paredes interiores) | `5·π/2` | **`−π/2`** |

Cada fila se dedujo contando caras a mano (detalle completo verificable
repitiendo el análisis geométrico de §1.1 vértice por vértice); aquí solo se
reportan los conteos de caras, ya que son enteros y no necesitan el binario.
Lo que sí sale del binario son los valores del defecto:

```
$ target/release/dz-cli.exe eval "2*pi - 3*(pi/2)" --mode fixed --scale 8
~1;6X2404846
$ target/release/dz-cli.exe eval "pi/2" --mode fixed --scale 8
~1;6X2404846
$ target/release/dz-cli.exe eval "2*pi - 4*(pi/2)" --mode fixed --scale 8
~0
$ target/release/dz-cli.exe eval "2*pi - 5*(pi/2)" --mode fixed --scale 8
~-1;6X2404846
$ target/release/dz-cli.exe eval "0 - pi/2" --mode fixed --scale 8
~-1;6X2404846
```

Los tres defectos son exactamente `+π/2`, `0` y `−π/2`, dígito a dígito.

### 1.3 La suma total, calculada, no asumida

```
$ target/release/dz-cli.exe eval "8*(2*pi-3*(pi/2)) + 16*(2*pi-4*(pi/2)) + 8*(2*pi-5*(pi/2))" --mode fixed --scale 8
~0
```

`8·(π/2) + 16·0 + 8·(−π/2) = 0`, coincidiendo con `2πχ = 2π·0 = 0`.

**Control de consistencia con el caso esférico ya conocido**: la misma
cuenta sobre un cubo aislado (8 esquinas, cada una con 3 caras, sin puntos
medios ni concavidades) debe reproducir `4π` (el `χ=2` de `05`):

```
$ target/release/dz-cli.exe eval "8*(2*pi - 3*(pi/2))" --mode fixed --scale 8
~10;6968317
$ target/release/dz-cli.exe eval "4*pi" --mode fixed --scale 8
~10;6968317
```

Coinciden. El mismo generador (defecto de esquina de cubo, `π/2`) que da
`4π` para χ=2 da `0` para χ=0, con el cambio exacto que Gauss–Bonnet
discreto predice: `2πχ`.

---

## 2. Pregunta 1 y 2: ¿desaparece π, o es un cero degenerado sin contenido?

**Respuesta corta: ninguna de las dos versiones ingenuas es correcta.**

No desaparece π del cálculo — aparece en **cada uno** de los 32 vértices,
literalmente como `±π/2` (16 de los 32 vértices llevan `π/2` con signo no
nulo). Y no es "0 = 0" sin contenido: es una cancelación genuina entre 8
términos `+π/2` y 8 términos `−π/2`, verificada sumando las tres clases por
separado (§1.3), no una tautología donde cada sumando ya era cero.

Pero tampoco es "una geometría sin π": π sigue siendo el generador de cada
defecto local. Lo que cambia con χ no es si π aparece, sino **cuántas veces
aparece con cada signo**, y el total es su suma con signo. Para χ=2 (16
esquinas de dos cubos, o 8 de uno, todas del mismo signo) suma a `4π≠0`;
para χ=0 (8 positivas, 8 negativas, más 16 vértices neutros de subdivisión)
suma a `0` porque las contribuciones positivas y negativas son iguales en
número y magnitud — un accidente **de este objeto concreto** (un anillo
simétrico de cubos), no un teorema general de "χ=0 implica que cada término
es cero". Un toro poliédrico asimétrico (por ejemplo, un anillo con cubos de
distinto tamaño) seguiría dando total `0`, pero con defectos locales
distintos de `±π/2` en cada vértice, aunque cancelando igual.

**Veredicto de la pregunta 2**: es un cero real, no degenerado — hay un
cálculo detrás, no una identidad vacía — pero como *cantidad publicable* no
aporta nada que Gauss-Bonnet discreto no dijera ya en `05` §1.3 (`Σdefecto
= 2πχ`, para cualquier χ, incluido 0). Es la misma fórmula, evaluada en otro
entero. No es un hallazgo nuevo: es la verificación numérica, con un objeto
concreto, de un caso particular de un teorema ya citado.

---

## 3. Pregunta 3: el área hiperbólica `4π(g−1)` — ¿reformulación o hallazgo?

Gauss–Bonnet suave: `∬ K dA = 2πχ`. Para una superficie compacta de
curvatura constante `K=−1` (geometría hiperbólica, género `g≥2`, que existe
por el teorema de uniformización), el área queda determinada por pura
topología:

```
Área = −2πχ = −2π(2−2g) = 4π(g−1)
```

```
$ target/release/dz-cli.exe eval "4*pi*(2-1)" --mode fixed --scale 8
~10;6968317
$ target/release/dz-cli.exe eval "0-2*pi*(2-2*2)" --mode fixed --scale 8
~10;6968317
$ target/release/dz-cli.exe eval "4*pi*(3-1)" --mode fixed --scale 8
~21;1714632
$ target/release/dz-cli.exe eval "0-2*pi*(2-2*3)" --mode fixed --scale 8
~21;1714632
```

Las dos formas de la fórmula coinciden dígito a dígito, en dos géneros
distintos, como debían — es la misma ecuación, reescrita.

**¿Es esto un hallazgo?** No, y hay que decirlo sin regatear. Es exactamente
la misma observación de `05` §1.3 (`2πχ` es una constante topológica
multiplicada por un entero, sea cual sea la curvatura interna), aplicada al
caso donde la superficie es tan simétrica (curvatura constante) que la
integral colapsa a `Área × K`, así que la propia área queda despejada como
`−2πχ/K`. No hay contenido geométrico nuevo: es el mismo teorema (el que
`05` ya citó) resuelto para una incógnita distinta. Y la premisa de la
pregunta original —que esto fuera "un múltiplo racional de π" mientras el
caso esférico "depende de una constante universal"— es una lectura
equivocada de la simetría: el caso esférico **también** es `4π/K`
(sustituyendo `χ=2`), con `K=+1/r²` para la esfera de radio `r`; la única
diferencia es que ahí `K` es libre (se puede elegir el radio) mientras que
en género `g≥2` la geometría hiperbólica *fija* `K=−1` por convención de
normalización, no porque el género lo imponga. Elegir `K=−1` en vez de
`K=+1` es la misma clase de elección arbitraria que elegir el radio de la
esfera — no es una propiedad topológica adicional.

Y el `π` está exactamente igual de presente en los dos casos: ambos son
`(constante topológica entera) × π`. Que uno se escriba `4π(g−1)` y el otro
`4πr²` no cambia su relación con π — ambos son múltiplos de π, uno con
coeficiente entero fijo (topológico) y otro con coeficiente real libre
(métrico). **No hay geometría "sin π" en ninguno de los dos.**

**Veredicto: reformulación de Gauss–Bonnet, no hallazgo.** Repite el patrón
que `04-hallazgo-cuerpo-12-y-medida-reticular.md` y `07` ya encontraron con
otras vías: una pregunta que suena nueva ("¿qué pasa con géneros mayores?")
resulta ser la misma ecuación evaluada en otro punto de su dominio.

---

## 4. Pregunta 4: ¿alguna cantidad no trivial, racional pura (sin π), de una familia con χ variable?

Aquí sí hay algo que reportar, aunque de alcance modesto y **no nuevo en su
mecanismo** — ya estaba en `05` §1.4, solo que no se había probado sobre un
objeto de género distinto de 0.

### 4.1 Curvatura combinatoria de Higuchi, aplicada al anillo de cubos

`05` §1.4 definió, sin ángulos, solo contando aristas y lados de cara:

```
κ_comb(v) = 1 − deg(v)/2 + Σ_{f∋v} 1/deg(f)
```

Para el anillo de cubos, `deg(v)` (número de aristas/caras en el vértice)
coincide con el número de caras contadas en §1.2, y todas las caras son
cuadrados (`deg(f)=4`):

| clase de vértice | `deg(v)` | `κ_comb(v) = 1 − deg(v)/2 + deg(v)/4` |
|---|--:|---|
| esquina exterior | 3 | `1 − 3/2 + 3/4` |
| punto medio de arista | 4 | `1 − 4/2 + 4/4` |
| esquina interior (cóncava) | 5 | `1 − 5/2 + 5/4` |

```
$ target/release/dz-cli.exe eval "1-3/2+3*(1/4)" --mode rational
1/4
$ target/release/dz-cli.exe eval "1-4/2+4*(1/4)" --mode rational
0
$ target/release/dz-cli.exe eval "1-5/2+5*(1/4)" --mode rational
-1/4
```

**Esto es lo que la pregunta 4 pedía**: `κ_comb` toma valores `1/4`, `0` y
`−1/4` — racionales **exactos**, sin `~`, sin ningún `π` en la definición ni
en el cálculo, en un objeto de género 1. No son ceros triviales: `1/4` y
`−1/4` son cantidades locales genuinamente distintas de cero en 16 de los 32
vértices (las mismas 16 esquinas, exterior e interior, que llevaban `±π/2`
en la versión angular).

Y coincide exactamente con la versión angular dividida por `2π` —no por
casualidad, sino porque todas las caras son cuadrados regulares: para una
cara regular de `n` lados el ángulo interior es `π−2π/n`, así que
`defecto(v) = 2π·κ_comb(v)` siempre que las caras sean regulares (deducción
directa, no verificada aquí con un caso irregular). La relación exacta entre
las dos columnas es la que ya predecía `05` §1.4-§1.5: `κ_comb` es el mismo
contenido que el defecto angular, con el factor `2π` factorizado fuera y
descartado.

Suma total, en modo racional (exacto, no aproximado):

```
$ target/release/dz-cli.exe eval "8*(1-3/2+3*(1/4)) + 16*(1-4/2+4*(1/4)) + 8*(1-5/2+5*(1/4))" --mode rational
0
```

`Σκ_comb = 0 = χ`, exacto y sin aproximación (`--mode rational`, no `~`),
donde la suma angular necesitaba `--mode fixed` con `~` porque `π` es
irracional. **Esta es la diferencia real y medible**: la versión angular da
`0` como límite de una serie truncada a la escala de trabajo (`~0`, con el
riesgo de que un objeto asimétrico deje residuo de redondeo); la versión
combinatoria da `0` como suma exacta de fracciones diádicas, verificable
dígito a dígito sin aproximación en ningún paso.

### 4.2 Coste medido — la diferencia de circuito, replicada en género 1

```
$ target/release/dz-cli.exe eval "8*(2*pi-3*(pi/2)) + 16*(2*pi-4*(pi/2)) + 8*(2*pi-5*(pi/2))" --mode fixed --scale 8 --gates
gates: ... total=6566284 depth=15575 recon=0
$ target/release/dz-cli.exe eval "8*(1-3/2+3*(1/4)) + 16*(1-4/2+4*(1/4)) + 8*(1-5/2+5*(1/4))" --mode fixed --scale 8 --gates
gates: ... total=2064975 depth=1737 recon=0
```

**3,18× más puertas y 8,97× más profundidad** para la versión angular sobre
el toro completo — reproduce el patrón por-vértice que `05` §1.4 midió sobre
el octaedro (6,4× / 12×; la razón no es idéntica porque aquí se están
sumando 32 términos con multiplicaciones enteras adicionales, no un único
vértice, pero el signo y el orden de magnitud del efecto son los mismos). Y
como en `05`, esta comparación de coste **no es la prueba** del hallazgo —
`pi` sigue facturando 0 puertas por memoización, así que el ahorro medido es
cota inferior de la ventaja real, no la ventaja completa.

### 4.3 Veredicto de la pregunta 4

**Positivo pero de alcance limitado, y no nuevo en su mecanismo.** `κ_comb`
ya era conocida como "racional siempre, para cualquier χ" desde `05` §1.4,
que además ya advertía su precio: es un invariante puramente combinatorio,
ciego a la geometría real del cuerpo (dos objetos con el mismo grafo de
incidencias tienen el mismo `κ_comb` vértice a vértice, aunque uno esté
deformado hasta casi aplanarse). Lo único que este documento añade es la
**verificación concreta** de que, sobre un objeto de género 1 real
(construido, no citado), los valores locales no triviales predichos por la
teoría (`±1/4`) son exactamente los que el binario calcula, exactos y sin
aproximar — algo que `05` no había hecho porque solo trabajó con la esfera
topológica (χ=2). No es una extensión matemática nueva: es la misma fórmula
de `05`, ejercitada sobre un segundo valor de χ, con el mismo resultado
cualitativo (evita π con certeza, paga con ceguera geométrica).

---

## 5. Pregunta 5: ¿conexión genuina con base 12?

**No, y hay que decirlo así de directo.** Se revisaron los números que
produjo la construcción, buscando algo dozenal real, no forzado:

- `V=32`, `E=64`, `F=32` — en dozenal, `28`, `54`, `28`
  (`$ target/release/dz-cli.exe eval "32" --mode fixed` da `28`, etc., por
  simple lectura posicional). Nada en estos valores está motivado por base
  12: `32=2⁵`, `64=2⁶` son consecuencia de que se usaron cubos (potencias de
  2 por la rejilla 4×4 de vértices y 8 cubos), no de haber elegido 12 nada.
- Los valores de `κ_comb` son `±1/4` y `0`. `1/4 = 2⁻²` termina en base 12
  (`0;3`, verificado abajo) — pero termina en base 12 **precisamente porque
  4 solo tiene el factor primo 2**, el mismo hecho de `12=2²·3` que domina
  toda la línea D del plan original y que ya está documentado hasta el
  cansancio (`99-sintesis.md` §4.1-§4.2). No es un hallazgo nuevo: es el
  mismo patrón de siempre, encontrado por tercera o cuarta vez.

  ```
  $ target/release/dz-cli.exe eval "1/4" --mode fixed --scale 8
  0;3
  ```

- Si el anillo se hubiera construido con pentágonos o con una rejilla base 5
  en vez de cuadrados en rejilla base 4/8, `κ_comb` habría dado fracciones
  de quintos, que **no** terminan en base 12 (el caso simétrico contrario ya
  documentado en la línea D). La elección de cubos —y por tanto de
  potencias de 2— es la que hace que el ejemplo termine en base 12, no
  ninguna propiedad de la topología ni del género.

**Veredicto explícito de la casilla (I) intrínseca**: no hay conexión
genuina entre "cambiar χ" y base 12. La única aparición de un patrón
dozenal (`1/4` terminando) es la misma coincidencia aritmética de `12=2²·3`
que atraviesa el resto de la investigación, disparada por la geometría
euclidiana concreta elegida (cubos), no por el género del objeto.

---

## 6. Veredicto global del ingrediente 1

**No hay geometría sin π accesible cambiando χ en vez de la métrica —
Gauss–Bonnet, en sus dos formas (discreta y suave), ata π a `2π` por
definición de "vuelta completa alrededor de un punto", y ese `2π` no
depende de χ: χ solo decide el **múltiplo entero** que lo acompaña,**
incluyendo el múltiplo `0`. Es exactamente la generalización que `05` §1.3
predecía sin comprobarla, ahora comprobada con un objeto de género 1
construido a mano:

| χ | objeto | Σ defecto angular | mecanismo |
|--:|---|---|---|
| 2 | cubo (o cualquier poliedro convexo) | `4π` | 8(ó 6, 12…) defectos, todos del mismo signo |
| 0 | anillo de 8 cubos (toro) | `0`, exacto | 8 defectos `+π/2`, 8 `−π/2`, cancelación genuina |
| `2−2g` (g≥2, suave) | superficie hiperbólica compacta | `4π(g−1)` | reformulación de la misma ecuación, curvatura fijada a `−1` |

El único resultado positivo del documento es de segundo orden: la
**curvatura combinatoria** (`κ_comb`, ya conocida de `05`) sigue evitando π
con certeza cuando χ cambia — da `±1/4` locales y `0` exacto de total, en
vez de `±π/2` y `~0` aproximado — pero paga exactamente el mismo precio que
en `05`: deja de ver la forma real, solo ve el grafo de incidencias. No es
un ingrediente que rescate nada nuevo; es la confirmación de que el
ingrediente que ya funcionaba (contar en vez de medir ángulos) sigue
funcionando igual de bien, y por la misma razón, cuando se cambia de χ=2 a
χ=0.

**Para la lista de ingredientes**: este primero se cierra en negativo sobre
la pregunta que lo motivaba (¿la topología evita π?) y en un "ya lo
sabíamos" sobre el único hilo que produjo algo cuantitativo (`κ_comb`). El
antagonismo de `99-sintesis.md` §1 (L₁ contra base 12) tenía un análogo
aquí, y tampoco aparece: cambiar χ no compite con nada de la línea D, porque
no toca representación en absoluto, solo combinatoria de incidencias.

---

## 7. Lo que no funcionó (obligatorio, no vacío)

- **La hipótesis de partida —"si χ≠2, quizás desaparece π"— no sobrevivió
  al primer cálculo.** π aparece en cada uno de los 32 vértices con signo,
  exactamente como en el caso esférico; lo único que cambia es que la suma
  con signo da 0 en vez de un múltiplo positivo. Se anota como predicción
  registrada y refutada, en el espíritu de `00-plan-investigacion.md` §4.
- **La pregunta 3 (área hiperbólica) se persiguió con la expectativa de que
  fuera un hallazgo nuevo** —el enunciado original la señalaba como "más
  interesante"— y resultó ser la misma ecuación de `05` §1.3 resuelta para
  otra variable. No se encontró ningún sentido en el que `4π(g−1)` sea
  "menos π" que `4πr²`; ambas son producto de una constante topológica
  entera por π, o de una constante métrica libre por π. Se intentó buscar
  una lectura donde la ausencia de un "radio libre" en el caso hiperbólico
  fuera geométricamente significativa, y no lo es: es la misma libertad
  (elegir la curvatura) con otro nombre.
- **No se intentó una demostración general de que todo toro poliédrico
  simétrico da cancelación exacta ±.** Se verificó un objeto (el anillo de
  8 cubos) y se generalizó el argumento cualitativamente (§2), pero no se
  construyó un segundo toro con geometría distinta para confirmar que la
  cancelación no es un artefacto de la simetría cúbica elegida.
- **No se exploraron géneros g≥2 discretos (poliedros con más de un
  agujero) ni superficies no orientables (χ negativo por otra vía, como el
  plano proyectivo o la botella de Klein).** El objeto de este documento
  cubre exactamente χ=0; χ<0 discreto queda sin construir.
- **No se comparó `κ_comb` con otras curvaturas combinatorias de la
  literatura para género≥1** — la misma limitación que `05` §"lo que no se
  hizo" ya declaraba para género 0, y sigue sin resolverse aquí.
- **La comparación de coste en puertas (§4.2) usa `pi` con memoización a
  coste cero** (`99-sintesis.md` §5.1): la ventaja de `κ_comb` medida es
  cota inferior, no la cifra real, exactamente la misma salvedad que ya se
  documentó en `05` y no se repitió por descuido, sino porque sigue siendo
  la verdad del instrumento.
- **No se buscó una cuarta pregunta más allá de las cuatro planteadas** —
  por ejemplo, si existe una noción de "defecto" no angular y no
  combinatorio (una tercera vía) que sea sensible a χ *y* a la forma real
  al mismo tiempo. Queda abierta para un ingrediente posterior de la lista,
  si aplica.
- **Nada de esto se sometió a revisión externa**, siguiendo la práctica ya
  declarada del resto del repositorio (`99-sintesis.md` §7).
