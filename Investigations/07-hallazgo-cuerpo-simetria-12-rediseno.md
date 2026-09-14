# Hallazgo — Rediseñar la bola unidad para que su grupo de simetría case con base 12

Extiende `99-sintesis.md` §1 (el antagonismo `D₄` de L₁ contra `2,3,4,6,12` de
base 12) con una propuesta generativa: en vez de preguntar si L₁ armoniza con
base 12 (no lo hace), se **redefine la norma** para que su bola unidad tenga
el grupo de simetría que base 12 sí regala — el hexágono regular, `D₆`, orden
12 — y se somete el resultado exactamente al mismo escrutinio que tumbó seis
intentos anteriores sobre la superficie 3D de L₁ (`01` §7.1, `03`, `04`, `05`).

Fecha de apertura: 2026-09-13. **Segunda pasada: 2026-09-13**, que cierra las
tres preguntas declaradas abiertas en la primera (§1.8, §2.6, §3.7), reverifica
las cifras publicadas contra el binario (§5.7) y enmienda el veredicto (§7.1).
Las secciones nuevas llevan «cerrada en la segunda pasada» en el título; las
correcciones a texto ya publicado van en bloques de cita marcados, nunca
borrando el original.

Protocolo: `00-plan-investigacion.md`. Toda cifra dozenal sale de
`target/release/dz-cli.exe` con el comando pegado al lado; `--mode rational`
donde la afirmación es de racionalidad, porque muestra fracción exacta si es
racional y un `~` con truncado/periódico si no lo es — el mismo criterio
visual inequívoco que usan `01`, `03`, `04`, `05`, `06`. Se clasifica en
casilla (I)/(II)/(III). Hecho no negociable: π es trascendente
(Lindemann–Weierstrass), y esa propiedad no depende de la base numérica; lo
que se investiga es si **otras** constantes propias del hexágono (que no
tienen por qué ser π) son racionales, y si esa racionalidad sobrevive al
escrutinio que ya destruyó las de L₁.

---

## 0. Resumen del veredicto (ancla)

**El hexágono gana en 2D donde L₁ no podía ganar (perímetro/diámetro), y de
forma genuinamente más robusta que L₁ (el "3" del hexágono resiste una
rotación arbitraria; el "4" de L₁ no resistía ni 45°). Pero el patrón de fondo
de toda la serie se repite intacto: el área del propio hexágono es
irracional, y en 3D el cuerpo natural motivado por `D₆` (el prisma hexagonal)
pierde ya en el volumen — antes incluso de llegar a la superficie, que es
donde L₁ fallaba.** Y el candidato que más "12" parece llevar en el nombre —
el dodecágono regular, `D₁₂`, orden 24 — resulta ser el peor de los cuatro en
perímetro/diámetro (irracional, con `√3` de por medio) y el mejor en área
(exactamente 3, y no por azar: es una consecuencia directa del teorema de
Niven). **No hay ningún cuerpo, entre los examinados, cuyas constantes de
perímetro/diámetro, área y superficie 3D sean simultáneamente racionales.**
Extiende P1 a una forma más fuerte: no es que L₁ no tenga un "π taxicab"
único; es que **ningún rediseño de la simetría de la bola unidad, ensayado
aquí de forma honesta, logra que las tres constantes ganen a la vez.**

| cuerpo | grupo | perímetro/diámetro (bola propia) | ¿robusto a rotación? | área (Lebesgue) | superficie 3D (extensión) |
|---|---|---|---|---|---|
| cuadrado/rombo (n=4) | `D₄`, orden 8 | **4**, racional | **no** (cae a 2 a 45°) | **2**, racional | — (es L₁, ya visto) |
| hexágono (n=6) | `D₆`, orden 12 | **3**, racional | **sí** | `3√3/2`, **irracional** | prisma hex.: `3√3+12` Hausdorff, `5π` Busemann, `42/π` HT — las tres irracionales; ya el volumen `3√3` lo es |
| octógono (n=8) | `D₈`, orden 16 | `8√2−8`, irracional | **no** (recorre `[2√2, 8√2−8]`) | `2√2`, irracional | — |
| decágono (n=10) | `D₁₀`, orden 20 | `(5√5−5)/2`, irracional (**√5**) | **sí** | irracional | — |
| dodecágono (n=12) | `D₁₂`, orden 24 | `24−12√3`, **irracional** | **no** (llega a 3 solo a 15°) | **3**, racional (Niven) | — |

> **Corrección (revisión del 2026-09-13).** La columna «área (Lebesgue)», el
> volumen `3√3` del prisma y la frase en negrita «no hay ningún cuerpo… cuyas
> constantes… sean simultáneamente racionales» **no son afirmaciones sobre la
> norma**. En un plano normado el área de Lebesgue de la bola unidad cambia con
> cualquier aplicación lineal, y una aplicación lineal da una norma isométrica.
> El hexágono de vértices `(±1, 0), (0, ±1), ±(1, −1)` es imagen afín del
> regular, tiene la misma `π_B = 3` y área **3**; su prisma `H × [−1, 1]` tiene
> volumen **6**. Las áreas de la tabla son las del polígono dibujado
> **euclidianamente** regular con circunradio 1, y la del dodecágono (3, por
> Niven) tampoco es invariante. Lo invariante —perímetro/diámetro, robustez,
> clasificación de §1.8— no cambia. Las superficies de Busemann y
> Holmes–Thompson (`5π`, `42/π`) sí son intrínsecas y siguen en pie. Detalle en
> `99-sintesis.md` §4.7 y §8.

Casillas: perímetro/diámetro del hexágono y del dodecágono son **(II)**
(la propiedad sobrevive al cambio de norma, la constante cambia); la
racionalidad del área del dodecágono es **(II)** también, pero por un
mecanismo distinto (teorema de Niven, ninguna relación con base 12); la
irracionalidad de la superficie del prisma hexagonal repite el patrón
**(I)** ya establecido en `01`/`03`/`04`/`05` para L₁, y §2.6 lo agrava:
cambiar de definición de área (Busemann, Holmes–Thompson) sustituye
irracionalidad algebraica por **trascendencia**, igual que en `03`/`04`.

**Resultado estructural de la segunda pasada (§1.8 + §3.7), que reorganiza
todo lo anterior.** Para el n-ágono regular con `n` par, la razón
perímetro/diámetro tiene una forma cerrada que se parte según `n mod 4`, y la
**misma** dicotomía gobierna la robustez rotacional:

```
n ≡ 2 (mod 4):  razón = n·sin(π/n)   creciente 3 → π   ROBUSTA siempre   racional SOLO en n=6 (=3)
n ≡ 0 (mod 4):  razón = n·tan(π/n)   decreciente 4 → π NUNCA robusta     racional SOLO en n=4 (=4)

racional ∧ robusto  ⟺  n = 6
```

Ambas exclusividades son el **teorema de Niven** (versión seno y versión
tangente), no la base 12. El hexágono no es especial por ser robusto (lo son
infinitos n-ágonos) ni por ser racional (también lo es el cuadrado): es el
único punto donde las dos propiedades se cruzan. Y es una demostración
independiente, puramente aritmética, de la mitad «polígono regular» del
teorema de Gołąb.

---

## 1. El hexágono en 2D

### 1.1 Los vértices y la coincidencia "cuerda = vértice"

Hexágono regular de circunradio 1, vértices `v_k = (cos 60k°, sin 60k°)`,
`k = 0..5`:

```
v0=(1,0)  v1=(1/2,√3/2)  v2=(-1/2,√3/2)  v3=(-1,0)  v4=(-1/2,-√3/2)  v5=(1/2,-√3/2)
```

**Afirmación**: `v1 − v0 = v2` exactamente. Verificación algebraica:
`v1−v0 = (1/2−1, √3/2−0) = (−1/2, √3/2) = v2`. ✓

**Por qué es específico de 60° y de ningún otro espaciado.** En complejos,
`v_k = ζ^k` con `ζ = e^{iπ/3}` (rotación de 60°, sexta raíz primitiva de la
unidad). La afirmación `v1−v0=v2` es `ζ−1=ζ²`, es decir `ζ²−ζ+1=0` — el
polinomio ciclotómico `Φ₆(x)`, del que `ζ` es raíz **por construcción**. Para
un n-ágono regular genérico con `θ=2π/n`, el mismo cálculo en forma polar da

```
ζ − 1 = e^{iθ} − 1 = 2 sin(θ/2) · e^{i(θ/2 + π/2)}
```

(factorizando `e^{iθ/2}(e^{iθ/2}−e^{-iθ/2}) = e^{iθ/2}·2i sin(θ/2)`, y
`i = e^{iπ/2}`). Para que esto sea **otro vértice** `v_m = e^{imθ}` hacen
falta dos condiciones simultáneas: magnitud `2 sin(θ/2) = 1` (el vértice tiene
radio 1) y fase `θ/2+π/2 ≡ mθ (mod 2π)` para algún entero `m`. La primera da
`θ = 60°` o `θ=300°` (mismo ángulo con signo); sustituyendo `θ=60°` en la
segunda: `30°+90°=120°=2·60°`, `m=2` — se cumple. Es un sistema de dos
ecuaciones que **coinciden en el mismo `θ`** solo para `n=6`: no es una
coincidencia doble, es una única ecuación algebraica (`cos θ = 1/2`) vista dos
veces. Álgebraicamente, `Z[ζ₆] = Z[ω]` con `ω = e^{i2π/3}` el entero de
Eisenstein primitivo (`ζ₆ = 1+ω`), así que la coincidencia es la sombra de que
el hexágono vive de forma natural en el retículo de Eisenstein — la misma
estructura que subyace a las teselaciones triangulares/hexagonales del plano.

**Consecuencia que se usa todo el documento: para el hexágono, el lado mide
exactamente lo mismo que el circunradio** (`|v1−v0| = |v2| = 1`). Esto es
un hecho euclidiano estándar (lado de hexágono regular = radio), pero aquí se
deriva porque es la causa raíz de todo lo que sigue — tanto de que el
perímetro salga limpio como de que sea insólitamente robusto a rotación
(§3).

### 1.2 La norma de gauge, sector por sector

Con el hexágono como bola unidad, la norma (función de Minkowski) es lineal
en cada uno de los 6 sectores angulares `[kθ,(k+1)θ]`, porque la frontera es
recta ahí: `‖x,y‖ = Ax+By` con `(A,B)` fijado por `‖v_k‖=‖v_{k+1}‖=1`. Para el
sector `[0°,60°]` (entre `v0` y `v1`):

```
A·1 + B·0 = 1        →  A = 1
A·(1/2) + B·(√3/2) = 1  →  B = 1/√3
```

```
‖(x,y)‖_hex = x + y/√3     para (x,y) en el sector [0°,60°]
```

(comprobación: `‖v1‖ = 1/2 + (√3/2)/√3 = 1/2+1/2 = 1` ✓). Por la simetría
`D₆` los otros cinco sectores son la misma fórmula rotada 60°k; se dan
explícitas en §3.2 porque hacen falta ahí.

### 1.3 Perímetro de autonorma: cómputo directo arista por arista

La longitud de autonorma de un segmento recto es simplemente la norma del
vector desplazamiento (la parametrización es lineal, así que integrar
`‖dx‖` a lo largo de la arista da `‖extremo−origen‖` sin más). Por §1.1,
`v1−v0 = v2`, y `‖v2‖_hex = 1` porque `v2` es un vértice de la propia bola
unidad. **Cada arista mide exactamente 1** en la norma que ella misma define.

```
perímetro = 6 · 1 = 6          diámetro = ‖v3−v0‖_hex = ‖(−2,0)‖_hex = 2·‖v0‖_hex = 2
razón = 6 / 2 = 3
```

```
$ target/release/dz-cli.exe eval "6" --mode rational
6
$ target/release/dz-cli.exe eval "2" --mode rational
2
$ target/release/dz-cli.exe eval "6/2" --mode rational
3
```

### 1.4 Contraverificación independiente — el teorema de Gołąb

`01-hallazgos-A-constantes.md` §4.3 ya citaba, sin explotarlo, el **teorema de
Gołąb**: en todo plano de Minkowski la semi-longitud de la bola unidad
(perímetro de autonorma dividido por 2, o de forma equivalente
perímetro/diámetro con diámetro=2) está en `[3,4]`, vale **4 si y solo si la
bola es un paralelogramo**, y **3 si y solo si es un hexágono afínmente
regular**. El hexágono aquí construido es afínmente regular (de hecho
regular a secas), así que Gołąb **predice** razón 3 sin necesidad de sumar
aristas — y coincide dígito a dígito con el cálculo directo de §1.3. Dos
derivaciones independientes (suma de aristas vía la coincidencia algebraica
de §1.1; caracterización general de Gołąb) confirman el mismo número.

De paso, esto explica el cuadrado: el hexágono y el rombo de L₁ (`n=4`,
vértices en los ejes) son los **dos únicos** casos, entre todos los cuerpos
convexos centralmente simétricos, que alcanzan los extremos de Gołąb. El
rombo de L₁ es exactamente el caso "paralelogramo" (razón 4, ya conocido de
`01`), y el hexágono es exactamente el caso "hexágono afínmente regular"
(razón 3, este documento). No son dos hallazgos independientes: son las dos
mitades de un único teorema clásico.

### 1.5 Área de Lebesgue: irracional, y por qué

El área euclidiana del hexágono de circunradio 1 es `A = (n/2) sin(2π/n)`
con `n=6`: `A = 3 sin(60°) = 3√3/2`.

```
$ target/release/dz-cli.exe eval "3*sqrt(3)/2" --mode rational
~2;721585EE840X
$ target/release/dz-cli.exe eval "3*sqrt(3)/2" --mode fixed --scale 8
~2;721586
```

Contraverificación independiente, evaluando la fórmula general con la propia
función `sin` de la calculadora en vez de sustituir `√3` a mano:

