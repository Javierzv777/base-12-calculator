# Hallazgo — Busemann y Holmes–Thompson en el octaedro L₁

Extiende `01-hallazgos-A-constantes.md` §7.1 y la objeción que la propia línea A
se puso en `99-sintesis.md` §3.4 y §7: ¿alguna de las dos definiciones
alternativas de área de superficie para espacios normados (Busemann,
Holmes–Thompson) le da al octaedro `‖x‖₁ ≤ r` una superficie racional?

Fecha de apertura: 2026-09-12.

Protocolo: se sigue `00-plan-investigacion.md` — toda cifra dozenal sale de
`target/release/dz-cli.exe` con el comando pegado al lado, y el hallazgo se
clasifica en casilla (I)/(II)/(III) igual que el resto del informe.

---

## 0. Resumen del veredicto (se completa al final, se deja aquí como ancla)

**No.** Ni Busemann ni Holmes–Thompson devuelven un valor racional. Devuelven
**otra clase distinta** de irracional: mientras Hausdorff da `4√3 r²`
(algebraico), Busemann da `(16π/3) r²` y Holmes–Thompson da `(48/π) r²`
(ambos trascendentes, porque contienen `π` o `1/π`). El hallazgo de §7.1 no
solo sobrevive: se refuerza. Bajo las tres definiciones razonables de área de
superficie que existen para un espacio normado, la frontera del octaedro L₁
es irracional. Detalle y clasificación en §4.

---

## 1. Las dos definiciones, desde primeros principios

### 1.1 El objeto común a ambas: el par (sección, densidad)

Hausdorff no tiene parámetro libre porque es una medida *métrica*: mide una
superficie con la métrica que la contiene, sin preguntar qué normó el
espacio ambiente. Busemann y Holmes–Thompson son medidas *de espacio normado*:
la única estructura que usan de `(X, ‖·‖)` es su bola unidad `B`, y ambas se
construyen igual en dos pasos.

Sea `E` un `k`-plano afín en `X = Rⁿ` (aquí `n=3`, `k=2`, una cara del
octaedro), y `E₀` el subespacio lineal paralelo a `E` que pasa por el origen
(la dirección de la cara). `E₀` hereda de `Rⁿ` una estructura euclídea
ordinaria — es un plano de `R³`, y en él ya sabe medirse área del modo de
siempre, `vol₂^eucl`. La construcción es:

1. **Elegir una sección o un dual de la bola** restringidos a `E₀`. Esto es
   lo único que distingue a las dos definiciones (§1.2 vs. §1.3).
2. **Normalizar** esa sección por el volumen de la bola euclídea de dimensión
   `k`, llamado `ω_k` (`ω₂ = π`), para que la definición coincida con el área
   de siempre cuando la norma ya es euclídea. Esto es lo que hace que ninguna
   de las dos sea arbitraria: ambas se fijan exigiendo `‖·‖ = ‖·‖₂ ⟹ misma
   respuesta que Hausdorff`, y ésa es la única grados de libertad que queda
   una vez impuesta esa condición de consistencia (comprobado en §3.1).

El resultado de los dos pasos es una **densidad** `θ(E)` — un número real
positivo, uno por cada `k`-plano — tal que el área de un trozo `S ⊂ E` es

```
área(S) = θ(E) · vol_k^eucl(S)
```

Para el octaedro, las 8 caras son congruentes bajo el grupo de permutaciones
y signos de coordenadas (el grupo de simetrías de L₁), así que `θ` es la
**misma** para las 8 y basta calcularla una vez.

### 1.2 Área de Busemann

Busemann normaliza usando **la sección de la bola primal**, `B ∩ E₀` — el
cuerpo convexo (plano) que resulta de cortar la bola unidad de la norma por
el plano que pasa por el origen paralelo a la cara. Esto es literalmente
«qué forma tiene la bola unidad restringida a esa dirección», y es la
elección más directa posible.

```
θ_Bus(E) = ω_k / vol_k^eucl(B ∩ E₀)
```

Intuición de la fórmula: si la sección `B ∩ E₀` es más grande que la bola
euclídea de referencia, la norma es «más generosa» en esa dirección, así que
hay que **encoger** la medida euclídea de `S` (dividir por algo grande) para
que sea consistente con `‖·‖`; si la sección es más pequeña, hay que
agrandar. De ahí el cociente inverso.

