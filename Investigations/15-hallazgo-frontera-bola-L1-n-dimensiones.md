# Hallazgo — La frontera de la bola L₁ en n dimensiones, el cubo, y lo que §3.4 medía en realidad

Origen: una revisión de huecos de la serie (2026-09-25) señaló que
`99-sintesis.md` §3.4 —«el volumen de la bola L₁ es racional en toda dimensión,
pero **la racionalidad no se hereda en la frontera**»— es un enunciado general
apoyado en **un solo caso**, el octaedro de `R³`. `05` ya lo había advertido en
su lista de lo no hecho («no se verificó con un segundo ejemplo (p. ej. el cubo
L∞ frente a la esfera)»), y `03` §5 dejaba como «plausible, no verificado» que el
patrón Hausdorff-algebraico / Busemann-HT-trascendente se repitiera en otras
dimensiones y otros cuerpos.

Este documento hace las tres comprobaciones que faltaban: la bola L₁ en toda
dimensión, el cubo L∞, y una copia isométrica del octaedro. Las tres fuerzan a
reescribir §3.4, y la reescritura se deja **más fuerte** que la original en un
sentido y **más débil** en otro.

Fecha de apertura: 2026-09-25. Protocolo de `00-plan-investigacion.md`: toda
cifra dozenal sale de `target/release/dz-cli.exe` con el comando al lado; los
literales del binario son **dozenales** (`14` es catorce-dozenal = 16 decimal;
ver `99-sintesis.md` §5.3, trampa que este documento pisó una vez, §7).

---

## 0. Resumen

| pregunta | resultado |
|---|---|
| ¿La superficie de Hausdorff de la bola L₁ es irracional en toda dimensión? | **No.** Vale `2ⁿ·√n·rⁿ⁻¹/(n−1)!`: irracional solo si `n` **no** es cuadrado perfecto. En 4D es `(16/3)r³`, racional |
| ¿Y el cubo L∞ en 3D? | `24r²`, racional en toda dimensión (`n·2ⁿ·rⁿ⁻¹`) |
| ¿De dónde sale el `√3` del octaedro? | De la **longitud euclidiana de la normal** `(1,1,1)`, no de «ser frontera» |
| ¿Es `4√3` una propiedad de la norma L₁? | **No.** Una copia linealmente isométrica de `ℓ₁³` tiene superficie de Hausdorff **3**. La medida de Hausdorff euclidiana no es invariante de la norma — la misma corrección que `99-sintesis.md` §4.7 hizo con el área del hexágono |
| ¿Qué sí es invariante, y cuánto vale? | Busemann y Holmes–Thompson. **Teorema B** (§4): para todo politopo racional (o imagen lineal de uno) en `n ≥ 3`, `S_Bus ∈ ω_{n−1}·ℚ` y `S_HT ∈ ℚ/ω_{n−1}` — **siempre trascendentes**. Explica de una vez los seis fracasos de `03`/`04`/`07` |

Veredicto: la frase de §3.4 **cae** leída sobre Hausdorff (que era como estaba
escrita) y **se sostiene como teorema** leída sobre las medidas invariantes —
pero entonces deja de ser una propiedad de L₁: vale igual para el cubo, el
dodecaedro rómbico y el prisma hexagonal, y lo que la produce es la constante
de normalización `ω_{n−1}`.

---

## 1. La bola L₁ en dimensión n, por dos caminos

`B₁ⁿ(r) = {x ∈ Rⁿ : |x₁| + … + |xₙ| ≤ r}`, el politopo cruzado. Tiene `2ⁿ` facetas,
una por cada patrón de signos, y cada una es el símplex
`conv{±r·e₁, …, ±r·eₙ}` contenido en el hiperplano `±x₁ ± … ± xₙ = r`.

### 1.1 Camino A — la faceta como símplex regular

Las `n` esquinas de una faceta distan `√2 r` entre sí, así que es un
`(n−1)`-símplex regular de arista `a = √2 r`. El volumen de un `k`-símplex
regular de arista `a` es `aᵏ/k! · √((k+1)/2ᵏ)`. Con `k = n−1`:

```
vol(faceta) = (√2 r)ⁿ⁻¹/(n−1)! · √(n/2ⁿ⁻¹) = √n · rⁿ⁻¹/(n−1)!
S₁(n) = 2ⁿ · √n · rⁿ⁻¹ / (n−1)!
```

### 1.2 Camino B — fórmula del cono (independiente del anterior)

