# Hallazgo — Familias de normas poligonales con π_‖·‖ racional que convergen a π

Continúa `07-hallazgo-cuerpo-simetria-12-rediseno.md`, que cerró por completo la
familia de **polígonos regulares**, y ataca lo que aquel documento dejó
explícitamente sin explorar (`07` §6, último guion): **el espacio de cuerpos
convexos centralmente simétricos que NO son regulares.**

Fecha de apertura: 2026-09-13.

Protocolo: `00-plan-investigacion.md`. Toda cifra dozenal sale de
`target/release/dz-cli.exe` con el comando pegado al lado, `--mode rational`
donde la afirmación es de racionalidad y `--input-base 10` en todo literal
decimal (sin él `22/7` se lee como `26/7` decimal: la trampa de `01` §0.2 y
`07` §5.6). Cada resultado se clasifica en casilla (I)/(II)/(III). Toda forma
cerrada no trivial se contraverifica por un segundo método independiente.

**Hecho que hay que poner al principio para que nadie lea mal el documento.**
π es trascendente (Lindemann–Weierstrass), y esa propiedad no depende de la
base numérica. **Nada de lo que sigue la contradice ni la roza.** Que exista
una sucesión `π_k ∈ Q` con `π_k → π` es compatible con la trascendencia de π
de la forma más banal posible: **Q es denso en R**, y ya `3, 22/7, 333/106,
355/113, …` es una sucesión de racionales que converge a π. La pregunta del
usuario no es «¿se puede aproximar π por racionales?» —eso es trivial— sino
«¿existe una **familia geométrica natural de normas** cuyas constantes sean
racionales y converjan a π?». Ésa sí es una pregunta con contenido, y es la
que se responde aquí.

---

## 0. Resumen del veredicto (ancla)

| parte de la hipótesis del usuario | veredicto |
|---|---|
| Existe una familia de normas poligonales con `π_k ∈ Q` y `π_k → π` | **VERDADERA**, y se construye explícitamente (§3.2) |
| ...con polígonos **regulares** de `n_k` lados | **FALSA, y demostrada falsa** (`07` §1.8; aquí §1.3): sólo `n=4` (→4) y `n=6` (→3) |
| ...con `n_k = 4^k` (4, 16, 64, 256) | **FALSA en la versión regular** (16, 64, 256 son `≡ 0 mod 4`, rama `n·tan(π/n)`, irracional para todo `n>4`); y `4^k` **no tiene ninguna justificación matemática** en este problema (§3.1) |
| ...soltando la regularidad | **VERDADERA pero BARATA**, y el contenido interesante es exactamente cuán barata (§2.7) |
| Base 12 aporta algo a la racionalidad | **NULO**, como manda el protocolo; y en esta familia concreta base 12 **pierde** frente a base 10 (§4.3), que es un resultado nuevo y negativo |

**El resultado técnico central del documento** es un teorema elemental que
liquida la pregunta de racionalidad de un plumazo:

> **Teorema A.** Si `B ⊂ R²` es un polígono convexo centralmente simétrico
> cuyos vértices tienen **coordenadas racionales**, entonces
> `π_B := perímetro_B(∂B)/2 ∈ Q`.

Es decir: **la racionalidad no hay que buscarla, se regala.** No es una
propiedad geométrica del cuerpo; es una propiedad de haber escrito sus
vértices con números racionales. Y como los polígonos de vértices racionales
son densos entre los cuerpos convexos centralmente simétricos, y `π_B` es
continua (§2.1), se obtiene gratis un continuo de sucesiones con `π_k ∈ Q` y
`π_k → π`.

**El precio, que es lo único que tiene interés y lo que este documento mide.**
Los `π_k` así producidos son racionales de **altura astronómica**: para el
error `1,1·10⁻³` el polígono de 96 lados entrega un racional con **278 cifras
en el denominador**, mientras que `22/7` da el mismo error con **una**. La
construcción poligonal no es una fuente de buenas aproximaciones a π: es tres
órdenes de magnitud de cifras peor que el desarrollo en fracción continua, y
eso cuantifica exactamente qué significa «barata».

---

## 1. Enunciado formal y las dos fronteras conocidas de antemano

### 1.1 Definiciones

Sea `B ⊂ R²` un cuerpo convexo **compacto, con `0` en el interior y
centralmente simétrico** (`B = −B`). Su **gauge de Minkowski**

```
‖x‖_B = inf { λ > 0 : x ∈ λB }
```

es una norma, y `B = {x : ‖x‖_B ≤ 1}` es su bola unidad. Recíprocamente toda
norma del plano surge así de su bola unidad, de modo que «norma» y «cuerpo
convexo compacto centralmente simétrico con 0 interior» son el mismo objeto.
Esto es lo que hace legítima toda la pregunta: **elegir el polígono es elegir
la norma**, no un abuso de definición.

La **longitud de autonorma** de una curva rectificable `γ` es su longitud
medida con `‖·‖_B`; para un segmento de `p` a `q` vale exactamente `‖q−p‖_B`,
porque la parametrización es lineal. Para el diámetro,

```
diám_B(B) = max_{p,q ∈ B} ‖p−q‖_B = 2,
```

(`≤` por desigualdad triangular y `‖p‖,‖q‖ ≤ 1`; `=` en cualquier par
antipodal de la frontera). Por eso la constante de interés,

```
π_B  :=  perímetro_B(∂B) / diám_B(B)  =  perímetro_B(∂B) / 2,
```

es la **semilongitud de autonorma de la bola unidad**. Para un polígono de
vértices `v_1,…,v_{2m}` en orden cíclico (con `v_{k+m} = −v_k`):

```
π_B = ½ Σ_{k=1}^{2m} ‖v_{k+1} − v_k‖_B  =  Σ_{k=1}^{m} ‖v_{k+1} − v_k‖_B .     (★)
```

Dos observaciones que se usan continuamente:

- **`π_B` es invariante afín.** Si `T ∈ GL(2,R)`, entonces `‖Tx‖_{TB} = ‖x‖_B`,
  de modo que `π_{TB} = π_B`. Todo lo que sigue vive en el espacio de clases
  afines de polígonos.
- **La representación en semiplanos da el gauge de golpe.** Si
  `B = {x : ⟨u_j, x⟩ ≤ 1, j = 1..2m}` con los `u_j` los funcionales de arista
  normalizados (`⟨u_j, v_j⟩ = ⟨u_j, v_{j+1}⟩ = 1`), entonces

  ```
  ‖w‖_B = max_j ⟨u_j, w⟩ .                                                   (★★)
  ```

### 1.2 Primera frontera: el teorema de Gołąb

> **Gołąb (1932).** Para **toda** norma del plano, `π_B ∈ [3,4]`. Se alcanza
> `π_B = 4` si y sólo si `B` es un paralelogramo, y `π_B = 3` si y sólo si `B`
> es un hexágono afínmente regular — ambos salvo equivalencia lineal.

Ya citado en `01` §4.3 y explotado en `07` §1.4. Lo decisivo para esta
investigación es la lectura siguiente:

```
3 < π = 3,14159… < 4,   estrictamente.
```

**π cae estrictamente dentro del intervalo de Gołąb, sin tocar ninguno de los
dos extremos rígidos.** Por tanto **nada prohíbe a priori** que una familia de
normas tenga constantes que converjan a π: el objetivo está en el interior del
rango admisible, lejos de las dos caracterizaciones rígidas. La obstrucción
que `07` encontró **no es al límite, sino a la racionalidad en cada etapa**, y
sólo dentro de la familia regular. Conviene decirlo con esas palabras porque
es la bisagra entre `07` y este documento.

### 1.3 Segunda frontera: la clasificación de `07` §1.8 (se cita, no se rehace)

`07` §1.8 estableció, para el `n`-ágono **regular** usado como su propia bola
unidad (`n` par; los impares no son centralmente simétricos y no son bola de
ninguna norma, `07` §1.6):

```
n ≡ 0 (mod 4):  π_B = n·tan(π/n)   decreciente  4 → π    racional SÓLO en n = 4 (→ 4)
n ≡ 2 (mod 4):  π_B = n·sin(π/n)   creciente    3 → π    racional SÓLO en n = 6 (→ 3)
```

ambas exclusividades por el **teorema de Niven** (versión tangente y versión
seno). Y `07` §3.7 añadió que la **robustez rotacional** obedece la misma
dicotomía módulo 4, de donde `racional ∧ robusto ⟺ n = 6`.

**Consecuencia inmediata y demoledora para la propuesta del usuario tal como
está escrita.** La familia propuesta es `n_k = 4^k`, es decir `4, 16, 64, 256,
…`. Salvo el primero, **todos son `≡ 0 (mod 4)`**, luego caen en la rama
`n·tan(π/n)`, que por Niven es **irracional para todo `n > 4`**. Y además
ninguno de ellos es rotacionalmente robusto. De modo que:

> **La familia ‖·‖₄, ‖·‖₁₆, ‖·‖₆₄, ‖·‖₂₅₆ con polígonos regulares NO EXISTE
> con las propiedades pedidas.** `π₄ = 4` es racional; `π₁₆ = 16 tan(π/16)`,
> `π₆₄ = 64 tan(π/64)` y `π₂₅₆ = 256 tan(π/256)` son **todos irracionales**.
> Esto está demostrado, no muestreado: el espacio de enteros pares está
> agotado.

