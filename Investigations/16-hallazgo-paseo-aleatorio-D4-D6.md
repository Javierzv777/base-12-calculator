# Hallazgo — Paseo aleatorio en Z² (D₄) frente a la red hexagonal (D₆): L₂ emerge al iterar, y la simetría 6 gana por un orden entero

Origen: la revisión de huecos del 2026-09-25 (la misma que abrió `15`) señaló
que el experimento 9 de `planteamientos.md` (difusión) solo se trató en el
continuo (`01-C` §3), y que el laplaciano **discreto** de 4 vecinos en Z² *es*
L₁: los vecinos de un punto son exactamente la esfera unidad L₁. Dos preguntas:

1. **¿Reaparece L₂ al iterar pasos L₁?** `99-sintesis.md` §3.1 ya lo afirmó con
   el límite de Gelfand `|zⁿ|₁^{1/n} → |z|₂`, pero la revisión crítica de §8
   reconoció que ese límite es trivial por equivalencia de normas. ¿Hay un
   mecanismo con contenido?
2. **¿Distingue algo, en el análisis y no solo en perímetro/diámetro, entre la
   simetría D₄ de L₁ y la D₆ que prefiere la base 12?** Es el antagonismo de
   `99-sintesis.md` §1 puesto a prueba en terreno nuevo.

Fecha de apertura: 2026-09-25. Protocolo de `00`: las cifras dozenales salen de
`target/release/dz-cli.exe`, con el comando al lado. La enumeración de paseos es
conteo de probabilidades en doble precisión, no aritmética dozenal. Igual que la
criba de Goldbach (`99-sintesis.md` §4.4), vive fuera del repositorio; su código
completo está en el Apéndice.

---

## 0. Resumen

| pregunta | resultado |
|---|---|
| ¿Emerge L₂ de L₁ al iterar? | **Sí, y no trivialmente.** Por el TCL, cualquier paso de varianza finita da un límite **cuadrático**; con una rotación de orden ≥ 3 en su simetría es **redondo** (Teorema 1). Es un segundo teorema de selección, hermano de Ostrowski: la multiplicación de ℂ selecciona `\|·\|₂`, y la suma iterada selecciona formas cuadráticas |
| ¿Dónde está π? | En ningún paso finito: `P(S₂ₙ = 0) = (C(2n,n)/4ⁿ)²` es racional para cada `n`. π aparece solo en el límite: `n·P → 1/π` (Wallis) en Z², y `n·P → √3/(2π)` en la red hexagonal |
| ¿D₄ o D₆? | **D₆ gana por un orden entero de isotropía.** Una capa de vecinos con simetría de orden `k` hace isótropos todos los momentos de orden `< k` (Teorema 2): D₄ llega al orden 2 y D₆ al orden 4 |
| ¿Se ve medido? | **Sí, con una predicción registrada antes de medir.** La anisotropía puntual decae como `n⁻³` en Z² y `n⁻⁵` en la red hexagonal. Pendientes medidas: **−2,994** y **−4,986**, en dos pares de puntos por red |
| ¿Y el 12? | Una red con simetría 12 sería isótropa hasta el orden 10, pero **no existe** (restricción cristalográfica, la F1 de `01-D`). D₆ es el techo de las redes |
| ¿Base 12? | Los pesos reticulares isótropos a 4.º orden (D1Q3, D2Q9, D2Q7) son todos **3-lisos**: terminan en base 12 y no en base 10. El 3 no lo elige nadie: es la **curtosis de la gaussiana**. Pasa la regla de poda de `01-D` §5.1. Pero lo trae el límite **L₂**, no L₁; base 6 empata; y a 6.º orden entra el 5 (`15 = 3·5`) y las constantes se vuelven irracionales |

---

## 1. El paseo L₁ y su retorno, exacto en cada paso

Paseo simple en Z²: pasos `±e₁, ±e₂`, cada uno con probabilidad `1/4`. El
conjunto de pasos es la esfera unidad L₁. Girar 45° y reescalar (la isometría
`L₁ ≅ L∞` del plano, `01-A`) desacopla el paseo en dos paseos 1D independientes,
de donde sale la fórmula exacta clásica

