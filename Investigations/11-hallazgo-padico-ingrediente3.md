# Hallazgo — Ingrediente 3: cambiar el sistema numérico (geometría p-ádica / ultramétrica)

Tercero de la lista de "ingredientes" candidatos a producir una geometría que
no necesite π, más allá de sustituir la norma sobre ℝⁿ (`03`–`05`, `07`, `08`,
agotado), la topología (`09`, negativo) o el axioma de aditividad de la
medida (`10`, positivo parcial con polaridad invertida). Ingrediente 3: no
tocar ni la métrica sobre ℝⁿ ni χ ni la aditividad, sino cambiar el propio
**sistema numérico métrico** — pasar de ℝ (con su valor absoluto arquimediano
`|·|_∞`) a ℚ_p, el cuerpo de los números **p-ádicos**, con el valor absoluto
`|·|_p` y la métrica **ultramétrica** `d(x,y) = max(d(x,z), d(z,y))` en vez
de la desigualdad triangular ordinaria.

Fecha de apertura: 2026-09-14. Protocolo de `00-plan-investigacion.md`: toda
cifra dozenal sale de `target/release/dz-cli.exe` con el comando al lado;
todo hallazgo se clasifica en casilla (I)/(II)/(III) o, si genuinamente no
encaja, en una cuarta categoría explicada; ningún resultado se elige por dar
el número bonito.

**Recordatorio no negociable** (el mismo de `05`, `09`, `10`): π es
trascendente (Lindemann–Weierstrass), independiente de la base numeral. Nada
de lo que sigue sugiere que π sea racional o que "esté mal". Se investiga si
existe, en un sistema numérico genuinamente distinto de ℝ, algún objeto
razonable llamado "círculo" o "tamaño" cuya descripción no necesite π — y, si
no lo hay, **por qué no lo hay**, con el mismo rigor que exigió negar el
ingrediente 1.

---

## 0. Resumen del veredicto (para quien no lea el resto)

**El resultado es de una categoría distinta a `09` y `10`, tal como anticipaba
el encargo, y hay que decirlo sin forzarlo a encajar.** No es "sí evita π" ni
"no evita π" en el sentido en que `10` respondió que sí (con precio) y `09`
que no. Es: **la pregunta "¿cuánto vale π aquí?" no tiene referente en ℚ_p**,
porque el objeto que genera π —una curva continua rectificable, o
equivalentemente un grupo de rotaciones continuo `SO(2)` actuando con una
métrica de arco compatible— no existe en un espacio totalmente disconexo.
No es que π "desaparezca de una fórmula que antes lo tenía" (eso sería
positivo, como `10`) ni que "reaparezca disfrazado" (eso sería negativo, como
`09`). Es que no hay fórmula que plantear.

Lo que **sí** existe, y es matemática real, verificada y sin ningún π:

- la medida de Haar de ℤ_p, exacta y racional en toda bola (§3);
- el número de subbolas de un nivel más fino dentro de una bola, que es
  exactamente el primo `p` — un entero, combinatorio, sin análogo de
  curvatura (§4);
- la dimensión de Hausdorff de ℚ_p bajo su propia métrica, que sale
  exactamente **1** — un entero limpio, pese a que el espacio es totalmente
  disconexo (derivado aquí, no solo citado; §4.3).

Y sobre la conexión con base 12 (punto 4 del encargo): existe el anillo de
los enteros **12-ádicos** `ℤ_12 ≅ ℤ_2 × ℤ_3` (vía el mismo teorema chino del
resto que usa `crates/dz-gates/src/backends/crt.rs`, verificado contra el
binario), pero **no hay ninguna ventaja aritmética real de `ℚ_2`/`ℚ_3` sobre
`ℚ_5` más allá de la terminación de fracciones ya conocida hasta el
cansancio** (`99-sintesis.md` §4.1–§4.2). El teorema de Ostrowski para ℚ
—el mismo tipo de resultado que ya cerró la puerta a "elegir L₁ para ℂ" en
`99-sintesis.md` §3.1— dice explícitamente que **ningún primo está
privilegiado**: toda completación no arquimediana de ℚ es un `ℚ_p` para algún
primo `p`, y no hay criterio interno a la teoría que prefiera 2 y 3 sobre 5.
La dimensión de Hausdorff de la §4.3 lo confirma numéricamente: da 1 para
cualquier `p`, sin distinguir a 2, 3 ni 5.

---

## 1. Marco mínimo: ℚ_p y la métrica ultramétrica

Para `x ∈ ℚ*` no nulo y un primo `p` fijo, escribir `x = p^v · (a/b)` con
`a, b` enteros no divisibles por `p` y `v ∈ ℤ` (la **valuación p-ádica**
`v = v_p(x)`). El valor absoluto p-ádico es

```
|x|_p = p^{-v_p(x)},      |0|_p = 0.
```

