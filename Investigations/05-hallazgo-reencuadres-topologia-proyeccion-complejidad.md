# Hallazgo — Tres reencuadres no métricos: topología, proyección discreta y complejidad descriptiva

Extiende la línea A (`01-hallazgos-A-constantes.md` §7.1) y `03-hallazgo-busemann-holmes-thompson.md`.
Esos dos documentos agotaron el paradigma de «superficie = número real dado por
una medida continua» para el octaedro L₁: Hausdorff da `4√3 r²` (algebraico),
Busemann da `(16π/3) r²` y Holmes–Thompson `(48/π) r²` (ambos trascendentes).
Los tres son variantes del mismo tipo de pregunta — «¿qué número mide el
borde?» — y los tres pierden.

Este documento no calcula una cuarta medida de área sobre el mismo cuerpo.
Cambia la pregunta tres veces:

1. **Topológica** — ¿el `π` que aparece es forzado por la característica de
   Euler, no por la elección de medida?
2. **Proyectiva/discreta** — ¿qué pasa si «tamaño del borde» se define como
   promedio de sombras sobre un conjunto *finito* de direcciones, no una
   integral sobre la esfera continua?
3. **Descriptiva** — ¿qué pasa si «tamaño» no es un número real en absoluto,
   sino la complejidad de la descripción exacta mínima, en el mismo espíritu
   que `K(F)` ya usa para fórmulas?

Fecha de apertura: 2026-09-12. Protocolo de `00-plan-investigacion.md`: toda
cifra dozenal sale de `target/release/dz-cli.exe` con el comando al lado;
todo hallazgo se clasifica en casilla (I)/(II)/(III); ningún resultado se
elige por dar el número bonito.

**Recordatorio no negociable, para que quede escrito una vez y no se repita
en cada sección**: π es trascendente (Lindemann–Weierstrass), independiente
de la base numeral. Nada de lo que sigue intenta ni sugiere que π sea
racional. Lo que se investiga es si un modelo conceptual *distinto* de
«tamaño de frontera» evita estructuralmente importar la trascendencia de π
—porque mide otra cosa— o si, al mirar con cuidado, la vuelve a importar por
otra puerta.

---

## Reencuadre 1 — ¿Es π forzado por la topología (Euler), no por la métrica?

### 1.1 El teorema de Descartes, sin elegir nada

Para cualquier poliedro convexo, la suma de los **defectos angulares** en
todos los vértices es `4π`. Esto es el teorema del ángulo total de Descartes,
equivalente a Gauss–Bonnet discreto (`2πχ`, con `χ=2` para cualquier
poliedro topológicamente esférico). No depende de la forma del cuerpo: no
sabe si el poliedro es «redondo» o no. Es un enunciado sobre la
**combinatoria angular**, no sobre ninguna medida de área que se haya
elegido.

### 1.2 El octaedro L₁, vértice por vértice

El octaedro `‖x‖₁ ≤ r` tiene 6 vértices `(±r,0,0),(0,±r,0),(0,0,±r)`. En cada
vértice concurren exactamente 4 caras (de las 8 caras triangulares
equiláteras totales), y las 4 son equiláteras: cada ángulo interior de un
triángulo equilátero es `π/3`.

**Suma de ángulos en un vértice**: `4 · π/3 = 4π/3`.

```
$ target/release/dz-cli.exe eval "4*pi/3" --mode fixed --scale 8
~4;23229064
```

**Defecto angular** (`2π` menos la suma de ángulos, el «hueco» que habría
que cerrar para aplanar el vértice):

```
$ target/release/dz-cli.exe eval "2*pi - 4*pi/3" --mode fixed --scale 8
~2;11714632
```

es decir `2π/3` (verificado simbólicamente: `2π − 4π/3 = 6π/3 − 4π/3 = 2π/3`,
y la cifra coincide con `eval "2*pi/3"` dígito a dígito, ver §1.4).

**Suma sobre los 6 vértices**:

```
$ target/release/dz-cli.exe eval "6*(2*pi/3)" --mode fixed --scale 8
~10;6968317
$ target/release/dz-cli.exe eval "4*pi" --mode fixed --scale 8
~10;6968317
```

Coinciden dígito a dígito: el teorema de Descartes se verifica exactamente
para este cuerpo concreto, calculando (no asumiendo) el defecto vértice por
vértice.

### 1.3 La pregunta central: ¿es π inevitable en *cualquier* curvatura de un poliedro convexo?

**Sí, para toda curvatura angular.** La razón estructural es que el defecto
angular en cada vértice es `2π` menos una suma de ángulos euclidianos
—y `2π` es la medida de ángulo total en el *plano tangente local*, que es
euclidiano seas cual sea la norma del espacio ambiente. Da igual con qué
métrica se mida el cuerpo entero (L₁, L₂, la que sea): la noción de «ángulo
interior de una cara» y la de «vuelta completa alrededor de un punto» son
nociones locales de la geometría euclidiana del plano tangente, y el
`2π` de la vuelta completa es una constante de la topología del disco
(o, equivalentemente, del grupo `SO(2)` que actúa en él), no de la norma
que se imponga sobre el espacio ambiente en el que el poliedro vive. El
octaedro L₁ y una esfera pinchada en 6 puntos comparten el mismo `4π`
exacto porque **ambos son topológicamente `S²`**, y el teorema de
Gauss–Bonnet dice `∫K dA = 2πχ` en el caso liso, `Σ defecto = 2πχ` en el
discreto — la constante es `2π` multiplicada por un entero (`χ`), en las
dos versiones, sea cual sea la geometría interna.