```
P(S₂ₙ = 0) = ( C(2n, n) / 4ⁿ )²          racional para todo n
```

`n·P(S₂ₙ = 0)`, con el binario:

```
$ dz-cli eval "1*(2/4)^2" --input-base 10 --mode rational                   # n=1
1/4
$ dz-cli eval "3*(20/64)^2" --input-base 10 --mode rational                 # n=3
63/194                                                                       # = 75/256
$ dz-cli eval "6*(924/4096)^2" --input-base 10 --mode rational              # n=6
78783/2134X8
$ dz-cli eval "12*(2704156/16777216)^2" --input-base 10 --mode fixed        # n=12
0;38X8512E41X9604X55809
$ dz-cli eval "1/pi" --mode fixed --scale 8
~0;39X058288E9457876455760X
```

Es la fórmula de Wallis: `C(2n,n)/4ⁿ ~ 1/√(πn)`, así que `n·P → 1/π`, con
corrección `1 − 1/(4n)` (en `n = 12`: `0,3183·(1 − 1/48) ≈ 0,3117`, lo que da
el binario). Mismo patrón que `08`: **racional en cada etapa, π solo en el
límite.** La diferencia es que aquí nadie eligió la sucesión, porque la produce
el paseo.

Los denominadores son potencias de 4, así que terminan en las dos bases.

---

## 2. Teorema 1 — el límite es cuadrático, y redondo si hay rotaciones

> **Teorema 1.** Sea `μ` una distribución de paso en `R²` con segundo momento
> finito y media cero, invariante por una rotación `R` de orden `k ≥ 3`.
> Entonces su covarianza es `Σ = σ²·I`, y `S_n/√n` converge a la gaussiana
> **isótropa** `N(0, σ²I)`.

Prueba: `Σ = RΣRᵀ`, así que `Σ` conmuta con `R`. Una rotación de ángulo
`≠ 0, π` no tiene vectores propios reales, así que la única matriz simétrica que
conmuta con ella es escalar. El resto es el TCL. ∎

Para Z²: `E[v_x²] = (1+0)/2 = 1/2` en cualquier dirección.

```
$ dz-cli eval "(1+0)/2" --mode rational              # Z², dirección de eje
1/2
$ dz-cli eval "(1+1/4+1/4)/3" --mode rational        # hexagonal: cos² de 0°, 60°, 120°
1/2
```

**Por qué esto no es el límite de Gelfand otra vez.** Gelfand da `|·|₂` con
independencia de la norma de partida, por equivalencia de normas, y no dice
nada. El TCL dice algo más fuerte y más selectivo:

- **Toda** distribución de paso de varianza finita, sea cual sea su geometría,
  converge a un límite cuya geometría es una **forma cuadrática**. Los pasos
  pueden ser L₁, L∞ o un hexágono, y el límite nunca es L₁: es una elipse.
- La simetría decide qué elipse. Con D₂ solo (pasos `±e₁`, `±2e₂`) sale una
  gaussiana elíptica. Con D₄, el círculo.

Es el segundo teorema de selección de la serie. §3.1 tenía el primero (Ostrowski
/ Gelfand–Mazur: la **multiplicación** de ℂ selecciona `|·|₂`); éste dice que la
**suma iterada** selecciona formas cuadráticas. π entra por la integral
gaussiana, que es la vuelta completa de `99-sintesis.md` §4.6 en otra forma
(casilla **I**).

La única salida conocida es la varianza infinita. Las leyes estables con medida
espectral concentrada en los ejes tienen función característica
`exp(−|t₁|^α − |t₂|^α)`; con `α = 1`, `exp(−‖t‖₁)`: L₁ sobrevive en el lado de
Fourier. Pero su densidad es el producto de dos Cauchy,
`1/(π²(1+x²)(1+y²))`, y π sigue ahí. No se persiguió más (§7).

---