`ℚ_p` es la completación de ℚ respecto a la métrica `d(x,y) = |x-y|_p`, igual
que ℝ es la completación respecto a `|·|_∞`. Son las **únicas** completaciones
no equivalentes de ℚ (teorema de Ostrowski, 1916): todo valor absoluto no
trivial sobre ℚ es equivalente a `|·|_∞` o a `|·|_p` para exactamente un
primo `p`. Este es el teorema que cierra el punto 4 del encargo; se usa en
detalle en §6.

La propiedad que hace a `|·|_p` cualitativamente distinta de `|·|_∞` es la
desigualdad **ultramétrica** (fuerte):

```
|x + y|_p ≤ max(|x|_p, |y|_p),     con igualdad si |x|_p ≠ |y|_p.
```

Verificación elemental: si `v_p(x) < v_p(y)`, entonces `x+y = p^{v_p(x)}(u +
p^{v_p(y)-v_p(x)}\cdot(\dots))` con el término entre paréntesis divisible por
`p`, así que `v_p(x+y) = v_p(x)` exactamente, luego `|x+y|_p = |x|_p =
\max(|x|_p,|y|_p)`. Es la propiedad que traduce en tres hechos geométricos
sin análogo euclidiano, todos estándar y verificables a mano a partir de la
desigualdad de arriba (se derivan aquí, no se citan sin más, porque son la
base de todo lo que sigue):

1. **Todo punto de una bola es su centro.** Si `b ∈ B(a,r) = {x: |x-a|_p ≤
   r}`, entonces para cualquier `x ∈ B(a,r)`: `|x-b|_p ≤ max(|x-a|_p,
   |a-b|_p) ≤ r`, así que `B(a,r) = B(b,r)`.
2. **Dos bolas son disjuntas o una contiene a la otra.** Si `B(a,r) ∩
   B(a',r') ≠ ∅` con `r ≤ r'` y `c` es un punto común, por (1) `B(a,r) =
   B(c,r) ⊆ B(c,r') = B(a',r')`.
3. **Las bolas son abiertas y cerradas a la vez (clopen).** `B(a,r)` es
   cerrada (preimagen de un cerrado por una función continua). Es abierta
   porque, por (1), cada uno de sus puntos tiene una bola del mismo radio
   enteramente contenida en ella.

De (2) y (3) sale que `ℚ_p` es **totalmente disconexo**: las bolas forman una
base de la topología y no hay ningún subconjunto conexo con más de un punto.
No hay curvas continuas no constantes, y por tanto no hay nada que
"rectificar" — el ingrediente que se necesita para que exista una noción de
longitud de arco simplemente no está en el espacio. Esto es la raíz de todo
lo que sigue, y conviene dejarlo dicho antes de entrar en las preguntas
concretas.

---

## 2. Pregunta 1: ¿hay un análogo de "círculo" o "esfera" en ℚ_p?

Sí existe el objeto — la **esfera** `S(a,r) = {x : |x-a|_p = r}` está bien
definida para cualquier `r` en el grupo de valores `{p^n : n∈ℤ} ∪ {0}` — pero
su estructura es combinatoria, no geométrica en el sentido de "curva
continua".

Fijando `r = p^{-k}` y usando (2)-(3) de §1: la bola `B(a,p^{-k})` se
descompone en exactamente `p` bolas disjuntas de radio `p^{-(k+1)}`,
correspondientes a los `p` residuos posibles módulo `p^{k+1}` que extienden
un residuo fijo módulo `p^k` (esto es `[ℤ_p : p^{k+1}ℤ_p] / [ℤ_p : p^kℤ_p] =
p^{k+1}/p^k = p`, el índice del cuerpo residual `𝔽_p = ℤ_p/pℤ_p`, que tiene
exactamente `p` elementos — hecho algebraico estándar, no citado sin
verificar: `ℤ_p/pℤ_p ≅ ℤ/pℤ` porque todo entero p-ádico es congruente módulo
`p` a exactamente uno de `0,1,...,p-1`).

La esfera `S(a,p^{-k}) = B(a,p^{-k}) \ B(a,p^{-(k+1)})` es entonces la unión
de **`p-1`** de esas `p` subbolas (todas menos la que contiene al propio
centro `a`, que es la que se resta). Es decir: la "esfera" no es una curva
1-dimensional que rodea al centro, es un conjunto de `p-1` bolas más
pequeñas, cada una a su vez subdivisible en `p` subbolas más finas, y así
indefinidamente — un árbol `p`-ario con raíz en `ℤ_p` (la imagen usual es el
**árbol de Bruhat–Tits** de `ℚ_p`, o, más concretamente para el anillo de
enteros, el árbol de expansión en base `p` dígito a dígito).

**No hay ningún análogo de "perímetro" o "diámetro" con sentido**, porque no
hay longitud de arco: la esfera no es una curva rectificable, es un
conjunto de Cantor (compacto, totalmente disconexo, sin puntos aislados)
cuando se mira "desde fuera" con la topología subespacio, y su única medida
natural es la que hereda como subconjunto medible de `ℚ_p` (§3), que es una
medida de **volumen**, no de longitud. La pregunta "razón entre el perímetro
de la esfera y su radio" no tiene un numerador que definir. Esto ya adelanta
el veredicto de §4: no es que la razón salga racional en vez de π, es que la
razón no existe como objeto matemático bien planteado.

