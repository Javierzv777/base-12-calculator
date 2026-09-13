# Hallazgos — Línea D: dozenal puro

Pregunta 10 de `planteamientos.md` + la lista de tareas de H₁. Protocolo:
`00-plan-investigacion.md`. Cifras dozenales compartidas: `02-referencia-medida.md`.

Esta línea aísla el **factor base 12** de la hipótesis H₁ del factor geometría.
Su predicción registrada es **P5** (el efecto dozenal se reduce a `N_periodicos`
más coste de lectura de literales) y **P6** (Goldbach no cede nada; lo máximo
defendible es el último dígito dozenal de los primos en `{1,5,7,↋}`).

Todas las cifras dozenales de este documento salen de
`target/release/dz-cli.exe`, con el comando al lado. Ninguna se convirtió a
mano.

**Aviso de alfabeto que gobierna toda la lectura de este fichero.** Un literal
suelto en `dz-cli` se lee **en base 12**. `1/24` es un veintiochoavo, no un
veinticuatroavo. Donde este documento escribe un denominador decimal, el
comando lleva `--input-base 10`; donde escribe un denominador dozenal, no lo
lleva. Los dos casos están marcados en cada tabla.

---

## 0. Resumen ejecutivo

| | veredicto |
|---|---|
| **P5** | **sobrevive en el espíritu, cae en la letra.** El efecto dozenal no es «`N_periodicos` + coste de lectura»: es **un solo mecanismo** (la factorización 12 = 2²·3) que se manifiesta de **cuatro** maneras medibles, dos de ellas estructurales y no listadas en P5: los **recíprocos exactos de un dígito** para 2, 3, 4 y 6, que sustituyen el circuito de división con un ahorro medido de **3,0×–3,7×** (§3.3), y el test de divisibilidad por 2, 3, 4, 6, 12 en `O(1)` frente a `O(nº de cifras)` (§3.4). Pero ninguna de las cuatro es evidencia *a favor de 12*: toda base `b` las obtiene para los divisores de `b`. El lead más prometedor —escalado gratis por `12^k`— resultó **falso en la medición** (§3.1). |
| **P6** | **confirmada, con un afilado modesto.** El hecho es una reformulación de la coprimalidad con 12, no un descubrimiento. El afilado real: en base 12 dos clases de resto (`n ≡ 4` y `n ≡ 8`) admiten **una sola** pareja de últimos dígitos, mientras que en base 10 las cinco clases admiten siempre **dos**. La aritmética modular dozenal es no uniforme y la decimal es uniforme. Es información, pero es información sobre `(Z/12)*`, no sobre los primos. |
| **familias** | 13 candidatas examinadas, **8 sobreviven** (2 de ellas solo parcialmente), **5 podadas**. |
| **diferencial de lectura** | 1.784 puertas a 2 cifras, 5.352 a 3, 11.310 a 4 (base 10 frente a base 12, mismo valor). Y el resultado más fuerte de la línea: la **profundidad** de leer base 12 es **constante en 254** para cualquier número de cifras, mientras la de base 10 crece 254 → 478 → 816 → 1.187. |
| **¿hizo base 12 algo más que `N_periodicos`?** | **Sí, dos cosas, y las dos son genéricas.** Ver §4. |
| **hallazgo transversal no previsto** | **Los dos factores de H₁ son antagónicos, no independientes.** Ver §1.0. |

---

## 1. Parte 1 — las familias de particiones 2, 3, 4, 6, 12

### 1.0 Hallazgo previo a toda tabla: L₁ y base 12 tiran en direcciones opuestas

Antes de recorrer las familias hay que decir algo que la formulación de H₁ no
anticipa y que condiciona todo lo que sigue.

La bola unitaria de L₁ en el plano es el rombo `|x| + |y| = 1`. Su grupo de
simetrías es el **diedral D₄, de orden 8**: las particiones que esa métrica
regala son en **2, 4 y 8**. No hay ninguna partición en 3, 6 ni 12 que L₁
distinga: el rombo no tiene ejes a 30° ni a 60°.

Ahora véase cuáles son las familias en las que el número 12 está **forzado por
las matemáticas** y no elegido por convenio: la restricción cristalográfica
(`n ∈ {1,2,3,4,6}`, mcm = 12), los enteros de Eisenstein, el empaquetamiento
hexagonal, las raíces duodécimas de la unidad, el peso 12 modular. Todas ellas
son familias de simetría **hexagonal o triádica**. Es decir: **exactamente las
familias donde 12 es forzado son las familias donde L₁ es la métrica
equivocada.** La norma cuya bola unitaria es un hexágono regular no es L₁ ni L₂
ni ninguna L_p: es una norma poligonal distinta.

Consecuencia para H₁, que es una conjunción: sus dos factores no son
independientes, son **antagónicos**. El factor L₁ privilegia potencias de 2; el
factor 12 privilegia 2²·3. Las familias que maximizan uno minimizan el otro.
Esto no refuta H₁ por sí solo —una conjunción puede cumplirse sin que sus
factores colaboren— pero destruye la motivación narrativa que la sostiene, que
era que ambos apuntaran al mismo sitio.

Por eso en las tablas que siguen la columna L₁ está a menudo vacía o marcada
«no aplica»: en una familia de simetría triádica la reformulación L₁ no es
«más simple», es **otra cosa**.

### 1.1 Criterio de poda, declarado antes de aplicarlo

Una familia se poda si el número 12 aparece por **convenio de unidades** o como
**cardinal sin contenido aritmético**. La prueba es contrafáctica: *si la
humanidad hubiera elegido otra unidad, ¿seguiría apareciendo el 12?* Si la
respuesta es no, no es evidencia.

Un cardinal (hay 12 pentominós, el cubo tiene 12 aristas) tampoco es evidencia,
por una razón distinta: un cardinal no es una **partición**. H₁ habla de
particiones en 2, 3, 4, 6, 12 —es decir, de **denominadores**— y un cardinal no
aparece nunca en un denominador. Que base 12 escriba ese cardinal como `10` es
una coincidencia de magnitud, no una simplificación.

### 1.2 Las 13 familias candidatas

| # | familia | 12 forzado? | veredicto |
|--:|---|---|---|
| F1 | Restricción cristalográfica: órdenes de rotación de una retícula, `n ∈ {1,2,3,4,6}`, mcm = 12 | **sí**, teorema | **sobrevive** |
| F2 | División del ángulo: brújula, reloj, rosa de los vientos | **parcial**: los denominadores sí, el «360» no | **sobrevive parcialmente** |
| F3 | Raíces duodécimas de la unidad, `Φ₁₂(x) = x⁴ − x² + 1`, `(Z/12)* = {1,5,7,↋}` | **sí**, teoría de cuerpos | **sobrevive** (y no gana nada: ver 1.3.3) |
| F4 | Enteros de Eisenstein, empaquetamiento hexagonal, número de besos 12 en FCC | **sí** | **sobrevive** |
| F5 | Subdivisión diádica–triádica; racionales de denominador 3-liso | **sí**, por definición de la base | **sobrevive** (es el caso (III) puro) |
| F6 | Euler–Maclaurin, Bernoulli, Newton–Cotes: 1/6, 1/12, 1/24, 1/30, 1/42 | **sí** | **sobrevive parcialmente** |
| F7 | Formas modulares de peso 12: `Δ`, `η = q^{1/24}∏`, `j = 1728·J`, `ζ(−1) = −1/12` | **sí** | **sobrevive** |
| F8 | Temperamento igual de 12 tonos: `7/12` como convergente de `log₂(3/2)` | **sí**, fracciones continuas | **sobrevive** (mejor caso de la línea) |
| F9 | Calendario y tiempo: 12 meses, 24 horas, 60 minutos | **no** | **podada** |
| F10 | Sistemas de medida: 12 pulgadas, docena, gruesa | **no** | **podada** |
| F11 | Los 12 pentominós libres | cardinal | **podada** |
| F12 | Las 12 aristas del cubo / 12 vértices del cuboctaedro | cardinal | **podada** |
| F13 | Las 32 clases cristalográficas de grupos puntuales | duplica F1 | **podada** |

Ocho sobreviven, cinco caen. Las razones de las cinco poda están en §5.

### 1.3 Las ocho familias supervivientes, con su tabla K(F)

Recordatorio de la regla de conteo (§2.1 del plan): `N_periodicos` es el único
sumando sensible a la base, y por tanto el único que puede variar entre las
columnas «decimal» y «dozenal» de la *misma* fórmula. Cuando en una tabla varía
otro sumando, eso es exactamente el hallazgo que hay que señalar, y en toda
esta línea ocurre **una sola vez** (F8).

#### F1 — Restricción cristalográfica

