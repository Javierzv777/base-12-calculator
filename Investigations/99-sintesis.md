# Síntesis — Geometría L₁ + representación dozenal

Cierre de la investigación abierta en `00-plan-investigacion.md` sobre la
hipótesis de `planteamientos.md`. Cuatro líneas (A constantes, B lugares,
C análisis, D dozenal) corrieron en paralelo contra el mismo protocolo, con el
calculador de puertas de este repositorio como instrumento. Los informes
completos están en `01-hallazgos-{A,B,C,D}-*.md`; las cifras compartidas y las
trampas del instrumento, en `02-referencia-medida.md`. Los documentos `03` a `16`
son extensiones posteriores al cierre, abiertas por preguntas del usuario sobre
hallazgos concretos; cada una está resumida en §4.5 a §4.16 (`14` en §4.16; ver
la nota de numeración más abajo).

Fecha de cierre: 2026-09-12. Última extensión incorporada: 2026-10-11 (`14`, el
exponente de irracionalidad de π afirmado por un preprint de `openai/math`,
empezado en otro equipo con `14-pi-exponent/HANDOFF.md`; resumido en §4.16
porque §4.14 ya estaba tomado). Antes, 2026-10-09 (`13`,
hipótesis de Riemann con la base 12 como enfoque; antes, `12`, Gödel, abierta
por un vídeo divulgativo). Ninguna de las dos pertenece a la lista de
"ingredientes" de `09`-`11`, que sigue siendo la abierta por la afirmación de
Kryon sobre π: ver §4.9-§4.11 para ese contexto y §4.12-§4.13 para éstas.
También 2026-09-25 (`15`, la frontera de la bola L₁ en n dimensiones, que
reescribe §3.4; `16`, el paseo aleatorio D₄ frente a D₆, que da a §3.1 un
mecanismo con contenido; resumidas en §4.14-§4.15). Se escribieron en otra
máquina antes que `12` y `13` y se renumeraron al integrarlas; en su commit
original se llamaban `12` y `13`. Antes, 2026-09-14 (`09`, `10` y `11`, la
lista de "ingredientes").
Revisión crítica aplicada: 2026-09-13 (ver §8); segunda corrección, 2026-09-25
(§3.4, §4.14 y §8).

---

## 1. Veredicto sobre H₁

**H₁ queda parcialmente confirmada y, sobre todo, confinada.**

Por el criterio de cierre del §5 del plan: hay más de tres familias con
ganancia en K y en coste medido, y al menos una ganancia no viene de
`N_periodicos` — así que la letra del criterio de «confirmada» se cumple. Pero
el criterio estaba mal escrito para lo que realmente importaba, y la
investigación produjo un resultado que lo desborda:

> **Los dos factores de H₁ son antagónicos.** La hipótesis pide que L₁ y la
> base 12 empujen en la misma dirección, y no lo hacen: empujan hacia
> particiones distintas.

La bola L₁ tiene grupo de simetrías lineales `D₄`, de orden 8, y regala
divisiones en **2, 4 y 8**. La base 12 regala divisiones exactas en
**2, 3, 4, 6 y 12**. Coinciden en 2 y 4; divergen exactamente donde cada una
es interesante — L₁ en el 8, base 12 en el 3, el 6 y el 12. Las familias de
problemas donde el 12 está *forzado* por la matemática y no por una convención
son todas triádicas o hexagonales (raíces duodécimas de la unidad, enteros de
Eisenstein, retículas hexagonales), y en 6 de las 8 que sobrevivieron a la poda
la columna L₁ está vacía o destruye la propiedad relevante: la geometría
hexagonal vive en L₂ o en retícula triangular, no en L₁, cuya bola es un
cuadrado girado.

Dicho sin rodeos: **la conjunción «L₁ + dozenal» no es una sinergia, es una
coincidencia de que ambos suenan a "más simple que lo estándar".** Cada factor
tiene su dominio de utilidad y los dominios casi no se solapan. Ésa es la
conclusión principal de esta investigación, y no estaba en la hipótesis de
partida.

Ese antagonismo admite una réplica obvia —cambiar la bola unidad por una cuya
simetría sí case con base 12— y se ensayó a fondo (§4.7). El resultado no
cambia el veredicto: el hexágono `D₆` consigue una constante racional y robusta
donde L₁ no podía, esa es la única constante invariante racional y robusta de
toda la familia, y la familia de n-ágonos regulares queda **clasificada por
completo**, no muestreada.

### Veredicto por factor, que es como hay que leerlo

| factor | veredicto | dónde |
|---|---|---|
| **L₁ simplifica** | **Sí, y a veces de forma no trivial** — 3 ganancias estructurales genuinas | B §recuento, A §5 |
| **L₁ es gratis** | **No.** Pierde unicidad con medida positiva, diferenciabilidad, invariancia rotacional y la estructura multiplicativa | B, C |
| **Base 12 simplifica** | **Sí, pero por un solo hecho**: `12 = 2²·3` | D §3 |
| **Base 12 es estructural** | **Casi no.** Un efecto real y medido, todo derivado de ese mismo hecho | D §3.5 |
| **La conjunción aporta** | **No demostrado, y con razón teórica para dudarlo** (antagonismo `D₄` vs `2²·3`) | D §transversal |

---

## 2. Las seis predicciones registradas

Se anotaron antes de investigar, justo para que el resultado no se pudiera
releer a favor. Balance: **cinco confirmadas, una caída en su cláusula
fuerte.**

| | predicción | veredicto |
|---|---|---|
| **P1** | no hay «π taxicab» único | **confirmada, y extendida a una forma más fuerte** — la constante depende de la propiedad conservada (4 para perímetro/diámetro, 2 para área, `4√3` para superficie en 3D) y §4.7 añade que entre los n-ágonos regulares solo el hexágono da una razón perímetro/diámetro racional **y** robusta (la forma «ninguno hace racionales las tres a la vez» cayó en la revisión: área y volumen de Lebesgue no son invariantes de la norma) |
| **P2** | L₁ y L∞ son las únicas `Lp` con razón entera | **confirmada** en lo esencial (A la matiza; ver su §4) |
| **P3** | la trigonometría L₁ pierde las fórmulas de suma | **confirmada, y más fuerte de lo predicho** |
| **P4** | Fourier bajo L₁ no da Plancherel ni se simplifica | **confirmada** |
| **P5** | el efecto dozenal se reduce a `N_periodicos` | **cae en la letra, sobrevive en el espíritu** |
| **P6** | Goldbach no cede nada geométrico | **confirmada**; el resultado es reformulación, no descubrimiento |

---

## 3. Los cuatro resultados que valen

De todo lo producido, esto es lo que sobreviviría a una revisión hostil.

### 3.1 L₂ no es una convención: la multiplicación de ℂ la selecciona (línea C)

Va **contra** la hipótesis en su forma ambiciosa. La fórmula de Gelfand aplicada
al módulo L₁ da

```
lim_{n→∞} |zⁿ|₁^{1/n} = |z|₂
```

Es decir: si se itera la multiplicación compleja midiendo con L₁, **L₂ reaparece
sola en el límite**. Pero conviene no sobrevenderlo: el límite es inmediato por
equivalencia de normas (`|·|₂ ≤ |·|₁ ≤ √2|·|₂`) y sale igual partiendo de
**cualquier** norma de ℂ, así que por sí solo no dice nada específico de L₁. El
enunciado que sí tiene contenido es el que el límite ilustra: `|·|₂` es el
**único valor absoluto multiplicativo** de ℂ que extiende el de ℝ (Ostrowski;
Gelfand–Mazur). No se puede «elegir» L₁ para el análisis complejo y esperar que
la geometría euclidiana se quede fuera; la estructura multiplicativa de ℂ la
selecciona. Es casilla (I) en su forma más dura — no «L₂ es cómoda», sino «L₂
está determinada» —, y el mérito es de ese teorema clásico, no del límite.

> **Ampliación 2026-09-25 (`16`).** Hay un segundo teorema de selección, esta vez
> aditivo y no trivial: por el TCL, la suma iterada de pasos de **cualquier**
> geometría con varianza finita converge a una forma **cuadrática**. Con una
> rotación de orden ≥ 3 en la simetría, esa forma es redonda (Teorema 1 de `16`).
> El paseo en Z², cuyos pasos son la esfera unidad L₁, converge a la gaussiana
> isótropa. π no está en ningún paso finito (`P(S₂ₙ=0) = (C(2n,n)/4ⁿ)²`,
> racional) y aparece en el límite (`n·P → 1/π`, Wallis). Ver §4.15.

La obstrucción de la que todo cuelga es más simple: `|zw|₁ ≠ |z|₁|w|₁`
(contraejemplo `z = w = 1+i`). Sin multiplicatividad no hay forma polar; sin
forma polar no hay mapa exponencial; y sin él se cae todo lo construido encima.
Hurwitz cierra la puerta por otro lado: `|x|+|y|` no es una forma cuadrática,
así que no existe `|z|² = z·z̄`.

### 3.2 La identidad pitagórica taxicab existe y no vale nada (línea B)

`|cos₁θ| + |sin₁θ| = 1` es el análogo de `sin²+cos² = 1`. Y el hallazgo es que
la pregunta estaba mal planteada desde el principio: **`sin²+cos² = 1` nunca fue
un teorema sobre funciones** — es la ecuación de la bola unidad, reparametrizada.
Por eso se traslada a cualquier métrica de forma vacua, y K(F) lo delata:
**3 contra 3, no baja**.

Lo que sí es un teorema —las fórmulas de adición— **no sobrevive, y no por
accidente**. B demostró la imposibilidad, no solo un contraejemplo: si existiera
`M(β)` con `(cos₁(α+β), sin₁(α+β)) = M(β)·(cos₁α, sin₁α)`, entonces `M(β)` sería
una isometría lineal de L₁ para todo β; pero `Isom_lin(L₁) = D₄` tiene orden 8,
y un grupo finito no aloja una familia continua no constante. Caen con ella la
ley del seno, la del coseno y la congruencia SSS (dos triángulos L₁-equiláteros
de lado 2 con t-ángulos `(1,1,2)` y `(1, 1;6, 1;6)`).

Cierre irónico y honesto: `cos₁`/`sin₁` son la onda triangular de período 8, y
su serie de Fourier **devuelve π y π²**.

### 3.3 Tres simplificaciones L₁ genuinas, y el precio exacto (línea B)

De 12 familias que se vuelven lineales por tramos, solo 3 escapan al sesgo
declarado de K(F) —es decir, no son simples *porque* K premia lo lineal, sino
porque resuelven algo que en L₂ no tiene forma cerrada:

| | en L₂ | en L₁ |
|---|---|---|
| perímetro de la elipse | integral elíptica, **sin forma cerrada** | `8a − 4c`, exacto |
| mediana geométrica | **no expresable por radicales** para n ≥ 5 | dos medianas independientes, por separabilidad |
| rutas mínimas | *sum-of-square-roots problem* (complejidad abierta) | comparación racional exacta en la rejilla de Hanan |