---

## 3. Pregunta 2: la medida de Haar p-ádica — cálculo explícito y honestidad sobre su relevancia

`ℚ_p` es un grupo topológico localmente compacto (bajo la suma), así que
tiene una medida de Haar, única salvo escala. Se normaliza `μ(ℤ_p) = 1`.

**Cálculo de `μ(p^kℤ_p)`, derivado aquí, no solo citado.** `ℤ_p` es unión
disjunta de las `p` clases `0+pℤ_p, 1+pℤ_p, \dots, (p-1)+pℤ_p` (cada entero
p-ádico tiene un único residuo módulo `p`). Por aditividad e invariancia por
traslación, cada clase tiene la misma medida, luego `μ(pℤ_p) = μ(ℤ_p)/p =
1/p`. Iterando el mismo argumento sobre `pℤ_p` en vez de `ℤ_p`:

```
μ(p^k ℤ_p) = p^{-k},   para todo k ≥ 0 entero,
```

y por traslación, `μ(a + p^kℤ_p) = p^{-k}` para cualquier bola de ese radio,
sea cual sea su centro `a`. Esto es exacto, aditivo, sin ningún `π` en
ninguna parte del argumento — es la construcción **más** limpia de "medida de
tamaño sin π" que ha aparecido en toda la serie, y no necesita ningún
teorema profundo: sale de contar residuos.

**Medida de la esfera**, usando la descomposición de §2:

```
μ(S(a,p^{-k})) = μ(B(a,p^{-k})) − μ(B(a,p^{-(k+1)})) = p^{-k} − p^{-(k+1)}
               = p^{-(k+1)}(p−1)
```

Para `k=0` (la esfera unidad, que coincide con el grupo de **unidades**
`ℤ_p^* = ℤ_p \ pℤ_p`):

```
μ(ℤ_p^*) = 1 − 1/p
```

Verificado con el binario para los tres primos de interés del punto 4 del
encargo, `p=2,3,5` (aritmética racional pura, sin nada p-ádico en el
cálculo — es sólo `1 - 1/p` evaluado con exactitud):

```
$ target/release/dz-cli.exe eval "1-1/2" --mode rational
1/2
$ target/release/dz-cli.exe eval "1-1/3" --mode rational
2/3
$ target/release/dz-cli.exe eval "1-1/5" --mode rational
4/5
```

**¿Es esto relevante a la pregunta del encargo, o es trivial?** Hay que ser
honesto en los dos sentidos:

- **Es genuinamente relevante como respuesta al ingrediente 2 revisitado**:
  es una medida real, aditiva (a diferencia de la capacidad logarítmica de
  `10`), sin π, bien definida, sobre un objeto ("bola p-ádica") que sí es un
  análogo legítimo de "disco". En ese sentido cumple lo que pedía la
  pregunta 2 del encargo.
- **No es relevante como respuesta a "evitar π del círculo"**, y no por
  timidez retórica sino porque no hay comparación posible: no existe una
  "razón perímetro/diámetro" p-ádica con la que contrastar `μ(bola)`. La
  medida de Haar mide un objeto de naturaleza distinta (un conjunto abierto,
  con "volumen") del que produce π en ℝ² (una curva, con "longitud"). Repetir
  aquí la comparación de `10` §2.3 (`cap(elipse) = (a+b)/2`, comparable
  al perímetro euclidiano porque ambos son funcionales sobre la *misma*
  familia de curvas) no tiene sentido en ℚ_p porque no hay familia de curvas
  con la que comparar. **Es una geometría sin π, pero no es una respuesta a
  la pregunta "¿qué reemplaza a π?" — es una confirmación de que aquí no hay
  nada que reemplazar.**

---

## 4. Pregunta 3: ¿hay un análogo de "área = πr²"? — por qué la ausencia de π es estructural

### 4.1 El conteo combinatorio, verificado

El número de bolas de radio `p^{-(k+1)}` contenidas en una bola de radio
`p^{-k}` es, por la construcción de §2, **exactamente `p`** — un entero,
sin aproximación, para cualquier `k`. Es la cantidad combinatoria más
cercana a "cuántas partes finas hacen falta para cubrir un todo más grueso",
el papel que en ℝ² cumple `(R/r)^2` (el factor de escala al cuadrado, de
donde sale el `r²` de `πr²`). Aquí el factor de escala es lineal en el
exponente: pasar de radio `p^{-k}` a `p^{-(k+1)}` multiplica el número de
bolas por `p^1`, no por `p^2`. **No hay "cuadrado" en absoluto.**

### 4.2 Por qué no hay cuadrado: no hay dimensión 2 con rotación