**Enunciado L₂.** Una isometría de una retícula de `R²` tiene orden
`n ∈ {1,2,3,4,6}`, porque la traza `2cos(2π/n)` debe ser entera.
**Enunciado L₁.** No aplica en el sentido de H₁: el grupo de simetrías de la
retícula cuadrada con métrica L₁ es `D₄` (orden 8) y el caso `n = 3, 6`
simplemente no existe. L₁ **elimina** la familia en vez de simplificarla.

Forma normalizada de la que se calcula K: los ángulos admisibles como
**fracción de vuelta**, `{1, 1/2, 1/3, 1/4, 1/6}`, y su mínimo común
denominador, `1/12`.

| sumando | decimal | dozenal |
|---|--:|--:|
| `N_raices` | 0 | 0 |
| `N_irracionales` | 0 | 0 |
| `N_operaciones` | 4 | 4 |
| `N_periodicos` | **2** (`1/3` = 0.333…, `1/6` = 0.1666…) | **0** |
| **K** | **6** | **4** |

```
$ target/release/dz-cli.exe eval "1/3" --input-base 10 --mode fixed
0;4
$ target/release/dz-cli.exe eval "1/6" --input-base 10 --mode fixed
0;2
$ target/release/dz-cli.exe eval "1/12" --input-base 10 --mode fixed
0;1
```

**Veredicto: confirma H₁, casilla (III), ganancia enteramente de
`N_periodicos`.** Nada estructural. Y con un coste que H₁ no contabiliza: el
factor L₁ de la conjunción *destruye* esta familia.

#### F2 — División del ángulo, brújula y reloj

**Enunciado L₂.** Las direcciones notables de la brújula y las horas del reloj
son los múltiplos de `1/12` de vuelta; las de la rosa de 8 vientos, múltiplos
de `1/8`.
**Enunciado L₁.** El «ángulo taxicab» que la línea B estudia parametriza el
rombo por perímetro recorrido; sus puntos naturales son los múltiplos de `1/8`
de perímetro (los 4 vértices y los 4 puntos medios). La rosa de 8 vientos
sobrevive a L₁; el reloj de 12 horas no.

| sumando | decimal | dozenal |
|---|--:|--:|
| `N_raices` | 0 | 0 |
| `N_irracionales` | 0 | 0 |
| `N_operaciones` | 1 | 1 |
| `N_periodicos` (ángulos `k/12`, `k = 1…11`) | **8** (todos salvo `3/12`, `6/12`, `9/12`) | **0** |
| **K** | **9** | **1** |

```
$ target/release/dz-cli.exe eval "5/12" --input-base 10 --mode fixed
0;5
$ target/release/dz-cli.exe eval "7/12" --input-base 10 --mode fixed
0;7
$ target/release/dz-cli.exe eval "11/12" --input-base 10 --mode fixed
0;E
```

Es la ganancia numérica más grande de toda la línea, y también la más vacía:
`k/12` en base 12 es `0;k` **por definición de la base**. La tabla mide una
tautología. Se incluye precisamente para dejar constancia de que `N_periodicos`
puede inflarse a voluntad eligiendo una familia de constantes cuyo denominador
sea la base.

**Veredicto: confirma H₁ formalmente, casilla (III), y es un caso de
sobreajuste del índice.** «Doce» no es un hallazgo sobre el reloj: el reloj es
de doce horas por convenio babilónico, y lo único forzado aquí es que
`{2,3,4,6}` dividan la vuelta —lo cual se debe a F1, no al reloj.

#### F3 — Raíces duodécimas de la unidad

**Enunciado L₂.** `ζ₁₂ = e^{2πi/12} = √3/2 + i/2`; `Φ₁₂(x) = x⁴ − x² + 1`;
`(Z/12)* = {1, 5, 7, 11} ≅ Z₂ × Z₂`.
**Enunciado L₁.** No aplica: el módulo L₁ de `ζ₁₂` es `√3/2 + 1/2 ≈ 1;36…`,
no 1. Las raíces de la unidad **no están en la circunferencia L₁**, así que la
familia no se traslada; la «raíz de la unidad L₁» sería otro objeto.

| sumando | decimal | dozenal |
|---|--:|--:|
| `N_raices` | 1 (`√3`) | 1 |
| `N_irracionales` | 1 (`√3`) | 1 |
| `N_operaciones` | 3 (`÷`, `+`, `×i`) | 3 |
| `N_periodicos` | 0 (`1/2` = 0.5 **termina en decimal**) | 0 (`0;6`) |
| **K** | **5** | **5** |

```
$ target/release/dz-cli.exe eval "sqrt(3)/2" --mode fixed --scale 12
~0;X485X9EEX9436
$ target/release/dz-cli.exe eval "1/2" --input-base 10 --mode fixed
0;6
```

**Veredicto: no concluyente tirando a refuta. ΔK = 0.** La familia cuyo nombre
literalmente contiene el número 12 **no gana nada** al escribirse en base 12.
La razón es instructiva: sus constantes tienen denominador 2, y 2 divide a las
dos bases. `Φ₁₂` tiene coeficientes enteros y es idéntica en toda base. El único
cambio es cosmético: `(Z/12)*` se escribe `{1,5,7,↋}` y esos son los cuatro
dígitos dozenales coprimos con la base — que es el contenido de P6 (§2).

Este es el dato más limpio a favor de P5 de toda la línea, y conviene subrayar
por qué: si hubiera un efecto dozenal *estructural*, ésta es la familia donde
tendría que aparecer.

#### F4 — Enteros de Eisenstein y empaquetamiento hexagonal

**Enunciado L₂.** `Z[ω]`, `ω = −1/2 + i√3/2`, norma `N(a+bω) = a² − ab + b²`;
6 unidades; 6 vecinos más próximos en el plano; número de besos 12 en FCC/HCP;
densidad `π/√12 = π√3/6 ≈ 0,9069`.
**Enunciado L₁.** No aplica, y por la razón de §1.0: la retícula de Eisenstein
es hexagonal y L₁ no tiene ejes a 60°. La «norma L₁ de Eisenstein»
`|a| + |b|` no es multiplicativa, así que se pierde lo único que hace de `Z[ω]`
un dominio euclídeo. Pérdida de propiedad, no simplificación.

| sumando | decimal | dozenal |
|---|--:|--:|
| `N_raices` (densidad `π√3/6`) | 1 | 1 |
| `N_irracionales` | 2 (`π`, `√3`) | 2 |
| `N_operaciones` | 2 | 2 |
| `N_periodicos` | **1** (`1/6` = 0.1666…) | **0** (`0;2`) |
| **K** | **6** | **5** |

```
$ target/release/dz-cli.exe eval "1/6" --input-base 10 --mode fixed
0;2
```

**Caveat que invalida media ganancia, y hay que decirlo.** `ΔK = −1` solo si la
densidad se escribe `π√3/6`. Escrita `π/√12` —la forma más habitual en la
literatura— `N_periodicos = 0` en **las dos** bases y `ΔK = 0`. Es decir:
**K(F) no es invariante bajo reescritura algebraica**, y una parte de cualquier
`ΔK` publicado en esta investigación se puede fabricar eligiendo la forma. Es
una limitación del instrumento, no de esta familia, y afecta a las cuatro
líneas.

**Veredicto: no concluyente.** La única ganancia es `N_periodicos`, y es
frágil ante la forma elegida. La estructura (multiplicatividad de la norma,
6 unidades, besos 12) es idéntica en toda base.

#### F5 — Subdivisión diádica–triádica

**Enunciado.** `x ∈ Q` tiene expansión finita en base `b` si y solo si los
primos del denominador reducido dividen a `b`. Para `b = 12` eso es
exactamente el conjunto de denominadores **3-lisos** (`2^a 3^c`), es decir, el
cierre de «partir en mitades y en tercios».

Esta familia no tiene un K(F) porque no es una fórmula: **es el enunciado del
efecto dozenal mismo.** Se incluye para fijar que todo lo demás de esta línea
es un corolario suyo. Tabla en los dos sentidos, citada de
`02-referencia-medida.md` §1 y reproducida aquí con denominadores decimales:

