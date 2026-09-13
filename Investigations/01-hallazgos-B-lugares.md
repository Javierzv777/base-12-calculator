# Hallazgos — Línea B: lugares geométricos y trigonometría

Preguntas 3, 4 y 5 de `planteamientos.md`. Protocolo: `00-plan-investigacion.md`.
Predicción bajo examen: **P3**.

Fecha: 2026-09-12.
Instrumento: `target/release/dz-cli.exe` (todas las cifras dozenales de este
documento salen de él; cada una lleva su comando al lado).
Cifras compartidas: se citan de `02-referencia-medida.md` en vez de recalcularse.

**Nota de método sobre los literales.** En `dz-cli` un literal desnudo se lee
**en base 12**: `1/24` es un veinteochoavo, no un veinticuatroavo. Reviso aquí
mis propias cifras contra esa trampa:

- todos los literales de §1, §2.5 y §3 son **dígitos sueltos `0`-`9`**
  (`2+3`, `9/4`, `2/3`, `8*3-4*2`, `2*(3^2-2^2)`, `7/3`, `1/2`), que denotan el
  mismo número en las dos bases. No hay ambigüedad posible;
- el único par multidígito escrito en dozenal a propósito es `8*21-4*15` (§2.6),
  donde `21₁₂ = 25₁₀` y `15₁₂ = 17₁₀`, y lo verifico en la misma sección con
  `eval "25" --input-base 10` → `21` y `eval "17" --input-base 10` → `15`;
- todo lo demás multidígito va con `--input-base 10` explícito y así está escrito
  en el comando.

Con eso queda **ninguna cifra de este documento afectada por la trampa**.

---

## 0. Resumen ejecutable

| | resultado |
|---|---|
| **P3** | **confirmada**, y con demostración, no solo con contraejemplo |
| identidad pitagórica taxicab | `\|cos₁θ\| + \|sin₁θ\| = 1` — cierta, y **vacía** (es la ecuación del lugar, no un teorema sobre funciones) |
| se vuelven lineales por tramos | **12 familias sí / 7 no** (§1.4), de las cuales solo **3** son ganancias estructurales y no tautológicas |
| precio mayor | pérdida de **unicidad** con medida positiva (bisectriz L₁, geodésicas L₁) y el hecho de que **SSS no determina los ángulos** en L₁ |
| efecto base 12 en esta línea | **1 ganancia, 1 pérdida, resto nulo**. Apoya **P5** |

Anticipo del sesgo declarado en el plan §2.1: K(F) baja en casi todas las
familias de esta línea, y eso **no** es informativo por sí solo. Lo que sí
informa está en §1.4 (las 7 familias donde L₁ no gana o pierde), §1.5–§1.6 (la no
unicidad), §2.2–§2.4 (las degeneraciones del lugar) y §5 («lo que no funcionó»).

---

## 1. Pregunta 3 — d₂ frente a d₁

### 1.1 Enunciados

**L₂.** `d₂(P,Q) = √((x₂−x₁)² + (y₂−y₁)²)`. Norma inducida por un producto
interno; grupo de isometrías `E(2) = ℝ² ⋊ O(2)`, de dimensión 3, con una parte
lineal **continua** (las rotaciones).

**L₁.** `d₁(P,Q) = |x₂−x₁| + |y₂−y₁|`. No procede de ningún producto interno
(la bola unidad no es una elipse). Grupo de isometrías `ℝ² ⋊ D₄`: la parte
lineal es **finita, de orden 8** (las simetrías del rombo: 4 rotaciones de
cuarto de vuelta y 4 reflexiones).

Ese contraste — dimensión 1 frente a dimensión 0 en la parte lineal — es el
único hecho que hace falta para todo lo demás de esta línea, incluida la
respuesta a la pregunta 5.

**Hecho estructural que conviene fijar ya.** La aplicación
`T(x,y) = (x+y, x−y)` cumple `‖T v‖_∞ = ‖v‖_1` exactamente. Es decir: el plano
L₁ y el plano L∞ son **isométricos** vía un giro de 45° con reescalado. Todo
resultado «L₁» de §1.4 tiene un gemelo L∞ obtenido por esta transformación, y
varios de los éxitos (el 1-centro, la rejilla de Hanan) son en realidad
resultados L∞ trasladados. No es una simplificación adicional: es la misma.

### 1.2 Coste medido — el número objetivo

El plan pide una cifra que no dependa del criterio humano.

**La cifra de referencia es la del fichero compartido**
(`02-referencia-medida.md` §3), y es la mejor de todas porque usa la terna
`(3,4,5)`, donde **el resultado euclidiano es el entero exacto `5`** y no hay
truncamiento que achacar:

| | d₂ = √(3²+4²) | d₁ = 3+4 | razón |
|---|--:|--:|--:|
| puertas | **43.631.965** | **39.469** | **≈ 1.105×** |
| profundidad | 16.001 | 441 | ≈ 36× |

Eso ya dice lo esencial: el sobrecoste no está en la irracionalidad del valor,
está en el **algoritmo** (`sqrt` corre Newton con divisiones restauradoras sin
saber que el resultado va a ser entero). Y el propio fichero compartido acota
qué demuestra y qué no, en particular que `5` y `7` no son dos respuestas a la
misma pregunta, y que esto es casilla (II) y no (III). Suscribo esas tres
salvedades y no las repito.

Lo que añado aquí es el **barrido**, para ver si la razón es estable o depende del
operando — cosa que la cifra única no puede responder. Mismo par de incrementos,
las dos distancias, backend MVL, modo fijo.

| Δ = (Δx, Δy) | d₁ (dozenal) | puertas d₁ | d₂ (dozenal, escala 8) | puertas d₂ | razón |
|---|---|--:|---|--:|--:|
| (2, 3) | `5` | 33.353 | `~3;73248650` | 27.264.979 | ≈ 581₁₂ |
| (5, 7) | `10` | 42.501 | `~8;72899974` | 25.795.982 | ≈ 426₁₂ |
| (7, 9) | `14` | 54.430 | `~E;49X29392` | 22.311.942 | |
| (↋, ↋) | `1X` | 69.720 | `~13;68145593` | 22.812.446 | |

```bash
target/release/dz-cli.exe eval "2+3"   --mode fixed --gates
# 5
# gates: cycle=6864 min=11060 max=8253 lit=5712 thr=1464 total=33353 depth=441 recon=0
target/release/dz-cli.exe eval "5+7"   --mode fixed --gates
# 10
# gates: cycle=8844 min=14030 max=10483 lit=7272 thr=1872 total=42501 depth=441 recon=0
target/release/dz-cli.exe eval "7+9"   --mode fixed --gates
# 14
# gates: cycle=10824 min=18332 max=13610 lit=9336 thr=2328 total=54430 depth=441 recon=0
target/release/dz-cli.exe eval "E+E"   --mode fixed --gates
# 1X
# gates: cycle=13464 min=23772 max=17580 lit=11976 thr=2928 total=69720 depth=441 recon=0

target/release/dz-cli.exe eval "sqrt(2^2+3^2)" --mode fixed --scale 8 --gates
# ~3;73248650
# gates: cycle=5733288 min=9106152 max=6750403 lit=4617912 thr=1057224 total=27264979 depth=19198 recon=0
target/release/dz-cli.exe eval "sqrt(5^2+7^2)" --mode fixed --scale 8 --gates
# ~8;72899974
# gates: cycle=5403288 min=8632132 max=6394738 lit=4369416 thr=996408 total=25795982 depth=18129 recon=0
target/release/dz-cli.exe eval "sqrt(7^2+9^2)" --mode fixed --scale 8 --gates
# ~E;49X29392
# gates: cycle=4681908 min=7459398 max=5527764 lit=3779208 thr=863664 total=22311942 depth=15775 recon=0
target/release/dz-cli.exe eval "sqrt(E^2+E^2)" --mode fixed --scale 8 --gates
# ~13;68145593
# gates: cycle=4768632 min=7641012 max=5658986 lit=3864840 thr=878976 total=22812446 depth=15968 recon=0

# la razón, calculada por el propio binario (entrada decimal):
target/release/dz-cli.exe eval "27264979/33353" --input-base 10 --mode fixed --scale 4
# 581;573106E0733705X37E892E6 (periodica en base 12, valor truncado)
target/release/dz-cli.exe eval "25795982/42501" --input-base 10 --mode fixed --scale 4
# 426;E497283907063EX5E45E9755 (periodica en base 12, valor truncado)
```

