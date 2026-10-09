# Hallazgo — La hipótesis de Riemann vista desde la base 12: qué se traslada, qué se mide y qué no

Quinta extensión posterior al cierre. La abre una pregunta directa del usuario
(2026-10-09): *¿se puede investigar el problema de Riemann usando la base 12
como enfoque?* Antes de esta fecha ningún documento de la serie lo había
tocado: las únicas apariciones de "Riemann" o "ζ" en `Investigations/` eran
`ζ(−1) = −1/12` como ejemplo de constante terminante (`01-D`, familia F7), el
lema de Riemann–Lebesgue (`01-C`) y el teorema de la aplicación de Riemann
(`10`, `99`).

Fecha de apertura: 2026-10-09. Protocolo de `00-plan-investigacion.md`: toda
cifra dozenal sale de `target/release/dz-cli.exe` con el comando al lado,
backend MVL; los literales sin más son **dozenales** (`--input-base 10` para
decimal); `--gates` imprime puertas y profundidad. Los guiones que repiten cada
medida están en `13-riemann/` (§11). Donde se usó un **oráculo externo** (Node
con `BigInt`, para contrastar) se dice expresamente; ninguna cifra dozenal del
documento sale de él.

**Recordatorio no negociable.** La hipótesis de Riemann (HR) no se resuelve
aquí, ni se acerca a resolverse. Es un enunciado sobre los ceros de una función
analítica; ninguna base numérica, calculadora ni arquitectura de puertas lo
demuestra ni lo refuta. Lo que este documento hace es separar, con el criterio
de la serie, **qué parte de la HR y de su entorno tiene algo que ver con escribir
los números en base 12** (casilla III), **qué parte no se traslada** (casilla
IV), y medir con el instrumento las comprobaciones finitas que sí son
expresables.

Sobre las casillas: (I) y (II) hablan de métrica (L₂ frente a L₁) y aquí no
aplican, porque la HR no tiene métrica. Se usan solo (III) "efecto de la
representación en base 12" y (IV) "la pregunta no se traslada", con el mismo
sentido que en `11` y `12`.

---

## 0. Resumen del veredicto (para quien no lea el resto)

| pregunta | veredicto | casilla |
|---|---|---|
| ¿Depende la verdad de la HR de la base? | **No.** La HR habla de la función ζ, no de numerales; cambiar de base es renombrar enteros sin tocar `+` ni `×`. Toda formulación equivalente es invariante. | (IV) |
| ¿Qué le aporta la base 12 a los valores de ζ? | **Un teorema limpio, en el lado trivial.** `ζ(1−2k)` **nunca** termina en base 10, y en base 12 termina exactamente cuando ningún primo `p ≥ 5` cumple `(p−1) ∣ 2k`; en ese caso el denominador es **exactamente 12**: una sola cifra dozenal fraccionaria. Ocurre infinitas veces (2k = 2, 14, 26, 34, 38, 62, …). Medido con Bernoulli calculados por puertas hasta 2k = 52. | (III), con raíz aritmética (von Staudt–Clausen + Adams) |
| ¿Y a los ceros? | **Nada.** Se localizaron los cinco primeros ceros en la recta crítica con la función Z de Hardy (Riemann–Siegel truncado en `C₀`), todo por puertas: error de 2,3·10⁻³ a 7,5·10⁻³ frente a la referencia, ≈ 3·10¹⁰ puertas por cero. Es una localización, no una verificación. `Re(s) = 1/2` es `0;6`, tan corto como `0.5`. | (IV) |
| Primos por clase mod 12 (sesgo de Chebyshev, HR generalizada) | La clase de un primo es su **última cifra dozenal**: leerla cuesta 0 puertas frente a 148.140 de `divrem n 10`. El sesgo (la clase 1 va detrás) es aritmética de los caracteres mod 12, no de la base. | (III) la lectura; (IV) el sesgo |
| Función de Möbius y Mertens | En base 12 la **última cifra** decide `μ(n) = 0` en el 64 % de los casos con cuadrado (por `4 ∣ n`); en base 10 la última cifra no decide ninguno. `M(n)² < n` se cumple, exacto, en las 1.727 filas `2 ≤ n ≤ 1000₁₂`; `M(1000₁₂) = −7`, contrastado fila a fila con un oráculo (0 discrepancias). | (III), compartida con toda base múltiplo de 4 |
| Criterio de Robin | Comprobado (aproximado, con margen ≫ error) en los 12 números colosalmente abundantes de `55440` a `2021649740510400`; falla en 2520 y 5040, como debe. Sus ceros finales en base 12 los supera la base 6. | (IV) la desigualdad; (III) los ceros finales |
| ¿Puede el instrumento probar la HR? | No. La HR es un enunciado Π₁: un contraejemplo sería un cálculo finito, una prueba no. `forall` es acotado (`12` §10). El instrumento puede, en principio, **refutar**; nunca demostrar. | (IV) |

La conclusión de fondo repite la de `11` y `12`: **la HR no tiene base**. El
único resultado de este documento genuinamente de base 12 vive en el lado
"trivial" de ζ (los enteros negativos), y su causa es que `12 = 2²·3` son
justamente los dos primos que el teorema de von Staudt–Clausen mete siempre en
el denominador de los números de Bernoulli.

---

## 1. Predicciones registradas antes de medir

- **PR1.** Ninguna medida dirá nada sobre la verdad de la HR; las que se
  puedan hacer son comprobaciones finitas ya conocidas, y la base 12 no cambia
  ninguna de ellas.
- **PR2.** `ζ(1−2k)` no termina nunca en base 10 (el 3 está siempre en el
  denominador); en base 12 termina para un conjunto escaso de `k`, al menos
  `k = 1` y `k = 7`. *Formulada después de ver la corrida corta (2k ≤ 14), no
  antes: se marca como predicción contaminada. La parte "infinitas veces" y la
  parte "denominador exactamente 12" sí se formularon antes de la corrida larga
  y del oráculo.*