```
$ target/release/dz-cli.exe eval "3*sin(2*pi/6)" --mode fixed --scale 8 --input-base 10
~2;721586
```

> **Corregido en la segunda pasada (§5.7, discrepancia 1).** La primera
> redacción escribió aquí `3*sin(2*pi/12)`, que es el n-ágono equivocado
> (`n=6` va en el denominador, no 12) y devuelve `~1;6 = 3/2`, no `~2;721586`.
> La cifra publicada era correcta; el comando pegado al lado no la producía.

(el literal del denominador necesita `--input-base 10`: sin él, `dz-cli` lee
`6` como dozenal —que da lo mismo— pero `12` valdría 14 decimal, la trampa de
la calculadora de `01` §0.2, en la que este documento tropezó al preparar las
cifras y se corrige aquí en vez de disimularse; ver §5.6 y §6). Coincide
dígito a dígito con la sustitución algebraica.
**El `~` en ambos modos es la prueba visual de irracionalidad**: el área del
hexágono, a diferencia de su perímetro de autonorma, no escapa a `√3`.

### 1.6 n = 4, 8, 12: ¿el patrón depende de "12", o de la coincidencia de §1.1?

Se repite el cálculo de §1.2–§1.3 para los otros n-ágonos regulares
relevantes: `n=4` (ya es L₁, para anclar el método), `n=8` (ni divisor ni
múltiplo de 12, control negativo) y `n=12` (el candidato "más 12" posible por
nombre).

**Primer hecho que hay que decir de inmediato**: un n-ágono regular con `n`
**impar** no es centralmente simétrico (`‖−x‖ ≠ ‖x‖`), así que **no es la
bola unidad de ninguna norma**. Se comprueba para `n=3`: el vértice
`v0=(1,0)` tiene gauge 1 por construcción, pero `−v0=(−1,0)` cae en el sector
`[v1,v2]`, cuya fórmula de gauge (derivada igual que en §1.2) da
`‖(x,y)‖ = −2x` ahí, luego `‖−v0‖ = 2 ≠ 1 = ‖v0‖`. Esto descarta `n=3,5,7`
como candidatos de norma sin necesidad de calcular nada más sobre ellos —
no es que "no sean de sabor 12" (`5,7` tampoco lo son, pero `3` sí lo es y
cae exactamente por esta razón, no por su factor). El cuerpo de trabajo real
son los **n pares**.

**n = 4 (el rombo/cuadrado, ya es L₁).** `θ=90°`. Cuerda `v1−v0`, magnitud
`2 sin45°=√2`, ángulo `45°+90°=135°`, cae en el sector `[v1,v2]`. La misma
maquinaria de §1.2 en ese sector da `‖(x,y)‖=−x+y` y `‖v1−v0‖=2`. Perímetro
`4·2=8`, diámetro `‖v0−v2‖=2`, razón `8/2=4`.

```
$ target/release/dz-cli.exe eval "8/2" --mode rational
4
$ target/release/dz-cli.exe eval "2" --mode rational
2
```

Coincide exactamente con `01-hallazgos-A-constantes.md` §1.1 (perímetro
`8r`, razón 4) — el método generaliza el resultado ya conocido de L₁ en vez
de contradecirlo, que es la primera comprobación de que la maquinaria de este
documento es correcta antes de aplicarla a casos nuevos.

**n = 8 (octógono, control: ni divisor ni múltiplo de 12).** `θ=45°`. Cuerda
`v1−v0`: magnitud `2 sin22.5°`, ángulo `22.5°+90°=112.5°`, cae en el sector
`[v2,v3]` (`90°–135°`). Resolviendo el sistema de ese sector
(`v2=(0,1)`, `v3=(−√2/2,√2/2)`) da `‖(x,y)‖ = (1−√2)x + y` ahí, y evaluando en
la cuerda: `‖v1−v0‖ = 2√2−2`.

```
$ target/release/dz-cli.exe eval "2*sqrt(2)-2" --mode rational
~0;9E36321813E4
$ target/release/dz-cli.exe eval "2*sqrt(2)-2" --mode fixed --scale 8
~0;9E363218
```

Perímetro `8·(2√2−2) = 16√2−16`, diámetro `2`, razón `8√2−8`:

```
$ target/release/dz-cli.exe eval "16*sqrt(2)-16" --mode rational --input-base 10
~6;76421514X768
$ target/release/dz-cli.exe eval "8*sqrt(2)-8" --mode rational
~3;392108685394
$ target/release/dz-cli.exe eval "8*sqrt(2)-8" --mode fixed --scale 8
~3;39210868
```

**Irracional** (`√2`, algebraico) — consistente con Gołąb: el octógono no es
ni paralelogramo ni hexágono afínmente regular, así que su razón debe caer
estrictamente en `(3,4)` sin tocar ningún extremo racional. Área:
`4 sin45° = 2√2`, también irracional.

**n = 12 (dodecágono, el candidato "más 12" posible).** `θ=30°`. Cuerda
`v1−v0`: magnitud `2 sin15°`, ángulo `15°+90°=105°`, cae en el sector
`[v3,v4]` (`90°–120°`, entre los vértices a `90°` y `120°`). Resolviendo:
`v3=(0,1)`, `v4=(−1/2,√3/2)` da `‖(x,y)‖ = (√3−2)x + y` en ese sector, y
sustituyendo `v1−v0 = (√3/2−1, 1/2)`:

```
‖v1−v0‖ = (√3−2)(√3/2−1) + 1/2 = 4 − 2√3
```

```
$ target/release/dz-cli.exe eval "4-2*sqrt(3)" --mode rational --input-base 10
~0;652048004X6X
$ target/release/dz-cli.exe eval "4-2*sqrt(3)" --mode fixed --scale 8 --input-base 10
~0;652048
```

Perímetro `12·(4−2√3) = 48−24√3`, diámetro `2`, razón `24−12√3`:

```
$ target/release/dz-cli.exe eval "48-24*sqrt(3)" --mode rational --input-base 10
~6;52048004X6X
$ target/release/dz-cli.exe eval "24-12*sqrt(3)" --mode rational --input-base 10
~3;27024002535
$ target/release/dz-cli.exe eval "24-12*sqrt(3)" --mode fixed --scale 8 --input-base 10
~3;27024
```

**Irracional** (`√3`, algebraico) — de nuevo, exactamente lo que Gołąb obliga:
el dodecágono, con `D₁₂` de orden 24, no es afínmente ni un paralelogramo ni
un hexágono, así que su razón está condenada a `(3,4)` sin tocar el racional.
**El dodecágono, el n-ágono con el nombre más "12" de todos, tiene la peor
razón perímetro/diámetro de los cuatro examinados** (numéricamente
`≈3.215`, entre el `8√2−8≈3.31` del octógono y el `3` del hexágono, pero
igual de irracional que ambos).

### 1.7 Y sin embargo el dodecágono gana en área — el teorema de Niven

El área de un n-ágono regular de circunradio 1 es `(n/2) sin(2π/n)`. Para
`n=12`: `6 sin(30°) = 6·(1/2) = 3` — **racional, de hecho entero.**

```
$ target/release/dz-cli.exe eval "6*sin(2*pi/12)" --mode fixed --scale 8 --input-base 10
~3
$ target/release/dz-cli.exe eval "6*sin(2*pi/12)" --mode rational --input-base 10
~3
$ target/release/dz-cli.exe eval "3" --mode rational
3
```

(el `~` en el cálculo por `sin` es el truncamiento de la serie, no una duda:
coincide con el `3` exacto de la sustitución algebraica `6·(1/2)`, y ambas
formas se dan para que quede la comprobación cruzada explícita.)

Esto **no es una coincidencia aislada de `n=12`, y no tiene nada que ver con
base 12**: es una instancia del **teorema de Niven** (1956), que dice que los
únicos valores racionales de `sin θ` para `θ` un múltiplo racional de `π`
son `0, ±1/2, ±1`. Como `2π/n` es siempre un múltiplo racional de `π`, el
área `(n/2)sin(2π/n)` solo puede ser racional cuando `sin(2π/n) ∈ {0,±1/2,±1}`,
lo que fuerza `2π/n ∈ {90°, 150°,...}` — y, restringido a `n≥3` entero,
**exactamente** `n=4` (`sin90°=1`) y `n=12` (`sin30°=1/2`). Se comprueba
computando el área de los ocho n-ágonos de `n=3` a `n=12` con la propia
función `sin` de la calculadora (`--input-base 10` en todos por el literal de
`n`):

| n | expresión | valor (dz-cli, `--mode fixed --scale 8`) | clase |
|--:|---|---|---|
| 3 | `1.5*sin(2*pi/3)` | `~1;3708X3` | irracional |
| 4 | `2*sin(2*pi/4)` | `~2` | **racional** |
| 5 | `2.5*sin(2*pi/5)` | `~2;464692X3` | irracional |
| 6 | `3*sin(2*pi/6)` | `~2;721586` | irracional |
| 7 | `3.5*sin(2*pi/7)` | `~2;8X0625066` | irracional |
| 8 | `4*sin(2*pi/8)` | `~2;9E363218` | irracional |
| 9 | `4.5*sin(2*pi/9)` | `~2;X86396X16` | irracional |
| 10 | `5*sin(2*pi/10)` | `~2;E3256X98` | irracional |
| 11 | `5.5*sin(2*pi/11)` | `~2;E823007` | irracional |
| 12 | `6*sin(2*pi/12)` | `~3` | **racional** |

(comandos completos con `--input-base 10` en §7). **`n=4` y `n=12` son los
únicos dos, de un continuo de enteros `n≥3`, con área racional** — un hecho
completamente caracterizado por Niven, cierto en cualquier base numérica, sin
relación con la base 12 más que la coincidencia numérica de que `12` es el
valor de `n`. Es exactamente el tipo de trampa que el protocolo pide
señalar: el "12" del nombre del teorema no es la base 12 del proyecto, es
simplemente que `360°/12 = 30°` es uno de los pocos ángulos con seno
racional.

### 1.8 ¿Existe un n-ágono con razón perímetro/diámetro racional *intermedia*?
— No, y se demuestra (cerrada en la segunda pasada)

Esta subsección quedó declarada abierta en la primera redacción del documento.
Se cierra aquí con demostración, no con búsqueda numérica.

**1.8.1 Forma cerrada general para todo `n` par.** §1.6 derivó `n = 4, 8, 12`
caso por caso; se generaliza la maquinaria. Sea `P_n` el n-ágono regular de
circunradio 1, `n = 2m` par (los impares ya están descartados en §1.6 por no
ser centralmente simétricos), `θ = 2π/n = π/m`, vértices `v_k = e^{ikθ}`.

La **función radial** de `P_n` (distancia euclídea del origen a la frontera en
la dirección `α`) es, por trigonometría elemental del triángulo
centro–vértice–pie de apotema,

```
R(α) = cos(θ/2) / cos(δ(α)),     δ(α) = dist( α − θ/2 , θZ ) ∈ [0, θ/2]
```

donde `cos(θ/2)` es la apotema y `δ(α)` el desvío angular de `α` respecto de
la **normal de arista** más cercana (las normales de arista están en
`θ/2 + kθ`; los vértices, en `kθ`, están a `δ = θ/2` de ellas, y ahí
`R = 1` ✓). El gauge de un punto a radio euclídeo `r` y ángulo `α` es
`r / R(α)`.

Aplicado a los dos ingredientes de la razón:

- **Arista.** `v_1 − v_0` tiene magnitud euclídea `2 sin(θ/2)` y dirección
  `(θ+π)/2` (§1.1), luego `δ_e = dist(π/2, θZ)` y
  `‖v_1−v_0‖ = 2 sin(θ/2) · cos(δ_e)/cos(θ/2)`. Las `n` aristas son la misma
  por rotación de `θ`.
- **Diámetro.** Los vértices están a `δ = θ/2`, gauge 1; por convexidad
  (mismo argumento que §3.3) `diám = ‖2v_0‖ = 2`.

```
razón(n) = n · ‖v₁−v₀‖ / 2 = n · sin(θ/2) · cos(δ_e) / cos(θ/2)
```

Ahora el único término que depende de la paridad de `m`. Como `θ = π/m`,
se tiene `π/2 = (m/2)·θ`, luego

```
m par  (n ≡ 0 mod 4):  π/2 ≡ 0     (mod θ)  ⇒  δ_e = 0    ⇒  razón = n·tan(π/n)
m impar(n ≡ 2 mod 4):  π/2 ≡ θ/2   (mod θ)  ⇒  δ_e = θ/2  ⇒  razón = n·sin(π/n)
```

**Ésta es la forma cerrada general, y es una dicotomía limpia módulo 4.**

**1.8.2 Contraverificación: reproduce los tres casos que §1.6 hizo a mano.**

| n | rama | forma cerrada general | forma cerrada de §1.6 | dz-cli (general) | dz-cli (§1.6) |
|--:|---|---|---|---|---|
| 4 | tan | `4 tan(π/4) = 4` | `4` | `~4` | `4` |
| 6 | sin | `6 sin(π/6) = 3` | `3` | `~3` | `3` |
| 8 | tan | `8 tan(π/8)` | `8√2−8` | `~3;39210868` | `~3;39210868` |
| 12 | tan | `12 tan(π/12)` | `24−12√3` | `~3;27024` | `~3;27024` |

```
$ target/release/dz-cli.exe eval "4*tan(pi/4)"   --mode fixed --scale 8 --input-base 10
~4
$ target/release/dz-cli.exe eval "6*sin(pi/6)"   --mode fixed --scale 8 --input-base 10
~3
$ target/release/dz-cli.exe eval "8*tan(pi/8)"   --mode fixed --scale 8 --input-base 10
~3;39210868
$ target/release/dz-cli.exe eval "8*sqrt(2)-8"   --mode fixed --scale 8
~3;39210868
$ target/release/dz-cli.exe eval "12*tan(pi/12)" --mode fixed --scale 8 --input-base 10
~3;27024
$ target/release/dz-cli.exe eval "24-12*sqrt(3)" --mode fixed --scale 8 --input-base 10
~3;27024
```

