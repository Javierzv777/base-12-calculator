# 01 — Diseño del sumador y del multiplicador

## Semi-sumador (Sección 2 del plan, implementado tal cual)

`half_adder` (`crates/dz-gates/src/circuits/adder.rs`) implementa
exactamente:

```
s = MAX_{j=0..11} MIN( lit(j, y), cycle^j(x) )
c = MAX_{j=0..11} MIN( lit(j, y), thr(12-j, x) )
```

con una corrección importante encontrada durante la implementación: `lit` y
`thr` devuelven el símbolo `MAX` (↋, el "verdadero" de la lógica de Post) y
no el dígito numérico `1`. La fórmula del acarreo, tal como está escrita en
el plan, produce entonces un *booleano MVL* (`ZERO` o `MAX`), no un dígito
de peso posicional 1. Para poder sumarlo en la siguiente posición hace falta
una "digitalización" final:

```rust
let carry = g.min(carry, Digit::ONE); // MIN(MAX, ONE) = ONE; MIN(ZERO, ONE) = ZERO
```

Este es el mismo truco de "MIN como multiplexor" que ya usa la propia
fórmula de la suma (`MIN(lit(j,y), cycle^j(x))` selecciona `cycle^j(x)` o lo
anula a `ZERO`), aplicado aquí a una constante en vez de a una variable. Sin
este paso, los 144 casos de la suma fallaban exactamente donde había
acarreo (verificado por el test diferencial antes de la corrección).

El sumador completo (`full_adder`) combina dos semi-sumadores y un `MAX` de
acarreos; `ripple_add` los encadena dígito a dígito sobre `Vec<Digit>`
little-endian.

## Multiplicador v1: shift-add (Sección 2)

Implementado en `crates/dz-gates/src/circuits/multiplier.rs` como
`digit_mul_v1`. Usa el mismo decodificador que el acarreo del sumador, pero
decodificando sobre *todos los valores posibles* de `y` y computando, para
cada uno, "`x` sumado a sí mismo `j` veces" vía `ripple_add` repetido —
nunca tabulando el producto ni leyendo `.tag()`.

## Multiplicador v2: SOP de dos niveles (Sección 2)

Implementado como `digit_mul_v2`, exactamente la forma canónica de la
Sección 2 del plan:

```
lo = MAX_{j,k} MIN( lit(j,x), lit(k,y), LO[j][k] )
hi = MAX_{j,k} MIN( lit(j,x), lit(k,y), HI[j][k] )
```