- **PR3.** En la carrera de primos mod 12 hasta `1000₁₂ = 1728`, la clase 1 va
  detrás de las clases 5, 7 y ↋ casi todo el tiempo.
- **PR4.** `|M(n)| < √n` para `2 ≤ n ≤ 1728` (la desigualdad de Mertens es falsa
  en general, pero no a esta escala).
- **PR5.** Robin se cumple en los colosalmente abundantes mayores que 5040 y
  falla en 5040; los márgenes son del orden de centésimas, muy por encima del
  error de truncado del instrumento.
- **PR6.** Los ceros finales de los números colosalmente abundantes no
  favorecen a la base 12 sobre la base 6.
- **PR7.** Con Riemann–Siegel truncado en la primera corrección, los cinco
  primeros ceros salen con error menor que `0;01` (≈ 0,007).

---

## 2. ¿Depende la HR de la base? (IV)

**Enunciado.** `ζ(s) = Σ n^{−s}` (Re s > 1), prolongada a todo ℂ menos `s = 1`.
Sus ceros "triviales" son `−2, −4, −6, …`; la HR afirma que todos los demás
tienen `Re(s) = 1/2`.

**Por qué la base no entra.** Un sistema de numeración posicional es una
biyección computable entre cadenas y enteros que no altera `+` ni `×`. Todo
enunciado de la aritmética (y de los reales construidos sobre ella) tiene el
mismo valor de verdad sea cual sea el nombre que se dé a cada número. La HR, en
cualquiera de sus formas equivalentes (§5, §6), es un enunciado así. Por tanto
**la pregunta "¿la base 12 ayuda a probar la HR?" no se traslada**: no hay
ninguna forma de la HR en la que la base sea un dato.

**Qué sí se traslada: el coste y la forma escrita.** Lo único que la base puede
cambiar es (a) cuánto cuesta calcular algo con *esta* máquina, que es de base
12 por construcción (`12` §3), y (b) si un valor concreto se escribe con un
número finito de cifras. Ambas cosas son casilla (III) por definición.

**La HR es Π₁, y eso fija qué puede hacer el instrumento.** Se sabe (Davis,
Matiyasevich y Robinson, 1976 — **no verificado aquí**) que la HR equivale a un
enunciado de la forma `∀n φ(n)` con `φ` decidible. De ahí dos consecuencias que
conectan con `12`:

1. Si la HR fuese **falsa**, existiría un `n` concreto que lo atestigua, y
   comprobarlo sería un cálculo finito: el instrumento podría, en principio,
   refutarla. En §6 se hace exactamente ese tipo de comprobación (Robin) en los
   únicos puntos donde un contraejemplo podría aparecer primero.
2. Si es **verdadera**, ningún barrido finito la demuestra. `dz-cli forall` es
   acotado a propósito (no hay operador `μ`), así que la frontera de lo que el
   instrumento puede decir es la misma que la de `12` §2.3: local sí, global no.

---

## 3. ζ en los enteros negativos: el único resultado de base 12 (III)

### 3.1 La medida

`ζ(1−2k) = −B₂ₖ/(2k)`, con `B₂ₖ` los números de Bernoulli. El guion
`13-riemann/bernoulli.sh` calcula los `Bₘ` **dentro del instrumento** con la
recurrencia de Akiyama–Tanigawa (`A[m] = 1/(m+1)`, `A[j−1] = j·(A[j−1] − A[j])`;
cada paso es una llamada a `dz-cli eval` en modo racional exacto, los contadores
`j`, `m+1` los nombra el propio instrumento desde base 10) y después evalúa
`−B₂ₖ/(2k)`. La columna "periódica" es el `periodic_note` de la respuesta JSON.

```
$ bash Investigations/13-riemann/bernoulli.sh 52      # salida: 13-riemann/bernoulli-52.out
```

| 2k (dec) | `ζ(1−2k)` en dozenal (instrumento) | mismo valor en decimal (oráculo) | termina en base 12 |
|--:|---|---|:-:|
| 2 | `−1/10` = `−0;1` | −1/12 | **sí** |
| 4 | `1/↊0` | 1/120 | no |
| 6 | `−1/190` | −1/252 | no |
| 8 | `1/180` | 1/240 | no |
| 10 | `−1/↋0` | −1/132 | no |
| 12 | `497/16↋60` | 691/32760 | no |
| 14 | `−1/10` = `−0;1` | −1/12 | **sí** |
| 16 | `2115/4880` | 3617/8160 | no |
| 18 | `−21477/8390` | −43867/14364 | no |
| 20 | `8506↋/39↊0` | 174611/6600 | no |
| 22 | `−38↋57/1↋0` | −77683/276 | no |
| 24 | `671↊8937/31↋00` | 236364091/65520 | no |
| 26 | `−2788↋7/10` = `−2788↋;7` | −657931/12 | **sí** |
| 28 | `7↊829↊52↋/2020` | 3392780147/3480 | no |
| 30 | `−239↋6568↋481/41890` | −1723168255201/85932 | no |
| 32 | `↊46150027715/9540` | 7709321041217/16320 | no |
| 34 | `−25478188↊3↋/10` | −151628697551/12 | **sí** |
| 36 | `↋↊40089↋136858↊451/1↋17↋160` | 26315271553053477373/69090840 | no |
| 38 | `−15366↋79↋78351/10` | −154210205991661/12 | **sí** |
| 40 | `99818443↋3↊6↋127317/221240` | 261082718496449122051/541200 | no |
| 42 | `−4911↊7↊2830690↊54197/37↊90` | −1520097643918070802691/75852 | no |
| 44 | `7↋05↊17551↋58↋587951/1720` | 2530297234481911294093/2760 | no |
| 46 | `−6920833↋98395545812↊7/3↋0` | −25932657025822267968607/564 | no |
| 48 | `5↊689↋9217844908↋303↋57↋0↋/8↋5200` | 5609403368997817686249127547/2227680 | no |
| 50 | `−2↋↊52↊5854964↊2769422025/↋0` | −19802288209643185928499101/132 | no |
| 52 | `547288↊↊17054368↋767↋827655/3820` | 61628132164268458257532691681/6360 | no |