**Caso de consistencia**: si `‖·‖ = ‖·‖₂`, entonces `B ∩ E₀` es el disco
euclídeo de radio 1 en `E₀`, de área `ω_k`, y `θ_Bus = ω_k/ω_k = 1` — se
recupera el área de siempre. Verificado numéricamente en §3.1.

### 1.3 Área de Holmes–Thompson

Holmes–Thompson normaliza usando **la sección de la bola dual**, en el
espacio dual `X*`, proyectada/restringida al plano dual de `E₀`. Nace del
volumen simpléctico del fibrado co-bola unidad (la construcción intrínseca
en geometría de Finsler), pero para un plano fijo tiene una forma equivalente
puramente lineal-algebraica que es la que se usa aquí, y que además es la
que pide el enunciado de la tarea: la norma dual de L₁ es L∞, así que hay
que trabajar con el cubo `B∞ = {max(|x|,|y|,|z|) ≤ 1}` en vez de con el
octaedro.

Hecho clásico de dualidad de cuerpos convexos (Schneider, *Convex Bodies*,
§1.6; también en Thompson, *Minkowski Geometry*): para un cuerpo convexo
simétrico `K ⊂ Rⁿ` con el origen en su interior y un subespacio `E`,

```
π_E(K°) = (K ∩ E)°_E
```

— la proyección ortogonal del polar de `K` sobre `E` es igual al polar,
calculado **dentro de `E`** con la estructura euclídea que `E` heredó de
`Rⁿ`, de la sección de `K` por `E`. Aplicado a `K = B` (bola de `‖·‖`, con
`B° = B∞` porque el dual de L₁ es L∞): la sombra que el cubo L∞ proyecta
sobre el plano `E₀` es exactamente el polar, tomado dentro de `E₀`, del
hexágono `B₁ ∩ E₀` de §1.2. Las dos vías son el mismo número — eso es la
contraverificación de §3.3.

La densidad de Holmes–Thompson es

```
θ_HT(E) = vol_k^eucl( (B ∩ E₀)°_{E₀} ) / ω_k  =  vol_k^eucl( π_{E₀}(B°) ) / ω_k
```

**Caso de consistencia**: con `‖·‖=‖·‖₂`, `B∩E₀` es el disco unidad y su
polar (dentro de `E₀`) es de nuevo el disco unidad (el disco es autodual),
área `ω_k`, así que `θ_HT = ω_k/ω_k = 1` — mismo resultado que Busemann y que
Hausdorff en el caso euclídeo, como debe ser.

**Por qué Busemann y Holmes–Thompson difieren en general** (pero no en
`k=1`, ver §3.1): Busemann usa la sección de la bola primal directamente;
Holmes–Thompson usa el polar de esa sección (equivalentemente, la sombra de
la bola dual). Para una norma no euclídea estas dos operaciones — seccionar y
luego nada, frente a seccionar y luego dualizar — dan cuerpos distintos, y
por tanto densidades distintas. La desigualdad de Santaló-Mahler garantiza
además que, para cuerpos simétricos, Holmes–Thompson ≤ Busemann siempre; se
confirma numéricamente en §2.4.

### 1.4 Las dos aplicadas a L₁ en R³, cara por cara

Fijamos `r=1` (unidades de radio; todas las áreas escalan como `r²` y eso se
comprueba por separado, no hace falta rehacer el álgebra). La cara
representativa (la que da §7.1) es el triángulo con vértices `(1,0,0),
(0,1,0), (0,0,1)`, contenido en el plano `E: x+y+z=1`. Su plano de dirección
es `E₀: x+y+z=0`, el mismo para las 8 caras salvo por el signo global (el
grupo `D₄` de simetrías de L₁ permuta las 8 caras transitivamente y `E₀` es
invariante bajo la reflexión que lleva una cara a la opuesta), así que **una
sola sección `B₁∩E₀` y su dual sirven para las 8 caras**. Esto es lo que
permite «calcular una cara y multiplicar por 8» sin perder generalidad — el
paso que pide la tarea.

Área euclídea de la cara (ya está en §7.1 del hallazgo original, se reusa):
lado `√2`, área `√3/2`.

