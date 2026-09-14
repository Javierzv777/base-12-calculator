# Hallazgo — Ingrediente 2: una medida de tamaño genuinamente no aditiva

Segundo de la lista de "ingredientes" candidatos a producir una geometría que
no necesite π, más allá de sustituir la norma (`03`–`05`, `07`, `08`) o la
topología (`09`, negativo). Ingrediente 2: en vez de tocar la métrica o χ,
tocar el **axioma de aditividad** de la propia medida de tamaño. Todo lo
medido hasta `09` —Hausdorff, Busemann, Holmes–Thompson, Ehrhart, defecto
angular, curvatura de Higuchi— es aditivo sobre uniones disjuntas por
construcción (se obtiene integrando o contando). La pregunta: ¿existe una
noción seria de tamaño, **no aditiva**, que evite π al medir el disco o la
esfera sin degenerar?

Fecha de apertura: 2026-09-13. Protocolo de `00-plan-investigacion.md`: toda
cifra dozenal sale de `target/release/dz-cli.exe` con el comando al lado;
todo hallazgo se clasifica en casilla (I)/(II)/(III); ningún resultado se
elige por dar el número bonito.

**Recordatorio no negociable** (el mismo de `05` y `09`): π es trascendente
(Lindemann–Weierstrass), independiente de la base numeral. Nada de lo que
sigue sugiere que π sea racional o que "esté mal". Se investiga si existe una
medida de tamaño rigurosa, no aditiva, cuyo valor sobre el disco o la esfera
sea libre de π — y, si existe, qué precio paga y si dice algo de L₁ o de base
12.

---

## 0. Resumen del veredicto (para quien no lea el resto)

**Sí existe**: la **capacidad logarítmica** (equivalentemente, el **diámetro
transfinito**) del disco de radio `r` es exactamente `r` — sin π, en ninguna
forma, para ningún radio. Es matemática clásica, no una construcción ad hoc
de este documento, y se verificó con dos fuentes independientes más una
derivación propia. Es honestamente **no aditiva** (de hecho falla incluso la
subaditividad simple, un resultado sorprendente de 1992 que se documenta
abajo). Así que el ingrediente 2, en su forma más literal, **funciona**.

Pero el hallazgo interesante no es ese — es lo que pasa al ponerlo a prueba
contra L₁, siguiendo el estándar del resto de la serie: la capacidad
logarítmica del **cuadrado** (la bola unidad de L₁) no es racional ni
algebraica simple. Es `√2·Γ(1/4)²/(4π^{3/2})` — **π reaparece**, combinado
con una segunda constante trascendente (`Γ(1/4)`, la constante lemniscática)
que es *algebraicamente independiente* de π. La polaridad que domina el resto
de la investigación —L₁ da lo racional, L₂ necesita π— **se invierte aquí**:
bajo capacidad, el disco L₂ es el caso limpio y el cuadrado L₁ es el caso
sucio. Y hay una razón estructural para la inversión, no es casualidad: se
explica en §4.

---

## 1. Barrido de candidatos (pregunta 1 del enunciado)

Revisión honesta, no exhaustiva pero sí representativa, de las nociones de
"tamaño no aditivo" con estatus matemático serio:

| candidato | ¿es no aditiva? | ¿evita π en el disco/esfera? |
|---|---|---|
| **Capacidad de Newton / logarítmica** | Sí — monótona, subaditividad solo en forma débil (ver §3) | **Sí**, exactamente, para el disco y la bola |
| Medidas de Hausdorff `H^s`, `s` no entero (dimensión fractal) | No — siguen siendo aditivas sobre uniones disjuntas (son medidas exteriores de Carathéodory) | No aplica: el disco tiene dimensión entera 2, `H^2` = área de Lebesgue = `πr²`. Las fractales evitan π porque miden *otra cosa* (conjuntos de Cantor, curvas de Koch), no el disco |
| Energía de Riesz `I_s(μ) = ∬|x-y|^{-s}dμdμ` | La energía mínima no es aditiva (es la generalización de la capacidad logarítmica/newtoniana a exponente `s` genérico) | Para `s≠0,d-2` la fórmula del disco involucra funciones Gamma y, en general, sigue trayendo π por el mismo mecanismo que la capacidad (ver §3.4). No se investigó en detalle: mismo género de resultado que la capacidad de Newton, esperable por interpolación |
| Constante isoperimétrica / cociente de Cheeger | Es un **cociente** (perímetro/área), no una medida aditiva en sí — se construye a partir de dos medidas aditivas, hereda π de ambas | No evita π: `h(disco) = perímetro/área → ` involucra π en las dos partes, se cancela parcialmente pero no desaparece (para el disco, la constante de Cheeger es `2/r`, sin π — pero es un caso especial degenerado del disco entero, no de su frontera, y no es una medida de "tamaño" en el sentido pedido) |
| Anchura de Cauchy y funcionales mixtos de Minkowski | La anchura media sí es aditiva bajo suma de Minkowski (fórmula de Cauchy: anchura media `= perímetro/π`), así que no cualifica como "no aditiva" | El perímetro sigue siendo `2πr`; la anchura media del disco es `2r`, sin π — pero es exactamente la misma cancelación aritmética ya vista en `05`/`09` (dividir una cantidad con π entre π), no un mecanismo nuevo |
| Medidas de Choquet / capacidades submodulares en general | Por definición, sí (subaditividad fuerte es axioma) | La capacidad logarítmica **es** el ejemplo canónico de esta familia — no hace falta construir una nueva, ver §3 |

Confirmando la expectativa declarada en el encargo: **la mayoría de las vías
sí involucra π**, y las que no lo hacen (Cheeger, anchura de Cauchy) lo evitan
por cancelación aritmética trivial (dividir entre π algo que ya tenía π), no
por un mecanismo estructural distinto. La única vía con un mecanismo
genuinamente distinto —y el único candidato de la tabla que es honestamente
**no aditivo** y evita π sin cancelación aritmética barata— es la capacidad.
El resto del documento se concentra en ella, como pedía el encargo.

---

## 2. Capacidad logarítmica del disco — derivación y verificación

### 2.1 Definición

Para un compacto `K ⊂ ℂ`, la **energía logarítmica** de una medida de
probabilidad `μ` soportada en `K` es

```
I(μ) = ∬ log(1/|z−w|) dμ(z)dμ(w)
```

y el **constante de Robin** es `V(K) = inf_μ I(μ)` (el ínfimo sobre medidas
de probabilidad en `K`). La **capacidad logarítmica** se define como
`cap(K) = e^{−V(K)}`. Es el mismo objeto que el **diámetro transfinito**
(teorema de Fekete–Szegő: coinciden), y el mismo que el **radio conforme**
para dominios simplemente conexos vía la aplicación de Riemann del exterior.

### 2.2 El disco: derivación propia, no solo citada

Sea `K` el disco cerrado de radio `R`. La aplicación `z ↦ z/R` lleva el
exterior de `K` al exterior del disco unidad, así que la función de Green del
dominio exterior con polo en `∞` es

```
g_Ω(z, ∞) = log|z/R| = log|z| − log R
```

Comparando con la forma asintótica estándar `g_Ω(z,∞) = log|z| − log(cap K) +
o(1)` cuando `z → ∞`, se lee directamente `log(cap K) = log R`, es decir

```
cap(disco de radio R) = R
```

**Sin π en ningún paso.** Verificado además contra dos fuentes independientes
(notas de Saff/Totik sobre teoría de potencial logarítmico, y la referencia
de MathWorld sobre capacidad logarítmica): ambas dan `logcap(K) = R` para el
disco de radio `R`, textualmente.

### 2.3 Dos controles adicionales, también verificados

- **Segmento de longitud `L`**: `cap = L/4`. Resultado clásico (vía la
  aplicación de Joukowski, que lleva el exterior del disco unidad al exterior
  de un segmento de longitud 4), confirmado en la literatura buscada.
- **Elipse de semiejes `a, b`**: `cap = (a+b)/2`. También clásico, misma
  familia de aplicaciones (Joukowski generalizado).

