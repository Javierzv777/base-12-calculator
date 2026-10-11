# Hallazgos — Línea A: constantes geométricas

Preguntas 1, 2 y 7 de `planteamientos.md`. Predicciones registradas que esta
línea debe juzgar: **P1** (no hay un «π taxicab» único) y **P2** (L₁ y L∞ son
las únicas `Lp` con razón perímetro/diámetro entera).

Fecha: 2026-09-12. Cifras dozenales: todas de
`target/release/dz-cli.exe`, con el comando al lado. Cifras decimales de
análisis numérico: de un script de cuadratura en Node, declarado en §0.3.

---

## 0. Notas de método (leer antes de las cifras)

### 0.1 La regla que gobierna todo el informe

Nunca se afirma que π = 4. **Cada constante que aparece aquí va acompañada de
dos etiquetas obligatorias: qué propiedad conserva y en qué métrica se mide el
numerador y el denominador.** El resultado central de esta línea es
precisamente que sin esas dos etiquetas la frase «la constante de L₁» no tiene
referente.

### 0.2 Trampa de la calculadora, encontrada en caliente

`dz-cli` lee **todo literal como dozenal** salvo que se le pase
`--input-base 10`. Durante esta investigación se escribió `eval "1/24"`
esperando un veinticuatroavo y la calculadora contestó
`0;05186X35186X35…  (periodica en base 12)`, que es correcto —`24` en dozenal
es veintiocho, y 1/28 no termina en base 12 porque 7 | 28. El valor que se
quería es:

```
$ target/release/dz-cli.exe eval "1/24" --input-base 10 --mode fixed --scale 10
0;06
```

Un veinticuatroavo **sí** termina (24 = 2³·3). Cualquier tabla de recíprocos
construida sin esta precaución mezcla dos preguntas distintas y saca
conclusiones invertidas sobre la base 12. Todas las cifras de este documento
cuyo literal tiene un valor decimal pretendido llevan `--input-base 10`; las
que son dozenales por naturaleza (`8`, `2`, `3;6`) no lo llevan, y se indica.

### 0.3 De dónde salen las cifras decimales

Las integrales de la familia `Lp` (§4) no son calculables con `dz-cli`, que no
hace cuadratura. Se calcularon con un script propio en Node
(cuadratura tanh-sinh, para absorber la singularidad algebraica de los
extremos, y función Γ por Lanczos). Se declara explícitamente porque **no son
cifras del instrumento del repositorio**: son análisis numérico auxiliar de
doble precisión, con el error que eso implica (≈1e-12 en la región `p ∈ [1,4]`,
y **divergente** para `p ≳ 20`, donde hubo que usar la dualidad `π_p = π_{p'}`
en vez del valor directo). Todo numeral dozenal que aparece a partir de esos
decimales se obtuvo pasándolos por `--input-base 10`, nunca convirtiéndolos a
mano.

### 0.4 Límite del contador de puertas para esta línea, declarado por adelantado

`dz-cli` **cobra 0 puertas por `pi`** (las constantes se sintetizan en un
backend desechable y se memoizan, a propósito). Por tanto el instrumento
objetivo del plan está **estructuralmente sesgado a favor de L₂** en esta
línea concreta: la fórmula euclidiana recibe su constante trascendente gratis.
En §6 se mide igualmente, y se reporta que en dos de los tres casos cabecera la
formulación L₁ sale **más cara**. Eso no es un fallo del instrumento: es la
respuesta honesta de que el coste vive en el *algoritmo* (raíz cuadrada,
división restauradora) y no en la forma simbólica — exactamente la lectura de
`02-referencia-medida.md` §3, donde √(3²+4²) cuesta 43.631.965 puertas frente a
39.469 de 3+4 **aunque su resultado sea el entero exacto 5**.

---

## 1. Pregunta 1 — circunferencia y qué fórmulas con π admiten análogo racional

### 1.1 Los dos enunciados

**L₂.** Para el disco de radio r, `C₂ = 2πr`, y la razón
perímetro/diámetro vale π, con perímetro y diámetro medidos ambos con la
longitud de arco euclidiana.

**L₁.** La bola `‖(x,y)‖₁ = |x|+|y| ≤ r` es el rombo de vértices `(±r,0)`,
`(0,±r)`. Medida **con longitud de arco L₁** —que es la única elección
coherente— cada lado vale `|Δx|+|Δy| = r+r = 2r`, el perímetro es `C₁ = 8r`, el
diámetro L₁ es `2r`, y la razón es **4**.

La coherencia no es un detalle: el mismo rombo tiene perímetro **euclidiano**
`4√2 r`, y entonces la razón sería `2√2`, irracional.

```
$ target/release/dz-cli.exe eval "4*sqrt(2)" --mode fixed --scale 10
~5;7X70643428
$ target/release/dz-cli.exe eval "2*sqrt(2)" --mode fixed --scale 10
~2;9E36321814
```

Es decir: el «4» no se obtiene cambiando de geometría, se obtiene cambiando de
geometría **y** midiendo la longitud con la misma norma con que se definió la
bola. Si se mezclan las dos métricas reaparece √2 inmediatamente.

### 1.2 Lema elemental que genera todos los análogos racionales

Es útil tener el mecanismo, no solo el caso del rombo.

> **Lema.** Para todo cuerpo convexo `K ⊂ R²` con caja envolvente de lados
> `w_x` y `w_y`, la longitud L₁ de `∂K` es exactamente `2(w_x + w_y)`.

*Demostración.* La longitud L₁ de una curva es `∫(|dx| + |dy|)`, que sobre un
lazo es la variación total de la coordenada `x` más la de `y`. Para un convexo,
la variación total de `x` al recorrer el borde es `2w_x` (sube una vez, baja una
vez), e igual para `y`. ∎

De aquí salen tres consecuencias, y las tres son más informativas que el «4».

**(a) La circunferencia L₁ es ciega a todo salvo la caja envolvente.** El rombo
de radio r y el cuadrado `[-r,r]²` tienen el mismo perímetro L₁, `8r`, igual
que cualquier convexo con esa caja. La fórmula se simplifica, sí, pero porque
**descarta información**: `C₁` deja de determinar la forma.