Las dos derivaciones —caso por caso resolviendo el sistema de gauge de cada
sector (§1.6), y la forma cerrada general por función radial (aquí)— coinciden
dígito a dígito en los cuatro `n`. Ésa es la contraverificación exigida por el
protocolo antes de usar la forma cerrada para nada.

**1.8.3 Estructura del recorrido: dos sucesiones monótonas que encierran a π.**

```
$ target/release/dz-cli.exe eval "pi"            --mode fixed --scale 8 --input-base 10
~3;18480949
```

| n ≡ 2 (mod 4) | `n sin(π/n)` | | n ≡ 0 (mod 4) | `n tan(π/n)` |
|--:|---|---|--:|---|
| 6 | `~3` | | 4 | `~4` |
| 10 | `~3;10E991EX` | | 8 | `~3;39210868` |
| 14 | `~3;1472873X` | | 12 | `~3;27024` |
| 18 | `~3;1611X03` | | 16 | `~3;2236424` |
| 22 | `~3;16X2X84` | | 20 | `~3;20192408` |
| ↓ | `→ π = ~3;18480949` | | ↓ | `→ π = ~3;18480949` |

(todos con `eval "<n>*sin(pi/<n>)"` / `eval "<n>*tan(pi/<n>)" --mode fixed
--scale 8 --input-base 10`; bitácora en §8.)

La rama `sin` **crece** desde 3 y la rama `tan` **decrece** desde 4, ambas
hacia π, sin alcanzarlo nunca (para cada `n` el valor es algebraico y π es
trascendente). Esto recupera los dos extremos de Gołąb como los **dos primeros
términos** de las dos sucesiones —`n=4` da el máximo 4, `n=6` el mínimo 3— y
explica por qué `01-hallazgos-A-constantes.md` §4.3 encontraba `π_p ∈ [π,4]`
para `Lp` mientras Gołąb da `[3,4]` en general: la familia `Lp` solo cubre la
mitad superior del intervalo; la mitad inferior `[3,π)` es territorio
exclusivo de los n-ágonos con `n ≡ 2 (mod 4)`, al que ninguna bola `Lp`
llega.

**1.8.4 El resultado: no hay ningún racional intermedio. Demostración.**

Como `n` es entero, `razón(n)` es racional si y solo si lo es `sin(π/n)`
(rama `sin`) o `tan(π/n)` (rama `tan`). En ambos casos el argumento `π/n` es
un **múltiplo racional de π**, que es exactamente la hipótesis del **teorema
de Niven** (el mismo que gobierna §1.7, aquí en sus tres versiones):

> Si `θ` es un múltiplo racional de `π`, entonces
> `sin θ` racional ⇒ `sin θ ∈ {0, ±1/2, ±1}`;
> `cos θ` racional ⇒ `cos θ ∈ {0, ±1/2, ±1}`;
> `tan θ` racional ⇒ `tan θ ∈ {0, ±1}`.

- **Rama `sin` (`n ≡ 2 mod 4`, `n ≥ 6`).** `π/n ∈ (0, π/6]`, luego
  `sin(π/n) ∈ (0, 1/2]`. Por Niven el único valor racional admisible en ese
  rango es `1/2`, que fuerza `π/n = π/6`, es decir **`n = 6`**, con
  `razón = 6·(1/2) = 3`.
- **Rama `tan` (`n ≡ 0 mod 4`, `n ≥ 4`).** `π/n ∈ (0, π/4]`, luego
  `tan(π/n) ∈ (0, 1]`. Por Niven el único valor racional admisible es `1`,
  que fuerza `π/n = π/4`, es decir **`n = 4`**, con `razón = 4·1 = 4`.

```
Los ÚNICOS n-ágonos regulares con razón perímetro/diámetro racional son
n = 4 (razón 4) y n = 6 (razón 3). No existe ninguno con valor racional
estrictamente dentro de (3,4).
```

Dos consecuencias que conviene enunciar por separado porque no son la misma
frase:

1. **La respuesta a la pregunta abierta es un NO demostrado**, no un «no se
   encontró». El espacio de búsqueda (los enteros pares `n ≥ 4`) queda
   **agotado**, no muestreado.
2. **Esto es una demostración independiente de la mitad «n-ágono regular» del
   teorema de Gołąb.** Gołąb es un teorema de geometría convexa que
   caracteriza los extremos `3` y `4`; aquí se obtiene, restringido a
   polígonos regulares, por pura aritmética de Niven, sin geometría convexa.
   Que las dos rutas den los mismos dos polígonos es la tercera
   contraverificación de esta sección (§1.4 fue la primera, §1.8.2 la
   segunda). Y hay un detalle fino que solo se ve por esta ruta: **Gołąb no
   distingue los dos extremos por ningún mecanismo común, pero Niven sí — son
   las dos versiones distintas del mismo teorema (seno para `n ≡ 2`, tangente
   para `n ≡ 0`), aplicadas a las dos ramas de la dicotomía módulo 4.**

**1.8.5 Por qué la familia de n-ágonos se comporta al revés que la familia
`Lp` — el contenido real de esta subsección.**

`01-hallazgos-A-constantes.md` §4.4 encontró que la familia `Lp` **sí** admite
un continuo de razones racionales: `π_p` es continua y estrictamente monótona
en `p ∈ [1,2]` con `π_1 = 4` y `π_2 = π`, así que por el teorema del valor
intermedio **toma todos los valores de `(π,4)`**, y §4.4 localizó por bisección
los `p` que dan `3,9 / 3,75 / 3,5 / 3,25 / 3,2`. Aquí ocurre exactamente lo
contrario. Las dos causas, en orden de importancia:

- **El parámetro es discreto, así que no hay teorema del valor intermedio.**
  `Lp` se mueve por un parámetro real `p`; los n-ágonos, por un entero `n`. La
  monotonía de §1.8.3 es tan real como la de `π_p`, pero una sucesión monótona
  de un continuo a otro **no tiene por qué pasar por ningún racional**: solo
  visita los puntos que visita. Es la diferencia entre recorrer un intervalo y
  enumerar una sucesión, y es suficiente por sí sola para explicar la
  asimetría.
- **Pero hay algo más fuerte, y es lo que convierte un «podría no haber
  ninguno» en un «no hay ninguno»: la naturaleza aritmética de los valores.**
  `n sin(π/n)` y `n tan(π/n)` son números **algebraicos**, y viven en el
  cuerpo ciclotómico real `Q(ζ_n)⁺`, de grado `φ(n)/2` sobre `Q`. Ser racional
  es ser de **grado 1** en un cuerpo cuyo grado crece con `n`: una condición
  de codimensión máxima, y el teorema de Niven es precisamente el enunciado de
  que esa condición solo se cumple en la lista finita y minúscula
  `{0, ±1/2, ±1}` (resp. `{0, ±1}`). Los `π_p` de la familia `Lp`, en cambio,
  **no son algebraicos ni se sabe que lo sean** — son periodos de tipo Γ
  (`01` §4.4 lo declara explícitamente abierto), así que no hay ninguna
  estructura de cuerpo de grado acotado que prohíba que caigan en `Q`, y de
  hecho caen infinitas veces.

La lectura conjunta, que es el hallazgo de esta subsección: **la racionalidad
en la familia `Lp` es barata (la regala la topología del continuo) y por eso no
significa nada; la racionalidad en la familia de n-ágonos es rígida (la
gobierna la aritmética de los cuerpos ciclotómicos) y por eso sí distingue
exactamente dos cuerpos — el cuadrado y el hexágono.** Que esos dos sean
precisamente los dos extremos de Gołąb es la razón de fondo de que sean los
únicos candidatos serios a «bola unidad con constante limpia», y de que no
haya un tercero esperando a ser descubierto. **Casilla (II)**: la propiedad
«razón perímetro/diámetro racional» sobrevive al cambio de norma, la constante
cambia (4 → 3), y nada de esto tiene relación con la base numérica.

---

## 2. Empujar a 3D: el prisma hexagonal

### 2.1 Por qué el prisma y no la esfera icosaédrica

El enunciado propone tres rutas: (a) prisma o bipirámide hexagonal
(`D₆ₕ`, orden 24), extendiendo el `D₆` 2D; (b) el icosaedro, cuyo grupo tiene
orden 60; (c) un zonotopo o celda de Voronoi con secciones hexagonales.

**(b) es una pista falsa, y conviene descartarla explícitamente antes de
construir nada.** El grupo icosaédrico completo (`Iₕ`) tiene orden 120
(`60` sin las reflexiones), y `60 = 2²·3·5`. Comparte los factores `2` y `3`
con `12 = 2²·3`, pero **introduce el `5`**, que es precisamente el factor que
`99-sintesis.md` §1 identifica como el que base 12 **no** regala (`1/5` no
termina en base 12, y las ternas pitagóricas —llenas de cincos— son el
ejemplo que la síntesis da de un dominio donde base 12 pierde). El icosaedro
tiene 12 vértices, pero eso es **cuenta de vértices**, no estructura de
grupo: su simetría rotacional (`A₅`, orden 60) es la del pentágono, no la de
un cociente ni un múltiplo limpio de `2,3,4,6,12`. Es exactamente el tipo de
espejismo num��rico que el propio encargo pide vigilar («¿es un red herring?»)
y la respuesta, mirando la factorización en vez del recuento de vértices, es
que sí lo es. No se desarrolla más.

**(a)/(c) coinciden**: el prisma hexagonal es simultáneamente la extensión
natural del `D₆` 2D a `D₆ₕ` (orden 24) en 3D, **y** la celda de Voronoi de la
red obtenida apilando la red triangular en columnas (una red hexagonal
compacta simple, no la HCP completa) — así que es zonotopo/Voronoi y
motivado por simetría a la vez, sin tener que elegir entre las dos
justificaciones del enunciado.

### 2.2 ¿Es la bola unidad de una norma?

Sí, y de la forma más simple posible: el máximo de dos normas es una norma
(su bola unidad es la intersección de las dos bolas, convexa por ser
intersección de convexos). Con `‖(x,y)‖_hex` la norma del hexágono de §1.2
(escalada a lado `s`, es decir usando el hexágono como bola unidad de
"radio" `s` en vez de 1) y `h` la semi-altura del prisma:

```
‖(x,y,z)‖_prisma = max( ‖(x,y)‖_hex / s ,  |z| / h )
```

Su bola unidad `{‖·‖_prisma ≤ 1}` es exactamente `{(x,y,z) : ‖(x,y)‖_hex ≤ s,
|z| ≤ h}`, el prisma hexagonal de lado `s` y semi-altura `h` — un cilindro
hexagonal, sin necesidad de imponer nada adicional. A diferencia del
dodecaedro rómbico de `04` (que necesitó derivarse como polar de un
zonotopo), aquí el gauge es literalmente el máximo de dos gauges ya
conocidos: no hace falta pasar por la construcción polar.

### 2.3 Superficie de Hausdorff, cara a cara

Fijar `s=1`, `h=1` (los dos parámetros libres del prisma; todo escala
`∝ s²` en las tapas y `∝ s·h` en el lateral, así que fijarlos a 1 no pierde
generalidad del argumento de racionalidad — un factor racional nunca cambia
la clase de irracionalidad de la suma). Dos tapas hexagonales de lado 1 más
seis caras rectangulares de `1×1`:

> **ERRATA corregida en la segunda pasada (ver §5.7).** La primera redacción
> escribió aquí «6 caras laterales, 1×1 cada una = 6» y por tanto
> `S = 3√3 + 6`. Es incorrecto **bajo su propio convenio**: `h` es la
> **semi**altura (§2.2), así que `h=1` significa altura total `2h = 2`, que es
> el mismo convenio con el que §2.4 calcula el volumen `V = (3√3/2)·(2h) = 3√3`.
> Con altura total 2 las caras laterales son `1×2`, no `1×1`. La cifra
> correcta es `3√3 + 12`. El texto de abajo ya está corregido; el número
> antiguo se deja citado aquí y no se borra, porque el protocolo exige que un
> error publicado quede visible. **Ninguna conclusión cambia**: `3√3+6` y
> `3√3+12` son ambos irracionales con el mismo `√3`, y el veredicto de §2.5
> se apoyaba en el volumen, que estaba bien.

```
área de una tapa hexagonal (lado 1) = 3√3/2      (fórmula estándar, lado en vez de circunradio)
2 tapas = 3√3
6 caras laterales, 1×2 cada una (lado 1, altura total 2h = 2) = 12
S_Hausdorff(prisma) = 3√3 + 12
```

```
$ target/release/dz-cli.exe eval "3*sqrt(3)" --mode rational
~5;242E4EEE4819
$ target/release/dz-cli.exe eval "3*sqrt(3)" --mode fixed --scale 8
~5;242E5
$ target/release/dz-cli.exe eval "3*sqrt(3)+12" --mode rational --input-base 10
~15;242E4EEE4819
$ target/release/dz-cli.exe eval "3*sqrt(3)+12" --mode fixed --scale 8 --input-base 10
~15;242E5
```

**Irracional** (`√3`, algebraico) — el `~` en modo racional es la prueba
visual, igual que en cada documento anterior de la serie.

### 2.4 Contraverificación independiente

**Vía 1 — fórmula estándar de área del hexágono en función del lado**,
`A = (3√3/2)a²`, con `a=1`: ya es la que se usó arriba; se contraverifica en
cambio con la fórmula en función del circunradio (§1.5, `A=3√3/2` para
circunradio 1 — coincide porque para el hexágono regular **lado = circunradio**,
§1.1, así que las dos fórmulas son la misma sustitución, no dos cálculos
independientes por casualidad sino por el mismo hecho geométrico que ya se
usó en §1). **Vía 2, genuinamente independiente**: shoelace directo sobre los
6 vértices de una tapa, `(cos60k°,sin60k°)` para `k=0..5`:

