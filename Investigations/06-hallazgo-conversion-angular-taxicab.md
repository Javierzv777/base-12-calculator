# Hallazgo — La conversión θ₁↔θ₂ y el reingreso de π en la trigonometría taxicab

Hilo nuevo, motivado por una hipótesis unificadora del usuario sobre por qué π
reaparece en `03-hallazgo-busemann-holmes-thompson.md`,
`04-hallazgo-cuerpo-12-y-medida-reticular.md`,
`05-hallazgo-reencuadres-topologia-proyeccion-complejidad.md` y en la sección
de trigonometría taxicab de `01-hallazgos-B-lugares.md`.

Fecha de apertura: 2026-09-13.
Protocolo: `00-plan-investigacion.md`. Toda cifra dozenal sale de
`target/release/dz-cli.exe` con el comando al lado; todo hallazgo se
clasifica en casilla (I)/(II)/(III).

**No negociable**: π es trascendente (Lindemann–Weierstrass), independiente de
la base. No se investiga si π es racional; se rastrea **por dónde entra** en
cada derivación.

---

## 0. Resumen del veredicto (ancla)

**La conjetura del usuario es parcialmente correcta, y la parte incorrecta es
la más importante de reportar.**

- **La conversión θ₁↔θ₂ es real y transcendente.** Derivada explícitamente:
  `θ₂(θ₁) = atan(θ₁/(2−θ₁))` sobre cada lado del rombo (§2.2), verificado con
  el binario (`atan(1) = π/4` exactamente en el punto simétrico `θ₁=1`, y
  `atan(1/3)` sin relación con π en `θ₁=1/2`, ambos §2.2). `K(F)` de esta
  conversión es **3** en L₁ frente a **0** en L₂, donde arco y ángulo son la
  misma cosa por definición (§4) — otra forma concreta de medir lo que
  `01-hallazgos-B-lugares.md` §3.3 ya había probado con `Isom_lin(L₁)=D₄`
  finito.
- **Pero esa conversión no es el mecanismo detrás de ninguno de los tres π
  ya encontrados en esta investigación.** Rastreado explícitamente en el
  álgebra real de cada documento (§3): Busemann y Holmes–Thompson (`03`, `04`)
  traen π por la constante universal de normalización `ω_k`, fija de
  antemano y nunca evaluada como función de la posición sobre la frontera; el
  defecto angular (`05` §1) trae π por el mismo mecanismo (`2π` de la vuelta
  completa en el plano tangente, vía Gauss–Bonnet), no por θ₁↔θ₂; y la serie
  de Fourier de `cos₁`/`sin₁` (`01` §3.6) trae π por la propia estructura del
  análisis armónico (frecuencias en `SO(2)`), independiente de qué
  parametrización de la curva se use.
- **Hay al menos tres mecanismos genuinamente independientes**, no uno:
  (a) la constante `ω_k`/`2π`, compartida por Busemann/HT y el defecto
  angular; (b) la conversión θ₁↔θ₂, real pero no usada por ninguno de los
  tres; (c) la estructura espectral de Fourier. Comparten una raíz última
  («una vuelta mide `2π`», §5.2) pero no son la misma invocación.

**Clasificación**: (I) para el hecho de que θ₁↔θ₂ es intrínsecamente
transcendente (`D₄` finito frente a `SO(2)` continuo, la misma obstrucción
estructural de P3); (I) para el mecanismo `ω_k`/defecto angular (topológico,
no de elección de medida); (I) para el mecanismo de Fourier (estructural del
análisis armónico). Ninguna parte de este hallazgo es (III): no hay ningún
paso en el que la base 12 intervenga.

Detalle completo en §1–§5; limitaciones en §6.

---

## 1. La hipótesis, formalizada

Sobre el rombo unidad `|x|+|y|=1` hay **dos parametrizaciones angulares
distintas**, y no son la misma función:

- **θ₁ — el parámetro de arco taxicab.** Longitud de arco **L₁** recorrida
  desde `(1,0)` en sentido antihorario. Es el parámetro natural de `cos₁`/`sin₁`
  (`01-hallazgos-B-lugares.md` §3.1): lineal por tramos, barato, sin
  trigonometría para definirlo. `θ₁ ∈ [0,8)`, con el octante `[0,1]` cubriendo
  la primera mitad del primer lado del rombo.
- **θ₂ — el ángulo de rotación euclídeo ordinario** de un vector dirección,
  `atan2(y,x)`. Es lo que hace falta cada vez que un cálculo tiene que
  relacionar la **orientación** de un elemento de frontera con algo euclídeo:
  una normal de superficie, una dirección de proyección, un ángulo entre dos
  direcciones.