**(b) El rango exacto de la razón perímetro/diámetro en el plano L₁.**
Con `D` el diámetro L₁ de `K` se tiene `max(w_x,w_y) ≤ D ≤ w_x + w_y`, luego

```
razón = 2(w_x+w_y)/D  ∈  [2, 4]
```

El **4 es el máximo** sobre convexos, y lo alcanza el rombo (donde `D = w_x =
w_y = 2r`). El **2 es el mínimo**, y lo alcanza cualquier cuerpo cuyo diámetro
sea la diagonal de su caja: en particular el cuadrado axial `[-a,a]²`
(perímetro L₁ `8a`, diámetro L₁ `4a`, razón `2`).

**(c) Corolario que desinfla el «4».** El cuadrado axial *es* el círculo L₁
girado 45° y reescalado. Así que **el mismo cuerpo, girado, pasa de razón 4 a
razón 2 en la misma métrica.** El 4 no es una propiedad de la forma del rombo;
es el valor que toma el cuerpo que resulta ser la bola unidad de la métrica en
la orientación en que lo es. Cualquier lectura del tipo «en taxicab los
círculos tienen π = 4» está confundiendo una forma con un papel.

Y por el mismo lema la razón toma todos los valores de `[2,4]`, enteros
incluidos: 3 se alcanza con `w_x = w_y = 1` y `D = 4/3`.

### 1.3 Cinco (siete) análogos adicionales con constante racional

| # | fórmula L₂ | constante L₂ | análogo L₁ | constante L₁ | métrica del numerador / denominador |
|---|---|---|---|---|---|
| E1 | `V = (4/3)πr³` (bola 3D) | `4π/3` | octaedro `‖x‖₁ ≤ r`: `V₁ = (4/3)r³` | `4/3` (π ↦ **1**) | Lebesgue / — |
| E2 | `V_n = π^{n/2}/Γ(n/2+1)·r^n` | trascendente ∀n | `V_n = 2^n r^n/n!` | **racional ∀n** | Lebesgue / — |
| E3 | `∫_{R^n} e^{-‖x‖₂²} = π^{n/2}` | `π^{n/2}` | `∫_{R^n} e^{-‖x‖₁} = 2^n` | `2^n` | — |
| E4 | densidad normal `(2π)^{-1/2}e^{-x²/2}` | `1/√(2π)` | Laplace `½e^{-\|x\|}` | `1/2` | — |
| E5 | sector: `A = ½r²θ` | `1/2` | sector L₁ con t-radián (vuelta = 8): `A = ¼r²θ` | `1/4` | Lebesgue / longitud L₁ |
| E6 | isoperimétrica `L² ≥ 4πA` | `4π` | `L² ≥ 16A` | `16`, i.e. `4` | Lebesgue / longitud L₁ |
| E7 | arco `∫₀¹dx/√(1-x²) = π/2` | `π/2` | `∫₀¹(1+\|y'\|)dx = 2` | `2` | longitud L₁ |

Notas imprescindibles sobre tres de ellos:

- **E2 es el mejor ejemplo de la línea.** El volumen de la bola L₂ es
  trascendente en toda dimensión; el de la bola L₁ es `2^n/n!`, racional en
  toda dimensión. Y la comparación es limpia porque **ambos se miden con la
  misma medida de Lebesgue**, que no depende de la norma: aquí no hay elección
  de métrica que sesgue el resultado. Es el análogo racional más robusto que se
  encontró.
- **E6 tiene una trampa que es en sí misma un hallazgo, y está en §2.3**: la
  constante isoperimétrica de L₁ es 4, pero **no la realiza el círculo L₁**.
- **E5 depende de una definición** (el t-radián de Krause, que hace que la
  vuelta completa mida 8 porque se define proporcional a la longitud de arco
  L₁). Con esa definición el área del sector es lineal en θ y la constante es
  `1/4`. No es un descubrimiento: es la única normalización que hace que
  `s = rθ` siga valiendo.

### 1.4 Casilla y veredicto de la pregunta 1

**Casilla (II)** — la propiedad se traslada y la constante cambia. No es (III):
`8` y `4` se escriben igual en base 10 y en base 12, y `K(F)` no baja ni un
punto por cambiar de base en esta fórmula (§5, fila F1).

Veredicto: **confirmado que existen análogos L₁ con constante racional, y son
abundantes** (siete aquí, de los cuales E2 es una familia infinita).
Simultáneamente **refutada la lectura fuerte**: la racionalidad no es un rasgo
de la métrica L₁ sino de la combinación (métrica, orientación, elección de
medida para el numerador). Cambiar cualquiera de los tres la destruye — §7.1 y
§7.3.

---

## 2. Pregunta 2 — área, y si existe un «π taxicab» único

### 2.1 Los dos enunciados

**L₂.** `A₂ = πr²`.

**L₁.** El rombo `|x|+|y| ≤ r` tiene medida de Lebesgue `A₁ = 2r²`. Constante
2. **Advertencia de categoría**: aquí el numerador es medida de Lebesgue, que
*no se deriva de la norma*. «Área» es la misma medida en los dos mundos; lo
único que cambió es la forma del cuerpo. Por eso el 2 y el 4 no son dos valores
de la misma magnitud, y esa es la raíz de P1.

### 2.2 La tabla de constantes: seis valores distintos, ninguno privilegiado

| propiedad conservada | numerador / denominador | L₂ | L₁ |
|---|---|--:|--:|
| perímetro / diámetro | long. L₁ / dist. L₁ | π | **4** |
| área / r² | Lebesgue / — | π | **2** |
| área del sector, `A = k r²θ` | Lebesgue / long. L₁ | `1/2` | **`1/4`** |
| volumen 3D, `V = k r³` | Lebesgue / — | `4π/3` | **`4/3`** |
| isoperimétrica, `L²/(4A)` | long. L₁ / Lebesgue | π | **4** (la alcanza el *cuadrado*) |
| isoperimétrica restringida a la bola | ídem | π | **8** |
| normalizador gaussiano | — | `√π` | **1** |
| `dA/dr` frente a `C` | — | coinciden | **fallan**: `4r` vs `8r` |

### 2.3 Por qué en L₂ todas estas constantes son el mismo número

En el plano euclidiano el círculo es **tres cosas a la vez**:

1. la bola de la norma,
2. la órbita del grupo de isometrías (`SO(2)`, continuo),
3. la solución del problema isoperimétrico.

Esa triple coincidencia es la que hace que «π» tenga un solo valor. En L₁:

1. la bola es el rombo;
2. el grupo de isometrías lineales es el diedral de orden 8 (las simetrías del
   rombo), **finito**;
3. el óptimo isoperimétrico **no es el rombo**.

El punto 3 merece prueba, porque es el hallazgo más fuerte de esta línea. Por
el lema de §1.2, maximizar el área de Lebesgue con perímetro L₁ fijo `L` es
maximizar `A(K) ≤ w_x·w_y` sujeto a `w_x + w_y = L/2`, lo que da
`A ≤ L²/16` con igualdad **solo** para el cuadrado axial de lado `L/4`. Para el
rombo de radio r: `L = 8r`, `A = 2r²`, mientras que `L²/16 = 4r²`. Es decir:

> **El círculo L₁ alcanza exactamente la mitad del área máxima posible para su
> propio perímetro L₁.** El óptimo isoperimétrico del plano L₁ es el cuadrado
> axial, no el círculo L₁.

Esto no es una rareza de cálculo: es la teoría de Busemann del área en espacios
de Minkowski, donde el minimizador es el *isoperimetrix*, el dual de la bola
girado 90°. El dual de `B₁` es `B_∞` (el cuadrado), y girado 90° sigue siendo
ese cuadrado. En L₂ el dual del disco girado es el disco, y ahí está la razón
estructural de la coincidencia euclidiana.

La consecuencia retórica es dura: **el papel más famoso de π —ser la constante
isoperimétrica— en L₁ le corresponde a un cuerpo que no es el círculo L₁.**

### 2.4 El fallo de `dA/dr = C`

En L₂, `d(πr²)/dr = 2πr = C`. En L₁, `d(2r²)/dr = 4r`, pero `C₁ = 8r`: sobra un
factor 2 exacto. La identidad euclidiana es la fórmula del cono
`A = ½∮ p ds` con `p` la distancia del origen a la recta tangente y `ds`
euclidiano. Para el rombo `p = r/√2` (no `r`) y `∮ds = 4√2 r`, y en efecto
`½·(r/√2)·4√2 r = 2r²`. **El √2 que la formulación L₁ había eliminado reaparece
en cuanto se pide una identidad que relacione área y perímetro.** Recuperar la
identidad exige introducir la norma dual, es decir, una segunda métrica: no se
sale del paso con una sola.

### 2.5 Casilla y veredicto de la pregunta 2 — P1

**Casilla (II)**, con un componente de casilla (I): la *unicidad* de π es una
propiedad de L₂ y no tiene análogo en L₁, porque las tres identidades del
círculo euclidiano (bola, órbita, óptimo) son propiedades de `SO(2)` y de la
autodualidad del disco.

**P1: CONFIRMADA, y con margen mayor del que la predicción pedía.** La
predicción anotaba dos valores (4 para perímetro/diámetro y 2 para área); se
encontraron **al menos seis mutuamente inconsistentes** (4, 2, 1/4, 4/3, 4 con
otro realizador, 8, 1), y además la razón perímetro/diámetro no es ni siquiera
constante sobre cuerpos: recorre todo `[2,4]`, y **el propio círculo L₁ girado
45° da 2**. La causa que la predicción proponía —la anisotropía de la bola L₁—
es correcta pero insuficiente; la causa precisa es la finitud del grupo de
isometrías más la pérdida de la autodualidad.

---

## 3. Pregunta 7 — integrales que contienen π

### 3.1 El par del planteamiento

| | L₂ | L₁ |
|---|---|---|
| enunciado | `∫₋₁¹ √(1-x²) dx = π/2` | `∫₋₁¹ (1-\|x\|) dx = 1` |
| qué es | área de Lebesgue de media bola L₂ | área de Lebesgue de media bola L₁ |
| constante | `π/2` | `1` |

```
$ target/release/dz-cli.exe eval "pi/2" --mode fixed --scale 10
~1;6X2404848
```

(`π/2` con escala 8 es `~1;6X2404846` en `02-referencia-medida.md` §2; el
último dígito difiere por truncamiento a distinta escala, no hay
contradicción.)

**Lectura honesta que el planteamiento pedía, y algo más:** no se ha demostrado
que π = 2; se ha integrado otra función. Y la observación adicional es que
**esta integral no es un hallazgo independiente**: es la pregunta 2 disfrazada
de integral. `∫₋₁¹` de la mitad superior de la bola *es* la mitad del área, así
que la «constante 1» es exactamente la «constante 2» de §2 dividida por dos. No
cuenta como familia nueva.

### 3.2 La integral que sí añade algo: el mismo π/2 se parte en dos

En L₂ el número `π/2` aparece en dos papeles distintos:

- como **área**: `∫₋₁¹ √(1-x²) dx = π/2`;
- como **longitud de arco**: `∫₀¹ dx/√(1-x²) = π/2` (el cuarto de
  circunferencia).

Que sean el mismo número es un accidente de L₂. En L₁:

- área de medio rombo: `∫₋₁¹ (1-|x|) dx = 1`;
- longitud L₁ del cuarto de rombo: `∫₀¹ (|dx|+|dy|)/dx · dx = ∫₀¹ (1+|y'|) dx =
  ∫₀¹ 2 dx = 2`.

> **Un único número euclidiano, π/2, se desdobla en 1 y en 2 al pasar a L₁,
> exactamente por un factor 2.** Ésta es la demostración más corta y más
> comprobable de P1 que se encontró: no hay que discutir qué constante «es» el
> π taxicab, basta observar que la misma cifra euclidiana tiene dos
> preimágenes.

### 3.3 La familia `Lp` interpolante

El cuarto de área de la bola `Lp` en 2D es

```
∫₀¹ (1 - x^p)^{1/p} dx = Γ(1+1/p)² / Γ(1+2/p)
```

lo que da la constante de área `A_p = 4Γ(1+1/p)²/Γ(1+2/p)`: vale `1/2·4 = 2`
en `p=1`, `π/4·4 = π` en `p=2`, y `→4` cuando `p→∞`. Es la interpolación
explícita entre la integral racional y la trascendente, y se usa en §4.