Lectura honesta de esta tabla, con tres reservas:

1. **El factor no es una constante.** El barrido da `581₁₂`, `426₁₂` (unos 800 y
   600 decimales) y la referencia compartida da `1.105` para `(3,4,5)`: el mismo
   cociente varía por un factor 2 según el par de puntos. Así que no es «L₂ cuesta
   *k* veces más»; es *«evaluar `sqrt` con el Newton de `dz-sci` a escala 8 cuesta
   entre 600 y 1.100 veces lo que una suma de un dígito en este repositorio»*. Lo
   que domina es la división restauradora dentro de Newton, el circuito más caro
   del proyecto (`docs/05-funciones-cientificas.md`). Lo que **sí** es
   independiente de la implementación es el dato cualitativo: d₁ es **exacta y
   termina**, d₂ lleva `~` (truncada) en todos los casos salvo ternas pitagóricas.
2. **El coste de d₂ *baja* al crecer los operandos** (27,3M → 22,3M de (2,3) a
   (7,9)) y el de d₁ *sube*. Es el Newton: converge en menos iteraciones cuando
   la semilla queda mejor colocada. Es decir, el coste de d₂ **no es monótono en
   el tamaño del dato**, lo que hace de él un mal instrumento comparativo fino, y
   explica por qué la referencia `(3,4,5)` y mi barrido no coinciden. Lo uso solo
   para el orden de magnitud.
3. **La escala 8 es una elección mía.** A escala 24 la brecha se abre; a escala 2
   se cierra. La brecha no es una constante geométrica.

### 1.3 K(F) — pregunta 3

Reglas del plan §2.1 aplicadas literalmente. `N_periodicos` cuenta constantes
**racionales de la fórmula** cuya expansión no termina en la base considerada.

#### (a) Distancia entre dos puntos

| | N_raíces | N_irrac | N_oper | N_period (dec) | N_period (doz) | K (dec) | K (doz) |
|---|--:|--:|--:|--:|--:|--:|--:|
| `√((x₂−x₁)²+(y₂−y₁)²)` | 1 | 0 | 5 | 0 | 0 | **6** | **6** |
| `\|x₂−x₁\|+\|y₂−y₁\|` | 0 | 0 | 5 | 0 | 0 | **5** | **5** |

Conteo de operaciones L₂: 2 restas, 2 potencias, 1 suma = 5. L₁: 2 restas,
2 valores absolutos, 1 suma = 5.

**K solo baja 1, y la base 12 no cambia nada.** Ésta es la primera evidencia de
que K(F) es un instrumento tosco: no ve que el *valor* de d₂ es irracional y el
de d₁ entero, porque las reglas cuentan constantes de la fórmula, no del
resultado. El coste medido (§1.2) sí lo ve. Donde K y las puertas discrepan,
el plan obliga a reportar las dos; queda reportado.

#### (b) Distancia de un punto a una recta `ax+by+c = 0` — norma dual

Este caso es más interesante que el anterior porque la simplificación **no** es
sustituir un radical por un valor absoluto, sino un hecho de análisis funcional:
la distancia a un hiperplano en `‖·‖_p` se calcula con la norma **dual** `‖·‖_q`,
`1/p + 1/q = 1`. Para `p = 1`, `q = ∞`:

```
d₂(P, L) = |ax₀+by₀+c| / √(a²+b²)
d₁(P, L) = |ax₀+by₀+c| / max(|a|, |b|)
```

y en dimensión `n` esto es `/ √(Σaᵢ²)` frente a `/ maxᵢ|aᵢ|`: L₂ necesita un
radical de una suma de `n` términos, L₁ no necesita ninguno **para todo `n`**.
Casilla (II), y es una de las simplificaciones más limpias de la línea.

| | N_raíces | N_irrac | N_oper | N_per (dec) | N_per (doz) | K (dec) | K (doz) |
|---|--:|--:|--:|--:|--:|--:|--:|
| L₂ | 1 | 0 | 9 | 0 | 0 | **10** | **10** |
| L₁ | 0 | 0 | 9 | 0 | 0 | **9** | **9** |

Medida, con `a=2, b=3`, punto tal que el numerador sea `9`:

```bash
target/release/dz-cli.exe eval "9/3" --mode fixed --gates
# 3
# gates: cycle=88044 min=81950 max=65333 lit=50256 thr=16728 total=302311 depth=1272 recon=0
target/release/dz-cli.exe eval "9/sqrt(2^2+3^2)" --mode fixed --scale 8 --gates
# ~2;5E5422725545319X475X7801
# gates: cycle=6384312 min=10111332 max=7497916 lit=5132160 thr=1177872 total=30303592 depth=19198 recon=0
```

`3` exacto y 302.311 puertas frente a `~2;5E5…` truncado y 30.303.592 puertas:
factor ≈ 100. Aquí L₁ gana en todo.

**Y aquí está el contraejemplo dozenal**, con `a=3, b=4` (terna pitagórica), que
merece estar en la misma sección y no escondido en §5:

```bash
target/release/dz-cli.exe eval "9/4" --mode fixed --gates
# 2;3
# gates: cycle=90684 min=86094 max=68442 lit=52428 thr=17304 total=314952 depth=1312 recon=0
target/release/dz-cli.exe eval "9/5" --mode fixed --scale 8 --gates
# 1;972497249724972497249724 (periodica en base 12, valor truncado)
# gates: cycle=142560 min=164520 max=127480 lit=93912 thr=28632 total=557104 depth=1312 recon=0
```

Con `a=3, b=4` la fórmula **L₂ da un racional exacto** (`9/5`, porque
`√(9+16)=5`) — no hay radical que eliminar, L₁ no tiene nada que ganar en
estructura — **y ese racional no termina en base 12** (`1;9724972497…`) mientras
sí termina en base 10 (`1,8`). O sea: en esta instancia concreta **la base 12
empeora la fórmula L₂**, y L₁ gana solo por el `4` del denominador. Es la
situación prevista en el plan §2.3 (`denominadores {5,10,20}` pierden) apareciendo
espontáneamente en el sitio menos buscado: la geometría analítica elemental está
llena de ternas pitagóricas, y las ternas pitagóricas están llenas de 5.

Concuerda con la tabla de recíprocos de `02-referencia-medida.md` §1, que ya
registra `1/5 = 0;2497 2497…` (no termina en base 12, sí en base 10) frente a
`1/4 = 0;3` y `1/3 = 0;4`. Mi aportación es solo señalar **dónde** aparece el
quinto sin que nadie lo invite: en el `√(a²+b²)` de toda terna pitagórica.

### 1.4 Recuento: qué se vuelve lineal por tramos y qué no

Éste es el núcleo de la pregunta 3. Clasifico cada familia y marco con ★ las que
considero ganancias **estructurales** (cambian la clase del problema, no solo el
número de símbolos).

#### Sí se vuelve lineal por tramos — 12 familias

| # | familia | L₂ | L₁ | casilla |
|--:|---|---|---|---|
| 1 | distancia punto-punto | radical | lineal por tramos, exacta | (II) |
| 2 | distancia punto-hiperplano | `√Σaᵢ²` | `max\|aᵢ\|` (norma dual), toda `n` | (II) |
| 3 | ecuación de la esfera | `Σxᵢ² = r²` (cuádrica) | `Σ\|xᵢ\| = r` (politopo) | (II) |
| 4 | elipse | cuádrica | hexágono (§2) | (II) |
| 5 ★ | **perímetro de la elipse** | integral elíptica `4aE(e)`, **sin forma cerrada elemental** | `8a − 4c`, exacto | (II) |
| 6 | área de la elipse | `πa√(a²−c²)` | `2(a²−c²)` | (II) |
| 7 | parábola (foco-directriz) | cuádrica | «V» más dos rayos verticales (§2.3) | (II) |
| 8 ★ | **mediana geométrica (Fermat-Weber)** | sin forma cerrada; **no expresable por radicales para `n ≥ 5`** (Bajaj, 1988) | `(mediana xᵢ, mediana yᵢ)`, exacta y racional | (II) |
| 9 | 1-centro / bola envolvente mínima | programa cuadrático | caja envolvente tras el giro de 45° | (II) |
| 10 | árbol de Steiner mínimo | razón de Steiner `√3/2`, **conjeturada** (Gilbert-Pollak) | razón `2/3`, **teorema** (Hwang, 1976); rejilla de Hanan | (II)+(III) |
| 11 ★ | **rutas mínimas con obstáculos** | longitudes = sumas de raíces; comparar dos de ellas es el *sum-of-square-roots problem*, sin cota polinómica conocida de precisión | óptimo sobre la rejilla de Hanan; longitudes = sumas de coordenadas de entrada, comparación **exacta en aritmética racional** | (II) |
| 12 | `k`-means → `k`-medians | objetivo cuadrático | objetivo lineal por tramos | (II), con reserva |