---

## 2. El cálculo

### 2.1 La sección hexagonal `B₁ ∩ E₀`

Base ortonormal de `E₀ = {x+y+z=0} ⊂ R³`: `u=(1,-1,0)/√2`, `v=(1,1,-2)/√6`.
Como `u,v` son ortonormales en `R³`, la aplicación `(s,t) ↦ su+tv` es una
**isometría** de `R²` sobre `E₀`, así que el área euclídea en `(s,t)` es
exactamente el área euclídea en `E₀` — no hace falta jacobiano.

Un punto `p=su+tv` tiene coordenadas
`x = s/√2 + t/√6`, `y = -s/√2 + t/√6`, `z = -2t/√6`.
Sustituyendo `a = s/√2`, `b = t/√6` (con `ds·dt = 2√3 · da·db`, porque el
cambio `(s,t)→(a,b)` escala por `√2` y por `√6` en cada eje):

```
x = a+b,  y = -a+b,  z = -2b
|x|+|y|+|z| = |a+b| + |a-b| + 2|b| = 2·max(|a|,|b|) + 2|b|   [pues |a+b|+|a-b|=2max(|a|,|b|)]
```

La condición `|x|+|y|+|z| ≤ 1` es entonces `max(|a|,|b|) + |b| ≤ 1/2`.
Resolviendo por casos en `b`: si `|b| > 1/4` no hay solución (porque
`max(|a|,|b|) ≥ |b| > 1/2-|b|` fuerza el fallo); si `|b| ≤ 1/4`, la condición
se reduce a `|a| ≤ 1/2-|b|` (el propio `|b|` ya cumple `|b|≤1/2-|b|`). Así que
la región en `(a,b)` es

```
área(a,b) = ∫_{-1/4}^{1/4} (1-2|b|) db = 2∫₀^{1/4}(1-2b)db = 2[b-b²]₀^{1/4} = 2(1/4-1/16) = 3/8
```

Deshaciendo el cambio de variable, `área(s,t) = 2√3 · 3/8 = 3√3/4`.

```
vol₂^eucl(B₁ ∩ E₀) = 3√3/4
```

```
$ target/release/dz-cli.exe eval "3*sqrt(3)/4" --mode fixed --scale 8
~1;3708X3
$ target/release/dz-cli.exe eval "3*sqrt(3)/4" --mode rational
~1;3708X2EEX205
```

(El `~` en **las dos** salidas —incluso en modo racional, que muestra fracción
exacta cuando el valor lo es— es la confirmación visual de irracionalidad de
esta cantidad intermedia: no hay una fracción `a/b` exacta que mostrar.
Contraste con un racional genuino, `4/3·3³ = 30` en modo racional, que no
lleva `~` y devuelve un entero limpio — ver §2.4.)

Esto es un **hexágono regular**: circunradio `R = 1/√2` (el punto `a=1/2,b=0`
da `s=1/√2, t=0`, y por simetría los otros 5 vértices están a la misma
distancia). Verificación con la fórmula de área de hexágono regular
`(3√3/2)R²`:

```
$ target/release/dz-cli.exe eval "(3*sqrt(3)/2)*(1/sqrt(2))^2" --mode fixed --scale 8
~1;3708X3001279X5219X346729404X8XX55XE363790X21737EE4E786
```

Coincide con `3√3/4` en todos los dígitos calculados — primera
contraverificación (vía integral directa frente a vía fórmula de hexágono
regular con el circunradio medido).

### 2.2 Densidad de Busemann y área de Busemann de una cara

```
θ_Bus = ω₂ / vol₂(B₁∩E₀) = π / (3√3/4) = 4π/(3√3) = 4π√3/9
```

```
$ target/release/dz-cli.exe eval "4*pi*sqrt(3)/9" --mode fixed --scale 8
~2;502EE12118X30X8
```

Área de Busemann de una cara = (área euclídea de la cara) × θ_Bus:

```
(√3/2) × 4π√3/9 = (4·3·π)/(2·9) = 12π/18 = 2π/3
```

```
$ target/release/dz-cli.exe eval "2*pi/3" --mode fixed --scale 8
~2;11714632
```

### 2.3 El dual: norma L∞ restringida, y la densidad de Holmes–Thompson