### 3.4 Otras integrales clásicas con π — resultado mayoritariamente negativo

| integral L₂ | valor | análogo L₁ propuesto | resultado |
|---|---|---|---|
| `∫₀^∞ e^{-x²}dx` | `√π/2` | `∫₀^∞ e^{-x}dx` | `1`, racional — **(II)**, pero ver §7.6 |
| `∫₋∞^∞ dx/(1+x²)` | `π` | `∫₋∞^∞ dx/(1+\|x\|)` | **diverge** logarítmicamente. Sin análogo |
| `∫₀^∞ sin x/x dx` | `π/2` | — | π sin contenido métrico. **(I)** |
| `∑ 1/n² ` | `π²/6` | — | π de la fórmula del producto de sen. **(I)** |
| `∫₀^{π/2} sin^n` (Wallis) | con π | — | el dominio *es* un ángulo euclidiano. **(I)** |
| `∫_{B₂} 1` | `π^{n/2}/Γ(n/2+1)` | `∫_{B₁} 1 = 2^n/n!` | racional ∀n — **(II)**, el mejor caso |

De seis familias examinadas, **tres son de casilla (I)** (no tienen análogo
porque su π no viene de ninguna métrica, sino de la teoría de funciones), una
**diverge** y solo dos trasladan.

### 3.5 Casilla y veredicto de la pregunta 7

**Casilla (II) para el par del planteamiento y para las bolas
n-dimensionales; casilla (I) para la mayoría de las integrales clásicas con
π.** El fenómeno conceptual que el planteamiento pedía describir es correcto,
pero la extensión a «otras integrales clásicas» sale **mal** más veces que
bien: el π de `∑1/n² = π²/6` o de `∫sin x/x` no es una constante geométrica que
admita sustitución de métrica, es un valor especial de funciones analíticas, y
no hay nada que sustituir.

---

## 4. P2 — ¿es L₁ (con L∞) la única `Lp` con razón perímetro/diámetro entera?

### 4.1 Formulación precisa

Para la bola unidad `B_p ⊂ R²`, medida **con la propia norma `‖·‖_p`** en
numerador y denominador: `diam_p(B_p) = 2` siempre, así que la razón es la
semi-longitud del borde, que se denota `π_p`. Con la parametrización
`x = (cos t)^{2/p}`, `y = (sin t)^{2/p}` (que satisface `x^p + y^p = 1`):

```
π_p = 2 ∫₀^{π/2} (2/p)·[ cos^{2-p}t · sin^p t + sin^{2-p}t · cos^p t ]^{1/p} dt
```

Se comprueba a mano en los dos extremos: en `p=2` el corchete vale 1 y la
integral es `π/2`, luego `π_2 = π`; en `p=1` el integrando es `2·sin 2t` y la
integral vale 2, luego `π_1 = 4`.

### 4.2 Valores calculados (cuadratura tanh-sinh, doble precisión, script de §0.3)

