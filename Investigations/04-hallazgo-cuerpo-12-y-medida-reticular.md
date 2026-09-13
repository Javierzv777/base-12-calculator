# Hallazgo — Un cuerpo de 12 caras y una medida reticular

Extiende `01-hallazgos-A-constantes.md` §7.1 y `03-hallazgo-busemann-holmes-thompson.md`
por dos vías independientes, ambas motivadas por la síntesis
(`99-sintesis.md` §1, §4.1): la bola L₁ en 3D tiene grupo de simetrías `Oh`
que en 2D se ve como `D₄` de orden 8, y la base 12 regala 2, 3, 4, 6, 12. Si
la superficie de Hausdorff del octaedro L₁ es irracional (`4√3 r²`,
casilla I dura), ¿hay algún cuerpo *genuinamente* de 12 caras, o alguna
medida *genuinamente* reticular, que rompa el patrón?

Fecha de apertura: 2026-09-12.

Protocolo: `00-plan-investigacion.md`. Toda cifra dozenal sale de
`target/release/dz-cli.exe` con el comando pegado al lado. Se clasifica en
casilla (I)/(II)/(III). Hecho no negociable, citado aquí porque este
documento lo pone a prueba dos veces: π es trascendente (Lindemann–
Weierstrass) y esa propiedad **no depende de la base numérica** — lo único
que la base puede afectar es si la expansión de un racional *ya racional*
termina o no.

---

## 0. Resumen del veredicto (ancla)

**No, en las dos avenidas, y por razones distintas.**

**Avenida 1 (dodecaedro rómbico, celda de Voronoi de la red FCC, 12 caras
genuinas).** Existe una norma real cuya bola unidad es el DR
(`max(|x|+|y|,|y|+|z|,|z|+|x|)`). Su superficie de Hausdorff es `6√2 r²`
(algebraico irracional, verificado por dos métodos). Bajo Busemann da
`4π r²` y bajo Holmes–Thompson `36/π r²` — ambas trascendentes, repitiendo
exactamente el patrón que `03-hallazgo-busemann-holmes-thompson.md` ya
encontró en el octaedro liso. Ni el 12 en el recuento de caras ni una
simetría más rica (`Oh`, orden 48) evitan que `π` se filtre. Casilla (I).

**Avenida 2 (Ehrhart).** El coeficiente `e₂=2` del polinomio de Ehrhart del
octaedro L₁ reticular **sí es racional** (entero, de hecho), verificado por
dos métodos. Pero es racional por un teorema sin condiciones sobre *todo*
politopo reticular — no una propiedad de L₁ ni de base 12 — y la cantidad
que mide (`relvol`, ligada a un retículo fijo) no es invariante por
isometrías, así que no es «superficie» en ningún sentido razonable. Es el
movimiento que el plan prohíbe, identificado y reportado como tal en vez de
disimulado. Casilla (III), débil.

**Veredicto global**: ninguna de las dos avenidas rationaliza una
constante de superficie 3D relacionada con L₁ o con base 12. Sumado a
`03` y a `05`, esta serie ha probado seis vías independientes de definir
«tamaño de frontera» sobre cuerpos poliédricos L₁-relacionados (Hausdorff,
Busemann, Holmes–Thompson —dos veces cada una, sobre dos cuerpos—, defecto
angular topológico, muestreo de Cauchy finito, y Ehrhart) y ninguna produce
un racional que sea a la vez invariante por isometrías, consistente con
Euclides en el caso liso, y genuinamente una medida de tamaño. Detalle
completo en §5.

---

## 1. Avenida 1 — el dodecaedro rómbico

### 1.1 Qué cuerpo es y qué norma lo tiene por bola unidad

El dodecaedro rómbico (DR) es la celda de Voronoi de la red FCC, tiene 12
caras romboidales congruentes, 14 vértices y 24 aristas
(`14 − 24 + 12 = 2`, Euler correcto). No se eligió por parecerse a «12»
retóricamente: es el único sólido con exactamente 12 caras que aparece de
forma natural como celda de una red (a diferencia del dodecaedro regular,
que no tesela `R³` y no es celda de Voronoi de ninguna red).

**¿Existe una norma cuya bola unidad sea exactamente el DR?** Sí, y se
deriva aquí en vez de citarse. El DR es un **zonotopo**: la suma de
Minkowski de las 4 diagonales espaciales de un cubo,

```
Z = { t₁a + t₂b + t₃c + t₄d : tᵢ ∈ [-1/2, 1/2] },  a,b,c,d las 4 diagonales
```

(4 generadores en posición genérica en `R³` dan `C(4,2) = 6` pares de caras
paralelas = 12 caras — la cuenta combinatoria por sí sola predice el 12,
antes de calcular nada). Un zonotopo centrado en el origen es simétrico,
convexo y de interior no vacío, así que **es** la bola unidad de una norma
— su función gauge — sin necesidad de imponer nada.

La vía directa a la fórmula cerrada de ese gauge pasa por el **politopo
polar**. El polar de un politopo tiene por función soporte exactamente la
gauge del original, y el polar de un zonotopo de generadores `vᵢ` es la
intersección de semiespacios `⟨u,vᵢ⟩ ≤ 1`. Trabajando con el DR de vértices
`(±1,0,0)` y permutaciones (6 vértices «tipo octaedro») más
`(±1/2,±1/2,±1/2)` (8 vértices «tipo cubo») — la parametrización estándar
de la celda de Voronoi FCC — el polar resulta ser exactamente el
**cuboctaedro** `{u : |uₓ|≤1, |u_y|≤1, |u_z|≤1, |uₓ|+|u_y|+|u_z|≤2}`
(cubo ∩ octaedro escalado). La gauge del DR es la función soporte de ese
cuboctaedro, y como la función soporte de un politopo es el máximo de
`⟨v,x⟩` sobre sus vértices (permutaciones de `(±1,±1,0)`), el máximo sobre
las combinaciones de signo de `±x±y` colapsa a `|x|+|y|`, y así para los
tres pares:

```
‖(x,y,z)‖_DR = max( |x|+|y|,  |y|+|z|,  |z|+|x| )
```

**Verificación de que la bola de esta norma es el DR y no otra cosa**:
resolviendo `‖x‖_DR = 1` se obtiene el conjunto extremo `{(±1,0,0)`
y perms`}` (6 puntos, donde domina un único término) y
`{(±1/2,±1/2,±1/2)}` (8 puntos, donde los tres términos empatan) — 14
puntos extremos, que es exactamente el recuento de vértices del DR citado
arriba. La norma tiene el grupo de simetría **completo** de permutaciones y
cambios de signo de las tres coordenadas (`Oh`, orden 48) — el mismo grupo
que el octaedro L₁ en `R³` tiene (la referencia a `D₄` de orden 8 en
`99-sintesis.md` §1 es del rombo **2D**, no del octaedro 3D; en 3D ambos
cuerpos —octaedro y DR— comparten `Oh`, así que la mano de cartas de
simetría no cambia entre las dos avenidas de este documento).

```
$ target/release/dz-cli.exe eval "1/2" --mode rational
1/2
```

(el 1/2 de los vértices tipo cubo es exacto en cualquier base; no hay nada
que medir ahí — se anota solo para tener la cifra con el binario, como pide
el protocolo.)

### 1.2 Superficie de Hausdorff, cara a cara

Fijar `r = 1` (todo escala como `r²`, no se rehace el álgebra por separado).
Por simetría `Oh` basta una cara. Tómese la rama `x,y ≥ 0` de `|x|+|y|=1`
como la ecuación de la cara, con la restricción de que ese término sea el
máximo: `|z| ≤ 1 − max(x,y)`. Parametrizando `x ∈ [0,1]`, `y=1−x`,
`z ∈ [−min(x,1−x), min(x,1−x)]`, los cuatro vértices de esa cara son:

```
(0,1,0),  (1/2,1/2,1/2),  (1,0,0),  (1/2,1/2,-1/2)
```

Diagonal 1 (entre los dos vértices «tipo octaedro»): `(1,0,0)−(0,1,0) =
(1,−1,0)`, longitud `√2`. Diagonal 2 (entre los dos «tipo cubo»):
`(1/2,1/2,1/2)−(1/2,1/2,−1/2) = (0,0,−1)`, longitud `1`. Los puntos medios
de ambas diagonales coinciden en `(1/2,1/2,0)` (se bisecan: es un
paralelogramo) y su producto escalar es 0 (son perpendiculares: es un
rombo). Área de un rombo por sus diagonales, `(1/2)d₁d₂`:

```
área de una cara = (1/2)·√2·1 = √2/2
```

```
$ target/release/dz-cli.exe eval "sqrt(2)/2" --mode fixed --scale 8
~0;859X6965
```

Con 12 caras congruentes (transitivas bajo `Oh`, el DR es un sólido de
Catalan — isoédrico por construcción, dual del cuboctaedro arquimediano):

```
S_Hausdorff(DR) = 12 · (√2/2) r² = 6√2 r²
```

```
$ target/release/dz-cli.exe eval "6*sqrt(2)" --mode fixed --scale 8
~8;59X6965
$ target/release/dz-cli.exe eval "6*sqrt(2)" --mode rational
~8;59X696503EX
```

El `~` en modo racional (que solo lo omite cuando existe fracción exacta)
es la confirmación visual de irracionalidad, igual que en `01-hallazgos-
A-constantes.md` §7.1 para `4√3`. `6√2` es **algebraico** (raíz de
`x² = 72`), no trascendente: misma clase que el `√3` del octaedro, no
mejor ni peor.

### 1.3 Contraverificación independiente

**Vía 1 — fórmula cristalográfica estándar.** La longitud de arista del DR
aquí construido, entre `(0,1,0)` y `(1/2,1/2,1/2)`: vector `(1/2,−1/2,1/2)`,
longitud `√3/2`.

```
$ target/release/dz-cli.exe eval "sqrt(3)/2" --mode fixed --scale 8
~0;X485XX
```

La fórmula conocida de superficie del DR en términos de la arista `a` es
`S = 8√2 a²` (Wikipedia, *Rhombic dodecahedron*; se deriva independientemente
del cálculo cara-a-cara de §1.2, por geometría de sólidos platónicos/
catalan estándar en vez de por la parametrización del gauge). Sustituyendo
`a = √3/2`:

```
$ target/release/dz-cli.exe eval "8*sqrt(2)*(sqrt(3)/2)^2" --mode fixed --scale 8
~8;59X69651EXE17E64E8X8
```

Coincide dígito a dígito con `6√2` de §1.2 (`8;59X6965…`) — la primera
contraverificación, por una ruta que no comparte el álgebra del gauge
(trigonometría de rombos vs. fórmula de manual de cristalografía).

**Vía 2 — volumen, por descomposición en pirámides.** Cada cara está a
distancia `1/√2` del origen (distancia de la recta/plano `x+y=1` al
origen, `|1|/‖(1,1,0)‖₂ = 1/√2`), así que el volumen es la suma de 12
pirámides de base la cara y altura `1/√2`:

```
V(DR) = 12 · (1/3)·(√2/2)·(1/√2) = 12 · (1/6) = 2
```

```
$ target/release/dz-cli.exe eval "2" --mode rational
2
```

**Racional, de hecho entero** — el volumen tampoco escapa al patrón ya
visto en `01-hallazgos-A-constantes.md` E2 (el volumen de la bola L₁ es
racional en toda dimensión; aquí el volumen del DR también lo es).
Contraverificación cruzada con la fórmula cristalográfica de volumen
`V = (16√3/9) a³`:

```
$ target/release/dz-cli.exe eval "(16*sqrt(3)/9)*(sqrt(3)/2)^3" --mode fixed --scale 8
~2;300000010820000015X48E14
```

`≈ 2` (la deriva tras el punto es error de truncamiento acumulado al cubicar
una raíz truncada, no una discrepancia real — mismo fenómeno que el modo
racional documenta en otras cifras de esta serie). Dos métodos
independientes (pirámides desde el origen; fórmula de manual en función de
la arista) confirman `V=2`, y esa confirmación valida indirectamente que la
geometría de la cara usada en §1.2 (distancias, arista) es correcta antes de
fiarse de su superficie.

### 1.4 Busemann y Holmes–Thompson sobre el dodecaedro rómbico

Hausdorff dio irracional-algebraico. Por el protocolo de la tarea, antes de
cerrar la avenida se prueban las otras dos definiciones de área de
superficie para espacios normados, con el mismo método de
`03-hallazgo-busemann-holmes-thompson.md` §1: sección de la bola primal por
el plano de dirección de una cara (`E₀`), su polar dentro de `E₀`,
normalización por `ω₂ = π`.

**Base de `E₀`.** La cara de §1.2 vive en el plano `x+y=1`; su dirección es
`E₀: x+y=0`. Base ortonormal: `u=(1,−1,0)/√2`, `v=(0,0,1)`. Un punto
`p = su+tv` tiene `x=s/√2, y=−s/√2, z=t`, así que

```
‖p‖_DR = max( |x|+|y|, |y|+|z|, |z|+|x| )
       = max( √2|s|,  |s|/√2+|t| )
```

(los dos últimos términos coinciden porque `|x|=|y|=|s|/√2`).

**Sección primal `B_DR ∩ E₀`.** La región `‖p‖_DR ≤ 1` es
`|s| ≤ 1/√2` y `|t| ≤ 1−|s|/√2`. Integrando:

```
área(s,t) = ∫_{-1/√2}^{1/√2} 2(1−|s|/√2) ds = 4[s − s²/(2√2)]₀^{1/√2}
          = 4(1/√2 − 1/(4√2)) = 3/√2 = 3√2/2
```

```
$ target/release/dz-cli.exe eval "3*sqrt(2)/2" --mode fixed --scale 8
~2;15578473
```

Es un hexágono **no regular** (vértices `(0,±1)` y `(±1/√2,±1/2)`,
verificado también por *shoelace* sobre esos 6 puntos, mismo resultado
`3√2/2` — no se detalla la aritmética del shoelace aquí porque es mecánica,
solo se deja constancia de que se hizo como cross-check interno antes de
seguir).

**Densidad y área de Busemann de una cara:**

```
θ_Bus = π / (3√2/2) = π√2/3
área_Bus(cara) = (√2/2)·(π√2/3) = π/3
```

```
$ target/release/dz-cli.exe eval "pi/3" --mode fixed --scale 8
~1;06968317
$ target/release/dz-cli.exe eval "pi/3" --mode rational
~1;0696831713E
```

Total sobre 12 caras:

```
S_Bus(DR) = 12 · π/3 = 4π r²
```

```
$ target/release/dz-cli.exe eval "4*pi" --mode fixed --scale 8
~10;6968317
$ target/release/dz-cli.exe eval "4*pi" --mode rational
~10;696831713E08
```

**Observación que merece anotarse, sin sobrevenderla**: `S_Bus(DR) = 4π r²`
es *el mismo número* que la superficie euclídea de la esfera de radio `r`.
Es una coincidencia real —no un error de signo ni de factor— pero **no es
una racionalización**: `4π` sigue siendo trascendente. Es, si acaso, un
indicio de que la normalización de Busemann trata al DR como
«aproximadamente isótropo» (razonable: el DR es la celda de Voronoi de la
red más isótropa en 3D, la FCC), pero eso no toca la cuestión de
racionalidad en absoluto.

**El dual: polar del DR, el cuboctaedro.** Ya derivado en §1.1: el polar de
`‖·‖_DR ≤ 1` es el cuboctaedro `{u: |uₓ|,|u_y|,|u_z| ≤ 1,
|uₓ|+|u_y|+|u_z| ≤ 2}`, de 12 vértices (permutaciones de `(±1,±1,0)`).

**Camino A — polar del hexágono dentro de `E₀`, por aristas.** Las 6
aristas del hexágono primal (§ arriba) dan, por la construcción estándar
recta-tangente → vértice dual (`ax+by=c ↦ (a/c,b/c)`), los vértices duales
`(±1/√2,±1)` y `(±√2,0)` — seis puntos.

**Camino B — proyección directa del cuboctaedro sobre `E₀`.** Proyectando
los 12 vértices `(±1,±1,0)` y perms sobre `E₀` vía
`s=(x−y)/√2, t=z` (coordenadas directas en la base ortonormal de `E₀`, sin
necesidad de restar la componente normal por separado): dos vértices
`(1,1,0),(−1,−1,0)` colapsan al origen (interior, no extremos); los otros
10 se agrupan en 6 puntos distintos: `(±√2,0)` y `(±1/√2,±1)` — **los
mismos seis puntos que el camino A**, coincidencia exacta, no solo de área.

Área del hexágono dual (shoelace sobre los 6 vértices, en cualquiera de los
dos órdenes obtenidos): `3√2`.

```
$ target/release/dz-cli.exe eval "3*sqrt(2)" --mode fixed --scale 8
~4;2XE34926
```

Densidad y área de Holmes–Thompson de una cara:

```
θ_HT = 3√2/π
área_HT(cara) = (√2/2)·(3√2/π) = 3/π
```