Los tres resultados comparten un patrón: son **racionales/algebraicos en los
parámetros lineales del cuerpo**, sin π, sin radicales más allá de los que ya
traía la propia definición de `a`, `b`. Nótese el contraste inmediato con lo
que la misma familia de cuerpos da bajo medidas *aditivas*: el perímetro de
esa misma elipse es una integral elíptica sin forma cerrada (`99-sintesis.md`
§3.3), y aquí, bajo capacidad, tiene forma cerrada trivial. Es la primera
señal de que capacidad no es "perímetro con otro nombre" — mide algo distinto
de verdad.

### 2.4 Extensión a 3D: capacidad de Newton de la bola

Con el núcleo `1/|x−y|` (el potencial newtoniano puro, sin las constantes
`4πε₀` de la convención física en SI), la capacidad se define análogamente:
`cap(K) = 1/W(K)` donde `W(K)` es la energía newtoniana mínima. Para la bola
de radio `r`, el potencial de la medida uniforme de masa 1 sobre la esfera,
evaluado en cualquier punto de la propia esfera, es `1/r` (hecho clásico de
electrostática: una capa esférica uniforme actúa, desde fuera y sobre sí
misma, como si toda la carga estuviera en el centro). Luego `W = 1/r` y

```
cap(bola de radio r) = r
```

de nuevo sin π. Confirmado contra la fórmula general de la literatura,
`cap(bola de radio r en ℝ^d) = c_d · r^{d−2}` para `d ≥ 3`, con `c_3 = 1` en
esta normalización del núcleo — coincide con la derivación propia. (Nota
sobre convención: en el SI, la capacitancia de una esfera aislada es
`C = 4πε₀r`; el π ahí es un artefacto de la constante de Coulomb, no de la
geometría — la propia fuente estándar de electrostática lo separa así. La
matemática pura, con el núcleo `1/|x−y|` sin reescalar, da `r` exacto.)

---

## 3. ¿Es genuinamente no aditiva? — más sorprendente de lo esperado

Aquí es donde el documento tenía que ser más cuidadoso, y la búsqueda
bibliográfica cambió el resultado a mitad de investigación.

### 3.1 La intuición ingenua

La intuición de "capacidad = tamaño eléctrico" sugiere subaditividad simple:
`cap(A∪B) ≤ cap(A) + cap(B)`, análoga a que la capacidad combinada de dos
conductores nunca supera la suma de sus capacidades aisladas. Esto se cita en
efecto en una monografía clásica del área (Landkof).

### 3.2 Es falsa como está enunciada

Búsqueda bibliográfica específica encontró que esa afirmación **es
incorrecta**: Pyrih (1992), *"Logarithmic capacity is not subadditive — a
fine topology approach"*, construye un contraejemplo explícito a la
subaditividad simple de la capacidad logarítmica, usando técnicas de
topología fina de teoría de potencial. La cita de Landkof, ampliamente
repetida durante décadas, no se sostiene en el caso general.

**Esto es, para los propósitos de este documento, una buena noticia
técnica**: la capacidad logarítmica no solo no es aditiva, ni siquiera es
subaditiva sin más en el sentido ingenuo. Es un ejemplo *más* patológico de
lo que el enunciado del ingrediente 2 pedía, no menos.

### 3.3 Lo que sí es cierto: subaditividad fuerte de `log(cap)`

La teoría de capacidades de Choquet (1954) sigue aplicándose, pero a la
cantidad correcta. Lo que se sostiene, y es lo que hace de la capacidad
logarítmica el ejemplo canónico de "capacidad de Choquet" en potencial
teórico, es la desigualdad de **subaditividad fuerte** para el logaritmo de
la capacidad:

```
log cap(K₁ ∪ K₂) ≤ log cap(K₁) + log cap(K₂) − log cap(K₁ ∩ K₂)
```

Esta es la propiedad que de verdad hace de `cap` (vía su logaritmo, o de
forma equivalente vía la energía `V = −log cap`) una capacidad en el sentido
axiomático de Choquet: monótona, continua, fuertemente subaditiva — y por
tanto sujeta al teorema de capacitabilidad de Choquet (todo boreliano es
"capacitable": su capacidad interior y exterior coinciden). La capacidad
newtoniana en dimensión `d ≥ 3` sí es fuertemente subaditiva **directamente**
(sin pasar por el logaritmo), porque su núcleo `|x−y|^{2−d}` es
positivo-definido en sentido de forma cuadrática, mientras que el núcleo
logarítmico en el plano solo lo es sobre conjuntos acotados y con signo — la
raíz técnica de por qué el caso 2D necesita el paso extra por el logaritmo y
el caso ≥3D no.