## 3. Teorema 2 — orden de isotropía y simetría

La isotropía del 2.º orden es gratis desde `k = 3`. La del 4.º orden es la que
importa para la física: decide si el error del laplaciano discreto, o la
corrección de Edgeworth, depende de la dirección.

> **Teorema 2.** Sea `μ` invariante por la rotación de orden `k`, y
> `M_m(θ) = E[(u_θ · v)^m]` su momento direccional de orden `m`. Entonces
> `M_m` solo tiene armónicos múltiplos de `k`. Como `M_m` es un polinomio
> trigonométrico de grado `m`, es **constante (isótropo) para todo `m < k`**.

Prueba: `M_m(θ + 2π/k) = M_m(θ)`, y un polinomio trigonométrico periódico de
período `2π/k` solo tiene frecuencias múltiplos de `k`. ∎

| simetría | isótropo hasta orden | primer armónico anisótropo |
|---|--:|--:|
| D₄ (Z², pasos L₁) | 2 (3 es impar, se anula) | 4 |
| D₆ (red hexagonal) | 4 (5 se anula) | 6 |
| D₁₂ | 10 | 12 |

### 3.1 Los números, con el binario

Cuarto momento direccional `E[(u·v)⁴]` de una capa de vecinos unitarios:

```
$ dz-cli eval "(1+0)/2" --mode rational               # Z², u en el eje
1/2
$ dz-cli eval "(1/4+1/4)/2" --mode rational           # Z², u a 45°
1/4
$ dz-cli eval "(1+1/14+1/14)/3" --mode rational       # hexagonal, u a 0°   (1/14 = 1/16)
3/8
$ dz-cli eval "(9/14+0+9/14)/3" --mode rational       # hexagonal, u a 30°  (cos² 30° = 3/4, cos⁴ = 9/16)
3/8
```

`3/8` es justo el valor isótropo (`E[cos⁴]` sobre el círculo). Cuarto cumulante
por paso, `κ₄(u) = M₄(u) − 3·M₂²`:

```
$ dz-cli eval "1/2-3*(1/2)^2" --mode rational     # Z², eje
-1/4
$ dz-cli eval "1/4-3*(1/2)^2" --mode rational     # Z², diagonal
-1/2
$ dz-cli eval "3/8-3*(1/2)^2" --mode rational     # hexagonal, toda dirección
-3/8
```

En Z² el cuarto cumulante depende de la dirección y cambia en un factor 2 entre
el eje y la diagonal. En la red hexagonal no depende.

### 3.2 La misma cosa, vista en el laplaciano discreto

Taylor sobre los vecinos (`Σ_k (e_k·∇)² = (k/2)Δ`, y `Σ_k (e_k·∇)⁴` según la
tabla de arriba):

```
Z²:          Δ_h f = Δf + (h²/12)·(∂⁴ₓ + ∂⁴ᵧ)f + O(h⁴)      ← no invariante por rotación
hexagonal:   Δ_h f = Δf + (h²/16)·Δ²f + O(h⁴)               ← invariante
```

```
$ dz-cli eval "2/(4*3*2*1)" --mode rational                    # 2/4!: coeficiente de Z²
1/10                                                            # = 1/12
$ dz-cli eval "((9/4)/(4*3*2*1))/(3/2)" --mode rational        # (9/4)/4! ÷ (3/2): hexagonal
1/14                                                            # = 1/16
```

(`1/12` se escribe `0;1` en base 12. Es `2/4!`, un coeficiente de Taylor, y no
tiene nada de dozenal. Aplicada la regla (c) de `01-D` §5.1, no cuenta.)

### 3.3 D₄ puede comprarlo, D₆ lo tiene gratis