**La conjetura del usuario, tal como se plantea aquí para ponerla a prueba**:
θ₁ y θ₂ no son la misma función, y convertir entre ellas exige `arctan` — una
función trascendente, con valores especiales como `atan(1) = π/4` cuando θ₁
cae en el punto simétrico de un octante. Y la propuesta es que **este paso de
conversión** —requerido silenciosamente cada vez que un cálculo necesita pasar
de «dónde estás en el círculo taxicab» a «hacia dónde apunta eso en el sentido
ordinario»— es el mecanismo único que alimenta π en Busemann/Holmes–Thompson,
en el defecto angular y en la serie de Fourier de `cos₁`/`sin₁`, en vez de ser
tres coincidencias independientes.

Las secciones 2–3 derivan el mapa y lo rastrean, caso por caso, por las tres
derivaciones ya escritas en este repositorio. El veredicto (§5) es que la
conjetura tiene una parte correcta y verificada (θ₁↔θ₂ es real, transcendente,
y con exactamente los valores especiales predichos) y una parte que no
sobrevive el rastreo: **ninguna de las tres derivaciones examinadas invoca esa
conversión**, ni implícita ni explícitamente. Hay un mecanismo común más
profundo del que tanto θ₁↔θ₂ como los otros tres son manifestaciones — pero no
son la misma manifestación, y confundirlas sería exactamente el tipo de
unificación forzada que el protocolo pide evitar.

---

## 2. El mapa θ₁↔θ₂, derivado explícitamente

### 2.1 Un lado basta; el resto sale de `D₄`

Por la simetría de orden 8 del rombo (`01-hallazgos-B-lugares.md` §1.1, §3.1),
basta derivar el mapa en el primer lado — de `(1,0)` a `(0,1)`, `θ₁ ∈ [0,2]` —
y extender por las 4 reflexiones y 4 rotaciones de cuarto de vuelta de `D₄`.
En ese lado el punto es `(1−t, t)` con `t ∈ [0,1]`, y la longitud de arco L₁
desde `(1,0)` es `t+t = 2t`, así que

```
θ₁ = 2t  ⟺  t = θ₁/2
```

### 2.2 La fórmula cerrada

```
θ₂(θ₁) = atan2(t, 1−t) = atan( t / (1−t) ) = atan( θ₁ / (2−θ₁) ),   θ₁ ∈ [0,2)
```

con `θ₂(2) = π/2` por continuidad (el límite `atan(θ₁/(2−θ₁))` cuando
`θ₁ → 2⁻` es `atan(+∞) = π/2`, y ahí `x=0`, así que `atan2` da exactamente
`π/2` sin ambigüedad). Verificación de los dos extremos y del punto simétrico
que el enunciado señala como caso de prueba (`θ₁ = 1`, la mitad del lado, el
límite del octante):

```bash
target/release/dz-cli.exe eval "atan(1/(2-1))" --mode fixed --scale 8
# ~0;95120242
target/release/dz-cli.exe eval "pi/4" --mode fixed --scale 8
# ~0;951202423
```

Coinciden dígito a dígito (hasta el truncamiento): `θ₂(1) = atan(1) = π/4`,
exactamente el valor que el enunciado de la tarea anticipaba para el límite del
octante. **Ojo con la lectura geométrica**: `θ₁=1` no es «1/8 de la vuelta»
medido en el sentido ingenuo de `1` sobre `8` de longitud total — sí lo es, y
además cae justo en el punto de la diagonal `x=y`, que es el eje de simetría
del propio octante bajo `D₄`. Las dos cosas coinciden porque el eje de simetría
de un lado del rombo pasa exactamente por su punto medio.

**Contraste con un punto sin ninguna simetría**, para confirmar que `π/4` es la
excepción y no la regla: en `θ₁ = 1/2` (un cuarto del lado, sin eje de simetría
que lo distinga),

```bash
target/release/dz-cli.exe eval "atan((1/2)/(2-1/2))" --mode fixed --scale 8
# ~0;3X3E9X01
target/release/dz-cli.exe eval "atan(1/3)" --mode fixed --scale 8
# ~0;3X3E9X01
```

`θ₂(1/2) = atan(1/3)`, coincidiendo con la fórmula (`(1/2)/(2−1/2) = 1/3`) y
sin ninguna relación conocida con una fracción racional de π (es la situación
típica del teorema de Niven: los únicos ángulos racionales en grados con
tangente racional son `0°, 45°, 90°`, y `atan(1/3)` no es uno de ellos). **La
mayoría de los valores de θ₂ no son fracciones bonitas de π**; `π/4` en
`θ₁ = 1` es la excepción que produce la simetría, no la norma.