| denominador (decimal) | factorización | base 12 | base 10 | quién gana |
|--:|---|---|---|---|
| 2 | 2 | `0;6` | `0.5` | empate |
| 3 | 3 | `0;4` | `0.333…` | **12** |
| 4 | 2² | `0;3` | `0.25` | empate |
| 6 | 2·3 | `0;2` | `0.1666…` | **12** |
| 8 | 2³ | `0;16` | `0.125` | empate |
| 9 | 3² | `0;14` | `0.111…` | **12** |
| 12 | 2²·3 | `0;1` | `0.08333…` | **12** |
| 16 | 2⁴ | `0;09` | `0.0625` | empate |
| 18 | 2·3² | `0;08` | `0.0555…` | **12** |
| 24 | 2³·3 | `0;06` | `0.041666…` | **12** |
| 36 | 2²·3² | `0;04` | `0.0277…` | **12** |
| 144 | 2⁴·3² | `0;01` | `0.00694…` | **12** |
| **5** | 5 | `0;2497 2497…` **no termina** | `0.2` | **10** |
| **10** | 2·5 | `0;1249 7249…` **no termina** | `0.1` | **10** |
| **20** | 2²·5 | `0;0724 9724…` **no termina** | `0.05` | **10** |
| **25** | 5² | `0;0591 5343 X0E6…` **no termina** | `0.04` | **10** |
| **100** | 2²5² | `0;0153 43X0 E62X…` **no termina** | `0.01` | **10** |
| 7 | 7 | `0;186X35 186X35…` | `0.142857…` | empate |
| 11 | 11 | `0;1111 1111…` | `0.0909…` | empate |
| 13 | 13 | `0;0E0E 0E0E…` | `0.076923…` | empate |
| 30 | 2·3·5 | `0;0497 2497…` | `0.0333…` | empate |
| 42 | 2·3·7 | `0;0351 86X3 5186…` | `0.0238095…` | empate |
| 90 | 2·3²·5 | `0;0172 4972 4972…` | `0.0111…` | empate |
| 720 | 2⁴·3²·5 | `0;0024 9724 9724…` | `0.00138…` | empate |

```
$ for f in 1/3 1/5 1/10 1/24 1/25 1/100 1/13 1/30 1/720; do \
    target/release/dz-cli.exe eval "$f" --input-base 10 --mode fixed; done
0;4
0;249724972497249724972497 (periodica en base 12, valor truncado)
0;124972497249724972497249 (periodica en base 12, valor truncado)
0;06
0;05915343X0E62X68781E0591 (periodica en base 12, valor truncado)
0;015343X0E62X68781E059153 (periodica en base 12, valor truncado)
0;0E0E0E0E0E0E0E0E0E0E0E0E (periodica en base 12, valor truncado)
0;049724972497249724972497 (periodica en base 12, valor truncado)
0;002497249724972497249724 (periodica en base 12, valor truncado)
```

**La pérdida es simétrica y del mismo tamaño que la ganancia.** Base 12 gana en
los denominadores 3-lisos y **pierde** en todos los que llevan un 5. Cuál de
las dos domina en la práctica no es un hecho matemático: depende de qué
fórmulas se consideren, y esa elección es del investigador. Una tabla que
enseñara solo `1/3` sería propaganda.

**Nota sobre el instrumento, no publicada antes.** `N_periodicos` es
**binario**: cuenta si termina o no, y es ciego a la **longitud del período**.
Los datos de arriba muestran que esa ceguera oculta un efecto real:

| n | período en base 12 | período en base 10 |
|--:|--:|--:|
| 7 | 6 | 6 |
| 11 | **1** | 2 |
| 13 | **2** | 6 |
| 5 | 4 | — (termina) |

`1/11 = 0;1111…` (período 1, porque 11 = 12−1) y `1/13 = 0;0E0E…` (período 2,
porque 13 = 12+1 y 13 | 12²−1). En base 10, `1/13` tiene período 6. Es decir:
base 12 **sí** representa 11 y 13 con períodos mucho más cortos, y K(F) no lo
ve. Es una ganancia real y pequeña, y sigue siendo casilla (III) y sigue siendo
un corolario de la factorización de la base (de `12²−1 = 11·13`). No la uso
para reclamar nada: la señalo como defecto del índice.

**Veredicto: confirma la parte dozenal de H₁ y a la vez la vacía.** Todo el
efecto es aquí, y es un hecho sobre la factorización de la base.

#### F6 — Euler–Maclaurin, Bernoulli, Newton–Cotes

**Enunciado L₂.** Corrección trapezoidal `−(b−a)h²/12·f''(ξ)`; Simpson con
pesos normalizados `{1/6, 4/6, 1/6}`; `B₂ = 1/6`, `B₄ = −1/30`, `B₆ = 1/42`,
`B₁₂ = −691/2730`; Boole con `{7/90, 32/90, 12/90}`.
**Enunciado L₁.** No aplica: la cuadratura es unidimensional y la métrica del
plano no interviene. Familia puramente (III).

| sumando (Simpson) | decimal | dozenal |
|---|--:|--:|
| `N_raices` | 0 | 0 |
| `N_irracionales` | 0 | 0 |
| `N_operaciones` | 4 | 4 |
| `N_periodicos` | **2** (`1/6`, `4/6`) | **0** (`0;2`, `0;8`) |
| **K** | **6** | **4** |

```
$ target/release/dz-cli.exe eval "1/6" --input-base 10 --mode fixed
0;2
$ target/release/dz-cli.exe eval "4/6" --input-base 10 --mode fixed
0;8
$ target/release/dz-cli.exe eval "1/12" --input-base 10 --mode fixed
0;1
$ target/release/dz-cli.exe eval "1/30" --input-base 10 --mode fixed
0;049724972497249724972497 (periodica en base 12, valor truncado)
$ target/release/dz-cli.exe eval "1/42" --input-base 10 --mode fixed
0;035186X35186X35186X35186 (periodica en base 12, valor truncado)
$ target/release/dz-cli.exe eval "1/90" --input-base 10 --mode fixed
0;017249724972497249724972 (periodica en base 12, valor truncado)
$ target/release/dz-cli.exe eval "691/2730" --input-base 10 --mode fixed
0;30546902X21E50546902X21E (periodica en base 12, valor truncado)
```

**Veredicto: confirma parcialmente, y la parte que falla es la mayoría de la
serie.** Base 12 gana en `B₂ = 1/6`, en la corrección trapezoidal `1/12` y en
los pesos de Simpson. **Empata desde `B₄` en adelante**: `30 = 2·3·5`,
`42 = 2·3·7`, `90 = 2·3²·5`, `2730 = 2·3·5·7·13` — todos llevan un 5 o un 7 y
ninguna de las dos bases los termina. La familia gana en sus tres constantes
más usadas y no gana en ninguna otra. Casilla (III), todo `N_periodicos`.

#### F7 — Formas modulares de peso 12

**Enunciado L₂.** `Δ(τ) = q∏(1−qⁿ)²⁴` es la forma cuspidal de peso 12 de nivel
mínimo; `η(τ) = q^{1/24}∏(1−qⁿ)`; `j = E₄³/Δ` se normaliza con **1728**;
`ζ(−1) = −1/12`; la dimensión crítica de la cuerda bosónica es `24 + 2`.
**Enunciado L₁.** No aplica en absoluto: el semiplano superior con
`SL₂(Z)` es geometría hiperbólica y la métrica que interviene es la de
Poincaré, no L₁ ni L₂ del plano. L₁ no tiene nada que decir aquí.

| sumando | decimal | dozenal |
|---|--:|--:|
| `N_raices` | 0 | 0 |
| `N_irracionales` | 0 | 0 |
| `N_operaciones` | 3 | 3 |
| `N_periodicos` (`1/24`, `1/12`, `1/1728`) | **3** | **0** |
| **K** | **6** | **3** |

```
$ target/release/dz-cli.exe eval "1/24" --input-base 10 --mode fixed
0;06
$ target/release/dz-cli.exe eval "1/12" --input-base 10 --mode fixed
0;1
$ target/release/dz-cli.exe eval "1/1728" --input-base 10 --mode fixed
0;001
$ target/release/dz-cli.exe eval "1728" --input-base 10 --mode fixed
1000
$ target/release/dz-cli.exe eval "10^3" --mode fixed
1000
$ target/release/dz-cli.exe eval "24" --input-base 10 --mode fixed
20
```

Dos observaciones que van más allá de `N_periodicos` y que son las mejores de
la Parte 1 después de F8:

1. **1728 = 12³ se escribe `1000`.** La constante de normalización del
   invariante `j` es un número redondo en base 12, de cuatro cifras en vez de
   cuatro, pero con tres ceros: `1/1728 = 0;001`, tres cifras, frente a
   `0.000578703703…`, infinitas. El exponente 24 se escribe `20`, es decir «dos
   docenas», y el peso 12 se escribe `10`. La familia entera se lee en base 12
   como potencias y múltiplos pequeños de la base.
2. **K(F) no mide esto.** `N_periodicos` captura `1/1728` pero no captura que
   el entero `1728` pase de 4 cifras significativas a `1` seguido de ceros.
   Falta un sumando `N_cifras`. Lo señalo como defecto y **no** lo uso para
   reclamar ganancia: añadir términos al índice después de ver los datos es
   precisamente el modo de fallo que el plan quiere evitar.

**Veredicto: confirma, casilla (III), ganancia de `N_periodicos` más una
compresión de cifras que el índice no cuenta.** Nada estructural: el peso 12 de
`Δ` es 12 en toda base, y `dim S₁₂(SL₂(Z)) = 1` es un teorema sin dígitos.