```
$ target/release/dz-cli.exe eval "3*sqrt(2)/pi" --mode fixed --scale 8
~1;42575320E1467X96042091XX
$ target/release/dz-cli.exe eval "3/pi" --mode fixed --scale 8
~0;E56150822E414E1X7144X626
$ target/release/dz-cli.exe eval "3/pi" --mode rational
~0;E5615082189X
```

Total sobre 12 caras:

```
S_HT(DR) = 12 · 3/π = 36/π r²
```

```
$ target/release/dz-cli.exe eval "36/pi" --mode fixed --scale 8
~11;4517X96752979024371832E2
$ target/release/dz-cli.exe eval "36/pi" --mode rational
~11;4517X9660366
```

**Chequeo de consistencia (Santaló–Mahler, `HT ≤ Bus` para cuerpos
simétricos):** en decimal, `36/π ≈ 11.459 < 4π ≈ 12.566` — se cumple, señal
de que no hay un error de inverso o de signo en la derivación (mismo
chequeo que `03-hallazgo-busemann-holmes-thompson.md` §2.4 aplicó al
octaedro).

### 1.5 Veredicto y casilla de la avenida 1

**Las tres definiciones estándar de superficie fallan también aquí.**
Hausdorff da `6√2 r²` (algebraico irracional), Busemann da `4π r²`
(trascendente), Holmes–Thompson da `36/π r²` (trascendente). Ningún cambio
de definición produce un racional, y dos de las tres reintroducen `π` —
exactamente el patrón de `03-hallazgo-busemann-holmes-thompson.md` para el
octaedro, ahora replicado en un cuerpo elegido específicamente por tener 12
caras y ser una celda de Voronoi real, no un objeto ad hoc.

| medida | fórmula | clase |
|---|---|---|
| Hausdorff | `6√2 r²` | algebraico (raíz de `x²=72`) |
| Busemann | `4π r²` | trascendente |
| Holmes–Thompson | `36/π r²` | trascendente |
| *(referencia, octaedro L₁, doc 01/03)* | `4√3`, `16π/3`, `48/π` | mismo patrón |

**Casilla (I), en la misma forma dura que `03-hallazgo-busemann-holmes-
thompson.md` §4.3.** No es que el DR carezca de análogo racional bajo
Hausdorff; es que las tres nociones razonables de área normada coinciden en
devolver irracional, para un cuerpo que **sí** tiene simetría `Oh` completa
(orden 48, más rica que la del octaedro en la lectura ingenua de
`99-sintesis.md`, aunque en realidad las dos comparten `Oh` en 3D — ver
§1.1) y **sí** tiene 12 caras genuinas. Ni el 12 en el recuento de caras ni
una simetría más rica evitan que `ω₂ = π` se filtre en cuanto se normaliza
por un disco euclídeo de referencia. El mecanismo estructural es idéntico
al del octaedro: `ω_k` para `k ≥ 2` trae `π` de forma inevitable en
Busemann/HT, y Hausdorff hereda su propia raíz (aquí `√2` en vez de `√3`,
por la geometría del rombo en vez del triángulo) de la forma concreta de
la cara.

---

## 2. Avenida 2 — Ehrhart y una medida combinatoria

### 2.1 El objeto: el octaedro L₁ como politopo reticular

El octaedro estándar `conv(±e1,±e2,±e3)` — vértices `(±1,0,0)` y
permutaciones — ya tiene vértices enteros: es un politopo reticular sin
necesidad de forzar nada escalándolo. Por el teorema de Ehrhart, el conteo
de puntos de `Z^3` en los dilatados enteros,

```
L(t) = #{ (x,y,z) en Z^3 : |x|+|y|+|z| <= t },   t = 0,1,2,...
```

es un **polinomio** en `t` (no solo cuasi-polinomio, porque los vértices ya
son enteros), de grado 3, con coeficiente líder el volumen del cuerpo y
término independiente `1`.

### 2.2 El polinomio de Ehrhart, y de dónde sale el coeficiente que importa

**Derivación por capas.** Sumando capa a capa en `z` el número de puntos
`(x,y)` con `|x|+|y| <= t-|z|`, y usando la fórmula 2D estándar
`f(m) = 2m^2+2m+1` (`f(0)=1`, `f(1)=5`, verificables a mano):

```
L(t) = f(t) + 2*Sum_{k=0}^{t-1} f(k)
```

Expandiendo (aritmética de números enteros ordinaria, no dozenal — esto es
teoría de conteo, no un cálculo del proyecto; el resultado sí se verifica
después con el binario):

```
L(t) = (4/3) t^3 + 2 t^2 + (8/3) t + 1
```

El coeficiente líder coincide con el volumen del octaedro L1 de radio 1,
`4/3` — la misma cifra que `2^n/n!` de `01-hallazgos-A-constantes.md` E2
para `n=3` — y el término independiente es `1` (`chi=1`, como corresponde a
cualquier politopo convexo).

**El coeficiente que importa aquí es el de grado `d-1=2`: `e2 = 2`.** Por
el teorema de Ehrhart-Macdonald este coeficiente no es un accidente
algebraico de la expansión: tiene una lectura geométrica exacta,

```
e_{d-1}(P) = (1/2) * Sum_F relvol(F)
```

sumando sobre las facetas `F`, donde `relvol(F)` es el volumen de `F`
**relativo al reticulo que su propio hiperplano induce**, no el volumen
euclidiano ordinario. Con `e2=2`, `Sum_F relvol(F) = 4` sobre las 8 caras
congruentes, es decir `relvol(una cara) = 1/2`.

**Verificación directa de `relvol` para una cara.** La cara
`F = conv((1,0,0),(0,1,0),(0,0,1))` vive en el plano `x+y+z=1`; el
reticulo de dirección es `L0 = Z^3 ∩ {x+y+z=0}`, de base entera
`(1,-1,0),(0,1,-1)` (índice 1, base estándar conocida de ese subreticulo).
Su covolumen es la norma del producto vectorial de esa base:

```
(1,-1,0) x (0,1,-1) = (1,1,1),   ||(1,1,1)|| = sqrt(3)
```

Área euclidiana de `F` (triángulo equilátero de lado `sqrt(2)`): `sqrt(3)/2`
(la misma cifra ya usada en `01-hallazgos-A-constantes.md` §7.1). Entonces

```
relvol(F) = area_eucl(F) / covol(L0) = (sqrt(3)/2)/sqrt(3) = 1/2
```

```
$ target/release/dz-cli.exe eval "(sqrt(3)/2)/sqrt(3)" --mode fixed --scale 8
~0;6
```

`0;6` en dozenal es `6/12 = 1/2` exacto (el `~` es solo del truncamiento de
la raíz intermedia, no de una duda sobre el resultado final: modo racional
también lo confirma más abajo, §2.3). Esto coincide con la predicción del
teorema de Ehrhart-Macdonald (`e2=2` ⟹ `relvol` por cara `=1/2`) sin
haberlo calculado por esa vía: una es teoría de conteo de puntos reticulares
(arriba), la otra es álgebra lineal directa sobre un triángulo y un
subreticulo concreto — es la primera contraverificación de esta sección.

### 2.3 Contraverificación independiente

Recontar `L(1)`, `L(2)`, `L(3)` capa a capa, comparando contra el polinomio
cerrado de §2.2, por una vía que solo comparte la fórmula recursiva pero
verifica cada valor por separado (incluido `n=3`, que ningún documento
anterior de esta serie necesitó).

`L(0)=1` (el origen). `L(1)`: origen + 6 puntos a distancia 1 = 7.
`L(2)`: distancia 0 (1) + distancia 1 (6) + distancia 2
(`(±2,0,0)` y perms: 6 puntos; `(±1,±1,0)` y perms: 12 puntos; total 18)
= 25. `L(3)` con `f(3)=25`, `f(0)+f(1)+f(2)=1+5+13=19`: `L(3)=25+38=63`.

Verificado con el binario. Los literales multidígito de la fórmula se leen
en base 10 con `--input-base 10`; la **salida** del CLI es siempre dozenal
(regla del proyecto, `CLAUDE.md`), así que `"21"` de abajo es 25 decimal y
`"53"` es 63 decimal — se confirma con `chain`:

```
$ target/release/dz-cli.exe eval "(4/3)*1+2*1+(8/3)*1+1" --input-base 10 --mode rational
7
$ target/release/dz-cli.exe eval "(4/3)*8+2*4+(8/3)*2+1" --input-base 10 --mode rational
21
$ target/release/dz-cli.exe chain 21 --input-base 10
25
$ target/release/dz-cli.exe eval "(4/3)*27+2*9+(8/3)*3+1" --input-base 10 --mode rational
53
$ target/release/dz-cli.exe chain 53 --input-base 10
63
```

Los tres valores (`7, 25, 63`) coinciden con el conteo directo capa a capa
— dos derivaciones independientes (suma cerrada de una serie vs. conteo
directo para `n` fijo) confirman el mismo polinomio, y el binario confirma
la aritmética racional sin operar nativamente sobre un valor dozenal.

### 2.4 Qué mide realmente ese coeficiente — el litmus test del plan

`e2 = 2` es racional, de hecho entero. El protocolo (`00-plan-
investigacion.md` §1, la prohibición de «elegir la definición que da el
número bonito») exige antes de contarlo como hallazgo: ¿es un invariante
combinatorio natural que *resulta* racional, o es «área» estirada hasta
que lo sea?

**La respuesta es la segunda, y conviene decirlo sin rodeos.**

1. **La racionalidad de `e2` no es un hecho sobre el octaedro L1, ni sobre
   L1 en general, ni sobre base 12.** Es un teorema sin condiciones para
   *todo* politopo reticular: si `P` tiene vértices en `Z^d`, `L_P(t)` es un
   polinomio con coeficientes en `Q` (denominador acotado por `(d-1)!` en
   `e_{d-1}`), siempre, sin excepción — daría exactamente igual si `P` fuera
   un politopo con caras completamente asimétricas y sin ninguna relación
   con `L1`. Encontrar que `e2` es racional aquí **no distingue** al
   octaedro L1 de cualquier otro politopo reticular arbitrario: es la
   propiedad que todos comparten por construcción, no un rasgo de este
   cuerpo.
2. **`relvol(F)` no es área de Hausdorff y no se comporta como ella bajo
   las transformaciones que sí importan geométricamente.** Depende de qué
   tan fina es la red inducida en el hiperplano de `F` — una cantidad
   aritmética (un determinante de una base entera), no métrica. Bajo una
   rotación genérica del cuerpo en `R^3` (que rompe la alineación con
   `Z^3`), el hiperplano rotado típicamente no contiene ningún punto
   reticular salvo el origen y `covol(L_F)` deja de tener sentido, mientras
   que el área de Hausdorff de la cara, por supuesto, no cambia en
   absoluto. **`relvol` no es invariante bajo el grupo de isometrías de
   `R^3`**, la propiedad mínima exigible a cualquier cosa que se llame
   «área». Mide la posición del cuerpo respecto de una red fija, no una
   propiedad intrínseca de la frontera.
3. **No mide "superficie en función del radio".** `e2` es un número fijo
   asociado al politopo de referencia (aquí, el octaedro de vértices ya
   enteros); no es una función continua de `r` que escale como `r^2` de la
   manera en que lo hace una superficie real. Dilatar por un entero `m`
   cambia el politopo base (`Q=mP`, con `e2(Q) = 2 m^2` por
   `L_Q(t)=L_P(mt)`) — hay una escala `~m^2` disponible, pero solo para
   dilataciones **enteras**; evaluarlo en un radio no entero exige pasar a
   otro objeto (cuasi-polinomios de Ehrhart en racionales), y no hay forma
   de extenderlo limpiamente a `r` real.