```
A = ½|Σ(x_k y_{k+1} − x_{k+1} y_k)|
```

Sumando los 6 términos (aritmética simbólica estándar, no se repite aquí
dígito a dígito por ser mecánica): cada término vale `sin60° = √3/2`, hay 6
términos, `A = ½·6·(√3/2) = 3√3/2` — coincide exactamente con la vía 1.

**Volumen, y por qué ya falla antes de llegar a la superficie.** El volumen
del prisma es `área_tapa · altura = (3√3/2)·(2h) = 3√3·h` (con altura total
`2h=2`, `h=1`): `V = 3√3`.

```
$ target/release/dz-cli.exe eval "3*sqrt(3)" --mode rational
~5;242E4EEE4819
```

**Irracional.** Esto es un contraste importante con el octaedro L₁ de `01`
§E2: allí el **volumen** de la bola era racional en toda dimensión
(`2ⁿ/n!`) y solo la superficie fallaba. Aquí el volumen **ya** es
irracional, porque hereda directamente el área de Lebesgue del hexágono
(§1.5), que nunca fue racional para empezar. El prisma hexagonal no llega
siquiera al punto de partida que tenía L₁: pierde un nivel antes.

### 2.5 Veredicto y casilla de la sección 2

**Casilla (I), en la misma forma que el octaedro y el dodecaedro rómbico de
`01`/`04`.** El prisma hexagonal, motivado genuinamente por `D₆ₕ` (extensión
3D de la simetría hexagonal) y genuinamente una celda de Voronoi de una red
real (la red hexagonal apilada en columnas), tiene superficie de Hausdorff
`3√3+6` (irracional algebraico) **y volumen** `3√3` (también irracional) —
peor que el patrón de los dos cuerpos ya examinados en la serie, que al
menos tenían volumen racional. La causa es simple y no requiere teoría
nueva: la superficie/volumen de cualquier cuerpo con caras o secciones
hexagonales hereda el `√3` del área del propio hexágono (§1.5), y ningún
reescalado racional lo elimina.

---

### 2.6 Busemann y Holmes–Thompson sobre el prisma hexagonal (cerrada en la
segunda pasada)

La primera redacción declaró esto abierto en §6: se argumentó *por analogía*
con `03` y `04` que las dos medidas alternativas también fallarían, pero no se
comprobó. Se comprueba aquí. El resultado es que **la analogía se confirma en
su conclusión y en su mecanismo, y se rompe en un detalle de orden que conviene
reportar.**

**2.6.1 Las definiciones, tal como las fija `03` §1.2–§1.3.** Para un
`k`-plano de dirección `E₀` y la bola unidad `B` de la norma:

```
θ_Bus(E) = ω_k / vol_k^eucl( B ∩ E₀ )
θ_HT (E) = vol_k^eucl( (B ∩ E₀)°_{E₀} ) / ω_k  =  vol_k^eucl( π_{E₀}(B°) ) / ω_k
área_•(S) = θ_•(E) · vol_k^eucl(S)
```

con `ω₂ = π`. **`ω_k` es el punto donde π vuelve a entrar**, y es la razón
estructural por la que `03` y `04` obtuvieron `16π/3`, `48/π` y sus análogos
en el dodecaedro rómbico: trascendentes, no solo irracionales.

A diferencia del octaedro (`03` §1.4) y del dodecaedro rómbico (`04` §1.4),
donde **todas** las caras son congruentes bajo el grupo y basta una densidad,
el prisma tiene **dos clases de caras** bajo `D₆ₕ` y hacen falta dos
densidades. Es la única complicación real respecto de los documentos previos.

**2.6.2 Clase 1 — las dos tapas hexagonales.** `E₀ = {z = 0}`, el plano `xy`.

- `B ∩ E₀ = H`, el hexágono de circunradio 1. `vol₂ = 3√3/2` (§1.5).
- `θ_Bus = π / (3√3/2) = 2π/(3√3) = 2π√3/9`.
- Área euclídea de una tapa: `3√3/2`. Área de Busemann de una tapa:

```
(3√3/2) · 2π√3/9 = (6π·3)/(2·9) = π          exactamente
```

- Dual. El polar de un hexágono regular de circunradio `R=1` y apotema
  `ρ=√3/2` es otro hexágono regular girado 30° con circunradio `R° = 1/ρ =
  2/√3`; su área es `(3√3/2)(R°)² = (3√3/2)(4/3) = 2√3`.
- `θ_HT = 2√3/π`. Área de Holmes–Thompson de una tapa:

```
(3√3/2) · 2√3/π = 18/(2π) = 9/π
```

**2.6.3 Clase 2 — las seis caras rectangulares.** Tómese la arista `v₀v₁` de
`H`; su dirección unitaria es `u = v₁−v₀ = v₂ = (−1/2, √3/2)` (§1.1: la cuerda
es **exactamente** otro vértice, y por tanto unitaria — la misma coincidencia
algebraica del documento, aquí por tercera vez). El plano de dirección de esa
cara es `E₀ = span{u, ẑ}`, y `(u, ẑ)` ya es una base ortonormal de `E₀`.

- **`B ∩ E₀` es un cuadrado de lado 2.** El gauge del prisma es
  `max(‖·‖_hex, |z|)`, así que para `p = a·u + b·ẑ`:
  `‖p‖ = max(|a|·‖u‖_hex, |b|) = max(|a|, |b|)`, porque `‖u‖_hex = ‖v₂‖_hex = 1`
  (es un vértice de la propia bola). Luego `B ∩ E₀ = {max(|a|,|b|) ≤ 1}`,
  el cuadrado `[−1,1]²`, de área **4**. *(Nota de paso, no usada después: la
  sección del prisma en la dirección de sus caras laterales es un cuadrado, o
  sea un plano `L∞` — el caso «paralelogramo» de Gołąb, razón 4, dentro de un
  cuerpo construido para dar 3.)*
- `θ_Bus = π/4`. Área euclídea de una cara lateral `= 1 × 2 = 2`. Área de
  Busemann de una cara lateral `= 2 · π/4 = π/2`.
- Dual: el polar del cuadrado `[−1,1]²` dentro de `E₀` es el cuadrado cruzado
  `{|ξ|+|η| ≤ 1}`, de área **2**. `θ_HT = 2/π`, y el área de
  Holmes–Thompson de una cara lateral `= 2 · 2/π = 4/π`.

**2.6.4 Totales.**

```
S_Bus = 2·π      + 6·(π/2) = 2π + 3π = 5π
S_HT  = 2·(9/π)  + 6·(4/π) = 18/π + 24/π = 42/π
```

```
$ target/release/dz-cli.exe eval "5*pi"  --mode fixed --scale 8 --input-base 10
~13;85E43XE9
$ target/release/dz-cli.exe eval "5*pi"  --mode rational --input-base 10
~13;85E43XEX7X9X
$ target/release/dz-cli.exe eval "42/pi" --mode fixed --scale 8 --input-base 10
~11;4517X96752979024371832E2
$ target/release/dz-cli.exe eval "42/pi" --mode rational --input-base 10
~11;4517X9660366
$ target/release/dz-cli.exe eval "2*sqrt(3)" --mode fixed --scale 8
~3;569E74
$ target/release/dz-cli.exe eval "2*pi*sqrt(3)/9" --mode fixed --scale 8
~1;2615E6706X51654
```

**2.6.5 Contraverificación independiente (dos vías por cada densidad, como
`03` §3.3).**

- **Tapas, Busemann, vía independiente:** por construcción `θ_Bus(E₀) ·
  vol₂(B∩E₀) = ω₂ = π`, así que el área de Busemann de **cualquier** traslado
  de la propia sección `B∩E₀` es `π` sin calcular nada. La tapa del prisma es
  exactamente un traslado de `H = B∩E₀`, luego su área de Busemann es `π`. Esta
  vía no toca ningún `√3` y coincide con el cálculo algebraico de §2.6.2, que
  sí los cancela a mano. ✓
- **Tapas, Holmes–Thompson, vía A (polar) vs. vía B (sombra del dual).** Vía A
  da `2√3` (arriba). Vía B: la norma del prisma es la suma-`ℓ∞` de `‖·‖_hex`
  en `R²` y `|·|` en `R`, así que su dual es la suma-`ℓ₁`, cuya bola es
  `B° = conv( H° × {0} , (0,0,±1) )` — una bipirámide sobre `H°`. Su sombra
  ortogonal sobre `{z=0}` es `H°` (los dos ápices caen al origen), de área
  `2√3`. ✓ Coincidencia exacta, y con dz-cli dígito a dígito:

```
$ target/release/dz-cli.exe eval "2*sqrt(3)" --mode fixed --scale 8
~3;569E74
$ target/release/dz-cli.exe eval "(3*sqrt(3)/2)*(2/sqrt(3))^2" --mode fixed --scale 8
~3;569E73EE22X417459X20EE86553282789340E52584284882329486
```

- **Caras laterales, Holmes–Thompson, vía A (polar) vs. vía B (sombra).** Vía
  A: polar del cuadrado = cuadrado cruzado, área 2. Vía B: sombra de la
  bipirámide `B°` sobre `E₀ = span{u,ẑ}`. Los ápices `(0,0,±1)` proyectan a
  `(0,±1)`. `H°` proyecta al segmento `[−w,w]` en el eje `u`, con `w` la
  función soporte de `H°` en la dirección `u`; como `H°` está girado 30°, `u`
  (a 120°) es **dirección de normal de arista** de `H°`, luego `w` = apotema
  de `H°` `= (2/√3)(√3/2) = 1`. La sombra es
  `conv([−1,1]×{0}, (0,±1))`, el cuadrado cruzado de área **2**. ✓ Las dos
  vías coinciden exactamente.
- **Desigualdad de Santaló–Mahler (`HT ≤ Bus`, `03` §1.3):**
  `42/π = ~11;45…` frente a `5π = ~13;85…`. Se cumple. No es un resultado
  nuevo; es el control de que no hay un inverso invertido en la derivación,
  exactamente el uso que le dio `03` §2.4.

**2.6.6 Veredicto: la analogía se confirma, con una ruptura de orden.**

| medida | prisma hexagonal | ¿racional? | naturaleza |
|---|---|---|---|
| Hausdorff | `3√3 + 12` = `~15;242E5` | no | **algebraico** irracional |
| Busemann | `5π` = `~13;85E43XE9` | no | **trascendente** |
| Holmes–Thompson | `42/π` = `~11;4517X968…` | no | **trascendente** |

- **Lo que confirma la analogía (y es lo que importa):** el mecanismo que `03`
  §4.1 y `04` §1.4 identificaron —`ω_k = π` reintroduce π por la puerta de
  atrás de la normalización— opera aquí **idéntico**. El `√3` del hexágono se
  cancela limpiamente en las dos densidades (es el mismo `√3` arriba y abajo),
  y lo que queda en su lugar es π. Así que las dos medidas alternativas
  **empeoran** la situación respecto de Hausdorff en el único eje que este
  documento mide: `3√3+12` es un irracional algebraico, y `5π` y `42/π` son
  trascendentes. No hay escapatoria por esta vía, igual que no la había para
  el octaedro ni para el dodecaedro rómbico. La conjetura del predecesor era
  correcta.
- **Dónde se rompe la analogía, y se dice porque es real:** en `03` §2.4 el
  orden era `Hausdorff (6,93) < HT (15,28) < Bus (16,76)` — Hausdorff era la
  **más pequeña** de las tres. Aquí el orden es
  `HT (~11;45 ≈ 13,37) < Bus (~13;85 ≈ 15,71) < Hausdorff (~15;24 ≈ 17,20)`:
  Hausdorff es la **más grande**. El orden entre Hausdorff y las otras dos no
  es invariante de la serie, y quien cite «Busemann sale mayor que Hausdorff»
  como patrón general estaría generalizando de dos casos. `HT ≤ Bus` sí es un
  teorema y sí se cumple en los tres cuerpos.
- **Un detalle que no cambia nada pero merece una línea, por el protocolo de
  `N_periodicos`:** el coeficiente racional que emerge es `5` (`S_Bus = 5π`), y
  `1/5` es precisamente el racional que **no termina** en base 12:

```
$ target/release/dz-cli.exe eval "1/5" --mode rational
1/5 (periodica en base 12)
```

  Es decir, incluso el coeficiente entero que sobrevive a la cancelación del
  `√3` es el factor que `99-sintesis.md` §1 señala como el malo para base 12.
  Se anota como curiosidad clasificada en **casilla (III)** y sin ningún peso
  argumental: es una propiedad de la representación de `1/5`, no del prisma.

**Casilla (I)** para el conjunto de §2.6, en la misma forma que `03` §4.3: la
irracionalidad de la superficie del cuerpo es intrínseca, y cambiar de
definición de área de superficie no la elimina — la agrava, cambiando
irracionalidad algebraica por trascendencia.

---

## 3. ¿Es el "3" tan frágil como el "4" de L₁? — No, y se puede probar

`01-hallazgos-A-constantes.md` §7.3 mostró que el "4" de L₁ es un artefacto
de alineación de ejes: el mismo círculo L₁ (el rombo) girado 45° sobre la
**misma** norma L₁ da razón 2, no 4. El encargo pide aplicar exactamente el
mismo escrutinio al "3" del hexágono. La respuesta, verificada aquí de forma
exhaustiva y no solo señalada, es que **no es igual de frágil — es
estrictamente más robusto —, y la razón es la misma coincidencia algebraica
de §1.1.**

### 3.1 El experimento: medir un hexágono girado con la norma del hexágono sin girar