#### F8 — Temperamento igual de 12 tonos — el mejor caso de la línea

**Enunciado L₂ / decimal.** El número de semitonos que aproxima un intervalo
justo de razón `r` es `k = round(12·log₂ r)`. Que 12 sea el número correcto de
divisiones no es convenio: `7/12` es un **convergente de la fracción continua**
de `log₂(3/2) = 0,58496…` (convergentes `1/2, 3/5, 7/12, 24/41`), y 12 es el
denominador donde el error cae por debajo de un coma por primera vez. Es
matemática forzada.

**Enunciado L₁.** No aplica: no hay geometría del plano en un intervalo
musical.

**Enunciado dozenal.** `k` = **el primer dígito dozenal de `log₂ r`,
redondeado**. La multiplicación por 12 desaparece: en base 12 multiplicar por
12 es mover el punto.

Medido:

```
$ for f in 3/2 4/3 5/4 6/5 5/3 8/5; do \
    target/release/dz-cli.exe eval "ln($f)/ln(2)" --mode fixed --scale 6; done
~0;70299464077E126369891074
~0;4E922757E440X9585232XE47
~0;3X43602701185449885X644E
~0;31X6343906627X15X12X6824
~0;8X158782E55941X61X915397
~0;81785E94EXX367723361577
```

| intervalo justo | `log₂ r` en base 12 | primer dígito, redondeado | semitonos en 12-TET | ¿coincide? |
|---|---|--:|--:|:-:|
| quinta `3/2` | `~0;70299…` | 7 | 7 | **sí** |
| cuarta `4/3` | `~0;4E922…` | 5 | 5 | **sí** |
| tercera mayor `5/4` | `~0;3X436…` | 4 | 4 | **sí** |
| tercera menor `6/5` | `~0;31X63…` | 3 | 3 | **sí** |
| sexta mayor `5/3` | `~0;8X158…` | 9 | 9 | **sí** |
| sexta menor `8/5` | `~0;81785…` | 8 | 8 | **sí** |

Seis de seis. Y el contraste con decimal es nítido: truncar `log₂(3/2)` a una
cifra decimal da `0,5 = 1/2`, que es el convergente **anterior** y el
temperamento **equivocado**; truncar a una cifra dozenal da `0;7 = 7/12`, que
es la respuesta correcta.

```
$ target/release/dz-cli.exe eval "7/12" --input-base 10 --mode fixed
0;7
```

| sumando | decimal | dozenal |
|---|--:|--:|
| `N_raices` | 0 | 0 |
| `N_irracionales` | 1 (`log₂ r`) | 1 |
| `N_operaciones` | **1** (`× 12`) | **0** (es un desplazamiento) |
| `N_periodicos` (`k/12`, genéricamente) | 1 | 0 |
| **K** | **3** | **1** |

**Veredicto: confirma H₁, y es el único caso de la línea donde el sumando que
baja no es `N_periodicos`.** `N_operaciones` baja en 1, porque en base 12
multiplicar por 12 es mover el punto y en base 10 es una multiplicación.

**Pero la ganancia es simbólica y no se realiza en el coste medido, y esto es
importante bastante como para corregirlo aquí mismo.** Medí si escalar por la
base es gratis en este motor y **no lo es**: `345*10` cuesta 97.252 puertas,
*más* que `345*E` (96.037), porque el bucle del multiplicador recorre los
dígitos del segundo operando y `12` escrito `10` tiene dos. El detalle y las
seis mediciones están en §3.1. El ahorro por desplazamiento existe dentro de
`dz-sci`, que mueve el campo `scale` a mano, pero **no es accesible desde una
expresión**, así que la fila `N_operaciones` de esta tabla registra una ventaja
simbólica sin respaldo medido.

Y aun si se realizara, hay que ser honesto sobre su alcance: la operación que
desaparece es «multiplicar por la base», y **toda** base la elimina para sí
misma. Si el temperamento tuviera 10 divisiones, base 10 haría lo mismo. El
mérito no sería de 12, sino de haber emparejado la base con el módulo del
problema.

### 1.4 Resumen de la Parte 1

| familia | ΔK | de qué sumando | casilla | veredicto |
|---|--:|---|:-:|---|
| F1 cristalográfica | −2 | `N_periodicos` | (III) | confirma |
| F2 ángulo/reloj | −8 | `N_periodicos` | (III) | confirma (sobreajustado) |
| F3 raíces 12-ésimas | **0** | — | (III) | **refuta** la expectativa |
| F4 Eisenstein | −1 o **0** | `N_periodicos`, frágil ante la forma | (III) | no concluyente |
| F5 diádica–triádica | — | es el enunciado del efecto | (III) | confirma y vacía |
| F6 Euler–Maclaurin | −2 (solo las 3 primeras constantes) | `N_periodicos` | (III) | confirma parcialmente |
| F7 peso 12 modular | −3 | `N_periodicos` (+ cifras, no contadas) | (III) | confirma |
| F8 12-TET | −2 | `N_periodicos` **y `N_operaciones`** (esta última sin respaldo medido: §3.1) | (III) | confirma, único caso con un sumando no periódico |

**Ocho familias, ocho casillas (III). Cero casillas (II) generadas por la base,
como debe ser: una casilla (II) es un hallazgo sobre la métrica y esta línea no
toca la métrica.** Siete de las ocho ganan solo por `N_periodicos`. Una (F8)
gana también una operación. Ninguna gana un radical ni un irracional: **base 12
no ha eliminado ni una sola raíz ni una sola constante irracional en toda la
línea**, y no podía, porque la irracionalidad no depende de la base.

---

## 2. Parte 2 — Goldbach y base 12

Nada de π en esta sección, según la instrucción de `planteamientos.md` §10: π no
es central en Goldbach y forzarlo sería el fallo que este plan quiere evitar.
La pregunta es si una representación dozenal revela estructura en `n = p + q`.

**Metodología.** La generación de primos y el recuento de parejas se hacen con
`awk` (es contabilidad en decimal, permitida por §2.2 del plan). **Toda**
conversión decimal → dozenal sale de `dz-cli`: se construyó primero una tabla
`p → grafía dozenal` para los 109 primos ≤ 600 y otra `n → grafía dozenal` para
los 299 pares `4 ≤ n ≤ 600`, ambas con 408 invocaciones del binario, y el
`awk` solo **lee el último carácter** de esas grafías. Ningún `% 12` produce
ninguna cifra publicada aquí.

```
$ for p in $(cat primes.txt); do \
    echo "$p $(target/release/dz-cli.exe eval "$p" --input-base 10 --mode fixed)"; done
$ n=4; while [ $n -le 600 ]; do \
    echo "$n $(target/release/dz-cli.exe eval "$n" --input-base 10 --mode fixed)"; n=$((n+2)); done
```

### 2.1 Verificación de P6: el último dígito dozenal de los primos

Sobre los 109 primos ≤ 600, distribución del último dígito de la grafía
dozenal:

| último dígito dozenal | primos |
|---|--:|
| `1` | 23 |
| `5` | 28 |
| `7` | 27 |
| `↋` | 29 |
| `2` | 1 (el primo 2, grafía `2`) |
| `3` | 1 (el primo 3, grafía `3`) |
| **violaciones con `p > 3`** | **ninguna** |

**P6 verificada.** Todo primo `> 3` tiene último dígito dozenal en
`{1, 5, 7, ↋}`, y el reparto entre las cuatro clases es sensiblemente uniforme
(23 / 28 / 27 / 29), como pide el teorema de Dirichlet. Ejemplos con su
comando:

```
$ target/release/dz-cli.exe eval "11" --input-base 10 --mode fixed
E
$ target/release/dz-cli.exe eval "13" --input-base 10 --mode fixed
11
$ target/release/dz-cli.exe eval "499" --input-base 10 --mode fixed
357
$ target/release/dz-cli.exe eval "491" --input-base 10 --mode fixed
34E
```

Esto es exactamente lo que P6 anticipó y **no es un descubrimiento**: es la
coprimalidad con 12 escrita en dígitos. `(Z/12)* = {1,5,7,11}` y un primo
`> 3` no puede compartir factor con `12 = 2²·3`. Ver el veredicto honesto en
§5.3.

### 2.2 El afilado: parejas de últimos dígitos admisibles según `n`

Aquí sí hay algo, y es modesto. Para cada `n` par se enumeraron todas las
descomposiciones `n = p + q` con `p ≤ q`, ambos `> 3`, y se registró la pareja
**no ordenada** de sus últimos dígitos dozenales. Resultado sobre
`4 ≤ n ≤ 600`:

| último dígito de `n` | nº de parejas distintas observadas | las parejas |
|---|--:|---|
| `0` | 2 | `{1,↋}` `{5,7}` |
| `2` | 2 | `{1,1}` `{7,7}` |
| **`4`** | **1** | **`{5,↋}`** |
| `6` | 2 | `{1,5}` `{7,↋}` |
| **`8`** | **1** | **`{1,7}`** |
| `↊` | 2 | `{5,5}` `{↋,↋}` |

Es decir: **si el último dígito dozenal de `n` es `4`, toda descomposición de
Goldbach de `n` con ambos sumandos mayores que 3 tiene últimos dígitos `5` y
`↋`, sin excepción. Si es `8`, son `1` y `7`.** Dos de las seis clases pares
son **rígidas**: una sola pareja admisible.

El control en base 10, calculado igual sobre el mismo rango (primos `> 5`,
últimos dígitos decimales en `{1,3,7,9}`):

| último dígito de `n` | nº de parejas distintas | las parejas |
|---|--:|---|
| `0` | 2 | `{1,9}` `{3,7}` |
| `2` | 2 | `{1,1}` `{3,9}` |
| `4` | 2 | `{1,3}` `{7,7}` |
| `6` | 2 | `{3,3}` `{7,9}` |
| `8` | 2 | `{1,7}` `{9,9}` |

**Base 10 es perfectamente uniforme: siempre dos parejas, ninguna clase
rígida.** La razón es estructural y se puede enunciar sin dígitos: en `Z/10` el
grupo de unidades es `{±1, ±3} ≅ Z₄` y sus 10 sumas no ordenadas se reparten
`2 + 2 + 2 + 2 + 2` entre las 5 clases pares; en `Z/12` el grupo es
`{±1, ±5} ≅ Z₂ × Z₂` y las 10 sumas se reparten `2 + 2 + 1 + 2 + 1 + 2` entre
las 6 clases pares. **La aritmética modular dozenal es no uniforme y la decimal
es uniforme, y no uniformidad es información.**

Casilla: **(III)**. Es un enunciado sobre la estructura aditiva de `(Z/12)*`
dentro de `Z/12`, hecho legible por la base. Y lo que **no** es: no restringe
la *existencia* de ninguna descomposición, solo la *forma* de las que existan.
Un contraejemplo de Goldbach sería un `n` sin ninguna pareja, y esta tabla es
compatible con eso. Aportación a la conjetura: **cero**. Como criba visual para
un humano que busca parejas a mano en un `n ≡ 4` o `n ≡ 8`: útil, porque reduce
el espacio de búsqueda a una sola combinación de dígitos finales.

### 2.3 Lo mejor que encontré: las bandas del cometa de Goldbach

`g(n)` = número de descomposiciones `n = p + q`, `p ≤ q`, ambos primos. Es
conocido (serie singular de Hardy–Littlewood) que `g(n)` crece con el factor
`∏_{p | n, p > 2} (p−1)/(p−2)`, que vale exactamente **2** cuando `3 | n`. El
«cometa de Goldbach» se parte por eso en bandas. Media de `g(n)` sobre
`100 ≤ n ≤ 600`, agrupada por el **último dígito dozenal** de `n`:

| último dígito dozenal de `n` | media de `g(n)` | valores |
|---|--:|--:|
| **`0`** | **19,33** | 42 |
| `2` | 9,68 | 41 |
| `4` | 10,79 | 42 |
| **`6`** | **19,05** | 42 |
| `8` | 9,29 | 42 |
| `↊` | 11,19 | 42 |

Banda alta `{0, 6}`: media 19,19. Banda baja `{2, 4, 8, ↊}`: media 10,24.
**Razón 1,87, es decir el factor 2 predicho, y la pertenencia a la banda es
una propiedad del último dígito.**

El mismo cálculo agrupado por el último dígito **decimal**:

| último dígito decimal de `n` | media de `g(n)` | valores |
|---|--:|--:|
| `0` | 16,00 | 51 |
| `2` | 12,18 | 50 |
| `4` | 13,10 | 50 |
| `6` | 12,78 | 50 |
| `8` | 12,06 | 50 |

**No hay bandas.** Las cinco medias están entre 12,06 y 16,00 y ninguna
partición de los dígitos decimales las separa, porque `3 | n` **no es una
propiedad del último dígito decimal de `n`**: `6, 12, 18, 24, 30` son todos
múltiplos de 3 y terminan en `6, 2, 8, 4, 0`, los cinco dígitos pares. El
`16,00` de la clase `0` es una mezcla (un tercio de esos `n` es múltiplo de 3),
no una banda.

**Éste es el mejor resultado dozenal de la línea D.** Base 12 convierte «¿en
qué banda del cometa está `n`?» en «¿acaba `n` en `0` o en `6`?», una
inspección de un dígito. Base 10 necesita una división (o la suma de todas las
cifras). Y a diferencia de F2 de la Parte 1, esto **no** es un sobreajuste: las
bandas del cometa son un fenómeno real de la distribución de los primos, no un
artefacto de la elección de constantes.

Casilla: **(III)**. Sigue siendo representación. Pero es representación que
hace visible un hecho aritmético no trivial, y eso es lo máximo que un cambio
de base puede aspirar a hacer.

### 2.4 Veredicto de la Parte 2

- **P6: confirmada.** El último dígito dozenal de los primos es `{1,5,7,↋}` y
  es una reformulación de la coprimalidad con 12.
- **Afilado 1 (§2.2):** dos de las seis clases pares admiten una sola pareja de
  últimos dígitos; base 10 no admite rigidez en ninguna. Informativo sobre
  `(Z/12)*`, **no** sobre Goldbach.
- **Afilado 2 (§2.3):** las bandas del cometa son legibles en el último dígito
  dozenal y no en el decimal. Es el hallazgo con más contenido de la sección.
- **Lo que no se consiguió:** ninguna representación L₁ aportó nada (§5.2), y
  ningún enunciado de esta sección acerca ni un paso a la conjetura.

---

## 3. Parte 3 — ataque a P5

> **P5.** «El efecto dozenal medible se reducirá a `N_periodicos` y al coste de
> lectura de literales. No aparecerá ninguna simplificación *estructural*
> atribuible a la base.»

El plan dice que es la predicción que más querría ver refutada. La ataqué por
tres vías, en orden de promesa creciente, y **una de las tres la refuta en la
letra**.

### 3.1 Vía 1 — ¿es gratis escalar por una potencia de la base? **No. Y cuesta más.**

Éste era el lead más prometedor y es un **resultado negativo limpio**. En
notación posicional, multiplicar por la base es mover el punto: cero
aritmética. `CLAUDE.md` documenta que `dz-sci` se apoya en eso internamente
(«escalar por `12^k` es mover `scale`, cero puertas»). La pregunta medible es
si ese ahorro es accesible desde una expresión.

```
$ target/release/dz-cli.exe eval "345*10"  --mode fixed --gates
3450
gates: cycle=20064 min=32368 max=24096 lit=16608 thr=4116 total=97252 depth=441 recon=0

$ target/release/dz-cli.exe eval "345*E"   --mode fixed --gates
3107
gates: cycle=19404 min=32190 max=23923 lit=16440 thr=4080 total=96037 depth=441 recon=0

$ target/release/dz-cli.exe eval "345*9"   --mode fixed --gates
2639
gates: cycle=21384 min=36492 max=27050 lit=18504 thr=4536 total=107966 depth=441 recon=0

$ target/release/dz-cli.exe eval "345/10"  --mode fixed --gates
34;5
gates: cycle=147576 min=186628 max=142300 lit=102144 thr=28212 total=606860 depth=1371 recon=0

$ target/release/dz-cli.exe eval "345/E"   --mode fixed --gates
38;111111111111111111111111 (periodica en base 12, valor truncado)
gates: cycle=136488 min=158116 max=122282 lit=89808 thr=26772 total=533466 depth=1371 recon=0

$ target/release/dz-cli.exe eval "345/9"   --mode fixed --gates
45;X8
gates: cycle=131208 min=152036 max=117402 lit=86016 thr=25428 total=512090 depth=1371 recon=0
```

| operación sobre `345` | puertas | profundidad |
|---|--:|--:|
| `× 10` (por la base) | 97.252 | 441 |
| `× ↋` (por un dígito cualquiera) | **96.037** | 441 |
| `× 9` | 107.966 | 441 |
| `÷ 10` (por la base) | **606.860** | 1.371 |
| `÷ ↋` | 533.466 | 1.371 |
| `÷ 9` | 512.090 | 1.371 |

**Multiplicar por la base cuesta más que multiplicar por `↋`, y dividir por la
base cuesta más que dividir por `↋` o por `9`.** El motivo es mecánico y vale
la pena decirlo porque invierte la intuición: el bucle del multiplicador
escolar recorre los **dígitos del segundo operando**, y `12` escrito `10` tiene
**dos** dígitos mientras `↋` tiene uno. La ventaja posicional de la base se
convierte en una **penalización posicional**, porque el algoritmo no salta el
dígito cero.

