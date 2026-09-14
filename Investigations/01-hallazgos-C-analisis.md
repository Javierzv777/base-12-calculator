# Hallazgos — Línea C: Análisis

Preguntas 6, 8 y 9 de `planteamientos.md`. Protocolo: `00-plan-investigacion.md`.
Cifras dozenales comunes: `02-referencia-medida.md`.

Esta es la **línea de falsación**. El plan (§3) dice que es «donde la hipótesis
puede morir» y que es la más importante aunque la que menos promete. El informe
está escrito con ese encargo: la mayoría de los hallazgos son de casilla (I) —
*eso es el hallazgo, no un fracaso* — y la sección «lo que no funcionó» se lee
**simétricamente**: incluye los sitios donde esperaba que L₁ fallara y no falló.

Instrumento: `target/release/dz-cli.exe`. Todos los numerales dozenales que
aparecen abajo llevan el comando que los produjo. Ninguno se convirtió a mano.

> **Trampa de literales.** En `dz-cli` un literal desnudo se lee en base 12.
> `1/24` es un veintiochavo, no un veinticuatroavo. Donde necesito constantes
> decimales uso `--input-base 10` y lo digo; donde uso literales dozenales
> (`14` = dieciséis, `368` = 512) lo verifico con el propio binario.

---

## 0. Resumen ejecutivo

| # | hallazgo | casilla | efecto sobre H₁ |
|---|---|---|---|
| C1 | `\|zw\|₁ ≠ \|z\|₁\|w\|₁` (contraejemplo `z=w=1+i`), con defecto multiplicativo agudo en `[1/2, 1]` | **(I)** | destruye la forma polar y con ella el mapa exponencial |
| C2 | Las multiplicaciones que son isometrías de L₁ forman **Z/4**, no S¹ | **(I)** | el grupo circunferencia continuo es euclidiano y solo euclidiano |
| C3 | `lim \|zⁿ\|₁^{1/n} = \|z\|₂` (fórmula de Gelfand): L₂ se **reconstruye** desde L₁ | **(I)**, la forma más fuerte | L₂ no es una convención: la estructura multiplicativa de ℂ la selecciona |
| C4 | L₂ es la única Lᵖ de producto interno (Jordan–von Neumann) ⇒ no hay Plancherel, ni laplaciano autoadjunto, ni equivariancia rotacional | **(I)** | confina H₁ fuera del análisis de Hilbert |
| C5 | La transformada de la bola L₁ en 2D **sí** es elemental: `4(cos η − cos ξ)/(ξ² − η²)` | **(II)** | **refuta la segunda cláusula de P4** |
| C6 | Pero es 2D-específica: en nD la forma cerrada tiene O(n) términos y un arreglo de singularidades removibles, frente a **una** Bessel para todo n | **(I)** | la ganancia de C5 es un accidente de dimensión baja |
| C7 | La transformada del politopo decae con exponente **dependiente de la dirección** (1 en normales a caras, 2 genérico), frente a un único (n+1)/2 en L₂ | **(I)** | «más discreto» se paga en anisotropía, no es gratis |
| C8 | El principio del módulo máximo vale **exacto** para `\|f\|₁`, sin pérdida de constante | **(II)** | esperaba que fallara; no falla |
| C9 | Variación total anisótropa: minimización exacta por corte mínimo en grafo, esparsidad por vértices del politopo, preservación de bordes | **(II)** con ganancia real | **el L₁ de H₁ gana aquí, y gana de verdad** |
| C10 | La raíz cuadrada del tiempo de extinción del flujo por curvatura **sobrevive** al cambio de métrica | **(II)** | esperaba que L₁ la eliminara; no la elimina |
| C11 | `N_periodicos = 0` en **todas** las fórmulas de esta línea, en las dos bases | **(III) nulo** | **confirma P5 desde esta línea**: el factor dozenal de H₁ no aporta nada aquí |

**Veredicto global de la línea C: H₁ queda CONFINADA, no refutada.** Su factor
L₁ es genuinamente productivo en una familia identificable (optimización
convexa lineal por tramos, esparsidad, energías cristalinas) y estructuralmente
imposible en otra (todo lo que pasa por producto interno, rotación o
holomorfía). Su factor dozenal **no aparece en ningún punto de esta línea**.

Y el hallazgo que unifica el informe:

> **La anisotropía de la bola L₁ es una sola cosa con dos caras.** Que el
> politopo cruzado tenga solo `2n` vértices, alineados con los ejes, es
> *exactamente* lo que hace que los óptimos de un programa lineal sean
> esparsos (C9) y *exactamente* lo que impide que exista un grupo de rotaciones
> (C2), una transformada radial (C6) o un ángulo (C4). No son dos fenómenos que
> haya que sopesar: son el mismo teorema leído en dos direcciones. Por eso H₁
> es confinable con precisión en vez de simplemente «parcialmente cierta».

---

## 1. Pregunta 6 — Números complejos bajo L₁

### 1.1 Enunciados

**L₂.** Para `z = x + iy`, `|z|₂ = √(x² + y²)`. Entonces:

1. `|z|₂² = z z̄` — el módulo al cuadrado **es** la forma norma de la extensión
   ℚ(i)/ℚ;
2. `|zw|₂ = |z|₂ |w|₂` — multiplicatividad (identidad de Brahmagupta–Fibonacci);
3. de (2) y de la aditividad del argumento sale la **forma polar**
   `z = r e^{iθ}` como isomorfismo de grupos `ℂ* ≅ ℝ₊ × S¹`;
4. `θ ↦ e^{iθ}` es el subgrupo uniparamétrico compacto, y su imagen es
   exactamente `{|z|₂ = 1}`.

**L₁.** `|z|₁ = |x| + |y|`. (1) no tiene análogo: `|x| + |y|` no es una forma
cuadrática — no es siquiera polinómica. (2) **falla**. (3) y (4) caen con ella.

### 1.2 C1 — El contraejemplo, y qué se rompe exactamente

Tómese `z = w = 1 + i`. Entonces `zw = (1+i)² = 2i`.

```
|z|₁ = |1| + |1| = 2        |w|₁ = 2        |z|₁ |w|₁ = 4
|zw|₁ = |0| + |2| = 2
```

`2 ≠ 4`. La multiplicatividad del módulo es falsa en L₁, con un factor 2 de
error en el ejemplo más pequeño posible.

Esto **no es una elección desafortunada de norma**: es imposible por
clasificación. Los datos:

- **Cota superior (submultiplicatividad), y es estricta en general.** Con
  `z = x+iy`, `w = a+ib`, `zw = (xa − yb) + i(xb + ya)`, luego
  `|zw|₁ = |xa − yb| + |xb + ya| ≤ |x||a| + |y||b| + |x||b| + |y||a| =
  (|x|+|y|)(|a|+|b|) = |z|₁|w|₁`. Así que `(ℂ, |·|₁)` **sí** es un álgebra de
  Banach, con `|1|₁ = 1`. Esto sobrevive: casilla (II).
- **Cota inferior aguda.** De `|·|₂ ≤ |·|₁ ≤ √2 |·|₂` y de la multiplicatividad
  de `|·|₂`: `|zw|₁ ≥ |zw|₂ = |z|₂|w|₂ ≥ (|z|₁/√2)(|w|₁/√2) = ½ |z|₁|w|₁`.
  Es decir

  ```
  ½ ≤ |zw|₁ / (|z|₁ |w|₁) ≤ 1       para todo z, w ≠ 0
  ```

  y **ambos extremos se alcanzan**: el 1 con `w = 1`, el ½ con el contraejemplo
  `z = w = 1+i` de arriba. El «defecto multiplicativo» de L₁ está por tanto
  medido exactamente, y su valor extremo es ½ — que en dozenal es un dígito:

  ```
  $ target/release/dz-cli.exe eval "1/2" --mode fixed --gates
  0;6
  gates: cycle=84480 min=75272 max=60422 lit=46944 thr=15936 total=283054 depth=1312 recon=0
  ```

  (Que `½` se escriba `0;6` y termine también en decimal es casilla (III) y
  **nula**: `0;6` y `0.5` son igual de simples. No lo cuento como ganancia.)
- **Por qué es imposible arreglarlo.** Una norma multiplicativa sobre ℝⁿ que
  provenga de una forma cuadrática obliga a que la forma sea de composición, y
  el **teorema de Hurwitz** dice que las álgebras de composición sobre ℝ solo
  existen en dimensión 1, 2, 4 y 8, con la forma «suma de cuadrados». En
  dimensión 2 la forma multiplicativa es `x² + y²` y **es única**. Así que
  `|·|₂` no es una opción entre varias: es la única norma en ℂ compatible con
  el producto. Casilla **(I)**, y del tipo duro: no «no encontré análogo», sino
  «un teorema de clasificación prohíbe que exista».

**Cadena de lo que se rompe, en orden.**

1. Muere `|zw| = |z||w|`.
2. Muere la forma polar *como descomposición de grupo*. La aplicación
   `z ↦ (|z|₁, z/|z|₁)` sigue siendo una biyección
   `ℂ* → ℝ₊ × {|·|₁ = 1}` — el rombo parametriza igual de bien — pero **no es
   un homomorfismo**: el radio no se multiplica. Queda una *parametrización*,
   no una descomposición. Esto es, punto por punto, el mismo veredicto que la
   línea B registra para P3 en la trigonometría taxicab, y no es coincidencia:
   es el mismo hecho (la rotación no es isometría de L₁) visto desde el álgebra
   en vez de la geometría.