Se fija la norma `‖·‖_hex` de §1.2 (la del hexágono `H` sin rotar). Se define
`K = R_φ(H)`, el mismo hexágono rotado un ángulo `φ` arbitrario (no
necesariamente múltiplo de 60°), y se calcula `perímetro_hex(K)/diámetro_hex(K)`
— exactamente la operación que en L₁ hizo caer el 4 a 2 al usar `φ=45°`.

**Los seis sectores de `‖·‖_hex`**, derivados como en §1.2 (se necesitan
todos para este cálculo):

```
[0°,60°]:    ‖(x,y)‖ =  x + y/√3
[60°,120°]:  ‖(x,y)‖ =  2y/√3
[120°,180°]: ‖(x,y)‖ = −x + y/√3
[180°,240°]: ‖(x,y)‖ = −x − y/√3
[240°,300°]: ‖(x,y)‖ = −2y/√3
[300°,360°]: ‖(x,y)‖ =  x − y/√3
```

### 3.2 Todas las aristas de `K` miden lo mismo bajo `‖·‖_hex`, para cualquier `φ`

La dirección de la cuerda del vértice `k` al `k+1` de `K` es, por §1.1,
`120°+φ+60k` (mod 360°). Al reducir módulo 60° (el ancho de cada sector),
el término `60k` se anula siempre: **la dirección local dentro de su sector
es `φ mod 60°` para las seis aristas, sin excepción**, independientemente de
`k`. Como `H` tiene simetría rotacional de 60° (`R_{60°}(H)=H`, así que
`‖R_{60°}x‖_hex = ‖x‖_hex`), la fórmula de gauge tiene la misma forma
funcional en cada sector una vez expresada en el ángulo local — y la cuerda
tiene magnitud euclidiana `2 sin30°=1` en todos los casos (§1.1: el hexágono
es el único n-ágono donde esta magnitud vale exactamente 1). Verificación
numérica en `φ=20°` (arbitrario, no múltiplo de 30° ni de 60°) de las seis
aristas, evaluando cada una en su sector correspondiente:

| arista | dirección | sector | valor |
|--:|--:|---|--:|
| 0 | 140° | `[120,180]` | 1,13719 |
| 1 | 200° | `[180,240]` | 1,13721 |
| 2 | 260° | `[240,300]` | 1,13715 |
| 3 | 320° | `[300,360]` | 1,13719 |
| 4 | 20°  | `[0,60]`    | 1,13721 |
| 5 | 80°  | `[60,120]`  | 1,13715 |

(las seis coinciden a la precisión mostrada; las diferencias en el último
dígito son redondeo de la comprobación manual con trigonometría de doble
precisión, no del binario). Cerrando en forma simbólica con la arista 4
(que cae exactamente en el sector `[0°,60°]` con ángulo local `φ`,
magnitud 1): el valor común es

```
g(φ) = cos φ + sin φ / √3
```

En `φ=0°`: `g(0)=1` (el hexágono sin rotar, arista=1, coincide con §1.3). En
`φ=30°`:

```
$ target/release/dz-cli.exe eval "2/sqrt(3)" --mode rational
~1;1X33X53EX458
$ target/release/dz-cli.exe eval "2/sqrt(3)" --mode fixed --scale 8
~1;1X33X53E88E54655E3483EXE
```

`g(30°) = cos30°+sin30°/√3 = √3/2+√3/6 = 2√3/3 = 2/√3 ≈ 1,1547`, que
coincide con el cálculo de la tabla interpolado entre `φ=20°` y `φ=… ` — y,
importante, **también coincide con el gauge del propio vértice de `K` en esa
dirección**, porque el vértice tiene la misma magnitud (1) que la arista, la
misma coincidencia de §1.1 operando dos veces.

### 3.3 Por qué eso fuerza que la razón sea 3 para cualquier `φ`, con prueba

Como todos los vértices de `K` tienen el mismo gauge `g(φ)` (mismo
argumento: dirección local `φ mod 60°`, magnitud 1), y `K` es la envolvente
convexa de esos 6 puntos, la convexidad de la norma da `‖p‖_hex ≤ g(φ)` para
todo `p ∈ K` (designaldad de Jensen sobre los segmentos entre vértices de
igual gauge), con igualdad en los vértices. Entonces, para cualesquiera
`p,q ∈ K`:

```
‖p−q‖_hex ≤ ‖p‖_hex + ‖q‖_hex ≤ 2g(φ)
```

y la cota se alcanza en un par de vértices antipodales, así que
`diámetro_hex(K) = 2g(φ)` **exactamente**. Y por §3.2 el perímetro de
autonorma es `6·g(φ)` exactamente (las seis aristas valen lo mismo). Luego

```
razón(φ) = 6g(φ) / 2g(φ) = 3          para TODO φ, no solo φ = 0°, 60°, 120°...
```

**El "3" del hexágono es invariante bajo rotación arbitraria de la propia
bola contra su propia norma — una afirmación estrictamente más fuerte que lo
que Gołąb garantiza por sí solo** (Gołąb dice que *cualquier* hexágono
afínmente regular tiene razón 3 **medido con su propia norma que él mismo
define** — es decir, cuando el cuerpo medido es exactamente la bola unidad de
la norma con la que se mide. Lo que se prueba aquí es distinto y más fuerte:
que un hexágono **rotado respecto de una norma hexagonal fija de otro
hexágono** — dos cuerpos con posiciones distintas, aunque congruentes — sigue
dando 3, cosa que Gołąb no dice por sí mismo).

### 3.4 El control negativo: el mismo experimento con el rombo de L₁ sí cambia

Repitiendo §3.2–§3.3 para `n=4` (el rombo, la propia norma L₁), con
`θ=90°`, magnitud de cuerda `2 sin45°=√2` (≠1, a diferencia del hexágono) y
desfase `θ/2+90°=135°`, que módulo `θ=90°` da **45°, constante,
independiente de `φ`** — no `φ` mismo como en el hexágono. El gauge local,
usando la fórmula de sector `x+y` (primer cuadrante):

```
edge(φ) = √2 · [cos(45°+φ) + sin(45°+φ)] = √2 · √2 sin(90°+φ) = 2 cos φ
```

En `φ=0°`: `edge=2` (perímetro 8, coincide con §1.6). En `φ=45°` (rotar el
rombo 45° — exactamente el caso del cuadrado axial de `01` §1.2c):

```
$ target/release/dz-cli.exe eval "sqrt(2)" --mode rational
~1;4E79170X07E8
$ target/release/dz-cli.exe eval "4*sqrt(2)" --mode rational
~5;7X70643427X8
```

`edge(45°)=2cos45°=√2`, perímetro `4√2`, diámetro `2√2` (por el mismo
argumento de convexidad de §3.3, aplicado a la magnitud del vértice, que
para `n=4` es 1 constante — el vértice conserva magnitud 1 aunque la arista
no):

```
$ target/release/dz-cli.exe eval "4*sqrt(2)/(2*sqrt(2))" --mode rational
~2
```

`razón(45°) = 4√2/2√2 = 2` — **coincide exactamente con el "2" ya conocido
de `01` §1.2c**, y confirma con esta maquinaria general que el "4" de L₁
**sí** cambia con la rotación, mientras el "3" del hexágono **no** cambia.
(Nótese el `~` en el último cálculo pese a que el valor exacto es
inequívocamente 2: `dz-cli` calcula `√2` truncado y luego divide, así que el
`~` aquí es un artefacto del instrumento —arrastra el truncamiento de la raíz
intermedia, no una duda real sobre el resultado exacto— y no de la
matemática; es la misma clase de aviso que `04` §2.2 documentó para
`(√3/2)/√3`.)

### 3.5 Por qué la asimetría entre los dos casos, en una frase

`edge(φ) mod θ` para el hexágono es exactamente `φ`, mientras que para el
rombo es `45°+φ` — un desfase que depende de `θ/2+90° mod θ`. Para que ese
desfase sea `0 (mod θ)`, y para que además la magnitud de la cuerda sea
exactamente 1 (la condición de §1.1 que hace `g(φ)` coincidir con el gauge
del propio vértice, sin la cual la prueba de convexidad de §3.3 no cierra),
la única solución con `n` entero par pequeño es `θ=60°`, `n=6`. **La
robustez del "3" no es una propiedad general de "buena simetría"; es la
misma coincidencia algebraica única de §1.1, reaparecida por segunda vez en
este documento con una consecuencia distinta.** No se comprobó si existe
algún otro `n` (la nota de §1.3 sobre `θ=180°/(2k+1)` sugiere `n=10` como
candidato a *fase* alineada, aunque falla la condición de magnitud —
`2 sin18°≈0,618≠1` — así que no se espera que reproduzca la invariancia
completa; no se persiguió más allá de esta observación).

> **Este párrafo es FALSO y se refuta en §3.7, donde por fin se calcula.**
> Se deja escrito en vez de borrarlo, porque el protocolo exige que una
> conjetura publicada y luego refutada quede visible. `n=10` **sí** reproduce
> la invariancia completa: la condición de magnitud `|cuerda| = 1` que este
> párrafo daba por necesaria **no lo es** — las magnitudes de la cuerda y del
> vértice entran en la razón solo por su cociente, y se cancelan. Lo único
> que importa es la condición de **fase**, y ésa se cumple para toda una
> familia infinita (`n ≡ 2 mod 4`), no solo para `n=6`. La frase «es la misma
> coincidencia algebraica única de §1.1» es, por tanto, incorrecta como
> explicación de §3; la explicación correcta es la dicotomía módulo 4 de
> §1.8.1, y está en §3.7.

### 3.6 Veredicto de la sección 3

**El "3" del hexágono NO es un artefacto de alineación como el "4" de L₁.**
Es, mostrado aquí con prueba y no solo con ejemplo, invariante bajo
**cualquier** rotación de la bola respecto de su propia norma — algo que ni
siquiera Gołąb garantiza directamente, y que además puede citarse junto a
Gołąb como una segunda confirmación independiente del mismo número. Es el
único resultado de todo este documento donde el hexágono es **estrictamente
mejor** que L₁, no solo distinto. Pero adviértase con la misma honestidad que
exige el resto del protocolo: esta robustez es de la constante de
**perímetro/diámetro**, no del programa en conjunto — el área (§1.5) y la
superficie 3D (§2.3) siguen siendo irracionales sin ningún atisbo de la
misma protección, exactamente como predice el mecanismo de §3.5 (que es
específico de la longitud de arco de autonorma, no del área de Lebesgue, que
nunca entra en el argumento de convexidad de §3.3).

### 3.7 El experimento de rotación para `n = 8, 10, 12` (cerrada en la segunda
pasada) — y la robustez resulta ser una **paridad**, no una coincidencia

Declarada abierta en §6. Se cierra aquí, y el resultado **corrige** §3.5.

**3.7.1 Forma cerrada de `razón(φ)` para todo `n` par.** Se usa la función
radial de §1.8.1, `R(α) = cos(θ/2)/cos(δ(α))` con
`δ(α) = dist(α − θ/2, θZ)`. Sea `K = R_φ(P_n)` el n-ágono rotado, medido con
la norma fija `‖·‖_{P_n}` del sin rotar.

- **Aristas de `K`:** magnitud euclídea `2 sin(θ/2)`, dirección
  `(θ+π)/2 + φ + kθ`; el `kθ` desaparece al reducir módulo `θ`, así que
  **las `n` aristas tienen el mismo gauge para cualquier `φ` y cualquier `n`**
  (esto sí es general, no era propio del hexágono), con
  `δ_e = dist(π/2 + φ, θZ)`.
- **Vértices de `K`:** magnitud 1, direcciones `φ + kθ`; igual,
  **todos con el mismo gauge**, `δ_v = dist(φ + θ/2, θZ)`. Por el argumento de
  convexidad de §3.3 (que no usa nada específico del hexágono),
  `diám_{P_n}(K) = 2·gauge(vértice)` exactamente, para todo `n` par.

```
razón(φ) = [n · 2 sin(θ/2) cos(δ_e)/cos(θ/2)] / [2 cos(δ_v)/cos(θ/2)]
         = n · sin(π/n) · cos(δ_e) / cos(δ_v)
```

Y ahora la misma dicotomía módulo 4 de §1.8.1, `π/2 = (m/2)θ` con `n = 2m`:

```
n ≡ 2 (mod 4):  δ_e = dist(φ + θ/2, θZ) = δ_v   ⇒  el cociente vale 1
                razón(φ) = n sin(π/n),  CONSTANTE en φ  →  INVARIANTE

n ≡ 0 (mod 4):  δ_e = dist(φ, θZ) ≠ δ_v = dist(φ + θ/2, θZ)
                razón(φ) = n sin(π/n) · cos(δ_e)/cos(δ_v),  DEPENDE de φ
                recorrido:  [ (n/2)·sin(2π/n) , n·tan(π/n) ]
                              φ = θ/2            φ = 0
```

**Ése es el resultado, y contradice §3.5.** La invariancia rotacional no es una
propiedad del hexágono: es una propiedad de **toda la clase `n ≡ 2 (mod 4)`**
— `n = 6, 10, 14, 18, …` — y falla para **toda** la clase `n ≡ 0 (mod 4)` —
`n = 4, 8, 12, 16, …`. Es exactamente la misma dicotomía que gobierna la forma
cerrada de §1.8, aparecida aquí por una razón distinta (fase de la cuerda
contra la retícula de normales de arista, en vez de valor del gauge en
`φ=0`), y es la unificación de las dos mitades de este documento.

Obsérvese que la condición «`|cuerda| = 1`» que §3.5 daba por esencial **no
aparece en ningún punto de la derivación**: las magnitudes `2 sin(θ/2)` y `1`
entran en numerador y denominador y se cancelan. Era una coincidencia del
hexágono, no el mecanismo.

**3.7.2 Verificación numérica, los cuatro `n`.** Todos con
`--mode fixed --scale 8 --input-base 10`.