### 3.4 Veredicto de la pregunta "¿es no aditiva de verdad?"

**Sí, sin ambigüedad, y en un sentido más fuerte de lo que se esperaba al
abrir el documento.** No es aditiva (trivialmente: la capacidad de un punto
es 0, la de dos puntos también, pero la capacidad de un segmento largo puede
ser arbitrariamente grande — la "capacidad total" no se comporta como una
suma de partes en absoluto). No es siquiera subaditiva sin más, en el sentido
ingenuo (Pyrih). Sí satisface la versión logarítmica de subaditividad fuerte,
que es la propiedad axiomática correcta y la que sostiene toda la teoría. Se
cumple así, con la corrección bibliográfica incluida, el punto 3 del
encargo (verificar los axiomas): monotonía sí, continuidad sí, subaditividad
simple **no** (documentado como fracaso real, no evitado), subaditividad
fuerte del logaritmo sí.

---

## 4. La prueba contra L₁ — y la inversión de polaridad

Siguiendo el estándar del resto de la serie (§4.5–§4.8 de `99-sintesis.md`):
ningún hallazgo cuenta como "conectado con L₁" sin calcular el objeto L₁
correspondiente y comparar. La bola unidad de L₁ es el cuadrado de vértices
`(±1,0), (0,±1)` — un rombo, lado `√2`, área 2.

### 4.1 La cifra

Búsqueda bibliográfica específica (no derivación propia — Schwarz–Christoffel
para un cuadrado exige funciones hipergeométricas, fuera del alcance de una
derivación autocontenida en este documento) da, para el cuadrado con vértices
en `1, i, −1, −i` (exactamente la bola unidad L₁):

```
cap(bola unidad L₁) = √2 · Γ(1/4)² / (4π^{3/2}) ≈ 0,83462684167...
```

Fuente citada explícitamente en la búsqueda: cálculo estándar de capacidad
logarítmica de un cuadrado vía la constante de la lemniscata (aparece en
Pólya–Szegő, *Isoperimetric Inequalities in Mathematical Physics*, y en
tratamientos modernos de capacidad logarítmica de polígonos). **No se
verificó esta cifra con una segunda fuente independiente** — es la única
cifra de este documento con una sola fuente, y se marca así explícitamente
como el punto más débil de la verificación (ver §7).

`Γ(1/4)` es la constante lemniscática, trascendente, y —por un teorema de
Chudnovsky/Nesterenko (independencia algebraica de `π`, `Γ(1/4)` y `e^π`)—
**algebraicamente independiente de π**. No es "otro nombre para π": es una
segunda constante trascendente genuinamente distinta que aparece
multiplicada y dividida por potencias fraccionarias de π.

### 4.2 El binario no puede verificar esta cifra

`dz-cli` no tiene función Gamma ni constante lemniscática en su catálogo
(`--functions`, comprobado: solo `pi`, `tau`, `euler` como constantes). Por
eso el valor de §4.1 se cita de la literatura y **no** se presenta como una
cifra dozenal calculada por el binario — sería importar un número que el
proyecto no puede producir con sus propias puertas, exactamente lo que el
protocolo prohíbe hacer "a mano".

### 4.3 Por qué la inversión no es casualidad — el mecanismo

Esto es lo que hace que la sección valga la pena, más allá de "otro número
feo". La capacidad logarítmica es, por el teorema de la aplicación de
Riemann, el **radio conforme** del exterior del cuerpo: se calcula a partir
de la aplicación biholomorfa entre el exterior del cuerpo y el exterior del
disco unidad, y su valor es el coeficiente principal de esa aplicación en
`∞`.

- Para el disco, la elipse y el segmento, esa aplicación exterior es
  **elemental**: identidad, Joukowski, Joukowski degenerado — funciones
  racionales o algebraicas simples de `z`. De ahí que la capacidad salga
  algebraica limpia (de hecho racional en los tres casos de §2.3).