**Conclusión parcial, y hay que decirla con precisión**: esto no explica por
qué Hausdorff/Busemann/Holmes–Thompson fallaron en dar un racional para la
*superficie* del octaedro — son magnitudes distintas (superficie ≠ curvatura
total) y no hay ningún teorema que fuerce a la superficie a heredar el `π`
de la curvatura total. Lo que sí explica es algo más general y quizás más
importante: **cualquier invariante que se defina como «suma de ángulos» o
«integral de una curvatura basada en ángulos» sobre un poliedro convexo
lleva `π` multiplicado por un entero topológico, siempre, sin excepción,
porque `π` está codificado en la definición misma de ángulo (la vuelta
completa), no en la forma del cuerpo.** El defecto angular no es una medida
que uno *elige* entre varias — es *la* cantidad que Gauss–Bonnet liga a la
topología, y por construcción trae `2π` consigo.

### 1.4 ¿Hay una curvatura discreta que evite los ángulos por completo?

Sí, y es donde este reencuadre aporta algo genuinamente nuevo: la
**curvatura combinatoria** (Higuchi, en el contexto de grafos planos; el
análogo discreto que no usa trigonometría). Se define, para cada vértice
`v` de un poliedro con grado `deg(v)` (número de aristas/caras que
concurren) y caras `f` de `deg(v)` lados incidentes en `v`:

```
κ_comb(v) = 1 − deg(v)/2 + Σ_{f ∋ v} 1/deg(f)
```

Ningún ángulo, ningún seno ni coseno: solo **conteos** — cuántas aristas,
cuántos lados tiene cada cara. Es el análogo de Gauss–Bonnet construido
exclusivamente con la combinatoria `(V,E,F)` del poliedro, no con su
geometría métrica.

**Para el octaedro**: cada vértice tiene `deg(v)=4` (4 aristas, 4 caras
concurrentes), y las 4 caras son triángulos, `deg(f)=3` para las 4:

```
κ_comb(v) = 1 − 4/2 + 4·(1/3) = 1 − 2 + 4/3 = 1/3
```

```
$ target/release/dz-cli.exe eval "1/3" --mode fixed --scale 8
0;4
$ target/release/dz-cli.exe eval "1/3" --mode rational
1/3
```

Sin `~`, sin truncamiento: `1/3` termina en base 12 (`0;4`, un solo dígito),
justo el caso favorable de `12 = 2²·3` que domina la línea D. Sumando sobre
los 6 vértices:

```
$ target/release/dz-cli.exe eval "6*(1/3)" --mode rational
2
```

**`Σ κ_comb(v) = 2 = χ`** — el teorema combinatorio de Gauss–Bonnet
(`Σκ_comb = V − E + F`, que para el octaedro es `6 − 12 + 8 = 2`) se verifica
exactamente, con una fórmula que nunca menciona un ángulo, y el resultado es
**racional puro** (de hecho entero), sin rastro de `π`.

Coste en puertas, con la misma escala, para que el contraste quede medido y
no solo argumentado:

```
$ target/release/dz-cli.exe eval "2*pi/3" --mode fixed --scale 8 --gates
gates: ... total=1861683 depth=16106 recon=0
$ target/release/dz-cli.exe eval "1/3"    --mode fixed --scale 8 --gates
gates: ... total=288889  depth=1312  recon=0
```

El defecto angular de un solo vértice cuesta **6,4× más puertas y 12×
más profundidad** que la curvatura combinatoria del mismo vértice — y eso
sin contar que `pi` está facturando 0 puertas por la memoización deliberada
del backend (`CLAUDE.md`, `dz-sci`; el sesgo ya está documentado en
`01-hallazgos-A-constantes.md` §0.4 y no se repite el error de leerlo como
ventaja real). El total sobre los 6 vértices (`4*pi` frente a `6*(1/3)`)
sale además engañosamente barato para la versión angular por la misma
razón — **se declara, no se explota**: la comparación de coste no es la
prueba de este reencuadre, la prueba está en la fórmula cerrada.

### 1.5 Pero: ¿esta curvatura discreta lleva información genuina de «tamaño de frontera», o es un invariante más débil?

Hay que responder esto con la misma honestidad con que `03-hallazgo-...`
respondió a su propia objeción. **`κ_comb` es un invariante topológico
puro, no geométrico.** Depende únicamente de `(V,E,F)` y de qué caras
concurren en qué vértices — la combinatoria del poliedro, no sus longitudes
ni sus ángulos reales. Consecuencia dura: **cualquier poliedro
combinatoriamente equivalente al octaedro (mismo grafo: 6 vértices de grado
4, 8 caras triangulares) tiene la MISMA `κ_comb` en cada vértice, aunque se
lo deforme afínmente hasta volverlo casi plano en un vértice y casi un pico
en otro.** Un octaedro muy alargado (`(±a,0,0),(0,±a,0),(0,0,±b)` con
`b ≫ a`) tiene ángulos interiores de cara completamente distintos del
regular, así que su defecto angular real *varía* de vértice a vértice (los
defectos se redistribuyen, aunque su **suma** sigue dando `4π` por Descartes
— eso es universal), mientras que `κ_comb(v) = 1/3` en los 6 vértices **sin
cambiar**, porque no ha cambiado el grafo. `κ_comb` no distingue el
octaedro regular de ningún estiramiento afín suyo, ni siquiera de un
octaedro con caras escalenas arbitrarias, mientras conserve la misma
triangulación.