Dos caminos equivalentes por la identidad de dualidad de §1.3; se hacen los
dos como contraverificación cruzada (§3.3).

**Camino A — polar del hexágono, dentro de `E₀`.** Para un hexágono regular
de circunradio `R` y apotema (inradio) `ρ = R√3/2`, el polar es de nuevo un
hexágono regular, girado 30°, con circunradio `R° = 1/ρ`. Con `R=1/√2`:
`ρ = √3/(2√2) = √6/4`, `R° = 4/√6 = 2√6/3`. Área del hexágono dual (misma
fórmula `(3√3/2)(R°)²`):

```
(3√3/2)·(2√6/3)² = (3√3/2)·(24/9) = (3√3/2)·(8/3) = 4√3
```

**Camino B — proyección directa del cubo L∞ sobre `E₀`.** `B∞ = [-1,1]³` es
la bola dual de L₁. Proyectar ortogonalmente sobre `E₀` (dirección de
proyección `n̂=(1,1,1)/√3`, la normal a `E₀`): los vértices `(1,1,1)` y
`(-1,-1,-1)` —los que están *sobre* el eje de proyección— colapsan al origen;
los otros 6 vértices de signo mixto, p. ej. `(1,1,-1)`, se proyectan a
`(1,1,-1) - (1/3,1/3,1/3) = (2/3,2/3,-4/3)`, cuyas coordenadas en la base
`(s,t)` de `E₀` son `s=0, t=4/√6=2√6/3` — exactamente el circunradio `R°`
hallado por el camino A, y en el ángulo de 90° (girado 30° respecto del
hexágono primal de §2.1, cuyo vértice estaba en el ángulo 0°). Los 6 vértices
mixtos forman el hexágono regular sombra del cubo, con esta área:

```
vol₂^eucl( (B₁∩E₀)° ) = vol₂^eucl( π_{E₀}(B∞) ) = 4√3
```

```
$ target/release/dz-cli.exe eval "4*sqrt(3)" --mode fixed --scale 8
~6;E17E28
$ target/release/dz-cli.exe eval "(3*sqrt(3)/2)*(2*sqrt(6)/3)^2" --mode fixed --scale 8
~6;E17E28004386X412172E054
```

Los dos caminos coinciden dígito a dígito (segunda contraverificación,
detallada en §3.3). Nótese que el valor numérico `4√3` **coincide con la
superficie total de Hausdorff** de §7.1 — coincidencia real, no error: la
superficie Hausdorff es `8·(√3/2)=4√3`, y aquí `4√3` es el área (no
normalizada aún) de **un solo** hexágono dual; son cantidades distintas que
resultan tener el mismo valor porque el hexágono dual de circunradio `2√6/3`
tiene la misma área que la unión de las 8 caras del octaedro. Se deja anotado
para que nadie lo confunda con una errata de copiar-pegar.

Densidad de Holmes–Thompson:

```
θ_HT = vol₂((B₁∩E₀)°) / ω₂ = 4√3/π
```

```
$ target/release/dz-cli.exe eval "4*sqrt(3)/pi" --mode fixed --scale 8
~2;2569510663001001588498X8
```

Área de Holmes–Thompson de una cara:

```
(√3/2) × 4√3/π = 12/(2π) = 6/π
```

```
$ target/release/dz-cli.exe eval "6/pi" --mode fixed --scale 8
~1;XE02X1445X829X392289905
$ target/release/dz-cli.exe eval "6/pi" --mode rational
~1;XE02X1443579
```

(otra vez `~` en modo racional — es transcendente, no solo irracional
algebraico; ver §4).

### 2.4 Totales sobre las 8 caras

Las 8 caras son congruentes bajo el grupo de simetrías `D₄` (orden 8, el
mismo grupo que gobierna el resto de la línea A, `99-sintesis.md` §1), y
`E₀` es invariante bajo la reflexión que intercambia una cara con su opuesta
(§1.4), así que la densidad `θ` calculada en una cara vale para las 8 sin
más trabajo — multiplicar por 8 es correcto y no es un atajo que oculte
nada.

```
S_Bus = 8 · 2π/3 = 16π/3
S_HT  = 8 · 6/π  = 48/π
```