- Para el cuadrado (o cualquier polígono), la aplicación exterior es de
  **Schwarz–Christoffel**: una integral que involucra `∫(w−v_k)^{−α_k}dw`
  sobre los vértices `v_k` con ángulos exteriores `α_k`. Para un cuadrado, esa
  integral es exactamente la que define la función Beta/Gamma incompleta que
  produce `Γ(1/4)` — la misma familia de integrales que aparece en la
  rectificación de la lemniscata de Bernoulli, de donde el nombre de la
  constante.

**Esto invierte exactamente la polaridad de `99-sintesis.md` §3.3.** Ahí, la
ventaja de L₁ era que ser lineal a trozos (un polígono) da forma cerrada
donde lo suave (la elipse) no la tiene — el perímetro de la elipse es una
integral elíptica sin solución, el de L₁ es `8a−4c` exacto. Aquí ocurre lo
contrario: ser lineal a trozos (tener esquinas) es lo que **fuerza** la
integral difícil (Schwarz–Christoffel/Gamma), mientras que ser suave (el
disco, la elipse) es lo que da la aplicación conforme elemental y por tanto
la capacidad limpia. La razón de fondo es que **capacidad y perímetro son
funcionales geométricamente opuestos en su sensibilidad a las esquinas**: el
perímetro es una integral de arco-longitud, indiferente a la suavidad (una
esquina no le añade coste, es solo una discontinuidad de la derivada); la
capacidad es un objeto **conforme**, y las esquinas son exactamente las
singularidades que rompen la elementalidad de una aplicación de Riemann. No
hay una "ventaja de L₁" universal esperando encontrarse — cada funcional
tiene su propia noción de qué cuenta como "simple", y son ejes distintos, tal
y como `99-sintesis.md` §4.5 ya advertía con el ejemplo K=5 empatado del
octaedro/esfera. Esta sección es un segundo ejemplo del mismo fenómeno, con
signo invertido.

### 4.4 Casilla

**(I) intrínseca a L₂, con una precisión importante.** La capacidad
logarítmica no es una noción que se pueda "trasladar a L₁" en el sentido en
que `05`/`09` trasladaban área o curvatura — es intrínsecamente una
construcción de **análisis complejo / funciones armónicas**: usa el núcleo
`log|z−w|` (solución fundamental del laplaciano en 2D) y aplicaciones
holomorfas (conformes). Esto es literalmente el mismo punto que
`99-sintesis.md` §3.1 ya estableció con la multiplicación de ℂ: la estructura
holomorfa/armónica **es** L₂, no una elección entre otras. Cuando se calcula
"la capacidad del cuadrado", lo que se está midiendo es un cuadrado *inmerso*
como subconjunto de ℂ con su estructura conforme euclidiana de fondo — no
existe una "capacidad logarítmica L₁" con su propio núcleo `‖z−w‖₁` que
generalice de forma natural la teoría (el análogo L₁ del laplaciano no es
elíptico de la misma manera, y no hay teorema de aplicación de Riemann para
normas no euclidianas). Así que, con rigor, **la pregunta "capacidad de la
bola L₁" ya presupone L₂** para poder definirse; el resultado de §4.1 es una
medida de cuánto "cuesta" en L₂ que el cuerpo tenga esquinas, no una medida
de L₁ en sus propios términos. Dicho de otro modo: esta sección no compara
L₁ contra L₂ en igualdad de condiciones — compara "disco" contra "cuadrado
insertado en el plano complejo", y el cuadrado pierde por depender de una
maquinaria (Riemann/Schwarz–Christoffel) que es puramente L₂ desde el primer
paso.

---

## 5. Conexión con base 12 (pregunta 4 del encargo)

**Ninguna, y de la forma más limpia de descartar posible: ni siquiera aparece
el patrón habitual de `12=2²·3`.**

Los tres denominadores que aparecen en las fórmulas verificadas —`R` mismo
(disco, sin denominador), `/4` (segmento), `/2` (elipse)— son potencias de 2.
Comprobado con el binario, sin que haga falta ningún racional con factor 3:

```
$ target/release/dz-cli.exe eval "1/4" --mode fixed --scale 8
0;3
$ target/release/dz-cli.exe eval "3/4" --mode fixed --scale 8
0;9
$ target/release/dz-cli.exe eval "1/2" --mode fixed --scale 8
0;6
```