| n | φ | `δ_e` | `δ_v` | expresión evaluada | razón |
|--:|--:|--:|--:|---|---|
| **4** (≡0) | 0° | 0° | 45° | `4*sin(pi/4)*cos(0)/cos(pi/4)` | `~4` |
| **4** | 45° | 45° | 0° | `4*sin(pi/4)*cos(45*pi/180)/cos(0)` | `~1;EEEEEEEEX1537884` (= 2) |
| **6** (≡2) | 11° | 26° | 26° | `12*tan(pi/6)*cos(11*pi/180)/(2*cos(11*pi/180)/cos(pi/6))` | `~3;000000008554` (= 3) |
| **8** (≡0) | 0° | 0° | 22,5° | `8*sin(pi/8)*cos(0)/cos(pi/8)` | `~3;39210867086X…` |
| **8** | 10° | 10° | 12,5° | `8*sin(pi/8)*cos(10*pi/180)/cos(12.5*pi/180)` | `~3;10840810X551…` |
| **8** | 20° | 20° | 2,5° | `8*sin(pi/8)*cos(20*pi/180)/cos(2.5*pi/180)` | `~2;X67XE5133X52…` |
| **8** | 22,5° | 22,5° | 0° | `8*sin(pi/8)*cos(22.5*pi/180)/cos(0)` | `~2;9E3632151E21E` (= `2√2`) |
| **10** (≡2) | 20° | 2° | 2° | `20*tan(pi/10)*cos(2*pi/180)/(2*cos(2*pi/180)/cos(pi/10))` | `~3;10E9920136292XX…` |
| **10** | 17° | 17° | 17° | `20*tan(pi/10)*cos(17*pi/180)/(2*cos(17*pi/180)/cos(pi/10))` | `~3;10E9920136292XX…` |
| **12** (≡0) | 0° | 0° | 15° | `12*sin(pi/12)*cos(0)/cos(pi/12)` | `~3;27024002977…` |
| **12** | 7,5° | 7,5° | 7,5° | `12*sin(pi/12)*cos(7.5*pi/180)/cos(7.5*pi/180)` | `~3;132X564` |
| **12** | 15° | 15° | 0° | `12*sin(pi/12)*cos(15*pi/180)/cos(0)` | `~3;00000000426119` (= 3) |

Contraverificación de los valores de contraste:

```
$ target/release/dz-cli.exe eval "2*sqrt(2)"     --mode fixed --scale 8
~2;9E363218
$ target/release/dz-cli.exe eval "10*sin(pi/10)" --mode fixed --scale 8 --input-base 10
~3;10E991EX
$ target/release/dz-cli.exe eval "(5*sqrt(5)-5)/2" --mode fixed --scale 8 --input-base 10
~3;10E992006
```

(los `~` y los desacuerdos en los últimos dígitos son truncamiento de serie de
`sin`/`tan`/`sqrt`, el mismo artefacto que §3.4 documentó; `~1;EEEEEEEE…` es
`2` y `~3;0000000…` es `3`.)

**3.7.3 Lectura, caso por caso.**

- **`n = 8`: NO es robusto, ni siquiera parcialmente.** La razón varía de
  forma continua entre `8 tan(π/8) = 8√2−8 ≈ 3,31` (ejes alineados) y
  `4 sin(π/4) = 2√2 ≈ 2,83` (rotado media arista). Ni un solo valor racional
  en el recorrido: los dos extremos son irracionales algebraicos y el interior
  es continuo. El octógono pierde por partida doble.
- **`n = 12`: NO es robusto, y su recorrido contiene una sorpresa.** Varía
  entre `24−12√3 ≈ 3,215` (alineado, irracional) y **exactamente 3** (rotado
  `θ/2 = 15°`). Es decir: **el dodecágono, que en §1.6 salía como el peor de
  los cuatro en perímetro/diámetro, alcanza el valor limpio 3 — pero solo en
  una orientación concreta, y por tanto como un artefacto de alineación, que
  es precisamente el defecto que `01` §7.3 le reprochaba a L₁.** La razón de
  ese 3 no es nueva: por §3.7.1 el valor en `φ = θ/2` es `(n/2)·sin(2π/n)`,
  que es **literalmente la fórmula del área** de §1.5/§1.7, así que por el
  teorema de Niven es racional exactamente en `n = 4` (→ 2) y `n = 12` (→ 3).
  El «2» de L₁ girado 45° y el «3» del dodecágono girado 15° son **el mismo
  fenómeno**, y ambos son artefactos.
- **`n = 10`: SÍ es completamente robusto** — invariante bajo rotación
  arbitraria, igual que el hexágono, contra lo que §3.5 conjeturó. Pero su
  constante `10 sin(π/10) = (5√5−5)/2 ≈ 3,090` es **irracional**, y de la peor
  especie posible para este proyecto: su irracionalidad es `√5`, el factor que
  `99-sintesis.md` §1 identifica como el que base 12 no regala (el mismo
  motivo por el que §2.1 descartó el icosaedro).

**3.7.4 La síntesis de §1.8 y §3.7, que es el resultado central de la segunda
pasada.** Las dos preguntas —¿qué razones son racionales? y ¿qué razones son
robustas a rotación?— se responden con **la misma dicotomía módulo 4**, y
cruzarlas deja exactamente un superviviente:

| clase | razón (alineado) | ¿robusta a rotación? | ¿racional? |
|---|---|---|---|
| `n ≡ 2 (mod 4)`: 6, 10, 14, 18… | `n sin(π/n)`, creciente `3 → π` | **sí, siempre** | **solo `n=6`** (→ 3), por Niven-seno |
| `n ≡ 0 (mod 4)`: 4, 8, 12, 16… | `n tan(π/n)`, decreciente `4 → π` | **no, nunca** | **solo `n=4`** (→ 4), por Niven-tangente |

```
Racional ∧ robusto  ⟺  n = 6.
```

**El hexágono no es especial por ser robusto (lo son infinitos), ni por ser
racional (también lo es el cuadrado). Es el único punto donde las dos
propiedades se cruzan.** Esto es a la vez un refuerzo y un recorte del
veredicto de §3.6: el «3» sigue siendo el mejor resultado del documento y
sigue siendo estrictamente más sólido que el «4» de L₁ —que ahora se entiende
como miembro de la clase estructuralmente frágil, no como un accidente—, pero
la explicación que §3.5 daba («la coincidencia algebraica única de §1.1») era
**incorrecta**, y la correcta es una paridad que el hexágono comparte con una
familia infinita.

**Casilla (II)** para §3.7 completa: la robustez rotacional y la racionalidad
son propiedades que sobreviven al cambio de norma y cambian de valor; ninguna
de las dos tiene relación con la base numérica. La aparición de `√5` en `n=10`
es lo más cerca que llega este documento de tocar **casilla (III)**, y es una
pérdida, no una ganancia.

---

## 4. Coste medido en puertas

Cifras de contexto, con la misma advertencia de `01` §0.4: `pi` cuesta 0
puertas por memoización, así que ninguna comparación de esta sección es
"L₁/hexágono vs. Euclides" — es solo el coste de evaluar cada constante ya
obtenida, para constatar (otra vez) que el coste vive en el algoritmo de
`sqrt` y no en la forma simbólica.

| expresión | valor | total | depth | comando |
|---|--:|--:|--:|---|
| perímetro hexágono `=6` | `6` | 13.576 | 254 | `eval "6" --mode fixed --scale 8 --gates` |
| perímetro rombo L₁ `=8` | `8` | 16.634 | 254 | `eval "8" --mode fixed --scale 8 --gates` |
| diámetro `=2` | `2` | 7.460 | 254 | `eval "2" --mode fixed --scale 8 --gates` |
| área dodecágono `=3` | `3` | 8.989 | 254 | `eval "3" --mode fixed --scale 8 --gates --input-base 10` |
| área hexágono `=3√3/2` | `~2;721586` | 19.235.975 | 13.816 | `eval "3*sqrt(3)/2" --mode fixed --scale 8 --gates` |
| razón dodecágono `=24−12√3` | `~3;27024` | 18.047.432 | 13.360 | `eval "24-12*sqrt(3)" --mode fixed --scale 8 --gates --input-base 10` |
| superficie Hausdorff prisma `=3√3+12` | `~15;242E5` | 18.034.120 | 13.360 | `eval "3*sqrt(3)+12" --mode fixed --scale 8 --gates --input-base 10` |
| superficie Busemann prisma `=5π` | `~13;85E43XE9` | **26.358** | 15.168 | `eval "5*pi" --mode fixed --scale 8 --gates --input-base 10` |
| superficie Holmes–Thompson prisma `=42/π` | `~11;4517X968…` | 3.150.605 | 15.168 | `eval "42/pi" --mode fixed --scale 8 --gates --input-base 10` |
| razón decágono `=10 sin(π/10)` | `~3;10E991EX` | 24.211.204 | 18.763 | `eval "10*sin(pi/10)" --mode fixed --scale 8 --gates --input-base 10` |

Las tres filas nuevas del prisma (§2.6) ilustran de forma especialmente cruda
la advertencia de `01` §6.1 y hay que leerlas con ella delante: **`5π` cuesta
26.358 puertas y `3√3+12` cuesta 18.034.120, casi mil veces más — y sin
embargo `5π` es trascendente y `3√3+12` es meramente algebraico.** El contador
de puertas mide el algoritmo de evaluación (`pi` está memoizada y cuesta 0;
`sqrt` corre Newton lleno de divisiones), no la complejidad aritmética del
número. Si alguien usara §4 como criterio de «qué constante es más simple»,
concluiría que la medida de Busemann es la buena, que es exactamente lo
contrario de lo que dice §2.6.6. **Ninguna cifra de esta sección es evidencia
sobre H₁, y ésta es la ilustración más clara de por qué.**

La lectura es la misma que en `01` §6.1: cualquier constante que involucre
una raíz cuadrada cuesta tres órdenes de magnitud más que una constante
entera, sin relación con si el resultado final es racional (el área del
hexágono, `~2;72...`, y la razón del dodecágono, `~3;27...`, cuestan lo
mismo en puertas que cualquier otra evaluación de `sqrt`, sea el resultado
final racional o no) — el coste vive en el algoritmo de Newton de `sqrt`,
no en la geometría ni en la base.

---

## 5. Lo que no funcionó — y lo que sí, contra lo esperado

Sección obligatoria del protocolo. Aquí hay dos tipos de resultado, y el
protocolo exige no mezclar el tono: fracasos reales, y un éxito real que
convenía subestimar más que sobrevender.

### 5.1 El área del hexágono no se salva de ningún modo explorado

Igual que `01` §7.1 con el octaedro: no se probaron aquí Busemann ni
Holmes–Thompson sobre el hexágono 2D, porque en el plano las tres
definiciones de longitud/área coinciden (`03` §1, reducción al caso `k=1`) —
así que no hay una segunda definición que probar en 2D, a diferencia de la
superficie 3D. El área del hexágono es irracional bajo la única definición
que existe en el plano, sin escapatoria.

*Corrección (revisión del 2026-09-13):* sí hay escapatoria, y no es una segunda
definición sino una segunda **coordenada**. Busemann y Holmes–Thompson coinciden
en el plano con la longitud, no con el área de Lebesgue de la bola, que no es un
invariante de la norma. El hexágono afín `(±1, 0), (0, ±1), ±(1, −1)` define la
misma geometría y tiene área 3. Lo que se probó irracional es el área del
hexágono **euclidianamente** regular, que es estructura ajena a la norma. §5.2
cae por la misma razón: el prisma afín tiene volumen 6.

### 5.2 El prisma hexagonal falla antes de llegar a la superficie

A diferencia de L₁, cuyo volumen era racional en toda dimensión (`01` E2) y
solo su superficie 3D fallaba, el prisma hexagonal tiene **volumen ya
irracional** (§2.4) — la comparación con el patrón L₁ es directamente peor,
no solo igual de mala.

### 5.3 El dodecágono, el candidato con más "12" en el nombre, es el peor en
perímetro/diámetro de los cuatro examinados

`§1.6`: razón `24−12√3 ≈ 3,215`, la más alejada de un entero de las cuatro
(`4, 3, 8√2−8≈3,31, 24−12√3≈3,215`) salvo por estar, como los demás no
extremos, dentro del intervalo de Gołąb. Si la intuición de partida era "el
n-ágono de 12 lados debería ser el más *base-12*", el resultado es
exactamente el opuesto en esta constante.

### 5.4 Éxito real y verificado, que no se buscaba: la robustez rotacional del "3"

`§3`. No estaba en el encargo original más que como pregunta de escrutinio
("¿es tan frágil como el 4 de L₁?"), y la respuesta resultó ser una
demostración completa de lo contrario, no solo una observación. Se reporta
con el mismo cuidado que el resto del documento: es un resultado genuino
sobre la constante de **perímetro/diámetro específicamente**, no una
propiedad general del hexágono ni un argumento a favor de que "rediseñar la
simetría para base 12" funcione en general — la sección 5.2/5.3 muestran que
el mismo rediseño falla en las otras dos constantes examinadas.

### 5.5 No se buscó sistemáticamente un `n` racional intermedio (§1.8)

**Cerrado en la segunda pasada: §1.8 lo demuestra, y la respuesta es que no
existe.** El resto del párrafo original sigue en pie y sigue abierto. Tampoco se buscó si existe algún cuerpo centralmente
simétrico **no regular** (afín a un hexágono deformado, o alguna otra clase)
cuya área **y** perímetro/diámetro sean ambos racionales simultáneamente —
la pregunta que decidiría si la tesis final de este documento (§6) tiene
alguna excepción further afield. No se encontró ninguna en el espacio
examinado (cuatro n-ágonos regulares, dos referencias externas: Niven y
Gołąb), pero el espacio de cuerpos convexos centralmente simétricos es
infinito-dimensional y no se agotó ni remotamente.