Reserva del 12: el objetivo se vuelve lineal por tramos pero el problema sigue
siendo NP-difícil. «La fórmula se simplificó» y «el problema se simplificó» no
son lo mismo, y K(F) no distingue. Lo mismo vale del 10 y del 11: la rejilla de
Hanan reduce un problema continuo a uno finito, que es mucho, pero el Steiner
rectilíneo sigue siendo NP-difícil.

Las tres ★ son las únicas donde la ganancia no es tautológica respecto al sesgo
de K(F), porque en las tres el lado L₂ **no tiene forma cerrada en absoluto** y
el lado L₁ sí:

- **5**: de una integral elíptica a `8a − 4c`. No es «menos raíces»: es pasar de
  una función trascendente de dos parámetros a un polinomio de grado 1.
- **8**: de un objeto que un teorema prohíbe escribir con radicales a una
  selección de orden (dos medianas). El mecanismo es la **separabilidad**:
  `Σ|x−xᵢ| + Σ|y−yᵢ|` se descompone en dos problemas independientes de una
  variable, lo que la suma de raíces no permite. Ésta es, a mi juicio, la
  ganancia más sólida de toda la línea B.
- **11**: cambia la clase de **exactitud computable**, no la longitud de la
  fórmula. Es el único caso donde puedo decir que L₁ hace *decidible con
  precisión acotada* algo que en L₂ no se sabe decidir así.

#### No se vuelve lineal por tramos, o L₁ pierde — 7 familias

| # | familia | por qué L₁ no gana | casilla |
|--:|---|---|---|
| 13 | **mediatriz y diagramas de Voronoi** | en L₂ la mediatriz **ya es una recta** (grado 1). En L₁ es una polilínea de hasta 3 tramos y, cuando `\|Δx\| = \|Δy\|`, **contiene regiones de área positiva** (§1.5). L₁ es estrictamente peor | (I) a favor de L₂ |
| 14 | **regresión** | mínimos cuadrados: forma cerrada `(XᵀX)⁻¹Xᵀy`, racional en los datos, sin radicales. Desviaciones absolutas mínimas (LAD): **programa lineal, sin forma cerrada**, solución no única y no continua en los datos. L₁ pierde | (I) a favor de L₂ |
| 15 | triangulación de Delaunay | la propiedad del círculo vacío se vuelve «rombo vacío»: existe pero no es única, y la propiedad de **maximizar el ángulo mínimo** no tiene análogo porque L₁ no define ángulos (§3.5) | (I) |
| 16 | Herón / resolución de triángulos | **SSS no determina los ángulos en L₁** (§3.5, contraejemplo explícito). No hay ley del seno ni del coseno | (I) |
| 17 | circuncentro / circunferencia por 3 puntos | en L₂: forma cerrada racional (cociente de determinantes). En L₁ el «rombo por 3 puntos» puede no existir o no ser único | (I) |
| 18 | hipérbola | **se degenera en dos rectas verticales**, y en el caso `a = c` en dos semiplanos (§2.3). Formalmente lineal por tramos; en contenido, el lugar desapareció | forma (II), contenido (I) |
| 19 | cualquier cantidad que dependa de la dirección | perímetro de un segmento girado, momentos de inercia, isoperimetría: en L₁ dependen de la orientación, así que **no hay fórmula invariante que simplificar** | (I) |

**Recuento: 12 sí, 7 no.** Y la lectura que me parece defendible: la partición
no es «L₁ simplifica la geometría», sino **«L₁ simplifica lo que ya era
optimización separable o combinatoria, y estropea lo que ya era lineal o
dependía de ángulos»**. Los 12 éxitos son casi todos problemas de *minimización
de sumas de distancias* o *ecuaciones de bolas*; los 7 fracasos son casi todos
problemas de *ángulo*, *unicidad* o *ajuste*.

### 1.5 La trampa de Voronoi, explícita

El plan exige no despachar esto como nota al pie. La mediatriz L₁ de
`P = (0,0)` y `Q = (2,2)`:

- En el cuadrante `{x ≤ 0, y ≥ 2}`: `d₁(X,P) = −x + y` y
  `d₁(X,Q) = (2−x) + (y−2) = −x + y`. **Idénticas.** Todo ese cuadrante cerrado
  es equidistante.
- Por simetría central, todo `{x ≥ 2, y ≤ 0}` también lo es.

Es decir: cuando el vector `Q−P` es diagonal (`|Δx| = |Δy|`), la «mediatriz»
tiene **medida de Lebesgue infinita**, no cero. Dos consecuencias que no son
cosméticas:

1. **El diagrama de Voronoi L₁ no es una partición del plano** en células con
   frontera de medida nula. Para que lo sea hay que **añadir una convención de
   desempate** (por ejemplo, orden de índice), y esa convención es información
   que no está en la geometría: dos implementaciones correctas pueden dar
   diagramas distintos. En L₂ eso no puede pasar: la mediatriz es una recta, sin
   excepciones, para todo par de puntos distintos.
2. El caso degenerado no es un caso de medida nula *en el espacio de entradas de
   interés*: los datos con coordenadas enteras o sobre rejilla — que son
   precisamente los datos donde L₁ se usa (VLSI, rutas urbanas) — están llenos de
   pares con `|Δx| = |Δy|`. La degeneración es el caso típico, no el patológico.

**Valoración frente a la ganancia en K(F).** En esta familia K(F) *sube* al pasar
a L₁ (una recta es más simple que una polilínea de tres tramos más una
convención), de modo que aquí ni siquiera hay que ponderar nada: L₁ pierde por
las dos varas. Es el contraejemplo más limpio a H₁ dentro de la línea B.

### 1.6 La trampa de las rutas mínimas, explícita

Entre `(0,0)` y `(m,n)` sin obstáculos, **toda** curva monótona en las dos
coordenadas tiene longitud L₁ exactamente `m+n`. El conjunto de geodésicas es:

- un **continuo** (dimensión infinita) en el plano;
- restringido a la rejilla, `C(m+n, n)` caminos distintos, todos óptimos.

```bash
target/release/dz-cli.exe eval "20"  --input-base 10 --mode fixed   # C(6,3)
# 18
target/release/dz-cli.exe eval "924" --input-base 10 --mode fixed   # C(12,6)
# 650
```

`C(6,3) = 18₁₂` geodésicas de rejilla entre `(0,0)` y `(3,3)`;
`C(12,6) = 650₁₂` entre `(0,0)` y `(6,6)`. En L₂ hay **una**.

**Cómo lo peso contra la ganancia de §1.4 nº 11.** No se cancelan, porque
responden a preguntas distintas, y conviene decir cuál:

- si la pregunta es **«¿cuál es la distancia?»**, L₁ gana sin matices: el valor
  es un entero y la comparación de dos rutas es exacta, mientras en L₂ ni se
  sabe acotar la precisión necesaria;
- si la pregunta es **«¿cuál es la ruta?»**, L₁ **no tiene respuesta**, tiene una
  familia de respuestas, y elegir una exige un criterio ajeno a la métrica
  (menos giros, lexicográfico, etc.). En L₂ la pregunta está bien puesta.

Y hay un caso donde esto muerde: la ruta L₁ **no depende continuamente de los
extremos**. Mover `Q` cruzando la diagonal salta de una familia de geodésicas a
otra. Cualquier algoritmo cuya salida sea *una* ruta es discontinuo en los datos.
Es el mismo defecto que en la regresión LAD (§1.4 nº 14) y en la mediana
geométrica con `n` par: **la no unicidad de L₁ es un fenómeno único con tres
caras**, y viene de que las bolas L₁ tienen caras planas. No es accidental ni
reparable: es lo que significa que la bola unidad sea un politopo.

---

## 2. Pregunta 4 — la elipse L₁

### 2.1 Enunciados y la simplificación