Contraste: cada fracción decimal del oráculo, convertida **por el
instrumento** (`dz-cli eval "<fracción>" --input-base 10`), coincide carácter a
carácter con la que el instrumento obtuvo por su cuenta: **26 de 26**.

El oráculo externo (`13-riemann/oracle.mjs`, Bernoulli exactos con `BigInt`)
reproduce las mismas fracciones y extiende la lista hasta 2k = 400
(`13-riemann/criterion.mjs`): **36 de 200** índices pares dan `ζ(1−2k)`
terminante en base 12 —

```
2,14,26,34,38,62,74,86,94,98,118,122,134,142,146,158,182,194,
202,206,214,218,254,266,274,278,298,302,314,326,334,338,362,386,394,398
```

— y en **todos** el denominador reducido es exactamente `12`; **ninguno** termina
en base 10; el criterio de §3.2 coincide en los 200 casos (`mismatches=0`).

### 3.2 El teorema, y por qué es 12 y no "la base"

**Proposición.** Sea `k ≥ 1`.

1. El denominador reducido de `ζ(1−2k)` es siempre múltiplo de 6. En
   particular `ζ(1−2k)` **nunca** tiene desarrollo decimal finito.
2. `ζ(1−2k)` tiene desarrollo **dozenal** finito si y solo si ningún primo
   `p ≥ 5` cumple `(p−1) ∣ 2k`. En ese caso `k` es impar, `3 ∤ k`, y
   `ζ(1−2k) = −N/12` con `N` entero primo con 6: exactamente **una** cifra
   dozenal tras el punto, y esa cifra es 1, 5, 7 u ↋.
3. Hay infinitos `k` así.

*Demostración.* Von Staudt–Clausen: el denominador de `B₂ₖ` es `∏ p` sobre los
primos con `(p−1) ∣ 2k`. Como `1 ∣ 2k` y `2 ∣ 2k`, **2 y 3 están siempre**. El
numerador de `B₂ₖ` es primo con su denominador, así que dividir entre `2k` no
puede cancelar el 3 ni el 2: queda (1).

(2, ⇒) Si algún `p ≥ 5` cumple `(p−1) ∣ 2k`, ese `p` está en el denominador de
`B₂ₖ` y, por el mismo argumento, en el de `ζ(1−2k)`: no termina en base 12.
(2, ⇐) Si ninguno lo cumple, el denominador de `B₂ₖ` es exactamente 6. Además
`k` es impar (si no, `4 ∣ 2k` y `p = 5` entraría) y `3 ∤ k` (si no, `6 ∣ 2k` y
`p = 7` entraría). Por el teorema de Adams (si `pᵃ ∣ 2k` y `(p−1) ∤ 2k`, entonces
`pᵃ` divide al numerador de `B₂ₖ`), todo el factor `k`, que solo tiene primos
`≥ 5` y para ninguno de ellos `(p−1) ∣ 2k`, se cancela contra el numerador. Queda
`B₂ₖ/(2k) = N'/(6·2)` con `N'` primo con 6, es decir denominador `12`.

(3) Si `q ≥ 5` es primo y `2q + 1` es compuesto, los divisores de `2q` son
`1, 2, q, 2q`, y `q+1` es par mayor que 2 y `2q+1` es compuesto: ningún `p ≥ 5`
cumple la condición. Para `q ≡ 1 (mod 3)`, `2q + 1 ≡ 0 (mod 3)` es compuesto, y
por Dirichlet hay infinitos primos `q ≡ 1 (mod 3)`. ∎

(Los dos teoremas citados son clásicos —von Staudt y Clausen, 1840; J. C. Adams,
1878— y **no se volvieron a demostrar aquí**; el oráculo los contrasta en los
200 casos de §3.1. La proposición en sí es un corolario directo; no se buscó en
la literatura y es razonable suponer que es conocida.)

### 3.3 Lectura honesta

- **Lo aritmético no depende de la base.** "El denominador es 12" es un hecho
  sobre números. Lo que la base 12 añade es que ese hecho se *ve*: `−1/12` es
  `−0;1`, y `ζ(−25) = −657931/12` es `−2788↋;7`. Es exactamente la casilla
  (III).
- **No es "la" base que gana: es la menor.** Cualquier base múltiplo de 12
  (24, 36, …) da también una cifra. La base 6 da `1/12 = 0.03₆`, dos cifras,
  y termina en los mismos casos. La base 12 es la **menor** base en la que estos
  valores tienen una sola cifra fraccionaria: es una propiedad del número 12,
  no de la elección de base.
- **Por qué es justo este 12.** Von Staudt pone siempre 2·3; la condición de
  que no entre ningún otro primo obliga a `2k ≡ 2 (mod 4)`, que añade el segundo
  2. Es el mismo `12 = 2²·3` que en `01-D` F7 aparecía como "peso 12" y
  `ζ(−1) = −1/12`; aquí se ve que `ζ(−1)` no es una coincidencia aislada sino el
  primer miembro de una familia infinita.
- **Y no dice nada de la HR.** Los valores en enteros negativos están
  completamente determinados y viven en el lado de los ceros triviales. La
  ecuación funcional los manda a `ζ(2k) = (−1)^{k+1} B₂ₖ (2π)^{2k} / (2·(2k)!)`,
  donde reaparece `π^{2k}`: en el lado racional de ζ no hay π, y en el lado
  positivo sí. (Eco del tema de la serie: el π entra en ζ por el factor
  `π^{−s/2} Γ(s/2)` de la ecuación funcional, no por la base.)