En ℝ², `πr²` nace de que el disco es la imagen de un cuadrado `r×r` bajo una
familia continua de rotaciones (o, equivalentemente, de que `x²+y²` es una
forma cuadrática **isótropa y con grupo de automorfismos continuo**, `SO(2)`,
que actúa transitivamente sobre cada círculo). Nada de eso tiene análogo
automático en `ℚ_p`: el espacio `ℚ_p` es 1-dimensional como cuerpo — no hay
"`ℚ_p²`" privilegiado como el escenario de un círculo salvo que se lo
construya aparte, y aun construyéndolo (`ℚ_p²` con la norma `max(|x|_p,
|y|_p)`), la "bola" resultante es el producto cartesiano `B(0,r) × B(0,r)`,
de medida `μ_2(B_2) = μ(B(0,r))^2 = r^2` — es decir, se comporta como la bola
`L∞` en el caso arquimediano (donde el "área" también es `(2r)^2`, sin π,
porque `L∞` tampoco tiene rotación continua). **Esto no es una casualidad
nueva: es el mismo mecanismo que `99-sintesis.md` §3.1 y `10` §4.4 ya
identificaron** — π aparece solo cuando hay una estructura cuadrática
isótropa con grupo de simetría continuo, y ni `L∞` en ℝⁿ ni ningún producto
de bolas `ℚ_p` la tienen.

(Existe la pregunta más fina de si una forma cuadrática genuina `x²+y²` sobre
`ℚ_p` —no el producto de normas, sino la propia ecuación diofántica— define
algo parecido a un "círculo". Es un objeto real de teoría de números (si
`x²+y² = 1` tiene soluciones en `ℚ_p` depende de si `-1` es un cuadrado en el
cuerpo residual `𝔽_p`, lo que a su vez depende de `p mod 4`, vía el símbolo
de Hilbert y la teoría de formas cuadráticas p-ádicas de Hasse–Minkowski). Se
señala como una vía genuina y no trivial, pero **no se persiguió a fondo
aquí** — no produce una "longitud" o "área" con la que comparar π, produce
una pregunta de existencia de soluciones (sí/no), que es un objeto de otra
naturaleza. Está en §8 como línea abierta.)

### 4.3 Dimensión de Hausdorff de ℚ_p — derivada, no solo citada, y el hallazgo limpio de esta sección

Aquí sí hay algo que calcular con rigor propio, en vez de repetir "no hay
área". `ℤ_p` es totalmente disconexo — su dimensión **topológica** (de
recubrimiento) es 0 — pero como espacio métrico tiene una dimensión de
**Hausdorff** perfectamente calculable, y **no es 0**.

Cubrir `ℤ_p` con bolas de radio `r = p^{-k}` requiere exactamente `N(r) =
p^k` bolas (una por cada clase módulo `p^k`, ningún solapamiento posible por
la propiedad (2) de §1, y no hace falta ninguna de menos porque las `p^k`
clases son disjuntas y cubren todo). Para exponente de escala `s`, la suma de
Hausdorff de ese recubrimiento óptimo es:

```
N(r) · r^s = p^k · (p^{-k})^s = p^{k(1-s)}
```

que tiende a `0` cuando `k→∞` si `s>1`, a `∞` si `s<1`, y es **constante e
igual a 1** exactamente en `s=1` (para todo `k`, `p^{k(1-1)} = p^0 = 1`). Por
la definición de dimensión de Hausdorff (el punto de salto entre medida 0 e
infinita), esto da

```
dim_H(ℚ_p) = 1,   exacto, para todo primo p.
```

y además, en el punto crítico `s=1`, la medida de Hausdorff `H^1` coincide
exactamente con la medida de Haar `μ` de §3 (mismo valor, `p^{-k}`, en el
mismo recubrimiento óptimo) — un hecho de consistencia interna que confirma
que ambas construcciones miden "lo mismo" de dos formas independientes.

**Esto es el resultado más limpio de la sección, y merece el mismo
tratamiento que el `1/4-1/2+...` de `09` o el `cap=r` de `10`: un número
entero, exacto, sin `~`, que responde honestamente a "¿cuánto de
'geométrico' queda cuando no hay curvas?"** — la respuesta es que `ℚ_p` sigue
teniendo una noción de dimensión, y esa dimensión es **1**, un entero limpio
sin depender del primo `p` elegido (2, 3 y 5 dan exactamente lo mismo: la
`p` se cancela algebraicamente en `log(p)/log(p)`). Esto contrasta con
fractales genuinos como el conjunto de Cantor (dimensión `log2/log3`,
irracional) — `ℚ_p` es "fractal" en el sentido de ser totalmente disconexo,
pero su dimensión de Hausdorff es un entero, no un exponente feo. Es un dato
en contra, no a favor, de leer `ℚ_p` como "más rico" o "más multidimensional"
que ℝ, en el sentido vago que sugiere la fuente New Age que motivó esta
serie: como espacio métrico, `ℚ_p` es 1-dimensional, igual que ℝ.

### 4.4 Veredicto de la pregunta 3: pregunta mal planteada, no pregunta respondida