El Teorema 2 habla de **una** capa. Con dos capas de distinta longitud, D₄ puede
cancelar su armónico 4 a mano. Los vecinos de eje (esfera L₁, `|v| = 1`) y los
diagonales (esfera L∞, `|v| = √2`) contribuyen al armónico 4 con signo opuesto
(`cos 4θ` frente a `cos 4(θ − 45°) = −cos 4θ`), y se cancelan si el peso total de
eje es **4 veces** el diagonal. Es exactamente el modelo **D2Q9** de lattice
Boltzmann (§5): 4/9 contra 1/9. La red cuadrada alcanza el 4.º orden mezclando
L₁ con L∞ en proporción fijada. La hexagonal lo alcanza con sus seis vecinos y
nada más.

### 3.4 Y el techo lo pone la cristalografía

Un paseo soportado en una red solo puede ser invariante por rotaciones de orden
`k ∈ {1, 2, 3, 4, 6}` (restricción cristalográfica; es la F1 de `01-D`, cuyo
`mcm = 12`). Así que **D₆ es la mejor isotropía automática que admite una red**:
orden 4. La simetría 12, que llegaría al orden 10, solo existe en módulos densos
como `Z[ζ₁₂]`, de rango 4, que no son redes del plano. El 12 aparece aquí como lo
que está **prohibido**, no como lo que se alcanza.

---

## 4. Medida: la anisotropía decae como `n⁻³` en D₄ y `n⁻⁵` en D₆

**Predicción, escrita antes de correr el programa.** A `|x|` fijo, la corrección
de Edgeworth local al orden `n^{−(m*−2)/2}` lleva el polinomio de Hermite
`H_{m*}(x/√n)`, y su parte anisótropa es proporcional a
`|x|^{m*}·n^{−m*/2}·cos(m*θ)`. Aquí `m*` es el orden del primer cumulante
anisótropo. Luego

```
P(S_n = x) / P(S_n = x') − 1  ~  C · n^{−(m*−1)}      con |x| = |x'|
m* = 4 (D₄):  n⁻³              m* = 6 (D₆):  n⁻⁵
```

(Los términos isótropos no distinguen `x` de `x'` porque tienen la misma norma.
En D₆ el producto `κ₄·κ₄` es isótropo, así que no adelanta la anisotropía.)

**Medida.** Programación dinámica exacta, en doble precisión, de la ley de `S_n`
hasta `n = 601` (Apéndice). Pares de puntos de la misma norma euclidiana en
órbitas distintas del grupo de simetría:

- Z²: `(5,0)` frente a `(4,3)` (norma 5, la terna pitagórica), y `(7,4)` frente a
  `(8,1)` (norma² 65). `n` impar, por paridad.
- Hexagonal, en coordenadas de Eisenstein `a + bω`, con norma `a² − ab + b²`:
  `(7,0)` frente a `(8,5)` (norma 49: `7` frente a `(3+ω)²`), y `(10,1)` frente a
  `(6,−5)` (norma 91).

```
== square Z^2, D4 (odd n)
n	P(5,0)/P(4,3)-1	P(7,4)/P(8,1)-1
51	6.901311e-4	-3.506311e-3
101	9.071118e-5	-4.555394e-4
201	1.165515e-5	-5.834238e-5
401	1.477980e-6	-7.392065e-6
601	4.400636e-7	-2.200606e-6
pendiente local 401->601:  -2.994   -2.994

== hex (triangular) lattice, D6
n	P(7,0)/P(8,5)-1	P(10,1)/P(6,-5)-1
51	1.403195e-6	1.016493e-5
101	4.816978e-8	3.401425e-7
201	1.589593e-9	1.115178e-8
401	5.114686e-11	3.582297e-10
601	6.803447e-12	4.763567e-11
pendiente local 401->601:  -4.985   -4.986
```

Las pendientes locales tienden de forma monótona a −3 y a −5 en los cuatro pares
(tabla completa en el Apéndice). **La predicción se confirma**, y es el tipo de
cifra que el plan pedía: falsable, registrada antes y no elegida.

Control del retorno hexagonal, `P(S_n = 0) ~ √3/(2πn)`:

```
$ dz-cli eval "sqrt(3)/(2*pi)" --mode fixed --scale 8
~0;338421769946016022707274                      # = 0,275664…
```