**Coste.** Los Bernoulli por puertas son caros y crecen deprisa (columna
`cum_gates` de `bernoulli-52.out`, acumulado desde `B₀`): 304.432 puertas hasta
`B₂`, 78.364.975 hasta `B₁₄`, 1.096.796.195 hasta `B₂₆` y **27.673.458.057**
hasta `B₅₂` (1.378 llamadas a `eval`). El crecimiento es el de los racionales
intermedios de Akiyama–Tanigawa (numeradores de decenas de cifras) pasando por
el `div_rem` de cada reducción de Euclides — el mismo cuello de botella que
domina `docs/05-funciones-cientificas.md`. La base no interviene: no hay ninguna
operación en la recurrencia que la base 12 abarate.

---

## 4. Primos por clase mod 12: la lectura es gratis, el sesgo no es de la base

### 4.1 Por qué mod 12 toca la HR

La HR **generalizada** (HRG) afirma lo mismo que la HR para las funciones `L` de
Dirichlet `L(s, χ)`. Para el módulo 12 hay cuatro caracteres, porque
`(ℤ/12)* = {1, 5, 7, ↋}` es `C₂ × C₂`: el trivial, el de mod 4, el de mod 3 y su
producto. Es la misma factorización `ℤ/12 ≅ ℤ/4 × ℤ/3` que en `11` resultó ser
el backend CRT del instrumento. Bajo la HRG (más una hipótesis de independencia
lineal de los ceros), Rubinstein y Sarnak (1994, no verificado aquí) explican el
**sesgo de Chebyshev**: los primos tienden a ir por delante en las clases que
*no* son cuadrados. Módulo 12 el único cuadrado de unidad es 1 (`1² = 5² = 7² =
↋² = 1`), así que la predicción es que la clase 1 va por detrás de las otras
tres.

### 4.2 La medida

`13-riemann/mertens.sh 1728` (salida: `mertens-1728.out`, una fila por `n`)
nombra cada `n` en dozenal, decide la primalidad con
`dz-cli exists "<n>-2" "<n> % (i+2) = 0"` y toma la clase de la **última cifra**
de la cadena que imprimió el instrumento.

| clase (última cifra dozenal) | 1 | 5 | 7 | ↋ |
|---|--:|--:|--:|--:|
| primos `> 3` hasta `1000₁₂ = 1728` | **61** | 69 | 69 | 68 |

En los 1.724 valores `5 ≤ n ≤ 1728`, la clase 1 **no va nunca por delante**
de ninguna de las otras tres. Empata con la 5 en 4 valores, con la 7 en 20 y con
la ↋ en 26, y en todos los demás va detrás. Se cumple PR3.

Contraste: el oráculo (`oracle-mertens.mjs`, `compare-mertens.mjs`, criba en
JavaScript) coincide con el instrumento en las 1.727 filas: `n`, primo o no,
`μ(n)` y `M(n)`. Cero discrepancias.

### 4.3 Qué es de la base y qué no

- **La lectura de la clase es (III).** En base 12 la clase mod 12 *es* la última
  cifra, y leerla cuesta **0 puertas**. Partiendo del mismo número en base 10
  hay que dividir:

  ```
  $ dz-cli divrem 1723 12 --input-base 10 --gates
  quotient: EE
  remainder: 7
  gates: ... total=161234 depth=814 recon=0
  $ dz-cli divrem EE7 10 --gates          # el mismo divrem sin lectura decimal
  gates: ... total=148140 depth=294 recon=0
  ```

  En una máquina decimal pasaría lo contrario con las clases mod 10. Es la
  misma conclusión que `12` §3: la base que sale gratis es la de la máquina.
- **El sesgo no es de la base (IV).** El recuento 61 / 69 / 69 / 68 es el mismo
  en cualquier base. Mod 12 es interesante porque es el módulo más pequeño con
  tres clases no cuadradas que compiten a la vez con la clase 1, no porque se
  escriba con una cifra.

---

## 5. Möbius y Mertens

### 5.1 El criterio

`μ(n)` vale 0 si `n` tiene un factor cuadrado y `(−1)^k` si es producto de `k`
primos distintos. La función de Mertens es `M(x) = Σ_{n≤x} μ(n)`. Littlewood
(1912): la HR equivale a `M(x) = O(x^{1/2+ε})` para todo `ε > 0`. La
**desigualdad de Mertens** `|M(x)| < √x` es más fuerte y es **falsa** (Odlyzko y
te Riele, 1985; no se conoce ningún contraejemplo explícito; citado, no
verificado aquí). Comprobarla a pequeña escala no dice nada sobre la HR. Se mide
porque es exacta (`n − M² > 0` es aritmética entera) y porque obliga al
instrumento a factorizar.

### 5.2 La medida

El mismo guion de §4 calcula `μ(n)` con `dz-cli valuation` sobre los primos ya
encontrados: para en cuanto un exponente es ≥ 2 o el cofactor es 1 o primo. Va
acumulando `M(n)` con `eval` y comprueba `sign(n − M·M) = 1` en cada fila.

```
$ tail -1 Investigations/13-riemann/mertens-1728.out
n=1000 kind=comp mu=0 M=-7 last=0 mertens_ok=1
```

- `M(1000₁₂) = M(1728) = −7`. El oráculo da lo mismo, y también coinciden los
  puntos de control decimales `M(10) = −1`, `M(100) = 1`, `M(1000) = 2`.
- `n − M(n)² > 0` en las **1.727** filas, `2 ≤ n ≤ 1728`. Se cumple PR4.
- El máximo de `|M|` es 16 (en `n = E45₁₂ = 1637`). El máximo de `M²/n` es
  `0,8`, en `n = 5`, donde `M(5) = −2`; desde ahí la razón no vuelve a acercarse
  a 1.