### 2.3 ¿Es transcendente el mapa? Sí, y por qué exactamente

`θ₂(θ₁) = atan(θ₁/(2−θ₁))` es composición de una función racional (algebraica
por definición) con `atan`, que es trascendente sobre `ℚ(θ₁)` salvo en un
conjunto de medida nula de entradas (el mismo hecho detrás del teorema de
Niven). Así que **el mapa entero es una función trascendente de `θ₁`**, no un
cambio de escala ni una función algebraica disfrazada. Esto confirma la mitad
correcta de la conjetura del enunciado: la conversión existe, es real, y es
transcendente — no una curiosidad ocasional, sino la naturaleza genérica del
mapa.

### 2.4 La distinción que hay que hacer antes de rastrear nada: ángulo radial frente a ángulo de giro

Aquí aparece un matiz que no estaba en el enunciado de la tarea y que resulta
decisivo para la sección 3: **`θ₂` tiene dos parientes que se parecen pero no
son el mismo objeto**, y solo uno de ellos es el mapa de §2.2.

- **Ángulo radial** — el que se ha derivado: `atan2(y,x)` del punto de la
  frontera visto desde el origen. Es **continuo** en toda la vuelta (la
  posición se mueve sin saltos, así que su dirección desde el origen tampoco
  salta), y es exactamente el `θ₂` que hace falta para escribir
  `cos₁θ₁ = (radio euclídeo)·cos θ₂` — la relación entre la parametrización de
  `cos₁`/`sin₁` y las funciones trigonométricas ordinarias.
- **Ángulo de giro (tangente)** — la dirección de la recta tangente a la
  frontera, la que interviene en el teorema de la rotación total (Umlaufsatz)
  y en cualquier curvatura tipo Gauss–Bonnet. Para el rombo, que es un polígono,
  este ángulo es **constante en el interior de cada lado y salta discontinuamente
  `π/2` en cada uno de los 4 vértices** — es la construcción de exactamente el
  defecto angular de §3.2, no una función continua de `θ₁`.

Las dos integran a `2π` sobre la vuelta completa (una por ser el ángulo
recorrido por un punto que rodea el origen una vez; la otra por el teorema de
la rotación total de una curva convexa cerrada), pero son **objetos distintos**:
uno es una función continua y no lineal de `θ₁` (transcendente, salvo en
puntos aislados); el otro es una función escalonada con 4 saltos de `π/2` y
constante en el resto. Confundirlos sería el primer error al rastrear el
mecanismo en §3.2, así que se deja fijado aquí antes de seguir.

**Verificación adicional, de la derivada del ángulo radial** (útil en §3.2 para
contrastar): `dθ₂/dθ₁ = 2/[(2−θ₁)²+θ₁²]`, calculada por diferenciación directa
de §2.2. En los tres puntos notables del lado:

```bash
target/release/dz-cli.exe eval "2/((2-0)^2+0^2)" --mode fixed --gates
# 0;6
target/release/dz-cli.exe eval "2/((2-1)^2+1^2)" --mode fixed --gates
# 1
target/release/dz-cli.exe eval "2/((2-2)^2+2^2)" --mode fixed --gates
# 0;6
```

`1/2, 1, 1/2` — máxima «densidad de giro radial» exactamente en el punto
simétrico `θ₁=1`, mínima en los extremos del lado (los vértices del rombo,
donde el radio euclídeo es más corto para una misma longitud de arco). Esta
densidad integra a `π/2` sobre el lado entero (el cuarto de vuelta que le
corresponde), pero **no tiene saltos**: es continua en todo punto, a diferencia
del ángulo de giro. Se deja como comprobación de consistencia interna del mapa,
no como parte del mecanismo de ningún documento anterior — eso se examina en §3.

---

## 3. Rastreo caso por caso

### 3.1 Busemann / Holmes–Thompson (docs 03, 04)

**No. La conversión θ₁↔θ₂ no aparece en ninguna de las dos derivaciones, ni
implícita ni explícitamente.** Hace falta revisar el álgebra real, no solo
concluir por analogía, y al hacerlo la razón es limpia.