Conclusión de esta vía, y hay que decirla sin rodeos: **la ventaja estructural
teórica de base 12 más citada no es realizable con este instrumento, y lo que
el instrumento mide es lo contrario.** El ahorro existe dentro de `dz-sci`
porque allí el desplazamiento se hace a mano sobre el campo `scale`; no existe
como propiedad de la aritmética expuesta. Esto **refuerza** P5.

### 3.2 Vía 2 — la profundidad de la lectura posicional. Estructural, pero es la casilla (III) que P5 ya concede

Citado de `02-referencia-medida.md` §4 (no recalculado):

| cifras | decimal / dozenal | puertas base 10 | puertas base 12 | prof. base 10 | prof. base 12 |
|--:|---|--:|--:|--:|--:|
| 2 | `12` / `10` | 12.276 | 10.492 | 254 | **254** |
| 3 | `144` / `100` | 20.405 | 15.053 | 478 | **254** |
| 4 | `1728` / `1000` | 30.924 | 19.614 | 816 | **254** |
| 5 | `20736` / `10000` | 43.227 | 24.175 | 1.187 | **254** |

Reproduje las dos primeras filas y añadí más valores, todos consistentes:

```
$ target/release/dz-cli.exe eval "360" --input-base 10 --mode fixed --gates
260
gates: cycle=6732 min=10098 max=7582 lit=5304 thr=1392 total=31108 depth=481 recon=0

$ target/release/dz-cli.exe eval "260" --mode fixed --gates
260
gates: cycle=5412 min=8302 max=6231 lit=4356 thr=1152 total=25453 depth=254 recon=0
```

Diferencial de lectura medido (mismo valor en las dos bases): **1.784** puertas
a dos cifras, **5.352** a tres, **5.655** para `360`/`260`, **11.310** a cuatro.

La observación de profundidad **sí es estructural**: la lectura posicional en
base 12 no tiene cadena de dependencias (la posición en el `Vec` *es* el peso),
mientras Horner es secuencial por construcción. Es una diferencia en la
**forma del circuito**, no en un conteo.

Pero no refuta P5, por una razón que hay que conceder: P5 enumera
explícitamente «el coste de lectura de literales» como uno de los dos efectos
esperados. La profundidad constante **es** el coste de lectura, medido con el
otro instrumento. Es el efecto que P5 predijo, mejor caracterizado.

Y con una asimetría que conviene registrar: esta ventaja es de la
**representación nativa**, no de la base 12 frente a la 10 en abstracto. Un
motor que trabajara nativamente en base 10 tendría profundidad constante al
leer decimal y Horner al leer dozenal. Lo que se mide es «leer la base en la
que está construido el circuito es gratis», no «12 es mejor que 10».

**Nota adicional sobre el instrumento, que afecta a estas cifras.** Un literal
dozenal suelto **no** es gratis en este motor, y su coste es proporcional al
*valor* del dígito, no a su posición:

```
$ for d in 0 1 2 6 9 E; do target/release/dz-cli.exe eval "$d" --mode fixed --gates; done
0  -> total=0        depth=0
1  -> total=5883     depth=254
2  -> total=7460     depth=254
6  -> total=13576    depth=254
9  -> total=18163    depth=254
E  -> total=19692    depth=254
```

Es `small_usize_to_dz`, que convierte por **conteo repetido** con puertas
(`CLAUDE.md`, capa 1). Consecuencia práctica: `10` (dos cifras, 10.492) cuesta
**menos** que `9` (una cifra, 18.163), porque lo que se paga es la suma de los
valores de los dígitos. Cualquier diferencial de lectura publicado mezcla, por
tanto, dos efectos —la cadena de Horner y la suma de dígitos— y solo el primero
es el que interesa. Los pares de la tabla están elegidos de modo que Horner
domine, y se comprueba en que el diferencial crece con el número de cifras y no
con el valor.

### 3.3 Vía 3 — los recíprocos cortos convierten divisiones en multiplicaciones. **Esto sí refuta P5 en la letra**

La división restauradora es el circuito más caro del proyecto. En base 12,
`1/2`, `1/3`, `1/4` y `1/6` son **exactos y de un solo dígito** (`0;6`, `0;4`,
`0;3`, `0;2`, medidos en §1.3 F5). Así que dividir por 2, 3, 4 o 6 se puede
sustituir por multiplicar por un dígito. Medido:

```
$ target/release/dz-cli.exe eval "345/2"    --mode fixed --gates
182;6
gates: cycle=115764 min=130526 max=101084 lit=74400 thr=22152 total=443926 depth=1371 recon=0

$ target/release/dz-cli.exe eval "345*0;6"  --mode fixed --gates
182;6
gates: cycle=26136 min=38424 max=28946 lit=20364 thr=5436 total=119306 depth=899 recon=0

$ target/release/dz-cli.exe eval "345/3"    --mode fixed --gates
115;8
gates: cycle=117348 min=131606 max=102007 lit=75180 thr=22464 total=448605 depth=1371 recon=0

$ target/release/dz-cli.exe eval "345*0;4"  --mode fixed --gates
115;8
gates: cycle=27984 min=40936 max=30864 lit=21744 thr=5832 total=127360 depth=1153 recon=0

$ target/release/dz-cli.exe eval "345/4"    --mode fixed --gates
X1;3
gates: cycle=119196 min=134058 max=103865 lit=76500 thr=22836 total=456455 depth=1371 recon=0

$ target/release/dz-cli.exe eval "345*0;3"  --mode fixed --gates
X1;3
gates: cycle=29832 min=44276 max=33320 lit=23400 thr=6228 total=137056 depth=899 recon=0

$ target/release/dz-cli.exe eval "345/6"    --mode fixed --gates
68;X
gates: cycle=129228 min=148154 max=114550 lit=84096 thr=24972 total=501000 depth=1371 recon=0

$ target/release/dz-cli.exe eval "345*0;2"  --mode fixed --gates
68;X
gates: cycle=35772 min=53246 max=40070 lit=28140 thr=7512 total=164740 depth=1153 recon=0
```

Y el **control obligatorio**, el caso donde base 12 pierde:

```
$ target/release/dz-cli.exe eval "345/5"       --mode fixed --gates
81
gates: cycle=119856 min=133852 max=103799 lit=76560 thr=22920 total=456987 depth=1371 recon=0

$ target/release/dz-cli.exe eval "345*0;2497"  --mode fixed --gates
80;EE3E
gates: cycle=207504 min=311256 max=233269 lit=162660 thr=41268 total=955957 depth=2703 recon=0
```

| `345 ÷ n` | división | `345 × 1/n` con recíproco corto | resultado | razón |
|---|--:|--:|---|--:|
| `÷ 2` | 443.926 | **119.306** (`× 0;6`) | idéntico, `182;6` | **3,72×** |
| `÷ 3` | 448.605 | **127.360** (`× 0;4`) | idéntico, `115;8` | **3,52×** |
| `÷ 4` | 456.455 | **137.056** (`× 0;3`) | idéntico, `X1;3` | **3,33×** |
| `÷ 6` | 501.000 | **164.740** (`× 0;2`) | idéntico, `68;X` | **3,04×** |
| **`÷ 5`** | 456.987 (`81`, exacto) | 955.957 (`× 0;2497`) | **`80;EE3E`, MAL** | **0,48×** |

Profundidad: de 1.371 baja a 899 o 1.153. Los resultados de las cuatro primeras
filas son **bit a bit idénticos** a la división — no es una aproximación.

**Por qué esto refuta P5 en la letra.** No es un cambio de escritura: es un
cambio de **qué circuito corre**. La división restauradora desaparece y la
sustituye un multiplicador. El ahorro medido es de 3–3,7× en puertas y ~1,5× en
profundidad, y **es exacto**. Base 10 no puede hacer esto para el 3: `1/3` no
termina en decimal, así que no hay recíproco corto exacto, y sustituirlo daría
un resultado equivocado —exactamente lo que le pasa a base 12 con el 5, que es
la fila de control.

Recuento de recíprocos exactos cortos, en los dos sentidos:

| longitud | base 12 | base 10 |
|---|---|---|
| 1 dígito | `1/2, 1/3, 1/4, 1/6` (**4**) | `1/2, 1/5` (**2**) |
| 2 dígitos | `1/8, 1/9, 1/16, 1/18, 1/24, 1/36` | `1/4, 1/20, 1/25, 1/50` |
| no existe | `1/5, 1/10, 1/20, 1/25` | `1/3, 1/6, 1/9, 1/12` |

Base 12 tiene el doble de recíprocos de un dígito y, sobre todo, tiene el del
**3**, que es el segundo primo. Base 10 tiene el del 5 y base 12 no.