El precio, medido y no minimizado: la **no unicidad L₁ tiene medida positiva**.
La bisectriz de `(0,0)`–`(2,2)` contiene dos cuadrantes enteros, así que el
diagrama de Voronoi L₁ no es una partición sin una convención ajena a la
geometría — y el caso degenerado es el *típico* sobre datos en rejilla, no una
rareza. Ahí K **sube**: pérdida limpia. La elipse L₁ degenera hasta lo absurdo:
con los focos en la **diagonal**, `F± = (±c, ±c)`, y `a = 2c`, el «lugar
geométrico» es el cuadrado macizo `[−c, c]²` (`01-B` §2.4). Con los focos en un
eje y los mismos `a` y `c` sale un hexágono corriente: la degeneración depende de
la orientación, que es justo la pérdida de invariancia rotacional.

### 3.4 La racionalidad no desciende del cuerpo a su frontera (línea A)

El único caso donde L₁ **empeora** la fórmula, y por eso el más informativo:

| | fórmula | K |
|---|---|--:|
| L₂ | `S₂ = 4πr²` | **3** |
| L₁ | `S₁ = 4√3 r²` | **4** |

El octaedro `‖x‖₁ ≤ r` tiene 8 caras equiláteras de lado `√2 r`, luego su
superficie Hausdorff-2 es `4√3 r²`. Cambiar el trascendente π por el algebraico
`√3` no baja `N_irracionales` (sigue en 1) y sube `N_raíces` de 0 a 1. Enunciado
general: **el volumen de la bola L₁ es racional en toda dimensión, pero la
racionalidad no se hereda en la frontera.**

A se puso la objeción a sí misma, y conviene conservarla: las definiciones de
área de Busemann o Holmes–Thompson podrían normalizar esto a un racional; no se
exploraron; y elegir la definición que da el número bonito es exactamente el
modo de fallo que el plan prohíbe. Hausdorff es la única sin parámetro libre.

> **Corrección 2026-09-25 (`15`).** El enunciado general de esta sección cae
> leído sobre Hausdorff, que es como estaba escrito. La superficie de Hausdorff
> de la bola L₁ en dimensión `n` es `2ⁿ·√n·rⁿ⁻¹/(n−1)!`, **racional en `n = 4`**
> (`16/3`: `dz-cli eval "2^4*sqrt(4)/(3*2*1)" --mode rational` → `14/3`) y en
> toda `n` que sea cuadrado perfecto. El cubo L∞ de 3D tiene frontera `24r²`. Y
> Hausdorff sí tiene un parámetro libre, la inmersión euclidiana: el octaedro
> `{|x|+2|y|+2|z| ≤ 1}`, isométrico a `ℓ₁³`, tiene superficie **3**. El `√3` es
> la norma euclidiana de la normal `(1,1,1)`, no una propiedad de L₁; es la misma
> corrección que §4.7 hizo con el hexágono. Lo que sí se sostiene, y ahora como
> teorema, es la versión invariante: en `n ≥ 3`, Busemann y Holmes–Thompson de
> todo politopo racional están en `ω_{n−1}·ℚ` y `ℚ/ω_{n−1}`, siempre
> trascendentes. Pero eso no es de L₁: vale para todo politopo. Detalle en §4.14.

---

## 4. El factor dozenal, aislado (línea D)

### 4.1 P5 cae en la letra

P5 decía que no habría **ninguna** simplificación estructural atribuible a la
base. Es falso, y el contraejemplo está medido. En base 12 los recíprocos de
2, 3, 4 y 6 son exactos y de **un solo dígito** (`0;6 0;4 0;3 0;2`), así que
dividir se puede sustituir por multiplicar, con resultado **idéntico**:

```
$ ../target/release/dz-cli.exe eval "345/3" --mode fixed --gates
115;8   → total=448605  depth=1371
$ ../target/release/dz-cli.exe eval "345*0;4" --mode fixed --gates
115;8   → total=127360  depth=1153
```

**3,52× más barato, mismo valor.** La cifra está ligada a la escala de división
por defecto (`DEFAULT_DIV_SCALE = 24`, que `--scale` no mueve en modo fijo): la
división restauradora crece con los dígitos que produce y `× 0;4` no, así que el
factor no es una constante del circuito. Base 10 no puede hacer esto para el 3. Y la
divisibilidad por 2, 3, 4, 6 y 12 se decide en `O(1)` mirando el último dígito,
frente a `O(cifras)` para el 3 en decimal. Eso es estructura de circuito, no
ortografía.

### 4.2 ...y sobrevive en el espíritu

Las cuatro manifestaciones son consecuencia **del mismo hecho único**,
`12 = 2²·3`, y cada una tiene su gemela decimal para el 5. Base 12 no gana en
abstracto: gana en problemas con tercios y pierde en problemas con quintos.
La tabla de recíprocos lo dice en los dos sentidos —`1/3 = 0;4` termina,
`1/5` **no** termina en base 12 y sí en decimal— y las ternas pitagóricas, que
están llenas de cincos, son un dominio donde base 12 **pierde**.

Y **base 12 no eliminó ni un radical ni un irracional en ninguna de las ocho
familias**. Todo su efecto vive en `N_periodicos`, más el ahorro de circuito de
§4.1. P5 hay que reescribirla, no tacharla.

### 4.3 El lead más prometedor no es accesible desde el instrumento

La sospecha era que escalar por la base fuese gratis. **Desde una expresión del
calculador no lo es**:

```
345*10  →  97.252 puertas        345*E   →  96.037     (¡la base cuesta MÁS!)
345/10  → 606.860                345/9   → 512.090
```

Pero hay que leerlo como lo que es: una propiedad del **multiplicador escolar**
de `dz-int`, no de la base. El bucle recorre los dígitos del segundo operando
sin saltar el cero, y `12` escrito `10` tiene dos dígitos mientras `E` tiene
uno. Y la mayor parte de esas cifras ni siquiera es multiplicación: es el
andamiaje del evaluador en modo fijo.

```
$ target/release/dz-cli.exe eval "345+0" --mode fixed --gates   → total=68360
$ target/release/dz-cli.exe eval "345*1" --mode fixed --gates   → total=78590
```

Descontado `345*1`, la diferencia real entre `× 10` y `× ↋` es de ~18.700 contra
~17.400 puertas. La ventaja posicional **sí** existe como estructura —`dz-sci`
escala por `12^k` moviendo `scale`, con cero puertas—; lo que no existe es un
camino que la exponga en la aritmética de expresiones. Un lead teóricamente
correcto que el instrumento no deja ver, no un lead refutado. `01-D` §3.1 ya lo
formulaba así («no realizable con este instrumento»); la primera versión de esta
síntesis lo endureció a «refutado», y no lo es.

### 4.4 Goldbach: reformulación limpia, no descubrimiento

109 primos ≤ 600 convertidos uno a uno con el binario: **cero violaciones** de
P6 con `p > 3`, reparto 23/28/27/29 en `{1, 5, 7, ↋}`. Es un hecho conocido
—coprimalidad con 12— escrito de forma bonita. D afiló dos cosas por encima:

- Rigidez de clases: si `n` acaba en `4`, toda descomposición de Goldbach tiene
  dígitos finales `{5, ↋}`; si acaba en `8`, `{1, 7}`. Dos clases **rígidas**,
  mientras base 10 admite siempre dos parejas en sus cinco clases.
- **Las bandas del cometa de Goldbach son legibles en un solo dígito**: media
  `g(n) = 19,19` para `n` acabado en `{0, 6}` frente a `10,24` para
  `{2, 4, 8, ↊}` — razón 1,87, que es el factor 2 de Hardy–Littlewood. En
  decimal las cinco medias se amontonan entre 12,06 y 16,00 sin bandas
  visibles.

**Ampliación posterior a `n ≤ 1.000.000`** (criba propia, 78.498 primos, 499.999
pares; el script vive fuera del repositorio, como `app/src/base10.ts`, porque no
es aritmética dozenal sino conteo de primos). El `1,87` de arriba era ruido de
muestra pequeña: la razón real converge al 2 teórico, y el emparejamiento por
clase es exacto hasta el céntimo.

| último dígito (base 12) | media `g(n)` | `n mod 3` |
|---|--:|--:|
| `0` | 5.015,35 | 0 |
| `6` | 5.015,34 | 0 |
| `4` | 2.515,17 | 1 |
| `↊` | 2.515,30 | 1 |
| `2` | 2.500,84 | 2 |
| `8` | 2.500,62 | 2 |

Banda alta / banda baja = **1,999754** contra el 2 que predice
`(3−1)/(3−2)`. Que `0` y `6` coincidan en `5.015,3` sobre 83.333 valores cada
uno no es una banda que se intuya en un dibujo: es la comprobación numérica de
que el último dígito dozenal **es** `n mod 3`.

Y la comparación honesta, que la versión corta de arriba no daba: **base 10
también tiene banda** —los acabados en `0`, razón **1,3332**— porque su último
dígito delata el 5, con factor `(5−1)/(5−2) = 4/3`. Cada base enseña su propio
primo impar. Base 12 gana por dos razones medibles, no por magia: su factor es
mayor (2 contra 4/3) y más frecuente (un tercio de los pares contra un quinto).
Residuo: dentro de los no múltiplos de 3, `{4,↊}` queda un 0,58% por encima de
`{2,8}` midiendo hasta 10⁶, y un 1,09% hasta 250.000. La primera versión de esta
síntesis lo declaró «sin que la teoría prediga diferencia»; **la teoría sí lo
predice, y con signo**. Es el **sesgo de Chebyshev**. Si `n ≡ 1 (mod 3)`
(acabados `4`, `↊`), los dos primos de la descomposición son `≡ 2 (mod 3)`; si
`n ≡ 2 (mod 3)` (acabados `2`, `8`), los dos son `≡ 1 (mod 3)`. Y los primos
`≡ 2 (mod 3)` son sistemáticamente algo más abundantes:

| límite | `π(x; 3, 1)` | `π(x; 3, 2)` | parejas con suma `≡ 1` sobre suma `≡ 2` |
|--:|--:|--:|--:|
| 250.000 | 10.996 | 11.046 | **+1,100%** |
| 1.000.000 | 39.231 | 39.265 | **+0,581%** |

(Conteo exacto de parejas ordenadas de primos `> 3` con `p + q ≤ x`, script
aparte, fuera del repositorio como la criba.) Reproduce las dos cifras del
residuo, y la caída a la mitad al cuadruplicar el rango es la esperable de un
sesgo que decae como `1/√x`. No es efecto de rango finito sin explicación: es un
fenómeno conocido de la distribución de primos en progresiones, y la base 12 lo
deja leer en el último dígito igual que deja leer el factor 2.