Repasando `03-hallazgo-busemann-holmes-thompson.md` §1–§2: la construcción
entera trabaja en **coordenadas cartesianas** del plano `E₀` (las `(s,t)` o
`(a,b)` de §2.1 de ese documento). La sección primal `B₁∩E₀` se calcula
integrando una desigualdad de valores absolutos en `(a,b)` — nunca se
parametriza esa sección por un ángulo, taxicab ni euclídeo. El polar (para
Holmes–Thompson) se obtiene con la relación algebraica «recta tangente ↦
vértice dual» (`ax+by=c ↦ (a/c,b/c)`, citado en el propio doc 03 y reusado en
el 04) — de nuevo, álgebra lineal sobre coordenadas, sin ningún `atan` de
por medio. Lo mismo en `04-hallazgo-cuerpo-12-y-medida-reticular.md` §1.4 para
el dodecaedro rómbico: la sección y su dual se calculan integrando
`|s|≤1/√2, |t|≤1−|s|/√2` — otra vez coordenadas, cero conversión angular.

**De dónde viene realmente su π, entonces.** Las dos densidades se definen
como

```
θ_Bus(E) = ω_k / vol_k(B∩E₀)         θ_HT(E) = vol_k((B∩E₀)°) / ω_k
```

y `π` entra **exclusivamente** a través de `ω_k` (`ω_2 = π`, el área del
disco euclídeo unidad de referencia, doc 03 §1.1). `ω_k` es una **constante
universal de normalización**, fijada de antemano por la condición de
consistencia con Hausdorff en el caso euclídeo (doc 03 §1.2, §1.3) — no
depende en absoluto de qué cuerpo se esté midiendo, ni de cómo se parametrice
su frontera. Es literalmente el mismo número para el octaedro, para el
dodecaedro rómbico, o para cualquier otro cuerpo normado en `R³` de dimensión
de sección `k=2`: se recalcula la sección y su dual, pero `ω_2` no cambia
nunca.

**El mecanismo, nombrado con precisión**: `ω_k` sí trae `π` de la misma raíz
última que la conversión θ₁↔θ₂ (§2.3 lo dice: `π/2` es la integral de la
densidad de giro radial sobre un cuarto de vuelta) — en el sentido de que
ambos, al final, remontan al hecho de que «una vuelta completa mide `2π`».
Pero **la vía por la que ese hecho entra en Busemann/HT no pasa por evaluar la
conversión θ₁↔θ₂ para ningún punto del cuerpo que se esté midiendo**: es una
constante externa, fijada una sola vez para siempre, no una función que se
evalúe sobre la frontera concreta de este octaedro o de este dodecaedro
rómbico. Es la diferencia entre «usar `π` como constante de conversión
metros↔pies, fijada de antemano» y «calcular `arctan` de algo que depende de
la forma del objeto que se mide» — la primera es lo que hacen Busemann/HT, la
segunda es lo que haría falta para que §2.2 fuera el mecanismo.

**Verificación negativa explícita, no solo ausencia de mención.** Si la
conversión θ₁↔θ₂ fuera necesaria en algún paso, el resultado de Busemann/HT
dependería de *dónde* en la frontera se sitúa cada cara respecto al
parámetro de arco taxicab — pero θ_Bus y θ_HT dan la **misma** densidad para
las 8 caras del octaedro (doc 03 §1.4, §2.4: «una sola densidad calculada en
una cara vale para las 8») y para las 12 del dodecaedro rómbico (doc 04 §1.4:
mismo argumento), precisamente porque el cálculo nunca consulta la posición
de la cara en ningún parámetro de arco — solo su sección euclídea, que es
invariante por la simetría del cuerpo. Si dependiera de θ₁, distintas caras
en distintas posiciones del recorrido darían densidades distintas, y no es lo
que se observa ni lo que predice el álgebra de esos documentos.

**Veredicto de esta subsección: mecanismo diferente.** El π de Busemann/HT
viene de la constante de normalización `ω_k`, no de una evaluación de la
conversión θ₁↔θ₂. Comparten una raíz común más profunda (§5), pero no son el
mismo paso computacional.

### 3.2 Defecto angular / Gauss–Bonnet (doc 05 §1)

**Tampoco. Y aquí la razón es aún más directa: el defecto angular no
parametriza el rombo L₁ en absoluto — trabaja con los ángulos interiores de
las caras del octaedro 3D, medidos con el producto interno euclídeo
ordinario de `R³`.**