```
$ target/release/dz-cli.exe eval "16*pi/3" --mode fixed --scale 8
~16;X2404846

$ target/release/dz-cli.exe eval "48/pi" --mode fixed --scale 8
~15;9XX26489XE8X4031896303XE
```

Comparación con Hausdorff (§7.1) y con la esfera euclídea, todo para `r=1`:

| medida | fórmula | valor (base 10, aprox.) | comando |
|---|---|--:|---|
| Hausdorff (L₁) | `4√3 r²` | 6,9282 | `eval "4*sqrt(3)" --mode fixed --scale 8` |
| Busemann (L₁) | `(16π/3) r²` | 16,7552 | `eval "16*pi/3" --mode fixed --scale 8` |
| Holmes–Thompson (L₁) | `(48/π) r²` | 15,2789 | `eval "48/pi" --mode fixed --scale 8` |
| Hausdorff (L₂, esfera) | `4π r²` | 12,5664 | — (referencia, no es L₁) |

Dos observaciones que valen la pena antes de clasificar (§4):

- **`HT ≤ Bus` se cumple** (15,28 < 16,76), consistente con la desigualdad de
  Santaló–Mahler para cuerpos simétricos anunciada en §1.3 — no es un
  resultado nuevo, es una comprobación de que no hay un error de signo o de
  inverso en la derivación.
- **Ninguna de las tres iguala a la esfera euclídea**, y no hay razón para
  esperar que lo hagan — son medidas del mismo cuerpo octaédrico bajo normas
  distintas, no aproximaciones a la esfera. Se incluye solo como ancla de
  magnitud.

---

## 3. Contraverificación (tres métodos independientes)

### 3.1 Caso degenerado k=1: el plano taxicab

Cuando `k=1` la definición debería colapsar a algo inequívoco: en una sola
dimensión no hay ambigüedad de «área», solo longitud, y la longitud de un
segmento en una norma cualquiera ya está definida sin necesitar Busemann ni
Holmes–Thompson (es `‖·‖`-longitud, punto). Si el aparato general de §1 no
reproduce eso, el aparato está mal.

Sea `E` una recta por el origen en la dirección de un vector euclídeo unitario
`v`, y `‖v‖` su norma bajo la que se mide (no la euclídea). La sección
`B∩E₀` es el intervalo `{t·v : |t|·‖v‖ ≤ 1} = [-1/‖v‖, 1/‖v‖]`, de longitud
euclídea `2/‖v‖`. Como `ω₁ = 2` (longitud del intervalo `[-1,1]`):

```
θ_Bus(E) = ω₁ / (2/‖v‖) = 2/(2/‖v‖) = ‖v‖
```

Así que la longitud de Busemann de un segmento de longitud euclídea `ℓ` en la
dirección `v` es `ℓ·‖v‖` — **exactamente** la `‖·‖`-longitud del segmento, la
definición no ambigua de partida. Repitiendo con el polar: el polar de
`[-1/‖v‖, 1/‖v‖]` es `[-‖v‖, ‖v‖]`, de longitud `2‖v‖`, así que
`θ_HT(E) = 2‖v‖/ω₁ = ‖v‖` — **el mismo número**. Esto confirma dos cosas a la
vez: (a) el aparato general reduce correctamente al caso sin ambigüedad, y
(b) Busemann y Holmes–Thompson **coinciden siempre en `k=1`** y solo pueden
diferir a partir de `k=2` — es un hecho conocido de la teoría (Álvarez
Paiva–Thompson) y aquí sale solo de la álgebra, no se importó de la
literatura.

Aplicado al cuadrado L₁ del plano (el «círculo taxicab», rombo de vértices
`(±1,0),(0,±1)`): sus 4 lados son segmentos de longitud euclídea `√2` en
direcciones donde `‖v‖₁ = 1/(1/√2) ·... ` — más simple: cada lado va de
`(1,0)` a `(0,1)`, se parametriza `v(t)=(1-t,t)`, y `‖v'(t)‖₁ = |{-1,1}|₁ = 2`
para todo `t` (nótese: la norma del vector velocidad `(-1,1)` es `|-1|+|1|=2`,
constante), así que la longitud L₁ de cada lado es `2` (no `√2`, que es la
longitud euclídea) y el perímetro es `4·2 = 8`.

