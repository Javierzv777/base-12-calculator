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

## Multiplicador (v1: shift-add, Sección 2)

Implementado en `crates/dz-gates/src/circuits/multiplier.rs`. La
multiplicación de un dígito (`digit_mul`) usa el mismo decodificador que el
acarreo del sumador, pero decodificando sobre *todos los valores posibles*
de `y` y computando, para cada uno, "`x` sumado a sí mismo `j` veces" via
`ripple_add` repetido — nunca tabulando el producto ni leyendo `.tag()`. El
multiplicador multi-dígito hace shift-add clásico: por cada dígito de `B`,
escala `A` por ese dígito (`scale_by_digit`, usando `digit_mul` + acarreo) y
lo desplaza (`shift_left`, anteponiendo dígitos `ZERO`), acumulando con
`ripple_add`.

No se implementó v2 (SOP dígito×dígito sintetizado en MVL) por presupuesto
de tiempo; ver la sección de conteos abajo para ver por qué **v1 ya es
sorprendentemente caro** y por qué v2 sería la optimización obvia a medir a
continuación (Fase 8).

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
| `digit_mul(7, 9)` (1 dígito) | 14 652 | 5 574 | 5 463 | 5 340 | 2 664 | **33 693** |
| `ripple_add` (4 dígitos × 4 dígitos) | 528 | 200 | 196 | 192 | 96 | **1 212** |
| `multiply` shift-add (3 dígitos × 2 dígitos) | 87 516 | 33 294 | 32 625 | 31 896 | 15 912 | **201 243** |

Observaciones:

- **Un solo `half_adder` cuesta 151 invocaciones de puerta** para sumar dos
  dígitos. El decodificador de 12 ramas (`MAX_j MIN(...)`) domina el costo:
  12 ramas para la suma más 12 para el acarreo, cada una con su propio
  `lit`/`thr`/`min`, más el costo de `cycle^j` (hasta 11 aplicaciones de
  `cycle` por rama). Es un diseño deliberadamente "de libro de texto"
  (decodificador + mux), no optimizado — el punto de la Fase 8
  (carry-lookahead) es precisamente reducir esto.
- **`digit_mul` cuesta ~223× más que `half_adder`** (33 693 vs 151) porque
  v1 multiplica por repetición: para decodificar sobre los 12 valores
  posibles de `y`, computa "`x` sumado a sí mismo `j` veces" para cada
  `j` de 0 a 11, es decir hasta 11 llamadas a `ripple_add` (cada una ~151
  puertas) *por cada una de las 12 ramas del decodificador*, la mayoría de
  las cuales se descartan por el `MIN(lit(j,y), ...)`. Esto es
  extremadamente ineficiente pero *puro* — nunca lee `.tag()` fuera de
  `dz-gates::backends`. Es exactamente el resultado que motiva probar v2
  (SOP sintetizado, que sería O(1) decodificador de 144 ramas en vez de
  este O(12×11) de sumas anidadas) en trabajo futuro.
- **Multiplicar dos números multi-dígito ya cuesta ~200 000 puertas** para
  operandos de solo 2-3 dígitos, todo heredado de que cada `digit_mul`
  individual es caro. Esto es aceptable para una calculadora interactiva
  (los números en el REPL rara vez superan unos pocos dígitos) pero sería
  inviable para aritmética de precisión arbitraria grande sin optimizar el
  multiplicador de un dígito primero.

## Qué no se construyó (honestidad de alcance)

- Carry-lookahead dozenal (generate/propagate) — Fase 8, no implementado.
  El diseño en el plan (`G = thr(12-y,x)`, `P = lit(11,s)`) está descrito
  pero no codificado ni medido.
- Multiplicador v2 (SOP) — no implementado; ver arriba.