Esto último es lo mejor de la línea y merece nombrarse con precisión: es una
**mejora de visualización de un hecho conocido** (el factor de Hardy–Littlewood
depende de los divisores pequeños de `n`, y 12 los expone en el último dígito).
No es un paso hacia la conjetura, y D no pretende que lo sea.

### 4.5 Perseguir la superficie racional del octaedro L₁ — cuatro intentos, un hallazgo que cambia de categoría

Cerrada la síntesis original, se abrió un hilo aparte para presionar
específicamente el hallazgo A §7.1 (la superficie del octaedro L₁ es `4√3 r²`,
irracional algebraico, el único caso de todo el informe donde L₁ **empeora**
K(F)). Cuatro documentos adicionales, en `03-`, `04-` y `05-`, agotaron las vías
razonables:

1. **Busemann y Holmes–Thompson** (`03-hallazgo-busemann-holmes-thompson.md`) —
   las dos medidas de área diseñadas específicamente para espacios normados, no
   euclidianos. Empeoran en vez de rescatar: `(16π/3) r²` y `(48/π) r²`,
   **ambas trascendentes**. Verificado por reducción al caso plano k=1, donde
   las tres definiciones coinciden y reproducen el perímetro ya conocido `8r`.
2. **Un cuerpo motivado por 12** (`04-hallazgo-cuerpo-12-y-medida-reticular.md`)
   — el dodecaedro rómbico (DR), celda de Voronoi del retículo FCC: el único
   sólido de exactamente 12 caras que aparece de forma natural como celda de
   una red (el dodecaedro regular no tesela `R³` y no es Voronoi de ninguna
   red). Es un zonotopo —suma de Minkowski de las 4 diagonales de un cubo—,
   lo que ya predice sus 12 caras por pura combinatoria (`C(4,2)=6` pares).
   Se derivó la norma real cuya bola unidad es el DR,
   `‖(x,y,z)‖ = max(|x|+|y|, |y|+|z|, |z|+|x|)`, obtenida como función
   soporte de su polar (el cuboctaedro). Resultado: **repite exactamente el
   fracaso del octaedro**. Hausdorff da `6√2 r²` (algebraico irracional,
   verificado con la fórmula cristalográfica `8√2a²` y con un chequeo de
   volumen independiente); Busemann da `4π r²` y Holmes–Thompson `36/π r²`
   — ambas trascendentes, otra vez. Ni el 12 en el recuento de caras ni una
   simetría más rica (`Oh`, orden 48, la misma que tiene el propio octaedro
   en 3D — el `D₄` de la síntesis original es del rombo 2D, no del sólido)
   evitan que π se filtre. Y una medida reticular de tipo Ehrhart: su
   coeficiente `e₂=2` sí es racional (verificado por dos métodos), pero por
   un teorema sin condiciones sobre *todo* politopo de vértices enteros —no
   una propiedad de L₁ ni de base 12— y la cantidad que mide no es invariante
   por isometrías: es el movimiento que el plan prohíbe, identificado y
   reportado como tal en vez de disimulado.
3. **Tres reencuadres no métricos**
   (`05-hallazgo-reencuadres-topologia-proyeccion-complejidad.md`): curvatura
   angular (Descartes/Gauss–Bonnet), proyección discreta (Cauchy) y
   complejidad descriptiva.

**Un resultado explica una parte de los fracasos anteriores, y es topológico, no
de elección de medida — pero no es la única causa, y eso también se puso a
prueba.** El defecto angular total de *cualquier* poliedro convexo suma `4π`,
por pura característica de Euler — no importa si el cuerpo es «redondo» o no.
Verificado con el binario: `6*(2*pi/3)` y `4*pi` coinciden dígito a dígito
(`~10;6968317`). Esto explica, no solo mide, por qué Hausdorff, Busemann y
Holmes–Thompson estaban condenados: π no vive en la norma que se elige, vive en
«vuelta completa» — en `2πχ`. Existe un sustituto puramente combinatorio libre
de π (curvatura de Higuchi, exacto `1/3` por vértice,
`0;4` en dozenal, termina), pero es ciego a la forma real: no distingue un
octaedro regular de uno estirado. Escapa a π perdiendo la geometría.

**El hallazgo que sí cambia de categoría, y es el más importante de este
segundo tramo:** la ecuación implícita del octaedro (`|x|+|y|+|z|≤r`) y la de
la esfera (`x²+y²+z²=r²`) **empatan en K=5** bajo las propias reglas del plan
— nada en la simplicidad de la *forma* distingue poliedro de esfera. Y sin
embargo sus *áreas* divergen sin remedio: octaedro K=4 (algebraico), esfera
K=3 (trascendente) — la clase de número *peor* saca K **más bajo**. En otro
eje, el octaedro admite descripción combinatoria finita exacta (6 vértices, 12
aristas, 8 caras, todo racional) mientras la esfera, lisa, no puede tenerla
nunca: exige ecuación implícita o límite infinito de poliedros.

Conclusión, y hay que nombrarla porque corrige una premisa tácita de toda esta
investigación: **complejidad de forma y complejidad de medida son ejes
independientes.** «Forma simple ⟹ medida simple» no es un axioma que valga en
general — el propio proyecto lo ha estado asumiendo sin decirlo, y este
reencuadre lo desmiente con un ejemplo dentro de su propio marco de trabajo
(K(F)). Es distinto de los otros tres intentos porque no compite en el mismo
terreno: no produce «otro número real que comparar con `4√3`», compara si
*existe* una descripción finita o no — cambia de categoría, así que no puede
colapsar de vuelta al mismo resultado como los demás.

**Balance del hilo completo: seis vías independientes de definir «tamaño de
frontera» sobre cuerpos poliédricos L₁-relacionados** (Hausdorff, Busemann y
Holmes–Thompson —cada una sobre dos cuerpos distintos, octaedro y dodecaedro
rómbico—, defecto angular topológico, muestreo de Cauchy finito, y Ehrhart),
**cero éxitos rescatando racionalidad, un hallazgo estructural que no estaba en
la lista de tareas.** (*Nota 2026-09-25*: `15` muestra que los cuatro fracasos
de Busemann/HT son instancias de un solo teorema —`S_Bus ∈ ω_k·ℚ` y
`S_HT ∈ ℚ/ω_k` para todo politopo racional— y que el punto de partida, `4√3`,
depende de la inmersión euclidiana; ver §4.14.) Ninguna produce un racional que sea a la vez invariante
por isometrías, consistente con Euclides en el caso liso y genuinamente una
medida de tamaño. Refuerza §3.4 en su forma más fuerte disponible: ninguna
noción razonable de tamaño de frontera —métrica, topológica o proyectiva— evita
la trascendencia salvo alineando ejes por artefacto, y la explicación de fondo
es que π está en la topología de la esfera, no en la elección del observador
ni en el número de caras del poliedro. Base 12 no interviene en ninguna de las
seis vías: toda (ir)racionalidad medida es independiente de la base, como
exige Lindemann–Weierstrass.

### 4.6 ¿Es la conversión angular taxicab el eslabón que faltaba? No — son al menos tres mecanismos distintos

Pregunta natural llegados a este punto: el defecto angular vive en `2πχ`, pero
`sin₁`/`cos₁` (la línea B) usan un parámetro de longitud de arco taxicab, `θ₁`,
que **no es** el ángulo euclidiano ordinario `θ₂ = atan2(y,x)`. ¿Es la
conversión entre los dos —que exige `arctan`, transcendente— el mecanismo
único detrás de todos los fracasos anteriores, en vez de una casualidad
repetida seis veces? Se puso a prueba explícitamente en
`06-hallazgo-conversion-angular-taxicab.md`.

La conversión existe y es real: `θ₂(θ₁) = atan(θ₁/(2−θ₁))` por arista del
rombo, con el valor simétrico del octante dando exacto `atan(1) = π/4`
(verificado con el binario, coincide con `pi/4` dígito a dígito) y un punto
genérico como `θ₁=1/2` dando `atan(1/3)`, sin relación conocida con π. Pero
rastreada álgebra por álgebra en los tres sitios donde debería aparecer, **no
aparece en ninguno**:

- Busemann y Holmes–Thompson (§4.5, docs `03`/`04`) trabajan enteros en
  coordenadas cartesianas sobre el plano de sección; π entra solo por `ω_k`, el
  volumen de la bola euclidiana de referencia — una constante normalizadora
  externa, nunca la conversión angular.
- El defecto angular (§4.5, doc `05`) usa ángulos interiores euclidianos
  ordinarios de las caras 3D y nunca parametriza la frontera taxicab por
  longitud de arco. Es el **mismo mecanismo** que Busemann/HT (`ω_k`/`2π`), no
  la conversión `θ₁↔θ₂`.
- La serie de Fourier de `cos₁`/`sin₁` (línea B) trae π por un **tercer**
  mecanismo, independiente de los otros dos: la dependencia intrínseca del
  análisis de Fourier en `SO(2)` como dominio de frecuencia, sin relación con
  cómo se parametrice la curva.

**Veredicto: al menos tres mecanismos independientes**, no uno, aunque los tres
trazan a la misma raíz última («una vuelta completa es `2π`»). La conversión
angular propuesta es real y tiene un costo medible propio —`K=3` en L₁ contra
`K=0` en L₂, donde longitud de arco y ángulo son idénticos por definición—
pero no es la causa de ninguno de los fracasos ya documentados. Es la
respuesta correcta para el estándar de esta investigación: no forzar una
unificación bonita cuando la matemática da varias causas separadas.

---

### 4.7 Rediseñar la bola unidad para que su simetría case con base 12 — la familia entera, clasificada

El antagonismo del §1 admite una réplica generativa, y el usuario la hizo: si
el problema es que `D₄` regala 2/4/8 y base 12 regala 2/3/4/6/12, **cámbiese la
bola unidad** por una cuyo grupo de simetría sea el que base 12 sí premia — el
hexágono regular, `D₆`, orden 12. Cualquier convexo centralmente simétrico es
bola unidad de una norma (su gauge de Minkowski), así que la pregunta es legítima
y no un abuso de definición. Se sometió al mismo escrutinio que tumbó los seis
intentos de §4.5 y §4.6, en `07-hallazgo-cuerpo-simetria-12-rediseno.md`.

**Lo que sí compra, y es el mejor resultado de esa línea.** La razón
perímetro/diámetro del hexágono bajo su propia norma es **3**, racional, y
—contra lo que el precedente de `01` §7.3 hacía temer— **robusta bajo rotación
arbitraria**: girando el hexágono un ángulo cualquiera y midiéndolo con la norma
del hexágono sin girar, todas las aristas siguen midiendo lo mismo y la razón
sigue siendo 3. Eso es estrictamente más sólido que el «4» de L₁, que colapsa a
2 con solo girar 45°, y el rombo sirvió de control negativo explícito en el mismo
experimento. Casilla **(II)**.

**Lo que no compra, que es todo lo demás, y ahora con la familia agotada.** La
segunda pasada del documento cerró las tres preguntas que la primera dejó
abiertas, y el saldo es:

| pregunta | resultado |
|---|---|
| ¿hay un n-ágono con razón racional *intermedia* en (3,4)? | **No, y está demostrado.** Forma cerrada por paridad: `n≡0 (mod 4)` → `n·tan(π/n)`; `n≡2` → `n·sin(π/n)`. Niven fuerza `n=4` (→4) y `n=6` (→3) como únicas racionales |
| ¿rescatan Busemann/HT la superficie del prisma hexagonal? | **No: la empeoran.** `S_Bus = 5π`, `S_HT = 42/π`, trascendentes, frente a `3√3+12`, meramente algebraica. Mismo `ω_k = π` de `03`/`04` |
| ¿es robusta la rotación para `n = 8, 10, 12`? | `n=8` no; `n=12` **no** (toca 3 solo en φ=15°, artefacto de alineación, igual que el 4 de L₁); `n=10` **sí**, pero su constante `(5√5−5)/2` es irracional |

De donde sale el enunciado que resume la línea: **`racional ∧ robusto ⟺ n = 6`**.

`07` concluía además que el área del hexágono es irracional (`3√3/2`) y que el
prisma hexagonal falla ya en el **volumen** (`3√3`). **Esa parte no se sostiene
como afirmación sobre la norma**, y la corrección es de fondo. En un plano normado
el área de Lebesgue de la bola unidad no es un invariante: cambia con cualquier
aplicación lineal, y una aplicación lineal da una norma isométrica. El hexágono
de vértices `(±1, 0), (0, ±1), ±(1, −1)` es imagen afín del regular, define la
misma geometría (su `π_B` sigue siendo 3, y por el Teorema A de `08` es racional
por tener vértices racionales), y su área es **3**; el prisma sobre él tiene
volumen **6**. El `√3` venía de dibujar el hexágono con la métrica euclidiana,
que es estructura ajena a la norma que se estudia. Lo mismo vale para las áreas
de la tabla de `07` §0: son áreas del polígono **euclidiano-regular**, no
invariantes de la norma. Lo que sí sobrevive intacto es todo lo que es invariante
lineal: la razón perímetro/diámetro, su clasificación por Niven y la robustez.

**El dodecágono merece un párrafo aparte porque es la trampa perfecta.** Es el
candidato con más «12» en el nombre (`D₁₂`, orden 24) y resulta ser el **peor**
de los cuatro en perímetro/diámetro (`24−12√3`, irracional) y, dibujado
euclidianamente regular con circunradio 1, el **mejor** en área (exactamente
**3**, entero). Pero esa área racional no tiene relación alguna con base 12: es
el **teorema de Niven** (1956) — `sin θ` racional para `θ` múltiplo racional de
`π` solo en `{0, ±1/2, ±1}` — que caracteriza `n=4` y `n=12` como los únicos
n-ágonos **euclidianamente regulares** de circunradio 1 con área racional, en
cualquier base numérica. El «12» del resultado es que `360°/12 = 30°`, no que
`12 = 2²·3`. Casilla **(III) aparente, (I) real**: no es ni siquiera un efecto de
representación, es aritmética de ángulos euclidianos — y, por la corrección de
arriba, tampoco es un invariante de la norma del dodecágono.

**Por qué este resultado vale más que los seis anteriores.** No por su signo —es
negativo como ellos— sino por su alcance. Los documentos `03`, `04` y `05`
cerraban con «entre los cuerpos examinados»; éste cierra con una **clasificación
exhaustiva** de la familia de n-ágonos regulares: no hay un tercer candidato
esperando. El contraste con `01` §4.4 es el hallazgo colateral y explica la
diferencia: la familia `Lp` regala infinitas razones racionales porque su
parámetro es **continuo** (el teorema del valor intermedio las produce gratis);
la familia de n-ágonos no regala ninguna porque su parámetro es **discreto** y
sus valores viven en cuerpos ciclotómicos `Q(ζ_n)⁺` de grado `φ(n)/2`, donde ser
racional es una condición de grado 1 que Niven caracteriza por completo.
**Racionalidad barata contra racionalidad rígida**: la de `Lp` no significaba
nada, la del hexágono sí, y aun así no basta.

Extensión de P1, en la forma que sí se sostiene: no es solo que no exista un «π
taxicab» único; es que **entre los n-ágonos regulares la única constante
invariante que sale racional y robusta es el 3 del hexágono**. La primera versión
de esta síntesis afirmaba algo más fuerte —que ningún cuerpo tiene
perímetro/diámetro, área y superficie 3D racionales a la vez— y **esa forma cae**:
el hexágono afín de vértices enteros tiene razón 3, área 3 y prisma de volumen 6.
Área y volumen de Lebesgue no son invariantes de una norma, así que no podían
sostener la afirmación. Queda abierta, y bien planteada, la versión con
superficies medidas intrínsecamente (Busemann/Holmes–Thompson, que sí son
invariantes y en `07` §2.6 salen `5π` y `42/π`).

Una nota de higiene, porque el estándar de esta serie es que un error publicado
quede visible: la reverificación de la segunda pasada (`07` §5.7) encontró tres
discrepancias en lo que la primera había publicado —un comando equivocado junto a
una cifra correcta, una superficie de Hausdorff internamente inconsistente
(`3√3+6` debía ser `3√3+12`) y una tabla auxiliar con ruido mal explicado—, las
tres corregidas y ninguna capaz de mover un veredicto. Que aparecieran es la
razón por la que la reverificación se hizo.

---

### 4.8 ¿Una familia de normas con `π_k` racional que converja a π? Sí — y por eso mismo no vale nada

La pregunta la trajo el usuario en su forma más ambiciosa: construir geometrías
`G₁, G₂, G₃, …` cuya «circunferencia» unidad sea un polígono de `n_k` lados
(propuesta: `n_k = 4^k`), con `π_k = P_k/2r_k` **racional en cada etapa** y
`π_k → π`. Se atacó en `08-hallazgo-familia-normas-pi-racional.md` partiendo de
§4.7 como resultado establecido, no rederivándolo.

**La mitad de la pregunta ya estaba contestada, y en contra.** §1.8 clasifica la
familia regular por completo: `16, 64, 256` son todos `≡ 0 (mod 4)`, caen en la
rama `n·tan(π/n)`, y Niven la cierra en `n=4`. **La familia `‖·‖₄, ‖·‖₁₆, ‖·‖₆₄`
de polígonos regulares no existe, y está demostrado que no.** Y `4^k` resulta no
tener ninguna justificación: no es una bisección, no corresponde a ningún paso
constructivo, y cada término ≥16 vive en la rama irracional y no robusta. El
sustituto con estructura es `2·3^(k+1)` = 6, 18, 54, 162 — la única sucesión
multiplicativa que nunca abandona la rama `n ≡ 2 (mod 4)`, la robusta. (Nota: la
sucesión `6·2^k` de Arquímedes **sí** la abandona, y en su primer paso.)

**La otra mitad estaba abierta y se cerró afirmativamente, en el peor sentido
posible.** `07` §6 declaraba sin explorar el espacio de convexos centralmente
simétricos **no regulares**, donde los vértices son parámetros continuos y la
rigidez de Niven desaparece. Ahí `08` demuestra dos cosas:

- **Continuidad** (Prop. 2): `π_B` varía continuamente con los vértices — y el
  documento señala el punto exacto donde el argumento ingenuo falla (hay que
  aplicar la desigualdad triangular inversa **dentro de la norma que se mueve**
  antes de comparar cuerpos, porque el gauge cambia cuando cambia el cuerpo).
  Consecuencia: para todo `2m ≥ 8` el conjunto alcanzable de `π_B` es
  exactamente el intervalo abierto `(3,4)`, ya que los dos extremos de Gołąb son
  rígidos en 4 y 6 lados.
- **Teorema A**, que es más barato que el TVI y más destructivo: *todo polígono
  convexo centralmente simétrico con vértices racionales tiene `π_B ∈ ℚ`* — los
  funcionales de arista salen de sistemas 2×2 racionales, luego
  `‖w‖_B = max_j ⟨u_j, w⟩` es un máximo de racionales. **La racionalidad es una
  propiedad del papel, no del cuerpo**, y es tautológica sobre un conjunto denso.

Con eso la sucesión pedida existe —cascos convexos de puntos racionales del
círculo: `π_k ∈ ℚ` por el Teorema A, `π_k → π` por continuidad— y hay
construcciones explícitas de 16 y 64 lados con `π_B = 22/7` exacto, verificadas
por dos vías independientes. El precio, que es el hallazgo:

| lo que se pierde | detalle |
|---|---|
| unicidad | una variedad de dimensión 11 de 16-gonos por cada valor objetivo |
| isotropía | ningún n-ágono **afínmente** regular tiene vértices racionales salvo `n = 3, 4, 6`; euclidianamente regular, solo el cuadrado |
| canonicidad | hay un continuo de sucesiones válidas, ninguna privilegiada |
| **honestidad del método** | los polígonos de `22/7` **reciben el número objetivo como entrada de una ecuación lineal** — el movimiento que §1 del plan prohíbe, en su forma más literal |

Y como método de aproximación es malo: a error `≈1,1·10⁻³` el racional del
96-gono construido en `08` necesita **278 cifras de denominador**, frente a **1**
de `22/7`; en el paso siguiente la penalización es ×294. El error decae como
`π³/(3n²)` —cuadrático en `n`—, y en esa construcción el denominador crece muy
deprisa porque cada vértice aporta **su propia** hipotenusa pitagórica. Eso es
una elección de la construcción, **no** una necesidad: tomando todos los vértices
con un denominador común `c = 5·13·17·…`, cada primo `≡ 1 (mod 4)` añadido duplica
los puntos racionales disponibles sobre la circunferencia, y el denominador
crecería mucho más despacio. La primera versión de esta síntesis llamaba
«estructural» al crecimiento exponencial; `08` §6 ya advertía que era lectura de
tabla. La penalización frente a los convergentes probablemente sobrevive (son
óptimos), pero su tamaño no está medido para la mejor construcción. **Como
método de aproximación no compite: es una reformulación geométrica de la
densidad de ℚ.**

**Y el sabor dozenal sale negativo, no neutro** (Prop. 4, hallazgo nuevo): los
puntos racionales del círculo tienen denominadores de la forma `p²+q²`, y
**ningún número de esa forma es divisible por 3** (si `3 | p²+q²` con
`gcd(p,q)=1`, los cuadrados mód 3 fuerzan `p ≡ q ≡ 0`, contradicción). Como
hipotenusas de ternas primitivas son además **impares**, así que sus factores
primos son solo los `≡ 1 (mod 4)`, lista que **empieza por 5**. De ahí
que `3/5` y `4/5` terminen en base 10 y **no** en base 12: en la aritmética
natural de esta construcción **base 10 es estrictamente mejor que base 12**.
No es casualidad — **esta construcción *son* ternas pitagóricas**, el dominio
que §1 ya había identificado como hostil a la base 12.