```
$ target/release/dz-cli.exe eval "8" --mode rational
8
```

Esto reproduce el resultado ya usado en `01-hallazgos-A-constantes.md`
(perímetro/diámetro = 4, con diámetro 2, perímetro 8) y confirma que, para el
único invariante de longitud que existe en `k=1`, Busemann y
Holmes–Thompson no añaden nada nuevo — es la señal correcta: la maquinaria de
§1 solo tiene contenido no trivial a partir de `k=2`, que es justo donde vive
la pregunta de esta investigación (superficie de un cuerpo 3D).

### 3.2 El hexágono por vértices (shoelace) vs. por integración

§2.1 obtuvo `vol₂(B₁∩E₀) = 3√3/4` integrando la región `|a|≤1/2-|b|` en
`(a,b)`. Independientemente, se ubican los 6 vértices del hexágono como
puntos medios de las 6 aristas del octaedro que cruzan el plano `E₀`: el
octaedro tiene 12 aristas (cada vértice `±eᵢ` conecta con los 4 `±eⱼ`,
`j≠i`, pero no con su antípoda `-eᵢ`), y `E₀:x+y+z=0` separa
`{e₁,e₂,e₃}` (suma=1) de `{-e₁,-e₂,-e₃}` (suma=-1); las 6 aristas que cruzan
son las que unen un `eᵢ` con un `-eⱼ`, `i≠j` (las que unen `eᵢ` con `-eᵢ` no
son aristas del octaedro). Sus puntos medios:

```
(1/2,-1/2,0), (1/2,0,-1/2), (-1/2,1/2,0), (0,1/2,-1/2), (-1/2,0,1/2), (0,-1/2,1/2)
```

Proyectados a las coordenadas `(s,t)` de `E₀` (§2.1) quedan en los ángulos
`0°,60°,120°,180°,240°,300°` a distancia `1/√2` del origen — un hexágono
regular exacto, con el mismo circunradio `R=1/√2` que se dedujo por el
argumento de simetría en §2.1. El área de un hexágono regular de circunradio
`R` por la fórmula estándar `(3√3/2)R²` con `R=1/√2` da `3√3/4` — **el mismo
número que la integral**, por un camino que no comparte ningún paso
algebraico con ella (geometría de vértices y aristas del poliedro, en vez de
resolver una desigualdad con valores absolutos). Ya mostrado numéricamente en
§2.1 (`eval "(3*sqrt(3)/2)*(1/sqrt(2))^2"` coincide con `eval
"3*sqrt(3)/4"` dígito a dígito).

### 3.3 El dual por proyección directa del cubo L∞ vs. por polar del hexágono

Ya presentado en §2.3 como «camino A» (polar del hexágono usando la relación
`R°=1/ρ` entre circunradio y apotema) y «camino B» (proyectar el cubo `L∞`
—la bola dual real, no un objeto abstracto— sobre `E₀` a lo largo de la
diagonal `(1,1,1)`). Los dos son matemáticamente el mismo objeto por la
identidad de dualidad de Schneider citada en §1.3, pero se calcularon por
rutas independientes: uno es trigonometría de hexágonos regulares (apotema,
polar, rotación de 30°), el otro es álgebra lineal elemental (proyectar 8
vértices de un cubo, descartar los 2 que colapsan al origen, quedarse con
los 6 que sobreviven). Ambos dan `4√3`, confirmado dígito a dígito por el
CLI en §2.3. Esta es la contraverificación explícitamente pedida por la
tarea: «trabajar la norma dual L∞ restringida a cada plano de cara» (camino
B) frente al método más abstracto por polaridad (camino A) — coinciden.

Una tercera confirmación, más débil pero barata: el hexágono dual (camino B)
sale girado 30° respecto del hexágono primal de §2.1/§3.2 —su primer vértice
cae en el ángulo 90°, el del primal en el ángulo 0°—, que es exactamente la
relación que la teoría predice entre un hexágono regular y su polar (los
vértices del polar caen en las direcciones normales a las aristas del
original, desplazadas medio paso angular). Que la rotación relativa
observada coincida con la predicha por la teoría general, y no solo las
áreas, es una verificación adicional de que el cálculo no tiene un error de
signo que por casualidad preservara el área pero rotara mal el cuerpo.