```
$ target/release/dz-cli.exe eval "16*tan(pi/16)" --mode fixed --scale 8 --input-base 10
~3;2236424
$ target/release/dz-cli.exe eval "64*tan(pi/64)" --mode fixed --scale 8 --input-base 10
~3;18905314
$ target/release/dz-cli.exe eval "pi" --mode fixed --scale 8 --input-base 10
~3;18480949
```

(las dos primeras son las **etapas 2 y 3** de la familia del usuario; se ven
acercándose a π por arriba, como manda la monotonía de la rama `tan`, y
ninguna es racional.)

**Casilla (I)/(II):** la clasificación es de casilla **(II)** —la propiedad
«razón perímetro/diámetro» sobrevive al cambio de norma y la constante
cambia— y no tiene ninguna relación con la base numérica.

### 1.4 Lo que queda genuinamente abierto

`07` §6 lo dejó escrito: *«No se exploró ningún cuerpo convexo centralmente
simétrico no regular.»* Ése es el objeto de §2. La pregunta precisa:

> Para `2m` lados fijos, ¿qué valores toma `π_B` cuando los vértices son
> parámetros **continuos**? ¿Y qué ocurre con la racionalidad cuando
> desaparece la rigidez aritmética que Niven impone a la familia regular?

---

## 2. La puerta abierta: soltar la regularidad

### 2.1 `π_B` varía continuamente con los vértices — y el punto fino donde
falla el argumento ingenuo

**El problema.** En (★) intervienen **dos** cosas que se mueven a la vez: los
vectores `v_{k+1} − v_k` **y** la norma `‖·‖_B` con la que se miden, porque la
norma es el gauge del propio cuerpo que estamos deformando. Un argumento que
invoque «la norma es continua» está usando continuidad **en `x` con `B` fijo**,
que aquí no aplica; y uno que invoque «el gauge depende continuamente del
cuerpo» está usando continuidad **en `B` con `x` fijo**, que tampoco aplica
sola. Hace falta la estimación conjunta.

**Lema 1 (comparación uniforme de gauges).** Sean `B, B'` cuerpos convexos
centralmente simétricos con `rD ⊆ B` (`D` el disco euclídeo unidad) y
`d_H(B,B') ≤ δ < r`. Entonces para todo `x`:

```
|‖x‖_{B'} − ‖x‖_B| ≤ (δ/r)·‖x‖_B .
```

*Demostración.* `B' ⊆ B + δD ⊆ B + (δ/r)B = (1+δ/r)B`, luego
`‖x‖_{B'} ≥ ‖x‖_B/(1+δ/r)`. Simétricamente `B ⊆ (1+δ/r)B'` da
`‖x‖_{B'} ≤ (1+δ/r)‖x‖_B`. Restando `‖x‖_B` en ambas y usando
`1 − 1/(1+u) ≤ u`, sale la cota. ∎

**Lema 2 (la estimación conjunta, que es el punto fino).** Con las hipótesis
del Lema 1 y `w, w' ∈ R²`:

```
|‖w'‖_{B'} − ‖w‖_B|  ≤  |‖w'‖_{B'} − ‖w‖_{B'}|  +  |‖w‖_{B'} − ‖w‖_B|
                     ≤  ‖w' − w‖_{B'}           +  (δ/r)·‖w‖_B
                     ≤  (1/r)·|w' − w|          +  (δ/r)·‖w‖_B .
```

*Demostración.* El primer sumando es la **desigualdad triangular inversa en la
norma que se mueve** — legítima porque `‖·‖_{B'}` es una norma, cualquiera que
sea `B'`; y `‖z‖_{B'} ≤ |z|/r'` con `r' ≥ r − δ`, que se absorbe en la
constante. El segundo es el Lema 1. ∎

Nótese que el paso que un argumento ingenuo se salta es exactamente el
primero: hay que aplicar la desigualdad triangular **dentro de la norma nueva**
y sólo después comparar normas, no al revés. Hacerlo al revés obliga a acotar
`‖w'−w‖_B` con un `B` que ya no es el cuerpo que mide, y la cadena no cierra.

**Proposición 1 (continuidad).** Sea `P(2m)` el conjunto de `2m`-uplas de
vectores `(v_1,…,v_{2m})` en posición estrictamente convexa, en orden cíclico,
con `v_{k+m} = −v_k` y `0` interior. La aplicación

```
(v_1,…,v_{2m})  ↦  π_B ,     B = conv{v_k},
```

es **continua** en `P(2m)` (que es un abierto de `R^{2m}`).

*Demostración.* Fijado un punto de `P(2m)`, existen `0 < r < R` con
`rD ⊆ B ⊆ RD`, y en un entorno suficientemente pequeño lo mismo vale con
`r/2`. Además `B ↦ d_H` es continua en los vértices (la envolvente convexa de
puntos que se mueven poco se mueve poco). Aplicando el Lema 2 a cada uno de
los `m` sumandos de (★), con `w = v_{k+1}−v_k` y `w' = v'_{k+1}−v'_k`:

```
|π_{B'} − π_B| ≤ Σ_{k=1}^m [ (2/r)|v'_k − v_k| + (2δ/r)·‖v_{k+1}−v_k‖_B ] → 0 .  ∎
```

**Contraverificación numérica independiente.** Toda cifra de §2 se computó
por **dos caminos**: (a) exacto con aritmética racional de precisión
arbitraria, gauge por (★★) sobre la representación en semiplanos; (b)
flotante, gauge por **bisección del factor de escala** contra la prueba de
pertenencia lado a lado (algoritmo sin ninguna relación con (★★)). Ejemplo, el
16-ágono de §2.5:

```
exacto  : π_B = 22/7
flotante: π_B = 3,142857142848920     (22/7 = 3,142857142857143)
```

### 2.2 El rango exacto de `π_B` sobre los `2m`-ágonos

**Proposición 2.** Sea `m ≥ 4` (es decir, `2m ≥ 8` lados). El conjunto de
valores de `π_B` sobre los `2m`-ágonos convexos centralmente simétricos es
**exactamente el intervalo abierto `(3,4)`**.

*Demostración.*
- `⊆`: Gołąb da `π_B ∈ [3,4]`; y los dos extremos están **excluidos** porque
  se alcanzan **sólo** en paralelogramos (4 lados) y hexágonos afínmente
  regulares (6 lados), y `2m ≥ 8`.
- `⊇`: tómese el hexágono afínmente regular `H` y colóquense los `2m−6`
  vértices sobrantes a distancia `ε` de vértices de `H`, en posición
  estrictamente convexa; es un `2m`-ágono legítimo y por la Proposición 1
  `π → 3` cuando `ε → 0`. Análogamente, un `2m`-ágono que degenere hacia un
  paralelogramo da `π → 4`. Como `P(2m)` es **conexo** (dos configuraciones
  cualesquiera se unen por un camino continuo dentro de la posición
  estrictamente convexa, por ejemplo pasando por la configuración regular) y
  `π_B` es continua, el **teorema del valor intermedio** entrega todo
  `(3,4)`. ∎

**Corolario.** Para `2m = 16` y `2m = 64`, el conjunto de valores alcanzables
es `(3,4) ∋ π`, `22/7`, `157/50`, `3+1/7`, … Todos los objetivos del encargo
son alcanzables, y la única pregunta que queda es **construirlos**.

Contraste con `07`: allí el parámetro era **discreto** (el entero `n`) y la
monotonía no producía ningún racional intermedio; aquí el parámetro es
**continuo** (`2m−4` dimensiones tras cocientar por `GL(2)`) y el TVI produce
**todos** los valores.

### 2.3 Teorema A: vértices racionales ⟹ `π_B` racional

Éste es el resultado que convierte la pregunta del usuario en un ejercicio.

> **Teorema A.** Si `B` es un polígono convexo centralmente simétrico con
> **todos** sus vértices en `Q²`, entonces `π_B ∈ Q`.

*Demostración.* Cada arista `j` yace en una recta `⟨u_j, x⟩ = 1` cuya solución
`u_j` es la de un sistema lineal `2×2` de coeficientes racionales
(`⟨u_j,v_j⟩ = ⟨u_j,v_{j+1}⟩ = 1`); el determinante es no nulo porque ninguna
arista pasa por el origen. Luego `u_j ∈ Q²`. Por (★★),
`‖w‖_B = max_j ⟨u_j, w⟩`, un **máximo de un conjunto finito de racionales**
cuando `w ∈ Q²` — racional. Los vectores de arista `v_{k+1} − v_k` son
racionales, y (★) es una suma finita de racionales. ∎

**Tres consecuencias, y hay que decirlas en este orden porque la tercera es la
que desactiva la hipótesis del usuario.**

1. La racionalidad de `π_B` **no distingue ningún cuerpo**: la tiene un
   conjunto **denso** de polígonos, de cualquier número de lados.