Repasando `05-hallazgo-reencuadres-topologia-proyeccion-complejidad.md` §1.2:
el defecto en un vértice es `2π` menos la suma de los ángulos interiores de
las 4 caras triangulares que concurren ahí, y cada ángulo interior de un
triángulo equilátero es `π/3` — un hecho de trigonometría plana ordinaria
(`arccos` de un producto escalar entre dos aristas del triángulo, o
simplemente la propiedad de que un triángulo equilátero tiene sus tres
ángulos iguales y suman `π`). **En ningún punto de esa derivación aparece el
rombo `|x|+|y|=1`, ni un parámetro de arco taxicab, ni la conversión θ₁↔θ₂.**
Las caras del octaedro son triángulos en `R³`; sus ángulos internos se miden
con geometría euclídea de `R³` sin ninguna referencia al hecho de que el
cuerpo completo es la bola de una norma L₁.

Y §2.4 de este mismo documento (06) ya distinguió el objeto correcto para
comparar: el defecto angular usa el **ángulo de giro** (el que salta en los
vértices de un politopo, la construcción discreta del teorema de la rotación
total), no el **ángulo radial** θ₂ de §2.2. Son parientes conceptuales — los
dos, en el límite adecuado, integran a `2π` sobre una vuelta completa — pero
el defecto angular del octaedro **nunca evalúa** la función `atan(θ₁/(2−θ₁))`
en ningún punto: es una suma finita de ángulos interiores de caras 3D, un
objeto combinatorio-geométrico completamente distinto.

**De dónde viene su π, entonces (repitiendo con precisión lo que doc 05 §1.3
ya explicó, para que quede en la misma cadena de razonamiento que este
documento):** el `2π` de cada defecto es la medida de una vuelta completa **en
el plano tangente local** de cada vértice — una constante de la topología del
disco (o, equivalentemente, del grupo `SO(2)`), no de la norma del espacio
ambiente. Sumado sobre los vértices vía Gauss–Bonnet discreto, da `2πχ`. Es
exactamente el mismo tipo de constante universal que `ω_k` en §3.1 — la misma
raíz última («una vuelta mide `2π`») — pero, otra vez, **entra como constante
fija multiplicada por un entero topológico (`χ`), no como el resultado de
evaluar una conversión angular específica de la forma L₁.**

**Veredicto de esta subsección: mecanismo independiente, y de hecho el mismo
mecanismo que §3.1 (la constante `2π`/`ω_k`), no el de §2.** El propio doc 05
§1.5 ya lo había notado sin nombrar θ₁↔θ₂: «`ω_k` hereda el mismo `2π` de la
vuelta completa que aparece en Gauss–Bonnet». Este documento confirma esa
observación por un camino distinto (mirando explícitamente si la conversión
θ₁↔θ₂ interviene en el álgebra, y comprobando que no) y aporta la precisión
adicional de que el defecto angular usa el ángulo de giro discreto, ni
siquiera el pariente continuo de θ₂.

### 3.3 Serie de Fourier de cos₁/sin₁ (doc 01 §3.6)

**Tampoco, y por una tercera razón, distinta de las dos anteriores.**

`01-hallazgos-B-lugares.md` §3.6 escribe

```
sin₁(θ) = (8/π²) Σ_{k impar} (−1)^{(k−1)/2} k⁻² sin(kπθ/4)
```

Los `π` que aparecen —tanto en el argumento `kπθ/4` como en el coeficiente
`8/π²`— son **estructurales de qué es una serie de Fourier**, no de la
geometría del rombo. Por definición, la serie de Fourier de una función
periódica de período `T` se escribe en la base `{sin(2πnθ/T), cos(2πnθ/T)}`
— aquí `T=8`, y `2π·n/8 = nπ/4` reproduce exactamente el argumento observado,
sin que la forma concreta de `sin₁` (triangular en vez de, por ejemplo,
trapezoidal) tenga nada que ver con ese `π` del argumento. **Cualquier**
función periódica de período 8 —taxicab, euclídea o arbitraria— se
descompone en esa misma base, con el mismo `π` en el argumento. Y el `π²`
del coeficiente sale de integrar la propia `sin₁` (una rampa lineal a
trozos) contra esas funciones base — cálculo de Fourier ordinario, que nunca
necesita preguntarse «qué dirección euclídea corresponde a este `θ₁`».

Dicho de otro modo: **el `π` de Fourier no entra por ninguna conversión
angular de la forma que se esté analizando — entra porque el análisis de
Fourier está construido, por definición, sobre el grupo del círculo
euclídeo `SO(2)`, cuyas frecuencias naturales son múltiplos de `2π/T`.** Esto
es lo que ya insinuaba (sin demostrarlo) el cierre de `01-hallazgos-
B-lugares.md` §3.6: «el `π` que la métrica L₁ expulsó de la geometría vuelve
por la puerta del análisis» — y ahora se puede decir con precisión **por qué
puerta**: no la de θ₁↔θ₂, sino la de que Fourier necesita un grupo
topológico con estructura de círculo para definir sus frecuencias, y ese
grupo es intrínsecamente euclídeo (`ℝ/2πℤ`), independientemente de qué
norma se use para parametrizar la curva que se está desarrollando en serie.

