# Síntesis — Geometría L₁ + representación dozenal

Cierre de la investigación abierta en `00-plan-investigacion.md` sobre la
hipótesis de `planteamientos.md`. Cuatro líneas (A constantes, B lugares,
C análisis, D dozenal) corrieron en paralelo contra el mismo protocolo, con el
calculador de puertas de este repositorio como instrumento. Los informes
completos están en `01-hallazgos-{A,B,C,D}-*.md`; las cifras compartidas y las
trampas del instrumento, en `02-referencia-medida.md`. Los documentos `03` a `08`
son extensiones posteriores al cierre, abiertas por preguntas del usuario sobre
hallazgos concretos; cada una está resumida en §4.5, §4.6, §4.7 y §4.8.

Fecha de cierre: 2026-09-12. Última extensión incorporada: 2026-09-13 (`07` y `08`).

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
donde L₁ no podía, pero ningún cuerpo de la familia tiene sus tres constantes
racionales a la vez, y la familia de n-ágonos regulares queda **clasificada por
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
| **P1** | no hay «π taxicab» único | **confirmada, y extendida a una forma más fuerte** — la constante depende de la propiedad conservada (4 para perímetro/diámetro, 2 para área, `4√3` para superficie en 3D) y §4.7 añade que **ningún** rediseño de la simetría de la bola unidad hace racionales las tres a la vez |
| **P2** | L₁ y L∞ son las únicas `Lp` con razón entera | **confirmada** en lo esencial (A la matiza; ver su §4) |
| **P3** | la trigonometría L₁ pierde las fórmulas de suma | **confirmada, y más fuerte de lo predicho** |
| **P4** | Fourier bajo L₁ no da Plancherel ni se simplifica | **confirmada** |
| **P5** | el efecto dozenal se reduce a `N_periodicos` | **cae en la letra, sobrevive en el espíritu** |
| **P6** | Goldbach no cede nada geométrico | **confirmada**; el resultado es reformulación, no descubrimiento |

---

## 3. Los cuatro resultados que valen

De todo lo producido, esto es lo que sobreviviría a una revisión hostil.

### 3.1 L₂ no es una convención: la multiplicación de ℂ la reconstruye (línea C)

El resultado más fuerte de la investigación, y va **contra** la hipótesis en su
forma ambiciosa. La fórmula de Gelfand aplicada al módulo L₁ da

```
lim_{n→∞} |zⁿ|₁^{1/n} = |z|₂
```

Es decir: si se itera la multiplicación compleja midiendo con L₁, **L₂ reaparece
sola en el límite**. No se puede «elegir» L₁ para el análisis complejo y
esperar que la geometría euclidiana se quede fuera; la estructura multiplicativa
de ℂ la selecciona. Es casilla (I) en su forma más dura — no «L₂ es cómoda»,
sino «L₂ está determinada».

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
rareza. Ahí K **sube**: pérdida limpia. La elipse L₁ degenera hasta lo absurdo
(con `a = 2c` el «lugar geométrico» es un cuadrado macizo).

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

**3,52× más barato, mismo valor.** Base 10 no puede hacer esto para el 3. Y la
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

### 4.3 El lead más prometedor murió midiendo

La sospecha era que escalar por la base fuese gratis. **No lo es**, y el
instrumento lo desmiente sin ambigüedad:

```
345*10  →  97.252 puertas        345*E   →  96.037     (¡la base cuesta MÁS!)
345/10  → 606.860                345/9   → 512.090
```

El bucle recorre los dígitos del segundo operando, y `12` escrito `10` tiene
dos dígitos mientras `E` tiene uno. La supuesta ventaja posicional se vuelve
penalización. Un lead teóricamente atractivo, refutado por medición en una
línea de comandos: es el mejor argumento para haber instrumentado esto.

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
Residuo declarado: dentro de los no múltiplos de 3, `{4,↊}` queda un 0,58%%
por encima de `{2,8}` sin que la teoría prediga diferencia; era 1,09%% midiendo
hasta 250.000, así que es efecto de rango finito.

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
la lista de tareas.** Ninguna produce un racional que sea a la vez invariante
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
Y el área del propio hexágono es irracional, y el cuerpo 3D que extiende su
simetría (el prisma hexagonal) falla ya en el **volumen**, antes siquiera de
llegar a la superficie donde el octaedro fallaba.

**El dodecágono merece un párrafo aparte porque es la trampa perfecta.** Es el
candidato con más «12» en el nombre (`D₁₂`, orden 24) y resulta ser el **peor**
de los cuatro en perímetro/diámetro (`24−12√3`, irracional) y el **mejor** en
área (exactamente **3**, entero). Pero esa área racional no tiene relación
alguna con base 12: es el **teorema de Niven** (1956) — `sin θ` racional para `θ`
múltiplo racional de `π` solo en `{0, ±1/2, ±1}` — que caracteriza `n=4` y `n=12`
como los únicos n-ágonos regulares de área racional, en cualquier geometría y en
cualquier base numérica. El «12» del resultado es que `360°/12 = 30°`, no que
`12 = 2²·3`. Casilla **(III) aparente, (I) real**: no es ni siquiera un efecto de
representación, es aritmética de ángulos.

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

Extensión de P1, en su forma fuerte: no es solo que no exista un «π taxicab»
único; es que **ningún rediseño de la simetría de la bola unidad produce un
cuerpo cuyas constantes de perímetro/diámetro, área y superficie 3D sean
racionales a la vez**. Cada candidato gana en una y pierde en otra, y las
obstrucciones son al menos tres y son independientes entre sí (Niven para las
áreas, Gołąb para la extremal de perímetro/diámetro, la herencia directa de `√2`
o `√3` del propio polígono) — el mismo patrón de «varias causas separadas, no
una» que §4.6 encontró para los fracasos de la superficie del octaedro.

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
| isotropía | ningún n-ágono regular tiene vértices racionales salvo `n = 3, 4, 6` |
| canonicidad | hay un continuo de sucesiones válidas, ninguna privilegiada |
| **honestidad del método** | los polígonos de `22/7` **reciben el número objetivo como entrada de una ecuación lineal** — el movimiento que §1 del plan prohíbe, en su forma más literal |

Y como método de aproximación es catastrófico: a error `≈1,2·10⁻³` el racional
del 96-gono necesita **278 cifras de denominador**, frente a **1** de `22/7`;
en el paso siguiente la penalización es ×294. La razón es estructural: el error
decae como `π³/(3n²)` —cuadrático en `n`— mientras el denominador crece
**exponencialmente** (es el producto de las hipotenusas pitagóricas usadas).
**No es un método de aproximación: es una reformulación geométrica de la
densidad de ℚ.**

**Y el sabor dozenal sale negativo, no neutro** (Prop. 4, hallazgo nuevo): los
puntos racionales del círculo tienen denominadores de la forma `p²+q²`, y
**ningún número de esa forma es divisible por 3** (si `3 | p²+q²` con
`gcd(p,q)=1`, los cuadrados mód 3 fuerzan `p ≡ q ≡ 0`, contradicción). Sus
factores primos son 2 y los `≡ 1 (mod 4)`, lista que **empieza por 5**. De ahí
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
distintos**: 4, 2 y `4√3` según la propiedad, lo que desinfla del todo la
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