**Veredicto por partes:** *verdadero* — existe la familia racional convergente a
π, y está construida; *falso y demostrado falso* — la versión con polígonos
regulares y cualquier justificación de `4^k`; *verdadero pero vacío* — la versión
no regular, por el Teorema A; *abierto* — qué caracteriza realmente a los cuerpos
con `π_B ∈ ℚ`, y si algún 64-gono de coordenadas pequeñas alcanza `22/7`.

La moraleja enlaza con §4.7 y la completa. Allí se distinguió la racionalidad
**barata** de la familia `Lp` (parámetro continuo, el TVI la regala) de la
**rígida** del hexágono (parámetro discreto, Niven la caracteriza entera). Este
documento muestra el caso extremo de la barata: cuando se puede pedir cualquier
racional, obtenerlo deja de ser información. **Los únicos dos racionales que
significan algo en toda la historia siguen siendo los que no se eligieron: el 4
del paralelogramo y el 3 del hexágono, y son las paredes del intervalo de
Gołąb, no puntos de su interior.**

---

### 4.9 ¿Evita π cambiar la topología en vez de la métrica? No — Gauss-Bonnet solo decide el múltiplo

El origen de esta pregunta y la siguiente (§4.10) es distinto a los anteriores:
el usuario trajo una afirmación de Kryon (canalización, no fuente matemática)
de que π «está mal» por calcularse en base 10, y la reinterpretó de forma
caritativa — no que π esté mal, sino que exista alguna geometría que
genuinamente no lo necesite, más allá de solo cambiar la norma (agotado en
`03`–`05`, `07`, `08`). Se abrió una lista ordenada de «ingredientes»
candidatos. El primero, en `09-hallazgo-topologia-euler-ingrediente1.md`:
en vez de tocar la métrica, tocar la **característica de Euler** χ del
objeto.

Motivación exacta: `05` §1.3 ya había mostrado que el defecto angular total de
cualquier poliedro convexo es `4π` porque χ=2, no por la norma. La pregunta:
si χ≠2, ¿desaparece π, o solo cambia el múltiplo?

**Construcción verificada, no citada.** Un anillo de 8 cubos unitarios (rejilla
3×3 con el cubo central retirado) — un toro poliédrico. Contado a mano:
V=32, E=64, F=32.

```
$ target/release/dz-cli.exe eval "32-64+32" --mode rational
0
```

χ=0, confirmado por conteo, no asumido. Tres clases de vértice (esquina
exterior, punto medio de arista, esquina interior del agujero) dan defectos
angulares `+π/2`, `0`, `−π/2` respectivamente (8, 16 y 8 vértices), verificados
uno a uno con el binario. La suma total:

```
$ target/release/dz-cli.exe eval "8*(2*pi-3*(pi/2)) + 16*(2*pi-4*(pi/2)) + 8*(2*pi-5*(pi/2))" --mode fixed --scale 8
~0
```

**π no desaparece: aparece en cada uno de los 32 vértices con signo.** El total
da 0 (`2πχ` con χ=0) por cancelación genuina de 8 términos `+π/2` contra 8
`−π/2` — un cero real, no una tautología «0=0» — pero sigue siendo la misma
fórmula de `05` §1.3, evaluada en otro entero. No es hallazgo nuevo.

Se exploró también si el área de una superficie hiperbólica compacta,
`4π(g−1)` (Gauss-Bonnet suave con `K=−1` fijo), fuera «menos π» que `4πr²`
en la esfera. No lo es: ambas son `(constante topológica entera) × π`; elegir
`K=−1` en vez de un radio libre es la misma clase de elección arbitraria.
Reformulación, no hallazgo.

**El único resultado positivo, y no es nuevo en su mecanismo**: la curvatura
combinatoria de Higuchi (`05` §1.4, sin ángulos, solo grados de vértice y
cara) aplicada al mismo toro da valores exactos `1/4`, `0`, `−1/4` —
racionales puros, sin `~`, verificados con `--mode rational`:

```
$ target/release/dz-cli.exe eval "8*(1-3/2+3*(1/4)) + 16*(1-4/2+4*(1/4)) + 8*(1-5/2+5*(1/4))" --mode rational
0
```

Evita π con certeza (3,18× menos puertas, 8,97× menos profundidad que la
versión angular, medido) pero paga el mismo precio que en `05`: ciego a la
forma real, solo ve el grafo de incidencias. Es la confirmación de que el
ingrediente que ya funcionaba (contar en vez de medir ángulos) sigue
funcionando igual cuando χ cambia — no un ingrediente nuevo.

**Base 12**: sin conexión genuina. El único patrón dozenal (`1/4` termina) es
el mismo `12=2²·3` de siempre, disparado por haber elegido cubos (potencias de
2), no por el género del objeto. Casilla (I) explícita.

**Veredicto del ingrediente 1: negativo.** Cambiar χ no da geometría sin π —
Gauss-Bonnet ata π a la noción de «vuelta completa», y χ solo decide el
múltiplo entero que la acompaña, incluido el múltiplo 0.

### 4.10 ¿Evita π una medida de tamaño no aditiva? Sí — y al someterla a L₁ la polaridad se invierte

Segundo ingrediente de la lista (`10-hallazgo-medida-no-aditiva-ingrediente2.md`):
en vez de tocar la métrica o χ, tocar el **axioma de aditividad** de la propia
medida. Todo lo medido en la serie hasta `09` (Hausdorff, Busemann,
Holmes–Thompson, Ehrhart, defecto angular, `κ_comb`) es aditivo sobre uniones
disjuntas por construcción.

**Sí existe, y es matemática clásica, no ad hoc.** La **capacidad
logarítmica** (= diámetro transfinito, teorema de Fekete–Szegő; = radio
conforme para dominios simplemente conexos) del disco de radio `R` es
`cap = R`, **exactamente, sin π**, derivado a mano vía la función de Green del
dominio exterior y confirmado contra dos fuentes independientes (Saff–Totik,
MathWorld). Se extiende a 3D: la capacidad de Newton de la bola de radio `r`
también es `r` exacto. El segmento de longitud `L` da `cap = L/4`; la elipse
de semiejes `a, b` da `cap = (a+b)/2` — los tres, racionales/algebraicos
limpios en los parámetros del cuerpo.

**Es genuinamente no aditiva, y más de lo esperado.** Una monografía clásica
del área (Landkof) afirma subaditividad simple (`cap(A∪B) ≤ cap(A)+cap(B)`);
la búsqueda bibliográfica encontró que **es falsa en general** — Pyrih (1992)
construye un contraejemplo explícito. Lo que sí se sostiene es la
subaditividad fuerte del **logaritmo** de la capacidad, la propiedad
axiomática correcta de una capacidad de Choquet (1954). Se documenta el error
de una fuente autorizada, corregido en vez de repetido.

**La prueba contra L₁, siguiendo el estándar del resto de la serie, invierte
la polaridad habitual.** La bola unidad de L₁ es el cuadrado de vértices
`(±1,0), (0,±1)`. Su capacidad logarítmica, cuya **fórmula** se cita de la
literatura (Pólya–Szegő; no rederivada — Schwarz–Christoffel de un cuadrado
exige funciones hipergeométricas fuera del alcance de una derivación
autocontenida):

```
cap(cuadrado L₁) = √2 · Γ(1/4)² / (4π^{3/2}) ≈ 0,8346...
```

*Actualización 2026-09-14*: a raíz de este hallazgo se implementó `gamma` en
el core del proyecto (`crates/dz-sci/src/gamma.rs`, serie de Stirling con
números de Bernoulli exactos, sin ningún atajo por `f64` nativo). El
**valor** ya no depende solo de la cita — se recalculó con el propio motor de
puertas:

```
$ target/release/dz-cli.exe eval "sqrt(2)*gamma(0;3)^2 / (4*exp(1;6*ln(pi)))" --mode fixed --scale 12
~0;X0229X48XE083XE069682291
```

coincide con la conversión de la cifra citada hasta donde ésta tenía
precisión (9 de 12+ dígitos dozenales), y cuesta 1.770.743.155 puertas —la
cantidad más cara de calcular de toda la lista de ingredientes. La fórmula
en sí sigue citada, no rederivada; el valor, ya no.

Trascendente, con π combinado con `Γ(1/4)` (la constante lemniscática), que es
**algebraicamente independiente** de π (Chudnovsky/Nesterenko). En todo el
resto de la investigación L₁ daba lo racional y L₂ necesitaba π (`99-sintesis.md`
§3.3: perímetro de elipse, mediana, rutas mínimas). **Aquí es al revés: el
disco L₂ es limpio, el cuadrado L₁ es sucio.**

La razón no es casualidad. La capacidad logarítmica es el radio conforme del
exterior del cuerpo, vía la aplicación de Riemann. Para el disco, la elipse y
el segmento esa aplicación es elemental (identidad, Joukowski). Para el
cuadrado, tiene esquinas, y las esquinas fuerzan Schwarz–Christoffel — la
misma integral que rectifica la lemniscata de Bernoulli, de donde sale
`Γ(1/4)`. Es exactamente lo opuesto de por qué L₁ ganaba en longitud de arco:
ahí una esquina no añadía coste (el perímetro es indiferente a la suavidad);
aquí la esquina es precisamente la singularidad que rompe la elementalidad de
la aplicación conforme. **Capacidad y perímetro son funcionales
geométricamente opuestos en su sensibilidad a las esquinas** — segundo ejemplo,
con signo invertido, del hallazgo transversal de §4.5: complejidad de forma y
complejidad de medida son ejes independientes.

Casilla **(I), con una precisión**: la capacidad logarítmica no es trasladable
a L₁ en el sentido en que área o curvatura sí lo eran en `05`/`09` — usa el
núcleo `log|z−w|` (solución fundamental del laplaciano en 2D) y aplicaciones
holomorfas; no existe una «capacidad L₁» con núcleo `‖z−w‖₁` que generalice la
teoría (no hay teorema de aplicación de Riemann para normas no euclidianas).
La pregunta «capacidad de la bola L₁» ya presupone L₂ para poder plantearse —
mismo punto que §3.1 (Ostrowski/Gelfand–Mazur): la estructura holomorfa **es**
L₂, no una elección entre otras.

**Base 12**: sin conexión, ni siquiera la barata de `12=2²·3` — los
denominadores que aparecen (2, 4) son potencias de 2, compartidas por ambas
bases (verificado: `1/4`, `3/4`, `1/2` terminan tanto en dozenal como en
decimal; el control de contraste, `1/3`, que sí discrimina, no aparece en
ninguna fórmula de capacidad).

**Veredicto del ingrediente 2: la pregunta central se responde que sí** —
existe una medida de tamaño rigurosa y no aditiva que evita π en el disco y
la esfera, dando exactamente el radio. Pero no vindica la lectura de Kryon:
`cap` no es «el tamaño del círculo» en el sentido intuitivo (no es área, no
es perímetro; coincide con el radio por un teorema específico de teoría del
potencial, no porque «corrija» nada), y al someterla a la prueba de L₁ que
exige el protocolo de esta serie, el resultado se revierte — es el caso
euclidiano el limpio.