**No hay traducción de "área = πr²" a `ℚ_p`, y la razón es estructural, no
una limitación de búsqueda.** `πr²` requiere simultáneamente (a) una
dimensión 2 con una forma cuadrática isótropa, y (b) un grupo de simetría
continuo que actúe transitivamente sobre las direcciones — ninguna de las dos
cosas existe en `ℚ_p` sin construirlas por fuera del cuerpo (y al
construirlas, como en el producto de normas de §4.2, lo que sale es el
análogo `L∞`, no un círculo). Lo único genuinamente cuantitativo que produce
la estructura ultramétrica por sí sola es el conteo `p` de §4.1 (entero,
combinatorio, sin curvatura) y la dimensión de Hausdorff `1` de §4.3 (entero,
limpio, independiente de `p`). Ninguno de los dos es "π reemplazado por un
racional" en el sentido en que `10` lo fue para la capacidad del disco: son
cantidades de **otra familia**, que no compiten con π porque no responden a
la misma pregunta que π responde.

Esto es exactamente la cuarta categoría que el encargo anticipaba como
resultado honesto posible: **la pregunta original no se traduce, y no porque
falte encontrar la traducción correcta, sino porque el objeto que la
plantea en ℝ² (rotación continua, `SO(2)`) no tiene análogo en un espacio
totalmente disconexo.** Forzar una clasificación en (I)/(II)/(III) del plan
original sería inexacto: no es "(I) intrínseco a L₂" en el sentido de `10`
§4.4 (ahí la pregunta sí estaba bien planteada — "capacidad de la bola L₁" —
y la respuesta era que dependía de estructura L₂; aquí la pregunta
"circunferencia/diámetro de la esfera p-ádica" ni siquiera está bien
planteada, porque no hay perímetro que definir). Se declara explícitamente
una clasificación aparte: **(IV) pregunta no trasladable** — el objeto de
comparación no existe en el sistema destino, así que no hay experimento que
hacer, solo una explicación de por qué no lo hay.

---

## 5. Pregunta 4: ¿conexión genuina con base 12?

### 5.1 El anillo 12-ádico existe, y es exactamente el patrón que ya usa este repositorio

`12` no es primo, así que no hay `ℚ_12` como cuerpo p-ádico (`|·|_12` no
satisface la ultramétrica de forma consistente con ser un valor absoluto de
un cuerpo, porque `12` no es primo — la valuación `v_12` no está bien
definida de forma multiplicativa: `v_12(2)·v_12(6) ≠ v_12(12)` de manera
consistente con un cuerpo, exactamente el tipo de fallo que predice
Ostrowski al exigir "primo"). Pero sí existe, con pleno sentido matemático,
el anillo de los **enteros 12-ádicos**, definido como el límite inverso

```
ℤ_12 := lim←_k  ℤ/12^k ℤ
```

(la misma construcción categórica que da `ℤ_p = lim← ℤ/p^kℤ`, aplicada a un
módulo compuesto). Como `12 = 4·3` con `gcd(4,3)=1`, el **teorema chino del
resto** da, para cada `k`, un isomorfismo de anillos

```
ℤ/12^k ℤ ≅ ℤ/4^k ℤ × ℤ/3^k ℤ
```

(exactamente el mismo teorema, aplicado nivel a nivel, que
`crates/dz-gates/src/backends/crt.rs` usa para un solo nivel, `k=1`: la
constante `RADIX=12` decompuesta en `(x mod 4, x mod 3)`, líneas 24-45 del
archivo — `decompose(x) = (x % 4, x % 3)` y su inversa `reconstruct` por
tabla). Tomando el límite inverso sobre `k`, y usando que `ℤ/4^kℤ = ℤ/2^{2k}ℤ`
es cofinal en la torre de potencias de 2 (todo `2^m` divide a algún `4^k`),
se obtiene

```
ℤ_12 ≅ ℤ_2 × ℤ_3
```

— el mismo tipo de resultado, en el límite de precisión infinita, que
`crt.rs` implementa a un solo nivel finito (`mod 12`) para la aritmética del
proyecto. **La analogía que pedía el encargo es exacta, no forzada**: el
backend CRT de este repositorio es, literalmente, la aritmética modular de
`ℤ/12ℤ ≅ ℤ/4ℤ × ℤ/3ℤ`, el primer peldaño finito de la misma torre cuyo
límite es `ℤ_12 ≅ ℤ_2 × ℤ_3`.

**Verificado con el binario, en el peldaño finito que sí puede ejercitar**:
comparar los tres backends sobre una multiplicación confirma que el backend
`crt` (que opera internamente vía `decompose`/`reconstruct`, es decir, vía el
isomorfismo `ℤ/12ℤ ≅ ℤ/4ℤ×ℤ/3ℤ`) da exactamente el mismo resultado numérico
que `mvl` y que el oráculo `table`, pagando un coste aparte
(`reconstruction_ops`) por tener que reconstruir el orden total cada vez que
lo necesita — la manifestación en puertas de que el producto de anillos
`ℤ/4×ℤ/3` no lleva un orden total gratis, aunque sí lleva la suma y el
producto gratis (componente a componente):