**Y por qué no es evidencia a favor de 12.** Es el mismo mecanismo único —la
factorización de la base— visto por tercera vez. Base 10 obtiene el regalo
gemelo para el 5, con la misma magnitud. Lo que el experimento establece es
condicional: **para problemas dominados por mitades y tercios, base 12 elimina
divisiones que base 10 no puede eliminar, con un ahorro medido de ~3,5×.** Y
«problemas dominados por mitades y tercios» es precisamente el antecedente de
H₁. Así que H₁, en su factor dozenal, queda **confirmada con un efecto medido
que va más allá de `N_periodicos`** — y confirmada de la manera más aburrida
posible, porque la condición que lo activa es la misma que `N_periodicos`
detecta.

### 3.4 ¿Cambia base 12 lo demostrable o lo expresable?

La pregunta directa del encargo, respondida en dos partes.

**Demostrable: no.** El cambio de base es una biyección computable entre
sistemas de numerales. Toda demostración en una base se transporta a la otra
mecánicamente. Los enunciados cuyo *contenido* depende de la base (normalidad
de un número, ley de Benford, primos repunit, criterios de divisibilidad) no
son el «mismo teorema más fácil»: son **teoremas distintos**. Cambiar la base
no simplifica una demostración; cambia el enunciado. Casilla (III) por
definición: es representación.

**Expresable: en un sentido preciso y limitado, sí.** El conjunto de racionales
con expansión posicional **finita** cambia: denominadores 3-lisos en base 12,
`{2,5}`-lisos en base 10. Ninguno contiene al otro y los dos son densos en `Q`.
Así que ningún número pasa de inexpresable a expresable —todos son expresables
como fracción en cualquier base—; lo que cambia es **cuáles admiten notación
posicional finita**. Eso es un hecho sobre la notación, no sobre los números.

**Lo único que cambia de clase de complejidad.** Un cambio real y medible:
los criterios de divisibilidad. En base 12, `2, 3, 4, 6, 12` dividen a la base,
así que el test es una inspección del **último dígito**, coste `O(1)`. En base
10 solo `2, 5, 10` lo son; `3` y `9` exigen la **suma de todas las cifras**,
`O(nº de cifras)`. Demostración por inspección de la tabla, con las grafías del
binario:

```
$ for n in 3 6 9 12 15 18 21 24 27 30 33 99 111 123 1002; do \
    target/release/dz-cli.exe eval "$n" --input-base 10 --mode fixed; done
3  6  9  10  13  16  19  20  23  26  29  83  93  X3  6E6
$ for n in 7 13 25 100 1000 1001; do \
    target/release/dz-cli.exe eval "$n" --input-base 10 --mode fixed; done
7  11  21  84  6E4  6E5
```

Los quince múltiplos de 3 acaban en `0, 3, 6, 9` sin excepción; los seis no
múltiplos, no. El test de divisibilidad por 3 pasa de `O(cifras)` a `O(1)`.
Es una **separación de complejidad genuina**, estructural, atribuible a la
base — y es la cuarta manifestación del mismo mecanismo único, y otra vez
genérica: base 10 tiene ese `O(1)` para el 5.

### 3.5 Veredicto sobre P5

**P5 cae en la letra y se mantiene en el espíritu; hay que reescribirla.**

Cae en la letra porque su cláusula fuerte —«no aparecerá ninguna simplificación
*estructural* atribuible a la base»— es falsa: §3.3 exhibe una sustitución de
circuito exacta con 3,5× de ahorro medido, y §3.4 una separación de complejidad
en los tests de divisibilidad. Ninguna de las dos es `N_periodicos` ni coste de
lectura.

Se mantiene en el espíritu porque las cuatro manifestaciones —`N_periodicos`,
coste y profundidad de lectura, recíprocos cortos, tests de divisibilidad— son
**consecuencias del mismo y único hecho**, `12 = 2²·3`, y ninguna es evidencia
de que 12 sea mejor que 10: cada una tiene su gemela decimal para el 5. Y su
lead más prometedor, el escalado gratis por la base, resultó **falso en la
medición** (§3.1).

Reescritura propuesta para el informe final:

> **P5′.** El efecto dozenal medible se reduce a **una sola causa**, la
> factorización de la base, con cuatro manifestaciones: `N_periodicos`, coste y
> profundidad de lectura de literales, disponibilidad de recíprocos exactos
> cortos (que sí es estructural: elimina circuitos de división), y tests de
> divisibilidad en `O(1)`. Ninguna favorece al 12 sobre el 10 en abstracto;
> todas favorecen a la base cuyos factores coinciden con los denominadores del
> problema.

---

## 4. ¿Hizo base 12 algo más que `N_periodicos`?

**Sí, dos cosas, y las dos son genéricas.** Resumen de las cuatro
manifestaciones, con su medida y su casilla:

| efecto | ¿estructural? | medida | ¿previsto por P5? | gemelo decimal |
|---|:-:|---|:-:|---|
| `N_periodicos` (1/3, 1/6, 1/12, 1/24 terminan) | no | ΔK = −1 a −3 en F1, F4, F6, F7 | **sí** | 1/5, 1/10, 1/20 terminan en decimal |
| Coste y profundidad de lectura de literales | **sí** (profundidad constante 254 frente a cadena de Horner) | 1.784 / 5.352 / 11.310 puertas; 254 frente a 478 / 816 / 1.187 | **sí** | un motor nativo decimal lo tendría al revés |
| Recíprocos exactos de un dígito para 2, 3, 4, 6 | **sí** (sustituye el circuito de división) | **3,04× a 3,72×** menos puertas, exacto | **no** | base 10 los tiene para 2 y 5 |
| Divisibilidad por 2, 3, 4, 6, 12 en `O(1)` | **sí** (cambia la clase de complejidad) | inspección de 1 dígito frente a suma de todas las cifras | **no** | base 10: `O(1)` para 2, 5, 10 |

Y la lista de lo que base 12 **no** hizo, que es más larga y más importante:

- **No eliminó ni un radical ni una constante irracional** en ninguna de las 8
  familias. No podía: la irracionalidad no depende de la base. `√3/2` es
  `~0;X485X9EEX9436` y `π` es `~3;184809493E92`, los dos igual de irracionales
  que en decimal.
- **No bajó `N_operaciones` en ninguna familia**, ni siquiera en F8: la
  corrección de §3.1 anula esa columna, porque multiplicar por la base no es
  gratis en este motor.
- **No tocó la casilla (II)**, como debe ser: ninguna de sus ganancias es
  geométrica. Las ocho familias dieron (III) y solo (III).
- **No hizo racional ni periódico a π.** Ninguna base puede.
- **No compensó su propia pérdida.** Los quintos son más caros en base 12 que
  los tercios en base 10, porque `1/5` en base 12 tiene período 4 y `1/3` en
  base 10 período 1.

---

## 5. Lo que no funcionó

### 5.1 Las cinco familias podadas

**F9 — Calendario y tiempo (12 meses, 24 horas, 60 minutos).** Podada por
convenio. El contrafáctico falla de inmediato: la razón entre el año trópico y
el mes sinódico es ≈ 12,368, **no** 12, así que ni siquiera el calendario está
forzado a 12 por la astronomía; los 12 meses son una aproximación
administrativa y el desajuste se paga con meses de longitud desigual. Las 24
horas y los 60 minutos son sexagesimales babilónicos. Escribir `20` por 24 y
`50` por 60 en base 12 no es evidencia de nada.

```
$ target/release/dz-cli.exe eval "24" --input-base 10 --mode fixed
20
$ target/release/dz-cli.exe eval "60" --input-base 10 --mode fixed
50
```

**F10 — Sistemas de medida (12 pulgadas, docena, gruesa 144).** Podada por
convenio, y es el caso más flagrante de razonamiento circular de toda la lista:
la pulgada se divide en 12 partes *porque* 12 tiene muchos divisores, es decir,
por la misma propiedad que se está tratando de verificar. Usarla como evidencia
es asumir la conclusión. Que la gruesa sea `100` en base 12 es tautológico.

**F11 — Los 12 pentominós libres.** Podada por ser un **cardinal, no una
partición**. Que haya exactamente 12 pentominós libres es un hecho
combinatorio genuino y no un convenio, pero el 12 nunca aparece en un
denominador: no hay ninguna fórmula de la teoría de pentominós donde `1/12`
intervenga. Base 12 escribe ese cardinal `10`, lo cual es bonito y no significa
nada. Mismo razonamiento para los 35 hexominós, que en base 12 se escriben
`2E` y nadie llamaría a eso una simplificación.

**F12 — Las 12 aristas del cubo, los 12 vértices del cuboctaedro.** Podada por
la misma razón. El 12 aquí sale de la fórmula de Euler `V − E + F = 2` con
enteros pequeños, y esa fórmula es idéntica en toda base. El número de besos 12
del empaquetamiento FCC (que sí sobrevive dentro de F4) tampoco entra en ningún
denominador: aparece como cardinal de una lista de vecinos.