En `n = 601` la predicción es `0,275664/601 = 4,58676·10⁻⁴`. El programa da
`4,582948·10⁻⁴`, cociente `0,99917`, con corrección de orden `1/n`. La `√3` es
intrínseca aquí, no un artefacto de la inmersión como en `15`: es `1/√det Σ` en
coordenadas de la red (pasos `±(1,0), ±(0,1), ±(1,1)`: `Σ = (1/3)[[2,1],[1,2]]`,
`det = 1/3`), y una
probabilidad de retorno no depende de cómo se dibuje la red.

---

## 5. El factor dozenal: pesos reticulares 3-lisos, y por qué

Los modelos de lattice Boltzmann eligen pesos `w_i` para que el paso reproduzca
los momentos de la gaussiana hasta el 4.º orden, que es lo que necesita
Navier–Stokes. Se derivan aquí resolviendo esas ecuaciones de momentos.

**D1Q3**, velocidades `{0, ±1}`: `2w₁ = c²` y `2w₁ = 3c⁴` (la gaussiana tiene
`E[X⁴] = 3σ⁴`). Luego `c² = 1/3`, `w₁ = 1/6`, `w₀ = 2/3`.

```
$ dz-cli eval "2*(1/6)" --mode rational          # 2w₁ = c²
1/3
$ dz-cli eval "3*(1/3)^2" --mode rational        # 3c⁴
1/3
```

**D2Q9** (eje + diagonal), con `w_eje = 1/9` y `w_diag = 1/36`:

```
$ dz-cli eval "2*(1/9)+4*(1/30)" --mode rational          # E[cₓ²] = E[cₓ⁴] = c²   (1/30 = 1/36)
1/3
$ dz-cli eval "4*(1/30)" --mode rational                  # E[cₓ²c_y²] = c⁴ = 1/9: la condición de isotropía de §3.3
1/9
$ dz-cli eval "4/9+4*(1/9)+4*(1/30)" --mode rational      # normalización
1
```

**D2Q7** (hexagonal, reposo + 6 vecinos): `E[cₓ²] = 3w = c²`, y
`E[cₓ⁴] = 6w·(3/8) = 3c⁴`. Luego `w = 1/12`, `c² = 1/4`, `w₀ = 1/2`.

```
$ dz-cli eval "6*(1/10)*(1/2)" --mode rational     # E[cₓ²] con w = 1/12
1/4
$ dz-cli eval "6*(1/10)*(3/8)" --mode rational     # E[cₓ⁴]
3/14                                                # = 3/16
$ dz-cli eval "3*(1/4)^2" --mode rational          # 3c⁴
3/14
```

| modelo | pesos | N_per base 10 | N_per base 12 |
|---|---|--:|--:|
| D1Q3 | `2/3, 1/6` | 2 | 0 |
| D2Q9 | `4/9, 1/9, 1/36` | 3 | 0 |
| D2Q7 | `1/2, 1/12` | 1 | 0 |

(`4/9 → 0;54`, `1/36 → 0;04` en el binario; en decimal `0,444…`, `0,0277…`.)

**Regla de poda de `01-D` §5.1, aplicada.** (a) ¿Se cambia eligiendo otra
unidad? No: son probabilidades, adimensionales, fijadas por ecuaciones. (b) ¿Es
un cardinal? No: son denominadores. (c) ¿Se justifica citando la divisibilidad
de 12? No: se justifica con la curtosis gaussiana `3` y, en D2Q7, con las seis
direcciones. **Pasa**, y es la primera familia de la serie que pasa la regla con
el 3 forzado desde el análisis y no desde la elección del objeto.

**Lo que hay que decir a continuación.**

- **El 3 lo trae L₂, no L₁.** Es el 4.º momento de la gaussiana, que es el límite
  **euclidiano** del Teorema 1. La aritmética `2²·3` casa aquí con la estructura
  del límite L₂. La conjunción de H₁ («L₁ + dozenal») sigue sin aparecer.