```
$ target/release/dz-cli.exe compare mul 25 34
result: 808
  backend     cycle      min      max      lit      thr      total    depth    recon
  mvl         16896    28156    20923    14376     3576      83927      441        0
  crt         16896    28156    20923    14376     3576      83927      441   118630
  ...
  verdict: mvl is cheapest. mvl and crt tie on all five primitive counters
  (83927) and on depth (441) ... The tie is broken by reconstructions:
  mvl 0, crt 118630.
```

Es el mismo hallazgo que `docs/02-crt-vs-mvl.md` y `99-sintesis.md` §1 ya
documentaban («CRT no compensa»), leído ahora como lo que estructuralmente
es: `ℤ/12ℤ` como anillo producto tiene suma y multiplicación baratas
(component-wise, "gratis") pero **no es un cuerpo** — tiene divisores de
cero (`ej. 4·3 ≡ 0 mod 12` con `4,3 ≠ 0`) y, por tanto, **no tiene orden
total compatible con la estructura de anillo sin reconstruir**. Esa
reconstrucción es exactamente el coste de 118.630 `recon` medido arriba. Es
la misma obstrucción, en otro lenguaje, de que `ℤ_12` (o su versión finita
`ℤ/12ℤ`) no es un dominio de integridad y por tanto no tiene cuerpo de
fracciones bien portado — no hay «`ℚ_12`» razonable, ni como cuerpo p-ádico
(12 no es primo) ni como localización del anillo (tiene divisores de cero).

### 5.2 ¿Alguna ventaja real de `p=2,3` sobre `p=5`, más allá de la terminación?

Se buscaron, con honestidad, ejes distintos al ya conocido:

- **Terminación de fracciones**: `1/2`, `1/3`, `1/4`, `1/6` terminan en
  dozenal; `1/5` no. Es **exactamente** el mismo hecho `12=2²·3` de la línea
  D (`99-sintesis.md` §4.1–§4.2), reformulado en el lenguaje de valuaciones
  p-ádicas (`x` tiene expansión dozenal finita si y solo si `v_5(x) ≥ 0`,
  es decir, si el denominador reducido de `x` no tiene factor 5 — y en base
  10, si `v_2(x)≥0` y `v_5(x)≥0`, mientras que en base 12 la condición es
  `v_5(x)≥0` sin restricción sobre `v_2` ni `v_3`). No es un hallazgo nuevo,
  es el mismo hecho leído con otro vocabulario:

  ```
  $ target/release/dz-cli.exe eval "1/2" --mode fixed --scale 8
  0;6
  $ target/release/dz-cli.exe eval "1/3" --mode fixed --scale 8
  0;4
  $ target/release/dz-cli.exe eval "1/4" --mode fixed --scale 8
  0;3
  $ target/release/dz-cli.exe eval "1/6" --mode fixed --scale 8
  0;2
  $ target/release/dz-cli.exe eval "1/9" --mode fixed --scale 8
  0;14
  $ target/release/dz-cli.exe eval "1/5" --mode fixed --scale 8
  0;249724972497249724972497 (periodica en base 12, valor truncado)
  ```

- **Tamaño del cuerpo residual y del grupo de unidades**: `|𝔽_2|=2`,
  `|𝔽_3|=3`, `|𝔽_5|=5`; `μ(ℤ_p^*) = 1-1/p` (§3) da `1/2`, `2/3`, `4/5`
  respectivamente. No hay ninguna propiedad de estos tres números que
  distinga «2 y 3 juntos» de «5 solo» — cada uno mide su propio primo, sin
  interacción entre ellos. No es una ventaja de `{2,3}` sobre `{5}`, es tres
  cálculos independientes.