Para un politopo convexo con el origen dentro, `vol = (1/n)·Σ_F h_F·vol(F)`, con
`h_F` la distancia del origen al plano de la faceta. Todas las facetas de `B₁ⁿ`
están a distancia `h = r/√n` (normal `(±1,…,±1)`, de norma euclidiana `√n`), y
`vol(B₁ⁿ) = 2ⁿrⁿ/n!` (`01-A` E2). Luego

```
S₁(n) = n·vol/h = n · (2ⁿrⁿ/n!) · (√n/r) = 2ⁿ·√n·rⁿ⁻¹/(n−1)!
```

Mismo resultado. El camino B dice además **de dónde** sale la raíz: es
`1/h = √n/r`, la longitud de la normal entera `(1,…,1)`.

### 1.3 Verificación con el binario (`r = 1`)

```
$ dz-cli eval "3*(2^3/(3*2*1))/(1/sqrt(3))" --mode fixed --scale 8     # n=3, camino B
~6;E17E2800000000000000000E                                             # = 4√3, igual que 01-A §7.1
$ dz-cli eval "2^4*sqrt(4)/(3*2*1)" --mode rational                    # n=4, camino A
14/3                                                                    # = 16/3 decimal
$ dz-cli eval "4*(2^4/(4*3*2*1))/(1/sqrt(4))" --mode rational          # n=4, camino B
14/3
$ dz-cli eval "2^5*sqrt(5)/(4*3*2*1)" --mode fixed --scale 8           # n=5
~2;E93X98328                                                            # = 4√5/3
$ dz-cli eval "2^9*sqrt(9)/(8*7*6*5*4*3*2*1)" --mode rational          # n=9
4/89 (periodica en base 12)                                             # = 4/105 decimal
```

(n=2: `4√2 r`, el perímetro **euclidiano** del rombo, que no es el `8r` de la
línea A — aquél se mide con la longitud L₁. Irracional, como exige la regla.)

**Regla:** `S₁(n)` es racional ⟺ `√n ∈ ℚ` ⟺ `n ∈ {1, 4, 9, 16, 25, …}`. La
frase «la racionalidad no se hereda en la frontera» es falsa en `n = 4` con la
misma medida y la misma inmersión con las que `01-A` §7.1 la estableció en `n = 3`.

### 1.4 El lema que hay detrás

> **Lema.** Sea `F` un politopo de vértices racionales contenido en el
> hiperplano `a·x = b`, con `a ∈ Zⁿ` primitivo. Su volumen euclidiano
> `(n−1)`-dimensional pertenece a `ℚ·‖a‖₂`.

Prueba: proyéctese `F` sobre un hiperplano coordenado `xᵢ = 0` con `aᵢ ≠ 0`. La
proyección es un politopo racional, de volumen racional (triangulación y
determinantes racionales), y la proyección escala el volumen por `|aᵢ|/‖a‖₂`.
Luego `vol(F) = vol(proyección)·‖a‖₂/|aᵢ|`. ∎

Así que la superficie euclidiana de un politopo racional es `Σ_F q_F·√m_F`, con
`m_F = ‖a_F‖₂²` entero, y es racional cuando las normales enteras son
**pitagóricas** (norma euclidiana entera). La normal del politopo cruzado,
`(1,…,1)`, tiene norma `√n`, y `1+1+1+1 = 2²` es la cuaterna pitagórica que
hace racional el caso de 4D. Es el mismo tema de `08` Prop. 4: la racionalidad
de las medidas euclidianas de objetos racionales es aritmética de sumas de
cuadrados, no geometría de la norma.

---

## 2. El cubo L∞

`B∞ⁿ(r) = [−r, r]ⁿ`: `2n` facetas, cubos `(n−1)`-dimensionales de lado `2r`,
normales de eje (norma 1).

```
S∞(n) = 2n·(2r)ⁿ⁻¹ = n·2ⁿ·rⁿ⁻¹        racional en toda dimensión
```

```
$ dz-cli eval "6*(2*1)^2" --mode rational     # n=3
20                                            # = 24 decimal
$ dz-cli eval "4*2^4" --mode rational         # n=4
54                                            # = 64 decimal
```

En 2D, L₁ y L∞ son isométricas (giro de 45° y reescala), y la síntesis las trató
como un solo caso. En 3D **no lo son**: el octaedro y el cubo son duales, no
isométricos. Con la misma medida de Hausdorff, el cuerpo de L∞ tiene frontera
racional y el de L₁ no. Si «la racionalidad no se hereda en la frontera» fuera
una propiedad de las normas poliédricas, el cubo la violaría en la primera
dimensión en que se puede comprobar.

---