3. Muere la lectura geométrica de `e^{iθ}`. La exponencial compleja **sigue
   existiendo y siendo entera** (la serie `Σ zⁿ/n!` es un objeto algebraico y
   topológico, y todas las normas de ℝ² son equivalentes, así que converge
   igual en `|·|₁`) — pero deja de tener nada que ver con la esfera unidad de
   la norma:

   ```
   |e^{iθ}|₁ = |cos θ| + |sin θ| ∈ [1, √2]
   ```

   El mínimo 1 se alcanza en los ejes (`θ = 0, π/2, …`) y el máximo `√2` en las
   diagonales (`θ = π/4, …`). Es decir: **el subgrupo uniparamétrico `e^{iθ}`
   no está contenido en ninguna esfera de L₁**, y por tanto la identidad de
   Euler pierde su contenido geométrico. Se conserva su contenido analítico.

   ```
   $ target/release/dz-cli.exe eval "sqrt(2)" --mode fixed --scale 8 --gates
   ~1;4E79170X
   gates: cycle=4104408 min=6480064 max=4806845 lit=3292176 thr=757380 total=19440873 depth=15219 recon=0
   ```

   (coincide con `02-referencia-medida.md` §2, que es el control de que el
   instrumento da hoy lo mismo.) El máximo de `|e^{iθ}|₁` cuesta 19.440.873
   puertas a ocho dígitos; el mínimo es `1` y es gratis. Esa asimetría de coste
   *dentro de la misma curva* es la forma cuantitativa de decir que la curva no
   es una esfera de L₁.
4. Muere el logaritmo *como inverso del módulo-argumento*. `log z = log|z| + i
   arg z` es precisamente la descomposición polar escrita aditivamente; sin (2)
   no hay `log|z|₁` con propiedad de homomorfismo. `log` sigue existiendo como
   función holomorfa multivaluada — otra vez: el objeto analítico sobrevive, el
   diccionario geométrico no.

### 1.3 C2 — El grupo circunferencia colapsa a Z/4

Pregunta precisa: ¿qué `u ∈ ℂ*` actúan por multiplicación como isometrías de
`(ℝ², |·|₁)`?

Una aplicación lineal es isometría de `|·|₁` **si y solo si** es una matriz de
permutación con signos (porque debe permutar los puntos extremos del politopo
cruzado, que son exactamente `±e₁, ±e₂`). La multiplicación por `u = a + ib` es
la matriz `[[a, −b], [b, a]]`. Las permutaciones con signo de esa forma son
cuatro:

| `u` | matriz |
|---|---|
| `1` | `[[1,0],[0,1]]` |
| `i` | `[[0,−1],[1,0]]` |
| `−1` | `[[−1,0],[0,−1]]` |
| `−i` | `[[0,1],[−1,0]]` |

Las reflexiones `[[1,0],[0,−1]]` y compañía **sí** son isometrías de L₁, pero
no son ℂ-lineales (son conjugado-lineales), así que no son multiplicaciones.

**Conclusión.** El grupo de multiplicaciones L₁-isométricas es
`{1, i, −1, −i} ≅ ℤ/4`. El grupo de isometrías lineales completo es el diedral
de orden 8 (simetrías del rombo/cuadrado), de las cuales la mitad son
conjugado-lineales.

```
S¹  (continuo, dimensión 1, compacto conexo)   ⟶   ℤ/4  (finito)
```

Esto es la versión honesta y no retórica de «π ↦ 4» en esta pregunta: lo que se
vuelve 4 no es una constante, es **el orden de un grupo**. Y el precio es
total: no hay subgrupo uniparamétrico, luego no hay generador infinitesimal,
luego no hay `d/dθ`, luego no hay ecuación diferencial del movimiento circular,
luego no hay funciones trigonométricas con fórmulas de adición (P3 de la línea
B, por esta otra vía). Casilla **(I)**.

En nD el mismo cálculo da el grupo hiperoctaédrico de orden `2ⁿ n!` — finito
para todo n, frente a `SO(n)` de dimensión `n(n−1)/2`.

### 1.4 C3 — L₂ se recupera desde L₁: el resultado más fuerte de la línea

Lo anterior podría leerse como «L₁ y L₂ son dos elecciones y una es más cómoda».
No lo son, y esto lo demuestra.

Para cualquier norma `N` en ℂ submultiplicativa, la **fórmula del radio
espectral de Gelfand** da `lim_n N(zⁿ)^{1/n} = ρ(z)`. Calculémoslo para
`N = |·|₁`: de `|·|₂ ≤ |·|₁ ≤ √2|·|₂` y `|zⁿ|₂ = |z|₂ⁿ`,

```
|z|₂  ≤  |zⁿ|₁^{1/n}  ≤  2^{1/(2n)} |z|₂   ⟶   |z|₂
```

**El módulo euclidiano es el radio espectral de la norma L₁.** No es que L₂ sea
preferible: es que si se parte de L₁ y se pregunta qué norma es compatible con
el producto, *la respuesta que sale es L₂*. Cualquier norma en ℂ devuelve `|·|₂`
por este límite. Casilla **(I)** en su forma más fuerte: L₂ no es sustituible
porque es **reconstruible**.

*Matiz (revisión del 2026-09-13).* Precisamente porque cualquier norma devuelve
`|·|₂`, el límite no dice nada específico de L₁: es inmediato por equivalencia
de normas en dimensión finita, como muestra la acotación de arriba. El contenido
de fondo es clásico y más fuerte que el límite: `|·|₂` es el **único valor
absoluto multiplicativo** de ℂ que extiende el de ℝ (Ostrowski; Gelfand–Mazur).
El límite lo ilustra; el mérito de «L₂ está determinada» es de ese teorema.

Verificación explícita y medida con `z = 1 + i`. Potencias: `z² = 2i`,
`z³ = −2+2i`, `z⁴ = −4`, `z⁵ = −4−4i`, `z⁶ = −8i`, `z⁷ = 8−8i`, `z⁸ = 16`.
Sus módulos L₁ son `2, 2, 4, 4, 8, 8, 16, 16`, o sea

```
|(1+i)ⁿ|₁ = 2^⌈n/2⌉        mientras que      |(1+i)ⁿ|₂ = 2^{n/2}
```

luego `|zⁿ|₁^{1/n} = 2^{⌈n/2⌉/n}`, que vale **exactamente `√2` para todo n par**
y baja hacia `√2` por arriba en los impares. Medido:

```
$ target/release/dz-cli.exe eval "2^4" --mode fixed          # comprueba que el literal dozenal 14 es dieciséis
14
$ target/release/dz-cli.exe eval "2^9" --mode fixed          # y que 368 es 512
368
$ target/release/dz-cli.exe eval "15" --input-base 10 --mode fixed   # y que diecisiete es 15
13
```

Cuidado: `15` decimal es `13` dozenal, no `15`. Rehecho con los literales
correctos (`n = 7` usa `2^(4/7) = exp(ln 16 / 7)`, y dieciséis es `14` dozenal;
`n = 17` usa `2^(9/17) = exp(ln 512 / 17)`, con 512 = `368` y diecisiete = `15`):

| n | `\|zⁿ\|₁^{1/n}` | valor dozenal | comando |
|--:|---|---|---|
| 7 | `2^{4/7}` | `~1;59E96E21` | `eval "exp(ln(14)/7)" --mode fixed --scale 8` |
| 17 | `2^{9/17}` | `~1;53X113X5` | `eval "exp(ln(368)/15)" --mode fixed --scale 8` |
| par | `√2` exacto | `~1;4E79170X` | `eval "sqrt(2)" --mode fixed --scale 8` |

`1;59E9… > 1;53X1… > 1;4E79…`: convergencia monótona por arriba hacia `√2`,
tal como predice la cota. El límite no es un número nuevo ni «dozenalmente
limpio»: es `√2`, irracional en toda base.

### 1.5 C4 (parcial) — lo demás de la teoría de funciones

| propiedad | ¿sobrevive a `\|·\|₁`? | casilla | por qué |
|---|---|---|---|
| `\|z\|² = z z̄` | **no** | (I) | `\|x\|+\|y\|` no es una forma cuadrática; Hurwitz lo prohíbe |
| multiplicatividad | **no** (defecto exacto en `[½,1]`) | (I) | C1 |
| forma polar como homomorfismo | **no** | (I) | consecuencia de C1 |
| `e^{iθ}` en la esfera unidad | **no** (`∈ [1,√2]`) | (I) | C1 |
| grupo circunferencia | **no** (→ ℤ/4) | (I) | C2 |
| ángulo, ortogonalidad | **no** | (I) | sin producto interno no hay ángulo; la ortogonalidad de Birkhoff–James existe pero **no es simétrica**, y por el teorema de James una norma en la que sea simétrica (dim ≥ 3) es euclidiana |
| conformidad; grupo conforme local de dimensión infinita en 2D | **no** | (I) | «conforme» significa «preserva ángulos L₂»; sin ángulos el enunciado no es ni falso, es mal planteado. El grupo de semejanzas de `(ℝ², L₁)` es 3-paramétrico con componente finita, frente al grupo conforme local euclidiano en 2D, que es de dimensión infinita (toda función holomorfa) |
| desigualdad triangular, completitud, compacidad, propiedad | **sí** | (II) | equivalencia de normas en dimensión finita |
| `ℂ` álgebra de Banach | **sí** | (II) | submultiplicatividad, C1 |
| teorema fundamental del álgebra | **sí** | (II) | solo usa continuidad y propiedad de `N`; ver «lo que no funcionó» |
| Liouville, aplicación abierta, estimaciones de Cauchy | **sí**, con constantes en `[1,√2]` | (II) | idem |
| **principio del módulo máximo** | **sí, exacto** | (II) | C8, abajo. Éste me sorprendió |

### 1.6 C8 — El principio del módulo máximo vale exacto para `|f|₁`

Esperaba una pérdida de constante `√2` y no la hay.

`|f|₁ = |Re f| + |Im f| = max(|Re f + Im f|, |Re f − Im f|)`, y como
`Re((1−i)f) = Re f + Im f` y `Re((1+i)f) = Re f − Im f`,

```
|f|₁ = max{ ±Re((1−i)f),  ±Re((1+i)f) }
```

es el máximo de **cuatro funciones armónicas** (la parte real de una función
holomorfa es armónica). El máximo de finitas funciones armónicas es
subarmónica, luego `|f|₁` es subarmónica y cumple el principio del máximo sin
constante.