**L₂.** `√((x−c)²+y²) + √((x+c)²+y²) = 2a`, con `a > c ≥ 0`. Cuádrica;
`x²/a² + y²/b² = 1` con `b² = a²−c²`.

**L₁.** `|x−c| + |y| + |x+c| + |y| = 2a`, que se simplifica a

```
|x−c| + |x+c| + 2|y| = 2a
```

La simplificación de `planteamientos.md` es correcta y merece señalarse por qué:
**la coordenada `y` se separa de la `x`**. El lado izquierdo es `g(x) + 2|y|` con
`g(x) = |x−c| + |x+c|`, una suma de una función de `x` y una de `y`. Es el mismo
mecanismo de separabilidad que hace funcionar la mediana geométrica (§1.4 nº 8),
y es el único mecanismo genuinamente productivo que he encontrado en toda la
línea B. En L₂ los dos radicales acoplan `x` e `y` de forma irreducible.

### 2.2 El lugar es un hexágono — y la degeneración exacta

`g(x) = |x−c| + |x+c|` es lineal por tramos con tres tramos:

```
g(x) =  −2x   si x ≤ −c
         2c   si −c ≤ x ≤ c      ← constante
         2x   si x ≥ c
```

Resolviendo `g(x) + 2|y| = 2a` por tramos (`h := a − c > 0`):

| tramo | ecuación | lugar |
|---|---|---|
| `\|x\| ≤ c` | `2c + 2\|y\| = 2a` ⟹ `\|y\| = h` | **dos segmentos horizontales** `y = ±h`, `x ∈ [−c, c]` |
| `x ≥ c` | `x + \|y\| = a` | dos segmentos de `(c, ±h)` a `(a, 0)` |
| `x ≤ −c` | `−x + \|y\| = a` | dos segmentos de `(−c, ±h)` a `(−a, 0)` |

**La elipse L₁ de focos `(±c, 0)` es el hexágono convexo de vértices**

```
(a, 0), (c, h), (−c, h), (−a, 0), (−c, −h), (c, −h)      con h = a − c
```

**La degeneración, con precisión.** El plan pide caracterizarla exactamente, así
que la enuncio como proposición:

> Sobre la banda `|x| ≤ c`, la suma de distancias L₁ a los dos focos **no
> depende de `x`**: vale `2c + 2|y|` para todo `x` de la banda. En consecuencia,
> el lugar contiene los dos segmentos horizontales `y = ±(a−c)` con
> `x ∈ [−c, c]`, de longitud euclídea `2c` cada uno, y en esos segmentos la
> función «suma de distancias a los focos» tiene **gradiente nulo en la
> dirección `x`**.

Lo que se pierde, nombrado:

1. **La propiedad focal desaparece en esos segmentos.** En L₂ la elipse es el
   lugar de los puntos donde la suma de distancias crece estrictamente al salir;
   la normal biseca el ángulo focal (ley de reflexión: todo rayo desde un foco
   pasa por el otro). En L₁ los puntos interiores de los segmentos horizontales
   son **puntos críticos degenerados** de la función focal: moverse
   horizontalmente no cambia nada. No hay reflexión focal que enunciar.
2. **Los focos dejan de ser dos puntos distinguidos.** Sobre la banda, *todo*
   punto `(t, 0)` con `|t| ≤ c` produce la misma suma que cualquier otro — el
   par de focos `(±c, 0)` da el mismo hexágono que… no, aquí hay que ser
   preciso: focos distintos dan hexágonos distintos (`h` y la anchura cambian),
   pero **la función focal no separa los puntos de la banda**, y eso es lo que
   destruye la geometría de los focos.
3. **El caso límite `c = a`** (`h = 0`) da el segmento `[−a, a] × {0}` recorrido,
   igual que en L₂. Aquí L₁ y L₂ coinciden en degenerar, así que este límite no
   es un defecto de L₁.
4. **`c = 0`** da el rombo `|x| + |y| = a`: el «círculo» L₁. Continuidad correcta
   con la línea A.

### 2.3 Las otras dos cónicas — donde la degeneración se vuelve catastrófica

#### Hipérbola L₁

`| d₁(P,F₁) − d₁(P,F₂) | = 2a` con `F± = (±c, 0)`, `0 < a < c`:

```
| (|x−c| + |y|) − (|x+c| + |y|) | = 2a   ⟹   | |x−c| − |x+c| | = 2a
```

**El `|y|` se cancela por completo.** La ecuación no contiene `y`. Y
`|x−c| − |x+c| = −2x` en `|x| ≤ c`, y `∓2c` fuera. Por tanto:

- si `a < c`: el lugar es `2|x| = 2a`, es decir **las dos rectas verticales
  completas `x = ±a`**. No hay ramas, no hay asíntotas, no hay curvatura, y el
  lugar es **no acotado en la dirección `y` para cada `x` fijo** — lo contrario
  de una hipérbola, cuyas ramas son gráficas sobre `x`;
- si `a = c`: para `x > c` la condición se cumple **idénticamente**, así que el
  lugar es la unión de los dos semiplanos cerrados `{x ≥ c} ∪ {x ≤ −c}`: un
  conjunto de **dimensión 2**;
- si `a > c`: el lugar es **vacío**.

Esto es, con diferencia, la peor degeneración de la línea B. La hipérbola L₂ es
una curva de género 0 con dos ramas, asíntotas, excentricidad y propiedad focal;
su análoga L₁ es «dos rectas», «dos semiplanos» o «nada», según un umbral. Se ha
vuelto lineal por tramos —formalmente el éxito que pide la pregunta 4— y en el
proceso **ha dejado de contener información**: no se puede recuperar `c` del
lugar (cualquier `c > a` da el mismo par de rectas). La aplicación
`(a, c) ↦ lugar` no es inyectiva. Eso es información destruida, no simplificada.

#### Parábola L₁

Foco `F = (0, p)`, directriz `y = −p` (`p > 0`). Con la distancia punto-recta de
§1.3(b), `d₁(P, \{y=−p\}) = |y+p| / max(0,1) = |y+p|`:

```
|x| + |y − p| = |y + p|
```

- `y ≥ p`: `|x| + y − p = y + p` ⟹ **`|x| = 2p`**: dos rayos verticales.
- `−p ≤ y ≤ p`: `|x| + p − y = y + p` ⟹ **`y = |x|/2`**, válido para `|x| ≤ 2p`.
- `y < −p`: `|x| = −2p`, **vacío**.

La parábola L₁ es la «V» `y = |x|/2` entre `(−2p, p)` y `(2p, p)`, prolongada por
dos rayos verticales hacia arriba. Es la frontera de un conjunto convexo, es
lineal por tramos, y —a diferencia de la hipérbola— **sí determina `p`** (el
vértice de la V está en el origen y los rayos están en `x = ±2p`). Éxito sin
catástrofe, aunque con una pérdida específica: la parábola L₂ no tiene rectas
contenidas, y esta tiene dos rayos rectos, en los que la propiedad
foco-directriz se satisface con **holgura degenerada** (la igualdad se cumple
para todo `y ≥ p`, otra vez un tramo de gradiente nulo, igual que en la elipse).

### 2.4 Pérdida de invariancia rotacional, medida sobre el mismo objeto

La elipse L₂ solo depende de `a` y `c`: girar los focos gira el lugar y da una
figura congruente. En L₁ no. Con focos en la **diagonal**, `F± = (±c, ±c)`:

```
|x−c| + |y−c| + |x+c| + |y+c| = 2a   ⟹   g(x) + g(y) = 2a
```

con la misma `g` de §2.2. El lugar es ahora un **octógono** (nivel de una suma
separable de dos funciones de tres tramos), no un hexágono. Un hexágono y un
octógono **no son congruentes**: la misma definición focal, los mismos `a` y `c`,
y dos figuras con distinto número de lados según cómo estén orientados los focos.

Y la degeneración empeora: en `|x| ≤ c` **y** `|y| ≤ c` se tiene
`g(x) + g(y) = 4c`, constante. Si `2a = 4c`, es decir **`a = 2c`**, el lugar
contiene el cuadrado completo `[−c, c]²`, de área `4c²`: un lugar de dimensión 2.

```bash
target/release/dz-cli.exe eval "2*2" --mode fixed     # a = 2c con c = 2
# 4
```

Con `c = 2` y `a = 4`, todo el cuadrado `[−2,2]²` es «la elipse». En L₂ no existe
ninguna combinación de `a` y `c` que produzca un lugar de área positiva.