### 4.11 ¿Evita π cambiar de sistema numérico? Ni sí ni no — la pregunta no se traslada

Tercero de la lista (`11-hallazgo-padico-ingrediente3.md`): en vez de tocar
métrica, χ o aditividad, cambiar el propio **sistema numérico métrico** — de
ℝ a `ℚ_p`, los números p-ádicos, con la métrica ultramétrica
`d(x,y) = max(d(x,z), d(z,y))`. Es la candidata más cercana a lo que Kryon
llama vagamente «multidimensional», pero es teoría de números real y bien
establecida, no física especulativa.

**El resultado es de una tercera categoría, distinta de `09` (negativo) y
`10` (positivo con matiz).** Derivado desde la desigualdad ultramétrica: todo
punto de una bola es su centro, dos bolas son disjuntas o anidadas, las bolas
son clopen — de donde `ℚ_p` es **totalmente disconexo**: no hay curvas
continuas no constantes, y por tanto nada que rectificar. La «esfera»
`S(a,r)` sí existe pero es unión de `p−1` subbolas (árbol `p`-ario de
Bruhat–Tits), no una curva: no hay perímetro que definir, no porque no se
haya encontrado, sino porque el objeto no existe.

Lo que sí existe y se derivó con rigor, sin ningún π:

- **Medida de Haar**: `μ(p^kℤ_p) = p^{-k}`, exacta, aditiva, deducida
  contando clases residuales. `μ(ℤ_p^*) = 1−1/p`, verificado con el binario
  para `p=2,3,5` (`1-1/2`, `1-1/3`, `1-1/5` → `1/2`, `2/3`, `4/5`). Es
  geometría sin π real — pero mide **volumen**, no responde a la pregunta del
  círculo: no hay «razón perímetro/diámetro» p-ádica con la que contrastarla.
- **Conteo de subbolas**: exactamente `p`, entero, sin cuadrado — porque no
  hay dimensión 2 con rotación isótropa continua (mismo mecanismo que hace
  que L∞ tampoco tenga π: §3.1 y §4.10 §4.4 ya lo habían visto desde otro
  ángulo).
- **Dimensión de Hausdorff de `ℚ_p`**: derivada desde cero, sale
  **exactamente 1** para cualquier primo — entero limpio, no un exponente
  fractal feo, y coincide con la medida de Haar en el punto crítico.

**Veredicto central**: la pregunta «¿cuánto vale π aquí?» no tiene referente
en `ℚ_p`, porque el objeto que genera π —curva rectificable, o grupo de
rotación continuo `SO(2)`— no existe en un espacio totalmente disconexo. No
es que π reaparezca disfrazado (como en `09`) ni que desaparezca con precio
(como en `10`): no hay fórmula que plantear. Se declara una categoría
explícita, **(IV) pregunta no trasladable**, distinta de (I)/(II)/(III).

**Base 12, con conexión genuina por primera vez en la lista de ingredientes.**
`12` no es primo, así que no hay `ℚ_12` como cuerpo — pero sí el anillo de
enteros 12-ádicos `ℤ_12 ≅ ℤ_2 × ℤ_3` vía el teorema chino del resto en el
límite inverso, exactamente el mismo teorema, a un solo nivel finito, que usa
`crates/dz-gates/src/backends/crt.rs` (`decompose`/`reconstruct` mod 4 × mod
3). Verificado con el binario que el paralelo es real:

```
$ target/release/dz-cli.exe compare mul 25 34
mvl y crt empatan en los cinco contadores (83.927) y en profundidad (441);
crt paga 118.630 reconstrucciones, mvl 0.
```

Leído ahora en su forma estructural: `ℤ/12ℤ` como anillo producto tiene suma
y producto gratis componente a componente, pero **no es cuerpo** (tiene
divisores de cero, `4·3≡0 mod 12`) y por tanto no tiene orden total
compatible sin reconstruir — el mismo «CRT no compensa» de `02-crt-vs-mvl.md`
y de §1, ahora explicado por qué no puede compensar. Pero **ninguna ventaja
aritmética real de 2,3 sobre 5** más allá de la terminación ya sabida:
cuerpo residual, grupo de unidades y dimensión de Hausdorff (que da 1 para
cualquier primo, sin excepción) tratan a cada primo por separado. Cierra el
teorema de Ostrowski — ningún primo está privilegiado en la clasificación de
completaciones de ℚ — el mismo tipo de resultado que ya cerró la puerta a
«elegir» L₁ para ℂ en §3.1.

**Veredicto del ingrediente 3**: ni positivo ni negativo en el sentido de `09`
o `10` — la pregunta original presupone estructura (curvas, rotación
continua) que este sistema numérico no tiene. Con los tres ingredientes: uno
negativo directo, uno positivo con polaridad invertida, uno que disuelve la
pregunta. Ninguno vindica la lectura de Kryon.

### 4.12 ¿Qué puede decir de Gödel este instrumento? Casi nada del teorema, algo de la aritmetización

Cuarta extensión posterior al cierre (`12-hallazgo-godel-incompletitud.md`),
abierta por un vídeo divulgativo sobre los dos teoremas de incompletitud. **Gödel
no se resuelve** y no se intentó; los teoremas no tienen π, métrica ni base, así
que la pregunta de la serie casi no se traslada (categoría **(IV)** de §4.11).
Predicciones `PG1`-`PG6` registradas antes de medir; `PG3` se cumplía solo
en el punto registrado y se retiró al re-medir con `valuation` (`12` §10.2); las demás se cumplen.

- **La calculadora es un evaluador de términos cerrados, no un sistema
  deductivo.** Su fragmento exacto decide igualdades y desigualdades cerradas
  (decidible y completo; le falta el cuantificador no acotado que activa Gödel).
  Su fragmento aproximado **no decide**: `sign(sqrt(2)*sqrt(2)-2)` da `~-1` y
  `sign(sin(1)^2+cos(1)^2-1)` da `~1`, y ambos términos valen 0. Es un límite de
  la implementación y de las funciones reales, no de Gödel.
- **Numeración (casilla III, dependiente de la máquina).** La posicional en base 12
  se lee y se descodifica a 0 puertas de conversión porque la representación del
  instrumento *es* esa; leer el mismo valor en base 10 crece de forma cuadrática
  (segunda diferencia constante 1.784, sobre una sola cadena de 6 cifras) y volver
  cuesta más. Las potencias de primos descodifican con división restauradora:
  6.254.722 puertas para 6 símbolos con `valuation` frente a 0 (la cifra primera, 37.366.435, era `mod` + `div` por
  paso). En una máquina decimal los papeles se invertirían; no se midió.
- **Que 2 y 3 sean los factores de 12 no compraba nada medible en `mod`** (por 2, 3, 4, 6
  cuesta lo mismo que por 5, 7, ↋; el instrumento no usaba la regla del último
  dígito). Se construyó (`divisible`: 36-39 puertas frente a 20.563-754.677 de `div_rem`, `12` §10.3). Lo otro real son los ceros finales, `min(⌊a/2⌋, b)`, que
  cualquier base con esos factores comparte. Coincidencia de vocabulario,
  casilla (III).
- **Consistencia acotada: (IV).** Las expresiones no tenían bucles ni
  cuantificadores; hoy `dz-cli exists|forall` es un bucle acotado de `dz-api` (`12` §10). Un candidato cuesta 195.367
  puertas, el barrido es lineal en candidatos y los candidatos son exponenciales
  en la longitud de la prueba. Nada de esto se acerca a `Con(F)`, que lleva `N`
  sin cota.

Lo que no se hizo: no se implementó `Prf`, ni lema diagonal, ni Rosser, ni Gentzen,
ni una máquina decimal; una sola cadena y solo backend MVL. Ver §7-§8 de `12`. Las
afirmaciones laterales del vídeo no se verificaron.

### 4.13 ¿Qué aporta la base 12 a la hipótesis de Riemann? Un teorema en el lado trivial, nada en los ceros

Quinta extensión posterior al cierre (`13-hallazgo-riemann-base12.md`), abierta
por una pregunta directa del usuario (2026-10-09). **La HR no se resuelve** y no
se intentó. Es un enunciado sobre los ceros de ζ y no depende de cómo se
escriban los números, así que la pregunta "¿la base 12 ayuda a probarla?" es
**(IV)**. Predicciones `PR1`-`PR7` registradas antes de medir, salvo `PR2`, que
quedó contaminada y se marca. Se cumplen todas menos `PR7`, que falla en uno de
cinco ceros.

- **El único resultado de base 12 es aritmético y está en el lado trivial
  (III).** `ζ(1−2k)` **nunca** tiene desarrollo decimal finito, porque von
  Staudt–Clausen pone siempre 2·3 en el denominador. En base 12 termina si y solo
  si ningún primo `p ≥ 5` cumple `(p−1) ∣ 2k`, y entonces el denominador es
  **exactamente 12**: `ζ(−1) = ζ(−13) = −0;1`, `ζ(−25) = −2788↋;7`, y hay
  infinitos así (Dirichlet). Los Bernoulli se calcularon por puertas hasta
  `B₅₂` (27.673 M de puertas) y coinciden 26 de 26 con un oráculo `BigInt`. El
  criterio se contrastó hasta 2k = 400. Base 12 es la **menor** base con una
  sola cifra fraccionaria, no la única. `ζ(−1) = −1/12` de `01-D` F7 resulta ser
  el primer miembro de una familia infinita, no una coincidencia aislada.
- **Los ceros (IV).** La función Z de Hardy (Riemann–Siegel truncado en `C₀`,
  todo real) localizó los cinco primeros ceros en la recta crítica, con errores
  de 2,3·10⁻³ a 7,5·10⁻³ y ≈ 3·10¹⁰ puertas por cero. Es una localización, no
  una verificación: no hay cota de error ni recuento `N(T)`. `1/2 = 0;6` no gana
  nada.
- **Formas aritméticas equivalentes, comprobadas en finito.** Mertens:
  `|M(n)| < √n` en `2 ≤ n ≤ 10000`, exacto, y `M(10000) = −23`; 0 discrepancias
  con el oráculo en 9.999 filas (hasta 1728 sin `isqrt`, hasta 10000 con ella). Robin: se cumple en 12 números colosalmente
  abundantes desde 55440 con margen `0;04`-`0;09`, y falla en 2520 y 5040. La
  HR es Π₁: el instrumento podría **refutarla** en un punto, pero nunca
  demostrarla (`12` §2.3).