La versión **estricta** también: si `|f|₁` alcanza un máximo interior en `z₀`,
la rama `h_j` que lo realiza cumple `h_j(z₀) ≥ h_i(z) ≥ h_j(z)` para todo `i, z`,
luego `h_j` alcanza su propio máximo interior, luego `h_j ≡ cte`, luego
`Re((1∓i)f) ≡ cte`, luego `f ≡ cte`.

Generalización gratis: el argumento no usa nada de `|·|₁`. Para **cualquier**
norma `N` en ℝ², `N(f) = sup_{ℓ ∈ bola dual} ℓ(f)` es supremo de funciones
armónicas, luego subarmónica. **El principio del módulo máximo es
independiente de la norma.** Casilla (II), y es la prueba de que no todo lo que
parece euclidiano lo es: aquí L₂ entraba solo por costumbre notacional.

### 1.7 K(F) para la pregunta 6

Aquí hay que aplicar la regla 3 del encargo y **no forzar un número**.

| fórmula | K(F) L₂ | K(F) L₁ |
|---|---|---|
| `\|z\| = √(x²+y²)` vs `\|z\|₁ = \|x\|+\|y\|` | raíces 1, irrac 0, ops 3 (`^,^,+`), per 0 → **4** | raíces 0, irrac 0, ops 3 (`\|·\|,\|·\|,+`), per 0 → **3** |
| forma polar `z = r e^{iθ}` | raíces 0, irrac 1 (`e`), ops 3, per 0 → **4** | **no existe.** No hay descomposición polar multiplicativa en L₁ (C1). K(F) de un objeto inexistente no es 0 ni ∞: es una casilla vacía |
| `e^{iθ} = cos θ + i sin θ` | ops 2, per 0 → **2** | **no existe** como identidad de un subgrupo de la esfera unidad L₁ |

La primera fila es la única comparación legítima, baja K de 4 a 3, y **es
tautológica**: el plan (§2.1) declara de antemano que K premia lo lineal por
tramos. Un punto de K por eliminar la única propiedad (multiplicatividad) que
hacía útil al módulo es exactamente el «bajar K al precio de perder una
propiedad» que el plan pide buscar activamente. `N_periodicos = 0` en las tres
filas y en las dos bases: **la base 12 no interviene en la pregunta 6 en
absoluto**.

### 1.8 Veredicto pregunta 6

**Confirma** que la teoría de la exponencial compleja depende esencialmente de
L₂, y precisa *dónde*: en un único punto, la multiplicatividad del módulo, del
que cuelga todo lo demás por implicación. Lo que sobrevive sobrevive por
equivalencia de normas (es decir: trivialmente, y con constantes peores), o
porque nunca había dependido de la norma (C8). Casilla **(I)** para el núcleo.
La base 12 no aparece.

---

## 2. Pregunta 8 — Transformadas de Fourier bajo L₁

### 2.1 Tres lecturas distintas de la pregunta, y por qué hay que separarlas

«Fourier bajo L₁» es ambiguo de una forma que es exactamente donde vive el
pensamiento ilusorio. Hay tres cosas:

- **(a) El espacio de funciones: L¹(ℝⁿ) en vez de L²(ℝⁿ).** Clásico y cerrado.
- **(b) La geometría del dominio: usar `|x|₁` en los objetos geométricos**
  (bolas, esferas, núcleos radiales).
- **(c) Cambiar el núcleo `e^{i⟨ξ,x⟩}` por algo «adaptado a L₁».**

**(c) es imposible, y ése es el primer hallazgo.** Los caracteres del grupo
`(ℝⁿ, +)` son *exactamente* `x ↦ e^{i⟨ξ,x⟩}`, por dualidad de Pontryagin. No
hay elección que hacer. Además `⟨ξ,x⟩` es el **emparejamiento bilineal
canónico** entre ℝⁿ y su dual, que no es una norma y no necesita ninguna. Por
tanto:

> **La transformada de Fourier no tiene variante L₁.** La transformada la fija
> el grupo, no la métrica. Lo que cambia al cambiar de norma es únicamente
> *qué funciones y qué conjuntos se le meten*, y *qué se puede decir de la
> salida*.

Lo que sí necesita L₂ es la **identificación de ℝⁿ con su dual** (`ξ ↔ x` vía
producto interno); de ahí sale la equivariancia rotacional, y de ahí todo lo
demás. Ese es el punto exacto de ruptura, y conviene nombrarlo con precisión
porque no es «la transformada» sino «la autodualidad métrica del dominio».

### 2.2 C4 — Plancherel: P4, primera cláusula

**Enunciado L₂.** `‖f̂‖₂ = (2π)^{n/2} ‖f‖₂`. Equivale a que la transformada sea
un operador **unitario** del espacio de Hilbert `L²` en sí mismo.

**Enunciado L₁.** No existe. Y otra vez, por clasificación, no por falta de
ingenio:

1. Una norma proviene de un producto interno **si y solo si** cumple la ley del
   paralelogramo (**Jordan–von Neumann**). De la familia `Lᵖ`, solo `p = 2` la
   cumple. Sin producto interno no hay «unitario», ni adjunto, ni base
   ortonormal, ni teorema espectral.
2. Lo mejor que hay para `p ≠ 2` es **Hausdorff–Young**:
   `‖f̂‖_{p'} ≤ C_p ‖f‖_p` para `1 ≤ p ≤ 2`, con `p' ` el conjugado. Es una
   *desigualdad en un solo sentido*, y no hay recíproca. Con `p = 1` da
   `‖f̂‖_∞ ≤ ‖f‖₁` (trivial) y **Riemann–Lebesgue**: `f̂ ∈ C₀`. Pero la
   transformada **no** aplica `L¹` en `L¹`: la indicatriz de un intervalo tiene
   transformada `sinc`, que no es integrable. Así que ni isometría, ni
   sobreyectividad, ni inversión sin sumación (Fejér, Abel).
3. Se caen con ella: Parseval, conservación de la energía, la autoadjunción de
   `Δ` (que es un enunciado sobre el producto interno de `L²`), el teorema
   espectral, la descomposición en armónicos, y el **principio de
   incertidumbre** de Heisenberg — que en su forma estándar se demuestra con
   Cauchy–Schwarz, o sea con el producto interno.

**Casilla (I).** Y hay que decir además que aquí la casilla (I) no admite el
consuelo habitual de «existe pero es otra cosa»: no hay otra cosa.

**Lo que sí sobrevive de la maquinaria de Fourier** (casilla (II), y sobrevive
entero porque nunca dependió de la norma):

| propiedad | estado |
|---|---|
| `(f * g)^ = f̂ ĝ` | **sobrevive** |
| `L¹(ℝⁿ)` es álgebra de Banach con la convolución | **sobrevive**, y es *mejor* que L² (que no es cerrado por convolución) |
| dualidad de Pontryagin, sumación de Poisson | **sobreviven**; son del grupo |
| Riemann–Lebesgue | **sobrevive** |
| traslación ↦ modulación, dilatación ↦ dilatación inversa | **sobreviven**; son afines |
| equivariancia bajo `A ∈ GL(n)`: `(f∘A)^ = \|det A\|^{-1} f̂ ∘ A^{-T}` | **sobrevive**, y es la clave de 2.3 |
| equivariancia bajo `O(n)` ⇒ radial ↦ radial ⇒ reducción n → 1 variable | **no sobrevive** |

Nótese el detalle de la segunda fila, porque es una trampa del enunciado de la
pregunta 8: **el «gana L¹» de la convolución es del espacio de funciones (a),
no de la métrica del dominio (b).** Son cosas distintas y conflarlas produciría
evidencia falsa a favor de H₁. Lo apunto como ganancia real, pero de (a).

### 2.3 C5 — La transformada de la bola L₁ en 2D: P4, segunda cláusula

Ahora el cálculo concreto que el encargo pide.

**L₂.** `1̂_{B₂}(ξ) = 2π J₁(|ξ|)/|ξ|` en 2D, y
`(2π)^{n/2} J_{n/2}(|ξ|)/|ξ|^{n/2}` en nD: **una** función de Bessel, **una**
variable (`|ξ|`), para **todo** n.

**L₁.** `B₁² = {|x| + |y| ≤ 1}` es el politopo cruzado, que en dimensión 2 —y
solo en dimensión 2— es un cuadrado girado 45°. Con `u = x+y`, `v = x−y` se
tiene `|x|+|y| = max(|u|,|v|)` y `dx dy = du dv / 2`, y
`ξx + ηy = u(ξ+η)/2 + v(ξ−η)/2`, luego el dominio se factoriza:

```
1̂_{B₁}(ξ,η) = ½ ∫_{|u|≤1} e^{−iu(ξ+η)/2} du · ∫_{|v|≤1} e^{−iv(ξ−η)/2} dv
             = 8 · sin((ξ+η)/2) · sin((ξ−η)/2) / ((ξ+η)(ξ−η))
```

y con `sin A sin B = ½[cos(A−B) − cos(A+B)]`, `A−B = η`, `A+B = ξ`:

```
                4 ( cos η − cos ξ )
1̂_{B₁}(ξ,η)  =  ───────────────────
                     ξ² − η²
```

En `ξ = η = 0` vale 2 = área del rombo. ✔ En la diagonal `η → ξ` el límite es
`2 sin ξ / ξ`. Es elemental, cerrada, sin funciones especiales, y **más simple
que la de Bessel por cualquier criterio razonable**.

**Verificación con el instrumento.** Las dos formas cerradas deben coincidir.
En `(ξ,η) = (2,1)`: la primera da `(4/3)(cos 1 − cos 2)`, la segunda
`(8/3) sin(1;6) sin(0;6)` (nótese `3/2 = 1;6` y `1/2 = 0;6`).