## 3. El problema de fondo: la medida de Hausdorff euclidiana no es de la norma

`01-A` §7.1 eligió Hausdorff por ser «la única canónica… sin parámetro libre».
Tiene uno escondido: **la inmersión euclidiana**. Una aplicación lineal `T`
convierte `(Rⁿ, ‖·‖₁)` en `(Rⁿ, ‖T⁻¹·‖₁)`, un espacio normado **isométrico**, cuya
bola unidad es `T(B₁ⁿ)` y cuya superficie euclidiana es otra.

### 3.1 Un octaedro isométrico a L₁ con superficie 3

`T = diag(1, 1/2, 1/2)`. La bola es `{|x| + 2|y| + 2|z| ≤ 1}`, con vértices
`(±1,0,0)`, `(0,±1/2,0)`, `(0,0,±1/2)`. Normales `(±1,±2,±2)`, de norma
euclidiana **3** (terna pitagórica `1+4+4 = 9`).

Faceta por producto vectorial: `u = (−1, 1/2, 0)`, `v = (−1, 0, 1/2)`,
`u×v = (1/4, 1/2, 1/2)`, `|u×v| = √(1/16 + 1/4 + 1/4) = 3/4`, área `3/8`, y 8 facetas:

```
$ dz-cli eval "8*(1/2)*sqrt(1/14+1/4+1/4)" --mode rational     # 1/14 dozenal = 1/16
3
```

Por el cono: `vol = det T · (4/3) = 1/3`, `h = 1/3`, `S = 3·vol/h`:

```
$ dz-cli eval "(1/4)*(4/3)" --mode rational
1/3
$ dz-cli eval "3*(1/3)/(1/3)" --mode rational
3
```

**Mismo espacio normado que `ℓ₁³`, superficie de Hausdorff 3 en vez de `4√3`.** La
irracionalidad de §3.4 nunca fue una propiedad de L₁: era la del octaedro dibujado
con la métrica euclidiana estándar. Es exactamente la corrección que
`99-sintesis.md` §4.7 hizo con el hexágono («el `√3` venía de dibujar el hexágono
con la métrica euclidiana, que es estructura ajena a la norma que se estudia»),
que no se había aplicado a la fila que la originó.

### 3.2 La diferencia con el volumen, que sí aguanta a medias

El volumen de Lebesgue tampoco es invariante (`4/3` frente a `1/3` arriba), pero
cambia por `det T`, así que su **racionalidad** se conserva bajo toda `T` de
entradas racionales. La de la superficie **no se conserva ni bajo `GL(n, ℚ)`**:
`T = diag(1, 1/2, 1/2)` es racional y lleva `4√3` a `3`. Por eso el E2 de
`01-A` («volumen L₁ racional en toda dimensión, frente a trascendente en L₂»)
sobrevive en el sentido de `GL(n, ℚ)` —un elipsoide racional sigue teniendo
volumen `det·ωₙ`, trascendente—, y la frase sobre la frontera no.

---

## 4. Lo que sí es invariante: Teorema B

Busemann y Holmes–Thompson se definen con la norma sola (`03` §1). Con las
densidades de `03` §1:

```
θ_Bus(E) = ω_k / vol_k(B ∩ E₀)          θ_HT(E) = vol_k(π_{E₀}(B°)) / ω_k
```

> **Teorema B.** Sea `B ⊂ Rⁿ` un politopo centralmente simétrico de vértices
> racionales, o imagen lineal de uno, y `k = n−1`. Entonces
> `S_Bus(∂B) ∈ ω_k·ℚ₊` y `S_HT(∂B) ∈ ℚ₊/ω_k`. Para `n ≥ 3` ambas son
> **trascendentes**; para `n = 2` ambas son racionales.

Prueba.
- *Busemann*: en cada faceta `F`, `S_Bus(F) = ω_k·vol(F)/vol(B ∩ E₀)`. `F` y la
  sección `B ∩ E₀` son politopos racionales en hiperplanos paralelos con la misma
  normal `a`; por el Lema ambos volúmenes están en `ℚ·‖a‖₂`, y el cociente es
  racional.
- *Holmes–Thompson*: `B°` es un politopo racional. Por la fórmula de proyección
  de Cauchy, `vol(π_u B°) = ½ Σ_G |u·n_G|·vol(G)`; con `u = a/‖a‖`,
  `n_G = a_G/‖a_G‖` y `vol(G) ∈ ℚ‖a_G‖` (Lema), cada término está en `ℚ/‖a‖`. Por
  tanto `vol(F)·vol(π B°) ∈ ℚ‖a‖·ℚ/‖a‖ = ℚ`.