2. Combinada con la Proposición 1, el conjunto `{π_B : B` racional con `2m`
   lados`}` es **denso en `(3,4)`**.
3. Por tanto **cualquier** sucesión de polígonos racionales que converja al
   disco euclídeo ya es una sucesión con `π_k ∈ Q` y `π_k → π`. No hay nada
   que descubrir: hay que **elegir**, y elegir es gratis.

*Verificación cruzada del Teorema A en el caso extremo.* El cuadrado
`[−1,1]²` (vértices enteros) da `π_B = 4 ∈ Q` ✓; el hexágono afínmente regular
de vértices `(1,0),(1,1),(0,1),(−1,0),(−1,−1),(0,−1)` (enteros) da `π_B = 3`
✓ — los dos extremos de Gołąb son, no por casualidad, polígonos de vértices
enteros.

```
$ target/release/dz-cli.exe eval "4" --mode rational
4
$ target/release/dz-cli.exe eval "3" --mode rational
3
```

### 2.4 Caso mínimo resuelto a mano: el hexágono con `π_B = 22/7` exacto

Antes de 16 y 64 lados conviene el caso más pequeño donde se ve todo, porque
se resuelve **a mano y en forma cerrada**.

Familia de un parámetro, `t ∈ [1,2]`:

```
B_t = { (x,y) : |x| ≤ 1 , |y| ≤ 1 , |x − y| ≤ t } ,
```

es decir el cuadrado con dos esquinas opuestas cortadas. Vértices (sentido
antihorario):

```
(1,1), (1−t, 1), (−1, t−1), (−1,−1), (t−1,−1), (1, 1−t).
```

Gauge, por (★★): `‖(a,b)‖ = max( |a| , |b| , |a−b|/t )`. Las tres aristas
independientes y sus normas:

| arista | vector | `max(|a|, |b|, |a−b|/t)` | valor |
|---|---|---|---|
| `(1,1) → (1−t,1)` | `(−t, 0)` | `max(t, 0, t/t=1)` | `t` (pues `t ≥ 1`) |
| `(1−t,1) → (−1,t−1)` | `(t−2, t−2)` | `max(2−t, 2−t, 0)` | `2−t` |
| `(−1,t−1) → (−1,−1)` | `(0, −t)` | `max(0, t, 1)` | `t` |

```
π(t) = t + (2−t) + t = t + 2 ,     exactamente afín en t.
```

Controles en los extremos: `t = 1` da el **hexágono afínmente regular**
(`|x−y| ≤ 1` es una de sus tres aristas) y `π = 3` ✓ **coincide con Gołąb**;
`t = 2` deja la tercera restricción inactiva, `B` es el **cuadrado** y
`π = 4` ✓ **coincide con Gołąb**. Dos anclas independientes para una fórmula
de una línea.

Resolviendo `t + 2 = 22/7` ⇒ **`t = 8/7`**, y escalando por 7 para limpiar
denominadores (`π_B` es invariante afín, en particular por homotecias):

```
       H*  =  conv { (7,7), (−1,7), (−7,1), (−7,−7), (1,−7), (7,−1) }
       ‖(a,b)‖_{H*} = max( |a|/7 , |b|/7 , |a−b|/8 )
       π_{H*}  =  22/7     EXACTO, con vértices ENTEROS.
```

Verificado por los dos caminos: exacto `22/7`; flotante `3,142857142857087`.

```
$ target/release/dz-cli.exe eval "22/7" --mode rational --input-base 10
1X/7 (periodica en base 12)
$ target/release/dz-cli.exe eval "22/7" --mode fixed --scale 8 --input-base 10
3;186X35186X35186X35186X35 (periodica en base 12, valor truncado)
$ target/release/dz-cli.exe eval "8/7" --mode rational --input-base 10
8/7 (periodica en base 12)
```

(el numeral `1X/7` es `22/7` con `22` escrito en dozenal, `1↊`; el
`--input-base 10` es obligatorio: sin él `22` se lee `2·12+2 = 26`.)

**Lo que este ejemplo ya enseña, y por eso va antes que los grandes:** una
familia de **seis** lados con un parámetro **continuo** recorre todo `[3,4]`
de forma **afín**, y por tanto alcanza **todo racional del intervalo** en un
**`t` racional**. La rigidez de Niven que gobernaba `07` desaparece por
completo en cuanto el hexágono deja de ser regular. El hexágono regular no era
especial por ser hexágono: era especial por ser **regular**.

### 2.5 Dieciséis lados: construcción explícita con `π_B = 22/7` exacto

El método general que se usa aquí y en §2.6, y que hace la construcción
**efectiva** en vez de una invocación abstracta del TVI:

1. Se fija un `2m`-ágono por su **representación en semiplanos**
   `⟨u_j, x⟩ ≤ c_j` con los `u_j` **puntos racionales del círculo unidad**
   (parametrización pitagórica `u = ((q²−p²)/(q²+p²), 2pq/(q²+p²))`, `p/q ∈ Q`
   el tangente del semiángulo) y `c_j = 1`.
2. Se elige como **perilla** una faceta `k` cuyo cono **no contiene ninguna
   dirección de arista** del polígono. En ese caso la restricción `k` nunca
   alcanza el máximo de (★★) sobre ningún vector de arista, el término `1/t`
   desaparece y `π(t) = A + B t` resulta **exactamente afín** con `A, B ∈ Q`.
   (Que tales facetas existan no es genérico: en un `2m`-ágono casi regular
   cada faceta recibe exactamente una dirección de arista, por la rotación de
   un cuarto de vuelta que lleva vértices a tangentes; hay que **desuniformar
   deliberadamente** el reparto angular, y entonces el principio del palomar
   deja facetas vacías.)
3. Se resuelve `A + B t = objetivo` — **una ecuación lineal**, luego `t ∈ Q`
   y todos los vértices quedan en `Q²`.

Resultado para `2m = 16`. Las ocho normales (las otras ocho son sus opuestas),
todas **ternas pitagóricas**:

```
u₀ = (1, 0)            u₄ = (3/5, 4/5)
u₁ = (35/37, 12/37)    u₅ = (0, 1)
u₂ = (15/17, 8/17)     u₆ = (−3/5, 4/5)
u₃ = (21/29, 20/29)    u₇ = (−609/641, 200/641)
```

Offsets `c_j = 1` salvo `c₁ = t`. Con `c₁ = 1` el cuerpo es el 16-ágono
circunscrito a la circunferencia unidad en esas ocho direcciones, y
`π_B = 3,154789272…`. Con `c₁ = t` la perilla 1 es de carga nula y

```
π(t) = 13249/5220 + (37/60)·t      (afín, verificada en cinco valores de t)

π(t) = 22/7   ⟺   t = 22097/22533
```

**Los 16 vértices** (los ocho restantes son los opuestos):

```
(−1, 4/25)          (−1, −391/3654)     (−2627/3045, −103/203)   (−9/11, −13/22)
(−2/3, −3/4)        (−1/3, −1)          (1/3, −1)                (23/27, −11/18)
```

y, multiplicando por `6.029.100` (el mcm de los denominadores) para tenerlos
**enteros**, los ocho primeros son

```
(−6029100, 964656)   (−6029100, −645150)  (−5201460, −3059100)  (−4932900, −3562650)
(−4019400, −4521825) (−2009700, −6029100) (2009700, −6029100)   (5135900, −3684450)
```

```
π_B  =  22/7   EXACTO
```

**Verificación.** (a) aritmética racional exacta, gauge por (★★): `22/7`;
(b) flotante independiente, gauge por bisección: `3,142857142848920` contra
`22/7 = 3,142857142857143` (coincidencia a `8·10⁻¹²`, que es el error de la
bisección, no del polígono); (c) comprobación de que el cuerpo es
estrictamente convexo, centralmente simétrico y de exactamente 16 vértices
distintos.

### 2.6 Sesenta y cuatro lados: lo mismo, y el precio se hace visible

Mismo método, `2m = 64`, 32 normales racionales unitarias con reparto angular
**deliberadamente no uniforme** (14 direcciones repartidas en `[0°, 87°)` y 18
en `[87°, 180°)`, cada una redondeada al punto racional del círculo con
tangente de semiángulo de denominador `≤ 10`):

```
u₀ = (1,0)            u₈  = (65/97, 72/97)    u₁₆ = (−11/61, 60/61)     u₂₄ = (−253/325, 204/325)
u₁ = (99/101, 20/101) u₉  = (28/53, 45/53)    u₁₇ = (−69/269, 260/269)  u₂₅ = (−989/1189, 660/1189)
u₂ = (40/41, 9/41)    u₁₀ = (8/17, 15/17)     u₁₈ = (−51/149, 140/149)  u₂₆ = (−1421/1621, 780/1621)
u₃ = (35/37, 12/37)   u₁₁ = (5/13, 12/13)     u₁₉ = (−36/85, 77/85)     u₂₇ = (−1107/1205, 476/1205)
u₄ = (77/85, 36/85)   u₁₂ = (7/25, 24/25)     u₂₀ = (−33/65, 56/65)     u₂₈ = (−1887/1985, 616/1985)
u₅ = (45/53, 28/53)   u₁₃ = (13/85, 84/85)    u₂₁ = (−3/5, 4/5)         u₂₉ = (−2773/2845, 636/2845)
u₆ = (4/5, 3/5)       u₁₄ = (0, 1)            u₂₂ = (−48/73, 55/73)     u₃₀ = (−3465/3497, 472/3497)
u₇ = (21/29, 20/29)   u₁₅ = (−21/221, 220/221)  [perilla]  u₂₃ = (−21/29,20/29)  u₃₁ = (−8840/8849, 399/8849)
```