**Verdicto honesto de este reencuadre**: el defecto angular (basado en
ángulos) es genuinamente forzado a llevar `π` por la topología —y esto **sí**
es una explicación estructural más profunda que «Hausdorff/Busemann/HT
resultaron ser malas elecciones»: no son malas elecciones, es que *cualquier*
invariante de curvatura que use ángulos hereda `2πχ` por Gauss–Bonnet, así
que buscar una cuarta definición de «curvatura angular» que evite `π` sería
buscar una excepción a un teorema, no un descubrimiento. Pero el sustituto
combinatorio que sí evita `π` (`κ_comb`) lo hace **pagando exactamente la
geometría que se le pedía medir**: es ciego a la forma real del cuerpo,
solo ve su grafo de incidencias. No es «el mismo tamaño de frontera medido
de otra forma» — es una pregunta distinta y más pobre («¿cuál es la
combinatoria de la triangulación?»), que coincide con el octaedro genérico
de su clase combinatoria, no con el octaedro L₁ específicamente definido por
`|x|+|y|+|z|≤r`. Esto es (I): la aparición de π aquí es intrínseca a medir
ángulos en absoluto, sea la norma la que sea — no es un artefacto de L₂ y
tampoco lo evita ninguna elección de norma. El camino combinatorio evita π
genuinamente, pero al precio de dejar de medir tamaño.

**¿Cambia esto la lectura de las tres medidas anteriores (Hausdorff,
Busemann, HT)?** Explica *por qué* dos de las tres (Busemann, HT) traen `π`:
ambas se normalizan dividiendo por `ω_k`, el volumen de la bola euclidiana de
referencia, que **también** hereda `π` de la vuelta completa del círculo — es
el mismo mecanismo estructural que el defecto angular, aplicado a
volúmenes en vez de a ángulos. No explica por qué Hausdorff evita `π` pero
introduce `√3` (Hausdorff no normaliza contra ninguna bola de referencia, así
que no tiene por qué heredar `2π`; hereda el `√3` de la altura de un
triángulo equilátero, que es puramente algebraico y no topológico). Así
que el reencuadre topológico separa con precisión **cuáles** de las tres
medidas anteriores estaban condenadas a `π` por una razón estructural
(Busemann, HT — normalizan contra `ω_k`) y cuál no lo estaba en absoluto por
esa razón (Hausdorff — su `√3` viene de trigonometría plana, no de
topología).

---

## Reencuadre 2 — Fórmula de proyección de Cauchy y promedios direccionales finitos

### 2.1 El teorema, y por qué la versión continua no ayuda

Fórmula de Cauchy: la superficie de un cuerpo convexo `K ⊂ R³` es
`4` veces el área media de sus sombras (proyecciones ortogonales) sobre
todas las direcciones de la esfera `S²`, integrando con la medida
uniforme. Es un **teorema**, no una definición alternativa: para cualquier
cuerpo convexo con superficie de Hausdorff bien definida, el promedio
continuo reproduce exactamente `S/4`. Aplicado al octaedro
(`S = 4√3 r²`, §7.1 de `01-hallazgos-A-constantes.md`):

```
promedio continuo de sombras = S/4 = √3 r²
```

```
$ target/release/dz-cli.exe eval "4*sqrt(3)/4" --mode fixed --scale 8
~1;894E98
```

Esto **no es un hallazgo nuevo**: es la misma cantidad irracional-algebraica
de siempre, vista por otra puerta. Si el promedio se toma sobre la esfera
continua, Cauchy garantiza que no se puede escapar del resultado ya
conocido. El punto interesante de este reencuadre no está en la versión
continua, sino en preguntarse si un promedio sobre un **conjunto finito** de
direcciones —en vez de integrar sobre todas— es una noción legítima y
distinta de «tamaño».

### 2.2 Las sombras del octaedro sobre tres familias finitas de direcciones

El octaedro tiene tres familias de direcciones privilegiadas ya presentes en
su propia estructura, sin importar nada de fuera:

- **6 direcciones-eje** `±e₁,±e₂,±e₃` — las normales de las 6 caras del cubo
  circunscrito, y también las direcciones de los 6 vértices del propio
  octaedro.
- **8 direcciones-diagonal** `(±1,±1,±1)/√3` — las normales de las 8 caras
  del octaedro, y también los 8 vértices del cubo circunscrito.
- **12 direcciones-arista** `(±1,±1,0)/√2` (y permutaciones) — los puntos
  medios de las 12 aristas del octaedro, normalizados; también los 12
  vértices del cuboctaedro.