- **Lo de base 12 que sí se mide es (III) y compartido.** La clase mod 12 de un
  primo es su última cifra, que se lee con 0 puertas frente a 148.140 de
  `divrem`. La última cifra decide `μ = 0` en el 64 % de los casos con cuadrado,
  y en base 10 en ninguno. Los ceros finales de los colosalmente abundantes los
  iguala o supera la base 6. El sesgo de Chebyshev mod 12 (la clase 1 nunca va
  por delante hasta 10000: 300 primos frente a 309, 311 y 307) es de los caracteres mod 12, no de la base.

Efecto sobre el instrumento: la primalidad sin cota √n exacta costaba 819 M de
puertas por primo. Después se añadió `isqrt` (Newton entero sobre `Dz`, solo
CLI), y la misma comprobación baja a 14,2 M. Lo que no se hizo: verificación
rigurosa de ceros, nada fuera de la recta (no hay complejos), Lagarias,
Nyman–Beurling, Li, densidades de Rubinstein–Sarnak, literatura para el
corolario de §3.2. Ver §8-§9 de `13`.

### 4.14 La frontera de la bola L₁ en n dimensiones — §3.4 se reescribe

Abierto por una revisión de huecos (`15-hallazgo-frontera-bola-L1-n-dimensiones.md`):
§3.4 era un enunciado general apoyado en un solo cuerpo, y `05` ya pedía el cubo
como segundo ejemplo. Tres comprobaciones, todas con el binario:

| comprobación | resultado |
|---|---|
| bola L₁ en dimensión `n` (dos derivaciones independientes: símplex regular y fórmula del cono) | `S₁(n) = 2ⁿ√n·rⁿ⁻¹/(n−1)!`: racional ⟺ `n` cuadrado perfecto. 4D: `16/3` (`14/3` en el binario) |
| cubo L∞ | `n·2ⁿ·rⁿ⁻¹`, racional siempre (`24r²` en 3D, `20` en el binario) |
| octaedro `{\|x\|+2\|y\|+2\|z\| ≤ 1}`, isométrico a `ℓ₁³` por `diag(1,½,½)` | superficie de Hausdorff **3**, por producto vectorial y por el cono |

El mecanismo es un lema: la superficie euclidiana de un politopo racional está
en `Σ ℚ·‖a_F‖₂` (normales enteras `a_F`), así que es racional cuando las normales
son pitagóricas. `(1,1,1,1)` tiene norma 2; `(1,2,2)`, norma 3. La irracionalidad
de §3.4 era aritmética de sumas de cuadrados en la inmersión elegida, el mismo
tema que `08` Prop. 4.

Lo invariante sale **más fuerte** que antes. **Teorema B**: para todo politopo
centralmente simétrico racional, o imagen lineal de uno, en `n ≥ 3`,
`S_Bus ∈ ω_{n−1}·ℚ₊` y `S_HT ∈ ℚ₊/ω_{n−1}`, ambas trascendentes. Encajan las seis
cifras de `03`/`04`/`07` (`16π/3`, `48/π`, `4π`, `36/π`, `5π`, `42/π`) y las
nuevas del cubo (`6π`, y `48/π` igual que el octaedro, como exige la dualidad de
Holmes–Thompson). Lo que `03` §5 dejaba en «plausible» es ahora un teorema para
politopos, y no dice nada de L₁: π lo pone la bola de referencia `ω_k`.

Reenunciado: *las medidas de frontera invariantes de toda norma poliédrica son
trascendentes en `n ≥ 3`; la medida euclidiana es algebraica y su racionalidad
depende del dibujo.* El «único caso donde L₁ empeora K(F)» lo empeora la
inmersión, no la norma: la misma geometría da K = 4 (`4√3r²`) o K = 2 (`3r²`).
Base 12: solo el `1/3` de `16/3 = 5;4`, casilla (III) trivial.

### 4.15 Paseo aleatorio D₄ frente a D₆ — L₂ emerge al iterar, y la simetría 6 gana por un orden

Abierto por la misma revisión de huecos (`16-hallazgo-paseo-aleatorio-D4-D6.md`).
El experimento 9 (difusión) solo se había tratado en el continuo, y el
laplaciano discreto de 4 vecinos en Z² **es** L₁.

**Teorema 1** (§3.1 ampliado arriba): varianza finita más una rotación de orden
≥ 3 dan un límite gaussiano isótropo. **Teorema 2**: una capa de vecinos
invariante por la rotación de orden `k` tiene todos sus momentos direccionales
de orden `< k` isótropos, porque solo admite armónicos múltiplos de `k`. D₄ llega
al orden 2 y D₆ al 4. El cuarto cumulante por paso en Z² vale `−1/4` en el eje y
`−1/2` en la diagonal; en la red hexagonal, `−3/8` en toda dirección (binario).
El laplaciano discreto lo repite: `Δ + (h²/12)(∂⁴ₓ+∂⁴ᵧ)` en Z², no invariante, y
`Δ + (h²/16)Δ²` en la hexagonal, invariante.

**Predicción registrada y confirmada.** A `|x|` fijo, la anisotropía puntual
`P(S_n=x)/P(S_n=x') − 1` (con `|x| = |x'|`) debe decaer como `n⁻³` en D₄ y como
`n⁻⁵` en D₆. Una enumeración exacta hasta `n = 601`, en dos pares de puntos por
red, da pendientes locales de **−2,994** y **−4,986**, monótonas hacia −3 y −5.
El retorno hexagonal cumple `n·P → √3/(2π)` (cociente 0,99917 en `n = 601`).

**Lo que cambia.** Es la primera vez en la serie que la simetría que prefiere la
base 12 (orden 6) es también la que prefiere el **análisis**, no solo la razón
perímetro/diámetro de §4.7. L₁ llega al 4.º orden solo si se le añade la esfera
L∞ con peso 4:1 (el modelo D2Q9); D₆ llega sola. El techo lo pone la restricción
cristalográfica: D₁₂ sería isótropa hasta el orden 10, pero ninguna red la tiene.

**Base 12.** Los pesos isótropos a 4.º orden (D1Q3 `2/3, 1/6`; D2Q9 `4/9, 1/9,
1/36`; D2Q7 `1/2, 1/12`) son todos 3-lisos, y es la primera familia que pasa la
regla de poda de `01-D` §5.1 con el 3 forzado por el análisis: es la curtosis
gaussiana, `E[X⁴] = 3σ⁴`. Tres límites, dichos en `16`: el 3 lo trae el límite
**L₂** y no L₁; base 6 empata; y a 6.º orden (`E[X⁶] = 15σ⁶`, `15 = 3·5`) las
constantes reticulares pasan a ser irracionales (`c² = 1 ± √(2/5)`) o no existen.
Casilla (I)+(III). P5 sigue en pie en el espíritu.

### 4.16 ¿Qué dice la base 12 del exponente de irracionalidad de π? Nada de la prueba, nada decidible, una imagen finita limpia

Sexta extensión posterior al cierre (`14-hallazgo-exponente-irracionalidad-pi.md`),
abierta por un preprint del repositorio `openai/math` (familia 017, 2026-09-24)
que afirma `μ(π) = 2` frente a la cota publicada 7,103. Empezó en otro equipo
(`14-pi-exponent/HANDOFF.md`) y se terminó aquí el 2026-10-11. **No se verificó
el preprint** ni su formalización Lean; nada de lo medido depende de que sea
cierto. Predicciones `PE1`-`PE5` registradas en el traspaso antes de medir; se
cumplen todas menos `PE3`.

- **(IV), y más fuerte que en `13`.** La prueba vive en ℂ y en geometría
  algebraica, sin primos ni `lcm`. El enunciado es `∀ν ∃Q ∀q ≥ Q` con `Q` no
  efectivo: ningún cálculo finito puede confirmarlo **ni refutarlo** (la HR, al
  menos, es refutable).
- **Fracción continua certificada.** De las 99 cifras dozenales de π, por
  intervalo y en racional exacto, salen 104 cocientes (`a0`-`a103`); de 399
  cifras (P3), 429; de 409, 434 (`a0`-`a433`). 0 discrepancias con un oráculo
  `BigInt` en las tres. La de 409 certifica **`a431 = 10034₁₂ = 20776`**, el
  cociente más grande de π en sus primeros miles.
- **Exponentes locales.** `λₙ = −log|π − pₙ/qₙ| / log qₙ` en 430 convergentes
  (399 cifras hasta `n = 422`, 409 para `n = 420–430`), error ≤ 1,75·10⁻¹²
  frente al oráculo (3,9·10⁻⁹ en la última fila, medida con un corte de
  fiabilidad más laxo). Todas `> 2` (teorema clásico), y el exceso medio sobre 2
  decrece como ~`1/n`: 0,016 en `n = 41–100`, 0,0031 en `n = 301–422`. Los picos
  están antes de los cocientes grandes (`a4 = 292`, `a21 = 84`, `a79 = 161`,
  `a307 = 436` → 2,017). El cociente gigante solo da **`λ₄₃₀ = 2,0200`**: llega
  tan tarde que `ln q₄₃₀ ≈ 497` lo diluye. El máximo sigue siendo **22/7
  (3,43)**, no 355/113 (3,20): `PE3` falla.
- **Lo único ligado a la base es (III) compartido.** El teorema acota las
  rachas de cifras repetidas en el desarrollo de π, en cualquier base. En 399
  cifras dozenales hay una sola racha de 3 (`↋↋↋` tras la cifra 105) y 34 de 2
  o más (33 esperadas al azar); `max r/k = 0,154` (0,087 en base 10), lo mismo
  que con 99.
- **Flint–Hills por puertas hasta `n = 2000`:** `S(2000) = 26;3841E08₁₂ ≈ 30,308`
  (oráculo: error relativo 3·10⁻⁹, por la escala 12 en `sin 355`). El término
  de 355 aporta 24,6 de los 30,3; los picos en numeradores de convergentes
  valen `≈ aₙ₊₁²/(π² pₙ)` y después de 355 no pasan de 10⁻⁵. La convergencia es
  lo que afirma el corolario, fuera del alcance formalizado; no se comprueba
  numéricamente.

Efecto sobre el instrumento: el tope de `--scale` pasa de 100 a 410
(`MAX_SCALE` en `dz-cli`, con test); π a escala 400 tarda 686 s y da 399 cifras
correctas, y a 410, 741 s. A esa escala la fracción continua y los `λₙ` solo fueron viables
reformulados en enteros: un `divrem` de cociente pequeño por paso, sin
normalizar fracciones con Euclides ni dividir a escala completa. Es otra vez la
lección de que la división restauradora domina el coste. Antes, la escala de
división `max(24, --scale)` (`f126e03`, otro equipo) hizo posible medir a 99
cifras. Lo que no se hizo: comparar con `e` como control, verificar el
preprint. Ver §8-§9 de `14`.

---

## 5. Lo que la investigación aprendió sobre su propio instrumento

Tres hallazgos sobre el calculador, no sobre la geometría. Van aquí porque
cualquiera que repita esto tropezará con ellos.

