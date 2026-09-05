# 00 — Fundamentos

## Qué se construyó (Fases 0-5 completas)

- **Fase 0**: workspace Cargo con seis crates (`dz-gates`, `dz-int`, `dz-num`,
  `dz-parse`, `dz-eval`, `dz-cli`), compilando limpio bajo `cargo build --workspace`
  y `cargo clippy --workspace --all-targets` sin warnings.
- **Fase 1**: `dz-gates` — trait `DozGates` (`cycle`, `min`, `max`, `lit`, `thr`)
  con tres backends (`MvlGates`, `CrtGates`, `TableGates`). Diferenciales
  144/144 exhaustivos, tanto de los cinco primitivos como de suma y
  multiplicación de un dígito, contra un oráculo de tabla independiente.
- **Fase 2**: semi-sumador, sumador completo, ripple-carry multi-dígito y un
  multiplicador shift-add — todos construidos exclusivamente por composición
  de las cinco primitivas (ver `01-diseno-sumador.md`).
- **Fase 3**: `dz-int::Dz` — enteros de precisión arbitraria, signo +
  magnitud, con resta implementada como suma vía complemento a 12
  (radix complement), comparación vía el comparador `ge` compuesto por
  puertas, y división restauradora (schoolbook) en base 12.
- **Fase 4**: `dz-parse` (lexer/parser dozenal, nunca usa
  `str::parse::<iN>`), `dz-eval` (evaluador de AST con manejo de errores
  como valores y el punto de extensión `DozOp`), y `dz-cli` (REPL). El
  criterio de "hecho" de la Fase 4 se cumple textualmente:

  ```
  dz> 4;6 * 3
  11;6
  ```

  con los tres backends (`mvl`, `crt`, `table`) dando el mismo resultado.

- **Fase 5**: tres piezas, todas en `dz-num`:
  - `DzFixed` — punto fijo dozenal (`magnitud / 12^escala`), suficiente
    para que `1÷3 = 0;4`, `1÷4 = 0;3`, `1÷6 = 0;2` salgan exactos.
  - `DzRational` — racionales exactos (`num/den`, `den` siempre positivo,
    reducidos por MCD de Euclides usando solo `Dz::div_rem`). **Es el modo
    por defecto del REPL**: `1/3` imprime `1/3` exacto, no una
    aproximación truncada. Incluye detección de periodicidad en base 12
    (`terminates_in_base12`, factoriza el denominador reducido sacando
    solo 2 y 3) — el REPL avisa explícitamente `(periodica en base 12)`
    en vez de mentir con un truncamiento silencioso.
  - `DzFloat` — punto flotante dozenal (DFP): signo + exponente sesgado de
    dos dígitos (rango crudo 0..143, con 0/143 reservados para
    cero/infinito-NaN) + mantisa de doce dígitos, con redondeo half-even
    correcto (ver `03-formato-dfp.md` para el diseño completo).

  El REPL soporta los tres modos (`:mode rational|fixed|dfp`), verificado
  en vivo (ver la sección "Sesión de REPL" más abajo).

## El contrato de pureza, tal como quedó implementado

`Digit` (`crates/dz-gates/src/digit.rs`) es un newtype `struct Digit(u8)` con
el campo sin `pub`, así que **ni siquiera el resto de `dz-gates`** puede
construir un `Digit` por literal de tupla fuera del módulo `digit.rs` — el
compilador lo impide, no solo una convención. `Digit` deriva únicamente
`Clone, Copy, PartialEq, Eq, Hash, Debug`; no implementa `Add`, `Sub`, `Mul`,
`Div`, `Ord`, `PartialOrd` ni `From<u32>`/`From<u8>`.

La única función que expone el `u8` interno es `Digit::tag()`. Se llama en
exactamente tres lugares de todo el workspace (verificado por
`tools/lint-purity.ps1`, que falla si aparece en cualquier otro sitio):