- **Base 6 empata.** Todos los denominadores son `2ᵃ3ᵇ`. Lo que se ve es «bases
  con factor 3», no «base 12». Es el control de bases que la revisión de huecos
  dejó pendiente.
- **A 6.º orden entra el 5.** La gaussiana tiene `E[X⁶] = 15σ⁶`, con
  `15 = 3·5`. Con velocidades `{0, ±1, ±2}` las ecuaciones de momentos llevan a
  `15c⁴ − 15c² + 4 = 0`, de discriminante negativo: **no hay solución real**.

```
$ dz-cli eval "15^2-4*15*4" --input-base 10 --mode rational
-13                                                 # = −15
```

  Con `{0, ±1, ±3}` sale `5c⁴ − 10c² + 3 = 0`, así que `c² = 1 ± √(2/5)`,
  **irracional**, con el 5 dentro del radical:

```
$ dz-cli eval "1-sqrt(2/5)" --mode fixed --scale 8
~0;44E149XX
$ dz-cli eval "5*(1-sqrt(2/5))^2 - X*(1-sqrt(2/5)) + 3" --mode fixed --scale 8     # X = diez
~-0;00000001E4X84124                                # 0 hasta el truncado de escala 8
```

  La ventaja 3-lisa es un fenómeno **del 4.º orden** y no sobrevive al siguiente.

Casilla: **(I) + (III)**. La gaussiana es geometría L₂; que sus momentos de orden
4 se escriban en base 12 sin período es representación.

---

## 6. Qué cambia en la síntesis

1. **§3.1 gana un mecanismo con contenido.** El límite de Gelfand era trivial.
   El Teorema 1 no lo es: la suma iterada de pasos L₁ produce una forma
   cuadrática, y D₄ la hace redonda. Es la versión aditiva de Ostrowski.
2. **El antagonismo de §1 se matiza a favor de D₆, no de L₁.** En perímetro/
   diámetro, `07` ya había visto que el hexágono gana. Aquí gana en el análisis,
   con un teorema (Teorema 2), una medida (`n⁻⁵` frente a `n⁻³`) y un techo
   (restricción cristalográfica). Es la primera vez en la serie que la simetría
   que prefiere la base 12 es también la que prefiere el análisis. L₁ necesita
   la esfera L∞ como muleta para llegar a donde D₆ llega sola.
3. **P5 sigue en pie en el espíritu.** Todo lo dozenal de este documento es
   `N_per` de denominadores 3-lisos. Lo nuevo es que el 3 viene forzado por la
   curtosis gaussiana, y que se pierde a 6.º orden.
4. **El 12 como techo prohibido.** D₁₂ sería isótropa hasta el orden 10, pero
   ninguna red la tiene. `mcm(1,2,3,4,6) = 12` es el número de simetrías que las
   redes se reparten sin poder juntarlas.

---

## 7. Lo que no funcionó, o no se hizo (obligatorio)

- **La enumeración es en doble precisión, no racional exacta.** BigInt sobre
  `1201²` celdas × 601 pasos era demasiado lento para una comprobación. Que las
  pendientes locales sean monótonas y coincidan en los dos pares de cada red
  indica que el ruido de redondeo no domina, pero en la red hexagonal, con
  `n = 601`, `|r − 1| ≈ 7·10⁻¹²` queda a solo dos o tres órdenes por encima de
  una estimación burda del redondeo acumulado (`~10⁻¹⁴`). No se acotó el error, y
  por eso no se llevó `n` más allá de 601.
- **La predicción de exponentes se derivó con Edgeworth local de forma
  esquemática**, sin comprobar las condiciones del teorema para redes
  periódicas (Bhattacharya–Rao). La medida la confirma; la prueba no está
  completa.
- **Solo 2D.** En 3D la red cúbica simple (simetría O_h, rotaciones de orden 3
  y 4) es isótropa a 2.º orden y no a 4.º con una sola capa; no se examinó si
  la FCC (12 vecinos) lo consigue. Lo que se recuerda de la literatura (hace
  falta D3Q19 o proyectar FCHC desde 4D) no se verificó aquí.