### 5.6 Trampa de la calculadora encontrada en caliente, y corregida en el propio documento

Al preparar las cifras de área con la fórmula general `(n/2)sin(2π/n)`, un
primer intento sin `--input-base 10` (`eval "6*sin(2*pi/12)"`, con `12`
leído como dozenal = 14 decimal) dio `~2;72X60E5`, que **no** coincidía con
la sustitución algebraica esperada (`3`). La discrepancia no era un error de
álgebra: era la trampa ya documentada en `01` §0.2 y en `99-sintesis.md` §5,
punto 3 ("los literales pelados se leen en dozenal"), en la que este propio
documento tropezó al construirse en vez de solo citarla de memoria. Se
verificó explícitamente que `2*pi/12` (sin `--input-base 10`) y `pi/6` dan
resultados distintos, y que con `--input-base 10` coinciden dígito a dígito
(§1.7). Se deja constancia en vez de limpiarla en silencio, siguiendo la
misma norma de honestidad del resto de la serie.

### 5.7 Reverificación de la primera pasada: **tres discrepancias encontradas**

Segunda pasada. Se reejecutó una muestra de la bitácora de §8 contra
`target/release/dz-cli.exe` y se contrastó cifra a cifra con el cuerpo del
documento. El protocolo obliga a reportar esto en voz alta: una cifra
equivocada dentro de un documento con aspecto de terminado es el peor
desenlace posible de esta serie.

**Lo que sí se verificó y salió exacto** (no hay motivo para desconfiar del
resto): `6/2 → 3`; `8/2 → 4`; `3*sqrt(3)/2 → ~2;721585EE840X` (racional) y
`~2;721586` (fijo); `2*sqrt(2)-2 → ~0;9E36321813E4`;
`8*sqrt(2)-8 → ~3;392108685394`; `4-2*sqrt(3) → ~0;652048004X6X`;
`48-24*sqrt(3) → ~6;52048004X6X`; `24-12*sqrt(3) → ~3;27024002535`;
`6*sin(2*pi/12) --input-base 10 → ~3` en los dos modos;
`2/sqrt(3) → ~1;1X33X53EX458`; `sqrt(2) → ~1;4E79170X07E8`;
`4*sqrt(2) → ~5;7X70643427X8`; `4*sqrt(2)/(2*sqrt(2)) → ~2`;
`3*sqrt(3) → ~5;242E4EEE4819`; **las ocho filas de la tabla de áreas de §1.7**
(`n = 3…12`), una por una; y **las seis filas de conteo de puertas de §4**,
exactas hasta el último dígito de los cinco contadores y de `depth`.

**Discrepancia 1 — §1.5, comando equivocado junto a una cifra correcta
(ironía incluida).** El documento publica como «contraverificación
independiente» del área del hexágono:

```
$ target/release/dz-cli.exe eval "3*sin(2*pi/12)" --mode fixed --scale 8 --input-base 10
~2;721586
```

El binario **no devuelve eso**. Devuelve:

```
$ target/release/dz-cli.exe eval "3*sin(2*pi/12)" --mode fixed --scale 8 --input-base 10
~1;6
```

que es correcto y es `3·sin(30°) = 3/2 = 1;6` en dozenal. El error está en el
**comando**, no en el valor: el área del hexágono es `(n/2)sin(2π/n)` con
`n = 6`, así que hay que escribir `3*sin(2*pi/6)`, no `3*sin(2*pi/12)`. Con el
denominador correcto:

```
$ target/release/dz-cli.exe eval "3*sin(2*pi/6)" --mode fixed --scale 8 --input-base 10
~2;721586
```

**La cifra publicada `~2;721586` es correcta y la conclusión de §1.5 (área
`3√3/2`, irracional) no cambia**; lo que falla es que el comando pegado al
lado no la produce — el `12` del denominador se coló de la fila `n=12` de
§1.7, que sí usa `2*pi/12` y sí da `~3`. Es decir: §5.6 se escribió
precisamente para documentar una trampa de la calculadora, y al corregirla se
introdujo en la línea de al lado un error de tipo distinto (denominador
equivocado en vez de base equivocada) que sobrevivió a la primera pasada.
**La corrección está aplicada en §1.5 y en la bitácora de §8.**

**Discrepancia 2 — §2.3 y §0, superficie de Hausdorff del prisma
internamente inconsistente.** §2.3 declara «`s=1`, `h=1`» y a continuación
«6 caras laterales, `1×1` cada una = 6», dando `S = 3√3 + 6`; pero §2.2 define
`h` como **semi**altura y §2.4 calcula el volumen como
`(3√3/2)·(2h) = 3√3` con altura total 2. Con altura total 2 las caras
laterales son `1×2` y la superficie es `3√3 + 12`. Los dos números no pueden
ser ambos correctos. El bueno es `3√3 + 12`:

```
$ target/release/dz-cli.exe eval "3*sqrt(3)+12" --mode rational --input-base 10
~15;242E4EEE4819
```

Corregido en §2.3 con nota de errata visible y en la tabla de §0. **Ninguna
conclusión cambia**: los dos valores son irracionales con el mismo `√3`, y el
veredicto de §2.5 se apoyaba en el volumen, que estaba bien calculado. Pero la
cifra publicada era falsa y queda constancia.

**Discrepancia 3 — §3.2, la tabla de las seis aristas tiene ruido mayor del
que declara.** La tabla da `1,13719 / 1,13721 / 1,13715` y atribuye la
dispersión a «redondeo de la comprobación manual con trigonometría de doble
precisión». La dispersión (`6·10⁻⁵`) es varios órdenes de magnitud mayor que
el redondeo de doble precisión, así que la explicación es incorrecta aunque la
conclusión no lo sea. El valor común exacto, medido con el binario, es

```
$ target/release/dz-cli.exe eval "cos(20*pi/180)+sin(20*pi/180)/sqrt(3)" --mode fixed --scale 8 --input-base 10
~1;179013879470230X5654E7X4
```

es decir `1;179013…` = `1,137152…`, que es la fila `1,13715` de la tabla; las
otras dos son aritmética manual floja. **La conclusión de §3.2 (las seis
aristas valen lo mismo) es correcta y ahora no depende de esa tabla en
absoluto**: §3.7.1 la demuestra simbólicamente para todo `n` par y todo `φ`.

**Balance.** Tres discrepancias, todas de transcripción o de aritmética
auxiliar, **ninguna que altere un veredicto**. Dos de ellas (1 y 2) eran
cifras o comandos publicados que el binario no reproduce, que es exactamente
la clase de fallo que esta serie más teme. La única conclusión sustantiva que
sí cae en esta segunda pasada no viene de aquí sino de §3.7: la explicación de
§3.5 sobre por qué el hexágono es robusto era falsa.

### 5.8 Lo que la segunda pasada cambió de fondo

No es un fracaso de cálculo sino una hipótesis explicativa refutada, y por eso
va aparte de §5.7. §3.5 y §7 sostenían que la robustez rotacional del «3» es
«la misma coincidencia algebraica única de §1.1, reaparecida por segunda vez».
§3.7 muestra que es **falso**: la robustez es una propiedad de paridad
(`n ≡ 2 mod 4`) compartida por una familia infinita —`6, 10, 14, 18, …`— y la
condición `|cuerda| = 1` que §3.5 daba por esencial se cancela en el cociente
y no interviene. El hexágono conserva su lugar, pero por un motivo distinto y
más preciso: **es el único n-ágono regular que es a la vez robusto y racional**
(§3.7.4), y esa intersección la determina el teorema de Niven, no la
coincidencia de §1.1. Se reporta como lo que es: la explicación publicada era
peor que el hecho publicado.

---

## 6. Limitaciones conocidas

- ~~**No se probaron Busemann ni Holmes–Thompson sobre el prisma
  hexagonal.**~~ **CERRADA en la segunda pasada: §2.6.** Se calcularon las dos
  explícitamente (`S_Bus = 5π`, `S_HT = 42/π`, ambas trascendentes, frente a
  la `3√3+12` algebraica de Hausdorff), con doble vía de contraverificación
  para cada densidad y con la comprobación de Santaló–Mahler. La analogía con
  `03`/`04` se confirma en su conclusión y en su mecanismo (`ω_k = π`); se
  rompe solo en el **orden** relativo entre Hausdorff y las otras dos, que en
  `03` era el inverso. Lo que sigue sin hacerse: la bipirámide hexagonal, y
  cualquier cuerpo `D₆ₕ` que no sea el prisma.
- **Las densidades de §2.6 se calculan para las dos clases de caras que
  `D₆ₕ` distingue, y no se verificó que no haya una tercera.** El argumento es
  que `D₆ₕ` actúa transitivamente sobre las 6 caras laterales (rotación de
  60°) y sobre las 2 tapas (reflexión en `z`), y que ésas son todas las caras
  de un prisma. Es correcto por inspección, pero se apoya en el recuento de
  caras, no en un cálculo de órbitas.
- **No se buscó una bipirámide hexagonal como alternativa al prisma.** El
  enunciado la propone como candidato equivalente; no se calculó porque el
  prisma ya basta para mostrar el patrón (volumen irracional), y no hay razón
  estructural para esperar que la bipirámide (que también tiene secciones
  hexagonales) escape al mismo `√3`.
- ~~**La sección 3 no se extendió a n=8, n=12 ni a reflexiones.**~~
  **CERRADA en la segunda pasada para las rotaciones: §3.7.** `n=8` y `n=12`
  no tienen ninguna versión, ni siquiera parcial, de la robustez (varían de
  forma continua en `φ`); `n=10` sí la tiene por completo, contra lo que §3.5
  conjeturó. **Sigue sin hacerse la parte de reflexiones**: todo §3 y §3.7
  trabaja con `R_φ` (rotaciones puras). Una reflexión de `K` respecto de un eje
  arbitrario no está cubierta por la derivación de §3.7.1, y no se calculó —
  aunque para cuerpos centralmente simétricos y polígonos regulares no se
  espera novedad, no se comprobó.
- **§3.7 mide una familia de un parámetro (el ángulo `φ`), no todo el grupo
  afín.** La pregunta «¿y bajo una transformación afín general?» no se toca.
  Gołąb es invariante afín, así que una respuesta completa exigiría separar lo
  que es invariante afín de lo que es invariante solo por rotación; §3.7
  responde lo segundo.
- **El teorema de Niven (§1.7, §1.8.4, §3.7.3) se usa como hecho citado, no se
  demuestra aquí.** Es un resultado clásico y estándar (Niven, *Irrational
  Numbers*, 1956); su demostración no es geométrica y está fuera del alcance
  de este documento. Conviene subrayar cuánto peso soporta ahora: **la
  demostración de §1.8.4 —el resultado más fuerte del documento— descansa
  enteramente sobre él**, en sus versiones de seno y de tangente. Si Niven
  cayera, caería §1.8.4; no hay aquí una segunda ruta independiente a ese «no
  existe» (§1.4 y §1.8.2 contraverifican los **valores**, no la exhaustividad).
- **La reverificación de §5.7 es una muestra, no la bitácora completa.** Se
  reejecutaron ~30 de los comandos de §8, elegidos por cubrir todas las cifras
  que sostienen un veredicto, más las seis filas de puertas. No se reejecutó
  cada línea, así que no puede descartarse una cuarta discrepancia en las
  líneas no muestreadas.
- **Las cifras nuevas de §1.8.3, §2.6 y §3.7 se obtienen con `sin`/`cos`/`tan`
  de `dz-sci`, que son series truncadas a la escala de trabajo.** Los `~` de
  esas tablas no distinguen por sí solos «irracional» de «racional calculado
  por una serie»: por eso cada afirmación de racionalidad va acompañada de la
  sustitución algebraica exacta (`6·(1/2) = 3`, `4·1 = 4`) y el papel del
  binario es contraverificar, no decidir. Es la misma disciplina que §1.7 ya
  aplicaba, ahora usada de forma sistemática.
- **No se exploró ningún cuerpo convexo centralmente simétrico no regular.**
  Todo lo examinado son n-ágonos regulares y sus extensiones prismáticas.
  Existe, en principio, un espacio mucho mayor de cuerpos convexos con
  simetría `D₆` o `D₁₂` sin ser regulares, no explorado.
- **El coste en puertas (§4) no es evidencia geométrica ni dozenal**, exactamente
  con la misma advertencia que `01` §6.1: `pi` cuesta 0 puertas por diseño,
  y el coste medido de `sqrt` domina cualquier comparación simbólica —
  ninguna cifra de §4 debe leerse como apoyo o refutación de H₁.
- **Ninguna parte de este documento se ha sometido a revisión externa.**

---

## 7. Veredicto final, honesto

**Redesignar la simetría de la bola unidad para que case con base 12 sí
compra algo real y verificado — la constante de perímetro/diámetro del
hexágono no solo es racional (3, por Gołąb) sino robusta bajo rotación
arbitraria (§3), lo que la hace estrictamente más sólida que el "4" de L₁,
que colapsaba a 2 con solo girar 45°. Eso es un resultado genuino y el mejor
de este documento.**

**Pero extiende, no revierte, el patrón de toda la serie.** El área del
propio hexágono (§1.5) es irracional; el volumen del cuerpo 3D natural que
extiende su simetría (§2.4) es irracional ya antes de llegar a la
superficie; y el dodecágono —el candidato con el nombre más "12" posible—
resulta ser el **peor** de los cuatro en perímetro/diámetro y solo gana en
área por una razón (el teorema de Niven) que no tiene relación alguna con
base 12, con L₁, ni con ningún rediseño de simetría: es un hecho sobre
`sin(2π/n)` cierto en cualquier geometría y cualquier base numérica.