```
$ target/release/dz-cli.exe eval "(4/3)*(cos(1)-cos(2))" --mode fixed --scale 14
~1;3377XX546366754

$ target/release/dz-cli.exe eval "(8/3)*sin(1;6)*sin(0;6)" --mode fixed --scale 14
~1;3377XX5463667365E86418565654
```

Coinciden en `1;3377XX5463667` — trece dígitos dozenales — y divergen después
por truncamiento de las series, con la discrepancia bajando al subir la escala
(a escala 6 coincidían en 6 dígitos, a escala 10 en 10). Identidad confirmada
por el binario, no a mano.

Coste de las dos formas, que también es un dato:

```
$ target/release/dz-cli.exe eval "(4/3)*(cos(1)-cos(2))" --mode fixed --scale 8 --gates
~1;3377XX554
gates: cycle=6261024 min=9788404 max=7268577 lit=4987464 thr=1154136 total=29459605 depth=16736 recon=0

$ target/release/dz-cli.exe eval "(8/3)*sin(1;6)*sin(0;6)" --mode fixed --scale 8 --gates
~1;3377XX53X812X2368
gates: cycle=5383356 min=8352374 max=6207206 lit=4265220 thr=991656 total=25199812 depth=16736 recon=0
```

**Y la contrapartida L₂ no es evaluable con este instrumento en absoluto**, que
es la forma más objetiva que tengo de decir «no es elemental»:

```
$ target/release/dz-cli.exe eval "besselj(1,1)" --mode fixed --scale 8
error: unknown function 'besselj'
```

El catálogo (`dz-cli --functions`) tiene `sqrt hypot exp ln log sin cos tan
asin acos atan` y las constantes `pi tau euler`. La transformada de la bola L₁
está dentro de ese catálogo; la de la bola L₂ está fuera. Eso no es un defecto
del binario: es la definición operativa de «elemental» aplicada sin criterio
humano.

**K(F) de la transformada de la bola unitaria, 2D.**

| | fórmula | raíces | irrac | operaciones | periódicos | K |
|---|---|--:|--:|--:|--:|--:|
| L₂ | `2π J₁(r)/r`, `r = (ξ₁²+ξ₂²)^{1/2}` | 1 | 1 (π) | 6 (`^,^,+,×,×,÷`) | 0 | **8** |
| L₁ | `4(cos η − cos ξ)/(ξ² − η²)` | 0 | 0 | 6 (`−,^,^,−,×,÷`) | 0 | **6** |

L₁ baja K de 8 a 6, y de los dos puntos uno es el radical y el otro es π. Pero
hay que leer la tabla con la advertencia del plan (§2.1) y con una segunda que
añado yo: **`K(F)` no tiene sumando para «función especial»**, así que la única
diferencia que de verdad importa entre las dos filas — que una necesita `J₁` y
la otra no — no está contabilizada en ninguna parte. Si se normalizara la
transformada L₂ para que no llevara π, `K(F)` daría 7 frente a 6 y la Bessel
seguiría siendo invisible. Detalle en §5.3.

`N_periodicos = 0` en las dos filas: el 4 y el 2 son enteros. **Aportación
dozenal: cero.**

### 2.4 Veredicto sobre P4

> **P4.** «Fourier bajo L₁ no da Plancherel. La transformada de la bola L₁ no
> es una función de Bessel elemental y **no** se simplifica.»

- **Primera cláusula: CONFIRMADA**, y por encima de lo pedido — no solo «no da
  Plancherel», sino que un teorema de clasificación (Jordan–von Neumann) impide
  que cualquier `Lᵖ` con `p ≠ 2` lo dé. §2.2.
- **Segunda cláusula: REFUTADA en dimensión 2.** La transformada de
  `1_{B₁²}` es `4(cos η − cos ξ)/(ξ² − η²)`: elemental, cerrada, y con
  `N_raices = 0` frente a la Bessel. La predicción registrada era incorrecta y
  lo dejo escrito así.

La refutación es real pero **acotada**, y las tres acotaciones siguientes son,
en mi juicio, más informativas que la refutación misma. Las doy como C6 y C7.

### 2.5 C6 — La simplificación es un accidente de dimensión 2

El truco entero de §2.3 es que `B₁² = R(B_∞²)` con `R` un giro de 45° y escala:
**en dimensión 2 el politopo cruzado es un cubo girado**. En `n ≥ 3` esto es
falso: el dual del cubo es el octaedro, y el octaedro no es un cubo. La
factorización en producto de `sinc` muere con la dimensión.

Lo que queda en general es el resultado de Brion / descomposición en simplices:
**la transformada de la indicatriz de cualquier politopo es elemental**
(trigonométrico-racional), como suma sobre los simplices de una triangulación.
Para `B₁ⁿ` hay `2ⁿ` orthantes, o sea la cota ingenua es `2ⁿ` términos.

Con la fibración por rebanadas se hace mejor. Rebanando `B₁³` por `x₁ = s` la
sección es `B₁²` de radio `r = 1−|s|`, y su transformada — homogeneidad — da
`4(cos(rc) − cos(rb))/(b²−c²)`, donde el `r²` se cancela. Integrando en `s`
sale (derivación completa y dos verificaciones independientes abajo):

```
                         a sin a                     b sin b                     c sin c
1̂_{B₁³}(a,b,c) = 8 [ − ───────────────── + ───────────────── − ───────────────── ]
                       (a²−b²)(a²−c²)       (a²−b²)(b²−c²)       (a²−c²)(b²−c²)
```

que es exactamente `−8 ·` la **diferencia dividida de segundo orden** de
`g(t) = √t · sin √t` en los nodos `t = a², b², c²`. Y la de 2D es
`−4 ·` la diferencia dividida de primer orden de `cos √t`; la de 1D es
`2 · (sin √t / √t)` evaluada. Con `f_n(t) = t^{(n−2)/2} · (sin√t` si n impar,
`cos√t` si n par`)`:

```
1̂_{B₁ⁿ}(ξ) = 2ⁿ (−1)^⌊n/2⌋ · Δ^{n−1} f_n (ξ₁², …, ξₙ²)
```

**Grado de certeza, declarado:** la fórmula está *derivada* para n = 1, 2, 3.
La constante `2ⁿ(−1)^⌊n/2⌋` está *forzada* para todo n por la normalización en
`ξ = 0` (el volumen de `B₁ⁿ` es `2ⁿ/n!`, y la diferencia dividida de orden
`n−1` en nodos coincidentes es el coeficiente de `t^{n−1}` de `f_n`, que vale
`(−1)^⌊n/2⌋/n!` — lo comprobé para n = 1..4). La **forma** de diferencia
dividida es conjetural para `n ≥ 4`: no la verifiqué. Sospecho que es conocida
(las diferencias divididas de potencias truncadas son B-splines, y las
proyecciones del politopo cruzado son B-splines), pero no lo afirmo.

Verificaciones del caso 3D, las dos con el binario:

```
# (a) las dos formas equivalentes en (1,2,3) — forma simétrica de 3 términos
$ target/release/dz-cli.exe eval "-8*((1*sin(1))/((1^2-2^2)*(1^2-3^2)) + (2*sin(2))/((2^2-1^2)*(2^2-3^2)) + (3*sin(3))/((3^2-1^2)*(3^2-2^2)))" --mode fixed --scale 10
~0;731024631324972497249728

# forma de rebanado, 2 términos
$ target/release/dz-cli.exe eval "8*((1*sin(1)-3*sin(3))/((1^2-3^2)*(2^2-3^2)) - (1*sin(1)-2*sin(2))/((1^2-2^2)*(2^2-3^2)))" --mode fixed --scale 10
~0;73102463132497249724972
```

(coinciden en `0;7310246313`, once dígitos; los literales `1,2,3` valen lo
mismo en base 12 que en base 10, así que aquí no hay trampa de base.)

```
# (b) control independiente: integrando z primero, F(a,0,0) = 8(a − sin a)/a³
$ target/release/dz-cli.exe eval "8*(1-sin(1))/1^3" --mode fixed --scale 10
~1;3276089X68

# la fórmula de 3 términos cerca de ese caso degenerado, (1, 0;001, 0;002)
$ target/release/dz-cli.exe eval "-8*((1*sin(1))/((1^2-0;001^2)*(1^2-0;002^2)) + (0;001*sin(0;001))/((0;001^2-1^2)*(0;001^2-0;002^2)) + (0;002*sin(0;002))/((0;002^2-1^2)*(0;002^2-0;001^2)))" --mode fixed --scale 10
~1;3276085E82368EXE2E909E08
```

Coinciden en `1;327608`, y `8(a − sin a)/a³` se derivó por una ruta
completamente distinta (integrar `z` primero y usar que el área de `B₁²` de
radio `r` es `2r²`). El `a=b=c=0` da `4/3` = `1;4` = volumen de `B₁³`. La
fórmula está validada en dos especializaciones independientes.

**Ahora el juicio que el encargo pide, sobre «más discreto / más simple».**

| | L₂, bola | L₁, politopo cruzado |
|---|---|---|
| forma cerrada | `(2π)^{n/2} J_{n/2}(r) / r^{n/2}` | diferencia dividida de orden `n−1`, `n` términos, `n(n−1)` factores |
| nº de variables | **1** (`r`) | **n** |
| nº de términos | **1**, para todo n | `n` (o `2ⁿ` por la ruta de vértices) |
| funciones especiales | 1 (Bessel) | 0 |
| singularidades de la fórmula | ninguna (analítica) | removibles en el arreglo `{ξ_i = ±ξ_j}`, `n(n−1)` hiperplanos |
| simetría de la salida | radial: función de una variable | invariante diedral de orden `2ⁿ n!`: función de n variables |

Es decir: **L₁ cambia una función especial por un número creciente de términos
elementales y por un arreglo de casos degenerados.** «Más simple» es justo en
n = 2, discutible en n = 3 y falso asintóticamente en n. Y la pérdida
estructural es la que importa: lo que L₂ hace no es «dar Bessel», es
**reducir n variables a 1**. Esa reducción es la equivariancia rotacional, es
un teorema sobre `O(n)`, y **no tiene sustituto**: el grupo que la reemplaza es
finito, y un grupo finito no reduce dimensiones. Casilla **(I)**, y es el
contenido verdadero de P4 aunque la letra de P4 fuese falsa.