| p | π_p | dual `p' = p/(p-1)` | π_{p'} | π_p en base 12 |
|---|--:|--:|--:|---|
| 1 | 4.000000000000 | ∞ | 4.000000000000 | `4` |
| 1,1 | 3.757084367281 | 11 | 3.752069662792 (†) | `3;9102X99817X0387…` |
| 1,25 | 3.498456808045 | 5 | 3.498455151364 | `3;5E94000782X507X…` |
| 1,5 | 3.259767993059 | 3 | 3.259767992966 | `3;314X670X2439X40…` |
| 1,75 | 3.163920671785 | 2,333 | 3.163920671785 | `3;1E730860680975E…` |
| **2** | **3.141592653590** | 2 | 3.141592653590 | `~3;18480949` (†† π) |
| 2,5 | 3.185083548912 | 1,667 | 3.185083548913 | `3;2279X862373786…` |
| 3 | 3.259767992966 | 1,5 | 3.259767993059 | `3;314X670X2439X40…` (= π_{1,5}, por dualidad) |
| 4 | 3.396934781510 | 1,333 | 3.396934823628 | `3;491XX0XX923272…` |
| ∞ | 4 (límite) | 1 | 4 | `4` |

(†) los desacuerdos en la última columna del par dual crecen con `p` porque la
cuadratura pierde precisión cuando la singularidad en los extremos se agudiza;
para `p ≳ 20` el valor directo es basura (a `p = 1000` el script devuelve
0,0029, absurdo) y hay que leer el dual. Se reporta como limitación, no se
esconde. (††) la fila `p=2` es π y su numeral dozenal se cita de
`02-referencia-medida.md` §2, no se recalculó.

Los numerales dozenales de esta columna salen de pasar el decimal por la
calculadora, p. ej.:

```
$ target/release/dz-cli.exe eval "3.259767993" --input-base 10 --mode fixed --scale 8
3;314X670X2439X407543110X3 (periodica en base 12, valor truncado)
```

### 4.3 Tres hechos estructurales

1. **Dualidad.** `π_p = π_{p'}` con `1/p + 1/p' = 1`, verificada numéricamente
   a 1e-13 en `p ∈ [1.5, 4]` (teorema de Schäffer: en el plano la longitud de
   la bola unidad coincide con la de la bola dual). Explica que `π_1 = π_∞`.
2. **Mínimo en el punto autodual.** `π_p` decrece en `[1,2]` y crece en
   `[2,∞]`, con mínimo `π_2 = π`. Medido: 3,141705 / 3,141620 / **3,141593** /
   3,141620 / 3,141701 en `p = 1,98 … 2,02`.
3. **Rango.** `π_p ∈ [π, 4]`, con máximo 4 solo en los extremos. Esto es un caso
   particular del **teorema de Gołąb**: en todo plano de Minkowski la
   semi-longitud de la bola unidad está en `[3,4]`, vale 4 **si y solo si** la
   bola es un paralelogramo, y vale 3 si y solo si es un hexágono afínmente
   regular.

### 4.4 Veredicto de P2: confirmada en su letra, refutada en su cláusula de escape

**En la lectura «entera»: P2 se CONFIRMA, y con demostración, no con
numérico.** El único entero en `[π, 4]` es 4; por Gołąb, `π_p = 4` exige que
`B_p` sea un paralelogramo; `B_p` es estrictamente convexo para todo
`p ∈ (1,∞)` y por tanto no es un paralelogramo. Luego `π_p < 4` estrictamente
en `(1,∞)` y **L₁ y L∞ son las únicas `Lp` con razón perímetro/diámetro
entera.** Esto es una prueba, no una medición.

**En la lectura «racional» —la cláusula del plan «si aparece un p intermedio
racional, P2 cae»— la cláusula cae trivialmente.** `π_p` es continua y
estrictamente monótona en `[1,2]` con `π_1 = 4` y `π_2 = π`, así que por el
teorema del valor intermedio toma **todos** los valores de `(π, 4)`, racionales
incluidos, y hay infinitos. Localizados por bisección:

| valor racional de `π_p` | p que lo realiza | verificación |
|--:|---|--:|
| 3,9 | 1,037943870312 | 3,900000000000 |
| 3,75 | 1,103368088775 | 3,750000000000 |
| 3,5 | 1,248882088864 | 3,500000000000 |
| 3,25 | 1,516530019753 | 3,250000000000 |
| 3,2 | 1,623002382384 | 3,200000000000 |

```
$ target/release/dz-cli.exe eval "1.248882089" --input-base 10 --mode fixed --scale 8
1;2EX099E289539586E4128398 (periodica en base 12, valor truncado)
```

Conclusión: **la cláusula estaba mal redactada como falsificador.** Lo que
distingue a L₁ y L∞ no es que su constante sea racional (hay un continuo de `p`
con constante racional) sino que sea **entera**, y más precisamente que sea el
**máximo** del rango, alcanzado por la caracterización rígida de Gołąb.

**Lo que queda abierto, y se declara abierto.** La pregunta interesante que P2
no formuló: ¿existe algún `p` **racional** distinto de 1 y ∞ con `π_p`
racional? No se sabe. Los `π_p` para `p` racional son periodos de tipo Γ y se
espera que sean trascendentes (`π_{3/2} = π_3 ≈ 3,2598` no está reconocido como
ningún valor cerrado conocido), pero **no se conoce demostración** y este
informe no la aporta. Es una pregunta de trascendencia, no de geometría, y cae
fuera del alcance de la línea A.

### 4.5 Dos avisos que limitan el alcance de P2

- **P2 habla solo de la familia `Lp`.** Entre *todas* las normas la
  integralidad no es rara: por Gołąb el hexágono afínmente regular da
  exactamente **3**, otro entero. Así que «constante entera» la tienen al menos
  dos clases afines de normas, y L₁ no es la única norma con esa propiedad,
  solo la única `Lp`.
- **Para el área, la versión análoga de P2 es FALSA.** La constante de área
  `A_p = 4Γ(1+1/p)²/Γ(1+2/p)` crece de 2 a 4, así que pasa por el **entero 3**
  en un `p` intermedio. Localizado por bisección: `A_p = 3.000000000000` en
  `p = 1,791473849870`.

  ```
  $ target/release/dz-cli.exe eval "1.791473850" --input-base 10 --mode fixed --scale 8
  1;95E800304427X9XX08X7X386 (periodica en base 12, valor truncado)
  ```

  Es decir: **existe una bola `Lp` cuya área vale exactamente `3r²`**. Si
  alguien quisiera argumentar «L₁ es especial porque sus constantes son
  enteras», este contraejemplo lo impide para la propiedad «área», y refuerza
  que la unicidad de L₁ en P2 depende de la rigidez de Gołąb sobre el
  *perímetro*, no de ninguna virtud general de la integralidad.

---

## 5. Tabla K(F) — cuatro sumandos desglosados, decimal y dozenal

Reglas de conteo: las de `00-plan-investigacion.md` §2.1. `N_per` es el único
sumando sensible a la base, y por eso se da en dos columnas. `r` es variable, no
constante, y no cuenta.

| # | fórmula | N_raíces | N_irrac. | N_oper. | N_per base 10 | N_per base 12 | **K dec** | **K doz** |
|---|---|--:|--:|--:|--:|--:|--:|--:|
| F1a | `C₂ = 2πr` | 0 | 1 (π) | 2 | 0 | 0 | **3** | **3** |
| F1b | `C₁ = 8r` | 0 | 0 | 1 | 0 | 0 | **1** | **1** |
| F2a | `A₂ = πr²` | 0 | 1 | 2 | 0 | 0 | **3** | **3** |
| F2b | `A₁ = 2r²` | 0 | 0 | 2 | 0 | 0 | **2** | **2** |
| F3a | `V₂ = (4/3)πr³` | 0 | 1 | 4 | 1 (`4/3`) | 0 (`1;4`) | **6** | **5** |
| F3b | `V₁ = (4/3)r³` | 0 | 0 | 3 | 1 | 0 | **4** | **3** |
| F4a | `V₂,₅ = (8π²/15)r⁵` | 0 | 1 | 5 | 1 (`4/15`… ) | 1 | **7** | **7** |
| F4b | `V₁,₅ = (4/15)r⁵` | 0 | 0 | 3 | 1 | 1 | **4** | **4** |
| F5a | isoperim. `L² ≥ 4πA` | 0 | 1 | 3 | 0 | 0 | **4** | **4** |
| F5b | isoperim. `L² ≥ 16A` | 0 | 0 | 2 | 0 | 0 | **2** | **2** |
| F6a | normal `(2π)^{-1/2}e^{-x²/2}` | 1 | 1 | 7 | 0 | 0 | **9** | **9** |
| F6b | Laplace `½e^{-\|x\|}` | 0 | 0 | 5 | 0 | 0 | **5** | **5** |
| F7a | `∫₀¹dx/√(1-x²) = π/2` | 1 | 1 | 2 | 0 | 0 | **4** | **4** |
| F7b | `∫₀¹(1+\|y'\|)dx = 2` | 0 | 0 | 2 | 0 | 0 | **2** | **2** |
| F8a | superficie 3D `S₂ = 4πr²` | 0 | 1 (π) | 2 | 0 | 0 | **3** | **3** |
| F8b | superficie 3D `S₁ = 4√3 r²` | 1 | 1 (√3) | 2 | 0 | 0 | **4** | **4** |

**F8 es la única fila del informe donde L₁ SUBE `K(F)`** (de 3 a 4), y por eso
se incluye: es el contraejemplo al patrón de las otras siete. Ver §7.1.

### 5.1 Lectura de la tabla — y es desfavorable a la mitad dozenal de H₁

- **La métrica baja K en siete de las ocho parejas**, entre 1 y 4 puntos, y la
  **sube en una** (F8, la superficie del octaedro). La bajada era esperable y el
  propio plan (§2.1) avisa de que es «casi tautológico»: K premia lo lineal por
  tramos y L₁ es lineal por tramos. **Valor probatorio bajo.** La subida, en
  cambio, sí es informativa, y es de las cosas que el plan pedía buscar
  activamente.
- **La base baja K en exactamente una fila de cada columna, F3, y por un solo
  punto**, porque `4/3` termina en base 12 (`1;4`) y no en base 10. En las otras
  siete parejas la columna dozenal es idéntica a la decimal.
- **Y en esa única fila, la base ayuda igual a la fórmula euclidiana:** F3a baja
  de 6 a 5 y F3b de 4 a 3, la misma unidad. El efecto dozenal es **ortogonal a
  la métrica**, que es literalmente lo que predice P5.
- **A partir de dimensión 5 la base 12 deja de ayudar** (F4): los factoriales
  introducen 5 y 7, y `4/15`, `4/45`, `8/315` no terminan en ninguna de las dos
  bases. Verificado:

```
$ target/release/dz-cli.exe eval "4/3"    --input-base 10 --mode fixed --scale 12
1;4
$ target/release/dz-cli.exe eval "2/3"    --input-base 10 --mode fixed --scale 12
0;8
$ target/release/dz-cli.exe eval "4/15"   --input-base 10 --mode fixed --scale 12
0;324972497249724972497249 (periodica en base 12, valor truncado)
$ target/release/dz-cli.exe eval "4/45"   --input-base 10 --mode fixed --scale 12
0;109724972497249724972497 (periodica en base 12, valor truncado)
$ target/release/dz-cli.exe eval "8/315"  --input-base 10 --mode fixed --scale 12
0;037X76620828XE7X76620828 (periodica en base 12, valor truncado)
$ target/release/dz-cli.exe eval "2/315"  --input-base 10 --mode fixed --scale 12
0;00XE7X76620828XE7X766208 (periodica en base 12, valor truncado)
$ target/release/dz-cli.exe eval "4/2835" --input-base 10 --mode fixed --scale 12
0;0025310414559E3931041455 (periodica en base 12, valor truncado)
```

Volumen de la bola L₁ en `R^n`, `2^n/n!`, y dónde termina en base 12:

| n | `2^n/n!` | base 12 | base 10 |
|--:|---|---|---|
| 1, 2 | `2` | termina | termina |
| 3 | `4/3` | **termina** (`1;4`) | no |
| 4 | `2/3` | **termina** (`0;8`) | no |
| 5 | `4/15` | no | no |
| 6 | `4/45` | no | no |
| 7 | `8/315` | no | no |
| 8 | `2/315` | no | no |
| 9 | `4/2835` | no | no |

La ventaja dozenal de esta línea es exactamente **dos casillas de una familia
infinita**, `n = 3` y `n = 4`, y desaparece para siempre en `n = 5` porque 5
divide a `5!`. Citando `02-referencia-medida.md` §1: `1/3 = 0;4` termina, pero
`1/5` y `1/X` **no** terminan en base 12 aunque sí en base 10. Esta línea no
encontró ninguna constante geométrica con quintos en el denominador donde la
base 12 *perdiera*, pero la tabla de arriba muestra que a partir de dimensión 5
todos los denominadores traen un 5.

---

## 6. Coste medido en puertas

Las tres fórmulas cabecera, con `r = 3` (entero, para que no haya divisiones de
reescalado que contaminen la comparación) y escala 8, backend MVL por defecto:

| fórmula | puertas | profundidad | comando |
|---|--:|--:|---|
| `C₂ = 2πr` | 39.081 | 15.168 | `eval "2*pi*3" --mode fixed --scale 8 --gates` |
| `C₁ = 8r` | **42.501** | 441 | `eval "8*3" --mode fixed --scale 8 --gates` |
| `A₂ = πr²` | 16.945 | 15.168 | `eval "pi*3^2" --mode fixed --scale 8 --gates` |
| `A₁ = 2r²` | **112.542** | 815 | `eval "2*3^2" --mode fixed --scale 8 --gates` |
| `V₂ = (4π/3)r³` | 1.807.101 | 16.070 | `eval "4*pi/3*3^3" --mode fixed --scale 8 --gates` |
| `V₁ = (4/3)r³` | **454.141** | 1.312 | `eval "4/3*3^3" --mode fixed --scale 8 --gates` |
| `1/√(2π)` (normal) | 41.242.035 | 18.510 | `eval "1/sqrt(2*pi)" --mode fixed --scale 8 --gates` |
| `1/2` (Laplace) | **283.054** | 1.312 | `eval "1/2" --mode fixed --scale 8 --gates` |

Salidas literales de los dos casos extremos:

```
$ target/release/dz-cli.exe eval "1/sqrt(2*pi)" --mode fixed --scale 8 --gates
~0;4954573236779X2E6E53E23X
gates: cycle=8495652 min=13915594 max=10280753 lit=6990648 thr=1559388 total=41242035 depth=18510 recon=0

$ target/release/dz-cli.exe eval "1/2" --mode fixed --scale 8 --gates
0;6
gates: cycle=84480 min=75272 max=60422 lit=46944 thr=15936 total=283054 depth=1312 recon=0
```

### 6.1 Lectura, que es en dos tercios negativa

- **En circunferencia y área la formulación L₁ sale MÁS CARA**: 42.501 frente a
  39.081, y 112.542 frente a 16.945 (un factor 6,6 en contra). El instrumento
  objetivo del plan **se niega a corroborar** la ganancia simbólica en los dos
  casos cabecera de las preguntas 1 y 2.
- **La causa principal está declarada en §0.4**: `pi` cuesta 0 puertas por
  diseño, así que la fórmula euclidiana recibe gratis su única dificultad. La
  comparación es un límite inferior del coste L₂, no una medición equitativa. No
  hay forma de desactivar la memoización desde el CLI, así que esto no se puede
  arreglar dentro del instrumento; se declara.
- **La causa secundaria es un artefacto de implementación, y es importante
  saberlo**: en modo fijo el coste está dominado por las divisiones
  restauradoras de reescalado, no por la forma simbólica. Medido en caliente:
  `eval "1*1"` cuesta 21.900 puertas y `eval "E*E"` cuesta 69.720 —el coste de
  una multiplicación es **dependiente de los datos**— y `eval "8*3;6"` cuesta
  138.511 porque alinear escalas obliga a una división. Por eso todas las
  mediciones de esta tabla usan `r = 3` entero.
- **Donde L₁ gana, gana por lo que dice `02-referencia-medida.md` §3**: cuando
  la fórmula L₂ contiene una raíz cuadrada o una división por un
  multidígito trascendente. `V₂/V₁` = 4,0×; `1/√(2π)` frente a `1/2` = **146×**.
  El coste vive en el algoritmo (Newton, división restauradora), no en el valor
  ni en la notación — igual que allí √(3²+4²) cuesta 43.631.965 puertas contra
  39.469 de 3+4 **pese a que el resultado euclidiano es el entero exacto 5**.
- **Ninguna de estas cifras es evidencia dozenal.** Todas son casilla (II). El
  diferencial de casilla (III) está en `02-referencia-medida.md` §4 y no
  interviene aquí, porque ninguna de las constantes de esta línea se lee en base
  10 en un uso realista.

---

## 7. Lo que no funcionó

Sección obligatoria. Se buscaron activamente análogos L₁ que **no** simplifican,
o que simplifican **perdiendo** algo. Se encontraron once.

### 7.1 La superficie de la bola L₁ en 3D es irracional

El patrón racional se rompe en el primer objeto de dimensión 3 que no es un
volumen. El octaedro `‖x‖₁ ≤ r` tiene 8 caras triangulares equiláteras de lado
`√2 r`, de área `(√3/2)r² ` cada una, así que su superficie (medida de Hausdorff
2-dimensional, la única canónica) es `4√3 r²` — irracional, frente al
`4πr²` euclidiano que al menos era un múltiplo racional de π.

```
$ target/release/dz-cli.exe eval "4*sqrt(3)" --mode fixed --scale 8
~6;E17E28
```

Cambiar un irracional trascendente por un irracional algebraico **empeora
K(F)**: `N_irracionales` sigue valiendo 1 (√3 cuenta igual que π) y
`N_raíces` sube de 0 a 1, así que `K` pasa de **3 a 4** (§5, fila F8). Es el
único caso del informe en que la formulación L₁ es *más* compleja que la
euclidiana según el propio índice del plan, y por eso se anota: el volumen L₁ es
racional en toda dimensión, pero **la racionalidad no se hereda del volumen a su
borde**.

Matiz honesto: existen definiciones alternativas de área de superficie en
espacios de Minkowski (Busemann, Holmes–Thompson) cuyas normalizaciones podrían
devolver un valor racional para el octaedro. No se exploraron aquí, y mientras
no se elija una, la medida de Hausdorff es la única sin parámetro libre. Elegir
la definición que dé el número bonito sería justamente el modo de fallo que el
plan quiere evitar.

> **Corrección 2026-09-25** (`15-hallazgo-frontera-bola-L1-n-dimensiones.md`).
> La generalización de esta sección no se sostiene. Con la misma medida de
> Hausdorff, la bola L₁ de 4D tiene frontera `(16/3)r³`, que es racional; el
> cubo L∞ de 3D tiene `24r²`; y un octaedro isométrico a `ℓ₁³`,
> `{|x|+2|y|+2|z| ≤ 1}`, tiene superficie 3. Hausdorff sí tiene un parámetro
> libre, la inmersión euclidiana, y el `√3` es la norma de la normal `(1,1,1)`.
> Las medidas invariantes (Busemann, Holmes–Thompson) son trascendentes para
> todo politopo racional en `n ≥ 3` (Teorema B de `15`), así que tampoco son
> una propiedad de L₁. Se deja el texto original visible.

### 7.2 `dA/dr = C` se rompe, y arreglarlo exige una segunda métrica

Detallado en §2.4: `4r ≠ 8r`. El √2 eliminado reaparece exactamente cuando se
pide una identidad que ligue área y perímetro, y la reparación pasa por la norma
dual. Una formulación «más simple» que pierde una identidad que en L₂ es
gratuita.

### 7.3 El 4 no sobrevive a una rotación de 45°

§1.2(c): el mismo cuerpo (el círculo L₁) girado 45° tiene razón
perímetro/diámetro **2** en la misma métrica L₁. Y su perímetro L₁ se multiplica
por √2 al girarlo, porque el lema de §1.2 mide la caja envolvente. La
racionalidad de las constantes L₁ es un artefacto de la **alineación con los
ejes**, no una propiedad geométrica. Cualquier problema cuyo planteamiento no
venga con ejes privilegiados no hereda nada de esto.

### 7.4 La circunferencia L₁ pierde la inyectividad

§1.2(a): `C₁` solo depende de la caja envolvente, así que infinitos cuerpos
convexos no congruentes comparten «circunferencia». La fórmula es más simple
porque mide menos. En L₂ el perímetro, junto con el área, casi determina el
cuerpo (isoperimetría); en L₁ no determina casi nada.

### 7.5 El círculo L₁ no resuelve el problema isoperimétrico de L₁

§2.3: alcanza la **mitad** del área máxima para su perímetro. El papel más
célebre de π queda en L₁ asignado al cuadrado axial. Este es el caso más claro
de «la propiedad sobrevive pero se le desengancha del objeto», y es la razón
de fondo de P1.

### 7.6 Laplace es más simple que Gauss, y pierde las tres propiedades por las
que Gauss importa

`½e^{-|x|}` no tiene π y K baja de 9 a 5 (§5, F6). Pero la distribución de
Laplace **no es estable bajo convolución** (la suma de dos Laplace no es
Laplace), **no es autotransformada de Fourier** (su transformada es
`1/(1+ξ²)`, una lorentziana, no otra Laplace) y —lo decisivo— **el teorema
central del límite sigue entregando `√(2π)`** al sumar variables Laplace. La
simplificación es real y es local: en cuanto se suman dos variables, π vuelve.
Esto es un caso de libro de «más simple por perder una propiedad».

### 7.7 La familia Cauchy/arctan no tiene análogo: diverge

`∫dx/(1+x²) = π` pero `∫dx/(1+|x|)` diverge. Bajar el exponente de 2 a 1 en el
denominador mata la integrabilidad. Para recuperarla hay que poner
`1/(1+|x|)^{1+ε}`, y entonces la constante es `2/ε`, que no es el análogo de
nada: se ha cambiado la familia, no la métrica.

### 7.8 Tres de las integrales clásicas con π son de casilla (I) y no admiten
ninguna sustitución

`∑1/n² = π²/6`, `∫₀^∞ sin x/x = π/2`, las integrales de Wallis. Su π procede
del producto de Euler para el seno o de la periodicidad de la exponencial
compleja, no de ninguna bola unidad. No hay «versión L₁» que construir, ni
siquiera mala. El planteamiento 7 pedía «extender el análisis a otras
integrales clásicas» y la respuesta honesta es que la extensión falla en la
mayoría de los casos.

### 7.9 La aguja de Buffon: el análogo está mal planteado, no es más simple

`P = 2L/(πd)` sale de promediar sobre un ángulo **uniforme**, es decir de una
medida invariante por rotación. L₁ no tiene ninguna, porque su grupo de
isometrías es finito. Así que o se conserva el ángulo euclidiano —y entonces π
se queda, en un problema supuestamente L₁— o se elige una medida angular
arbitraria (el t-radián), y la constante resultante es un artefacto de esa
elección, no un hallazgo. No es una simplificación: es una indeterminación.

### 7.10 El contador de puertas no corrobora las preguntas 1 y 2

§6.1: `C₁` y `A₁` salen **más caras** que `C₂` y `A₂` en el instrumento
objetivo. Sea por la memoización de `pi` (sesgo declarado) o por las divisiones
de reescalado (artefacto de implementación), el hecho es que **la única medida
independiente del criterio humano de que dispone este proyecto no respalda las
dos fórmulas cabecera de la línea A**. Se deja escrito así.

### 7.11 La base 12 no aporta nada estructural a esta línea

§5.1: de ocho parejas de fórmulas, la base cambia `K(F)` en **una**, por **un**
punto, y ayuda por igual a la versión euclidiana. Y la familia donde podría
haber sido sistemática (volúmenes `2^n/n!`) deja de beneficiarse en `n = 5`
porque entra el factor 5, que en base 12 es el factor malo. Es exactamente lo
que predice P5, y esta línea no encontró ni un indicio en contra.

---

## 8. Veredictos

| ítem | casilla | veredicto |
|---|---|---|
| Pregunta 1 (circunferencia, ≥5 análogos) | **(II)** | Confirmada la existencia de análogos racionales (7 dados, uno de ellos una familia infinita). Refutada la lectura de que la racionalidad sea propiedad de L₁: depende de métrica + orientación + elección de medida. |
| Pregunta 2 (área, «π taxicab» único) | **(II)**, con un núcleo **(I)** | No hay π taxicab único: ≥6 constantes inconsistentes. La *unicidad* es propiedad de L₂ y es de casilla (I). |
| Pregunta 7 (integrales con π) | **(II)** el par del planteamiento y las bolas n-dim; **(I)** la mayoría de las integrales clásicas | El par `π/2 ↦ 1` no es un hallazgo independiente (es la pregunta 2 en forma de integral). La extensión a otras integrales falla en 4 de 6 familias. |
| **P1** | — | **CONFIRMADA**, con margen mayor que el predicho. |
| **P2** | — | **CONFIRMADA en la lectura «entera», con demostración** (Gołąb + estricta convexidad). Su cláusula de escape («si aparece un p intermedio racional») **cae trivialmente** por el teorema del valor intermedio: hay infinitos p con `π_p` racional. La cláusula estaba mal redactada como falsificador. |
| **P5** (no era de esta línea, pero se cruzó) | — | Corroborada aquí: el único efecto dozenal medido es `N_periodicos`, en una fila, y ayuda igual a la fórmula euclidiana. |

### 8.1 Aporte a H₁

H₁ exige, para «confirmada», ≥3 familias con ganancia en K **y** en coste
medido, y al menos una ganancia que no venga de `N_periodicos`. Esta línea
aporta:

- **ganancia en K: 7 de 8 familias, y pérdida en 1** (F8, la superficie del
  octaedro, donde K sube de 3 a 4), pero con el valor probatorio que el propio
  plan le asigna (sesgo declarado de K hacia lo lineal por tramos);
- **ganancia en coste medido: 2 de 4 medidas** (volumen 4,0×, normalizador
  146×), y **2 en contra** (circunferencia 0,92×, área 0,15×);
- **ganancia no procedente de `N_periodicos`: sí, y es toda la ganancia
  simbólica** — pero es ganancia de **métrica**, casilla (II), y por tanto
  «nunca evidencia a favor de la base 12» según §1 del plan.

Contribución neta: la mitad métrica de H₁ recibe apoyo débil (y tautológico en
K, mixto en coste); la mitad dozenal de H₁ **no recibe ningún apoyo de esta
línea**, y el único efecto observado es el previsto por P5.

### 8.2 El hallazgo más inesperado

No es el «4». Es que **el círculo L₁ no es el óptimo isoperimétrico del plano
L₁** —alcanza exactamente la mitad del área máxima para su propio perímetro
L₁, y el óptimo es el cuadrado alineado con los ejes—, combinado con que
**la longitud L₁ de cualquier convexo es `2(w_x + w_y)`** y por tanto no ve
más que la caja envolvente. Las dos cosas juntas explican de una vez por qué no
hay un π taxicab: en L₂ el círculo es simultáneamente la bola, la órbita del
grupo de isometrías y el óptimo isoperimétrico, y es esa triple coincidencia
—no la métrica— la que le da a π un valor único. En L₁ las tres se separan, y
cada una se lleva su constante.