1. `dz-gates/src/backends/{mvl,crt,table}.rs` — la propia definición de las
   tablas de verdad de cada backend (no hay otra forma de *escribir* una
   tabla de verdad sin, en algún punto, mirar el símbolo que se está
   tabulando).
2. `dz-parse/src/alphabet.rs` — la conversión símbolo-alfabeto (`char <->
   Digit`), análoga a `char::from_u32`, no una operación aritmética sobre
   una cantidad dozenal.
3. `crates/dz-int/tests/arithmetic.rs` — comparación contra el oráculo
   externo `num-bigint`, exclusivamente en código de test (PLAN.md
   Sección 8.3 lo autoriza explícitamente).

Todo lo demás — sumador, multiplicador, `Dz` (enteros), `DzFixed` (punto
fijo), el lexer, el parser, el evaluador — se construye combinando
`cycle`/`min`/`max`/`lit`/`thr` más aritmética nativa *solo* sobre
`usize` (índices de `Vec`, contadores de bucle, longitudes), tal como
autoriza la Sección 0 del plan.

## Alfabeto y notación

- Dígitos: `0 1 2 3 4 5 6 7 8 9 ↊ ↋` (U+218A, U+218B).
- Alias ASCII de entrada aceptados: `X`/`x`/`T`/`t` → diez, `E`/`e` → once
  (la salida siempre usa los glifos Unicode canónicos).
- Separador fraccionario: punto Humphrey `;` (`4;6` = 4 + 6/12 = 4.5 decimal).
- Agrupación de cuatro dígitos con `,` aceptada y tratada como cosmética
  (el lexer la ignora).

## Periodicidad: por qué la base 12 importa

12 = 2²·3 tiene más divisores pequeños que 10 = 2·5. Fracciones que
requieren periodo infinito en base 10 terminan exactamente en base 12:

| Fracción | Base 10 | Base 12 |
|---|---|---|
| 1/2 | 0.5 (exacta) | 0;6 (exacta) |
| 1/3 | 0.333... (periódica) | 0;4 (exacta) |
| 1/4 | 0.25 (exacta) | 0;3 (exacta) |
| 1/6 | 0.1666... (periódica) | 0;2 (exacta) |
| 1/5 | 0.2 (exacta) | 0;2497... (periódica) |

Verificado en el REPL (`dz-cli`, backend MVL):

```
dz> 1/3
0;4
dz> 1/4
0;3
dz> 1/6
0;2
```

(`1/5` no termina en base 12, ya que 5 no divide a 12; con la implementación
actual de punto fijo trunca a `DEFAULT_DIV_SCALE = 24` dígitos fraccionarios
en vez de detectar la no-terminación — una limitación conocida, ver la nota
de alcance).

## Conteos de puertas medidos

Ver `crates/dz-gates/examples/gate_counts.rs` (ejecutable con
`cargo run -p dz-gates --example gate_counts`) y `01-diseno-sumador.md`
para el desglose completo por operación.

## Alcance no cubierto en esta entrega

- Carry-lookahead dozenal y comparación de profundidad de circuito contra
  ripple-carry — Fase 8, pendiente.
- Multiplicador v2 (SOP sintetizado) — solo se implementó v1 (shift-add);
  ver `01-diseno-sumador.md` para por qué v1 ya es sorprendentemente caro
  y por qué esto hace que `DzRational`/`DzFloat` (que dependen de `Dz::mul`
  y de división, ambas construidas sobre el multiplicador) sean lentos en
  los tests de propiedades (ver la nota en
  `crates/dz-num/tests/rational.rs`: los rangos de valores de los
  `proptest` se mantuvieron pequeños a propósito por esto).
- Subnormales en DFP y "cero con signo" persistente — ver
  `03-formato-dfp.md`.
- Redondeo por operación en `evaluate_dfp` (actualmente evalúa la
  expresión completa en racionales exactos y redondea una sola vez al
  final) — ver `03-formato-dfp.md`.
- GUI Tauri — explícitamente fuera de alcance de esta tarea.