**Esto confirma, en una forma más fuerte que la que P1 registró
originalmente, la tesis que este documento se propuso probar honestamente:
no se encontró ningún cuerpo, entre los examinados, cuyas constantes de
perímetro/diámetro, área y superficie 3D sean simultáneamente racionales.**
Cada candidato gana en una y pierde en otra, sin excepción: el rombo de L₁
gana en las dos 2D y no tiene superficie 3D propia que perseguir con este
método (ya es el objeto que `01` estudió); el hexágono gana en
perímetro/diámetro (y de forma robusta) y pierde en área y en volumen 3D; el
dodecágono gana en área y pierde en perímetro/diámetro; el octógono pierde en
las dos. **No hay almuerzo gratis, y tampoco lo hay al elegir qué simetría
mimetiza la bola unidad** — la conjunción de racionalidades simultáneas que
haría de una de estas normas un reemplazo estructural de L₂ no aparece en
ningún punto del espacio de búsqueda examinado, y el mecanismo de fondo
(Niven para las funciones trigonométricas de ángulos racionales de `π`,
Gołąb para la extremal de perímetro/diámetro, la herencia directa de `√3` o
`√2` del propio polígono para el área) es tan variado que no hay una única
causa que "arreglar": son al menos tres obstrucciones estructurales
independientes, cada una atada a una constante distinta, exactamente el
mismo tipo de hallazgo que `99-sintesis.md` §4.6 encontró para los
mecanismos detrás del fracaso de la superficie del octaedro.

**No hay excepción que reportar aquí. La búsqueda de un cuerpo cuya
racionalidad "cierre el círculo" en las tres constantes a la vez fracasa de
nuevo, y esta vez con un resultado positivo genuino (§3) mezclado dentro del
fracaso — que es, si acaso, la lectura más honesta posible: rediseñar la
simetría mueve qué constante concreta se salva, no si el conjunto se salva.**

### 7.1 Enmienda de la segunda pasada

La segunda pasada cerró las tres preguntas que la primera dejó abiertas y
reverificó las cifras publicadas. **El veredicto de arriba no se revierte: se
refuerza en su conclusión y se corrige en una de sus explicaciones.** En
detalle, y sin suavizar:

1. **§1.8 lo refuerza y lo convierte en exhaustivo.** Donde el veredicto decía
   «entre los examinados», ahora hay una demostración: **no existe ningún
   n-ágono regular con razón perímetro/diámetro racional fuera de `n=4` (→4) y
   `n=6` (→3)**, y en particular ninguno con valor racional dentro de `(3,4)`.
   El espacio de búsqueda de esta constante queda **agotado**, no muestreado —
   algo que ningún documento anterior de la serie había conseguido para
   ninguna de sus constantes. El contraste con `01` §4.4 es el hallazgo
   colateral: la familia `Lp` regala infinitas razones racionales porque su
   parámetro es continuo y sus valores no son algebraicos; la familia de
   n-ágonos no regala ninguna porque su parámetro es discreto y sus valores
   viven en cuerpos ciclotómicos de grado creciente, donde ser racional es
   una condición que Niven caracteriza por completo. **La racionalidad barata
   de `Lp` no significaba nada; la racionalidad rígida del hexágono sí
   significa algo — y aun así no basta, porque solo toca una de las tres
   constantes.**
2. **§2.6 lo refuerza cerrando la última escapatoria del prisma.** Busemann
   (`5π`) y Holmes–Thompson (`42/π`) no rescatan la superficie: la empeoran,
   sustituyendo un irracional algebraico por dos trascendentes, por el mismo
   mecanismo (`ω_k = π`) que `03` y `04` habían identificado. La conjetura por
   analogía de la primera pasada era correcta y ahora está verificada.
3. **§3.7 lo corrige.** La afirmación de §3.5/§7 de que la robustez del «3»
   es «la misma coincidencia algebraica única de §1.1» es **falsa**. La
   robustez es una paridad (`n ≡ 2 mod 4`) que el hexágono comparte con
   `10, 14, 18, …`, y la condición de magnitud que se creía esencial se
   cancela. El hexágono conserva el mérito, pero enunciado correctamente:
   **es el único n-ágono regular simultáneamente robusto y racional.** De paso,
   la corrección baja aún más la retórica del «12»: el dodecágono alcanza el
   valor limpio 3, pero solo en una orientación — es decir, exactamente el
   mismo tipo de artefacto de alineación que `01` §7.3 le reprochaba al «4» de
   L₁, y por exactamente la misma fórmula (`(n/2)sin(2π/n)`, racional solo en
   `n=4` y `n=12` por Niven).
4. **§5.7 encontró tres discrepancias de cifra o de comando**, todas
   corregidas y ninguna capaz de mover un veredicto: un comando equivocado
   junto a una cifra correcta (§1.5), una superficie de Hausdorff internamente
   inconsistente (`3√3+6` debía ser `3√3+12`, §2.3) y una tabla auxiliar con
   ruido mal explicado (§3.2). Se reportan porque el estándar de esta serie es
   que un error publicado quede visible, no porque cambien nada.

**Lectura final, con la enmienda incorporada.** El rediseño de la simetría de
la bola unidad produce **un** resultado genuinamente bueno —el «3» del
hexágono, racional y robusto, único en su clase por un teorema aritmético— y
**ninguna** conjunción. Y la segunda pasada añade la pieza que faltaba para
que eso sea una afirmación fuerte en vez de una impresión: **no hay un tercer
candidato esperando.** La familia de n-ágonos regulares está completamente
clasificada, y el único cuerpo que sobrevive a los dos filtros sigue perdiendo
en área, en volumen y en las tres superficies 3D. El «no hay almuerzo gratis»
deja de ser una observación sobre cuatro ejemplos y pasa a ser un enunciado
sobre la familia entera.

---

## 8. Comandos de referencia (bitácora completa)

Todos ejecutados contra `target/release/dz-cli.exe` desde la raíz del
repositorio. Se listan en el orden en que aparecen en el documento.

```
eval "6" --mode rational
eval "2" --mode rational
eval "6/2" --mode rational
eval "8/2" --mode rational
eval "3*sqrt(3)/2" --mode rational
eval "3*sqrt(3)/2" --mode fixed --scale 8
eval "3*sin(2*pi/6)" --mode fixed --scale 8 --input-base 10    # CORREGIDO en 2ª pasada (§5.7): era "2*pi/12", que da ~1;6
eval "2*sqrt(2)-2" --mode rational
eval "2*sqrt(2)-2" --mode fixed --scale 8
eval "16*sqrt(2)-16" --mode rational --input-base 10
eval "8*sqrt(2)-8" --mode rational
eval "8*sqrt(2)-8" --mode fixed --scale 8
eval "4-2*sqrt(3)" --mode rational --input-base 10
eval "4-2*sqrt(3)" --mode fixed --scale 8 --input-base 10
eval "48-24*sqrt(3)" --mode rational --input-base 10
eval "24-12*sqrt(3)" --mode rational --input-base 10
eval "24-12*sqrt(3)" --mode fixed --scale 8 --input-base 10
eval "6*sin(2*pi/12)" --mode fixed --scale 8 --input-base 10
eval "6*sin(2*pi/12)" --mode rational --input-base 10
eval "3" --mode rational
eval "1.5*sin(2*pi/3)" --mode fixed --scale 8 --input-base 10
eval "2*sin(2*pi/4)" --mode rational --input-base 10
eval "2.5*sin(2*pi/5)" --mode fixed --scale 8 --input-base 10
eval "3*sin(2*pi/6)" --mode fixed --scale 8 --input-base 10
eval "3.5*sin(2*pi/7)" --mode fixed --scale 8 --input-base 10
eval "4*sin(2*pi/8)" --mode fixed --scale 8 --input-base 10
eval "4.5*sin(2*pi/9)" --mode fixed --scale 8 --input-base 10
eval "5*sin(2*pi/10)" --mode fixed --scale 8 --input-base 10
eval "5.5*sin(2*pi/11)" --mode fixed --scale 8 --input-base 10
eval "2/sqrt(3)" --mode rational
eval "2/sqrt(3)" --mode fixed --scale 8
eval "sqrt(2)" --mode rational
eval "4*sqrt(2)" --mode rational
eval "4*sqrt(2)/(2*sqrt(2))" --mode rational
eval "5*sqrt(3)" --mode rational
eval "5*sqrt(3)" --mode fixed --scale 8
eval "2*pi/12" --mode fixed --scale 8 --input-base 10
eval "pi/6" --mode fixed --scale 8 --input-base 10
eval "sin(2*pi/12)" --mode fixed --scale 8 --input-base 10
eval "6" --mode fixed --scale 8 --gates
eval "8" --mode fixed --scale 8 --gates
eval "3*sqrt(3)/2" --mode fixed --scale 8 --gates
eval "2" --mode fixed --scale 8 --gates
eval "24-12*sqrt(3)" --mode fixed --scale 8 --gates --input-base 10
eval "3" --mode fixed --scale 8 --gates --input-base 10
```

### 8.1 Comandos añadidos en la segunda pasada (§1.8, §2.6, §3.7, §5.7)

```
# §1.8.2 — la forma cerrada general reproduce los casos de §1.6
eval "4*tan(pi/4)"   --mode fixed --scale 8 --input-base 10
eval "6*sin(pi/6)"   --mode fixed --scale 8 --input-base 10
eval "8*tan(pi/8)"   --mode fixed --scale 8 --input-base 10
eval "12*tan(pi/12)" --mode fixed --scale 8 --input-base 10
eval "12*tan(pi/12)" --mode rational   --input-base 10

# §1.8.3 — las dos sucesiones monótonas que encierran a pi
eval "pi"            --mode fixed --scale 8 --input-base 10
eval "10*sin(pi/10)" --mode fixed --scale 8 --input-base 10
eval "14*sin(pi/14)" --mode fixed --scale 8 --input-base 10
eval "18*sin(pi/18)" --mode fixed --scale 8 --input-base 10
eval "22*sin(pi/22)" --mode fixed --scale 8 --input-base 10
eval "16*tan(pi/16)" --mode fixed --scale 8 --input-base 10
eval "20*tan(pi/20)" --mode fixed --scale 8 --input-base 10
eval "(5*sqrt(5)-5)/2" --mode fixed --scale 8 --input-base 10
eval "10*sin(pi/10)" --mode rational   --input-base 10

# §2.6 — Busemann y Holmes-Thompson sobre el prisma hexagonal
eval "5*pi"          --mode fixed --scale 8 --input-base 10
eval "5*pi"          --mode rational   --input-base 10
eval "42/pi"         --mode fixed --scale 8 --input-base 10
eval "42/pi"         --mode rational   --input-base 10
eval "2*sqrt(3)"     --mode fixed --scale 8
eval "(3*sqrt(3)/2)*(2/sqrt(3))^2" --mode fixed --scale 8
eval "2*pi*sqrt(3)/9" --mode fixed --scale 8
eval "3*sqrt(3)+12"  --mode rational   --input-base 10
eval "3*sqrt(3)+12"  --mode fixed --scale 8 --input-base 10
eval "1/5"           --mode rational

# §3.7.2 — el experimento de rotacion para n = 4, 6, 8, 10, 12
eval "4*sin(pi/4)*cos(0)/cos(pi/4)"                        --mode fixed --scale 8 --input-base 10
eval "4*sin(pi/4)*cos(45*pi/180)/cos(0)"                   --mode fixed --scale 8 --input-base 10
eval "12*tan(pi/6)*cos(11*pi/180)/(2*cos(11*pi/180)/cos(pi/6))"  --mode fixed --scale 8 --input-base 10
eval "8*sin(pi/8)*cos(0)/cos(pi/8)"                        --mode fixed --scale 8 --input-base 10
eval "8*sin(pi/8)*cos(10*pi/180)/cos(12.5*pi/180)"         --mode fixed --scale 8 --input-base 10
eval "8*sin(pi/8)*cos(20*pi/180)/cos(2.5*pi/180)"          --mode fixed --scale 8 --input-base 10
eval "8*sin(pi/8)*cos(22.5*pi/180)/cos(0)"                 --mode fixed --scale 8 --input-base 10
eval "2*sqrt(2)"                                           --mode fixed --scale 8
eval "20*tan(pi/10)*cos(2*pi/180)/(2*cos(2*pi/180)/cos(pi/10))"   --mode fixed --scale 8 --input-base 10
eval "20*tan(pi/10)*cos(17*pi/180)/(2*cos(17*pi/180)/cos(pi/10))" --mode fixed --scale 8 --input-base 10
eval "12*sin(pi/12)*cos(0)/cos(pi/12)"                     --mode fixed --scale 8 --input-base 10
eval "12*sin(pi/12)*cos(7.5*pi/180)/cos(7.5*pi/180)"       --mode fixed --scale 8 --input-base 10
eval "12*sin(pi/12)*cos(15*pi/180)/cos(0)"                 --mode fixed --scale 8 --input-base 10
eval "12*sin(pi/12)*cos(15*pi/180)/cos(0)"                 --mode rational   --input-base 10

# §4 — filas nuevas de coste
eval "3*sqrt(3)+12"  --mode fixed --scale 8 --gates --input-base 10
eval "5*pi"          --mode fixed --scale 8 --gates --input-base 10
eval "42/pi"         --mode fixed --scale 8 --gates --input-base 10
eval "10*sin(pi/10)" --mode fixed --scale 8 --gates --input-base 10

# §5.7 — reverificacion; el comando que destapo la discrepancia 1 y el exacto de §3.2
eval "3*sin(2*pi/12)" --mode fixed --scale 8 --input-base 10   # devuelve ~1;6, NO ~2;721586
eval "cos(20*pi/180)+sin(20*pi/180)/sqrt(3)" --mode fixed --scale 8 --input-base 10
```

Todas las salidas están reproducidas junto a su cifra en el cuerpo del
documento; esta bitácora es solo un índice para volver a ejecutar la serie
completa sin tener que buscar comando por comando.