**Y el arreglo de singularidades no es cosmético: se mide.** El límite
diagonal exacto de la fórmula 2D en `ξ = η = 2` es `sin 2`:

```
$ target/release/dz-cli.exe eval "sin(2)" --mode fixed --scale 24
~0;XXE3236990865X316E8E176X
```

Evaluando la fórmula cerrada a distancia `δ = 12^{-6}` de la diagonal, con
escalas crecientes:

| escala | `4(cos(2−δ) − cos 2)/(2² − (2−δ)²)`, `δ = 0;000001` |
|--:|---|
| 8 | `~0;XE000028900008230002069` |
| 10 | `~0;XXE300288990082253020673` |
| 14 | `~0;XXE324008997002254900674` |
| 18 | `~0;XXE32400551500143430040X` |
| 24 | `~0;XXE324005515299EXX3855E8` |

```
$ target/release/dz-cli.exe eval "4*(cos(2-0;000001)-cos(2))/(2^2-(2-0;000001)^2)" --mode fixed --scale 8
```
(y lo mismo con `--scale 10|14|18|24`.)

**A escala 8 la fórmula elemental se equivoca en el segundo dígito
fraccionario** (`0;XE0…` frente a `0;XXE3…`), y necesita escala ≈ 14 para dar
cuatro dígitos correctos. La causa es la cancelación catastrófica: numerador y
denominador tienden a cero juntos. `J₁(r)/r` es analítica ahí y no tiene ese
problema.

Matiz honesto: parte de la degradación es de **este instrumento** —
`dz-cli` es punto fijo, y un cociente de dos cantidades pequeñas pierde dígitos
absolutos de una forma que un flotante normalizado no perdería. La conclusión
robusta no es «la fórmula L₁ es inservible», es: **la forma cerrada elemental
de L₁ trae un requisito de precisión que la forma de Bessel no trae, y ese
requisito es medible.** Lo cual es exactamente la clase de coste que `K(F)` no
ve — y lo anoto como crítica del instrumento `K(F)`, abajo.

### 2.6 C7 — Decaimiento anisótropo: el otro precio

`1̂_{B₂}` decae como `|ξ|^{-(n+1)/2}` **uniformemente en dirección** (en 2D,
`r^{-3/2}`). Para el politopo no hay un exponente: hay uno por dirección.

- En dirección de eje (`η = 0`): `4(1 − cos ξ)/ξ²`, que decae como `ξ^{-2}`.
- En dirección normal a una cara (`η = ξ`, las cuatro diagonales): el límite es
  `2 sin ξ / ξ`, que decae solo como `ξ^{-1}`.

Es el fenómeno estándar de los politopos: las caras planas producen
decaimiento lento en su normal. Medido en `|ξ| = 10` (dozenal) = doce:

```
$ target/release/dz-cli.exe eval "2*sin(10)/10" --mode fixed --scale 8
~-0;10X649036

$ target/release/dz-cli.exe eval "4*(1-cos(10))/10^2" --mode fixed --scale 8
~0;0075E3464

$ target/release/dz-cli.exe eval "(2*sin(10)/10)/(4*(1-cos(10))/10^2)" --mode fixed --scale 6
~-18;7500X5609847E29358107937
```

El cociente es `−18;75…` dozenal, de módulo ≈ veinte y pico: un orden `ξ`
completo de diferencia entre las dos direcciones a la misma frecuencia. (No es
un número limpio porque ambas direcciones oscilan; lo que el dato fija es el
orden, no una constante.)

Consecuencia analítica: la bola L₂ tiene un único exponente de decaimiento y
por tanto pertenencias `Lᵖ` limpias para su transformada; el politopo no, y sus
propiedades de sumabilidad hay que discutirlas dirección por dirección. **Ésta
es la razón técnica de por qué los problemas de multiplicadores y de
convergencia de sumas parciales sobre politopos son *distintos* de los de la
bola** (el problema del multiplicador de la bola de Fefferman frente a los
politópicos, donde el resultado se sabe y es más fácil). No lo desarrollo más
porque se sale de mi alcance verificable; lo dejo anotado como dirección con su
nivel de certeza: **plausible, no verificado aquí**.

Un punto donde L₁ **gana de verdad** en esta pregunta, y hay que decirlo con la
misma fuerza: el **conteo de puntos reticulares**.
`#{m ∈ ℤ² : |m|₁ ≤ N} = 2N² + 2N + 1`, exacto y cerrado, mientras que el
problema del círculo de Gauss (`#{|m|₂ ≤ r}`) tiene término de error cuyo
exponente óptimo **es un problema abierto**. Aquí L₁ convierte un problema
abierto en una identidad de dos líneas. Casilla (II) con simplificación
genuina. La base 12: `2N² + 2N + 1` no tiene ninguna constante racional, luego
`N_periodicos = 0` en las dos bases y el efecto dozenal es **cero**.

### 2.7 C5b — Núcleos: el sitio donde L₁ gana limpiamente en Fourier

El núcleo de calor `e^{−|x|₂²/4t}` es gaussiano, y la gaussiana es (teorema de
Maxwell) **la única** función que es a la vez radial y producto de funciones de
las coordenadas. Está exactamente en la intersección de las dos estructuras de
simetría, y por eso no distingue L₁ de L₂. No hay contraste que medir ahí.

Donde sí hay contraste es en el núcleo **exponencial**, que es el objeto
natural una vez que se elige una norma:

| | núcleo | transformada |
|---|---|---|
| L₁ | `e^{−\|x\|₁} = ∏ᵢ e^{−\|xᵢ\|}` | `∏ᵢ 2/(1+ξᵢ²) = 2ⁿ / ∏ᵢ(1+ξᵢ²)` — **racional** |
| L₂ | `e^{−\|x\|₂}` | `2ⁿ π^{(n−1)/2} Γ((n+1)/2) / (1+\|ξ\|²)^{(n+1)/2}` — potencia semientera |

En 2D: `4/((1+ξ₁²)(1+ξ₂²))` frente a `2π/(1+|ξ|²)^{3/2}`.

| K(F) | raíces | irracionales | operaciones | periódicos | total |
|---|--:|--:|--:|--:|--:|
| L₂: `2π/(1+ξ₁²+ξ₂²)^{3/2}` | 1 | 1 (π) | 7 (`^,^,+,+,^,÷,×`) | 0 | **9** |
| L₁: `4/((1+ξ₁²)(1+ξ₂²))` | 0 | 0 | 6 (`^,^,+,+,×,÷`) | 0 | **6** |

Y la lectura estructural, más importante que el K: `∏(1+ξᵢ²)` es un **símbolo
polinómico**, luego corresponde a un operador diferencial genuino y local,
`∏ᵢ(1 − ∂ᵢ²)`; `(1+|ξ|²)^{(n+1)/2}` es una potencia fraccionaria de `(1−Δ)`,
que es solo pseudodiferencial y **no local**. Ése es un triunfo real de L₁:
cambia un operador no local por uno local y polinómico.

**Con su precio exacto.** El símbolo principal de `∏ᵢ(1−∂ᵢ²)` es `∏ᵢ ξᵢ²`, que
**se anula en los hiperplanos coordenados**. El operador no es elíptico. Luego
no hay regularidad elíptica, y su función de Green tiene singularidades
direccionales a lo largo de los ejes. Recurre el mismo patrón de toda la línea:
*L₁ compra fórmulas elementales y paga con degeneración en las direcciones
coordenadas.*

Coste medido de las dos transformadas en `ξ = (1,1)` y `ξ = (1,2)`:

```
$ target/release/dz-cli.exe eval "4/((1+1^2)*(1+1^2))" --mode fixed --scale 8 --gates
1
gates: cycle=119592 min=131244 max=102063 lit=75612 thr=22896 total=451407 depth=1272 recon=0

$ target/release/dz-cli.exe eval "2*pi/exp((3/2)*ln(1+1^2+1^2))" --mode fixed --scale 8 --gates
~1;2615E670906861EE73X39077
gates: cycle=52146204 min=85106618 max=62892696 lit=42785424 thr=9555972 total=252486914 depth=32220 recon=0

$ target/release/dz-cli.exe eval "4/((1+1^2)*(1+2^2))" --mode fixed --scale 8 --gates
0;497249724972497249724972 (periodica en base 12, valor truncado)
gates: cycle=184932 min=228654 max=175488 lit=127320 thr=37128 total=753522 depth=1443 recon=0

$ target/release/dz-cli.exe eval "2*pi/exp((3/2)*ln(1+1^2+2^2))" --mode fixed --scale 8 --gates
~0;5168E9913215E115120623EE
gates: cycle=66630696 min=108485672 max=80190049 lit=54577680 thr=12203316 total=322087413 depth=32804 recon=0
```

| `ξ` | L₁: puertas | L₂: puertas | razón | prof. L₁ | prof. L₂ |
|---|--:|--:|--:|--:|--:|
| `(1,1)` | 451.407 | 252.486.914 | **≈ 559×** | 1.272 | 32.220 |
| `(1,2)` | 753.522 | 322.087.413 | **≈ 427×** | 1.443 | 32.804 |

Tres lecturas obligadas de esta tabla:

1. La razón de ~500× es real y es el número más fuerte que la pregunta 8 aporta
   a H₁. Sale de que la potencia semientera obliga a `exp(1;6 · ln(·))`, o sea
   a dos trascendentes, mientras L₁ solo multiplica y divide. Coincide en orden
   de magnitud con el hallazgo central de `docs/05-funciones-cientificas.md`
   (una trascendente cuesta 2–4 órdenes de magnitud más que una
   multiplicación).