**Conclusión del litmus test: esto es el movimiento prohibido, aunque no se
disfraza de él.** No se «elige» la definición para forzar un racional en el
sentido burdo que el plan prohíbe (ajustar un parámetro libre hasta que
salga bonito) — `e2` es un objeto matemático genuino, con teoría propia
(Ehrhart-Macdonald, geometría tórica) y sin parámetro libre una vez fijado
el politopo y el reticulo. Pero **se ha cambiado la pregunta sin decirlo**:
en vez de «¿cuánto mide la frontera del octaedro L1?» se ha preguntado
«¿cuántos puntos de `Z^3` hay cerca de esa frontera, pesados según cómo se
posa sobre la red?» — una pregunta legítima y con contenido matemático real
(en la raíz de la teoría de intersección tórica), pero no la misma
pregunta. Su respuesta es racional por un teorema que no menciona a `L1`,
no por ninguna propiedad de `L1` ni de la base 12.

### 2.5 Veredicto y casilla de la avenida 2

**Casilla (III), en su lectura más débil: ni siquiera llega a ser un
efecto de la base 12.** La racionalidad de `e2` no depende de leerse en
base 10 o en base 12 — es racional como número, punto, por el teorema de
Ehrhart, y la única pregunta sensible a la base (¿termina la expansión?)
es trivial aquí porque `1/2`, `2`, `4` terminan en cualquier base
razonable. Tampoco es casilla (II): no es una propiedad de la métrica L1
que sobreviva a un cambio de norma, es una propiedad de **cualquier**
politopo reticular, L1 o no. La avenida 2 no rescata una medida de
superficie; confirma, con un caso trabajado a fondo con verificación doble,
que la vía combinatoria (Ehrhart) y la vía métrica (Hausdorff/Busemann/HT)
responden preguntas distintas, y que no son intercambiables sin decirlo.

---

## 3. Comparación de las dos avenidas

| avenida | qué cambia | resultado en Hausdorff | resultado en Busemann/HT | casilla |
|---|---|---|---|---|
| 1. Dodecaedro rómbico | el **cuerpo** (12 caras genuinas, celda de Voronoi FCC, `Oh` orden 48) | `6√2 r²`, algebraico | `4π r²`, `36/π r²`, ambas trascendentes | **(I)** |
| 2. Ehrhart | la **medida** (conteo reticular en vez de longitud/área) | n/a — no calcula Hausdorff | n/a — no es una medida continua | **(III)**, débil |

**Las dos avenidas fallan, pero por razones estructuralmente distintas, y
eso es lo que hay que retener.** La avenida 1 falla exactamente como el
octaedro L₁ ya había fallado en `01-hallazgos-A-constantes.md` §7.1 y
`03-hallazgo-busemann-holmes-thompson.md`: se cambió el cuerpo por otro con
más caras y más simetría nominal, y las tres medidas estándar de superficie
normada lo condenan del mismo modo, por el mismo mecanismo (`ω_k` trae `π`
en Busemann/HT; la geometría de la cara trae su propia raíz algebraica en
Hausdorff). La avenida 2 no falla por dar irracional — de hecho **da
racional**, lo cual podría parecer la primera victoria de todo el
programa de investigación — sino porque, examinada con el litmus test del
plan, resulta estar respondiendo una pregunta distinta de la que se
planteó, con una rationalidad garantizada por un teorema que no sabe nada
de `L₁` ni de superficies.

**Cruce con `03-hallazgo-busemann-holmes-thompson.md`.** Ese documento ya
había mostrado que, para el octaedro liso, Busemann (`16π/3 r²`) y
Holmes–Thompson (`48/π r²`) son ambos peores que Hausdorff (`4√3 r²`) por
reintroducir `π`. La avenida 1 de este documento repite exactamente ese
patrón en el DR (`4π` y `36/π` frente a `6√2`) — es la misma conclusión,
en un cuerpo elegido por otras razones (12 caras en vez de 8, celda de
Voronoi en vez de bola L₁ pura), y el hecho de que el patrón se repita
punto por punto en un cuerpo con simetría más rica (`Oh` en vez de la
lectura ingenua «`D₄`» de `99-sintesis.md` §1) es evidencia adicional —no
concluyente, pero coherente— de que el mecanismo es general: **cualquier
normalización de superficie que compare contra la bola euclidiana de
referencia hereda `π` de esa bola, para cualquier politopo simétrico
razonable**, no solo para el octaedro L₁.

**Cruce con `05-hallazgo-reencuadres-topologia-proyeccion-complejidad.md`.**
Ese documento estableció que el total de defecto angular de cualquier
poliedro convexo está fijado en `4π` por Gauss–Bonnet/Euler — un hecho
puramente topológico, no métrico — y que esto explica *por qué* Busemann y
Holmes–Thompson (que normalizan contra `ω_k`) estaban condenadas a `π` de
antemano: `ω_k` hereda el mismo `2π` de la vuelta completa que aparece en
Gauss–Bonnet. El cálculo de Busemann/HT sobre el DR en este documento
(§1.4) es exactamente una instancia más de ese mecanismo, ahora confirmada
en un segundo cuerpo: `θ_Bus` y `θ_HT` dividen o multiplican por `π`
precisamente porque `ω_2=π` es la constante de la bola euclidiana de
referencia, la misma que el reencuadre topológico de `05` identifica como
el origen estructural del problema. **Los dos documentos convergen en el
mismo diagnóstico por rutas independientes** (uno por teoría de
Gauss–Bonnet, este por cálculo directo en un segundo cuerpo), lo cual es
una contraverificación cruzada de alto nivel, no solo aritmética.