Offsets `c_j = 1` salvo `c₁₅ = t`. Con todos a 1, `π_B = 3,143531642…`. La
faceta 15 es de carga nula, luego

```
π(t) = A + (1105/5858)·t ,

A = 1395487978012459808133012799554431251402107869824973
    ───────────────────────────────────────────────────── ,
    472262223012982640147074639107215798519140600963200

π(t) = 22/7  ⟺  t = 88764722885485632329221780496818401086619733202227
                    ───────────────────────────────────────────────────
                    89083263303063471724567681156277476504549396392000
```

```
π_B  =  22/7   EXACTO,  con 64 lados y vértices racionales.
```

Verificación flotante independiente: `3,14285714282361` contra
`3,14285714285714`.

**Y aquí está lo que hay que mirar, no el resultado.** El 16-ágono necesitó
denominadores de 7 cifras; el 64-ágono necesita **50 cifras** en el parámetro
y coordenadas del mismo orden. El objetivo `22/7` tiene una cifra de
denominador. **Todo ese peso es artefacto de la construcción, no información
sobre π.**

### 2.7 El test de la casa: ¿familia natural, o afinar hasta que salga bonito?

`07` §1.8.5 y `99-sintesis.md` §4.7 fijaron la distinción que hay que aplicar
aquí sin piedad:

| | racionalidad **barata** (`Lp`, `01` §4.4) | racionalidad **rígida** (hexágono/cuadrado, `07` §1.8) |
|---|---|---|
| parámetro | continuo (`p ∈ [1,2]`) | discreto (`n ∈ 2Z`) |
| mecanismo | teorema del valor intermedio | teorema de Niven sobre `Q(ζ_n)⁺` |
| cuántos cuerpos | un continuo, infinitos | **exactamente dos** |
| qué significa | **nada** | caracteriza el cuerpo |

**La construcción de §2.4–§2.6 cae del lado barato, y de hecho es
estrictamente más barata que `Lp`.** Tres razones, en orden creciente de
gravedad:

1. **El parámetro es continuo y de dimensión alta.** El espacio de `2m`-ágonos
   centralmente simétricos módulo `GL(2,R)` tiene dimensión `2m − 4`: para 16
   lados, **12 parámetros libres**; para 64, **60**. `Lp` tenía **uno**. El
   TVI no sólo entrega el valor: lo entrega en una subvariedad de codimensión
   1, es decir en un continuo de polígonos por cada valor objetivo.
2. **La racionalidad ni siquiera exige el TVI: la regala el Teorema A.** En
   `Lp` la racionalidad de `π_p` para un `p` dado era un hecho no trivial (los
   `π_p` son periodos de tipo Γ y no se sabe si son trascendentes, `01` §4.4);
   aquí basta escribir los vértices con números racionales. **La racionalidad
   es una propiedad del papel, no del cuerpo.**
3. **La construcción presupone la respuesta.** El objetivo `22/7` entra como
   dato en la ecuación lineal `A + Bt = 22/7` y sale como `t`. La familia no
   **produce** una aproximación a π: se le **impone** una. Es exactamente el
   modo de fallo que `00-plan-investigacion.md` prohíbe («elegir la definición
   que da el número bonito»), aquí en su versión más literal: elegir el
   **cuerpo** que da el número pedido.

La única mitad del test que la construcción aprueba es la de **honestidad de
método**: no se cambió la definición de `π_B` ni una vez, y el cuerpo
construido es una norma legítima con la constante afirmada. Lo que no aprueba
es la mitad de **contenido**: no hay nada en el 16-ágono de §2.5 que lo
distinga de sus vecinos salvo que se le pidió `22/7`.

**Casilla (II)** para todo §2 — la propiedad sobrevive al cambio de norma, la
constante cambia, y nada tiene que ver con la base numérica.

---

## 3. La sucesión, y si `4^k` está justificado

### 3.1 `4^k` no tiene ninguna justificación, y el motivo es doble

**Motivo 1 — aritmético, y es fatal en la versión regular.** `4^k ≡ 0 (mod 4)`
para todo `k ≥ 1`. Por `07` §1.8 eso mete la sucesión entera en la rama
`n·tan(π/n)`, que es **irracional para todo `n > 4`** y **nunca**
rotacionalmente robusta (`07` §3.7). La sucesión propuesta elige, de las dos
ramas disponibles, precisamente la mala en las dos propiedades a la vez.

**Motivo 2 — geométrico: no hay ninguna construcción que doble-doble.** Las
sucesiones de `n` que sí tienen justificación en este problema son las que
provienen de una **operación geométrica** sobre el polígono anterior:

| sucesión | operación | rama `mod 4` | comentario |
|---|---|---|---|
| `6·2^k` = 6, 12, 24, 48… | **bisección de Arquímedes** desde el hexágono | 6 → `sin`; **12, 24, 48… → `tan`** | la operación clásica; abandona la rama robusta en el primer paso |
| `12·2^k` = 12, 24, 48… | bisección desde el dodecágono («sabor 12») | siempre `tan` | nunca toca la rama robusta |
| `2·3^{k+1}` = 6, 18, 54, 162… | **trisección** desde el hexágono | **siempre `sin`** | única sucesión multiplicativa que **no abandona nunca** la rama racional-y-robusta |
| `4^k` = 4, 16, 64, 256… | ninguna | siempre `tan` | **arbitraria** |

`4^k` no es bisección (no dobla el número de lados: lo cuadruplica), no
corresponde a ningún paso de construcción con regla y compás, y no preserva
ninguna propiedad de la etapa anterior. **Es arbitraria**, y hay que decirlo
así.

**Observación original de esta sección, que sale de cruzar `07` §1.8 con
§3.1.** Arquímedes bisecta: `6 → 12 → 24 → …`. El hexágono está en la rama
`n ≡ 2 (mod 4)` (racional, robusta), pero `6·2 = 12 ≡ 0 (mod 4)`: **la primera
bisección ya expulsa la sucesión de la rama buena, y ningún paso posterior la
devuelve.** En cambio `2·3^{k+1} = 6, 18, 54, 162, 486, …` es `2·(impar)`,
luego `≡ 2 (mod 4)` **siempre**: es la única sucesión multiplicativa clásica
que se mantiene en la rama rotacionalmente robusta. Numéricamente:

```
$ target/release/dz-cli.exe eval "6*sin(pi/6)"     --mode fixed --scale 8 --input-base 10
~3
$ target/release/dz-cli.exe eval "18*sin(pi/18)"   --mode fixed --scale 8 --input-base 10
~3;1611X03
$ target/release/dz-cli.exe eval "54*sin(pi/54)"   --mode fixed --scale 8 --input-base 10
~3;18173X6
$ target/release/dz-cli.exe eval "162*sin(pi/162)" --mode fixed --scale 8 --input-base 10
~3;1843E99
$ target/release/dz-cli.exe eval "486*sin(pi/486)" --mode fixed --scale 8 --input-base 10
~3;184774X6
$ target/release/dz-cli.exe eval "pi"              --mode fixed --scale 8 --input-base 10
~3;18480949
```

Crece monótona hacia `~3;18480949` = π, como manda `07` §1.8.3. **Pero todos
sus términos salvo el primero son irracionales** (Niven), así que esta
sucesión mejora a `4^k` en estructura y robustez, y **no** resuelve la
pregunta del usuario. Para eso hace falta soltar la regularidad, §3.2.

### 3.2 Una sucesión explícita con `π_k ∈ Q` para todo `k` y `π_k → π`

**Construcción (rational-inscribed polygons).** Para `n = 2m`:

- `P_n := conv { ± c(s_k) : k = 0,…,m−1 }`, donde
  `c(s) = ((1−s²)/(1+s²), 2s/(1+s²))` es la parametrización racional del
  círculo unidad y `s_k ∈ Q` es la **mejor aproximación racional con
  denominador `≤ Q(n)`** de `tan(kπ/2m)`.
- Todos los vértices están en `Q²` **y sobre la circunferencia unidad**.

**Proposición 3.** `π_{P_n} ∈ Q` para todo `n` (Teorema A), y
`π_{P_n} → π` cuando `n → ∞` con `Q(n) → ∞` (los `P_n` convergen al disco en
distancia de Hausdorff; Proposición 1 y `π_D = π`).

Nótese que ambas mitades ya estaban demostradas: **la sucesión es un corolario
de §2.1 + §2.3, no un descubrimiento.** Medido (columna `reg` = valor del
polígono **regular** del mismo `n`, es decir `n·tan(π/n)` o `n·sin(π/n)` según
`07` §1.8, para contraverificar que los `P_n` racionales convergen al regular):