Terminan en dozenal, pero **también terminan en decimal** (`1/4=0,25`,
`3/4=0,75`, `1/2=0,5`) porque 2 es factor primo compartido por ambas bases
(`10=2·5`, `12=2²·3`). El control de contraste, para que quede explícito qué
*sí* discrimina entre bases y qué no:

```
$ target/release/dz-cli.exe eval "1/3" --mode fixed --scale 8
0;4
```

`1/3` sí termina en dozenal y no en decimal — pero **ningún denominador 3
aparece en ninguna fórmula de capacidad verificada en este documento**. Y la
capacidad del cuadrado (§4.1), al ser trascendente, ni siquiera entra en la
discusión de terminación: `N_periodicos` no se aplica a un número que no es
racional.

**Veredicto explícito de casilla (I) para la pregunta dozenal**: no hay
conexión, ni siquiera la conexión barata de `12=2²·3` que atraviesa el resto
de la línea D (`99-sintesis.md` §4.1–§4.2). La capacidad logarítmica, en los
casos donde da un valor racional, lo hace con denominadores que son potencias
de 2 puras — el subgrupo de casos donde base 10 y base 12 **empatan**, no
donde base 12 gana. Es, si acaso, un contraejemplo modesto a la esperanza
implícita de la línea D: no toda familia de fórmulas "bonitas" trae consigo
el factor 3 que favorece a la base 12.

---

## 6. Veredicto global del ingrediente 2

| sub-pregunta | resultado |
|---|---|
| ¿existe una medida de tamaño rigurosa, no aditiva, que evite π en el disco/esfera? | **Sí** — capacidad logarítmica/de Newton, `cap = r` exacto, verificado por derivación propia y dos fuentes |
| ¿es genuinamente no aditiva (no solo "menos aditiva")? | **Sí, más de lo esperado** — falla incluso la subaditividad simple (Pyrih 1992), corrigiendo una afirmación repetida en la literatura clásica (Landkof); se sostiene la subaditividad fuerte del logaritmo, que es la propiedad axiomática correcta (Choquet) |
| ¿desaparece π de verdad, o reaparece al relacionar la capacidad con cantidades familiares? | **Reaparece parcialmente**: la razón capacidad/circunferencia es `1/(2π)` para cualquier disco (verificado: `2*pi ≈ 6;34941696` dozenal con `--gates`, `1/(2π) ≈ 0;1XE02X1...`), y la densidad de la medida de equilibrio sobre el círculo es uniforme `dθ/(2π)` — π gobierna cómo se *distribuye* la carga de equilibrio aunque no aparezca en el *total*. `cap` mide algo dimensionalmente distinto de área o perímetro (tiene unidades de longitud, como un "radio efectivo"), así que no es una sustituta de ninguna de las dos, es una tercera cantidad |
| ¿conecta con L₁? | **Sí, y en sentido negativo respecto al resto de la serie** — la capacidad del cuadrado (bola L₁) es `√2·Γ(1/4)²/(4π^{3/2})`, trascendente, involucra π combinado con una segunda constante algebraicamente independiente (`Γ(1/4)`). Invierte la polaridad habitual: aquí es L₂ (el disco) quien da la fórmula limpia y L₁ (el cuadrado) quien la complica, por una razón estructural real (las esquinas rompen la elementalidad de la aplicación conforme, exactamente al revés de cómo ayudaban a la longitud de arco en `99-sintesis.md` §3.3) |
| ¿conecta con base 12? | **No**, ni siquiera en la forma barata de `12=2²·3` — los denominadores que aparecen (2, 4) son potencias de 2, comunes a ambas bases |

**Lectura honesta de conjunto.** El ingrediente 2 es el primero de la serie
de "ingredientes" (`09` fue el primero cronológicamente y dio negativo en su
pregunta central) que produce un **sí** genuino y no forzado a la pregunta
que lo motivaba: existe una medida de tamaño real, publicada, no inventada
para esta investigación, que es honestamente no aditiva y da exactamente `r`
—sin π— para el disco y la esfera. Eso responde la pregunta 2 del encargo
afirmativamente y con rigor.