Las tres son órbitas del grupo de simetría del octaedro (orden 48, `O_h`),
así que dentro de cada familia todas las direcciones son equivalentes por
simetría y basta calcular una sombra por familia.

**(a) Dirección-eje** `u = e₁`. La sombra es la proyección de
`{|x|+|y|+|z|≤r}` sobre el plano `(y,z)`: para cualquier `(y,z)` con
`|y|+|z|≤r` basta `x=0` para estar en el cuerpo, y no se puede exceder esa
región. La sombra es exactamente el disco L₁ 2D, área `2r²` —el mismo
número que `A₁` en `01-hallazgos-A-constantes.md` §2.1—:

```
$ target/release/dz-cli.exe eval "2" --mode rational
2
```

**(b) Dirección-diagonal** `u = (1,1,1)/√3`. Proyectando los 6 vértices
`±eᵢ` sobre el plano `E₀: x+y+z=0` (cálculo directo, sin pasar por dualidad):
`proj(e₁) = (2/3,−1/3,−1/3)`, y análogamente para los otros 5, con
`|proj(eᵢ)|² = 2/3` para los 6 — hexágono regular de circunradio `√(2/3)`.
Área de hexágono regular `(3√3/2)R²` con `R²=2/3`:

```
área = (3√3/2)(2/3) = √3
```

```
$ target/release/dz-cli.exe eval "sqrt(3)" --mode fixed --scale 8
~1;894E98
```

(coincide dígito a dígito con `S/4` de §2.1 — ver §2.4 sobre si esto es
casualidad).

**(c) Dirección-arista** `u = (1,1,0)/√2`. Con base ortonormal del plano
perpendicular `w₁=(1,−1,0)/√2, w₂=(0,0,1)`: `e₁↦(1/√2,0)`, `e₂↦(−1/√2,0)`,
`e₃↦(0,1)`, `−e₁↦(−1/√2,0)`, `−e₂↦(1/√2,0)`, `−e₃↦(0,−1)`. Los 6 vértices
colapsan a 4 puntos distintos: `(±1/√2,0)` y `(0,±1)` — un rombo de
diagonales `√2` (horizontal, `2·1/√2`) y `2` (vertical). Área
`(d₁d₂)/2 = √2`:

```
$ target/release/dz-cli.exe eval "sqrt(2)" --mode fixed --scale 8
~1;4E79170X
```

### 2.3 Los tres promedios finitos

Como las tres familias son órbitas del mismo grupo de simetría, cada
promedio es simplemente el valor constante de esa familia (no hay
variación que promediar dentro de cada una):

| familia finita | tamaño | sombra individual | promedio | naturaleza |
|---|--:|---|---|---|
| direcciones-eje | 6 | `2r²` | **`2r²`** | racional |
| direcciones-diagonal | 8 | `√3 r²` | **`√3 r²`** | algebraico irracional |
| direcciones-arista | 12 | `√2 r²` | **`√2 r²`** | algebraico irracional |

```
$ target/release/dz-cli.exe eval "(2+2+2+2+2+2)/6" --mode rational
2
```

### 2.4 Verdicto honesto: ¿es esto un tamaño de frontera legítimo, y hay contenido nuevo?

**Lo que sí es cierto y es genuinamente distinto de las tres medidas
anteriores**: ninguna de las tres sombras individuales, ni ninguno de los
tres promedios, contiene `π`. Esto no es casualidad ni un truco de
normalización — es estructural, y vale la pena decir por qué con precisión:
**cualquier promedio sobre un conjunto *finito* de direcciones es una suma
finita de áreas de polígonos con vértices en coordenadas algebraicas
(combinaciones racionales de raíces cuadradas de la dimensión del
espacio), y una suma o cociente finito de números algebraicos es
algebraico.** `π` solo puede entrar por una **integral continua** sobre la
esfera (de donde sale `ω_k`, como en Busemann/Holmes–Thompson) o por una
suma infinita/límite. Un promedio sobre 6, 8 o 12 direcciones no hace
ninguna de las dos cosas, así que **discretizar la fórmula de Cauchy evita
la trascendencia con certeza, no por suerte**, sea cual sea el conjunto
finito de direcciones elegido. Esto es un hallazgo real y verificable, y no
estaba en `03-hallazgo-...`, que solo consideró el caso continuo.

**Lo que no es cierto, y hay que decirlo sin regatear**: la premisa de la
tarea sugería que «cada sombra individual, para direcciones especiales
alineadas, puede ser racional» y que promediar cantidades racionales daría
un racional «por construcción». Eso solo se verificó para **una** de las
tres familias (la de los ejes, que da `2r²`, exactamente el área ya
conocida del disco L₁ 2D). Las otras dos familias —las que de hecho
usan la estructura propia del octaedro (sus 8 caras, sus 12 aristas) en vez
de la del cubo circunscrito— dan sombras **individualmente irracionales**
(`√3`, `√2`), y como las 8 (o las 12) sombras son idénticas por simetría, no
hay ninguna cancelación de irracionalidades que promediar: el promedio de 8
copias de `√3` es `√3`, sin más. **La racionalidad de la familia de ejes no
es una propiedad de "promediar sobre un conjunto finito" en general: es,
otra vez, un artefacto de que esas 6 direcciones son los propios ejes de
coordenadas con los que se definió la norma L₁** —el mismo fenómeno que
`01-hallazgos-A-constantes.md` §7.3 ya documentó para el perímetro
(«el 4 no sobrevive a una rotación de 45°»): la sombra es racional
precisamente cuando se proyecta *a lo largo* de un eje de simetría de la
propia norma, y algebraica-pero-irracional en cuanto se proyecta en
cualquier otra dirección simétrica del cuerpo, incluidas las que la propia
estructura combinatoria del octaedro (caras, aristas) sugeriría como
«naturales».