### 5.3 La última cifra decide `μ = 0`: (III), compartida

En base 12 `4 ∣ n` si y solo si la última cifra es `0`, `4` u `8`, y
`9 ∣ n` se decide con las **dos** últimas, porque `12² = 16·9`. Contado sobre las
filas del instrumento: de los **677** `n ≤ 1728` con `μ(n) = 0`,

| decidido por | base 12 | base 10 |
|---|--:|--:|
| la última cifra | **432** (64 %), por `4 ∣ n` | **0**: ningún cuadrado `> 1` divide a 10 |
| las dos últimas cifras | **576** (85 %), por `4 ∣ n` o `9 ∣ n` | 484 (71 %), por `4 ∣ n` o `25 ∣ n` (oráculo) |

El instrumento tiene ese atajo para `4` (`dz-cli divisible EE8 4 --gates`:
`method: last-digit`, 66.094 puertas, casi todas de leer el operando; el circuito
propio son ~38). Para `9` cae a `div_rem` (`divisible EE8 9`: 113.603 puertas).
La regla de las dos cifras para 9 **no está construida**.

**Lectura.** Es (III) y no es exclusiva de la base 12. La propiedad útil es que
4 divida a la base, y eso lo cumplen 4, 8, 12, 16, 20, 24… La base 12 es la
menor que además decide `9 ∣ n` con dos cifras, porque `9 ∣ 12²`; la base 6 lo
hace igual (`6² = 4·9`), aunque con cifras más cortas. Además el atajo solo
descarta: un `n` libre de cuadrados sigue necesitando factorización completa, y
**ahí está todo el coste**. Un primo cerca de 1728 cuesta 819.675.170 puertas de
`exists` (§8.3), y el filtro de la última cifra ahorra 66 mil.

---

## 6. Criterio de Robin en los números colosalmente abundantes

### 6.1 El criterio y por qué estos puntos

Robin (1984): la HR equivale a `σ(n) < e^γ · n · ln ln n` para todo `n > 5040`,
con `σ` la suma de divisores y `γ` la constante de Euler–Mascheroni. Además, si
la desigualdad falla en algún `n > 5040`, falla también en un número
**colosalmente abundante** (CA). Por eso los CA son los únicos puntos donde
merece la pena mirar: un contraejemplo a la HR aparecería ahí primero. La lista
de CA se tomó de la literatura (OEIS A004490) y **no se verificó aquí**. No
importa para la comprobación, porque la desigualdad se evalúa igual sea o no CA
el número.

### 6.2 γ por puertas, en n = 12

`γ` no está en el catálogo del instrumento. Se calculó con Euler–Maclaurin en
`n = 12`:

```
γ ≈ H₁₂ − ln 12 − 1/(2·12) + 1/(12·12²) − 1/(120·12⁴) + 1/(252·12⁶) − 1/(240·12⁸)
```

Escrito en dozenal, los dos primeros coeficientes son `1/20` y `1/1000` (el
instrumento lo confirma: `dz-cli eval "1/(2*12)" --input-base 10` → `1/20`;
`"1/(12*12^2)"` → `1/1000`), y los denominadores siguientes son `↊0`, `190`,
`180` (convertidos también con el instrumento desde 120, 252 y 240). Es una
curiosidad de casilla (III): en `n = 12` los dos primeros términos de la
corrección son una cifra dozenal; los siguientes no, porque llevan 5 y 7.

```
$ dz-cli eval "1+1/2+1/3+1/4+1/5+1/6+1/7+1/8+1/9+1/X+1/E+1/10 - ln(10) - 1/20 + 1/(10*100) - 1/(X0*10000) + 1/(190*1000000) - 1/(180*100000000)" --mode fixed --scale 12 --gates
~0;6E15188X675EE8773193X009
gates: ... total=12320191 depth=70680 recon=0
$ dz-cli eval "0.57721566490153286060651209" --input-base 10 --mode fixed --scale 18      # referencia
0;6E15188X6760E381E7543345 (periodica en base 12, valor truncado)
```

Coinciden diez cifras dozenales (`0;6E15188X67`). La diferencia, ≈ 1,1·10⁻¹³,
es del tamaño del primer término omitido, `1/(132·12¹⁰)` ≈ 1,2·10⁻¹³.
`e^γ` sale `~1;9458398XEX01`, frente a la referencia `1;9458398XEX02975…`
(`dz-cli eval 1.78107241799019798523650410 --input-base 10 --mode fixed --scale 12`).

### 6.3 La comprobación

```
$ bash Investigations/13-riemann/robin.sh        # salida: 13-riemann/robin.out
```

Para cada `n` dado por su factorización, el instrumento calcula `n` en dozenal,
`σ(n) = ∏ (p^{a+1} − 1)/(p − 1)` de forma exacta, la razón de Robin
`σ(n)/(n ln ln n)` y el margen `e^γ − razón` (modo fijo, escala 12). Margen
positivo significa que Robin se cumple.

| n (decimal) | n (dozenal) | σ(n) (dozenal) | razón | margen `e^γ − razón` | Robin |
|--:|--:|--:|---|---|:-:|
| 2520 | `1560` | `5500` | `~1;97X4` | `~−0;0348` | falla (n ≤ 5040) |
| 5040 | `2E00` | `E240` | `~1;95X9` | `~−0;0151` | falla (n ≤ 5040) |
| 55440 | `28100` | `E2400` | `~1;9021` | `~0;0436` | ✓ |
| 720720 | `2X9100` | `1108800` | `~1;8968` | `~0;06XE` | ✓ |
| 1441440 | `596200` | `2266000` | `~1;8896` | `~0;0781` | ✓ |
| 4324320 | `1546600` | `6980000` | `~1;8823` | `~0;0834` | ✓ |
| 21621600 | `72X8600` | `2E1E4000` | `~1;8746` | `~0;0911` | ✓ |
| 367567200 | `X3120600` | `448E00000` | `~1;8838` | `~0;0820` | ✓ |
| 6983776800 | `142XX29600` | `73XX400000` | `~1;8987` | `~0;0690` | ✓ |
| 160626866400 | `27169742600` | `127988000000` | `~1;8X23` | `~0;0634` | ✓ |
| 321253732800 | `52317285000` | `2564E2280000` | `~1;8X17` | `~0;0640` | ✓ |
| 9316358251200 | `10656X5641000` | `61X03E6800000` | `~1;8979` | `~0;069X` | ✓ |
| 288807105787200 | `28484903467000` | `144X8X699400000` | `~1;8984` | `~0;0693` | ✓ |
| 2021649740510400 | `16X8X931E7X1000` | `98XX63368000000` | `~1;8X13` | `~0;0644` | ✓ |