- **Las leyes estables** (§2, la única salida con L₁ en el límite) se citaron y
  no se midieron.
- **Base 6 no se probó por separado.** Se afirma que empata porque los
  denominadores son `2ᵃ3ᵇ`. Es cierto por aritmética, pero el control de bases
  sigue pendiente como hilo propio.
- **No se midió coste en puertas.** Las comparaciones de coste de esta serie
  miden sobre todo la implementación de `sqrt` y `pi` (`99-sintesis.md` §5), y
  aquí no hay una fórmula L₁ y otra L₂ del mismo problema que comparar.
- **Casi seguro que nada de esto es nuevo.** El 4.º orden de isotropía hexagonal
  es la razón de ser del gas reticular FHP (Frisch–Hasslacher–Pomeau, 1986)
  frente al HPP cuadrado; D2Q9 es de Qian–d'Humières–Lallemand (1992); la
  isotropía de tensores en redes es de Wolfram (1986). Lo que aporta este
  documento es situarlo dentro del marco (I)/(II)/(III) de la serie y medirlo
  con una predicción registrada antes. Sin revisión externa, como el resto.

---

## Apéndice — script de enumeración (Node ≥ 18, fuera del repositorio)

```js
// node walk.mjs 601
const NMAX = +process.argv[2] || 401;
const checkpoints = new Set([25, 51, 101, 151, 201, 301, 401, 601].filter(n => n <= NMAX));
function run(steps, pairs, name) {
  const R = NMAX + 1, W = 2 * R + 1, idx = (x, y) => (x + R) * W + (y + R);
  let cur = new Float64Array(W * W), nxt = new Float64Array(W * W);
  cur[idx(0, 0)] = 1;
  const p = 1 / steps.length, rows = [];
  for (let n = 1; n <= NMAX; n++) {
    nxt.fill(0);
    for (let x = -n + 1; x <= n - 1; x++) for (let y = -n + 1; y <= n - 1; y++) {
      const v = cur[idx(x, y)]; if (v === 0) continue;
      for (const [dx, dy] of steps) nxt[idx(x + dx, y + dy)] += v * p;
    }
    [cur, nxt] = [nxt, cur];
    if (checkpoints.has(n)) rows.push([n, cur[idx(0, 0)], ...pairs.map(([a, c]) => cur[idx(...a)] / cur[idx(...c)] - 1)]);
  }
  console.log(`\n== ${name}`);
  for (const r of rows) console.log(r.join('\t'));
  for (let i = 1; i < rows.length; i++)
    console.log(`${rows[i-1][0]}->${rows[i][0]}\t` + pairs.map((_, j) =>
      ((Math.log(Math.abs(rows[i][2+j])) - Math.log(Math.abs(rows[i-1][2+j]))) /
       (Math.log(rows[i][0]) - Math.log(rows[i-1][0]))).toFixed(3)).join('\t'));
}
run([[1,0],[-1,0],[0,1],[0,-1]], [[[5,0],[4,3]], [[7,4],[8,1]]], 'square Z^2, D4 (odd n)');
run([[1,0],[-1,0],[0,1],[0,-1],[1,1],[-1,-1]], [[[7,0],[8,5]], [[10,1],[6,-5]]], 'hex (triangular) lattice, D6');
```

Salida completa de pendientes locales (`d log|r−1| / d log n`):

```
square:  25->51 -2.965 -3.025 | 51->101 -2.970 -2.987 | 101->151 -2.980 -2.985
         151->201 -2.985 -2.988 | 201->301 -2.989 -2.990 | 301->401 -2.992 -2.993 | 401->601 -2.994 -2.994
hex:     25->51 -4.952 -5.114 | 51->101 -4.935 -4.972 | 101->151 -4.952 -4.964
         151->201 -4.963 -4.969 | 201->301 -4.973 -4.976 | 301->401 -4.980 -4.982 | 401->601 -4.985 -4.986
```

Tiempo: 18 s para las dos redes hasta `n = 601`.