Medido con `Q(n) = 8n` (columna «cifras del denominador»); los dos valores
exactos que se citan en fracción se calcularon además con `Q = 10`, que da
denominadores manejables y por tanto legibles para `dz-cli`.

| `n` | `π_n` | `|π_n − π|` | `reg` de `07` §1.8 | cifras del denom. (`Q=8n`) |
|--:|---|--:|---|--:|
| 6 | `3,000640915415` | `1,4·10⁻¹` | `3,000000000000` | 5 |
| 12 | `3,215390296040` | `7,4·10⁻²` | `3,215390309173` | 25 |
| 24 | `3,159659939006` | `1,8·10⁻²` | `3,159659942098` | 56 |
| 48 | `3,146086214973` | `4,5·10⁻³` | `3,146086215131` | 133 |
| 96 | `3,142714599628` | `1,1·10⁻³` | `3,142714599645` | 278 |
| 192 | `3,141873049978` | `2,8·10⁻⁴` | `3,141873049980` | 741 |
| 4 | `4` | `8,6·10⁻¹` | `4,000000000000` | 1 |
| 16 | `3,182597502345` | `4,1·10⁻²` | `3,182597878075` | 47 |
| 64 | `3,144118385161` | `2,5·10⁻³` | `3,144118385246` | 225 |
| 256 | `3,141750369167` | `1,6·10⁻⁴` | `3,141750369169` | 881 |

Con `Q = 10`: `π₆ = 292/97` (denominador de 2 cifras) y
`π₁₆ = 68162191/21417370` (8 cifras), que son los que se pasan al binario más
abajo.

**Contraverificación fuerte, y merece subrayarse.** Los valores exactos de
esta tabla, calculados con la maquinaria de §2 (gauge por semiplanos sobre
polígonos racionales), reproducen la forma cerrada de `07` §1.8.1 —
`n·tan(π/n)` para `n ≡ 0 (mod 4)` y `n·sin(π/n)` para `n ≡ 2 (mod 4)` — a
todas las cifras que la aproximación racional permite. Por ejemplo `n = 8` da
`38651877/11664380 = 3,3137085…` contra `8 tan(π/8) = 8√2−8 = 3,3137085…`, y
`n = 6` da `292/97 = 3,00064…` que tiende a `6 sin(π/6) = 3` al afinar `Q`.
**Son dos derivaciones completamente distintas —`07` por función radial y
trigonometría; ésta por aritmética racional exacta sobre vértices— y coinciden.**
Es la contraverificación independiente que exige el protocolo, y de paso es la
primera confirmación externa de la dicotomía módulo 4 de `07`.

```
$ target/release/dz-cli.exe eval "292/97" --mode rational --input-base 10
204/81 (periodica en base 12)
$ target/release/dz-cli.exe eval "292/97" --mode fixed --scale 10 --input-base 10
3;01599340EX62287E0159934 (periodica en base 12, valor truncado)
$ target/release/dz-cli.exe eval "68162191/21417370" --mode rational --input-base 10
1X9E1867/720X38X (periodica en base 12)
$ target/release/dz-cli.exe eval "68162191/21417370" --mode fixed --scale 10 --input-base 10
3;2235829682E82X29E9217101 (periodica en base 12, valor truncado)
```

### 3.3 Qué cuesta: unicidad, isotropía, canonicidad

Las tres preguntas que el encargo pide responder con precisión:

- **¿Es único cada polígono? NO, ni remotamente.** Por la Proposición 2 y el
  Teorema A, para cada `n` y cada valor racional objetivo hay un conjunto de
  dimensión `2m − 5` de polígonos que lo realizan (una hipersuperficie dentro
  de los `2m−4` parámetros afines). El 16-ágono de §2.5 es **un punto de una
  variedad de dimensión 11**.
- **¿Es isotrópico? NO, y no puede serlo.** Isotropía en el sentido de `07`
  §3.7 (invariancia de `π_B` bajo rotación arbitraria del cuerpo contra su
  propia norma) es una propiedad de la rama `n ≡ 2 (mod 4)` **regular**. Un
  polígono de vértices racionales no es regular salvo el cuadrado y las
  imágenes afines del hexágono regular — de hecho **ninguna imagen afín de un
  `n`-ágono regular con `n ∉ {3,4,6}` admite coordenadas racionales**, porque
  sus vértices generan `Q(ζ_n)⁺` de grado `φ(n)/2 > 1`. *(Corregido en la
  revisión del 2026-09-13: la versión anterior decía «ningún `n`-ágono
  regular», lo que sugería que el triángulo equilátero y el hexágono regular
  euclidianos admiten coordenadas racionales; no las admiten — el `√3` lo
  impide —, y euclidianamente regular sólo el cuadrado las tiene.)* La
  construcción compra racionalidad vendiendo simetría.
- **¿Es canónica la sucesión? NO: hay un continuo de sucesiones.** Cambiar el
  denominador máximo `Q(n)`, el reparto angular, o la elección de las
  aproximaciones `s_k` da otra sucesión igual de válida. Ni siquiera el
  **valor** `π_n` es canónico: sólo su límite lo es.
- **¿Produce la respuesta o la presupone?** La sucesión de §3.2 **la produce**
  (nadie eligió los `π_n`: salen de aproximar el círculo). Las construcciones
  de §2.5–§2.6 **la presuponen** (el `22/7` entra por la ecuación). Hay que
  distinguirlas con claridad, y la buena noticia para la hipótesis del usuario
  es que la que produce también existe. La mala es §3.4.

### 3.4 Calidad de la aproximación: mucho peor que la fracción continua

Convergentes de π (que son, por teoría de fracciones continuas, **óptimos**:
todo racional `p/q` con `|p/q − π| < 1/(2q²)` es convergente):

| convergente | valor | error | cifras del denominador |
|---|---|--:|--:|
| `3/1` | 3,000000000000 | `1,4·10⁻¹` | 1 |
| `22/7` | 3,142857142857 | `1,3·10⁻³` | 1 |
| `333/106` | 3,141509433962 | `8,3·10⁻⁵` | 3 |
| `355/113` | 3,141592920354 | `2,7·10⁻⁷` | 3 |
| `103993/33102` | 3,141592653012 | `5,8·10⁻¹⁰` | 5 |

Comparación directa **a igual error**:

| error | racional poligonal (§3.2) | mejor racional (convergente) | penalización |
|---|---|---|---|
| `≈ 1,4·10⁻¹` | `n=6`, den. de **5 cifras** (2 con `Q=10`) | `3/1`, den. de **1 cifra** | ×5 |
| `≈ 1,1·10⁻³` | `n=96`, den. de **278 cifras** | `22/7`, den. de **1 cifra** | **×278** |
| `≈ 1,6·10⁻⁴` | `n=256`, den. de **881 cifras** | `333/106`, den. de **3 cifras** | **×294** |

El mecanismo: el error del polígono decae como `π³/(3n²)` (el término
de Taylor de `n·tan(π/n)`), es decir **cuadrático en `n`**, mientras que el
denominador del racional crece muy deprisa en `n` **en esta construcción**,
porque cada vértice aporta su propia hipotenusa pitagórica. *(Revisión del
2026-09-13: la versión anterior lo llamaba «razón estructural». No lo es.
Tomando todos los vértices con un denominador común `c = 5·13·17·…`, cada primo
`≡ 1 (mod 4)` añadido duplica los puntos racionales de denominador `c` sobre la
circunferencia, y el denominador necesario para `n` vértices crecería mucho más
despacio. La penalización frente a los convergentes probablemente sobrevive,
porque éstos son óptimos, pero su tamaño para la mejor construcción no está
medido.)* Un convergente `p/q` alcanza error
`≈ 1/q²`, y la construcción poligonal alcanza el mismo error con `q` de
cientos de cifras en vez de una o tres.

**Conclusión cuantificada:** como método de aproximación a π, la construcción
de normas poligonales racionales es **dos a tres órdenes de magnitud peor en
cifras** que la fracción continua, y ni siquiera compite con Arquímedes
(cuyas cotas `223/71 < π < 22/7` tienen denominadores de dos cifras). No es un
método de aproximación: es una reformulación geométrica de la densidad de `Q`.

---

## 4. Base 12

El protocolo es explícito: **la base no puede crear racionalidad**. Lo que
sigue examina sólo lo que legítimamente puede examinarse.

### 4.1 ¿Ayuda `12 = 2²·3` a subdividir?

Sí, y es el único punto donde base 12 aporta algo real — de casilla **(III)**,
puramente representacional. Las sucesiones con justificación geométrica (§3.1)
tienen `n` de la forma `2^a·3^b`, y por tanto `1/n` **termina en base 12**:

```
$ target/release/dz-cli.exe eval "1/6"   --mode rational --input-base 10
1/6
$ target/release/dz-cli.exe eval "1/12"  --mode rational --input-base 10
1/10
$ target/release/dz-cli.exe eval "1/18"  --mode rational --input-base 10
1/16
$ target/release/dz-cli.exe eval "1/24"  --mode rational --input-base 10
1/20
$ target/release/dz-cli.exe eval "1/48"  --mode rational --input-base 10
1/40
$ target/release/dz-cli.exe eval "1/54"  --mode rational --input-base 10
1/46
$ target/release/dz-cli.exe eval "1/96"  --mode rational --input-base 10
1/80
$ target/release/dz-cli.exe eval "1/162" --mode rational --input-base 10
1/116
$ target/release/dz-cli.exe eval "1/486" --mode rational --input-base 10
1/346
$ target/release/dz-cli.exe eval "1/3"   --mode fixed --scale 8 --input-base 10
0;4
$ target/release/dz-cli.exe eval "1/6"   --mode fixed --scale 8 --input-base 10
0;2
$ target/release/dz-cli.exe eval "1/12"  --mode fixed --scale 8 --input-base 10
0;1
```

(ninguno lleva el aviso «periodica»: **todos terminan**.) En base 10, en
cambio, `1/6 = 0,1666…`, `1/12 = 0,08333…`, `1/18`, `1/24`, `1/48`, `1/54`,
`1/96`, `1/162` **no terminan ninguno**, porque todos llevan el factor 3.

**Pero el detalle que desactiva el entusiasmo**: la sucesión propuesta por el
usuario, `4^k`, es precisamente **la única de la tabla de §3.1 en la que base
12 no aporta nada**, porque `4^k = 2^{2k}` sólo tiene doses y por tanto
termina **igual de bien en base 10**:

```
$ target/release/dz-cli.exe eval "1/4"   --mode rational --input-base 10
1/4
$ target/release/dz-cli.exe eval "1/16"  --mode rational --input-base 10
1/14
$ target/release/dz-cli.exe eval "1/64"  --mode rational --input-base 10
1/54
$ target/release/dz-cli.exe eval "1/256" --mode rational --input-base 10
1/194
```

`1/4 = 0,25`, `1/16 = 0,0625`, `1/64 = 0,015625`, `1/256 = 0,00390625`: todos
terminan en base 10. **La ventaja dozenal de la subdivisión existe
exactamente en las sucesiones que el usuario no propuso** (bisección y
trisección desde el hexágono) **y es nula en la que sí propuso.** Casilla
**(III)**, y con signo desfavorable a `4^k`.

### 4.2 ¿Terminan las constantes que aparecen? (`N_periodicos`)

Ninguna de las constantes centrales del documento termina en base 12:

```
$ target/release/dz-cli.exe eval "22/7"       --mode rational --input-base 10
1X/7 (periodica en base 12)
$ target/release/dz-cli.exe eval "8/7"        --mode rational --input-base 10
8/7 (periodica en base 12)
$ target/release/dz-cli.exe eval "157/50"     --mode rational --input-base 10
111/42 (periodica en base 12)
$ target/release/dz-cli.exe eval "13249/5220" --mode rational --input-base 10
7801/3030 (periodica en base 12)
$ target/release/dz-cli.exe eval "37/60"      --mode rational --input-base 10
31/50 (periodica en base 12)
$ target/release/dz-cli.exe eval "22097/22533" --mode rational --input-base 10
10955/11059 (periodica en base 12)
$ target/release/dz-cli.exe eval "355/113"    --mode rational --input-base 10
257/95 (periodica en base 12)
$ target/release/dz-cli.exe eval "333/106"    --mode rational --input-base 10
239/8X (periodica en base 12)
```

`22/7` falla por el 7, `157/50` por el 5, `37/60` por el 5, `355/113` por el
113. El único par de objetivos «bonitos» que **sí** terminaría es el de
denominador `2^a 3^b`:

```
$ target/release/dz-cli.exe eval "19/6" --mode fixed --scale 8 --input-base 10
3;2
$ target/release/dz-cli.exe eval "25/8" --mode fixed --scale 8 --input-base 10
3;16
```

es decir `3+1/6 = 3;2` y `3+1/8 = 3;16`, dos cifras dozenales exactas. **Pero
son peores aproximaciones a π que `22/7`** (`|19/6 − π| = 2,4·10⁻²`,
`|25/8 − π| = 1,6·10⁻²`, contra `1,3·10⁻³`), así que preferirlos sería elegir
el número bonito en la base bonita a costa de la exactitud — exactamente lo
que el protocolo prohíbe. Se registra y no se usa.

### 4.3 Resultado negativo nuevo: la construcción es **estructuralmente hostil**
a base 12

Éste no estaba previsto y es el hallazgo dozenal real del documento, con signo
negativo.

> **Proposición 4.** Sea `P` un polígono cuyos vértices son puntos racionales
> de la circunferencia unidad (la construcción de §3.2). Entonces los
> denominadores de sus coordenadas son de la forma `p² + q²` con
> `gcd(p,q) = 1`. Ningún número de esa forma es divisible por **3**. Por
> tanto **ninguna** de esas coordenadas puede tener expansión dozenal finita
> salvo las triviales.

*Demostración.* Si `3 | p²+q²`, entonces `p² ≡ −q² (mod 3)`; como los cuadrados
módulo 3 son `{0,1}`, la única solución es `p ≡ q ≡ 0 (mod 3)`, que contradice
`gcd(p,q)=1`. ∎

Los primos que dividen a `p²+q²` primitivo son `2` y los `≡ 1 (mod 4)`; y como
aquí `p²+q²` es la hipotenusa de una terna pitagórica primitiva (`p`, `q` de
paridad opuesta), es **impar**, así que el `2` tampoco aparece *(precisión
añadida en la revisión del 2026-09-13)*. Quedan
`5, 13, 17, 29, 37, 41, 53, 61, 73, 89, 97, 101, …` — y la lista **empieza por
5**, que es precisamente el primo que `99-sintesis.md` §1 identifica como el
que base 12 no regala y base 10 sí.

```
$ target/release/dz-cli.exe eval "3/5"   --mode rational --input-base 10
3/5 (periodica en base 12)
$ target/release/dz-cli.exe eval "4/5"   --mode rational --input-base 10
4/5 (periodica en base 12)
$ target/release/dz-cli.exe eval "5/13"  --mode rational --input-base 10
5/11 (periodica en base 12)
$ target/release/dz-cli.exe eval "12/13" --mode rational --input-base 10
10/11 (periodica en base 12)
$ target/release/dz-cli.exe eval "8/17"  --mode rational --input-base 10
8/15 (periodica en base 12)
```

**Lectura honesta:** `3/5` y `4/5` **terminan en base 10** (`0,6` y `0,8`) y
**no terminan en base 12**. En la aritmética de los puntos racionales del
círculo —que es la aritmética natural de esta construcción— **base 10 es
estrictamente mejor que base 12**, porque el 5 aparece en el primer término y
reaparece sin parar (`25, 65, 85, 325, …`), y base 12 no lo tiene. Casilla
**(III)**, aportación dozenal **negativa**. Es el mismo mecanismo que
`99-sintesis.md` §1 identificó con las ternas pitagóricas, reaparecido aquí de
forma inevitable: **esta construcción ES ternas pitagóricas.**

### 4.4 Coste medido en puertas

Con la advertencia estándar de `01` §6.1 y `07` §4: estas cifras miden el
algoritmo de evaluación, **no** la complejidad aritmética del número, y
ninguna es evidencia sobre H₁.

| expresión | valor dozenal | total | depth | recon |
|---|---|--:|--:|--:|
| `22/7` | `3;186X35186X35…` | 635.844 | 1.342 | 0 |
| `355/113` | `3;18480X24051…` | 950.178 | 1.423 | 0 |
| `68162191/21417370` (π₁₆ de §3.2) | `3;2235829682E…` | 4.526.850 | 5.515 | 0 |
| `16*tan(pi/16)` (π₁₆ regular) | `~3;2236424` | 67.157.836 | 19.347 | 0 |

```
$ target/release/dz-cli.exe eval "22/7"              --mode fixed --scale 8 --gates --input-base 10
gates: cycle=160116 min=189962 max=146554 lit=107220 thr=31992 total=635844 depth=1342 recon=0
$ target/release/dz-cli.exe eval "355/113"           --mode fixed --scale 8 --gates --input-base 10
gates: cycle=222288 min=298100 max=225946 lit=160596 thr=43248 total=950178 depth=1423 recon=0
$ target/release/dz-cli.exe eval "68162191/21417370" --mode fixed --scale 8 --gates --input-base 10
gates: cycle=959640 min=1501196 max=1116346 lit=767952 thr=181716 total=4526850 depth=5515 recon=0
$ target/release/dz-cli.exe eval "16*tan(pi/16)"     --mode fixed --scale 8 --gates --input-base 10
gates: cycle=13937748 min=22586570 max=16702586 lit=11376516 thr=2554416 total=67157836 depth=19347 recon=0
```

Lo único que estas cifras dicen, y es la misma lección de `07` §4: **la versión
racional de la constante (4,5 millones de puertas) es ~15× más barata de
evaluar que la versión regular irracional (67 millones), porque ésta pasa por
`tan` y por tanto por `sqrt` y división restauradora.** Es un hecho sobre el
algoritmo, no sobre la geometría; el propio `22/7`, que es una aproximación
peor que ambas, cuesta 635.844. **Ningún criterio de «simplicidad» basado en
puertas ordena estas constantes por su calidad matemática**, y quien lo usara
concluiría que `22/7` es la mejor, que es falso.