(Razón y margen truncados a cuatro cifras dozenales; la salida completa está en
`robin.out`. La columna decimal de `n` es la factorización multiplicada por el
oráculo, solo para orientarse.)

**Lectura.** Los márgenes son de `0;04` a `0;09` (≈ 0,03–0,06), y el error del
instrumento en `e^γ` y en `ln ln n` está en la duodécima cifra dozenal: once
órdenes de magnitud por debajo. Así que la marca `~` no compromete el
veredicto en ninguna fila. Cada fila cuesta entre 342 M y 975 M de puertas, con
profundidad 70.680 en todas; la profundidad la marca el `exp` de `γ`, no `n`.
El margen **no** crece con `n`: oscila y baja despacio. Es lo esperado, porque
`σ(n)/(n ln ln n)` tiende a `e^γ` por debajo (Gronwall) y la HR dice que no la
cruza. Ninguna de estas filas dice nada nuevo, porque la desigualdad está
comprobada en la literatura hasta cotas astronómicas (no verificado aquí). Lo
que muestran es que el instrumento **puede** hacer la clase de comprobación que
refutaría la HR, y nada más.

**Por qué no con `forall`.** El cuerpo lleva `ln` y `exp`, que son aproximados,
y `forall` los rechaza a propósito: un cuantificador necesita una decisión
(`12` §10). Por eso Robin va en un guion y no en el cuantificador. Un `forall`
exacto necesitaría aritmética de intervalos que el instrumento no tiene (§9).

### 6.4 Los ceros finales: (III), y la base 12 no es la mejor

Los CA son ricos en 2 y 3, así que en base 12 terminan en ceros: `5040 = 2E00₁₂`,
y los cuatro últimos de la tabla acaban en `000`. ¿Favorece eso a la base 12?
Para `n = 2ᵃ3ᵇ5ᶜ·…` el número de ceros finales es `min(a, c)` en base 10,
`min(⌊a/2⌋, b)` en base 12 y `min(a, b)` en base 6:

| | ceros en base 10 | base 12 | base 6 |
|---|:-:|:-:|:-:|
| 2520 | 1 | 1 | 2 |
| 5040 a 1441440 (4 filas) | 1 | 2 | 2 |
| 4324320 | 1 | 2 | 3 |
| 21621600 a 160626866400 (4 filas) | 2 | 2 | 3 |
| 321253732800 a 2021649740510400 (4 filas) | 2 | 3 | 3 |

(Los recuentos en base 12 coinciden con las cadenas que imprime el
instrumento.) La base 12 nunca pierde contra la 10, pero **la base 6 la iguala o
la supera en todas las filas**. Se cumple PR6: es la misma regularidad de ceros
finales de `12` §5, compartida con cualquier base cuyos únicos primos sean 2 y 3.
Casilla (III), y la base 12 no es su mejor representante.

---

## 7. Los ceros: Z(t) de Hardy por puertas

### 7.1 Qué se puede calcular sin complejos

El instrumento no tiene números complejos, pero en la recta crítica no los
necesita. La función de Hardy `Z(t) = e^{iθ(t)} ζ(1/2 + it)` es **real** para
`t` real y tiene los mismos ceros que `ζ` en la recta. La fórmula de
Riemann–Siegel, truncada en la primera corrección `C₀`, solo usa `ln`, `cos` y
`sqrt`:

```
Z(t) ≈ 2 Σ_{n≤N} n^{−1/2} cos(θ − t ln n) + (−1)^{N−1} (t/2π)^{−1/4} · cos(2π(p² − p − 1/16)) / cos(2πp)
θ(t) ≈ (t/2) ln(t/2π) − t/2 − π/8 + 1/(48t) + 7/(5760t³)
N = ⌊√(t/2π)⌋,   p = √(t/2π) − N
```

`13-riemann/hardy-z.sh` escribe la expresión **en dozenal**, con las constantes
convertidas por el instrumento (16 → `14`, 48 → `40`, 5760 → `3400`). El punto
medio de cada paso de bisección también lo calcula el instrumento,
`(a+b)*0;6`. La `N` la fija el guion para cada intervalo: es 1 para `t < 2π·4 ≈
25,13` y 2 hasta `2π·9`.

```
$ bash Investigations/13-riemann/hardy-z.sh eval 12 1
~-0;1363E8432010E65X0X59647
gates: ... total=1204644341 depth=39812 recon=0
$ bash Investigations/13-riemann/hardy-z.sh eval "12;2" 1
~0;034716EE7375E5834083681X
$ bash Investigations/13-riemann/hardy-z.sh eval "26;5" 2
~0;020919E677626821X6091891
gates: ... total=1774237494 depth=40374 recon=0
```

`Z(12₁₂) < 0 < Z(12;2₁₂)`: hay un cambio de signo entre 14 y 14,17 (decimal).
Cada evaluación cuesta de 1.150 M a 1.775 M de puertas, según `N`, y unos 7 s.

### 7.2 Los cinco primeros ceros