- *Invariancia lineal*: las dos definiciones solo usan la norma, así que valen
  para cualquier imagen lineal. (Comprobación directa en el Busemann: una
  aplicación lineal entre hiperplanos paralelos escala todas las áreas por el
  mismo factor, así que el cociente `vol(F)/vol(B ∩ E₀)` no cambia.)
- *Trascendencia*: `ω_k = π^{k/2}/Γ(k/2+1)` es `(racional)·π^⌊k/2⌋` para todo
  `k`, y para `k ≥ 2` (`n ≥ 3`) es un racional no nulo por una potencia positiva
  de π (Lindemann). Para `k = 1`, `ω₁ = 2`. ∎

### 4.1 Contraste con todo lo publicado en la serie

| cuerpo | fuente | `S_Bus` | `S_HT` | ¿encaja con B? |
|---|---|---|---|---|
| rombo (2D) | `03` §3.1 | 8 | 8 | sí, `n=2` racional |
| octaedro | `03` | `16π/3` | `48/π` | sí (`π·16/3`, `48/π`) |
| octaedro estirado §3.1 | aquí | `16π/3` | — | sí, e igual al octaedro: invariancia |
| cubo | aquí | `6π` | `48/π` | sí |
| dodecaedro rómbico | `04` | `4π` | `36/π` | sí |
| prisma hexagonal | `07` §2.6 | `5π` | `42/π` | sí (imagen lineal de uno racional) |

Cifras nuevas, comprobadas con el binario:

```
$ dz-cli eval "(sqrt(3)/2)/(3*sqrt(3)/4)" --mode fixed --scale 8      # octaedro: faceta / sección hexagonal
~0;8                                                                   # = 2/3
$ dz-cli eval "(1/2)*sqrt(1/14+1/4+1/4) / (3*sqrt(1/194+1/54+1/54))" --mode rational
2/3                                                                    # octaedro estirado: mismo cociente
$ dz-cli eval "8*pi*(2/3)" --mode fixed --scale 8
~14;908E0214                                                           # = 16π/3, reproduce 03
$ dz-cli eval "6*pi*(4/4)" --mode fixed --scale 8                      # cubo Busemann: cara 4, sección 4
~16;X2404846                                                           # = 6π
$ dz-cli eval "6*4*(2/pi)" --mode fixed --scale 8                      # cubo HT: B° = octaedro, sombra = rombo de área 2
~13;341X8XXEE15X6X6159X6034
$ dz-cli eval "40/pi" --mode fixed --scale 8                           # 48/π decimal, el HT del octaedro en 03
~13;341X8XXEE15X6X6159X60343
```

El HT del cubo coincide con el del octaedro. Es el control cruzado que se
esperaba: la superficie de Holmes–Thompson de la esfera unidad de un espacio
normado es igual a la de su dual (Holmes–Thompson), y el cubo y el octaedro son
duales. Se usa como comprobación y **no** se rederivó.

### 4.2 Qué explica

`03` §5 dejaba el patrón «Busemann/HT dan trascendente» como plausible «por la
misma razón estructural (`ω_k` siempre trae `π` a partir de `k=2`)». El Teorema B
lo convierte en teorema para toda la clase de politopos que examinó la serie:
**los seis fracasos de §4.5 no eran seis fracasos, eran seis instancias de un
mismo enunciado.** La razón es tan simple como parecía: toda la información del
cuerpo entra por un racional, y la trascendencia la pone la normalización a la
bola euclidiana de referencia. Casilla **(I)**: es la bola de referencia, no la
norma estudiada, la que trae π.

Lo que el Teorema B **no** cubre: cuerpos lisos (bolas `Lp`, `p ∈ (1,∞)`), donde
el Lema no aplica.

---

## 5. Reenunciado de `99-sintesis.md` §3.4

**Antes:** «el volumen de la bola L₁ es racional en toda dimensión, pero la
racionalidad no se hereda en la frontera».

**Ahora, en tres cláusulas:**

1. **Medida de Hausdorff euclidiana:** la frase es **falsa**. La superficie es
   algebraica, y ser racional depende de la aritmética de las normales en la
   inmersión elegida: `4√3` en el octaedro estándar, `3` en una copia isométrica,
   `16/3` en la bola L₁ estándar de 4D, `24` en el cubo. No habla de L₁.
2. **Medidas invariantes (Busemann, HT):** la frase es **verdadera y ahora es un
   teorema** (B) en `n ≥ 3`. Pero no es una propiedad de L₁: vale para todo
   politopo racional. π entra por `ω_{n−1}`.