---

## 5. Lo que no funcionó

Sección obligatoria del protocolo, y en este documento es larga porque la
mayor parte del trabajo fue de descarte.

### 5.1 El primer plan —buscar un `n`-ágono regular de 16 ó 64 lados con razón
racional— estaba muerto antes de empezar

Es literalmente lo que `07` §1.8.4 demuestra imposible. Se comprobó que el
encargo original del usuario (`n_k = 4^k`) cae íntegro en la rama mala, y se
abandonó esa vía sin gastar cálculo en ella. **El primer resultado del
documento es un «no» heredado, no un hallazgo propio.**

### 5.2 El teorema del valor intermedio no da construcciones: hubo que hacer la
familia afín a mano

Un TVI abstracto garantiza que existe un 16-ágono con `π_B = 22/7`, pero **no
lo exhibe**, y el encargo pedía coordenadas explícitas. El primer intento
—tomar un 14-ágono racional y cortarlo con una banda `|⟨u,x⟩| ≤ t`— produjo

```
π(t) = A + B·t + C/t ,   con A = 4388850087647/1640179284800, B = 3/448, C = 18/41,
```

es decir una **hipérbola**, no una recta. Resolver `π(t) = 22/7` da entonces una
**ecuación cuadrática** cuyo discriminante resultó ser
`79302731947685870714487/384312583755291358720000`, que **no es cuadrado de un
racional**. El `t` solución es un irracional cuadrático y el polígono deja de
tener vértices racionales. Tres tandas de búsqueda aleatoria sobre bases y
direcciones de banda no encontraron ningún discriminante cuadrado — como era
de esperar, porque los cuadrados tienen densidad nula.

**Lo que desbloqueó el problema** fue entender **por qué** aparece el término
`C/t`: la restricción-perilla contribuye `⟨u,w⟩/t` al máximo de (★★), y sólo
gana sobre alguna arista `w` si la dirección de `w` **apunta al cono de esa
faceta**. De ahí la regla de §2.5, paso 2: elegir una faceta **de carga nula**.
En un polígono casi regular no existe ninguna (la rotación de un cuarto de
vuelta reparte exactamente una arista por faceta), así que hubo que **romper la
uniformidad angular a propósito** para que el principio del palomar dejara
facetas vacías. Ésa es la única idea no rutinaria del documento, y llegó
después de tres callejones sin salida.

### 5.3 Intentos de preservar `π_B` al añadir vértices: todos fallidos

Se buscó una operación «refinar sin cambiar la constante», que habría dado los
16 y 64 lados gratis a partir del hexágono exacto de §2.4. Tres intentos:

- **Subdividir una arista con vértices sobre ella.** No cambia el cuerpo, así
  que los «nuevos vértices» no están en posición estrictamente convexa: es un
  16-ágono **falso**. Descartado por deshonesto.
- **Empujar los puntos medios hacia afuera una cantidad `ε`.** Sí da un
  polígono genuino, y `π_B → 22/7` cuando `ε → 0`, pero **nunca vale
  exactamente** `22/7`: la constante se mueve con `ε`.
- **Usar la aditividad de la norma dentro de un sector lineal**
  (`‖e₁‖+‖e₂‖ = ‖e₁+e₂‖` si ambas aristas caen en el cono de la misma
  faceta, que es el fenómeno de la escalera taxicab). Falla por el punto fino
  de §2.1: al deformar el cuerpo **cambia la propia norma**, y con ella qué
  faceta es la que mide. La aditividad se pierde justo donde haría falta.

### 5.4 No se consiguió un 64-ágono con constantes pequeñas

El 64-ágono de §2.6 tiene un parámetro con 50 cifras en numerador y
denominador. Se barrieron 5 valores del denominador máximo de las normales × 7
repartos angulares × 55 ángulos de corte (unas 2.000 configuraciones, de las
que 12 dieron acierto exacto) y el mínimo alcanzado fue ése. **No se sabe si
existe un 64-ágono con `π_B = 22/7` y coordenadas de tamaño moderado**, y el
método de §2.5 no da ninguna forma de buscarlo dirigidamente.

### 5.5 El sabor dozenal resultó ser negativo, no neutro

Se esperaba (predicción P5 del plan) que el efecto dozenal fuese nulo fuera de
`N_periodicos`. Resultó **peor**: §4.3 muestra que la construcción natural
(puntos racionales del círculo) vive sobre denominadores en los que **base 10
gana y base 12 pierde**, por un teorema elemental sobre `p²+q²` módulo 3. No
es un empate, es una derrota medible, y hay que reportarla como tal.

### 5.6 Entorno: no había Python en la máquina

Trivial pero conviene dejarlo escrito porque afecta a la reproducibilidad: los
experimentos numéricos se escribieron en **Node.js con `BigInt`** (aritmética
racional exacta implementada a mano), no en Python/sympy, porque `py`,
`python` y `python3` no están instalados en esta máquina. Los guiones viven en
el directorio temporal de la sesión y **no** se añadieron al repositorio.

---

## 6. Limitaciones conocidas

- **La Proposición 2 usa la conexidad de `P(2m)` sin demostrarla en detalle.**
  Se afirma que dos configuraciones estrictamente convexas centralmente
  simétricas se unen por un camino continuo dentro del mismo conjunto; es
  cierto y estándar (radialmente hacia la configuración regular), pero aquí se
  usa como hecho, no se escribe la homotopía.
- **El teorema de Gołąb se cita, no se demuestra**, igual que `07` §1.4. Todo
  §1.2 y el «`3 < π < 4` estricto» de §1.4 descansan sobre él.
- **El Teorema A no tiene recíproco, y no se investigó si lo tiene.** Hay
  polígonos con vértices irracionales y `π_B` racional (por ejemplo cualquier
  imagen afín irracional de los de §2), así que «racional» no caracteriza
  «vértices racionales». Qué caracteriza exactamente a los cuerpos con `π_B`
  racional queda abierto.
- **La cota `error ≈ π³/(3n²)` de §3.4 se toma del desarrollo de `n·tan(π/n)`**
  y se contrasta con la tabla, pero no se demuestra para los polígonos
  racionales `P_n` (que no son regulares); lo que sí se verifica es que sus
  valores convergen a los del regular.
- **El crecimiento «exponencial» del denominador en §3.4 es una lectura de la
  tabla, no un teorema.** Las cifras medidas (5, 25, 56, 133, 278, 741 para
  `n = 6,12,24,48,96,192`, con `Q(n)=8n`) dan cocientes `cifras/n` de
  `0,8 / 2,1 / 2,3 / 2,8 / 2,9 / 3,9`: un denominador del orden de `10^{3n}`
  con la tasa creciendo despacio. No se demostró esa tasa, y además depende de
  la elección de `Q(n)`.
- **La búsqueda de facetas de carga nula (§2.5, paso 2) es heurística.** Se
  comprueba por cómputo directo sobre cada candidato, y la afinidad de `π(t)`
  se **verifica** en cinco valores de `t`, pero no se demuestra un criterio
  general de cuándo existe una faceta de carga nula.
- **No se exploró el análogo en 3D**, ni la pregunta de si una familia de
  poliedros con constante de superficie racional converge a la esfera. `07`
  §2 da razones para esperar un fracaso, pero no es lo mismo que comprobarlo.
- **No se examinó la robustez rotacional de los polígonos construidos.** Por
  `07` §3.7 sólo la clase `n ≡ 2 (mod 4)` **regular** es robusta; los cuerpos
  de §2.5–§2.6 no son regulares, así que casi seguro no lo son, pero **no se
  midió**. Si lo fueran sería un resultado, y no se buscó.
- **Todas las cifras dozenales de escala finita llevan el aviso de truncado o
  de periodicidad del propio binario**; las afirmaciones de racionalidad se
  apoyan siempre en la aritmética exacta (fracción), nunca en el desarrollo
  dozenal.
- **Ninguna parte de este documento se ha sometido a revisión externa.**

---

## 7. Veredicto

### 7.1 Verdadero / falso / abierto, por partes

| afirmación | veredicto | dónde |
|---|---|---|
| «Se puede definir un `π_‖·‖` para normas poligonales» | **VERDADERA**, y es estándar | §1.1 |
| «Vale 4 para el taxicab (`n=4`)» | **VERDADERA** | `07` §1.6, §1.2 aquí |
| «Existen polígonos con `n=16, 64` y `π_‖·‖` racional» | **VERDADERA**, construidos | §2.5, §2.6 |
| «Existe una sucesión de normas con `π_k ∈ Q` y `π_k → π`» | **VERDADERA**, construida | §3.2 |
| «Con polígonos **regulares** de `n_k = 4^k` lados» | **FALSA, demostrada** | `07` §1.8; §1.3 |
| «`n_k = 4^k` tiene alguna justificación» | **FALSA**: es arbitraria | §3.1 |
| «Esa familia es una construcción **natural**» | **FALSA**: hay un continuo de ellas, y la que da el número pedido lo recibe como dato | §2.7, §3.3 |
| «La construcción dice algo sobre π» | **FALSA**: es 278× peor que `22/7` a igual error | §3.4 |
| «Base 12 aporta algo» | **FALSA, y peor que nula**: aporta negativo | §4.3 |
| «Base 12 ayuda a subdividir» | **VERDADERA pero irrelevante**: ayuda en `6·2^k` y `2·3^k`, **no** en `4^k` | §4.1 |
| Qué caracteriza exactamente a los cuerpos con `π_B ∈ Q` | **ABIERTA** | §6 |
| Si existe un 64-ágono racional con `π_B = 22/7` y coordenadas pequeñas | **ABIERTA** | §5.4 |