- **Estructura del grupo de unidades**: para `p` impar, `ℤ_p^* ≅ μ_{p-1} ×
  (1+pℤ_p)`, con `1+pℤ_p ≅ (ℤ_p,+)` vía el logaritmo p-ádico (hecho
  estándar de la teoría, no verificado aquí a mano por ser álgebra
  conmutativa estándar y no aportar nada nuevo al argumento). El factor de
  torsión `μ_{p-1}` (raíces `(p-1)`-ésimas de la unidad, las "unidades de
  Teichmüller") es el análogo discreto más cercano a una "rotación": para
  `p=3`, `μ_2 = \{±1\}` (2 elementos); para `p=5`, `μ_4` (4 elementos); para
  `p=2` el caso es degenerado (`μ_1` trivial, hay que usar `1+4ℤ_2`). Ninguno
  de estos grupos finitos es un círculo continuo — son grupos cíclicos
  finitos, el análogo p-ádico más cercano posible a "un conjunto discreto de
  direcciones", y su tamaño (`p-1`) no favorece a 2 o 3 sobre 5 en ningún
  sentido salvo el trivial de que son números más pequeños.
- **Dimensión de Hausdorff (§4.3)**: da exactamente `1` para **cualquier**
  primo — 2, 3 y 5 empatan sin excepción, porque el primo se cancela
  algebraicamente en `log(p)/log(p)`. Es un resultado explícitamente
  **en contra** de cualquier ventaja de 2 o 3 sobre 5 en esta vía: no hay
  ninguna.

**El cierre teórico, y es el más fuerte disponible**: el teorema de
Ostrowski (§1) —el mismo tipo de resultado de clasificación que `10` §4.4 y
`99-sintesis.md` §3.1 ya usaron para cerrar la puerta a "elegir" L₁ sobre ℂ—
dice que **ℚ_2, ℚ_3 y ℚ_5 son las tres completaciones de ℚ respecto a sus
tres primos**, y no hay ningún cuarto criterio, interno a la teoría de
cuerpos locales, que privilegie a unos sobre otros. El **teorema del
producto** (`∏_p |x|_p · |x|_∞ = 1` para todo `x ∈ ℚ*`, consecuencia directa
de que la factorización en primos de `x` es única) trata a **todos** los
primos simétricamente: cada uno mide su propio factor, y el principio
local-global de Hasse–Minkowski para formas cuadráticas necesita **todos**
los `ℚ_p` simultáneamente, no un subconjunto privilegiado. No hay ningún
sentido matemático riguroso en el que "`ℚ_2` y `ℚ_3` sean mejores que `ℚ_5`"
— son completaciones distintas, no comparables, cada una necesaria para su
propia pregunta.

### 5.3 Veredicto de la pregunta 4

**La conexión con base 12 existe en un solo sitio, y es exactamente la
esperada, no una nueva.** `ℤ_12 ≅ ℤ_2 × ℤ_3` es matemáticamente real y tiene
un paralelo genuino y verificado con `crates/dz-gates/src/backends/crt.rs`
(mismo teorema, un nivel finito de la torre). Pero **no aporta ninguna
ventaja aritmética que no fuera ya conocida**: el único patrón dozenal que
aparece (terminación de fracciones con denominador `2^a·3^b`) es el mismo
`12=2²·3` de siempre, y **ninguna** de las cantidades genuinamente p-ádicas
investigadas (medida de Haar, cuerpo residual, grupo de unidades, dimensión
de Hausdorff) distingue a `{2,3}` de `{5}` en un sentido que no sea "miden
primos distintos". Ostrowski cierra la pregunta con una razón estructural,
no solo con la ausencia de un hallazgo: **no puede haber** tal ventaja,
porque el marco teórico trata a todos los primos por igual.

---

## 6. Veredicto global del ingrediente 3

| sub-pregunta | resultado |
|---|---|
| ¿hay un análogo de "círculo/esfera" en `ℚ_p`? | Existe el objeto (`S(a,r)`, unión de `p-1` subbolas) pero no tiene longitud ni perímetro — no es una curva, es un árbol combinatorio (§2) |
| ¿la medida de Haar es relevante como "geometría sin π"? | Sí como medida rigurosa y sin π (§3), **no** como respuesta a la pregunta del círculo — mide un objeto de otra familia (volumen, no perímetro) |
| ¿hay traducción de "área=πr²"? | No, y es estructural: faltan **las dos** condiciones que producen π (dimensión 2 con forma isótropa, y grupo de rotación continuo). El conteo `p` (§4.1) y la dimensión de Hausdorff `=1` (§4.3, derivada aquí) son las cantidades limpias que sí existen, y no compiten con π porque responden otra pregunta |
| ¿la pregunta "evitar π" está bien planteada en `ℚ_p`? | **No** — es el resultado central del documento. Se clasifica en una categoría aparte, **(IV) pregunta no trasladable**, explicada en §4.4 |
| ¿conexión genuina con base 12? | Sí en un punto exacto (`ℤ_12≅ℤ_2×ℤ_3`, paralelo real con `crt.rs`), pero sin ninguna ventaja aritmética nueva — Ostrowski explica por qué no puede haberla (§5) |

**Lectura de conjunto, comparada con `09` y `10`.** Los tres ingredientes de
la lista abierta por la reinterpretación de Kryon han dado, hasta ahora, tres
respuestas de tres formas distintas: `09` (topología/χ) dio **negativo
directo** — la pregunta estaba bien planteada y la respuesta fue que no, π
sigue apareciendo, solo cambia el múltiplo. `10` (medida no aditiva) dio
**positivo con matiz** — la pregunta estaba bien planteada, la respuesta fue
que sí existe una medida sin π, pero no es "la circunferencia" en el sentido
intuitivo y, contra L₁, se invierte la polaridad habitual de la serie. Este
documento (`11`, sistema numérico p-ádico) da algo genuinamente **distinto de
ambos**: la pregunta deja de estar bien planteada. No es una tercera
variación sobre "sí" o "no" — es la constatación, con matemática real
detrás (ultramétrica, árbol de bolas, teorema de Ostrowski), de que el marco
conceptual en el que "evitar π" tiene sentido (curvas rectificables, grupos
de rotación continuos) es exactamente lo que un espacio totalmente disconexo
no tiene. Esto no es una forma más débil de responder la pregunta: es la
respuesta correcta a una pregunta que, formulada así, presupone estructura
que `ℚ_p` no posee — el mismo tipo de precisión que `10` §4.4 ya aplicó a
"capacidad L₁" (la pregunta presupone L₂ para poder plantearse), llevado un
paso más allá: aquí ni siquiera hay un candidato a "capacidad p-ádica" que
proponer, porque no hay estructura conforme ni holomorfa sobre la que
definirlo.

---

## 7. Lo que no funcionó (obligatorio, no vacío)

- **La búsqueda de un análogo directo de "perímetro/diámetro" en `ℚ_p` no
  produjo nada** — no por falta de esfuerzo, sino porque el objeto (curva
  rectificable) no existe. Se intentó forzar la pregunta usando la esfera
  `S(a,r)` como si fuera una curva y preguntando por su "longitud" bajo
  alguna medida natural, y no hay ninguna: la única medida natural sobre
  `S(a,r)` es la restricción de la medida de Haar (un volumen, calculado en
  §3), no una longitud de arco. Se abandonó esa vía explícitamente en vez de
  inventar una "longitud p-ádica" ad hoc sin respaldo teórico.
- **La forma cuadrática `x²+y²=1` sobre `ℚ_p` (§4.2, interludio) se
  identificó como una vía genuina y no se persiguió a fondo.** Es un objeto
  real de teoría de números (símbolo de Hilbert, Hasse–Minkowski) que
  determina si el "círculo unidad p-ádico" tiene puntos racionales, pero no
  produce una cantidad comparable a π (es una pregunta de existencia,
  sí/no, no una medida). Se dejó fuera del alcance de este documento porque
  mezclar esa teoría con la pregunta de "tamaño sin π" habría exigido un
  documento aparte del mismo calibre que `10` dedicó a la capacidad
  logarítmica, y no estaba claro de antemano que fuera a producir una
  cantidad comparable.
- **No se verificó con el binario ninguna cantidad genuinamente p-ádica**
  (medida de Haar, dimensión de Hausdorff, estructura de `ℤ_p^*`) porque
  `dz-cli` solo hace aritmética entera/racional en base 12, sin ningún
  concepto de valuación `p`-ádica ni de límite inverso — coherente con lo
  que pedía el encargo (usar el binario solo para lo dozenal, derivar o
  citar el resto explícitamente). Las cifras `1-1/2`, `1-1/3`, `1-1/5` de §3
  son verificaciones de la **aritmética racional** de la fórmula, no de la
  teoría p-ádica en sí, y se presentan como lo que son.
- **La estructura de `ℤ_p^*` para `p` impar (`μ_{p-1} × (1+pℤ_p)`) se citó
  como hecho estándar de la teoría, sin rederivarla** — a diferencia del
  resto del documento (Haar, dimensión de Hausdorff, el conteo `p`, todo
  derivado a mano), esa descomposición es álgebra conmutativa/teoría de
  cuerpos locales más profunda (isomorfismo logarítmico) y no aportaba
  contenido nuevo al argumento central si se rederivaba; se marca así en vez
  de disimular la diferencia de rigor con el resto del documento, siguiendo
  el estándar que `10` §7 ya aplicó a la fórmula del cuadrado.
- **No se comparó la construcción con la línea B (trigonometría L₁,
  `06-hallazgo-conversion-angular-taxicab.md`)** de forma explícita, aunque
  el mecanismo es primo hermano: allí la trigonometría taxicab fallaba
  porque el grupo de simetría del rombo es finito (`D₄`, orden 8) y no puede
  alojar una familia continua; aquí el grupo "de rotación" p-ádico
  (`ℤ_p^*`, vía `μ_{p-1}×(1+pℤ_p)`) tampoco es continuo, por una razón
  distinta (es profinito, no un grupo de Lie finito). No se investigó si
  ambos fallos son instancias de un principio común más general
  ("ningún grupo totalmente disconexo aloja una familia continua no
  trivial", que de hecho es casi tautológico una vez dicho así) o si merece
  un tratamiento propio; se deja como posible cuarto ingrediente si la lista
  continúa.
- **No se exploró la energía/capacidad p-ádica** (el análogo p-ádico de la
  capacidad logarítmica de `10`, que existe en la literatura de teoría del
  potencial no arquimediana) — habría sido la comparación más directa con
  `10`, y se dejó fuera por alcance: este documento ya tenía su propio eje
  central (la pregunta mal planteada de §4.4) y añadir una cuarta sección
  de la extensión de la vía de `10` habría diluido el resultado principal.
- **Nada de esto se sometió a revisión externa**, siguiendo la práctica ya
  declarada del resto del repositorio (`99-sintesis.md` §7).