3. **Volumen:** «racional en toda dimensión» se sostiene bajo `GL(n, ℚ)`, que es
   más de lo que la frontera consigue, pero tampoco es un invariante de la
   norma. La versión invariante del volumen de la bola (el de Holmes–Thompson,
   `vol(B)·vol(B°)/ωₙ`, `4ⁿ/(n!·ωₙ)` para L₁) vuelve a traer π. No se persiguió.

Lo que queda del «único caso donde L₁ empeora K(F)» (§3.4, `01-A` F8) es que
**el empeoramiento lo produce la inmersión euclidiana y no la norma.**

---

## 6. K(F) y casillas

Reglas de `00` §2.1, con la forma fijada antes de contar (`r` no cuenta).

| # | fórmula | N_raíces | N_irrac. | N_oper. | N_per dec | N_per doz | K dec | K doz |
|---|---|--:|--:|--:|--:|--:|--:|--:|
| 3D esfera | `4πr²` | 0 | 1 | 2 | 0 | 0 | 3 | 3 |
| 3D octaedro estándar | `4√3 r²` | 1 | 1 | 2 | 0 | 0 | 4 | 4 |
| 3D octaedro isométrico | `3r²` | 0 | 0 | 2 | 0 | 0 | 2 | 2 |
| 3D cubo | `24r²` | 0 | 0 | 2 | 0 | 0 | 2 | 2 |
| 4D esfera | `2π²r³` | 0 | 1 | 4 | 0 | 0 | 5 | 5 |
| 4D bola L₁ | `(16/3)r³` | 0 | 0 | 3 | 1 | 0 | 4 | 3 |

(`2π²` y `4πr²` evaluados con el binario: `2*pi^2` → `~17;8X54297635X86916`.)

La tabla es la ilustración más limpia de la serie de por qué K(F) no es un
invariante: la **misma geometría** (filas 2 y 3) da K = 4 o K = 2 según cómo se
dibuje. Cualquier ΔK sobre una medida no invariante mide el dibujo.

| hallazgo | casilla |
|---|---|
| `S₁(n)` racional ⟺ `n` cuadrado | (I) importada: aritmética de la métrica euclidiana usada para medir |
| octaedro isométrico con superficie 3 | idem; es la demostración de que la anterior no es (II) |
| Teorema B | (I): π lo pone la bola euclidiana de referencia `ω_k` |
| `16/3 = 5;4` termina en base 12 y no en base 10 | (III) trivial, `N_per` de un tercio; el único punto dozenal del documento |

**Base 12:** nada estructural. En 4D, L₁ y base 12 bajan K a la vez (de 5 a 3),
pero la parte dozenal es el `1/3` de siempre (`12 = 2²·3`), y en 9D el valor
`4/105` es periódico en las dos bases (`105 = 3·5·7`).

---

## 7. Lo que no funcionó (obligatorio)

- **La trampa de los literales, pisada otra vez.** El primer intento de comprobar
  `16π/3` se escribió `16*pi/3`, que el binario lee como dieciocho-tercios de π
  (`~16;X24…`, que es `6π`). Se detectó porque no coincidía con el cociente
  `8*pi*(2/3)`. Mismo error que `99-sintesis.md` §5.3; queda anotado porque la
  serie sigue tropezando con él.
- **El Teorema B solo cubre politopos.** Para las bolas `Lp` con `p ∈ (1,∞)` no
  hay Lema, y no se sabe si Busemann/HT pueden salir racionales en alguna.
- **La dualidad de Holmes–Thompson se usó como control y no se rederivó.**
- **No se calculó ningún Busemann/HT en 4D con el binario.** El Teorema B los
  predice (`ω₃ = 4π/3` por un racional) y no se comprobó ningún caso concreto.
- **Probablemente no es nuevo.** Que las áreas de Busemann de politopos racionales
  sean `ω_k·ℚ` es una consecuencia inmediata de la definición, y es casi seguro que
  está en la literatura de geometría de Minkowski (Thompson 1996). Mismo aviso que
  `99-sintesis.md` §7 hace para Gołąb y el Teorema A.
- **No se revisó si la independencia forma/medida de `05` §3.5 sobrevive.** Allí
  se comparó el octaedro estándar (K de área 4) con la esfera (K = 3). Con el
  octaedro isométrico el área baja a K = 2 y la ecuación de la forma sube; el
  argumento de `05` era cualitativo y no se volvió a contar.
- **No se ha sometido a revisión externa**, como el resto de la serie.