**F13 — Las 32 clases cristalográficas de grupos puntuales.** Podada por
duplicar F1. El 32 no es dozenalmente notable (`28` en base 12) y todo el
contenido con 12 dentro es el teorema de la restricción cristalográfica, que ya
es F1.

**Regla destilada de las cinco poda**, para que las otras líneas la reutilicen:
*un número no es evidencia de una base si (a) se puede cambiar eligiendo otra
unidad, (b) es un cardinal que nunca aparece en un denominador, o (c) su
presencia se justifica citando la divisibilidad de 12, que es lo que se quiere
demostrar.*

### 5.2 Lo que no funcionó en la Parte 2

- **La representación L₁ de Goldbach no dio nada.** Intenté colocar los pares
  `(p, q)` con `p + q = n` en la retícula y buscar estructura bajo la métrica
  L₁: el conjunto `{(p,q) : p + q = n}` está **contenido en una recta
  antidiagonal**, donde la distancia L₁ entre dos puntos cualesquiera es
  exactamente el doble de la diferencia de abscisas. L₁ es, sobre esa recta,
  una reparametrización lineal de la recta real: no aporta ninguna estructura
  que la recta no tuviera. Es un resultado negativo trivial y por eso
  `planteamientos.md` §10 ya avisaba de no forzar la geometría aquí.
- **Las coordenadas «dozenales» `(n div 12, n mod 12)` no revelaron nada.**
  Dibujar los primos en una retícula de 12 columnas concentra todos los primos
  `> 3` en 4 de las 12 columnas — que es P6 otra vez, no un hallazgo nuevo. Las
  4 columnas no muestran ningún patrón interno (la distribución 23/28/27/29 de
  §2.1 es la esperada por Dirichlet).
- **La rigidez de §2.2 no restringe la existencia.** Un `n` que refutara
  Goldbach sería un `n` sin ninguna pareja, y la tabla de parejas admisibles no
  excluye ese caso. No se ganó ni un paso.

### 5.3 Veredicto honesto sobre P6: ¿descubrimiento o reformulación?

**Reformulación.** Sin matices en lo esencial y con un matiz en el margen.

Lo esencial: «todo primo `> 3` acaba en `1, 5, 7` o `↋` en base 12» es
**equivalente por definición** a «todo primo `> 3` es coprimo con 12», que es
inmediato porque `12 = 2²·3`. No hay ningún contenido que no estuviera en la
frase «un primo mayor que 3 no es divisible por 2 ni por 3». El único aporte de
la base es hacer la frase visible en un dígito. Eso es pedagogía y es
presentación, y P6 lo dijo por adelantado con esas palabras.

El matiz, que es lo único que añadí: la **no uniformidad** de las sumas de
unidades en `Z/12` frente a la uniformidad en `Z/10` (§2.2), y la legibilidad
de las bandas del cometa en el último dígito (§2.3). Ninguna de las dos es un
descubrimiento sobre los primos; las dos son hechos elementales de aritmética
modular que la base 12 expone en la posición de un dígito y la base 10 esconde
detrás de una división. La segunda tiene más contenido que la primera porque el
hecho que expone (la serie singular de Hardy–Littlewood) no es elemental.

Lo que **no** haría en un informe final: presentar §2.2 o §2.3 como
«estructura en Goldbach revelada por base 12». No hay estructura en `n = p + q`
que estas tablas revelen. Hay estructura en `Z/12` y en la serie singular, que
ya se conocía, escrita de forma que se lea de un vistazo.

### 5.4 Lo que no funcionó en el instrumento

Cinco limitaciones encontradas midiendo, todas reutilizables por las otras
líneas:

1. **`K(F)` no es invariante bajo reescritura algebraica.** La densidad del
   empaquetamiento hexagonal da `ΔK = −1` escrita `π√3/6` y `ΔK = 0` escrita
   `π/√12`. Una parte de cualquier `ΔK` publicado se puede fabricar eligiendo
   la forma. Mitigación: fijar la forma antes de mirar el resultado y declararla.
2. **`N_periodicos` es binario y ciego a la longitud del período.** Oculta que
   `1/11 = 0;1111…` tiene período 1 en base 12 y 2 en decimal, y que
   `1/13 = 0;0E0E…` tiene período 2 frente a 6. Base 12 gana ahí y el índice no
   lo ve.
3. **`K(F)` no cuenta cifras de los enteros.** No ve que `1728` (la constante
   del invariante `j`) sea `1000` en base 12. Faltaría un `N_cifras`. **No lo
   añadí**, porque añadir términos al índice después de ver los datos es
   exactamente el sesgo que el plan quiere evitar.
4. **El coste en puertas no es invariante bajo conmutatividad, y las constantes
   memoizadas lo falsean.** `pi*9` cuesta 19.556 puertas y `9*pi` cuesta
   46.220: el bucle recorre los dígitos del segundo operando. Y `pi` reporta
   **0 puertas** por memoización, así que toda fórmula L₂ que contenga `π` está
   **sub-facturada**. Medido honestamente con `4*atan(1)`, el perímetro L₂
   `2*(4*atan(1))*3` cuesta **352.771.272** puertas con profundidad 60.913,
   frente a **42.501** y profundidad 441 del L₁ `8*3` — un factor 8.300× en
   puertas. Con `pi` memoizado el mismo par daba 64.261 frente a 42.501, un
   factor 1,5×. **Cualquier comparación L₁/L₂ que use `pi` sin precio está mal
   por tres órdenes de magnitud.** La profundidad es el instrumento robusto
   aquí, porque `pi` sí reporta profundidad.
5. **`--scale` no controla la escala de la división.** `eval "1/5" --mode fixed
   --scale 6` devuelve 24 dígitos igual que `--scale 24`: manda
   `DEFAULT_DIV_SCALE = 24`. `--scale` sí controla las trascendentes. Hay que
   decir qué escala se pidió y qué escala se obtuvo.

### 5.5 Lo que no funcionó en la hipótesis

El hallazgo de §1.0 es el que más incomoda y no tiene mitigación: **los dos
factores de H₁ son antagónicos.** La bola L₁ tiene grupo de simetrías `D₄`
(orden 8) y regala particiones en 2, 4 y 8; base 12 regala particiones en 2, 3,
4, 6 y 12. Las familias donde el 12 está *forzado* (restricción cristalográfica,
Eisenstein, hexagonal, raíces duodécimas de la unidad, peso 12 modular) son
todas de simetría triádica o hexagonal, y en todas ellas L₁ no simplifica: o no
aplica, o destruye la propiedad que hacía interesante a la familia (la norma
`|a| + |b|` sobre `Z[ω]` no es multiplicativa, y con eso se va el carácter
euclídeo del anillo).

De las 8 familias supervivientes, **en 6 la columna L₁ está vacía o marcada «no
aplica»**. La línea D no puede por sí sola refutar H₁ —es una conjunción y le
toca solo un factor—, pero sí puede decir esto: **la intuición que motiva H₁, la
de que L₁ y base 12 apuntan al mismo sitio, es falsa.** Apuntan a factorizaciones
distintas del mismo número de simetrías, y donde una gana la otra no está.

---

## 6. Casillas, para la síntesis

| hallazgo | casilla | por qué |
|---|:-:|---|
| F1–F8: `ΔK` por `N_periodicos` | (III) | cambia la escritura de constantes racionales |
| F8: primer dígito dozenal de `log₂ r` = semitonos de 12-TET | (III) | la matemática (convergente de la fracción continua) es idéntica |
| §2.1 último dígito dozenal de los primos en `{1,5,7,↋}` | (III) | coprimalidad con 12 escrita en dígitos |
| §2.2 rigidez de las clases `4` y `8` | (III) | estructura aditiva de `(Z/12)*` |
| §2.3 bandas del cometa legibles en un dígito | (III) | la serie singular es base-independiente |
| §3.2 profundidad constante 254 al leer dozenal | (III) | propiedad de la base nativa del circuito |
| §3.3 recíprocos cortos eliminan divisiones (3,5×) | (III) | estructural pero de representación: no cambia el número |
| §3.4 divisibilidad por 2,3,4,6,12 en `O(1)` | (III) | idem |
| §1.0 antagonismo L₁ / base 12 | **(I) + (III)** | la simetría `D₄` de la bola L₁ es geometría; su choque con `2²·3` es aritmético |
| §5.4-4 coste real de `π` frente a `pi` memoizado | (II) | es un hallazgo sobre el coste de L₂, no sobre la base |

Ocho familias, y todas las ganancias dozenales en (III). Eso es precisamente lo
que el plan pedía que esta línea estableciera.