1. **`pi` factura 0 puertas**, por memoización deliberada. Es correcto como
   producto y **veneno como instrumento**: sub-factura el lado euclidiano de
   toda comparación en órdenes de magnitud. Medido de verdad con `4*atan(1)`,
   el perímetro L₂ cuesta **193.718.767 puertas** frente a ~4·10⁴ del L₁.
   Cualquier cifra de este informe medida con `pi` es cota inferior.
2. **El coste no es invariante bajo conmutatividad**: `pi*9` cuesta 19.556 y
   `9*pi` cuesta 46.220, mismo valor y misma profundidad. El conteo mide el
   circuito que se armó, no la función matemática. No es un error del
   instrumento; es lo que el instrumento es.
3. **Los literales pelados se leen en dozenal.** `1/24` es un veintiochavo. Es
   un error *silencioso* —devuelve un número válido y equivocado— y cazó a dos
   de las cuatro líneas. Para decimal, `--input-base 10`.

Y un hallazgo de circuito que no estaba publicado en el repositorio: **leer un
número en dozenal tiene profundidad constante 254** con una cifra o con cinco,
mientras la lectura decimal crece 254 → 478 → 816 → 1.187. Horner es una cadena
secuencial de dependencias; la lectura posicional dozenal no tiene cadena,
porque la posición en el `Vec` ya *es* el valor posicional.

---

## 6. La pregunta del planteamiento, respondida

> ¿Qué propiedades expresadas mediante π son propiedades de L₂, y cuáles
> sobreviven al pasar a L₁? ¿Puede la base 12 hacer más evidente la estructura
> algebraica resultante?

**Primera mitad: casi todo lo interesante es de L₂.** Sobreviven las propiedades
*métricas de primer orden* —perímetros, áreas, distancias, lugares geométricos
definidos por sumas de distancias— y ahí L₁ sí da constantes racionales y a
veces resuelve cosas que L₂ deja sin forma cerrada (§3.3). No sobrevive nada que
dependa de: producto interno (Plancherel, laplaciano autoadjunto), invariancia
rotacional (Fourier radial, trigonometría), o la estructura multiplicativa de ℂ
(§3.1). Y «π ↦ constante racional» no es un cambio, es **varios cambios
distintos**: 4, 2 y `4√3` según la propiedad (el `4√3` depende de la
inmersión euclidiana y no de la norma, §4.14), lo que desinfla del todo la
retórica de «hacer que π sea 4».

**Segunda mitad: no.** La base 12 no hizo más evidente ninguna estructura
algebraica. Hizo más corta la escritura de tercios y sextos, ahorró un factor
~3,5 al cambiar divisiones por multiplicaciones, y expuso en un dígito un
patrón de Goldbach que ya se conocía. No eliminó un solo radical ni un solo
irracional. Y su dominio natural (3, 6, 12) es **casi disjunto** del de L₁
(2, 4, 8), así que la conjunción que la hipótesis daba por buena es
precisamente la parte que no se sostiene.

---

## 7. Lo que no se hizo

Para que nadie lea aquí más de lo que hay:

- Las áreas de Busemann y Holmes–Thompson, que en el cierre original quedaron sin
  explorar (§3.4), sí se calcularon después: sobre el octaedro y el dodecaedro
  rómbico en `03`/`04` (§4.5) y sobre el prisma hexagonal en `07` §2.6 (§4.7).
  No cambiaron el signo de ningún resultado: lo empeoraron, sustituyendo
  irracionales algebraicos por trascendentes.
- Las comparaciones de coste entre `sqrt` y sumas miden **la implementación de
  `dz-sci`**, no la geometría: un `sqrt` mejor cambiaría los factores, aunque no
  el orden de magnitud.
- P2 quedó confirmada «en lo esencial», con matices propios de la línea A; no se
  cerró para todo `p` racional.
- Comparar el coste de d₁ y d₂ es legítimo solo donde el problema admite de
  verdad las dos métricas (rutas en retícula). Donde el problema pide distancia
  euclidiana, `5` y `7` no son dos respuestas a la misma pregunta.
- No se contrastó nada de esto con la literatura. Gołąb es de 1932 y la geometría
  de Minkowski lleva setenta años de trabajo publicado: la dicotomía mod 4 de
  §4.7 y el Teorema A de §4.8 son **probablemente redescubrimientos**, y este
  documento no está en condiciones de afirmar lo contrario.
- De la familia de cuerpos con simetría 12, solo se examinaron n-ágonos regulares
  y su extensión prismática. El espacio de convexos centralmente simétricos con
  `D₆` o `D₁₂` **sin** ser regulares queda sin explorar (`07` §6), y la
  clasificación exhaustiva de §4.7 vale para los regulares, no para ése.
- Nada de esto se ha sometido a revisión externa. Los cuatro informes fueron
  producidos por agentes que trabajaron contra el mismo protocolo y verificando
  con el mismo binario; las afirmaciones centrales de la línea D se
  re-verificaron a mano en la síntesis, las demás no.
- `09` cubre solo χ=0 (un toro simétrico construido a mano); no se construyó un
  segundo toro asimétrico para confirmar que la cancelación de defectos no es
  artefacto de la simetría cúbica, ni se exploró χ<0 discreto (más de un
  agujero, o no orientable).
- `10` cita, sin rederivar, la **fórmula** de la capacidad logarítmica del
  cuadrado (`√2·Γ(1/4)²/(4π^{3/2})`) — Schwarz–Christoffel de un cuadrado
  excede lo razonable para una derivación autocontenida. *Actualizado
  2026-09-14*: el **valor** ya no depende solo de esa cita — se implementó
  `gamma` en el core (`crates/dz-sci/src/gamma.rs`) precisamente por este
  hallazgo, y `dz-cli eval "sqrt(2)*gamma(0;3)^2 / (4*exp(1;6*ln(pi)))"` lo
  recalcula desde las puertas propias, coincidiendo con la cita hasta donde
  ésta tenía precisión. Sigue sin una segunda fuente bibliográfica de la
  fórmula en sí. Tampoco se exploró si alguna bola `Lp`,
  `p∈(1,∞)`, tiene capacidad logarítmica racional — la familia `Lp` regaló
  racionales baratos para perímetro/diámetro (§4.7–§4.8) y no se comprobó si
  el mismo truco funciona aquí.
- `15` demuestra el Teorema B solo para politopos. Para las bolas `Lp` lisas no
  hay lema, y no se sabe si Busemann/HT pueden salir racionales. Tampoco se
  calculó con el binario ningún Busemann/HT en 4D, y la dualidad de
  Holmes–Thompson se usó como control sin rederivarla.
- `16` es solo 2D y en doble precisión, con los exponentes de Edgeworth
  derivados de forma esquemática. No trata la red cúbica ni la FCC en 3D, ni las
  leyes estables (la única vía en la que L₁ sobrevive al límite, en el lado de
  Fourier), y todo lo que contiene es casi seguro conocido (FHP 1986, D2Q9 1992).
- La lista de «ingredientes» abierta por la reinterpretación de Kryon (§4.9,
  §4.10, §4.11) no se declaró cerrada por el usuario; puede continuar con un
  cuarto ingrediente si se retoma.
- `11` deja explícitamente abierta la forma cuadrática `x²+y²=1` sobre `ℚ_p`
  (símbolo de Hilbert, Hasse–Minkowski) como vía no perseguida, y señala como
  posible cuarto ingrediente si la serie continúa: la línea B (`06`, fórmulas
  de adición imposibles en L₁ porque `D₄` es finito) y el ingrediente 3
  (grupo de unidades p-ádico profinito, tampoco continuo) podrían ser
  instancias de un mismo principio — «ningún grupo totalmente disconexo aloja
  una familia continua no trivial» — sin investigar si es una obstrucción
  común genuina o una coincidencia de vocabulario.

---

## 8. Revisión crítica (2026-09-13)

Una revisión posterior rehízo a mano las derivaciones centrales y volvió a medir
las cifras clave con el binario. **Se sostienen**: las superficies de Hausdorff,
Busemann y Holmes–Thompson del octaedro (`4√3`, `16π/3`, `48/π`) y del
dodecaedro rómbico (`6√2`, `4π`, `36/π`); la robustez rotacional del 3 del
hexágono; la dicotomía mod 4 con Niven; el perímetro `8a−4c`; la bisectriz L₁
con dos cuadrantes; la imposibilidad de las fórmulas de adición; el Teorema A; y
las cifras `345*10`, `345*E`, `345/3`, `345*0;4`, que reproducen exactas.

Lo que se corrigió, dejado visible por el mismo estándar de `07` §5.7:

| § | decía | corregido a | ¿mueve un veredicto? |
|---|---|---|---|
| 4.4 | residuo `{4,↊}` vs `{2,8}` «sin que la teoría prediga diferencia», «efecto de rango finito» | sesgo de Chebyshev, con signo y magnitud reproducidos (+1,100% y +0,581%) | no; quita un misterio |
| 4.3 | escalar por la base, «refutado por medición» | límite del multiplicador escolar y del andamiaje del evaluador; la ventaja posicional existe pero no es accesible desde expresiones | sí, de refutación a no concluyente |
| 4.7 | área del hexágono y volumen del prisma irracionales; «ningún cuerpo tiene las tres constantes racionales» | área y volumen de Lebesgue no son invariantes de una norma; el hexágono afín entero tiene área 3 y prisma de volumen 6 | sí, cae la forma fuerte de la extensión de P1 |
| 4.8 | «ningún n-ágono regular tiene vértices racionales salvo 3, 4, 6» | afínmente regular: 3, 4, 6; euclidianamente regular: solo 4 | no |
| 4.8 | crecimiento exponencial del denominador «estructural» | artefacto de una hipotenusa por vértice; un denominador común lo frena | no; la penalización queda sin medir en su mejor caso |
| 4.8 | factores primos de las hipotenusas «2 y los ≡ 1 (mod 4)» | hipotenusas primitivas son impares: solo los `≡ 1 (mod 4)` | no |
| 3.1 | el límite de Gelfand como «resultado más fuerte» | el límite es trivial por equivalencia de normas; el contenido es Ostrowski / Gelfand–Mazur | no; cambia el mérito |
| 3.3 | `a = 2c` da un cuadrado macizo | solo con focos diagonales | no |
| 4.1 | 3,52× | ligado a `DEFAULT_DIV_SCALE = 24` | no |
| 3.4 | «la racionalidad no se hereda en la frontera», en general (corregido 2026-09-25, `15`) | falso con Hausdorff: racional en 4D (`16/3`), en el cubo (`24`) y en una copia isométrica del octaedro (`3`); verdadero como teorema con Busemann/HT, pero para todo politopo racional, no solo para L₁ | sí: §3.4 deja de ser un resultado sobre L₁ |
| 4.5 | seis vías «independientes», cero éxitos | las cuatro de Busemann/HT son instancias del Teorema B de `15` | no; las explica |

El veredicto de §1 sobre H₁ y el antagonismo `D₄` contra `2²·3` no cambian.