Pero el resultado no vindica la reinterpretación caritativa de la afirmación
de Kryon en la forma en que se esperaba. Primero, porque `cap` no es "el
tamaño del círculo" en el sentido intuitivo que motiva la pregunta —no es
área, no es perímetro, no tiene la misma dimensión que ninguna de las dos
cosas que la gente mide con una cinta métrica o llenando con pintura—, es una
cantidad de teoría del potencial (capacitancia electrostática / radio
conforme) que coincide numéricamente con el radio por una razón específica de
esa teoría, no porque haya "corregido" nada sobre el círculo ordinario.
Segundo, y es el hallazgo con más contenido del documento, porque al someter
la idea a la prueba de conexión con L₁ que exige el protocolo de esta serie,
**el resultado se revierte**: es el disco euclidiano el que sale limpio bajo
capacidad, y el cuadrado L₁ el que se ensucia con una segunda trascendente.
Si algo dice esto sobre la relación L₁/L₂ en general, es un dato más a favor
del hallazgo transversal de `99-sintesis.md` §4.5: **complejidad de forma y
complejidad de medida son ejes independientes**, y aquí, además, ni siquiera
apuntan siempre en la misma dirección — a veces L₁ gana (arco, mediana,
rutas), a veces empata en forma y pierde en medida (octaedro/esfera), y aquí
**pierde directamente** (cuadrado peor que disco, bajo esta medida
concreta).

---

## 7. Lo que no funcionó (obligatorio, no vacío)

- **La cifra de §4.1 (capacidad del cuadrado) tiene una sola fuente
  verificada**, no dos como el resto de las fórmulas citadas. `dz-cli` no
  puede calcularla (sin función Gamma), así que no hay ni siquiera un control
  numérico independiente con el propio instrumento del proyecto. Es la parte
  más débil del documento y se declara así explícitamente, en vez de
  presentarla con la misma confianza que `cap(disco)=r`, que sí tiene
  derivación propia completa.
- **No se verificó una derivación propia de la fórmula del cuadrado.** A
  diferencia de §2.2 (disco, derivado a mano vía función de Green), la
  Schwarz–Christoffel de un cuadrado requiere maquinaria de funciones
  hipergeométricas que está fuera del alcance razonable de este documento sin
  volverse un tratado de análisis complejo aparte. Se cita, no se rederiva —
  y se dice así, en vez de disimular la diferencia de rigor entre las dos
  secciones.
- **No se exploró la energía de Riesz para exponentes `s` genéricos**, que
  la tabla de §1 señala como la generalización natural y que probablemente
  repite el mismo patrón (racional para el disco en casos especiales,
  Gamma/Beta para el cuadrado) sin que se haya comprobado.
- **No se investigó si existe alguna bola L_p, `p∈(1,∞)`, cuya capacidad
  logarítmica sea racional** — la familia `Lp` regaló racionales "baratos" en
  `99-sintesis.md` §4.7–§4.8 para la razón perímetro/diámetro por tener un
  parámetro continuo; no se comprobó si el mismo truco funciona aquí. Es la
  pregunta más obvia que este documento deja abierta.
- **La afirmación de Landkof sobre subaditividad simple, repetida durante
  décadas, resultó falsa** (Pyrih 1992). Se documenta el error bibliográfico
  encontrado en el camino, en vez de repetirlo sin verificar — es exactamente
  el tipo de trampa que `00-plan-investigacion.md` pide vigilar, solo que
  aquí la trampa estaba en una fuente autorizada, no en la propia
  investigación.
- **No se relacionó esta sección con el hallazgo de Gelfand/Ostrowski de
  `99-sintesis.md` §3.1** más allá de una observación en §4.4 — ambos
  resultados dicen "L₂ está determinada, no elegida" por razones de
  estructura compleja/holomorfa, pero no se exploró si son la misma obstrucción
  vista dos veces o dos obstrucciones independientes que coinciden en la
  conclusión, la misma pregunta que `06-hallazgo-conversion-angular-taxicab.md`
  ya se hizo (y respondió que no) para otro trío de mecanismos.
- **Nada de esto se sometió a revisión externa**, siguiendo la práctica ya
  declarada del resto del repositorio (`99-sintesis.md` §7).