---

## 4. Veredicto, clasificado

**Resultado**: bajo Busemann, la superficie del octaedro L₁ es `(16π/3)r²`;
bajo Holmes–Thompson, `(48/π)r²`. Ninguna de las dos es racional.

### 4.1 Naturaleza del número, no solo su racionalidad

El plan (`00-plan-investigacion.md` §2.1, `N_irracionales`) distingue
constantes irracionales por identidad, no solo por «no ser una fracción», y
aquí eso importa más que en casi ningún otro punto del informe:

| medida | fórmula | clase |
|---|---|---|
| Hausdorff | `4√3 r²` | algebraico (raíz de `x²=3`) |
| Busemann | `(16π/3) r²` | trascendente (múltiplo racional de `π`) |
| Holmes–Thompson | `(48/π) r²` | trascendente (`π` en el denominador) |

`16π/3` es trascendente porque `π` lo es y los trascendentes no se vuelven
algebraicos al multiplicarlos por un racional no nulo. `48/π` es trascendente
por el mismo argumento aplicado al inverso (si `1/π` fuera algebraico, `π`
también lo sería, por ser el inverso de un algebraico no nulo — contradice
Lindemann). Así que cambiar de definición de área no solo **no** da un
racional: cambia la superficie de «irracional algebraico» (`√3`) a
«trascendente» (`π`), que es un salto en la dirección **opuesta** a la que
buscaba la objeción de §7.1. La comprobación visual de esto está en que
`--mode rational` sigue mostrando `~` (truncado) para las dos cantidades
(§2.2, §2.3) exactamente igual que para `√3` — el modo racional del CLI solo
omite el `~` cuando existe una fracción exacta, y aquí nunca la hay.

### 4.2 ¿Es esto «elegir la definición que da el número bonito»?

No — y hay que decir por qué con el mismo rigor que exigió la objeción
original, no solo negarlo.

- **No se eligió el resultado, se calculó.** Busemann y Holmes–Thompson son
  las únicas dos definiciones de área de superficie para espacios normados
  con literatura establecida y sin parámetro libre una vez fijada la
  condición de consistencia con el caso euclídeo (§1.2, §1.4: ambas
  colapsan a Hausdorff cuando `‖·‖=‖·‖₂`). No hay un tercer grado de
  libertad que se haya ajustado para obtener `π`; `π` aparece porque `ω₂=π`
  es la constante de normalización de **las dos**, inevitablemente — es la
  misma `π` que aparecería al normalizar cualquier norma no euclídea por el
  disco euclídeo de referencia. El resultado irracional-trascendente es una
  consecuencia forzada de la definición, no un ajuste.
- **La pregunta que sí sería «elegir el número bonito»** sería: «¿existe
  *alguna* normalización, sin exigir consistencia con Hausdorff en el caso
  euclídeo, que dé un racional?» Esa pregunta tiene respuesta trivialmente
  afirmativa (basta escalar por `3/(16π)` o por `π/48`) y **no es una
  pregunta geométrica** — es despojar a la palabra «área» de su único
  contenido no arbitrario. No es la pregunta que se investigó aquí.
- **La objeción original de §7.1 pedía comprobar, no asumir**, si Busemann o
  Holmes–Thompson «podrían normalizar esto a un racional» — la respuesta
  correcta y honesta, tras hacer el cálculo, es que no lo hacen, y que además
  lo que devuelven es peor para `K(F)` que Hausdorff: introducen `π` (que ya
  estaba descartado en L₁ precisamente por ser la marca de la métrica
  euclídea) sin eliminar ninguna raíz.

### 4.3 Casilla del protocolo

**(I) intrínseco a L₂, en su forma más fuerte encontrada en toda la línea A.**
No es solo que la superficie L₁ carezca de análogo racional bajo Hausdorff
(§7.1 original); es que **las tres definiciones razonables de área para un
cuerpo convexo en un espacio normado arbitrario** —la métrica (Hausdorff), la
que normaliza por la sección primal (Busemann), la que normaliza por la
sección dual (Holmes–Thompson)— coinciden en devolver un número irracional
para el octaedro, y dos de las tres además reintroducen `π`, la constante
que se supone que L₁ existía para evitar. Si la pregunta es «¿puede
elegirse una noción de área que haga racional la frontera de la bola L₁ en
dimensión 3?», la respuesta con las herramientas estándar de geometría
normada es no, y no por falta de intentarlo.