2. **La cifra L₂ está subestimada.** `pi` reporta 0 puertas a propósito (se
   memoiza en un backend desechable, §5 del SKILL), así que el verdadero coste
   de la fórmula euclidiana es mayor que 252.486.914. Declaro el sesgo y noto
   que va en contra de mi conclusión, no a favor.
3. **Contradato dozenal, y va contra H₁.** En `ξ = (1,2)` el valor L₁ es
   `4/10 = 2/5`, que **no termina en base 12** (`0;4972 4972…`) y sí termina en
   decimal (`0.4`). El propio caso donde la fórmula L₁ es más limpia es un caso
   donde la base 12 pierde. Es el escenario exacto de `02-referencia-medida.md`
   §1: los quintos pagan por el cambio de base. No es anecdótico: la fórmula
   `4/∏(1+ξᵢ²)` produce denominadores `1+k²`, y `1+2² = 5`, `1+3² = 10`,
   `1+7² = 50` son todos múltiplos de 5. **La familia de constantes que esta
   fórmula genera está sesgada hacia denominadores que base 12 no sabe
   escribir.**

### 2.8 Veredicto pregunta 8

- **Plancherel: no hay, y no puede haber** (casilla I, por teorema de
  clasificación). P4 primera cláusula confirmada.
- **La transformada de la bola L₁ en 2D sí es elemental y sí se simplifica**
  (casilla II). P4 segunda cláusula **refutada**, y lo registro como
  refutación.
- **Pero la simplificación no escala**: es un accidente de que el politopo
  cruzado sea un cubo girado solo en 2D, y el precio es (a) `n` variables en
  vez de 1, (b) un arreglo de singularidades removibles con requisito de
  precisión medible, (c) decaimiento con exponente dependiente de la dirección.
  La propiedad que de verdad hace valiosa a L₂ aquí no es Bessel: es la
  **reducción de n variables a 1**, y ésa es casilla (I) sin sustituto.
- **Los núcleos adaptados a L₁ ganan** limpiamente: símbolo racional
  polinómico frente a potencia semientera, K(F) 6 frente a 9, y ~500× menos
  puertas medidas. A cambio: no elípticos.
- La base 12 no aporta nada, y en el caso concreto de los núcleos **resta**.

---

## 3. Pregunta 9 — `‖∇f‖₂` frente a `‖∇f‖₁`

### 3.1 Hay que separar tres funcionales, no dos

Otra conflación que arruinaría el análisis si no se nombra:

| funcional | nombre | homogeneidad | de quién es la norma |
|---|---|---|---|
| `∫ \|∇f\|₂²` | energía de Dirichlet | 2 (cuadrática) | L² de la función *y* L₂ del gradiente |
| `∫ \|∇f\|₂` | VT **isótropa** | 1 | L¹ de la función, **L₂ del gradiente** |
| `∫ (\|∂₁f\| + \|∂₂f\|)` | VT **anisótropa** | 1 | L¹ de la función y **L₁ del gradiente** |

Cuando la literatura de imagen dice «regularización L₁» casi siempre se
refiere a la **segunda** fila, que es L₂ en el punto donde esta investigación
cuenta (la norma del gradiente). El objeto honesto para H₁ es la tercera. Lo
digo porque el argumento «L₁ funciona mejor en imagen, luego H₁» se apoya en la
mayoría de los casos en la fila equivocada.

### 3.2 Lo que se pierde al pasar de la fila 1 a la fila 3 (casilla I)

| propiedad | `∫\|∇f\|₂²` | `∫\|∇f\|₁` |
|---|---|---|
| ecuación de Euler–Lagrange | `Δf = 0`, **lineal** | `div(sgn ∇f) = 0`, degenerada, no de un solo valor |
| superposición de soluciones | **sí** | **no** (1-homogénea, no cuadrática) |
| autoadjunción del operador | **sí**, respecto del producto interno de L² | el enunciado no existe: no hay producto interno que lo haga |
| teoría espectral, autofunciones, separación de variables | **sí** | **no** |
| solución fundamental / núcleo de calor | **sí**, gaussiano | **no**, el flujo es no lineal |
| invariancia rotacional | **sí**; `Δ` es el único operador de 2º orden rotacionalmente invariante salvo escala | **no**; girar la imagen 45° cambia la respuesta |
| unicidad del minimizador con dato de borde | **sí** | **no** en general (las facetas pueden no ser únicas) |
| regularidad elíptica | **sí**, analiticidad | **no**; los minimizadores son facetados |

La fila de la autoadjunción merece una frase porque es el corazón de la
casilla (I) de toda esta línea: «`Δ` es autoadjunto» significa
`⟨Δu, v⟩ = ⟨u, Δv⟩` **con el producto interno de L²**. Sin producto interno no
es que el enunciado sea falso: es que no se puede escribir. De ahí cuelgan el
teorema espectral, la descomposición en modos, la conservación de la energía y
la teoría de semigrupos autoadjuntos.

### 3.3 K(F) de las superficies mínimas — y de lo que se pierde al bajarlo

Éste es el caso limpio que el plan (§2.1) pide buscar: **L₁ baja K y se lleva
por delante una propiedad**.

| | funcional | raíces | irrac | ops | per | K |
|---|---|--:|--:|--:|--:|--:|
| L₂ | `∫ √(1 + u_x² + u_y²)` | 1 | 0 | 5 (`^,^,+,+,^½`) | 0 | **6** |
| L₁ | `∫ (1 + \|u_x\| + \|u_y\|)` | 0 | 0 | 4 (`\|·\|,\|·\|,+,+`) | 0 | **4** |

La forma L₁ sale de tomar como energía de superficie la norma ℓ¹ de la normal
del grafo, `φ(−∇u, 1) = 1 + |u_x| + |u_y|`: el área anisótropa es entonces
`|Ω| + ∫|∇u|₁`, o sea **constante más variación total anisótropa**. Es exacta y
lineal por tramos.

Medido en `∇u = (1,2)`:

```
$ target/release/dz-cli.exe eval "sqrt(1+1^2+2^2)" --mode fixed --scale 8 --gates
~2;54887522
gates: cycle=3553836 min=5561078 max=4129105 lit=2832792 thr=655992 total=16732803 depth=13895 recon=0

$ target/release/dz-cli.exe eval "1+abs(1)+abs(2)" --mode fixed --scale 8 --gates
4
gates: cycle=10032 min=15332 max=11486 lit=8004 thr=2064 total=46918 depth=628 recon=0
```

| | valor | puertas | profundidad | exacto |
|---|---|--:|--:|---|
| L₂ `√6` | `~2;54887522` | 16.732.803 | 13.895 | no (`~`) |
| L₁ | `4` | 46.918 | 628 | **sí** |

**≈ 357× menos puertas, 22× menos profundidad, y respuesta exacta en vez de
truncada.** Es un resultado sólido a favor del factor L₁ de H₁, y del mismo
tipo que el número central de `02-referencia-medida.md` §3 (el 1.105× de
`√(3²+4²)` frente a `3+4`), con la misma advertencia: son dos funcionales
distintos, no dos respuestas a la misma pregunta.

**Y lo que se paga, dicho con la misma precisión.** El funcional L₁ (a) no es
invariante bajo rotaciones del ℝ³ ambiente, así que **no es un «área»** en
ningún sentido geométrico invariante — es una energía superficial cristalina
legítima, pero llamarla «superficie mínima» conflaciona dos cosas; (b) su
Euler–Lagrange es degenerada y los minimizadores son poliédricos, con facetas
que pueden no ser únicas; (c) se pierde la rigidez tipo Bernstein y la
analiticidad. `K(F)` baja de 6 a 4 y no ve nada de esto — **crítica del
instrumento, no del hallazgo.**

### 3.4 C10 — Donde esperaba que L₁ quitara la raíz y no la quita

Predicción propia, antes de calcular: el flujo por curvatura media euclidiano
tiene círculos que se contraen con `ṙ = −1/r`, luego
`r(t) = √(r₀² − 2t)` y extinción en `t = r₀²/2` — una raíz cuadrada. Esperaba
que la versión cristalina (flujo por curvatura cristalina, Taylor–Almgren,
Giga–Giga, Chambolle–Morini–Novaga), donde un rectángulo alineado con los ejes
**sigue siendo un rectángulo** y el flujo se reduce a un sistema **finito** de
EDOs para las posiciones de las facetas, eliminara la raíz.

No la elimina. La razón es de escalado y no de métrica: la curvatura
(cristalina o no) es 1-homogénea de grado `−1`, luego el flujo es invariante
bajo `x → λx, t → λ²t` para **cualquier** anisotropía. Un minimizador
autosemejante cumple por tanto `ṙ = −c/r` con `c` dependiente de la anisotropía,
y `r(t) = √(r₀² − 2ct)` **en las dos métricas**. La raíz cuadrada del tiempo de
extinción es una consecuencia de la homogeneidad más el escalado parabólico, y
es **invariante al cambio de norma**.

```
$ target/release/dz-cli.exe eval "sqrt(1-2*0;3)" --mode fixed --scale 8 --gates
~0;859X6965
gates: cycle=2158332 min=3387614 max=2514313 lit=1723752 thr=398880 total=10182891 depth=8342 recon=0
```

(`r₀ = 1`, `ct = 0;3` = un cuarto; `√(1 − ½) = √½`, que es `0;859X6965`, el
mismo numeral que `sqrt(2)/2`.) `K(F)` de `r(t)`: `N_raices = 1` en **ambas**
métricas. **Ganancia nula.** Lo que cambia es `c`, una constante, o sea
casilla (II) sin simplificación. Dozenal: nada.

Éste es el hallazgo de «esperaba fallo de L₂ y hubo empate» más útil de la
línea, porque identifica *qué clase* de raíces L₁ no puede quitar: las que
vienen del **escalado del problema**, no de la métrica. Y da una regla
predictiva: L₁ elimina raíces que provienen de `√(suma de cuadrados)`; no
elimina raíces que provienen de análisis dimensional.

### 3.5 C9 — Donde L₁ gana de verdad, con el mismo rigor

