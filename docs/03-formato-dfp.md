# 03 — Formato de punto flotante dozenal (DFP)

**Estado: implementado** (`dz_num::DzFloat`, `crates/dz-num/src/dfp.rs`).

## Formato elegido

- **Signo**: 1 bit (`bool` nativo — no es una cantidad dozenal, es metadato).
- **Exponente sesgado**: conceptualmente dos dígitos dozenales (rango crudo
  `0..=143`), sesgo `71`. Codificaciones reservadas (análogas a IEEE-754):
  - Crudo `0` → el valor es **cero** (mantisa ignorada).
  - Crudo `143` (`↋↋`, "todo elevens", el análogo dozenal de "todo unos" en
    IEEE-754) con mantisa cero → **Infinito** (con signo). Con mantisa
    distinta de cero → **NaN**.
  - Crudo `1..=142` → número normal, exponente real (sin sesgo) en
    `-70..=71`.
- **Mantisa**: doce dígitos dozenales, normalizada (el dígito más
  significativo de los doce es distinto de cero). Se almacena como un
  entero `Dz` `M` con `12^11 <= M < 12^12`.
- **Valor** cuando es normal: `(-1)^signo * M * 12^(exponente_real - 11)`.

Tanto el exponente sesgado como la mantisa se implementan como `Dz` y se
operan **exclusivamente** con las operaciones ya construidas de `Dz`
(`add`, `sub`, `mul`, `div_rem`, `compare`, `is_even`) — nunca con
aritmética nativa sobre el valor de un dígito. Las posiciones (cuántos
dígitos mide un `Vec`, cuántas veces itera un bucle) siguen siendo
aritmética nativa sobre `usize`/longitudes, tal como permite la Sección 0
del plan; en ningún punto nuevo se llama a `.tag()` fuera de los archivos
ya exentos.

No se implementan subnormales (una simplificación deliberada, documentada
en el código): un exponente que desbordaría por abajo redondea a cero en
vez de perder precisión gradualmente.

## Corrección sobre el redondeo half-even (la nota anterior estaba mal)

La entrega anterior de este documento afirmaba que en base 10 "el 5 es
siempre el punto medio" y sugería que la base 12 sería más simple o más
complicada por comparación. **Eso es incorrecto y no se construyó nada
sobre esa premisa equivocada.**

Lo correcto: en base 10 solo hay empate exacto cuando el resto a redondear
es *exactamente* `5000...` — un dígito `5` seguido únicamente de ceros. Un
`5` con algún dígito distinto de cero detrás no es un empate: es
estrictamente "más de la mitad" y redondea hacia arriba sin ambigüedad. En
base 12 ocurre exactamente lo mismo con el dígito `6` (la mitad de 12):
empate exacto solo cuando el resto es `6` seguido únicamente de ceros;
`6` seguido de cualquier dígito no nulo redondea hacia arriba sin
ambigüedad, igual que en base 10. Como 12 es par, existe un dígito que
representa exactamente la mitad del radix — igual que en base 10 (que
también es par). No hay ni simplificación ni complicación relativa a base
10 en este aspecto: **la estructura del empate es la misma** en cualquier
base de radix par.

Lo que sí es una observación correcta y útil: al ser 12 par, la **paridad**
de un número dozenal (par/impar) se lee en su último dígito exactamente
igual que en base 10 (`0,2,4,6,8,↊` son dígitos "pares"; `1,3,5,7,9,↋` son
"impares"). Por eso el criterio de "redondear al par" en half-even se
puede decidir mirando solo el dígito menos significativo del resultado
retenido, sin mirar el resto del número — implementado en
`dz_gates::circuits::is_even_digit` (un decodificador `MAX_j MIN(lit(j,x),
...)` sobre los seis dígitos pares) y expuesto como `Dz::is_even`.

## Algoritmo de redondeo implementado

`DzFloat::from_dzfixed` (la función de codificación) recibe un valor exacto
`DzFixed` y:

1. Si la magnitud tiene 12 dígitos significativos o menos: el resultado es
   exacto, solo se rellena con dígitos `ZERO` a la derecha (multiplicar por
   una potencia de 12 — construcción de `Vec`, no hay redondeo que hacer).
2. Si tiene más de 12: se divide por `12^(exceso)` con `Dz::div_rem`
   (división entera exacta compuesta por puertas), obteniendo cociente `q`
   y resto `r`. Luego:
   - Si `2r < divisor`: se descarta el resto, `q` queda igual (redondeo
     hacia abajo, la diferencia era menos de la mitad).
   - Si `2r > divisor`: `q = q + 1` (redondeo hacia arriba, más de la
     mitad — incluye el caso "dígito `6` seguido de un dígito no nulo").
   - Si `2r == divisor` **exactamente** (el único empate real): se
     redondea a par, `q = q + 1` solo si `q` es impar (`!Dz::is_even`).
   - Si el redondeo hacia arriba desborda la mantisa a 13 dígitos (todo
     dígitos `↋` redondeando a `12^12` exacto), se renormaliza dividiendo
     una vez más entre 12 y sumando 1 al exponente — el mismo ajuste que
     cualquier formato de punto flotante necesita en ese caso límite.

Toda esta aritmética (`2r`, la comparación, el `+1`, la renormalización)
usa `Dz::add`/`Dz::compare`/`Dz::div_rem`, nunca native `+`/`-`/`<` sobre
dígitos.

## Aritmética DFP

`DzFloat::add/sub/mul/div` decodifican ambos operandos a `DzFixed` exactos
(`to_dzfixed`, también compuesto solo de `Dz::mul`/`Dz::div_rem`/`Dz::add`
según el signo del exponente), operan con la aritmética exacta ya
construida en `DzFixed`/`Dz`, y vuelven a codificar el resultado con el
mismo algoritmo de redondeo de arriba. Los casos especiales (`NaN`,
`Infinito`, `Infinito - Infinito`, `0 * Infinito`, `x / 0`) siguen las
convenciones habituales de IEEE-754, adaptadas: ver las tablas de casos en
`crates/dz-num/src/dfp.rs`.

`evaluate_dfp` (en `dz-eval`) evalúa la expresión completa en racionales
exactos primero y redondea **una sola vez** al final, en vez de redondear
después de cada operación — evita que el error de redondeo se acumule por
operación. Una variante que redondee tras cada paso (más fiel a cómo se
comporta hardware de punto flotante real) queda anotada como extensión
posible, no implementada.

## Nota sobre `evaluate_dfp` y `1/0` en el REPL

`dz-eval::evaluate_dfp` evalúa la expresión completa como `DzRational`
exacto primero (para no acumular redondeo por operación) y solo al final
convierte a `DzFloat`. Como consecuencia, `1/0` escrito directamente en el
modo `dfp` del REPL da `error: division by zero` (el error viene de
`DzRational::div`, antes de llegar a `DzFloat`) en vez de `Inf`, aunque
`DzFloat::div` en sí mismo **sí** implementa correctamente la semántica
`x/0 = Inf`, `0/0 = NaN` (verificado directamente por
`special_value_arithmetic` en `crates/dz-num/tests/dfp.rs`). Es una
limitación conocida de cómo `dz-eval` compone las capas, no del formato
DFP en sí — queda anotada en vez de disimulada.

## Qué falta / limitaciones conocidas

- Sin subnormales (ver arriba).
- Sin señal de "cero con signo" persistente desde `Dz` (un `Dz` en cero
  siempre tiene `Sign::Zero`, así que `DzFloat::from_dzfixed` de un valor
  exactamente cero pierde cualquier signo que tuviera antes de llegar
  ahí; solo `DzFloat::zero(true)`/`neg()` aplicado directamente a un cero
  ya construido preserva `-0`).
- No hay pruebas de estrés de los límites exactos del exponente (`biased
  <= 0` / `biased >= 143`) más allá de los tests unitarios de overflow y
  underflow básicos.