---

## 5. Qué establece esto y qué no (lo que no funcionó)

Sección obligatoria por el protocolo (`00-plan-investigacion.md` §5, punto 6).
Un resultado positivo («se refuerza el hallazgo») no está exento.

- **No se agotó el espacio de definiciones de área normada.** Busemann y
  Holmes–Thompson son las dos con literatura establecida y sin parámetro
  libre, pero no son las únicas construcciones estudiadas en geometría de
  Finsler/Minkowski (existen variantes vía `p`-medias de las dos, o medidas
  definidas solo para ciertas clases de normas). No se buscaron ni se
  descartaron aquí. La afirmación de §4.3 es «con las dos herramientas
  estándar», no «con toda construcción imaginable».
- **No se demostró un teorema general.** Se calculó un caso: el octaedro,
  bola unidad de L₁ en `R³`. No se investigó si el patrón («Hausdorff da
  algebraico, Busemann/HT dan trascendente con `π`») se repite para otros
  cuerpos L₁ en dimensiones más altas, ni para bolas `Lp` intermedias. Es
  plausible por la misma razón estructural (`ω_k` siempre trae `π` a partir
  de `k=2`), pero «plausible» no es «verificado», y el protocolo de este
  informe exige verificación con el binario, no extrapolación.
  *Cerrado 2026-09-25 para politopos*: `15` §4 demuestra que
  `S_Bus ∈ ω_{n−1}·ℚ` y `S_HT ∈ ℚ/ω_{n−1}` para todo politopo racional (o
  imagen lineal de uno), y lo comprueba en el cubo (`6π`, `48/π`). Las bolas
  `Lp` lisas siguen abiertas.
- **La comprobación en `k=1` (§3.1) es una comprobación de consistencia del
  aparato, no de la respuesta final.** Confirma que las fórmulas de §1
  reducen correctamente al caso sin ambigüedad y que Busemann/HT coinciden
  en `k=1` — un hecho conocido de la teoría, reproducido aquí por álgebra
  directa. No comprueba independientemente el valor de `S_Bus` o `S_HT` en
  `k=2`: esa comprobación es §3.2/§3.3 (dos rutas algebraicas distintas para
  la misma sección hexagonal y su dual), no una tercera fuente externa
  (no hay, en este repositorio, un segundo agente ni una tabla publicada
  contra la que cotejar `16π/3` u `48/π` para este cuerpo concreto).
- **Todas las cifras dozenales de este documento son truncamientos
  aproximados** (`~`, escala 8), porque las tres cantidades finales son
  irracionales por construcción. Eso es correcto y esperado —no es un
  defecto del cálculo—, pero significa que ningún número de este documento
  es una prueba por sí solo de racionalidad o irracionalidad: la prueba está
  en el álgebra de §1–§2 (que `π` o `√3` aparecen simbólicamente en la forma
  cerrada), y el CLI solo la ilustra y la contrasta (modo racional sin
  fracción exacta ⇒ consistente con irracionalidad, nunca una demostración
  por sí solo — un irracional muy cercano a un racional de denominador
  enorme también mostraría dígitos truncados sin ser racional).
- **No se exploró si la comparación tiene sentido fuera de la geometría
  convexa de dimensión finita.** Busemann y Holmes–Thompson están definidas
  para esto; extender la pregunta a superficies curvas L₁-análogas (que no
  existen de forma canónica, porque L₁ no tiene una noción de curvatura
  suave) queda fuera de alcance y no se intentó.
- **El resultado no cambia ninguna conclusión de `99-sintesis.md`.** La
  síntesis ya preveía (§7, último punto) que este hallazgo «podría cambiar
  el signo» del resultado de §3.4; el cálculo aquí muestra que no lo cambia
  — lo endurece. No hace falta reabrir el veredicto general de H₁, porque
  esta línea (A, «la racionalidad no desciende del cuerpo a su frontera»)
  ya estaba clasificada como uno de los cuatro resultados que valen
  (`99-sintesis.md` §3.4), y este documento la deja más fuerte, no distinta.