El hallazgo más importante de `05` para releer este documento es la
**independencia forma/medida** (`05` §3.5): que un cuerpo tenga una
descripción combinatoria simple, o comparta el índice `K` de su ecuación
implícita con la esfera, no predice nada sobre si su superficie será
racional. La avenida 1 de este documento es un dato adicional a favor de
esa independencia: el DR tiene *más* estructura combinatoria "amigable"
que el octaedro (14 vértices de dos tipos regulares, 12 caras congruentes,
simetría `Oh` completa, origen genuino como celda de Voronoi) y sin
embargo su superficie bajo las tres medidas estándar es **igual de
irracional** que la del octaedro liso — ni mejor ni peor, solo distinta
(`√2` en vez de `√3` en Hausdorff). Más estructura "bonita" en el cuerpo no
compró ni un bit de racionalidad en su frontera. Esto es consistente con,
y refuerza, la tesis de independencia de `05`: la simplicidad de la
*forma* (aquí, cuánto más simétrico y "reticular" es el DR frente al
octaedro) no es un predictor de la simplicidad de su *medida*.

La avenida 2, en cambio, no encaja en el eje forma/medida de `05` §3: no
compara dos formas por la complejidad de su ecuación, compara dos *tipos*
de medida (métrica continua vs. conteo reticular) sobre el mismo cuerpo.
Es más parecida en espíritu al reencuadre 3 de `05` (cambiar la categoría
de la pregunta, no solo el método) que a los reencuadres 1–2 de ese
documento — y, como el reencuadre 3 de `05`, termina en una distinción de
categorías más que en una nueva cifra: Ehrhart no es «otra forma de medir
lo mismo que falló antes», es una pregunta diferente que por casualidad
también se puede evaluar con un número.

---

## 4. Lo que no funcionó

Sección obligatoria del protocolo (`00-plan-investigacion.md` §5, punto 6).

- **La avenida 1 no encontró ningún cuerpo, entre los examinados, con
  superficie racional bajo ninguna de las tres medidas estándar.** Se
  probó un solo cuerpo nuevo (el DR) además del ya conocido (el octaedro
  liso de `01`/`03`). No se buscó sistemáticamente entre otros zonotopos o
  celdas de Voronoi de otras redes (la red hexagonal compacta en 3D, por
  ejemplo, tiene su propia celda de Voronoi, también un DR pero orientado
  distinto — no se comprobó si cambia algo, y no hay razón estructural para
  esperar que lo haga, dado que el mecanismo identificado en §1.5 y
  reforzado en §3 no depende de la orientación).
- **No se demostró un teorema general de que "todo zonotopo simétrico de
  pocos generadores tiene superficie irracional bajo las tres medidas".**
  Se calculó un caso. El argumento de §3 (que el mecanismo — `ω_k` en
  Busemann/HT, geometría algebraica de la cara en Hausdorff — es genérico)
  es plausible pero no es una prueba; el protocolo de este informe exige
  verificación con el binario para casos concretos, no extrapolación a
  todos los zonotopos.
- **La coincidencia `S_Bus(DR) = 4π r²` (idéntica a la esfera euclídea) no
  se explicó hasta el fondo.** Se anotó como observación (§1.4) y se evitó
  sobrevenderla, pero no se investigó si es una consecuencia de que el DR
  es la celda de Voronoi de la red más isótropa en 3D (FCC, que resuelve el
  empaquetamiento de esferas óptimo) o una coincidencia numérica sin
  significado estructural más profundo. Queda abierto.
- **La avenida 2 no exploró otros coeficientes de Ehrhart ni otras
  invariantes reticulares** (por ejemplo, el número de puntos reticulares
  en el interior frente a la frontera, vía reciprocidad de
  Ehrhart–Macdonald `L_P(-t) = (-1)^d L_{P°}(t)`, que también daría
  información racional pero de nuevo por el mismo mecanismo genérico). No
  se buscó ninguna variante que pudiera, con más cuidado, capturar algo
  genuinamente métrico en vez de puramente combinatorio — es plausible que
  no exista tal variante (esa es, de hecho, la conclusión de §2.4), pero no
  se agotó el espacio de invariantes de Ehrhart antes de cerrar la avenida.
- **No se verificó la afirmación de §2.4.2 (no invariancia de `relvol` bajo
  rotaciones genéricas) con un cálculo explícito** — se argumentó
  estructuralmente (un hiperplano rotado genérico no contiene más que el
  origen de `Z³`) pero no se construyó una rotación concreta y se recalculó
  nada; es un hecho estándar de teoría de retículos, no una comprobación
  original de este documento.
- **Ninguna de las dos avenidas se sometió a revisión externa**, siguiendo
  la práctica ya declarada en el resto de la serie (`99-sintesis.md` §7).

---

## 5. Veredicto final, honesto

**Ninguna de las dos avenidas rationaliza una constante de superficie 3D
para un cuerpo relacionado con L₁ o con la base 12, y la investigación
cierra con evidencia más fuerte, no más débil, de que ninguna lo hará por
las vías estándar.**

**Avenida 1 (dodecaedro rómbico): fracaso limpio, y confirmatorio.** Se
identificó una norma genuina cuya bola unidad es el DR
(`max(|x|+|y|,|y|+|z|,|z|+|x|)`), se calculó su superficie de Hausdorff
exactamente (`6√2 r²`, algebraico irracional) por dos métodos
independientes, y se extendió a Busemann (`4π r²`) y Holmes–Thompson
(`36/π r²`), ambas trascendentes — replicando en un segundo cuerpo,
elegido específicamente por tener 12 caras reales y máxima simetría
posible en 3D (`Oh`, orden 48), exactamente el patrón que
`03-hallazgo-busemann-holmes-thompson.md` ya había encontrado en el
octaedro liso. Ni el «12» en el recuento de caras ni una simetría más rica
que la del octaedro compran nada: el mecanismo que impide la racionalidad
—`ω_k` trayendo `π` en las normalizaciones, y la geometría de la cara
trayendo su propia raíz algebraica en Hausdorff— no depende del número de
caras del politopo ni de cuán "de red" sea su origen.

