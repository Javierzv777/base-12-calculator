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

### "¿Esto no es el backend de tabla disfrazado?" — la objeción, tomada en serio

Es una objeción razonable y hay que responderla con las cartas boca
arriba, no esquivarla.

**Lo que hace el backend de tabla (`TableGates`, `dz-gates/src/backends/table.rs`):**
en tiempo de ejecución, para un `x` y un `y` concretos, hace
`MUL_TABLE[x.tag()][y.tag()]` — indexa memoria *directamente* con el tag
del operando. La tabla **es** la operación; no hay ningún `DozGates::lit`,
`::min` ni `::max` de por medio, y por diseño ese backend no cuenta
ninguna puerta para la multiplicación (por eso el plan lo llama "no es un
backend de producción", solo un oráculo para tests).

**Lo que hace `digit_mul_v2`:** en tiempo de ejecución, para un `x` y un
`y` concretos, evalúa **144 ramas** de `MIN(lit(j,x), lit(k,y), CONST)` y
las combina con `MAX` — 876 llamadas a primitiva en total (medido, ver
abajo), sin importar qué valores concretos tengan `x` e `y`. Nunca llama a
`x.tag()`/`y.tag()`. El costo se puede medir, se puede comparar contra v1,
y **es genuinamente mayor que cero** — a diferencia del backend de tabla,
donde el costo de la multiplicación es, por construcción, cero puertas.

La distinción exacta que sostenemos: **las constantes `LO[j][k]`/
`HI[j][k]` son la especificación de la función** (qué debería salir para
cada combinación posible de entradas), no la implementación de cómo se
calcula esa salida para una entrada dada. Esto es idéntico en naturaleza a
los `cycle^j(x)` precomputados de la fórmula de la suma del semi-sumador
(Sección 2), o a la constante `Digit::new(RADIX-1-j)` de
`ones_complement_digit` (`circuits/complement.rs`): en ambos casos hay una
tabla de valores por-rama fijada de antemano, y el circuito real es la
estructura decodificador+mux que la usa. Nadie objeta que `half_adder` sea
"el backend de tabla disfrazado" aunque también tiene una tabla de 12
valores (`cycle^0(x), cycle^1(x), ..., cycle^11(x)`) escondida en su
fórmula — la diferencia con el backend de tabla es siempre la misma: ¿se
indexa con el tag del operando en tiempo de ejecución (tabla), o se decodifica
el operando en señales booleanas y se multiplexa (SOP/circuito)?
`digit_mul_v2` hace lo segundo.

**Dónde la objeción sí tiene razón, y hay que decirlo:** las constantes
`LO[j][k]`/`HI[j][k]` se calculan, en `build_lo_hi_tables()`
(`multiplier.rs`), con aritmética nativa (`j * k`, `% RADIX`, `/ RADIX`)
sobre `j` y `k` — dos contadores de bucle 0..11, nunca un `.tag()` leído de
un `Digit` real. Eso es aritmética nativa que "sabe" lo que es multiplicar
en base 12, ejecutada en tiempo de compilación/construcción para fijar el
cableado del circuito. Es exactamente la misma categoría de excepción que
ya usan `backends/mvl.rs` (`CYCLE_TABLE` vía `(x+1) % RADIX`) y
`backends/table.rs` (`MUL_TABLE` vía `x * y`) — con la diferencia de que
esas viven en `backends/` (el lugar ya señalado como sede legítima de
"definir qué significa cada símbolo") y `build_lo_hi_tables` vive en
`circuits/multiplier.rs`, un archivo que hasta ahora no necesitaba tocar
aritmética nativa de ningún tipo salvo sobre contadores de bucle triviales
(`RADIX - j`, no productos). Si el estándar es "cero aritmética nativa que
codifique el significado de la multiplicación, en ningún archivo", esto lo
incumple técnicamente. Nuestra posición es que la línea correcta a trazar
no es "en qué archivo vive el código" sino "¿qué hace el circuito con un
`Digit` real en tiempo de ejecución?" — y ahí `digit_mul_v2` nunca lee un
tag ni indexa con uno. Pero es una línea que se puede trazar distinto, y
se documenta aquí en vez de darla por sentada.

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
  no se tocó en este trabajo. Sigue siendo el candidato obvio para
  carry-lookahead (Fase 8).

## Qué no se construyó (honestidad de alcance)

- Carry-lookahead dozenal (generate/propagate) — Fase 8, no implementado.
  El diseño en el plan (`G = thr(12-y,x)`, `P = lit(11,s)`) está descrito
  pero no codificado ni medido.