El encargo pide que esta sección tenga el mismo peso que las anteriores. Lo
tiene, y es la razón por la que mi veredicto es «confinada» y no «refutada».

**(a) Esparsidad, y su causa geométrica exacta.** Minimizar `‖x‖₁` sujeto a
`Ax = b` es un **programa lineal**; minimizar `‖x‖₂` es un problema cuadrático.
Los óptimos de un PL están en **vértices** del poliedro factible, y los
vértices de la bola ℓ¹ son exactamente los `2n` puntos `±eᵢ` — vectores con una
sola coordenada no nula. Por eso los óptimos ℓ¹ son esparsos. La bola ℓ² **no
tiene vértices**, y por eso los óptimos ℓ² nunca son esparsos.

Esto no es una heurística ni un fenómeno empírico: es la geometría de los
puntos extremos, y es **el mismo hecho** que mata la invariancia rotacional
(C2, C6). Lo subrayo porque es el hallazgo central del informe:

> La bola ℓ¹ tiene `2n` vértices alineados con los ejes; la ℓ² no tiene
> ninguno. Que haya vértices ⇒ óptimos esparsos (gana L₁). Que estén alineados
> con los ejes ⇒ no hay grupo de rotaciones (pierde L₁). Es un solo teorema.

**(b) Optimización exacta: corte mínimo en grafo.** Ésta es la ganancia más
fuerte, y es un salto de *clase de complejidad*, no de constante. La variación
total **anisótropa** discreta de una función binaria sobre una retícula es
exactamente una **función de corte** de un grafo, luego minimizarla con
términos de datos submodulares es un problema de **flujo máximo / corte
mínimo**, resoluble **exactamente** en tiempo polinómico (Boykov–Kolmogorov;
Chambolle). La versión **isótropa** (`∫|∇f|₂`, fila 2 de §3.1) no tiene esa
estructura y solo se sabe **aproximar**.

**L₁ convierte «aproximable» en «exactamente resoluble».** Casilla (II) con
ganancia genuina y estructural, no de constante, y sin contraparte en L₂.
Y encaja con el proyecto: un PL con datos enteros tiene óptimos racionales
exactos, y el simplex con aritmética exacta no necesita ninguna trascendente —
es decir, no necesita ninguno de los circuitos que en este repositorio cuestan
2–4 órdenes de magnitud.

**(c) Preservación de bordes.** Un minimizador de la energía de Dirichlet en 2D
está en `H¹` y no puede saltar a través de una curva; un minimizador de
variación total está en `BV` y **sí** puede. Por eso el desenfoque de bordes es
intrínseco al funcional cuadrático y su ausencia es intrínseca a la VT. Real,
riguroso, y a favor de L₁.

**(d) Sensado comprimido.** La recuperación exacta de vectores esparsos por
minimización ℓ¹ bajo RIP / propiedad del espacio nulo (Candès–Romberg–Tao,
Donoho) es un teorema, y la minimización ℓ² **nunca** recupera lo esparso. A
favor de L₁, sin reservas sobre el resultado. **Con una reserva sobre la
lectura:** en sensado comprimido el ℓ¹ está en el **objetivo** y la geometría
ℓ₂ está en las **restricciones** (RIP se enuncia con normas ℓ₂). O sea que la
historia estrella de «L₁ vence a L₂» es en realidad **híbrida**: ℓ¹ encima de
geometría ℓ₂. Eso no le quita nada al teorema, pero sí a la retórica de que L₁
reemplaza a L₂.

**(e) La constante isoperimétrica se vuelve entera.** Para la VT anisótropa, el
perímetro de un conjunto `E` es `∫_{∂E} |ν|₁`, y la forma de Wulff — el
minimizador isoperimétrico — es la bola `ℓ∞`, o sea un **cuadrado alineado con
los ejes**. Cuentas: cuadrado de lado `s`, perímetro `4s`, área `s²`, luego
`P² = 16 A`; disco de radio `r`, `∫₀^{2π}(|cos θ| + |sin θ|) r dθ = 8r`, luego
`P² = 64r² > 16πr²`. La desigualdad es

```
L₂:  P² ≥ 4π A      →      L₁ anisótropa:  P² ≥ 16 A
```

```
$ target/release/dz-cli.exe eval "4*pi" --mode fixed --scale 8 --gates
~10;6968317
gates: cycle=8052 min=6938 max=5572 lit=4332 thr=1464 total=26358 depth=15168 recon=0

$ target/release/dz-cli.exe eval "16" --input-base 10 --mode fixed --gates
14
gates: cycle=3828 min=5914 max=4435 lit=3096 thr=816 total=18089 depth=254 recon=0

$ target/release/dz-cli.exe eval "14/(4*pi)" --mode fixed --scale 8
~1;3341X8XXEE15X6X6159X6034
```

`4π = 10;6968317…` (trascendente, infinito en toda base) frente a `16`, que es
`14` en base 12 (el literal dozenal `14` verificado contra `2^4` en §1.4).
El cociente es `1;3341X8…`, o sea `4/π`. `K(F)`: L₂ tiene `N_irracionales = 1`,
L₁ tiene `0`. Ganancia de 1 punto, y de casilla **(II)**.

Nota de honestidad sobre el solape: este resultado toca la pregunta 2 de la
línea A («¿hay un π taxicab único?») y refuerza P1 desde aquí: la constante que
sale para *esta* propiedad conservada es `16`, no `4` ni `2`. Es un tercer
valor, y eso es evidencia adicional de que no hay un «π taxicab» único, sino
uno por propiedad. Lo cedo a la línea A y lo cito solo por completitud del
operador.

Y dozenal: `16 → 14`, `4 → 4`, `2 → 2`. Todos enteros, terminan en toda base.
`N_periodicos = 0` en ambas formulaciones. **La base 12 no gana nada, otra
vez.**

### 3.6 Veredicto pregunta 9

**No concluyente en una dirección única, y eso es el resultado**, no una
evasión:

- **L₂ es estructuralmente irreemplazable** para difusión lineal, teoría
  espectral, autoadjunción, invariancia rotacional, unicidad y regularidad
  (casilla I, §3.2).
- **L₁ es estructuralmente superior** para optimización exacta (PL / corte
  mínimo, cambio de clase de tratabilidad), esparsidad (vértices del politopo),
  preservación de bordes (BV frente a H¹) y constantes isoperimétricas enteras
  (casilla II con ganancia real, §3.5).
- **Los dos hechos tienen la misma causa**: la bola ℓ¹ es un politopo con pocos
  vértices alineados con los ejes. No es una compensación fortuita; es un
  teorema leído en dos direcciones.
- La base 12 no interviene en ningún punto de la pregunta 9.

---

## 4. `N_periodicos` en toda la línea C — C11 y P5

El plan (§2.3) dice que `N_periodicos` es el único sumando sensible a la base y
por tanto el que aísla el efecto dozenal. Recuento completo de la línea:

| fórmula | constantes racionales | `N_per` base 10 | `N_per` base 12 |
|---|---|--:|--:|
| `\|z\|₁ = \|x\|+\|y\|` | ninguna | 0 | 0 |
| defecto multiplicativo `½` | `½` | 0 | 0 |
| `\|(1+i)ⁿ\|₁ = 2^⌈n/2⌉` | ninguna | 0 | 0 |
| `1̂_{B₁²} = 4(cos η − cos ξ)/(ξ²−η²)` | `4` (entero) | 0 | 0 |
| `1̂_{B₂²} = 2π J₁(r)/r` | `2` (entero) | 0 | 0 |
| `1̂_{B₁ⁿ} = 2ⁿ(−1)^⌊n/2⌋ Δ^{n−1}f_n` | potencias de 2 | 0 | 0 |
| núcleo L₁: `4/∏(1+ξᵢ²)` | `4` | 0 | 0 |
| núcleo L₂: `2π/(1+\|ξ\|²)^{3/2}` | `2`, `3/2` | 0 | 0 |
| conteo reticular `2N²+2N+1` | enteros | 0 | 0 |
| área anisótropa `1+\|u_x\|+\|u_y\|` | ninguna | 0 | 0 |
| extinción `√(r₀²−2ct)` | `2` | 0 | 0 |
| isoperimétrica `16` vs `4π` | `16`, `4` | 0 | 0 |

**Cero en todas las filas y en las dos bases.** Y no por casualidad: las
fórmulas del análisis tienen coeficientes que son **enteros, potencias de 2, o
π**. Los enteros terminan en cualquier base; π no termina en ninguna. El
discriminante dozenal solo muerde en racionales con denominador no trivial, y
el análisis clásico casi no los produce en sus formas cerradas.

> **P5 queda confirmada desde la línea C, y en su forma más dura.** El efecto
> dozenal medible en esta línea no es «pequeño»: es **nulo**. No aparece
> ninguna simplificación estructural atribuible a la base, y tampoco aparece la
> ganancia barata de `N_periodicos` que P5 admitía como suelo, porque no hay
> racionales periódicos que ganar.

El único sitio donde base 12 aparece con signo es **en contra**: al *evaluar*
el núcleo L₁ en `ξ = (1,2)` sale `2/5 = 0;4972 4972…`, periódico en base 12 y
terminante en decimal (§2.7, punto 3). Y eso es (III), de evaluación, no de
fórmula, así que ni siquiera cuenta en `K(F)`.

Consecuencia para H₁: el **factor dozenal de la conjunción H₁ es falso en la
línea C**. Como H₁ es una conjunción y el plan (§1) permite refutarla por
cualquiera de sus dos factores por separado, esto es una refutación parcial
localizada, no una impresión.

---

## 5. Lo que no funcionó

Obligatoria y no vacía, y leída **simétricamente** como pide el encargo: no
solo errores propios, también sitios donde esperaba que L₁ fallara y no falló.

### 5.1 Predicciones propias que resultaron falsas