**Éste es el precio central de la pregunta 4**: en L₁ una cónica no es un objeto
determinado por sus invariantes métricos, sino por sus invariantes métricos **más
su orientación respecto de los ejes**, y para ciertas combinaciones colapsa a un
conjunto de interior no vacío. La razón es una y la misma de §1.1: el grupo
lineal de isometrías es `D₄`, finito, así que «la orientación» es un parámetro
real que no se puede eliminar.

### 2.5 K(F) — pregunta 4

#### (a) Perímetro de la elipse — la ganancia más grande de la línea B

**L₂.** No existe forma cerrada elemental: `P = 4a·E(e)`, integral elíptica
completa de segunda especie. Para poder contar algo uso la aproximación II de
Ramanujan, `P ≈ π[3(a+b) − √((3a+b)(a+3b))]`, que es lo mejor que se escribe con
símbolos elementales — y sigue siendo **aproximada**.

**L₁.** Longitud del hexágono medida en la propia métrica L₁: cada lado inclinado
va de `(a,0)` a `(c,h)`, con longitud L₁ `(a−c) + h = 2h`; hay cuatro. Cada
segmento horizontal mide `2c`; hay dos. Total:

```
P₁ = 8h + 4c = 8(a−c) + 4c = 8a − 4c
```

Comprobaciones: `c = 0` ⟹ `8a` (perímetro del rombo, coincide con la línea A);
`c = a` ⟹ `4a` (el segmento `[−a,a]` recorrido dos veces). Correcto en ambos
límites.

| | N_raíces | N_irrac | N_oper | N_per (dec) | N_per (doz) | K (dec) | K (doz) | exacta |
|---|--:|--:|--:|--:|--:|--:|--:|---|
| L₂ (Ramanujan II) | 1 | 1 (π) | 9 | 0 | 0 | **11** | **11** | no |
| L₁ `8a − 4c` | 0 | 0 | 3 | 0 | 0 | **3** | **3** | **sí** |

```bash
# a = 3, c = 2 (dozenal nativo)
target/release/dz-cli.exe eval "8*3-4*2" --mode fixed --gates
# 14
# gates: cycle=22044 min=35026 max=26159 lit=18132 thr=4608 total=105969 depth=628 recon=0
```

`P₁ = 14₁₂` (veinte, decimal) con 105.969 puertas, exacto. El lado L₂ no admite
una comparación de puertas honesta porque `dz-sci` no implementa `E(e)`; decir
«infinito» sería retórica. Lo que puedo afirmar sin instrumento es lo cualitativo:
**la forma cerrada elemental no existe en L₂ y existe en L₁**, y eso no depende de
ninguna implementación.

Casilla **(II)**. La base 12 no aporta nada: las constantes `8` y `4` son enteras
y terminan en toda base. `K(dec) = K(doz)`.

#### (b) Área de la elipse

Área del hexágono (rectángulo central `2c × 2h` más dos triángulos de área `h²`):
`4ch + 2h² = 2h(2c+h) = 2(a−c)(a+c)`, es decir

```
A₁ = 2(a² − c²)
```

Bonito: `A₁ = 2b²` con el **mismo** `b² = a² − c²` de L₂, y con `c = 0` da `2a²`,
el área del rombo de la línea A. La constante `π ↦ 2` para áreas se mantiene en
la elipse, no solo en el círculo. Eso sí es un hallazgo de casilla (II) con
contenido: la sustitución `π ↦ 2` para áreas es **estable bajo la deformación
focal**, no un accidente del caso isótropo.

| | N_raíces | N_irrac | N_oper | N_per (dec) | N_per (doz) | K (dec) | K (doz) |
|---|--:|--:|--:|--:|--:|--:|--:|
| L₂ `π·a·√(a²−c²)` | 1 | 1 (π) | 5 | 0 | 0 | **7** | **7** |
| L₁ `2(a²−c²)` | 0 | 0 | 4 | 0 | 0 | **4** | **4** |

```bash
target/release/dz-cli.exe eval "2*(3^2-2^2)" --mode fixed --gates
# X
# gates: cycle=40128 min=67388 max=49974 lit=34212 thr=8304 total=200006 depth=1002 recon=0
target/release/dz-cli.exe eval "pi*3*sqrt(5)" --mode fixed --scale 8 --gates
# ~19;0X87811702755X53
# gates: cycle=3636204 min=5722130 max=4246033 lit=2909796 thr=671040 total=17185203 depth=15168 recon=0
target/release/dz-cli.exe eval "sqrt(5)" --mode fixed --scale 8
# ~2;29EE1325
```

`A₁ = X₁₂` (diez) exacto con 200.006 puertas, frente a `A₂ = ~19;0X878117…`
truncado con 17.185.203: factor ≈ 86. Nótese que `pi` en sí cuesta 0 puertas por
memoización (`SKILL.md` §5; `02-referencia-medida.md` §2 lo registra con
`depth=15168`), así que **el coste L₂ es todo del `sqrt`**, no de π.

#### (c) La razón de Steiner rectilínea — el único lugar donde la base 12 paga

Familia nº 10 de §1.4, aislada aquí porque es el **único** caso de la línea B
donde `N_periodicos` se mueve.

| | forma | N_raíces | N_irrac | N_oper | N_per (dec) | N_per (doz) | K (dec) | K (doz) |
|---|---|--:|--:|--:|--:|--:|--:|--:|
| L₂ | `√3/2`, **conjeturada** | 1 | 1 | 1 | 0 | 0 | **3** | **3** |
| L₁ | `2/3`, **teorema** | 0 | 0 | 1 | 1 | **0** | **2** | **1** |

```bash
target/release/dz-cli.exe eval "2/3" --mode fixed --gates
# 0;8
# gates: cycle=88704 min=81360 max=65028 lit=50208 thr=16848 total=302148 depth=1312 recon=0
target/release/dz-cli.exe eval "sqrt(3)/2" --mode fixed --scale 8 --gates
# ~0;X485XX
# gates: cycle=4110876 min=6552834 max=4855641 lit=3319308 thr=759108 total=19597767 depth=13452 recon=0
```

`2/3 = 0;8` **exacto, un solo dígito dozenal**, 302.148 puertas. En base 10 sería
`0,666…`, periódico — el caso «gana base 12» de la tabla de recíprocos de
`02-referencia-medida.md` §1 (`1/3 = 0;4`), aplicado a `2/3`.
`√3/2 = ~0;X485XX`, irracional en cualquier base, 19.597.767 puertas
(factor ≈ 65); coherente con `√3 = ~1;894E98` de `02-referencia-medida.md` §2.

Aquí conviven los dos factores de H₁ y se pueden separar limpiamente:

- el paso de `√3/2` (irracional, conjeturado) a `2/3` (racional, demostrado) es
  **(II)**: geometría, no representación;
- el paso de `0,666…` a `0;8` es **(III)**: representación, no geometría. Un
  dígito frente a una expansión infinita.

Es el único punto de la línea B donde puedo decir que la base 12 aporta algo
observable, y aporta exactamente lo que P5 predijo que aportaría: `N_periodicos`.

### 2.6 El efecto (III), medido: casi siempre es cero

Las fórmulas L₁ de esta línea tienen constantes `2`, `4`, `8`, `1/2`, `2/3`.
Todas menos la última son dígitos sueltos o enteros. Comprobación de que la base
de entrada no cuesta nada cuando los literales son de un dígito:

```bash
target/release/dz-cli.exe eval "8*3-4*2" --input-base 10 --mode fixed --gates
# 14
# gates: ... total=105969 depth=628 recon=0        <-- idéntico
target/release/dz-cli.exe eval "8*3-4*2" --mode fixed --gates
# 14
# gates: ... total=105969 depth=628 recon=0
```

**Exactamente el mismo conteo**, porque los dígitos `0`-`9` sueltos son el mismo
símbolo en las dos bases y Horner cortocircuita (`docs/10`). Con literales de dos
cifras sí hay diferencia, y es pequeña:

```bash
target/release/dz-cli.exe eval "8*25-4*17" --input-base 10 --mode fixed --gates
# E0
# gates: cycle=36828 min=58758 max=43834 lit=30324 thr=7644 total=177388 depth=628 recon=0
target/release/dz-cli.exe eval "8*21-4*15" --mode fixed --gates
# E0
# gates: cycle=35772 min=57494 max=42868 lit=29628 thr=7452 total=173214 depth=628 recon=0
target/release/dz-cli.exe eval "25" --input-base 10 --mode fixed   # 21
target/release/dz-cli.exe eval "17" --input-base 10 --mode fixed   # 15
target/release/dz-cli.exe eval "4174/173214" --input-base 10 --mode fixed --scale 6
# 0;0357823E52X01E41667EE916 (periodica en base 12, valor truncado)
```