**Comprobación de que esto no depende de θ₁↔θ₂**: si se cambiara la
parametrización del rombo por longitud de arco euclídea en vez de taxicab
(es decir, se desarrollara `sin₁` en función de θ₂ en vez de θ₁), la serie de
Fourier resultante tendría **otro período** (la longitud euclídea del rombo,
`4√2`, no `8`) y otros coeficientes, pero seguiría siendo una serie de
Fourier con `π` en el argumento — el `π` de Fourier no desaparece ni cambia
de naturaleza al reparametrizar, porque no depende de qué parametrización
del rombo se use, solo de que exista *alguna* periodicidad a desarrollar.

**Veredicto de esta subsección: mecanismo independiente de los otros dos.**
Ni la constante `ω_k`/defecto angular (§3.1–§3.2) ni la conversión θ₁↔θ₂
(§2) son responsables de este `π`; es un tercer mecanismo, más elemental de
lo que parecía: la propia maquinaria del análisis armónico presupone un
círculo euclídeo como dominio de frecuencias.

### 3.2 Defecto angular / Gauss–Bonnet (doc 05 §1)

TODO.

### 3.3 Serie de Fourier de cos₁/sin₁ (doc 01 §3.6)

TODO.

---

## 4. K(F) del propio mapa de conversión

El objeto que hay que comparar aquí no son dos fórmulas de la misma cantidad
(como en el resto del informe), sino **el coste de la propia conversión entre
parametrizaciones**, en L₁ frente a L₂. Es una extensión razonable de `K(F)`
a un caso que el plan (`00-plan-investigacion.md` §2.1) no anticipó
literalmente — no hay una categoría para «aplicación de una función
trascendente no elemental como `atan`» en la lista `{+,−,×,÷,^,|·|,max,min}` —
así que se cuenta explícitamente aquí, siguiendo el mismo criterio ad hoc que
`05-hallazgo-...` §3.4 ya usó para extender `K(F)` a ecuaciones de frontera:
una aplicación de función unaria cuenta como una operación.

**Caso L₂ — la identidad trivial.** En el círculo euclídeo, el parámetro de
arco (en radianes) **es**, por definición, el ángulo de rotación: `θ₂ = θ₁`.
No hace falta ninguna conversión porque son la misma cosa. `K = 0`
(0 raíces, 0 irracionales, 0 operaciones, 0 periódicos).

**Caso L₁ — la conversión de §2.2.** `θ₂(θ₁) = atan(θ₁/(2−θ₁))`:

| | N_raíces | N_irrac. | N_oper. | N_period. | K |
|---|--:|--:|--:|--:|--:|
| L₂: `θ₂ = θ₁` | 0 | 0 | 0 | 0 | **0** |
| L₁: `θ₂ = atan(θ₁/(2−θ₁))` | 0 | 0 | 3 (resta, división, `atan`) | 0 | **3** |

`K` sube de 0 a 3. **Es el resultado opuesto al que la conjunción H₁ predice
por defecto**, y conviene decirlo con la misma franqueza que el resto de la
serie: convertir entre el parámetro natural de L₁ y la dirección euclídea
ordinaria es *más caro simbólicamente* que en L₂, donde no hay nada que
convertir. Esta es, en sí misma, una forma concreta y medible de decir lo
mismo que `01-hallazgos-B-lugares.md` §3.3 demostró de otro modo (que
`Isom_lin(L₁) = D₄` es finito, orden 8, mientras `Isom_lin(L₂) = SO(2)` es
continuo): en L₂ la rotación por arco **es** la rotación por ángulo, así que
no hay conversión que escribir; en L₁ moverse una unidad de arco taxicab y
girar una unidad de ángulo euclídeo son dos operaciones distintas que solo
coinciden en los 4 vértices del rombo, y relacionarlas exige el `atan` de §2.

No se calcula ningún coste en puertas para este `K(F)` porque el objeto
comparado es una *fórmula simbólica*, no una expresión numérica evaluable
sobre operandos concretos — el mismo motivo por el que `05` §3.2/§3.3 tampoco
adjuntó `--gates` a sus tablas de `K` sobre ecuaciones de frontera.