### 7.2 La lectura de fondo

**La hipótesis del usuario es correcta en su conclusión y equivocada en todo
lo que la haría interesante.** Sí existe una familia de normas poligonales con
`π_k` racional convergiendo a π. Pero:

- **no** con polígonos regulares, donde está demostrado que sólo hay dos
  constantes racionales en todo el universo de los `n`-ágonos (`07` §1.8);
- **no** con `n_k = 4^k`, que es la peor elección posible: cae entera en la
  rama irracional y no robusta, y es además la única sucesión candidata en la
  que base 12 no aporta nada;
- y cuando se sueltan las dos condiciones que la hacían difícil —regularidad y
  `4^k`— **la racionalidad deja de ser un hallazgo y pasa a ser una
  tautología**: el Teorema A la regala a cualquier polígono escrito con
  números racionales.

**Este documento extiende el patrón de `07` §7 en vez de romperlo, pero por el
otro lado.** `07` cerró la familia regular con un «no existe» exhaustivo:
racionalidad **rígida**, exactamente dos cuerpos, caracterizados por Niven.
Aquí se abre la familia no regular y aparece el fenómeno opuesto y simétrico:
racionalidad **gratuita**, un continuo de cuerpos, caracterizada por nada.
**Entre los dos regímenes no hay ningún punto intermedio interesante**, y ésa
es la forma correcta de enunciar el resultado conjunto:

```
Regular      → la racionalidad es rígida, rarísima (n = 4, 6) y significa algo.
No regular   → la racionalidad es densa, gratuita y no significa nada.
No hay un tercer régimen.
```

La constante `π` no se toca en ninguno de los dos. Sigue siendo trascendente,
sigue siendo el límite de ambas ramas de `07` §1.8.3, y ninguna sucesión de
racionales —de ésta ni de ninguna otra construcción— lo alcanza. Lo único que
este documento añade sobre π es una medida de **cuán mala** es esta vía para
aproximarlo: cientos de cifras de denominador donde la fracción continua gasta
una.

---

## 8. Bitácora de comandos

Todos contra `target/release/dz-cli.exe`. Se listan los que producen cifra
publicada; los repetidos en el cuerpo no se duplican aquí.

```
# §1.3  la familia del usuario, versión regular
eval "16*tan(pi/16)" --mode fixed --scale 8 --input-base 10      -> ~3;2236424
eval "64*tan(pi/64)" --mode fixed --scale 8 --input-base 10      -> ~3;18905314
eval "pi"            --mode fixed --scale 8 --input-base 10      -> ~3;18480949

# §2.3  extremos de Golab, vértices enteros
eval "4" --mode rational                                          -> 4
eval "3" --mode rational                                          -> 3

# §2.4  hexágono exacto con pi_B = 22/7
eval "22/7" --mode rational  --input-base 10                      -> 1X/7 (periodica en base 12)
eval "22/7" --mode fixed --scale 8 --input-base 10                -> 3;186X35186X35186X35186X35 (periodica, truncado)
eval "8/7"  --mode rational  --input-base 10                      -> 8/7 (periodica en base 12)

# §3.1  la sucesión de trisección desde el hexágono (rama n = 2 mod 4)
eval "6*sin(pi/6)"     --mode fixed --scale 8 --input-base 10     -> ~3
eval "18*sin(pi/18)"   --mode fixed --scale 8 --input-base 10     -> ~3;1611X03
eval "54*sin(pi/54)"   --mode fixed --scale 8 --input-base 10     -> ~3;18173X6
eval "162*sin(pi/162)" --mode fixed --scale 8 --input-base 10     -> ~3;1843E99
eval "486*sin(pi/486)" --mode fixed --scale 8 --input-base 10     -> ~3;184774X6

# §3.2  valores exactos de la sucesión racional
eval "292/97"            --mode rational --input-base 10          -> 204/81 (periodica en base 12)
eval "292/97"            --mode fixed --scale 10 --input-base 10  -> 3;01599340EX62287E0159934 (periodica, truncado)
eval "68162191/21417370" --mode rational --input-base 10          -> 1X9E1867/720X38X (periodica en base 12)
eval "68162191/21417370" --mode fixed --scale 10 --input-base 10  -> 3;2235829682E82X29E9217101 (periodica, truncado)
eval "38651877/11664380" --mode rational --input-base 10          -> 10E3EE99/3XX6278 (periodica en base 12)

# §3.4  convergentes de la fracción continua
eval "355/113" --mode rational --input-base 10                    -> 257/95 (periodica en base 12)
eval "355/113" --mode fixed --scale 8 --input-base 10             -> 3;18480X240512026701336077 (periodica, truncado)
eval "333/106" --mode rational --input-base 10                    -> 239/8X (periodica en base 12)

# §4.1  subdivisión: 2^a 3^b termina en base 12
eval "1/3"   --mode fixed --scale 8 --input-base 10               -> 0;4
eval "1/6"   --mode fixed --scale 8 --input-base 10               -> 0;2
eval "1/12"  --mode fixed --scale 8 --input-base 10               -> 0;1
eval "1/6"   --mode rational --input-base 10                      -> 1/6
eval "1/12"  --mode rational --input-base 10                      -> 1/10
eval "1/18"  --mode rational --input-base 10                      -> 1/16
eval "1/24"  --mode rational --input-base 10                      -> 1/20
eval "1/48"  --mode rational --input-base 10                      -> 1/40
eval "1/54"  --mode rational --input-base 10                      -> 1/46
eval "1/96"  --mode rational --input-base 10                      -> 1/80
eval "1/162" --mode rational --input-base 10                      -> 1/116
eval "1/486" --mode rational --input-base 10                      -> 1/346
eval "1/4"   --mode rational --input-base 10                      -> 1/4
eval "1/16"  --mode rational --input-base 10                      -> 1/14
eval "1/64"  --mode rational --input-base 10                      -> 1/54
eval "1/256" --mode rational --input-base 10                      -> 1/194

# §4.2  N_periodicos de las constantes del documento
eval "157/50"       --mode rational --input-base 10               -> 111/42 (periodica en base 12)
eval "13249/5220"   --mode rational --input-base 10               -> 7801/3030 (periodica en base 12)
eval "37/60"        --mode rational --input-base 10               -> 31/50 (periodica en base 12)
eval "22097/22533"  --mode rational --input-base 10               -> 10955/11059 (periodica en base 12)
eval "19/6" --mode fixed --scale 8 --input-base 10                -> 3;2
eval "25/8" --mode fixed --scale 8 --input-base 10                -> 3;16

# §4.3  denominadores pitagóricos: base 10 gana, base 12 pierde
eval "3/5"   --mode rational --input-base 10                      -> 3/5 (periodica en base 12)
eval "4/5"   --mode rational --input-base 10                      -> 4/5 (periodica en base 12)
eval "5/13"  --mode rational --input-base 10                      -> 5/11 (periodica en base 12)
eval "12/13" --mode rational --input-base 10                      -> 10/11 (periodica en base 12)
eval "8/17"  --mode rational --input-base 10                      -> 8/15 (periodica en base 12)

# §4.4  coste en puertas
eval "22/7"              --mode fixed --scale 8 --gates --input-base 10
  -> gates: cycle=160116 min=189962 max=146554 lit=107220 thr=31992 total=635844 depth=1342 recon=0
eval "355/113"           --mode fixed --scale 8 --gates --input-base 10
  -> gates: cycle=222288 min=298100 max=225946 lit=160596 thr=43248 total=950178 depth=1423 recon=0
eval "68162191/21417370" --mode fixed --scale 8 --gates --input-base 10
  -> gates: cycle=959640 min=1501196 max=1116346 lit=767952 thr=181716 total=4526850 depth=5515 recon=0
eval "16*tan(pi/16)"     --mode fixed --scale 8 --gates --input-base 10
  -> gates: cycle=13937748 min=22586570 max=16702586 lit=11376516 thr=2554416 total=67157836 depth=19347 recon=0
```

**Nota de reproducibilidad.** Los cómputos exactos de polígonos (aritmética
racional de precisión arbitraria, gauge por representación en semiplanos) y la
verificación flotante independiente (gauge por bisección) se hicieron con
guiones de un solo uso en Node.js/`BigInt` en el directorio temporal de la
sesión; **no** forman parte del repositorio, por la misma norma con que `01`
§0.3 trató sus guiones de cuadratura. Todo dato dozenal publicado, sin
excepción, sale de los comandos de arriba.