4.174 puertas de diferencia sobre 173.214, `≈ 0;0357…₁₂` (un 2,4 %). Es el coste
de **leer** dos literales decimales de dos cifras, no una propiedad geométrica, y
es consistente con el diferencial tabulado en `02-referencia-medida.md` §4
(1.784 puertas por un literal decimal de dos cifras: aquí hay dos, `25` y `17`,
y `2 × 1.784 = 3.568`, del orden de los 4.174 observados; la diferencia restante
la aporta que el acumulador de Horner ya no arranca en cero).

Y el dato de profundidad de esa misma referencia refuerza el punto desde otro
ángulo: leer dozenal tiene profundidad **constante** (254) sea el número de una
cifra o de cinco, porque la posición en el `Vec` ya *es* el peso, mientras Horner
en base 10 es secuencial y su profundidad crece con las cifras. Es la mejor
versión del efecto (III) que existe en el repositorio — y aun así **no toca
ninguna fórmula de esta línea**, porque las constantes L₁ son `2`, `4` y `8`.

**Conclusión (III) de la línea B:** el efecto base 12 aparece una vez
(`2/3 → 0;8`), retrocede una vez (`9/5` periódico en base 12 y finito en base 10,
§1.3b), y en el resto es idénticamente nulo porque las constantes L₁ son enteras.
Esto **apoya P5** desde una línea que no era la encargada de ponerla a prueba.

---

## 3. Pregunta 5 — trigonometría taxicab, y la prueba de P3

### 3.1 Definición

Sobre el rombo unidad `|x| + |y| = 1`, cuyo perímetro L₁ es `8` (línea A):

> **t-ángulo.** `θ ∈ [0, 8)` es la longitud de arco **L₁** desde `(1,0)` en
> sentido antihorario. `cos₁θ` y `sin₁θ` son las coordenadas del punto así
> alcanzado.

En el primer cuadrante el punto es `(1−t, t)` con `t ∈ [0,1]`, y su longitud de
arco L₁ desde `(1,0)` es `t + t = 2t`. Luego `θ = 2t`, y

```
cos₁θ = 1 − θ/2 ,   sin₁θ = θ/2        para 0 ≤ θ ≤ 2
```

Extendiendo por las simetrías de `D₄`:

> **`sin₁` es la onda triangular** de período `8`, amplitud `1`, con
> `sin₁(0)=0`, `sin₁(2)=1`, `sin₁(4)=0`, `sin₁(6)=−1`.
> **`cos₁θ = sin₁(θ+2)`**, la misma onda desplazada.

Ésa es toda la construcción. Conviene decirlo sin adorno: las funciones
trigonométricas taxicab **son la onda triangular**, un objeto elemental y
perfectamente conocido.

Valores, del binario:

```bash
target/release/dz-cli.exe eval "1/2" --mode fixed     # sin1(1) = 1/2
# 0;6
target/release/dz-cli.exe eval "2/3" --mode fixed     # sin1(4/3) = 2/3
# 0;8
```

`sin₁(1) = 0;6₁₂`, `sin₁(4/3) = 0;8₁₂`. Todos los valores de `sin₁` en t-ángulos
racionales son racionales con denominador potencia de 2 por el denominador del
ángulo — no hay una sola irracionalidad en la tabla. Contraste con `sin(π/3) =
√3/2`. Casilla (II), y es la observación que más a favor juega de L₁ en esta
pregunta.

### 3.2 La identidad pitagórica: qué la sustituye, y por qué eso no vale nada

```
|cos₁θ| + |sin₁θ| = 1        para todo θ
```

Cierta, exacta, de grado 1 en vez de 2. Y hay que decir lo siguiente en voz alta,
porque es la trampa conceptual de la pregunta 5:

> `sin²θ + cos²θ = 1` **no es un teorema sobre las funciones trigonométricas**.
> Es la ecuación de la circunferencia unidad, `x² + y² = 1`, escrita con la
> parametrización. Por construcción, la «identidad pitagórica» de *cualquier*
> métrica es la ecuación de su bola unidad, así que se traslada siempre y su
> traslado no contiene información.

Verificación de que `K` refleja esto:

| | forma | N_raíces | N_irrac | N_oper | N_per (dec) | N_per (doz) | K (dec) | K (doz) |
|---|---|--:|--:|--:|--:|--:|--:|--:|
| L₂ | `cos²θ + sin²θ = 1` | 0 | 0 | 3 | 0 | 0 | **3** | **3** |
| L₁ | `\|cos₁θ\| + \|sin₁θ\| = 1` | 0 | 0 | 3 | 0 | 0 | **3** | **3** |

**K no baja.** Dos potencias y una suma frente a dos valores absolutos y una
suma. El caso ejemplar de «L₁ no baja K», que el plan §2.1 pedía buscar
activamente: aquí está, y en la fórmula que la pregunta 5 proponía como
candidata estrella.

El contenido no trivial de la trigonometría L₂ no está en la identidad
pitagórica, está en la **ley de adición**. Y eso es lo que examina P3.

### 3.3 P3 — demostración de que no hay fórmula de adición

**Fallo del enunciado clásico.** Con `α = β = 1`: `sin₁(2) = 1`, mientras
`sin₁(1)cos₁(1) + cos₁(1)sin₁(1) = ½·½ + ½·½ = ½ ≠ 1`. Refutado por un solo
caso.

Pero un contraejemplo solo mata *esa* fórmula. P3 afirma algo más fuerte: que
**ninguna** fórmula de adición no trivial puede existir. Eso se demuestra.

> **Proposición.** No existe una familia de matrices `M(β) ∈ GL(2,ℝ)`,
> `β ∈ ℝ`, tal que
> `(cos₁(α+β), sin₁(α+β))ᵗ = M(β)·(cos₁α, sin₁α)ᵗ` para todos `α, β`.
>
> *Demostración.* Supongamos que existe. Poniendo `α = 0`:
> `M(β)·(1,0)ᵗ = (cos₁β, sin₁β)ᵗ`, así que `β ↦ M(β)` es continua y no constante.
> Por otra parte, el t-ángulo es longitud de arco L₁, de modo que el
> desplazamiento `α ↦ α+β` es una biyección del rombo unidad en sí mismo que
> preserva la longitud de arco L₁. Si esa biyección es la restricción de la
> aplicación lineal `M(β)`, entonces `M(β)` lleva el rombo al rombo preservando
> la métrica L₁, es decir `M(β) ∈ O(ℝ², ‖·‖₁)`. Pero el grupo de isometrías
> lineales de `(ℝ², ‖·‖₁)` es el grupo diédrico `D₄`, de **orden 8** (las
> simetrías del rombo). Un grupo finito no contiene una familia continua no
> constante. Contradicción. ∎

**P3 queda confirmada, y confirmada por la razón exacta que P3 alegaba**: la
rotación no es isometría de L₁, el grupo es finito de orden 8. La predicción
registrada acertó tanto en la conclusión como en el mecanismo.

Corolario, porque es donde se ve el alcance: lo que la ley de adición *es*, en
L₂, es la afirmación de que `θ ↦ (cos θ, sin θ)` es un **homomorfismo de grupos**
`ℝ → SO(2)`. Todo lo que sigue de ahí —fórmulas de ángulo doble, `e^{iθ}`,
`y'' = −y`, Fourier, el teorema de adición de las funciones elípticas— cuelga de
que `SO(2)` sea un grupo de Lie de dimensión 1. En L₁ ese grupo es `D₄`, y
`D₄ ≅ ℤ/4 ⋊ ℤ/2` no tiene más subgrupos continuos que el trivial. **Casilla (I),
en el sentido más fuerte del plan: no es que el análogo sea otra cosa; es que no
hay dónde alojarlo.**