---

## 5. Catálogo de mecanismos y veredicto

### 5.1 Cuatro mecanismos, no uno

| # | mecanismo | dónde aparece | naturaleza | ¿usa θ₁↔θ₂? |
|---|---|---|---|---|
| (a) | constante universal `ω_k` / `2π` de la vuelta completa | Busemann, Holmes–Thompson (§3.1); defecto angular (§3.2) | constante fija, independiente del cuerpo o de la parametrización | **no** |
| (b) | conversión θ₁↔θ₂ (`atan(θ₁/(2−θ₁))`) | ninguna de las derivaciones examinadas; sí es intrínseca a la propia trigonometría taxicab (línea B, §1.4 del doc 01, y §2 de este documento) | función específica de la forma del rombo, evaluada punto a punto | — (es el propio mecanismo) |
| (c) | estructura espectral de Fourier (`SO(2)` como dominio de frecuencias) | serie de Fourier de `cos₁`/`sin₁` (§3.3) | propiedad del análisis armónico en sí, independiente de la curva desarrollada | **no** |
| (d) | trigonometría algebraica de la cara (p. ej. `cos 60° = 1/2`) | el `√3` de la superficie de Hausdorff del octaedro (`01-hallazgos-A-constantes.md` §7.1, citado en `03` §4.1) | algebraico, ni siquiera trascendente — se incluye por contraste | **no**, y ni siquiera trae π |

El mecanismo (d) se incluye a propósito para que el catálogo no dé la
impresión de que «todo en esta investigación remonta a π»: la parte
algebraica de la superficie del octaedro (`√3`) no tiene nada que ver con
ninguno de los otros tres; viene de la razón entre lados de un triángulo
equilátero, un hecho puramente algebraico sin relación con vueltas completas
ni con círculos.

### 5.2 ¿Hay una raíz común más profunda?

Sí, para (a) y (b) — y aquí conviene ser preciso sobre qué tipo de parentesco
es, porque no es identidad de mecanismo, es identidad de **origen último**.
Tanto `ω_2 = π` (la mitad de la vuelta completa, ver §3.1) como la integral de
la densidad de giro radial de §2.4 (`∫dθ₂ = π/2` sobre un cuarto de vuelta)
remontan al mismo hecho primitivo: **una vuelta completa mide `2π` radianes**,
una convención (o, más profundamente, un hecho sobre la medida del grupo
`SO(2)`) que no depende de ninguna norma. Pero de ahí a «Busemann/HT/defecto
angular calculan θ₁↔θ₂» hay un salto que el álgebra real no sostiene (§3.1,
§3.2): (a) usa esa constante **fija y externa**, calculada una sola vez para
`SO(2)` en abstracto; (b) evalúa una **función que depende del punto concreto
del rombo**. Son dos formas distintas de invocar la misma convención
ancestral, no la misma invocación repetida tres veces. El mecanismo (c)
comparte la misma raíz de forma aún más indirecta (Fourier también presupone
`SO(2)`, pero como *dominio de frecuencias*, no como constante de
normalización de un volumen ni como conversión de un punto).

### 5.3 Veredicto directo, en los términos del encargo

- **¿Es transcendente la conversión θ₁↔θ₂?** Sí (§2.3), y con exactamente el
  patrón de valores especiales predicho: racional-π solo en los puntos fijos
  de la simetría `D₄` (`θ₁=1 ⟹ θ₂=π/4`, verificado en §2.2), y sin relación
  conocida con π en el resto (`θ₁=1/2 ⟹ θ₂=atan(1/3)`, §2.2).
- **¿Explica el π de Busemann/Holmes–Thompson?** No. Ese π entra por `ω_k`, una
  constante de normalización fija que nunca se evalúa como función de la
  posición sobre la frontera del cuerpo (§3.1).
- **¿Explica el π del defecto angular, o es independiente?** Es independiente,
  y de hecho es el **mismo** mecanismo que Busemann/HT (a), no el de θ₁↔θ₂:
  ambos remontan a la constante `2π` de la vuelta completa en el plano
  tangente, no a una conversión evaluada sobre el rombo L₁ (§3.2).
- **¿Y el π² de Fourier?** Tampoco: es un tercer mecanismo, la propia
  estructura del análisis armónico, presente para cualquier función periódica
  con cualquier parametrización (§3.3).