`IT=22 bash Investigations/13-riemann/hardy-z.sh` (salida: `hardy-z.out`); 24
evaluaciones de `Z` por cero, ≈ 3·10¹⁰ puertas por cero. La referencia es el
valor publicado (tablas de Odlyzko, citado de memoria a 20 cifras decimales y
**no** contrastado con la fuente), convertido a dozenal por el instrumento:

| # | `N` | cero localizado (instrumento) | referencia (dozenal) | `|error|` (dozenal) | ≈ decimal |
|--:|:-:|---|---|---|--:|
| 1 | 1 | `12;1790X93…` | `12;17497E14E2…` | `0;00432X…` | 2,5·10⁻³ |
| 2 | 1 | `19;036158E…` | `19;0321020131…` | `0;004056…` | 2,3·10⁻³ |
| 3 | 1 | `21;02796X2…` | `21;0169186888…` | `0;011051…` | 7,5·10⁻³ |
| 4 | 2 | `26;516E7E6…` | `26;5122293922…` | `0;004952…` | 2,8·10⁻³ |
| 5 | 2 | `28;E235725…` | `28;E27952E343…` | `0;0043X0…` | 2,5·10⁻³ |

(La columna decimal es el error dozenal leído por el oráculo, solo para
orientarse.) Los cinco cambios de signo están donde deben estar. El error es
el de la **fórmula**: unas 2,5 milésimas, coherente con omitir el término `C₁`.
El cero 3 sale tres veces peor y **supera `0;01`, así que PR7 falla en ese
cero**. Es el único de los cinco con `p` cerca de 1 (`t = 25,01`, justo antes
del salto `N = 1 → 2` en `25,13`). No se analizó por qué el error crece ahí.

### 7.3 Lo que esto dice de la HR y de la base 12

- **De la HR, nada que no se supiera.** Que los primeros ceros están en la recta
  se sabe desde Riemann (sus notas, publicadas por Siegel en 1932) y Gram (1903); citado, no verificado aquí. Además esto no es una
  verificación: no hay cota de error ni recuento `N(T)` (§9).
- **De la base, nada: casilla (IV).** `Re(s) = 1/2` se escribe `0;6`, una cifra,
  igual que `0.5`. Las partes imaginarias de los ceros no tienen ninguna forma
  cerrada conocida ni se sabe si son racionales, y no hay ninguna base en la que
  "terminen". El coste de una evaluación lo dominan `ln`, `cos` y `sqrt`, cuyo
  cuello de botella es la división restauradora (`docs/05`). La base 12 no abarata
  nada de eso.
- **Sí aparece π, otra vez.** Todo el cálculo pasa por `t/2π` y `π/8`. Es el π
  de la ecuación funcional (§3.3), no el de ninguna métrica.

---

## 8. Lo que no funcionó

1. **La trampa del literal, otra vez.** La primera comprobación de `ζ(−1)`
   fue `dz-cli eval "-1/12"`, y el instrumento contestó `−1/12 (periodica en base
   12)`. Era correcto: `12` sin más es dozenal, catorce, y `1/14` sí es periódico.
   Es la trampa que ya registra `02-referencia-medida.md`, y se volvió a pisar.
   El valor bueno es `-1/10`, o `-1/12` con `--input-base 10`.
2. **La bisección fue mucho más fina que la fórmula.** 22 iteraciones estrechan
   el intervalo hasta ~10⁻⁷, pero Riemann–Siegel truncado en `C₀` se equivoca en
   ~2,5·10⁻³ (§7). Con intervalos iniciales de 0,08–0,25 bastaban 5–7 pasos; unas 15 de las 22 iteraciones por cero, con ~1.200 M de puertas
   cada una, no aportan nada. **La precisión de una bisección no es la precisión
   del cero**: la primera solo dice dónde cambia de signo *la fórmula*.
3. **La primalidad no admite la cota √n.** `exists` necesita una cota exacta, y
   `floor(sqrt(n))` es aproximado porque `sqrt` lo es salvo en cuadrados
   perfectos. El guion de §5 prueba por eso todos los divisores hasta `n−1`: para
   `n = EE7` (1723, primo) son 819.675.170 puertas en 1.721 candidatos, cuando con
   la cota √n bastarían 40: unas 43 veces más de lo necesario. Un `isqrt` exacto (Newton entero sobre `Dz`) lo
   arreglaría, pero no existe en el instrumento.
4. **Robin y Lagarias no caben en `forall`.** Los dos llevan `ln`/`exp`, y el
   cuantificador rechaza un cuerpo aproximado. Robin se hizo en un guion (§6).
   Lagarias (`σ(n) ≤ H_n + e^{H_n} ln H_n`) no se intentó, porque su margen en
   `n = 1` es exactamente cero y ninguna evaluación aproximada lo decide (es el
   caso `sign(sqrt(2)*sqrt(2)-2)` de `12` §2.2).
5. **Glifos en bash.** La salida JSON trae `↊`/`↋`, que ocupan varios bytes, y
   `${nd: -1}` cortaba el carácter por la mitad. Se pasó a la salida ASCII
   (`X`/`E`) para el nombre de `n`. Pero `M` se siguió leyendo del JSON, y el
   primer comparador con el oráculo no entendía `↊`/`↋`: dio **98 discrepancias
   falsas**, todas en filas con `M = −↊` o `−↋`. Al arreglar el comparador quedaron
   cero. El instrumento no se había equivocado; el contraste sí.