**¿Sobrevive *algo* de tipo aditivo?** Sí, y es instructivo ver qué: si `α`, `β`
y `α+β` caen en el **mismo tramo lineal**, entonces
`sin₁(α+β) = sin₁α + sin₁β` (Cauchy), porque en ese tramo `sin₁` es lineal. Es
decir, la ley de adición no se deforma en otra ley trigonométrica: **degenera en
la aditividad de una función lineal**, localmente, y falla en cuanto se cruza un
vértice del rombo. Eso no es una identidad trigonométrica: es la constatación de
que estamos mirando una recta a trozos.

### 3.4 Inventario: qué sobrevive y qué muere

| propiedad | L₂ | L₁ | casilla |
|---|---|---|---|
| identidad de la bola unidad | `cos²+sin²=1` | `\|cos₁\|+\|sin₁\|=1` | (II), vacía (§3.2) |
| periodicidad | `2π` | **`8`**, racional | (II) |
| paridad | `cos` par, `sin` impar | igual | (II) |
| desfase cofuncional | `cos θ = sin(θ+π/2)` | `cos₁θ = sin₁(θ+2)` | (II) |
| reflexión | `sin(π−θ)=sinθ` | `sin₁(4−θ)=sin₁θ` | (II) |
| valores en ángulos racionales | irracionales (`√2/2`, `√3/2`) | **racionales siempre** | (II), a favor de L₁ |
| suma de ángulos de un triángulo | `π` | **`4`** (§3.5) | (II) |
| **fórmula de adición** | `sin(α+β)=…` | **no existe** (§3.3) | **(I)** |
| ángulo doble | `sin 2θ = 2 sinθ cosθ` | falla: en `[0,1]`, `sin₁(2θ)=θ` frente a `2 sin₁θ cos₁θ = θ − θ²/2` | **(I)** |
| `sin' = cos` | sí | **no**: `sin₁' = ½·`(onda **cuadrada**), que no está en el espacio generado por `{sin₁, cos₁}` | **(I)** |
| `y'' = −y` | sí | **no** (`sin₁'' = 0` en el interior de cada tramo + deltas en los vértices) | **(I)** |
| ley del seno | sí | **no** (§3.5) | **(I)** |
| ley del coseno | sí | **no** (§3.5) | **(I)** |
| congruencia SSS / SAS | sí | **no** (§3.5) | **(I)** |

La fila de la derivada merece un comentario, porque conecta con la línea C: la
derivada de la onda triangular es la onda **cuadrada**, que no es ni `sin₁` ni
`cos₁` ni combinación lineal de ellas. El par `(cos₁, sin₁)` **no es cerrado bajo
derivación**, y por tanto no resuelve ningún sistema lineal de primer orden, y
por tanto no proviene de ninguna exponencial de matrices. Ahí muere, de raíz, toda
la posibilidad de una «exponencial compleja L₁».

### 3.5 Suma de ángulos: sobrevive. Leyes del seno y del coseno: no.

**Suma de ángulos = 4.** Verificado en tres triángulos y con razón estructural:

| triángulo | lados L₁ | t-ángulos | suma |
|---|---|---|--:|
| `(0,0),(1,0),(0,1)` | `1, 1, 2` | `2, 1, 1` | **4** |
| `(0,0),(2,0),(1,1)` | `2, 2, 2` | `1, 1, 2` | **4** |
| `(0,0),(3,0),(1,1)` | `3, 2, 3` | `1, 2/3, 7/3` | **4** |

```bash
target/release/dz-cli.exe eval "2/3" --mode fixed        # 0;8
target/release/dz-cli.exe eval "7/3" --mode fixed        # 2;4
target/release/dz-cli.exe eval "1+2/3+7/3" --mode fixed --gates
# 4
# gates: cycle=191400 min=182660 max=145105 lit=111036 thr=36540 total=666741 depth=1312 recon=0
```

Los t-ángulos del tercer triángulo son `1`, `0;8₁₂` y `2;4₁₂`, y su suma, hecha
por el binario, es exactamente `4`. Razón estructural: el rombo unidad es
**centralmente simétrico**, así que la longitud de arco de `u` a `−u` es la mitad
del perímetro, `4`, para toda dirección `u`; el argumento clásico del ángulo
exterior solo necesita eso. **Vale en cualquier plano de Minkowski**, con «`π`»
igual a la mitad del perímetro de su bola unidad. Casilla (II), coherente con la
línea A (`π ↦ 4` para perímetro).

**La ley del seno falla.** Triángulo `(0,0),(2,0),(1,1)`: lados L₁ `2, 2, 2`
—equilátero— y t-ángulos `1, 1, 2`.

```bash
target/release/dz-cli.exe eval "2/(1/2)" --mode fixed    # a / sin1(1)
# 4
target/release/dz-cli.exe eval "2/1"     --mode fixed    # c / sin1(2)
# 2
```

`a/sin₁A = 4` y `c/sin₁C = 2`. No hay razón común. (En el primer triángulo de la
tabla la razón sí es constante e igual a `2`, lo que muestra que la ley se cumple
en algunos triángulos y falla en otros: no es ley.)

*Nota de honestidad.* Recuerdo haber leído que en la literatura de geometría
taxicab (Thompson–Dray, «Taxicab angles and trigonometry», 2000) se afirma una
ley del seno taxicab. Mi contraejemplo me parece verificable a mano y no he
podido consultar la fuente en esta sesión, así que dejo constancia de la tensión:
**o su definición de t-ángulo o de longitud difiere de la mía, o la afirmación
lleva hipótesis que no he reproducido.** Lo que afirmo es únicamente lo que
verifico arriba, con la definición de §3.1.

**La ley del coseno no puede existir.** Prueba más fuerte que un contraejemplo
numérico: exhibo **dos triángulos con los mismos tres lados L₁ y distintos
t-ángulos**, con lo que ninguna función de los lados puede dar un ángulo.

| triángulo | vértices | lados L₁ | t-ángulos |
|---|---|---|---|
| T₁ | `(0,0), (2,0), (1,1)` | `2, 2, 2` | `1, 1, 2` |
| T₂ | `(0,0), (1;6, 0;6), (0;6, 1;6)` | `2, 2, 2` | `1, 1;6, 1;6` |

```bash
target/release/dz-cli.exe eval "1.5"  --input-base 10 --mode fixed   # 1;6
target/release/dz-cli.exe eval "0.5"  --input-base 10 --mode fixed   # 0;6
```

(Comprobación de T₂: `d₁(A,B) = 1;6 + 0;6 = 2`, `d₁(A,C) = 0;6 + 1;6 = 2`,
`d₁(B,C) = |1;6−0;6| + |0;6−1;6| = 1 + 1 = 2`. Equilátero L₁. T-ángulos
calculados con §3.1: en `A` las direcciones normalizadas L₁ son
`(0;9, 0;3)` y `(0;3, 0;9)`, con arcos `0;6` y `1;6`, luego `A = 1`; en `B` y en
`C` sale `1;6` cada uno. Suma `1 + 1;6 + 1;6 = 4` ✓.)

> **Corolario.** En L₁ los tres lados **no determinan** el triángulo salvo
> isometría. En consecuencia: no hay ley del coseno, **falla la congruencia
> SSS**, y «resolver un triángulo» no es un problema bien puesto.

Esto es un resultado de casilla (I) de primer orden, y es el que más daño hace a
la idea de «trigonometría taxicab»: una trigonometría es, esencialmente, el
diccionario entre lados y ángulos. En L₁ ese diccionario no existe, porque los
ángulos no son deducibles de la métrica. Y no podían serlo: el t-ángulo es
**estructura añadida** (una elección de parametrización del rombo), no algo que
la norma determine, mientras en L₂ el ángulo se lee del producto interno, que la
norma sí determina (identidad de polarización).

### 3.6 ¿Matemática nueva o reparametrización?

**Reparametrización.** El veredicto, con sus razones:

1. `cos₁` y `sin₁` **son** la onda triangular, conocida desde el siglo XIX y
   omnipresente en procesamiento de señal. No hay objeto nuevo.
2. Cada «identidad» superviviente (§3.4) es o la ecuación del lugar, o una
   simetría de `D₄` reescrita, o un hecho elemental de la onda triangular.
3. Lo que en L₂ es contenido —la ley de adición, y todo lo que de ella deriva—
   no se deforma: **desaparece**, por la Proposición de §3.3.