**Veredicto global: (b), uno entre varios mecanismos genuinamente
independientes — no el eslabón perdido.** La hipótesis del usuario identificó
correctamente un objeto real, transcendente, con la estructura de valores
especiales predicha, y hizo un trabajo legítimo al señalar que **debería**
existir un candidato de este tipo dado que la trigonometría taxicab necesita,
en algún momento, relacionar posición-por-arco con dirección euclídea. Pero al
rastrear las tres derivaciones concretas que motivaron la pregunta, ninguna
resulta usarlo: las tres evitan la conversión θ₁↔θ₂ por la misma razón
estructural —trabajan en coordenadas cartesianas o con ángulos internos de
cara/vértice medidos directamente en el espacio ambiente, nunca parametrizando
por arco taxicab— y sus π's remontan, en cambio, a (a) la constante universal
`ω_k`/`2π`, compartida entre Busemann, Holmes–Thompson y el defecto angular, y
(c) la estructura espectral de Fourier, un mecanismo propio y no compartido con
los otros dos. **Son al menos tres mecanismos, no dos y no uno**, y θ₁↔θ₂ es un
cuarto objeto real que vive en la geometría de la línea B pero que esta
investigación no encuentra operando en la línea A/reencuadres.

Esto es exactamente el tipo de resultado que el protocolo prefiere a una
unificación limpia: la conjetura tenía una mitad verificable y cierta (la
conversión existe, es transcendente, tiene los valores especiales previstos)
y una mitad que no sobrevive el escrutinio del álgebra real de los documentos
citados (no es su mecanismo). Reportar las dos mitades por separado es más
valioso que forzar el ajuste.

---

## 6. Lo que no funcionó / limitaciones

Sección obligatoria por el protocolo. No vacía.

- **No se demostró negativamente para *toda* derivación posible de área
  normada o curvatura, solo para las tres concretas ya escritas en este
  repositorio.** Es posible imaginar una construcción de área normada
  (distinta de Busemann y Holmes–Thompson) que sí necesitara parametrizar la
  frontera por arco taxicab en algún paso intermedio y por tanto sí invocara
  θ₁↔θ₂ — no se buscó ni se descartó una construcción así; el rastreo fue
  fiel a lo que `03`, `04` y `05` realmente escriben, no a todo lo que
  cualquier definición alternativa de área o curvatura *podría* escribir.
- **La distinción ángulo radial / ángulo de giro (§2.4) es correcta pero no se
  formalizó más allá de lo necesario para separar mecanismos.** No se derivó
  una fórmula cerrada general del ángulo de giro como función de θ₁ (sería
  una función escalonada trivial de definir, pero no se hizo explícitamente
  aquí porque no hacía falta para el veredicto de §3.2).
- **No se verificó si mecanismo (b) — la conversión θ₁↔θ₂ — aparece en algún
  otro documento de la investigación no revisado en este hilo** (por ejemplo,
  si existiera un cálculo futuro de curvatura L₁ vía longitud de arco taxicab
  en vez de vía ángulos internos de cara). Se limitó explícitamente el
  rastreo a los tres documentos que motivaron la pregunta (`03`, `04`, `05`)
  y a la propia sección de trigonometría de `01`.
- **El teorema de Niven se citó, no se demostró**, para justificar que
  `atan(1/3)` no es una fracción racional de π; es un resultado estándar
  (los únicos ángulos racionales de grados con seno/coseno/tangente racional
  son `0°, 30°, 45°, 60°, 90°` y sus reflejos, y `atan(1/3)` no corresponde a
  ninguno), no una prueba original de este documento.
- **La extensión de `K(F)` a §4 (contar `atan` como una operación) es una
  convención ad hoc**, igual que lo fue la extensión de `05` §3.4 a
  ecuaciones de frontera — el plan (`00-plan-investigacion.md` §2.1) no
  anticipó funciones trascendentes no elementales en su lista de operadores,
  y otra persona podría contar distinto (por ejemplo, dando a `atan` el mismo
  peso que a un radical, no a una operación binaria simple).
- **No se comprobó si mecanismo (a) — `ω_k`— y mecanismo (c) — Fourier— son
  ellos mismos reducibles a un único mecanismo más profundo** más allá de la
  observación de §5.2 de que ambos presuponen `SO(2)`/`2π`. Formalizar esa
  posible unificación de segundo orden (que `ω_k` y las frecuencias de Fourier
  son, en un sentido preciso de teoría de representaciones, la misma
  estructura de grupo actuando de dos formas distintas) queda fuera del
  alcance de este documento.
- **No se sometió nada de esto a revisión externa**, siguiendo la práctica ya
  declarada en el resto del repositorio (`99-sintesis.md` §7).