1. **P4, segunda cláusula: la escribí yo (como línea C) y es falsa.** La
   transformada de la bola L₁ en 2D **sí** es elemental y **sí** se simplifica:
   `4(cos η − cos ξ)/(ξ²−η²)`. La predicción registrada era incorrecta. Lo que
   la salva parcialmente es que la intuición subyacente (que se pierde la
   reducción a una variable) era correcta y es lo que de verdad importaba; pero
   la letra de la predicción falló y queda por escrito.
2. **Esperaba pérdida de constante en el principio del módulo máximo. No hay
   ninguna** (C8). Peor para mi intuición: el argumento correcto demuestra que
   el principio vale para **toda** norma, porque `N(f)` es siempre supremo de
   funciones armónicas. L₂ entraba ahí solo por costumbre de notación. Este es
   el aviso más útil del informe: no todo lo que se escribe con `|·|` depende
   de `|·|`.
3. **Esperaba que el flujo cristalino eliminara la raíz cuadrada del tiempo de
   extinción. No la elimina** (C10). La raíz viene del escalado parabólico y de
   la 1-homogeneidad de la curvatura, no de la métrica, y por tanto es
   invariante al cambio de norma. `K(F)` empata, y eso acota qué clase de
   radicales L₁ puede quitar.
4. **Esperaba que el teorema fundamental del álgebra o Liouville dieran
   problemas con `|·|₁`. No dan ninguno.** Sobreviven enteros por equivalencia
   de normas en dimensión finita, con constantes en `[1, √2]`. Este «no falló»
   es en realidad un aviso en sentido contrario: *sobrevivir por equivalencia
   de normas es sobrevivir trivialmente*, y no es evidencia a favor de L₁. Una
   propiedad que se conserva porque todas las normas de ℝⁿ son equivalentes no
   distingue nada. Hay que resistir la tentación de apuntar esas filas como
   victorias de casilla (II).
5. **Esperaba que la anisotropía de L₁ estropeara el conteo de puntos
   reticulares. Lo mejora hasta hacerlo trivial**: `2N²+2N+1` exacto frente al
   problema del círculo de Gauss, abierto. Esperaba coste, obtuve un problema
   abierto convertido en identidad.

### 5.2 Errores de método cometidos y corregidos

6. **Caí en la trampa de los literales dozenales** que el coordinador señaló.
   Al comprobar el `1/24` de la fórmula 3D escribí `eval "4/24"` y obtuve
   `0;186X35 186X35…` — periódico. La razón es que `24` dozenal es veintiocho,
   así que estaba calculando `1/7`, no `1/6`. Lo detecté porque `1/6` tiene que
   terminar en base 12 (`0;2`) por la factorización `12 = 2²·3`, y el resultado
   contradecía la tabla de `02-referencia-medida.md` §1. Corregido reescribiendo
   todos los denominadores como expresiones (`(1^2-2^2)*(1^2-3^2)`) en vez de
   como literales, y verificando cada literal dozenal contra el binario
   (`2^4 → 14`, `2^9 → 368`, `15 --input-base 10 → 13`). **La tabla de
   recíprocos de la referencia funciona como detector de este error**, y lo
   apunto para las otras líneas.
7. **`pi` reporta 0 puertas**, lo que **subestima** todas mis cifras de coste
   euclidianas (§2.7). Va en contra de mi conclusión, así que la declaro en vez
   de aprovecharla; pero significa que las razones 559× y 427× son cotas
   inferiores del favor a L₁, no medidas exactas.
8. **Intenté medir la fórmula de Bessel y no pude.** `eval "besselj(1,1)"` da
   `error: unknown function 'besselj'`. Convertí la limitación en dato
   (§2.3: el catálogo del binario como definición operativa de «elemental»),
   pero hay que decir que es un apaño: **no tengo una comparación de coste
   medida entre la transformada de la bola L₁ y la de la bola L₂**, porque el
   instrumento no evalúa la segunda. La única comparación de coste real de la
   pregunta 8 es la de los núcleos exponenciales (§2.7), que es un par distinto.

### 5.3 Límites del instrumento K(F), encontrados por uso

9. **`K(F)` no tiene casilla para «función especial».** En la comparación
   central de la pregunta 8, la diferencia que importa es «Bessel frente a
   cosenos», y `K(F)` la contabiliza como `N_irracionales = 1` por el π — que
   es un accidente de la normalización, no la diferencia real. La tabla de §2.3
   da **8 frente a 6**; si se normalizara la transformada L₂ para que no llevara
   π sería **7 frente a 6**, y la Bessel no habría entrado en la cuenta en
   ningún momento. El índice mide ortografía, no dificultad.
10. **`K(F)` no ve el número de variables.** La transformada de la bola L₂ es
    función de 1 variable para todo n; la de la L₁ es de n. Es la diferencia
    estructural más importante de toda la pregunta 8 y `K(F)` le asigna cero.
11. **`K(F)` no ve el condicionamiento.** La fórmula elemental L₁ pierde dos
    dígitos fraccionarios cerca de su arreglo de singularidades removibles a
    escala 8 (§2.5, medido), y la Bessel es analítica ahí. `K(F)` premia a la
    L₁.
12. **`K(F)` no ve las propiedades perdidas.** En §3.3 baja de 6 a 4 a cambio
    de unicidad, suavidad, invariancia rotacional y el propio carácter de
    «área». El plan ya declaraba este sesgo (§2.1); lo confirmo con tres
    instancias medidas, y concluyo que **en esta línea `K(F)` es
    sistemáticamente favorable a L₁ por construcción y su evidencia debe
    descontarse casi por completo**. Las cifras de puertas no tienen ese
    problema y son el instrumento que hay que creer.

### 5.4 Lo que no conseguí establecer

13. **La fórmula general nD de la transformada del politopo cruzado queda como
    conjetura.** Derivé n = 1, 2, 3 y fijé la constante `2ⁿ(−1)^⌊n/2⌋` para todo
    n por normalización del volumen (comprobada hasta n = 4), pero **no
    verifiqué la forma de diferencia dividida para n ≥ 4**. Sospecho que es un
    resultado conocido por la conexión con B-splines. No lo afirmo.
14. **El decaimiento direccional está establecido para las dos direcciones
    extremas y medido en una frecuencia; no caractericé el exponente en
    dirección genérica ni deduje pertenencias `Lᵖ`.** El comentario sobre
    problemas de multiplicadores (§2.5, final) queda marcado como plausible y
    no verificado por mí.
15. **No conseguí un único enunciado de «cuándo gana L₁» más fuerte que la
    dicotomía vértices/rotaciones.** La regla que sí tengo — L₁ gana cuando la
    respuesta debe ser combinatoria y alineada con los ejes, pierde cuando debe
    ser invariante por rotaciones y suave — es correcta en todos los casos que
    examiné, pero es una descripción, no un teorema, y no sabría cómo
    falsificarla.

---

## 6. Veredicto de la línea C

Contra los criterios de cierre del plan (§5), y por separado para cada factor
de la conjunción H₁:

**Factor L₁ (métrica): CONFINADO, con frontera identificada.**

- Existe una familia de problemas donde L₁ gana de verdad y no por
  tautología de `K(F)`: optimización convexa lineal por tramos (PL, corte
  mínimo, con **cambio de clase de tratabilidad**), esparsidad, energías
  superficiales cristalinas, núcleos con símbolo racional, conteo reticular
  exacto. Las cifras medidas son de 357× a 559× menos puertas, con respuestas
  exactas donde L₂ da truncados.
- Existe una familia donde L₁ es **estructuralmente imposible**, no
  simplemente peor: todo lo que pasa por producto interno (Plancherel,
  Parseval, autoadjunción, teoría espectral, incertidumbre), por el grupo de
  rotaciones (transformada radial, reducción `n → 1`, invariancia del
  laplaciano) o por la multiplicatividad del módulo (forma polar,
  exponencial compleja como mapa geométrico, holomorfía, conformidad). Cada
  una de estas imposibilidades está respaldada por un **teorema de
  clasificación** — Hurwitz, Jordan–von Neumann, James, Pontryagin — y no por
  un fracaso de búsqueda.
- La frontera entre las dos familias es **una sola propiedad**: la bola ℓ¹ es
  un politopo con `2n` vértices alineados con los ejes. Eso da esparsidad y
  quita rotaciones. Por eso H₁ se puede confinar con precisión en vez de
  declararse vagamente «parcialmente cierta».
- Y hay un resultado que va más allá de confinar: **L₂ es reconstruible**
  (C3). `lim |zⁿ|₁^{1/n} = |z|₂` para toda norma. No es que L₂ sea la elección
  cómoda: es la que la estructura multiplicativa de ℂ devuelve sola.

**Factor dozenal (representación): REFUTADO en esta línea.**

`N_periodicos = 0` en las doce fórmulas examinadas, en las dos bases (§4). No
hay ninguna simplificación estructural atribuible a la base, y tampoco la
ganancia aritmética barata que P5 admitía, porque las fórmulas del análisis
tienen coeficientes enteros, potencias de 2 o π. El único sitio con signo
definido va **en contra**: los núcleos `4/∏(1+ξᵢ²)` generan denominadores
`1+k²` sesgados hacia múltiplos de 5, que base 12 no sabe escribir.

**H₁ en su conjunto, desde la línea C: no refutada, confinada.** Sobrevive como
enunciado sobre L₁ y solo sobre L₁, restringido a la familia combinatoria, y
**sin su segundo factor**. Dicho con las palabras que el plan (§3) pide usar si
esto pasaba: la exponencial compleja, la transformada de Fourier y la difusión
**sí** dependen de L₂ de forma esencial, y H₁ queda por tanto **confinada a la
geometría elemental y a la optimización combinatoria, y fuera del análisis**.

Para la síntesis (`99-sintesis.md`), los tres números que esta línea aporta:
**559×** (transformada de núcleo, L₂ frente a L₁, `ξ=(1,1)`), **357×**
(integrando de superficie mínima), **0** (aportación dozenal medida en toda la
línea).