**¿Es esto un tamaño de frontera legítimo o un artefacto de discretización?**
Es legítimo como invariante — y de hecho tiene un nombre y un uso real:
en estereología y cristalografía, el promedio de proyección sobre un
conjunto finito de direcciones simétricamente distribuido es una técnica
estándar (proyección estereográfica muestreada, análisis de partículas por
sombra), y converge al área media de Cauchy cuando el conjunto de
direcciones se refina. Con 6, 8 o 12 direcciones **no** es una aproximación
precisa del área de Cauchy continua (ninguno de los tres valores —`2`,
`√3≈1,732`, `√2≈1,414`— está cerca del promedio continuo verdadero, que
también vale `√3` por la propia fórmula de Cauchy aplicada a la superficie
ya conocida, y el hecho de que la familia diagonal coincida dígito a dígito
con ese valor exacto en §2.2(b) es una coincidencia de simetría de esa
familia concreta, no una propiedad general del muestreo finito — no se
afirma lo contrario). Lo que sí es cierto sin reservas: **es una medida
distinta, con contenido real (no es un número inventado — tiene una
interpretación física genuina de "sombra media bajo iluminación desde N
direcciones simétricas"), y su comportamiento respecto a la racionalidad
depende enteramente de qué familia de direcciones se elija**, lo cual es en
sí mismo el hallazgo: no existe una noción canónica de «promedio finito de
sombras» sin especificar el conjunto de direcciones, exactamente como
`01-hallazgos-A-constantes.md` ya encontró que no existe una constante
«perímetro/diámetro» canónica sin especificar la orientación.

### 2.5 Casilla y veredicto de este reencuadre

**(III)** en su forma más pura para el resultado positivo (ninguna sombra
finita trae `π`, y la razón es aritmética: sumas finitas de algebraicos son
algebraicas — este hecho no depende de la métrica L₁ en absoluto, sería
igual de cierto para el cubo o para cualquier poliedro). **(II)**, con la
misma advertencia de `01-hallazgos-A-constantes.md` §7.3, para la
racionalidad de la familia de ejes: es una propiedad de la alineación entre
las direcciones muestreadas y los ejes de simetría de la norma, no de L₁ ni
de la base 12. **Novedad real de este reencuadre**: es la primera vez en
esta investigación en que se distingue explícitamente entre «evitar `π`»
(que el muestreo finito consigue siempre, con certeza estructural) y
«obtener un racional» (que depende de con qué dirección se alinee el
muestreo, y que solo una de las tres familias examinadas consigue). Las
medidas anteriores (Hausdorff, Busemann, HT) confundían necesariamente
ambas preguntas porque solo tenían una respuesta cada una; el muestreo
finito las separa en dos preguntas independientes, y esa separación **es**
el contenido nuevo.

---

## Reencuadre 3 — Complejidad descriptiva de la forma frente a complejidad de su medida

### 3.1 La pregunta que ningún área continua puede hacer

Hausdorff, Busemann y Holmes–Thompson, y también el muestreo del reencuadre
2, tienen todos la misma forma: producen **un número real** y preguntan si
es racional. Este reencuadre cambia la pregunta de raíz: en vez de «¿qué
número mide el borde?», pregunta **«¿cuál es la descripción exacta y finita
mínima que reconstruye el borde, sin ambigüedad y sin límite?»** — la misma
lógica que el proyecto ya aplica a fórmulas con `K(F)`, extendida del
*símbolo* al *objeto geométrico* que el símbolo describe.

### 3.2 Las dos descripciones exactas, lado a lado

**Ecuación implícita del borde** (la forma más comprimida de decir «qué
puntos están en la frontera»):

| cuerpo | ecuación | N_raíces | N_irrac. | N_oper. | K |
|---|---|--:|--:|--:|--:|
| octaedro L₁ | `\|x\|+\|y\|+\|z\| = r` | 0 | 0 | 5 (3 `\|·\|` + 2 sumas) | **5** |
| esfera L₂ | `x²+y²+z² = r²` | 0 | 0 | 5 (3 potencias + 2 sumas) | **5** |

Con las reglas de conteo de `00-plan-investigacion.md` §2.1 (`|·|` cuenta 1
cada aparición, una suma de `k` términos cuenta `k−1`), **las dos ecuaciones
implícitas empatan exactamente en `K=5`.** Ninguna es más simple que la
otra por este índice. Es un resultado que hay que subrayar porque no era
obvio de antemano: se podría haber esperado que el octaedro, siendo
poliédrico y «hecho de líneas rectas», tuviera una descripción más simple
que la esfera lisa — no la tiene, según este índice. Y viceversa: se podría
haber esperado que la ecuación de la esfera, al ser «la forma redonda
canónica», fuera trivialmente más simple — tampoco, empata.

**Fórmulas de superficie de los mismos dos cuerpos** (ya tabuladas en
`01-hallazgos-A-constantes.md` §5, filas F8a/F8b, se reusan aquí con el
mismo desglose):

| cuerpo | fórmula | N_raíces | N_irrac. | N_oper. | K | clase del número |
|---|---|--:|--:|--:|--:|---|
| octaedro L₁ | `S₁ = 4√3 r²` | 1 | 1 (√3) | 2 | **4** | algebraico irracional |
| esfera L₂ | `S₂ = 4π r²` | 0 | 1 (π) | 2 | **3** | trascendente |

### 3.3 El resultado que separa los dos ejes

Poniendo las dos tablas juntas:

| | K(ecuación del borde) | K(fórmula de superficie) |
|---|--:|--:|
| octaedro L₁ | 5 | 4 |
| esfera L₂ | 5 | 3 |

**Las dos filas empatan en la primera columna y difieren en la segunda.**
Esto es la demostración directa, no solo la sospecha, de que **complejidad
de la forma** (cuánto cuesta escribir la ecuación que la define) y
**complejidad de la medida** (cuánto cuesta escribir la fórmula de su
superficie) son **ejes independientes**. Concretamente:

- Tener una ecuación de frontera igual de simple (`K=5` en las dos) **no
  predice** cuál de las dos tendrá la superficie más simple según `K(F)`.
  De hecho el resultado es contraintuitivo en el sentido opuesto al que
  esta investigación esperaría por defecto: **la esfera, con superficie
  trascendente, saca un `K` menor (3) que el octaedro, con superficie
  «solo» algebraica (4)** — porque `K(F)` cuenta la ocurrencia de un
  radical (`N_raíces`) con el mismo peso que la de una constante
  irracional distinta (`N_irrac.`), así que introducir `√3` cuesta un punto
  más que introducir `π`, aunque `√3` sea, en el sentido de la teoría de
  números, una cantidad "menos rara" que `π` (algebraica de grado 2 contra
  trascendente). **`K(F)` no es un índice de "qué tan irracional" es un
  número — es un índice de longitud de fórmula, y esas dos cosas no son la
  misma pregunta.** Este es un matiz que ya estaba latente en
  `01-hallazgos-A-constantes.md` §5.1 (donde F8 ya se señalaba como la
  única fila que sube `K`) pero que aquí se vuelve explícito y con nombre:
  hay **tres** clasificaciones posibles de un mismo número —racional /
  algebraico irracional / trascendente— y el índice `K(F)` del plan solo
  ve una de ellas de forma indirecta (`N_irrac.`, que cuenta *distintas*
  constantes irracionales, tratando `√3` y `π` como una unidad cada una,
  sin distinguir su clase), mientras que `N_raíces` introduce una segunda
  penalización que no tiene que ver con la clase del número sino con la
  *notación* usada para escribirlo (`√3` se escribe con un radical, `π` no,
  aunque ambas sean "una constante irracional que hay que llevar en la
  fórmula").
- La independencia se puede afirmar en la otra dirección también: que la
  superficie de un cuerpo sea complicada (algebraica o trascendente) **no
  implica** que su ecuación de frontera lo sea. El octaedro tiene la
  fórmula de área "peor" de las dos (K=4 > 3) y sin embargo su ecuación de
  frontera es tan simple como la de la esfera (K=5 = 5).

### 3.4 La asimetría que `K(F)` no puede ver en absoluto: finitud combinatoria

Hay una segunda noción de «simplicidad de la forma» que ni la ecuación
implícita ni ninguna medida de área capturan, y es la que de verdad separa
al octaedro de la esfera de forma asimétrica — a favor del octaedro esta
vez, y sin ambigüedad:

**El borde del octaedro admite una descripción combinatoria exacta,
finita y completamente racional, sin ecuación:** 6 vértices en
`(±r,0,0),(0,±r,0),(0,0,±r)` (coordenadas racionales, de hecho enteras si
`r` lo es), 12 aristas (la lista exacta: cada `eᵢ` con cada `±eⱼ`, `j≠i`),
8 caras triangulares (la lista de qué tripletas de vértices forman cada
cara). Esta lista **reconstruye el borde exactamente** — cada punto de la
frontera es una combinación convexa de los vértices de alguna cara de la
lista — y no necesita ningún número irracional para especificarse: es
datos finitos, enteros o racionales, punto.

**El borde de la esfera no admite ninguna lista finita de vértices, aristas
y caras que lo reconstruya exactamente.** Es una superficie lisa
(curvatura gaussiana constante y positiva en todo punto), y ningún
poliedro —con un número finito de caras planas— puede coincidir con ella
en más que un conjunto de medida nula (los propios vértices, si se
inscriben). Para "describir exactamente" la esfera con datos combinatorios
finitos, la única vía es un **límite**: una sucesión de poliedros inscritos
cuyo número de caras `n → ∞`, y el límite no es ningún término finito de
esa sucesión, es una idealización a la que la sucesión converge. La
alternativa —abandonar la descripción combinatoria y usar la ecuación
implícita `x²+y²+z²=r²`— es perfectamente válida y, como se vio en §3.2,
tan simple como la del octaedro (`K=5` las dos) — pero es una descripción
**de otro tipo**: no es una lista de piezas planas finitas, es una
condición algebraica sobre las tres coordenadas.

Esto es una distinción real, no forzada: hay dos preguntas distintas —«¿cuál
es la ecuación implícita más corta?» (empate, §3.2) y «¿el borde es una
unión finita de piezas planas (un politopo) o no?» (el octaedro sí, la
esfera estructuralmente no) — y solo la segunda separa a los dos cuerpos.
La segunda pregunta no es una que `K(F)`, ni Hausdorff, ni Busemann, ni
Holmes–Thompson, ni ninguna medida de área basada en un número real puedan
formular siquiera: todas ellas presuponen que el cuerpo ya está dado, y
preguntan cuánto mide; esta pregunta se hace **antes**, sobre cómo se
especifica el propio cuerpo.

### 3.5 ¿Es esto el hallazgo más importante del documento? Argumento explícito, sin sobrevender

Se nombra explícitamente porque el enunciado de la tarea lo pedía y porque,
tras trabajarlo, sí sostiene el peso: **la premisa implícita de toda esta
investigación —recogida sin decirlo en `99-sintesis.md` y en la propia
`00-plan-investigacion.md`— es que "forma simple" y "medida simple" deberían
ir juntas, y que cuando no van juntas (el caso F8, la superficie del
octaedro) eso es una anomalía a explicar.** Este reencuadre muestra que no
es una anomalía: es la regla. Los dos ejes —complejidad de la
*especificación* del cuerpo y complejidad/clase de su *medida*— son
independientes por construcción, y se puede exhibir en los dos sentidos
con el mismo par de cuerpos:

1. Especificación empatada (`K=5` en la ecuación implícita de ambos), medida
   distinta y en dirección contraintuitiva (`K=4` algebraico para el
   octaedro contra `K=3` trascendente para la esfera — la esfera "gana" en
   el índice simbólico a pesar de ser la trascendente).
2. Especificación estructuralmente distinta en el sentido combinatorio
   (politopo finito exacto para el octaedro, imposible para la esfera),
   que **no se refleja en absoluto** en ninguna medida de área de las ya
   calculadas — el octaedro "gana" aquí, de forma limpia y sin depender de
   ninguna convención de conteo.

**Nombre para el hallazgo, tal como pide la tarea**: la investigación venía
tratando implícitamente «¿la forma es simple?» como una *proxy* de
«¿la medida será simple?», y las dos preguntas no están correlacionadas —
ni en el sentido débil de una tendencia estadística sobre los pocos casos
examinados, ni en el sentido fuerte de una implicación lógica. Llamarlo
**la independencia forma/medida** es exacto: un cuerpo puede tener
frontera exactamente especificable con datos finitos y racionales y aun así
tener una medida (de cualquier tipo razonable) algebraica o trascendente; y
un cuerpo cuya frontera es imposible de especificar con datos finitos
(la esfera) puede tener, por la vía de la ecuación implícita, una
descripción tan corta como la de un politopo, y una medida cuyo índice
simbólico (`K(F)`) resulta *menor*, no mayor, que la del politopo.

### 3.6 ¿Es esto un reencuadre genuinamente distinto, o el mismo resultado por otra vía?

**Genuinamente distinto**, y con un criterio objetivo para afirmarlo: los
reencuadres 1 y 2 —aun cuando aportaron algo real (§1.4 la curvatura
combinatoria, §2.4 la certeza estructural de evitar π por discretización)—
en última instancia siguen produciendo **un número** (una curvatura total,
un área media) y preguntando por su racionalidad o su clase. Éste no
produce ningún número comparable a `4√3`, `16π/3` o `48/π`: produce una
comparación de **dos objetos de naturaleza distinta** —una lista finita de
datos combinatorios contra la ausencia de tal lista— que no es un punto en
la misma recta numérica que las medidas de área, y por tanto no puede
"colapsar de vuelta" al mismo resultado por una vía distinta, porque no está
contestando la misma pregunta. Es el único de los tres reencuadres que
cambia la *categoría* de la respuesta (de "número real" a "existencia o no
de una descripción finita"), no solo el *método* para llegar a un número.

---

## Verdicto por reencuadre — resumen obligatorio

| reencuadre | ¿cambia la mirada? | ¿colapsa al mismo resultado? | casilla |
|---|---|---|---|
| 1. Topológico (Descartes/Gauss–Bonnet) | Sí — explica *por qué* Busemann/HT estaban condenadas a `π` (normalizan contra `ω_k`, que hereda `2π` de la topología del círculo), algo que `03-hallazgo-...` no había explicado, solo medido. El sustituto combinatorio (`κ_comb`) evita `π` con certeza. | Parcialmente — el defecto angular clásico *es* el mismo tipo de resultado de siempre (un múltiplo de `π`), solo que ahora se entiende como forzado por `χ`, no por elección de medida. El sustituto que sí es nuevo (`κ_comb`) deja de medir tamaño: mide combinatoria pura. | **(I)** para el defecto angular (πχ es intrínseco a cualquier curvatura basada en ángulos); resultado nuevo pero de alcance limitado para `κ_comb` |
| 2. Proyectivo/discreto (Cauchy finito) | Sí — separa por primera vez «evitar π» (que el muestreo finito logra siempre, con certeza aritmética) de «obtener un racional» (que depende de la alineación con los ejes de la norma, y solo ocurre en una de las tres familias probadas). | No del todo — la familia que da racional es la misma familia axis-aligned de siempre (§7.3 de la línea A); las familias "genuinamente nuevas" (caras, aristas del propio octaedro) dan irracionales algebraicos, ni mejores ni peores que Hausdorff. | **(III)** para «evitar π» (aritmético, no geométrico); **(II)** para la racionalidad de la familia de ejes |
| 3. Descriptivo (K(F) de la forma vs. de la medida) | **Sí, sin reservas** — cambia la categoría de pregunta, no solo el método. Muestra que la premisa «forma simple ⟹ medida simple» de toda la investigación es lógicamente independiente, con un contraejemplo limpio en las dos direcciones. | No — no hay ningún «mismo resultado» al que volver, porque la pregunta ya no es sobre un número. | **(III)** para el empate en la ecuación implícita (efecto de notación/índice, no de la métrica); **(I)** para la imposibilidad combinatoria finita de la esfera (es la geometría diferencial de L₂, no una elección) |

**Veredicto global**: de los tres, el reencuadre 3 es el que constituye un
paradigma genuinamente distinto, no una repetición del mismo cálculo por
otra vía — la propia tarea lo anticipaba y el trabajo lo confirma. El
reencuadre 1 aporta una explicación estructural real (por qué Busemann/HT
estaban condenadas) pero, en su forma que sí mide algo geométrico (el
defecto angular), reproduce el mismo tipo de constante de siempre por una
razón que ahora se entiende mejor; su versión que evita π genuinamente
(`κ_comb`) deja de ser una medida de tamaño. El reencuadre 2 aporta una
distinción fina y verificable (evitar-π es aritmético y automático;
obtener-racional depende de la alineación) que no estaba en el resto de la
investigación, pero no cambia el diagnóstico de fondo sobre L₁: sigue siendo
la alineación con los ejes, no la geometría L₁ en abstracto, la que decide.

---

## Lo que no se hizo (obligatorio, no vacío)

- **No se demostró un teorema general para el reencuadre 1.** Se verificó
  Descartes para el octaedro regular concreto (§1.2) y se argumentó (§1.3)
  que la razón es Gauss–Bonnet discreto, un teorema conocido — pero no se
  intentó una prueba original de que *toda* curvatura basada en ángulos
  sobre *cualquier* poliedro convexo debe traer `π`; se dio el argumento
  estructural (el ángulo pleno es una constante topológica del plano
  tangente) sin formalizarlo más allá de citar Gauss–Bonnet.
- **`κ_comb` no se comparó con otras curvaturas combinatorias de la
  literatura** (existen variantes, p. ej. la de Ishida o la basada en
  ángulos "combinatorios" de 60°/90° en vez de grados reales de cara). No
  se descartaron ni se probaron; `κ_comb` (Higuchi) se eligió porque es la
  más citada y la más simple de verificar a mano.
- **En el reencuadre 2 solo se probaron tres familias de direcciones**, las
  tres más naturales dado el octaedro/cubo dual, no una búsqueda exhaustiva
  de qué conjuntos finitos de direcciones dan sombras racionales. Es
  plausible que existan otras familias finitas (p. ej., las 12 direcciones
  de un icosaedro inscrito, que no tiene relación de simetría privilegiada
  con el octaedro) que también den algo interesante; no se exploraron.
- **No se demostró formalmente que "suma finita de algebraicos es
  algebraica" sea la única razón por la que las tres sombras evitan π** —es
  un hecho estándar de teoría de cuerpos (los números algebraicos forman un
  cuerpo cerrado bajo `+,−,×,÷`), citado aquí, no reprobado desde cero.
- **En el reencuadre 3, la comparación de `K(F)` para ecuaciones implícitas
  usa las mismas reglas de conteo del plan, diseñadas originalmente para
  fórmulas cerradas de constantes, no para ecuaciones de frontera.**
  Aplicarlas a `|x|+|y|+|z|=r` y a `x²+y²+z²=r²` es una extensión razonable
  pero no estaba en el protocolo original, y alguien podría contar
  diferente (p. ej., no contar el `=` ni la variable `r` como parte del
  costo, que es lo que se hizo aquí, siguiendo la convención ya usada en
  `01-hallazgos-A-constantes.md` §5 para `C₂=2πr`).
- **No se exploró si la independencia forma/medida (§3.5) se sostiene para
  otros pares de cuerpos** más allá de octaedro/esfera — es un resultado de
  un caso, presentado con el lenguaje de un principio general porque el
  argumento que lo sostiene (dos ejes de conteo lógicamente no
  correlacionados en `K(F)`, más la imposibilidad combinatoria de aproximar
  exactamente una superficie curva con un politopo finito) es general y no
  depende de los números concretos de este par, pero no se verificó con un
  segundo ejemplo (p. ej. el cubo L∞ frente a la esfera).
- **No se sometió nada de esto a revisión externa**, siguiendo la práctica
  ya declarada del resto del repositorio (`99-sintesis.md` §7).