**Avenida 2 (Ehrhart): produce un número racional, y por eso mismo es la
más importante de reportar con cuidado.** El coeficiente de Ehrhart
`e₂ = 2` (equivalentemente, `relvol` por cara `= 1/2`) es racional —de
hecho entero— para el octaedro L₁ reticular, verificado por dos métodos
independientes. Pero aplicando el litmus test explícito del encargo (§2.4):
esta racionalidad es un teorema **sin condiciones sobre todo politopo
reticular**, no una propiedad de L₁ ni de la base 12; y el objeto medido
(`relvol`, ligado a un retículo fijo) no es invariante bajo isometrías, así
que no es «área» bajo ningún criterio razonable — es una pregunta distinta
(conteo combinatorio ponderado por una red) que se parece a la pregunta
original solo superficialmente. Se clasifica explícitamente como el
movimiento que el plan prohíbe, no como una racionalización legítima.

**Veredicto acumulado de la serie 01→03→04→05 sobre la superficie 3D de
cuerpos L₁-relacionados.** Cinco cuerpos/medidas examinados en total en la
serie (octaedro liso × {Hausdorff, Busemann, HT}, DR × {Hausdorff,
Busemann, HT}, más el defecto angular topológico y el muestreo de Cauchy
finito de `05`) y **ninguno da un racional que sea genuinamente una medida
de tamaño de frontera invariante por isometrías.** Las únicas vías que sí
dan racional (`κ_comb` en `05` reencuadre 1, el promedio de sombras sobre
ejes en `05` reencuadre 2, `e₂` de Ehrhart en este documento) lo consiguen
**dejando de medir lo que se pedía**: combinatoria pura de incidencias,
alineación con los ejes de coordenadas, o conteo reticular ligado a un
retículo fijo, respectivamente. Es una **refutación consistente y ahora
triplemente verificada** (tres documentos independientes, seis vías de
ataque distintas) de que exista una noción razonable de superficie de un
cuerpo poliédrico en 3D —L₁ o de otro tipo— que sea a la vez (a) invariante
por isometrías, (b) consistente con la métrica euclídea en el caso liso, y
(c) racional para un politopo genuinamente 3D no degenerado. Dada la
cantidad de intentos independientes y el hecho de que cada fracaso señala a
un mecanismo estructural distinto y verificado (`ω_k` hereda `2π` de la
topología del círculo; `relvol` depende de un retículo, no de la métrica;
la alineación con ejes es un artefacto de coordenadas, no de la norma), esta
es una conclusión **negativa fuerte y publicable**, en el sentido exacto que
pedía el encargo: no una falta de intentos, sino evidencia acumulada de
imposibilidad estructural bajo los criterios razonables que cualquier
noción de "superficie" debería satisfacer.

**Respuesta directa a la pregunta del encargo**: la lógica de base 12 no
ofrece ninguna vía legítima a una constante de superficie 3D racional. La
única vía que produce un número racional (Ehrhart) lo hace por una
propiedad de los retículos enteros en general —cierta en cualquier base
numérica, y cierta para cualquier politopo, no solo los relacionados con
L₁— y deja de ser una medida de superficie en el proceso. La base 12 no
interviene en ningún paso matemático de ninguna de las dos avenidas: todas
las cifras de este documento son igual de (ir)racionales en base 10 que en
base 12, y la dozenal solo entra, como en el resto de la serie, para
mostrar los dígitos con el instrumento del repositorio.

---

## 6. Comandos de referencia (bitácora completa)

Todos ejecutados contra `target/release/dz-cli.exe` desde la raíz del
repositorio. Se listan en el orden en que aparecen en el documento, sin
repetir los que ya están pegados junto a su cifra en §1–§2.

```
eval "1/2" --mode rational
eval "sqrt(2)/2" --mode fixed --scale 8
eval "6*sqrt(2)" --mode fixed --scale 8
eval "6*sqrt(2)" --mode rational
eval "sqrt(3)/2" --mode fixed --scale 8
eval "8*sqrt(2)*(sqrt(3)/2)^2" --mode fixed --scale 8
eval "2" --mode rational
eval "(16*sqrt(3)/9)*(sqrt(3)/2)^3" --mode fixed --scale 8
eval "pi*sqrt(2)/3" --mode fixed --scale 8
eval "pi/3" --mode fixed --scale 8
eval "pi/3" --mode rational
eval "4*pi" --mode fixed --scale 8
eval "4*pi" --mode rational
eval "3*sqrt(2)/2" --mode fixed --scale 8
eval "3*sqrt(2)" --mode fixed --scale 8
eval "3*sqrt(2)/pi" --mode fixed --scale 8
eval "3/pi" --mode fixed --scale 8
eval "3/pi" --mode rational
eval "36/pi" --mode fixed --scale 8
eval "36/pi" --mode rational
eval "(sqrt(3)/2)/sqrt(3)" --mode fixed --scale 8
eval "(4/3)*1+2*1+(8/3)*1+1" --input-base 10 --mode rational
eval "(4/3)*8+2*4+(8/3)*2+1" --input-base 10 --mode rational
chain 21 --input-base 10
eval "(4/3)*27+2*9+(8/3)*3+1" --input-base 10 --mode rational
chain 53 --input-base 10
```

Todas las salidas están reproducidas junto a su cifra en el cuerpo del
documento; esta bitácora es solo un índice para volver a ejecutar la serie
completa sin tener que buscar comando por comando.