6. **PR2 quedó contaminada** (§1): se escribió después de ver `2k ≤ 14`. Lo que
   se predijo limpio fue la extensión ("infinitas veces", "denominador
   exactamente 12").

---

## 9. Lo que no se hizo

- **Ninguna demostración de que los ceros estén en la recta.** Un cambio de
  signo de `Z` demuestra un cero en la recta solo si el error de la fórmula es
  menor que `|Z|` en los extremos, y no se acotó el resto de Riemann–Siegel
  (cotas de Gabcke). Tampoco se contaron los ceros hasta una altura `T` (método
  de Turing, `N(T)` por el argumento de `ζ`), que es lo que haría falta para
  afirmar "todos los ceros hasta `T` están en la recta". Lo de §7 es una
  localización numérica, no una verificación.
- **Nada fuera de la recta crítica.** El instrumento no tiene complejos, y `Z(t)`
  solo existe en la recta. Buscar un cero fuera de ella, que es lo único que
  refutaría la HR por ese lado, no es expresable.
- **Ninguna de las formulaciones analíticas fuertes**: Nyman–Beurling,
  coeficientes de Li, constante de de Bruijn–Newman, positividad de Weil. Todas
  son enunciados sobre funciones reales o complejas sin base.
- **Ni Rubinstein–Sarnak ni ninguna densidad del sesgo de Chebyshev**: §4 cuenta
  primos y no estima densidades logarítmicas.
- **Ni literatura para la proposición de §3.2.** Es un corolario de dos
  teoremas clásicos y casi seguro que es conocida; no se buscó.
- **Ni máquina de base 6, 10 ni 24.** Las comparaciones entre bases de §3.3 y
  §6.4 son por fórmula, no medidas.
- **Solo backend MVL**, y Mertens solo hasta `1000₁₂ = 1728`.
- **Ni aritmética de intervalos** que permitiera un `forall` exacto para Robin.

---

## 10. Predicciones contra resultados

| predicción | resultado |
|---|---|
| PR1 — nada sobre la verdad de la HR; la base no cambia ninguna comprobación | **Se cumple.** Todas las medidas son comprobaciones finitas conocidas, y ninguna depende de la base salvo en coste o en cómo se escribe el resultado. |
| PR2 — `ζ(1−2k)` nunca termina en decimal; en dozenal, para un conjunto escaso de `k` (al menos 1 y 7) | **Se cumple, contaminada** (§1). Además es más fuerte de lo predicho: hay infinitos `k` y el denominador es siempre exactamente 12 (§3.2). |
| PR3 — la clase 1 va detrás hasta 1728 | **Se cumple**: no va por delante en ningún `n`, con 50 empates en total. |
| PR4 — `|M(n)| < √n` para `2 ≤ n ≤ 1728` | **Se cumple** en las 1.727 filas; el máximo de `M²/n` es 0,8, en `n = 5`. |
| PR5 — Robin se cumple en los CA `> 5040` y falla en 5040, con márgenes de centésimas | **Se cumple**: márgenes de `0;04` a `0;09` en 12 CA, y falla en 2520 y en 5040. |
| PR6 — los ceros finales de los CA no favorecen la base 12 frente a la 6 | **Se cumple**: la base 6 la iguala o la supera en las 14 filas. |
| PR7 — los cinco primeros ceros con error `< 0;01` | **Falla en uno.** Cuatro ceros tienen error ≈ `0;0040`–`0;0050`; el tercero, `0;0110`. |

---

## Correcciones

| fecha | dónde | qué |
|---|---|---|
| 2026-10-09 | §8.3 | Después de las corridas se añadió al CLI una raíz entera exacta, `isqrt`, por Newton entero sobre `Dz` (`dz-int::isqrt`, solo CLI), desarrollada en una rama de trabajo e integrada en `main` el mismo día (274 tests en verde). Con ella la cota `√n` sí se puede escribir: `dz-cli exists "isqrt(EE7)-1" "EE7 % (i+2) = 0" --gates` da `holds: false` con **14.248.934** puertas frente a 819.675.170, ≈ 57 veces menos. `isqrt(EE7)` solo cuesta 531.103. Las cifras de §4–§5 se tomaron **sin** `isqrt` y no se rehicieron. |

---

## 11. Reproducción

Desde la raíz del repositorio, tras `cargo build -p dz-cli --release`. Los bucles
y las sumas viven en los guiones; todas las cifras dozenales y todos los
recuentos de puertas son del binario. Los `.mjs` son el **oráculo externo**
(Node + `BigInt`) y solo sirven para contrastar.

| guion | qué hace | salida guardada | tiempo aprox. |
|---|---|---|--:|
| `13-riemann/bernoulli.sh 52` | `B₀…B₅₂` por Akiyama–Tanigawa en el instrumento, y después `ζ(1−2k)` | `bernoulli-52.out` | ~25 min |
| `node 13-riemann/oracle.mjs 52` | los mismos `ζ(1−2k)` en decimal exacto | (contraste de §3.1) | <1 s |
| `node 13-riemann/criterion.mjs 400` | criterio de §3.2 contra los Bernoulli exactos hasta 2k = 400 | (§3.1) | ~2 s |
| `13-riemann/mertens.sh 1728` | nombre dozenal, primalidad (`exists`), `μ` (`valuation`), `M(n)`, `sign(n − M²)` | `mertens-1728.out` | ~1–2 h |
| `node 13-riemann/oracle-mertens.mjs 1728` | `μ`, `M`, clases mod 12, en decimal | (contraste de §4–§5) | <1 s |
| `node 13-riemann/compare-mertens.mjs` | contraste fila a fila de `mertens-1728.out` y estadísticas de §4–§5 | (§4.2, §5.2) | <1 s |
| `13-riemann/robin.sh` | γ, `e^γ`, `σ(n)` y razón de Robin en los CA | `robin.out` | ~3,5 min |
| `IT=22 13-riemann/hardy-z.sh` | bisección de `Z(t)` en los cinco primeros ceros | `hardy-z.out` | ~1 h |
| `13-riemann/hardy-z.sh eval <t> <N>` | un valor de `Z(t)` con `--gates` | — | ~7 s |

Las rutas de los guiones son relativas a `Investigations/`; se ejecutan como
`bash Investigations/13-riemann/<guion>`.