4. Ironía final, que me parece la mejor forma de cerrar la pregunta 5: la
   descripción **espectral** de la onda triangular es
   `sin₁(θ) = (8/π²) Σ_{k impar} (−1)^{(k−1)/2} k⁻² sin(kπθ/4)`. Es decir, en
   cuanto se le pide a la «trigonometría taxicab» algo analítico —su espectro—
   reaparecen `π`, `π²` y los senos euclidianos. El `π` que la métrica L₁
   expulsó de la geometría vuelve por la puerta del análisis. Los denominadores
   `k²` con `k` impar (`1, 9, 25, 49, …`) incluyen `25` y `49`, así que la serie
   no es más amable en base 12 que en base 10.

---

## 4. Veredictos

| pregunta | veredicto |
|---|---|
| **3** | **Confirma parcialmente H₁, con excepciones explícitas.** 12 familias se vuelven lineales por tramos, 7 no o empeoran. De las 12, solo 3 son ganancias estructurales (perímetro de la elipse, mediana geométrica, exactitud de rutas mínimas). El factor de coste medido es de dos a tres órdenes de magnitud, pero atribuible a `sqrt` en esta implementación, no a la geometría. La base 12 no aporta nada aquí. |
| **4** | **Confirma la forma, refuta el contenido.** Toda cónica L₁ es un politopo o polilínea: éxito rotundo en «lo aparentemente cuadrático se vuelve lineal por regiones». Pero la elipse contiene segmentos donde la función focal es constante, la hipérbola colapsa a dos rectas (o a dos semiplanos, o al vacío), la elipse de focos diagonales es un octógono no congruente con el hexágono, y con `a = 2c` el «lugar» es un cuadrado macizo. La ganancia en K es grande y real; la pérdida de información también. |
| **5** | **P3 confirmada. No hay matemática nueva.** `cos₁`/`sin₁` son la onda triangular; la identidad pitagórica se sustituye por `\|cos₁\|+\|sin₁\|=1`, que no baja K y no dice nada; no existe ninguna ley de adición lineal (demostrado); y además mueren la ley del seno, la ley del coseno y la congruencia SSS. Sobreviven periodicidad `8`, paridades, desfase cofuncional y suma de ángulos `= 4`. |

**Sobre P3 en particular, para que no se pueda releer a favor:** P3 se confirma en
los dos niveles en que se enunció —conclusión (no hay fórmulas de adición) y
mecanismo (`|Isom_lin(L₁)| = 8 < ∞`)— y resulta ser **más fuerte de lo que se
predijo**: no solo caen las fórmulas de adición, cae todo el diccionario
lado↔ángulo. P3 no necesita ser corregida; si acaso, ampliada.

---

## 5. Lo que no funcionó

Sección obligatoria. Diez puntos, ordenados de más a menos dañino para H₁.

1. **La bisectriz L₁ tiene área positiva, y el diagrama de Voronoi no es una
   partición.** Para `|Δx| = |Δy|` dos cuadrantes cerrados enteros son
   equidistantes (§1.5). L₂ nunca tiene este problema. Y el caso degenerado es el
   **típico** en datos sobre rejilla, que es donde L₁ se usa. Aquí `K` **sube** al
   pasar a L₁: no hay ni siquiera un intercambio que ponderar, es pérdida limpia.

2. **SSS no determina los ángulos** (§3.5, T₁ vs T₂: dos triángulos
   L₁-equiláteros de lado `2` con t-ángulos `(1,1,2)` y `(1, 1;6, 1;6)`). No hay
   ley del coseno, ni ley del seno, ni congruencia SSS. La trigonometría taxicab
   no es una trigonometría: es una parametrización del rombo con un nombre
   prestado.

3. **La hipérbola L₁ destruye información.** `| |x−c| − |x+c| | = 2a` no contiene
   `y`, y su lugar es «dos rectas verticales» (si `a<c`), «dos semiplanos» (si
   `a=c`) o «vacío» (si `a>c`). La aplicación `(a,c) ↦ lugar` no es inyectiva:
   del lugar no se recupera `c`. Es el peor caso de «más simple porque hay menos».

4. **La elipse L₁ es degenerada en toda una banda.** En `|x| ≤ c` la suma de
   distancias focales no depende de `x` (§2.2): el lugar contiene dos segmentos
   horizontales de longitud `2c` donde el gradiente focal se anula, y con ellos se
   va la propiedad de reflexión focal, que es *la* razón por la que la elipse
   euclídea le importa a alguien (espejos, órbitas, susurros).

5. **La base 12 no aparece.** En toda la línea B, `N_periodicos` se mueve **una
   sola vez** (`2/3 → 0;8`, §2.5c). Las constantes L₁ son `2`, `4`, `8`: enteras,
   terminan en cualquier base. Medido: `eval "8*3-4*2"` cuesta **105.969 puertas
   idénticas** leyendo en base 10 o en base 12, porque los dígitos sueltos son
   gratis. Con literales de dos cifras la diferencia es 4.174 puertas sobre
   173.214, un 2,4 %: coste de **lectura**, no simplificación estructural (y ese
   coste ya estaba tabulado en `02-referencia-medida.md` §4, así que no es un
   hallazgo mío sino la confirmación de que esta línea no añade nada al eje III).
   **P5 queda apoyada desde una línea que no era la encargada de probarla.**

6. **Y una vez la base 12 empeora las cosas.** La distancia punto-recta L₂ con
   `a=3, b=4` es `9/5`, **racional exacto** (no hay radical que L₁ pueda
   eliminar), y `9/5 = 1;9724972497…₁₂` es **periódico en base 12** y finito en
   base 10 — exactamente el `1/5 = 0;2497 2497…` de `02-referencia-medida.md` §1.
   Las ternas pitagóricas —donde L₂ ya es exacta— están llenas de 5, y 5 es justo
   el factor que la base 12 no digiere. La geometría analítica elemental es un mal
   terreno para el dozenal, no un buen terreno.

7. **La regresión L₁ es estrictamente peor.** Mínimos cuadrados:
   `(XᵀX)⁻¹Xᵀy`, forma cerrada racional, sin radicales, única, continua en los
   datos. Desviaciones absolutas mínimas: programa lineal, **sin forma cerrada**,
   solución no única, no continua en los datos. Es el contraejemplo más nítido a
   «lo cuadrático se vuelve lineal por tramos»: aquí lo lineal por tramos es lo
   que *no* tiene fórmula.

8. **La no unicidad de L₁ es un fenómeno único con tres caras**, y todas cuestan:
   geodésicas (`C(6,3) = 18₁₂`, `C(12,6) = 650₁₂` caminos óptimos de rejilla, y un
   continuo en el plano), bisectrices (§1.5), mediana geométrica con `n` par
   (rectángulo entero de óptimos), regresión LAD. La causa es una: **las bolas L₁
   tienen caras planas**. No es un defecto de las definiciones ni algo que una
   convención arregle; es lo que significa que la bola unidad sea un politopo. Y
   arrastra un corolario práctico serio: cualquier algoritmo que devuelva *un*
   óptimo es **discontinuo en los datos de entrada**.

9. **El par `(cos₁, sin₁)` no es cerrado bajo derivación.** `sin₁' = ½·`onda
   cuadrada, que no está en el generado de `{sin₁, cos₁}`. Sin sistema lineal de
   primer orden, sin `y''=−y`, sin exponencial de matrices, sin `e^{iθ}`. Aviso
   anticipado a la línea C: el fracaso de Fourier bajo L₁ (**P4**) ya se ve desde
   aquí, y por la misma razón de orden 8.

10. **La identidad pitagórica no baja K** (§3.2: `3` frente a `3`). El caso que la
    pregunta 5 ofrecía como candidato estrella es exactamente uno de los que el
    plan §2.1 pedía buscar: L₁ no gana. Y peor: la identidad que se «gana» es
    tautológica, porque `sin²+cos²=1` nunca fue un teorema sobre funciones, sino
    la ecuación de la bola unidad reescrita — así que se traslada a toda métrica y
    su traslado es vacío en todas.

**Reserva metodológica propia, no imputable a H₁.** Los factores de coste de
§1.2, §1.3b, §2.5 (de ≈ 65× a ≈ 1.105×, éste último de
`02-referencia-medida.md` §3) miden `sqrt` tal como lo implementa
`dz-sci` (Newton con división restauradora) a una escala que yo elegí. No son
constantes geométricas y no deben citarse como tales. Lo que es independiente de
la implementación, y lo único que sostengo en ese eje, es la dicotomía
**exacto/truncado**: las fórmulas L₁ de esta línea dan enteros o racionales de
denominador `2^i·3^j`; las L₂ dan valores con `~`.