Decodificador 1-de-12 sobre `x` (`lit(j,x)`, 12 señales) y sobre `y`
(`lit(k,y)`, 12 señales), 144 ramas `(j,k)`, cada una multiplexada con
`MAX`. `LO[j][k]`/`HI[j][k]` son constantes: para cada rama, `MIN(lit(j,x),
lit(k,y))` es el AND de dos booleanos MVL ("x es exactamente j" y "y es
exactamente k"); `MIN(ese AND, CONST)` selecciona la constante si la rama
coincide, o la anula a `ZERO` — el mismo truco de "MIN como multiplexor"
que ya usa `half_adder`. En tiempo de ejecución el circuito nunca lee
`x.tag()`/`y.tag()`: decodifica ambos operandos en 24 señales booleanas
(12+12) y las combina combinacionalmente.

`digit_mul` (el nombre usado por `scale_by_digit`/`multiply`, y por lo
tanto por `dz-int`) es ahora un alias de `digit_mul_v2` — **v2 es el
multiplicador por defecto**. `digit_mul_v1` se conserva, exportado por
separado, exactamente para poder seguir comparando los dos diseños.

### "¿Esto no es el backend de tabla disfrazado?" — objeción planteada, y resuelta

Esta sección documentó originalmente una concesión: las constantes
`LO[j][k]`/`HI[j][k]` de `digit_mul_v2` se calculaban con `j * k` nativo
(dos contadores de bucle, nunca un `.tag()` leído de un `Digit` real), y
esa entrega admitía que era "aritmética nativa que sabe lo que es
multiplicar en base 12", aunque viviera fuera de `backends/`. Una revisión
posterior identificó que la concesión no era suficiente: había que mirar
*qué primitivas invierte v2 en contenido aritmético real*, no solo dónde
vive el código que construye la tabla.

**El diagnóstico.** El desglose de `digit_mul_v2` (`cargo run -p dz-gates
--example gate_counts`) es `cycle=0, thr=0, min=432, max=288, lit=156`.
`lit` selecciona, `min` conjunta, `max` combina — ninguna de las tres
aporta contenido dozenal: son puro enrutamiento booleano. Compárese con
`digit_mul_v1`, que gasta `cycle=14652`: el sucesor, aplicado repetidamente,
sí es contenido dozenal real (es la definición misma de "sumar"). Eso
significa que en v2 **toda la semántica de la multiplicación vivía en las
constantes**, y esas constantes salían de `j * k` nativo. La defensa
original ("el costo se mide, nunca se lee `.tag()`") es correcta pero
responde a una pregunta distinta de "¿de dónde salió el *significado* de
la multiplicación?". Las tablas de `cycle`/`min`/`max` en `backends/` son
axiomas — definen qué son los símbolos, no hay otra forma de escribirlos.
La multiplicación no es un axioma: es exactamente la operación derivada
que este proyecto entero se propone construir desde las primitivas.
Derivarla con `*` nativo, aunque fuera para "solo" fijar una constante,
era hacer con la mano lo que el circuito decía estar computando con
puertas.

**El arreglo.** `LO[j][k]`/`HI[j][k]` ya no se calculan con aritmética
nativa. `lo_hi_table()` (`circuits/multiplier.rs`) construye la tabla una
única vez, en tiempo de ejecución, memoizada en un `OnceLock`, llamando a
`digit_mul_v1(init_gates, Digit::new(j), Digit::new(k))` para cada uno de
los 144 pares — `digit_mul_v1` ya está compuesto enteramente de puertas y
ya está verificado equivalente al oráculo en las 144 combinaciones. No
queda ningún `*`, `%` ni `/` sobre un valor de dígito en todo el archivo.
`j`/`k` siguen siendo contadores de bucle usados para construir los
`Digit` de prueba (`Digit::new(j)`), exactamente como ya hace
`Digit::all()` — eso nunca fue el problema; el problema era computar el
*producto* nativamente, y eso ya no ocurre en ningún sitio de este
archivo.

`init_gates` es una instancia de `MvlGates` desechable, creada solo para
esta síntesis; su `GateCount` nunca se lee, así que el costo (único,
memoizado) de construir la tabla no contamina el conteo de puertas medido
para una llamada de un usuario a `digit_mul_v2` — verificado: `cargo run -p
dz-gates --example gate_counts` sigue reportando exactamente **876** por
llamada, igual que antes del cambio.

La síntesis usa `MvlGates` fijo, no genérico sobre el backend `G` de quien
llama. Es una decisión deliberada: la multiplicación es una única función
matemática, y los tests diferenciales
(`digit_mul_v1_v2_and_oracle_agree_144_all_backends`) ya establecen que
los tres backends calculan el mismo `(lo, hi)` para las 144 combinaciones
— así que qué backend sintetiza la especificación no puede cambiar cuál es
la especificación.

**Con esto resuelto, la comparación con el backend de tabla queda así:**
en tiempo de ejecución, `TableGates` hace `MUL_TABLE[x.tag()][y.tag()]` —
indexa memoria *directamente* con el tag del operando; la tabla **es** la
operación, y por diseño no cuenta ninguna puerta (de ahí que el plan lo
llame "no es un backend de producción"). `digit_mul_v2`, en cambio, evalúa
144 ramas de `MIN(lit(j,x), lit(k,y), CONST)` combinadas con `MAX` — 876
llamadas a primitiva, medibles, comparables contra v1 — y nunca lee
`x.tag()`/`y.tag()` en tiempo de ejecución; decodifica ambos operandos en
24 señales booleanas y las combina combinacionalmente. La diferencia ya no
depende de una lectura generosa de "especificación vs. implementación": la
especificación (`LO`/`HI`) ahora también se deriva de puertas, así que no
queda ningún `*`/`%`/`/` nativo escondido en ninguna parte de la cadena
v2 → resultado. Es, ahora sí, "una memoización sintetizada de una función
derivada con puertas", no una tabla importada desde aritmética binaria.

## Conteos de puertas medidos

Medido con `cargo run -p dz-gates --example gate_counts` (backend MVL salvo
donde se indica; el conteo a nivel de las 5 primitivas es **idéntico** entre
MVL y CRT porque ambos implementan el mismo `DozGates` y los circuitos de
más arriba llaman a las primitivas el mismo número de veces
independientemente del backend — la diferencia de costo real de CRT está en
cuánto cuesta *realizar* cada primitiva, no en cuántas veces se invoca; ver
`02-crt-vs-mvl.md`).

| Operación | cycle | min | max | lit | thr | **total** |
|---|--:|--:|--:|--:|--:|--:|
| `half_adder(7, 9)` (1 dígito) | 66 | 25 | 24 | 24 | 12 | **151** |
| `digit_mul_v1(7, 9)` shift-add | 14 652 | 5 574 | 5 463 | 5 340 | 2 664 | **33 693** |
| `digit_mul_v2(7, 9)` SOP | 0 | 432 | 288 | 156 | 0 | **876** |
| `digit_mul_v1(11, 11)` shift-add (peor caso) | 14 652 | 5 574 | 5 463 | 5 340 | 2 664 | **33 693** |
| `digit_mul_v2(11, 11)` SOP | 0 | 432 | 288 | 156 | 0 | **876** |
| `digit_mul_v1(0, 0)` shift-add (mejor caso) | 8 712 | 3 324 | 3 258 | 3 180 | 1 584 | **20 058** |
| `digit_mul_v2(0, 0)` SOP | 0 | 432 | 288 | 156 | 0 | **876** |
| `ripple_add` (4 dígitos × 4 dígitos) | 528 | 200 | 196 | 192 | 96 | **1 212** |
| `multiply` (3 dígitos × 2 dígitos, por defecto = v2) | 1 980 | 3 342 | 2 457 | 1 656 | 360 | **9 795** |

Observaciones:

- **v2 cuesta 876 puertas, siempre — v1 cuesta entre 20 058 y 33 693 según
  el valor de `y`.** La razón de la variación en v1 es estructural: su
  decodificador ejecuta, para cada `j` de 0 a 11, `j` aplicaciones de
  `ripple_add` antes de descartar la rama con `MIN(lit(j,y), ...)` — así
  que el costo total depende de *cuánto suma* en las ramas descartadas
  además de en la que sobrevive, y por construcción crece con `y`
  (`digit_mul_v1(x, 0)` no sale gratis: sigue evaluando las ramas `j=1..11`
  aunque el resultado final sea cero). v2, al ser un decodificador de
  profundidad constante sobre *ambos* operandos, no tiene ninguna
  dependencia de los valores concretos: siempre son exactamente 144 ramas,
  siempre 12 `lit` para `x` + 144 `lit` para `y` = 156, siempre 432 `min` y
  288 `max`. **Reducción: ~38.5x en el mejor caso de v1 comparado con v2
  (33 693/876), ~23x en el caso más favorable a v1 (20 058/876).**
- **A nivel multi-dígito la ganancia se diluye pero sigue siendo grande:**
  `multiply` de 3x2 dígitos pasa de 201 243 puertas (v1) a 9 795 (v2, ~20.5x
  menos), porque `ripple_add`/`half_adder` (que no cambiaron) ahora
  representan una fracción mayor del costo total.
- **Un solo `half_adder` sigue costando 151 invocaciones de puerta.** El
  decodificador de 12 ramas (`MAX_j MIN(...)`) domina el costo de la suma;
  no se tocó en este trabajo. Fue el punto de partida del carry-lookahead
  (Fase 8), abajo.

## Carry-lookahead dozenal (Fase 8)

Implementado en `crates/dz-gates/src/circuits/adder.rs` como
`lookahead_add`, con la misma semántica observable que `ripple_add`
(mismo relleno con `Digit::ZERO` del operando corto, misma regla de limbo
extra, mismo `Vec` de salida para toda entrada). Verificado exhaustivamente
en 1 dígito (144 casos × 3 backends) y en 2 dígitos (12⁴ = 20 736 casos).

### Generate y propagate

`generate_propagate(g, x, y)` devuelve dos **booleanos MVL**
(`Digit::ZERO` / `Digit::MAX`, nunca el dígito 1):

```
G = MAX_{j=0..11} MIN( lit(j, y), thr(12-j, x) )     "x + y ≥ 12"
P = lit(11, s)   donde (s, _) = half_adder(x, y)      "x + y = 11"
```

PLAN.md escribía `G = thr(12−y, x)`, pero eso es taquigrafía: `thr` toma un
umbral `j: u8` **constante**, no un `Digit`. El circuito real es el mismo
decodificador de 12 ramas que ya usaba el acarreo del `half_adder`; de
hecho `G` *es* ese acarreo antes de digitalizarlo con `MIN(·, ONE)`. Por eso
no hay circuito nuevo aquí ni copia del decodificador: `half_adder` y
`generate_propagate` comparten un único cuerpo (`half_adder_raw_net`) y el
`half_adder` público simplemente le añade la digitalización final.

`G` y `P` son mutuamente excluyentes, así que `max` sirve como OR sin
ambigüedad: `P` exige `x + y = 11 < 12`, luego `G` es falso; y `G ∧ P`
exigiría `x + y = 23`, imposible con dos dígitos cuyo máximo es 22. Hay un
test de las 144 combinaciones que lo comprueba.

### La red de prefijos

La recurrencia `c_{i+1} = G_i ∨ (P_i ∧ c_i)` expandida da la forma plana

```
c_i = G_{i-1} + P_{i-1}·G_{i-2} + P_{i-1}·P_{i-2}·G_{i-3} + …
```

que ya no tiene dependencia secuencial. Escrita ingenuamente, sin embargo,
recalcula cada prefijo de propagación en cada posición, y un `fold` a la
izquierda sobre los términos devolvería la profundidad a O(n) — es decir,
anularía el experimento entero. La implementación es la evaluación por
doblado recursivo (Kogge-Stone) de esa misma expansión plana: *es* la
reducción binaria balanceada del `MAX` de términos y del `MIN` de cada
prefijo, compartiendo cada prefijo entre todas las posiciones que lo
necesitan. El operador de bloque

```
(G_hi, P_hi) ∘ (G_lo, P_lo) = ( G_hi ∨ (P_hi ∧ G_lo),  P_hi ∧ P_lo )
```

es asociativo, así que tras `⌈log₂ n⌉ ` etapas de doblado `gen[i]` contiene
el generate de grupo de todo el bloque `0..=i`, que es exactamente el
acarreo que sale de la posición `i`.

### El dígito de suma final

El acarreo se mantiene como booleano MVL de principio a fin, así que la
suma final **no** se rehace con un segundo `half_adder` (151 puertas y ~27
niveles por posición). Es un multiplexor de 5 puertas sobre la semisuma:

```
out_i = MAX( MIN( lit(0, c_i), s_i ),  MIN( c_i, cycle(s_i) ) )
```

`lit(0, c)` es el complemento booleano de `c` (cierto justo cuando `c` es
`ZERO`), de modo que se elige `s_i` si no llega acarreo y `cycle(s_i)` —la
primitiva sucesor, o sea «+1»— si llega. El caso de desbordamiento
`s_i = 11` ya está contemplado: eso es precisamente `P_i`, y la red de
prefijos ya reenvió el acarreo más allá de esta posición. Aquí está la
mitad de la ganancia en puertas: el ripple gasta dos `half_adder` completos
por dígito, el lookahead uno solo.

### Cómo se mide la profundidad

`GateCount` cuenta *invocaciones*: es un contador plano sin noción de qué
puerta alimenta a cuál, y un `Digit` no lleva procedencia, así que la
profundidad no se puede deducir de él. En vez de duplicar los circuitos
(una versión «normal» y otra «instrumentada», que acabarían divergiendo),
los sumadores se escribieron **una sola vez**, genéricos sobre el trait
`GateNet` de `crates/dz-gates/src/circuits/depth.rs`, con dos
instanciaciones:

- `Values` — la señal es un `Digit` y cada método delega tal cual al
  backend. Es lo que usan las funciones públicas, así que sus conteos son
  bit a bit los de siempre (hay un test de regresión que fija 151 y 1 212).
- `Levels` — la señal es `Sig { value, level }`: el valor lo sigue
  calculando el backend y el nivel se propaga en paralelo.

El modelo: entrada primaria o constante = nivel 0; cada invocación de
primitiva produce `1 + max(niveles de sus entradas)` (`lit`/`thr` son
unarias en `x`, el índice `j` es estructura del circuito, no señal); la
profundidad del circuito es el nivel máximo de sus salidas.

**No contamina `GateCount`.** No se añadió ningún campo a `GateCount`
(`total()` y su `impl Add` siguen sumando exactamente las cinco primitivas
publicadas). Una pasada con `Levels` sí invoca al backend para calcular
valores, así que conteo y profundidad se miden en pasadas separadas con
backends frescos y desechables — la misma disciplina que `lo_hi_table()`
usa para su backend de síntesis.

### Resultados medidos

Peor caso de propagación (`[↋]×n + [1]`), backend MVL:

| n | puertas ripple | prof. ripple | puertas lookahead | prof. lookahead | ratio puertas | ratio prof. |
|--:|--:|--:|--:|--:|--:|--:|
| 1 | 303 | 30 | 157 | 17 | 0,52× | 0,57× |
| 2 | 606 | 46 | 316 | 18 | 0,52× | 0,39× |
| 4 | 1 212 | 78 | 640 | 21 | 0,53× | 0,27× |
| 8 | 2 424 | 142 | 1 300 | 23 | 0,54× | 0,16× |
| 16 | 4 848 | 270 | 2 644 | 25 | 0,55× | 0,09× |

Profundidad de un `half_adder` de un dígito: **15** (12 de esos 15 niveles
son el pliegue lineal de 12 `max` del decodificador).

Ajustes: la profundidad del ripple es exactamente afín, `16n + 14` (un
`full_adder` por posición, cada uno añadiendo 16 niveles a la cadena de
acarreo). La del lookahead crece logarítmicamente: 15 niveles del
decodificador compartido, más 2 por cada etapa de doblado que el acarreo de
esa posición atraviesa, más el mux; de n = 1 a n = 16 sube solo de 17 a 25.
Las puertas del ripple son `303n` exactamente; las del lookahead,
`156n + 1 + 3·Σ_k (n − 2^k)`, o sea O(n log n) — pero con una constante
lineal (156 frente a 303) que la red de prefijos tarda muchísimo en
compensar.

### Lectura honesta de los números

Se esperaba que el conteo total **subiera** (es el precio habitual del
lookahead: O(n log n) puertas a cambio de O(log n) de profundidad) y que la
profundidad lo justificara igualmente. **No ocurrió: el lookahead gana en
las dos métricas a la vez, y el punto de cruce es n = 1.** Conviene ser
preciso sobre por qué, porque es un resultado sobre *este* sumador, no
sobre el carry-lookahead en general:

1. **El `full_adder` del ripple era caro por construcción.** Encadena dos
   `half_adder` (302 puertas) por posición. El lookahead usa un solo
   decodificador por posición (150 puertas, más 1 `lit` para `P`) y resuelve
   el acarreo entrante con un mux de 5 puertas. Ese ahorro de ~150 puertas
   por dígito domina de sobra al coste O(n log n) de la red de prefijos,
   que a n = 16 son solo 147 puertas de 2 644 (el 5,6 %).
2. **El punto de cruce de la profundidad en n = 1 es real pero modesto**
   (30 → 17): a un dígito no hay ninguna red de prefijos, la mejora viene
   solo de haber eliminado el segundo `half_adder`. La ganancia
   *estructural* aparece a partir de n = 2, donde el ripple crece 16
   niveles por dígito y el lookahead 2 por etapa de doblado.
3. **El pliegue lineal de 12 `max` del `half_adder` sigue ahí y ahora es el
   cuello de botella.** De los 17-25 niveles del lookahead, 15 son el
   decodificador compartido; la red de prefijos aporta 2 por etapa. No se
   rebalanceó a propósito: el 151 publicado es un resultado de la
   investigación y un árbol balanceado lo convertiría en 150 en silencio.
   Rebalancear ese pliegue (12 niveles → 4) bajaría el lookahead de 16
   dígitos de 25 a ~17 niveles; es el siguiente experimento obvio, y es
   independiente de esta fase.

Ninguna capa por encima de `dz-gates` usa todavía `lookahead_add`:
`dz-int` sigue llamando a `ripple_add`. La sustitución es trivial (misma
firma, misma semántica) pero se deja fuera de esta fase para que los
números publicados de `multiply`, `div_rem`, etc. sigan siendo comparables
con los de las fases anteriores.

## Qué no se construyó (honestidad de alcance)

- Rebalanceo del pliegue lineal de 12 `max` dentro de `half_adder`. Se
  midió su coste en profundidad (12 de los 15 niveles) pero no se cambió:
  reduciría el 151 publicado a 150 y hay que decidirlo como un cambio de
  resultado, no colarlo dentro de la Fase 8.
- `dz-int` no adoptó `lookahead_add`; sigue en `ripple_add`.
