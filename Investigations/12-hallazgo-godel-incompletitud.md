# Hallazgo — Gödel: qué puede decir de la incompletitud un instrumento que solo evalúa términos cerrados

Cuarta extensión de la serie posterior al cierre (`09`–`11` fueron los
"ingredientes" para evitar π). Ésta no es de la misma familia: la abre un vídeo
divulgativo del canal Math Rocks sobre los dos teoremas de incompletitud de
Gödel (numeración por potencias de primos, diagonalización, Rosser 1936,
Gentzen 1936, y la tesis de que una IA no puede tener un procedimiento final,
efectivo, consistente y completo para la aritmética). El propio vídeo concede
que el límite vale igual para los humanos. Sus afirmaciones laterales (que una
empresa resolvió Navier–Stokes, una formalización en Lean 4 en 2026) **no se
verificaron y nada de lo que sigue depende de ellas**.

Fecha de apertura: 2026-10-07. Protocolo de `00-plan-investigacion.md`: toda
cifra de puertas sale de `target/release/dz-cli.exe` con el comando al lado,
backend MVL salvo que se diga otra cosa; los literales sin más son **dozenales**
(`--input-base 10` para decimal); `--gates` imprime puertas y profundidad.

**Recordatorio no negociable.** Gödel no se "resuelve". Los teoremas son
resultados demostrados sobre sistemas formales efectivamente axiomatizados,
consistentes y que interpretan una aritmética mínima; ninguna calculadora, base
numérica, métrica ni arquitectura de puertas los revoca. La pregunta que sí
admite respuesta es qué puede *medir* este instrumento de la maquinaria que
rodea a los teoremas (numeración, comprobación local de pruebas, cotas), y qué
es de nuevo **no trasladable** a él.

---

## 0. Resumen del veredicto (para quien no lea el resto)

| pregunta | veredicto | casilla |
|---|---|---|
| ¿Qué sistema formal es la calculadora? | Un **evaluador de modelo**, no un sistema deductivo. Su fragmento exacto decide igualdades y desigualdades de términos cerrados sin cuantificadores: decidible y completo, Gödel no le aplica. Su fragmento aproximado (`sqrt`, `sin`, …) **no decide**: devuelve respuestas marcadas `~`, y el signo de un término que vale 0 sale `-1` o `1`. | (IV) para "¿le aplica Gödel?"; el límite aproximado es de la implementación |
| ¿Cuánto cuesta numerar en base 12 frente a base 10? | Leer una numeración posicional en base 12 cuesta **0** puertas de conversión y su descodificación 0; en base 10, lectura **cuadrática** (segunda diferencia constante 1.784) y vuelta aún más cara. La numeración por potencias de primos descodifica con divisiones: 37,4 M de puertas para 6 símbolos frente a 0. | (III), y dependiente del instrumento |
| ¿Compra algo que los dos primeros primos sean los factores de 12? | **No medible.** `mod` por 2, 3, 4, 6 cuesta lo mismo que por 5, 7, ↋. Lo único real es una regularidad de ceros finales que cualquier base con factores 2 y 3 (6, 12, 24…) comparte. | (III), coincidencia de vocabulario |
| ¿Se puede comprobar consistencia acotada? | Una comprobación **por candidato** sí (195.367 puertas por `x*x = M`); el cuantificador y el bucle **no existen en el instrumento**, viven en el script que lo invoca. Coste lineal en el número de candidatos, y el número de candidatos es exponencial en la longitud de la prueba. | (IV): no realizable con este instrumento |

La conclusión de fondo es la de `11`: **Gödel no tiene π, ni métrica, ni base
numérica**, así que la pregunta de la serie ("¿qué parte es L₂, qué parte es
base 12?") casi no se traslada. Lo único genuinamente de base 12 es el coste de
la *aritmetización*, que depende de qué representación usa la máquina.

---

## 1. Predicciones registradas antes de medir

Se anotan antes de ejecutar nada, como `P1–P6` del plan.

- **PG1.** El fragmento exacto de la calculadora es decidible y completo y no
  admite el argumento de Gödel (no tiene cuantificadores ni bucle no acotado);
  el fragmento aproximado (trascendentes) no decide igualdades.
- **PG2.** Leer una numeración posicional en base 12 cuesta cero de conversión
  y profundidad constante (254); en base 10 el coste crece de forma
  superlineal con la longitud del mensaje.
- **PG3.** Descodificar una numeración por potencias de primos (que necesita
  división restauradora, el circuito más caro) cuesta al menos 10 veces su
  codificación para un mensaje de 6 símbolos, y la razón crece con la longitud.
- **PG4.** Que 2 y 3 sean los dos primeros primos y los factores de 12 **no**
  producirá ninguna ventaja medible en la división del instrumento
  (`mod` por 2, 3, 4, 6 dentro de un factor 2 del coste de `mod` por 5, 7, ↋,
  sin orden sistemático).
- **PG5.** Una comprobación acotada "ningún `x < N` cumple `R(x)`" cuesta un
  número de puertas lineal (a lo sumo ligeramente superlineal) en `N`, con la
  profundidad de **un solo** candidato, porque los candidatos son independientes.
- **PG6.** Cualquier regularidad que se encuentre en la numeración por primos
  leída en dozenal será de casilla (III), compartida con otras bases con los
  mismos factores primos.

---

## 2. ¿Qué sistema formal es la calculadora? (pregunta 1)

### 2.1 Qué decide

Las hipótesis del primer teorema son tres: teoría efectivamente axiomatizada,
consistente, y capaz de interpretar la aritmética de Robinson Q (necesita `+`,
`×` **y** cuantificación no acotada sobre los naturales: las sentencias Σ₁
`∃x φ(x)` con `φ` sin cuantificadores no acotados). La calculadora tiene `+` y
`×`, y no tiene cuantificadores.

Lo que hace es evaluar términos cerrados (sin variables) y devolver un valor.
Eso no es una teoría con axiomas y reglas de inferencia sino la evaluación de
un modelo; la igualdad y la desigualdad cerradas se reducen a evaluar y comparar.
En el fragmento exacto (modo racional; `+ − × ÷`, `^` con exponente entero no
negativo, `floor`, `abs`, `min`, `max`, `sign`, `sqrt` exacto de cuadrados
perfectos) el procedimiento termina siempre:

```
$ dz-cli eval "sign(2^3*3^1*5^4 - 8820)" --gates
0
gates: cycle=93192 min=154628 max=114772 lit=78696 thr=19152 total=460440 depth=1446 recon=0
$ dz-cli eval "sqrt(4)-2" --gates
0
gates: ... total=25321222 depth=14348 recon=0
```

El primero decide la igualdad cerrada `15000 = 15000` (dec.) con 460.440
puertas. La teoría de igualdades y desigualdades cerradas es decidible y
completa por una razón trivial: toda sentencia cerrada atómica se evalúa a
verdadero o falso. Ahí Gödel no entra.

### 2.2 Dónde no decide, y no por Gödel

En el fragmento aproximado el instrumento marca el resultado con `~` y **no
certifica** nada:

```
$ dz-cli eval "sign(sqrt(2)*sqrt(2)-2)" --gates
~-1
gates: ... total=99966306 depth=49241 recon=0
$ dz-cli eval "sign(sin(1)^2+cos(1)^2-1)" --gates
~1
gates: ... total=250615738 depth=69760 recon=0
```

Los dos términos valen exactamente 0 y el instrumento responde `-1` y `1`,
ambos con la marca de aproximado. Decidir la igualdad con 0 de expresiones
cerradas con `exp`, `sin`, `abs` es un problema conocido indecidible en
general (Richardson, 1968), y es una indecidibilidad **distinta** de la de
Gödel (no viene de la aritmética de enteros sino de las funciones reales). Lo
que importa aquí: la marca `~` es la forma honesta que tiene la implementación
de decir "no decido". No es un fallo de pureza; es un límite de truncar series.

### 2.3 Qué habría que añadir para que Gödel aplique

Un solo ingrediente: **una cuantificación existencial no acotada sobre los
naturales** (un operador `μ`, un bucle `while`, o `∃x` sin cota). Con eso las
sentencias Σ₁ son expresables y Q queda interpretada; el vaciado completo
(representabilidad de las funciones recursivas, lema diagonal) es trabajo
posterior que **no se hizo** (§7). Con cuantificadores *acotados* `∃x < N` el
sistema sigue siendo decidible por exhaustión finita: el salto de decidible a
indecidible es exactamente quitar la cota. Es el "local contra global" del
vídeo: comprobar **una** prueba es un cálculo cerrado; descartar **todas** es
un `∀` no acotado.

**Casilla:** (IV) para la pregunta "¿le aplica Gödel a la calculadora?": la
calculadora no es un sistema formal en el sentido del teorema, de modo que la
pregunta no se traslada; lo que sí se puede clasificar es el límite del
fragmento aproximado (de la implementación, no de la lógica).

---

## 3. Coste de numerar en base 12 y en base 10 (pregunta 2)

Mensaje de prueba: los dígitos `3, 1, 4, 1, 5, 9` (comunes a las dos bases), de
1 a 6 símbolos. Una numeración de Gödel es cualquier inyección computable
cadenas → naturales; dos candidatas:

- **(a) Potencias de primos**: `2^3·3^1·5^4·7^1·11^5·13^9…`. Los primos
  decimales 11 y 13 se escriben `↋` y `11` en dozenal; los exponentes
  `3,1,4,1,5,9` son iguales en ambas bases.
- **(b) Posicional**: la cadena leída como numeral en base `B`.

### 3.1 Lectura de la posicional: tabla A, el control equivocado, y tabla C

El primer intento comparó la cadena `m` leída en base 12 con la misma cadena
leída en base 10. Es un control **malo**: son valores distintos y el coste del
evaluador depende del valor (salió no monótono: 72.510 para `3141` frente a
72.278 para `31415` en base 10). El control correcto es el de `docs/10`: el
mismo **valor** leído en cada base.

Tabla C (`dz-cli eval "<m>" --input-base 10 --mode fixed --gates` frente a
`dz-cli eval "<dozenal del mismo valor>" --mode fixed --gates`):

| cifras | cadena decimal | valor dozenal | total leyendo base 10 | total leyendo base 12 | **la lectura** | prof. b10 | prof. b12 |
|--:|---|---|--:|--:|--:|--:|--:|
| 1 | `3` | `3` | 8.989 | 8.989 | **0** | 254 | 254 |
| 2 | `31` | `27` | 24.811 | 22.421 | **2.390** | 254 | 254 |
| 3 | `314` | `222` | 28.656 | 22.092 | **6.564** | 540 | 254 |
| 4 | `3141` | `1999` | 72.510 | 59.988 | **12.522** | 878 | 254 |
| 5 | `31415` | `1621↋` | 72.278 | 52.014 | **20.264** | 1.261 | 254 |
| 6 | `314159` | `13197↋` | 102.881 | 73.091 | **29.790** | 1.689 | 254 |

Las diferencias sucesivas de la lectura son 2.390, 4.174, 5.958, 7.742, 9.526:
la **segunda diferencia es constante, 1.784**, que es exactamente el sobrecoste
de leer `12` frente a `10` en la tabla de `CLAUDE.md`. Es decir, la lectura
crece de forma cuadrática en el número de cifras (el acumulador de Horner crece
a cada paso y cada multiplicación por diez cuesta proporcional a su longitud).
La profundidad en base 12 es 254 **constante**; en base 10 crece con la
longitud. Esto es medido sobre **un** mensaje; no se afirma que la segunda
diferencia sea idéntica para cualquier cadena de dígitos.

### 3.2 Descodificar la posicional

`dz-cli chain <n> --input-base 12 --gates` y `dz-cli chain <v> --input-base 10 --gates`
(`v` = el valor dozenal de la cadena decimal):

| cifras | vuelta base 12 | vuelta base 10 |
|--:|--:|--:|
| 1 | 0 | 48 |
| 2 | 0 | 15.149 |
| 3 | 0 | 46.190 |
| 4 | 0 | 76.683 |
| 5 | 0 | 152.428 |
| 6 | 0 | 218.465 |

**Lectura honesta de este 0.** No significa que descodificar sea gratis en
base 12. Significa que **la representación interna del instrumento es la
numeración posicional en base 12**: devolver los símbolos es leer la memoria,
no un cálculo. En una máquina cuya representación nativa fuera decimal los
papeles se invertirían: leer la numeración posicional en base 10 sería gratis y
la de base 12 pagaría la conversión. Esa inversión es una inferencia, **no se
midió** (no hay instrumento decimal). Es un efecto de casilla (III) y de
instrumento, no una propiedad de la base 12.

Un matiz de aritmética de símbolos que no es nada profundo: una posicional con
símbolos `1..B−1` (sin el cero, para que la numeración sea inyectiva y no pierda
ceros a la izquierda) cabe en base 12 con 11 símbolos y en base 10 con 9. Un
lenguaje mínimo de aritmética de primer orden (`0, S, +, ×, =, ¬, →, ∀, (, ), x`)
tiene justo 11. Es una coincidencia de diseño del alfabeto y no hay nada que
medir; se anota para que nadie la lea como un resultado.

### 3.3 Potencias de primos: codificar y descodificar

Codificar es una expresión (`dz-cli eval "<expr>" --mode fixed --gates`);
descodificar no se puede escribir como expresión, porque no hay bucles. Se
ejecutó con un script externo que, para cada primo `p`, repite `mod N p`; si da 0
aplica `div N p` y cuenta (el bucle y el contador viven **fuera** del
instrumento; las puertas son las del binario, sumadas). Cada paso son dos
llamadas (`mod` y `div`), así que la cifra de descodificación es una **cota
superior**: un `div_rem` fusionado, que el CLI no expone, costaría
aproximadamente la mitad.

> **Corrección posterior (§10).** Se escribió "aproximadamente la mitad", y esa
> estimación era mala: al implementar `valuation` y repetir la medida, la
> descodificación de 6 símbolos bajó de 37.366.435 a 6.254.722 puertas (6×, no
> 2×). La causa es que `mod` del CLI pasa por `DzRational::rem` (cruce de
> productos y mcd), mucho más caro que un `div_rem` desnudo. La tabla de abajo se
> conserva tal como se publicó; las cifras vigentes están en §10.2.

| n | expresión (dozenal) | código `N` (dozenal) | cifras de `N` | codificar (puertas) | profundidad | descodificar (puertas) | llamadas | descod./codif. |
|--:|---|---|--:|--:|--:|--:|--:|--:|
| 1 | `2^3` | `8` | 1 | 109.669 | 815 | 413.088* | 8 | 3,8 |
| 2 | `2^3*3^1` | `20` | 2 | 167.851 | 1.002 | 626.724 | 10 | 3,7 |
| 3 | `…*5^4` | `8820` | 4 | 428.484 | 1.198 | 2.512.354 | 19 | 5,9 |
| 4 | `…*7^1` | `50920` | 5 | 542.267 | 1.490 | 4.049.816 | 22 | 7,5 |
| 5 | `…*↋^5` | `333↋2↋10↊0` | 10 | 1.102.836 | 1.982 | 12.967.748 | 33 | 11,8 |
| 6 | `…*11^9` | `689↋28↋40↊↋177286↊0` | 19 | 2.735.092 | 4.011 | 37.366.435 | 52 | 13,7 |

\* la fila 1 descodifica con los primos `2 3` porque el código de un solo símbolo
solo puede tener exponente en el 2; el segundo primo se comprueba y da 0.

(Los códigos se imprimen con `X` para diez y `E` para once en la salida del CLI;
aquí van con `↊`/`↋`.)

Tres lecturas:

1. **PG3 se cumple en el punto registrado** (n = 6: 13,7×) y la razón crece con
   `n`, pero **no** en todos los `n`: a n = 3 es 5,9× y a n = 4 es 7,5×. La
   predicción era "al menos 10× para 6 símbolos", no para cualquier longitud.
2. Frente a la posicional en base 12, descodificar potencias de primos pasa de
   **0** a 37.366.435 puertas (para 6 símbolos). Es la misma jerarquía de coste
   que `docs/05` encontró entre multiplicación y división restauradora.
3. **El código crece más que el mensaje**: 1, 2, 4, 5, 10, 19 cifras para 1 a 6
   símbolos frente a `n` cifras de la posicional. La numeración por primos es
   inyectiva pero poco densa; casi ningún entero es un código válido.

### 3.4 ¿Compra algo que 2 y 3 sean los factores de 12?

Dos medidas, ambas contra la intuición de "dividir por 2 o 3 debería ser fácil
en dozenal".

**(i) `mod`.** El instrumento divide con división restauradora genérica, no con
la regla del último dígito. `dz-cli mod <N> <p> --gates` sobre `N = 8820` y sobre
`N = 689↋28↋40↊↋177286↊0`:

| `p` | `N = 8820` | `N` de 19 cifras |
|--:|--:|--:|
| 2 | 227.148 | 1.389.598 |
| 3 | 331.181 | 1.524.990 |
| 4 | 240.415 | 1.491.431 |
| 6 | 268.217 | 1.450.428 |
| 5 | 290.061 | 1.369.978 |
| 7 | 284.122 | 1.435.799 |
| ↋ | 302.644 | 1.281.692 |

Las filas de 2, 3, 4, 6 no son más baratas que las de 5, 7, ↋ ni en un orden
sistemático: el cociente máximo/mínimo es 1,46 en la primera columna y 1,19 en
la segunda, y el más barato de la segunda es `p = ↋`. **PG4 se cumple.** El
hecho matemático (divisibilidad por 2, 3, 4, 6 se decide con el último dígito,
porque `12 = 2²·3`) es cierto, pero **el instrumento no lo explota**: no hay
circuito que lo haga. Existe `is_even_digit` (`crates/dz-gates/src/circuits/parity.rs`),
que por lectura del código son 6 `lit` y 6 `max` (12 invocaciones; **no se midió
con el binario**, que no lo expone), y no hay un análogo para 3, 4, 6. La
propuesta de §6 era construirlos; está hecho y medido en §10.3, y lo que sigue
sobre `mod` describe el instrumento **antes** de esa adición.

**(ii) Ceros finales.** Sí hay una regularidad, real y medible con el binario.
El número de ceros finales de un código en base 12 es `min(⌊a/2⌋, b)`, donde `a`
y `b` son los exponentes de 2 y 3 (es la valuación 12-ádica parcial):

| expresión | código (dozenal) | `a` | `b` | ceros finales | `min(⌊a/2⌋, b)` |
|---|---|--:|--:|--:|--:|
| `2^1*3^1*5^1` | `26` | 1 | 1 | 0 | 0 |
| `2^2*3^1*5^1` | `50` | 2 | 1 | 1 | 1 |
| `2^1*3^2*5^1` | `76` | 1 | 2 | 0 | 0 |
| `2^4*3^2*5^1` | `500` | 4 | 2 | 2 | 2 |
| `2^3*3^1*5^4` | `8820` | 3 | 1 | 1 | 1 |

(Comandos: `dz-cli eval "<expresión>" --mode fixed`.) Es gratis de leer en
posicional. Pero en base 10 los ceros finales dan `min(a, c)` con `a` y `c` los
exponentes de 2 y 5, y en base 6 darían `min(a, b)`: **cualquier base da la
valuación parcial en los primos que la dividen**. No es especial de 12, ni
siquiera de la elección de los primos del vídeo (con base 6 o base 24 coincide
igual). **PG6 se cumple**: casilla (III), coincidencia de vocabulario entre
"los dos primeros primos" y "los factores de 12". El enlace es el mismo que
`11` §6 documentó para ℤ₁₂ ≅ ℤ₂ × ℤ₃: estructura real, sin ventaja de cálculo
añadida respecto a cualquier otro módulo.

---

## 4. Comprobación acotada de consistencia (pregunta 3)

El segundo teorema prohíbe una prueba **interna y global** de `Con(F)`, es
decir un `∀` sobre todos los códigos de prueba. La versión acotada, "ninguna
prueba de `0 = 1` con código `< N`", es una conjunción finita de predicados
primitivo-recursivos: un cálculo finito. ¿Lo puede expresar la calculadora?

**No.** Las expresiones no tienen bucles ni cuantificadores, ni siquiera
acotados, ni listas. Lo que sí se puede hacer es poner el bucle **fuera**, en el
script que invoca al binario, y medir el coste por candidato. Se usó un
predicado de juguete con la forma de `∃x < N R(x)`, con `R(x)` = `x*x = M`
comprobada como `sign(x*x - M) = 0` (la enumeración de candidatos la hace el
script, y solo se publican las puertas del binario).

Coste por candidato, un solo caso:

```
$ dz-cli eval "sign(X3*X3-8909)" --gates
0
gates: cycle=38676 min=66082 max=48969 lit=33480 thr=8160 total=195367 depth=578 recon=0
```

Escaneo acumulado para `M = 8820` (dozenal; 15.000 dec., **no** es cuadrado, así
que recorre todo el rango) y para `M = 8909` (15.129 dec. = 123², se encuentra
en el candidato nº 123):

| candidatos (dec.) | acumulado `M = 8820` | factor | acumulado `M = 8909` |
|--:|--:|--:|--:|
| 12 | 1.973.449 | | 2.124.200 |
| 24 | 4.242.695 | 2,15 | 4.528.907 |
| 48 | 9.016.727 | 2,13 | 9.567.139 |
| 96 | 19.227.031 | 2,13 | 20.267.907 |
| 144 | 29.844.813 | | (halla `x = ↊3` en el nº 123: 26.470.981) |

Cada duplicación de candidatos multiplica el coste por 2,13-2,15: lineal con
una ligera subida porque el candidato crece. **PG5 se cumple** en puertas. La
profundidad máxima por candidato sube de 675 a 764 en todo el rango; esa
profundidad **no se suma** entre candidatos en la medida que cada uno es
independiente, y por tanto un desenrollado paralelo tendría la profundidad de
un candidato más una reducción OR. Eso es un argumento, **no una medida**: el
instrumento no ejecuta en paralelo.

### 4.1 Qué cambia con una prueba de verdad

Con una numeración por primos cada candidato `c` tiene que descodificarse antes
de comprobarse: el coste por candidato es **al menos** el de §3.3 (37,4 M de
puertas a 6 símbolos, y la comprobación sintáctica de la prueba va aparte). Con la
posicional en base 12 esa parte vale 0 en este instrumento, y el coste es el de
la comprobación. La elección de numeración mueve el coste **por candidato**;
**no** toca el número de candidatos, que es `11^L` para pruebas de `L` símbolos
sobre un alfabeto de 11 (más aún con la numeración por primos, que es poco
densa y obliga a recorrer enteros que no son códigos). Esa cifra la dejo
simbólica a propósito: no se calculó con el binario.

El otro eje: lo que se comprueba es siempre **verdadero y finito**. `Con_N(F)` es
una sentencia Π₁ acotada y demostrable; el teorema de Gödel habla de `Con(F)`
con `N` sin cota. Ningún `N` finito, por grande que sea, se acerca a ella.

**Casilla:** (IV), **no realizable con este instrumento**: no es que el
resultado sea negativo, es que la calculadora no tiene el cuantificador sobre el
que descansa la pregunta. Lo que sale del experimento es solo el coste de un
paso, en puertas.

---

## 5. Lo que ha aprendido la investigación sobre el instrumento

1. **Es un evaluador, y un buen sitio para ver dónde empieza la indecidibilidad:**
   en el límite entre el fragmento exacto y el aproximado, no en la aritmética de
   enteros. Ese límite no es el de Gödel.
2. **La marca `~` es la forma de decir "no decido"**, y `sign` sobre un término
   aproximado devuelve un signo sin garantía (§2.2). Cualquier comparación de
   orden construida sobre trascendentes hereda esa falta de garantía.
3. **No hay división fusionada** (`div_rem`) expuesta, así que toda cifra de
   descodificación publicada es una cota superior (de hasta 2×, según se creyó;
   en realidad ~6×, ver §10.2).

---

## 6. Qué habría que construir para ir más lejos (implementado, ver §10)

> **Estado: implementado.** Las cuatro propuestas se construyeron después de
> este documento (solo CLI, desde puertas, sin ningún atajo por otra
> representación). Se conserva el texto original de cada propuesta; dónde está
> cada cosa y qué se midió con ella, en §10. El `μ`-operador **no** se
> implementó, a propósito.

- `divrem a b`, que devuelve cociente y resto a la vez, para medir descodificación
  sin el doble coste.
- `valuation(n, p)`, valuación `p`-ádica exacta, para descodificar el exponente en
  una sola llamada.
- Un circuito de divisibilidad por 2, 3, 4, 6 que lea **solo el último dígito**
  (un decodificador de 12 ramas, por analogía con `is_even_digit`): permitiría
  medir cuánto vale de verdad la regla del último dígito frente a la división
  restauradora.
- Un combinador acotado `forall/exists x < N` implementado como un bucle en
  `dz-api` (nunca como operador nativo), para que el barrido de §4 sea una sola
  llamada con su propio `GateCount`. Tampoco introduciría un cuantificador no
  acotado: seguiría decidible.
- Un `μ`-operador es el ingrediente que haría aplicar a Gödel; no se recomienda
  como mejora de la calculadora, solo se nombra como lo que cambiaría la
  clasificación de §2.

---

## 7. Lo que no funcionó

1. **El primer control de lectura estaba mal planteado** (§3.1): leer la misma
   cadena en las dos bases compara valores distintos, y el coste del evaluador
   depende del valor, con lo que salió no monótono. Hubo que rehacerlo con el
   mismo valor en las dos bases.
2. **Un literal dozenal tomado por decimal.** En un primer ensayo de igualdad
   se escribió `sign(2^3*3^1*5^4 - 15000)` esperando 0 y salió `-1`:
   `15000` es dozenal y no es 15.000 decimal. Es la trampa de la
   guía del CLI y se repitió aquí. La comparación correcta usa `8820`, que es
   15.000 decimal.
3. **El instrumento no puede expresar el barrido** de §4. Todo lo que se publica
   sobre consistencia acotada se midió con el bucle fuera del binario; no es una
   capacidad de la calculadora.
4. **No se pudo medir la regla del último dígito.** Para 2 existe un circuito que
   se leyó, no se midió; para 3, 4 y 6 no existe. La pregunta "¿la regla del
   último dígito es barata en puertas?" quedó sin respuesta medida en esta
   versión; **se respondió en §10.3** tras construir el circuito.
5. **La descodificación es una cota superior** (`mod` + `div` por paso); la razón
   descod./codif. de §3.3 se creyó inflada hasta 2× y estaba inflada ~6×
   (corrección en §10.2).
6. **No hay veredicto sobre CRT ni la tabla**: todas las cifras son del backend
   MVL. Para estos circuitos las cinco primitivas son idénticas por
   construcción, y solo `recon` separaría a CRT; no se midió.
7. **Las afirmaciones laterales del vídeo no se verificaron**, y no se intentó.

---

## 8. Lo que no se hizo

- No se implementó ningún sistema formal: ni el predicado de prueba `Prf`, ni el
  lema diagonal, ni una sentencia de Gödel. El experimento de §4 usa un predicado
  de juguete; la afirmación "una prueba real cuesta al menos el coste de descodificar"
  es una cota inferior sobre un cálculo no hecho.
- No se trató a Rosser ni a Gentzen (aunque el vídeo los cita), ni la teoría de
  tipos o la aritmética de Heyting.
- No se midió una máquina decimal: la inversión de §3.2 es un argumento.
- Una sola cadena (`3,1,4,1,5,9`) y un solo conjunto de primos; no hay barrido de
  mensajes, y la segunda diferencia constante de §3.1 es de **esa** cadena.
- No se probó la función `β` de Gödel como numeración alternativa.
- No hay medida de tiempo de ejecución, solo puertas y profundidad.
- El número de candidatos `11^L` de §4.1 no se evaluó con el binario.

---

## 9. Veredicto

**Gödel no se resuelve y esta investigación no lo pretendió.** Hay tres cosas
genuinas:

1. **(IV)** La calculadora es un evaluador de términos cerrados, decide un
   fragmento decidible y completo, y le faltan los cuantificadores no acotados
   que activan el teorema. El único límite de decisión que se midió dentro del
   instrumento (el de `sqrt`/`sin`) es de la implementación y de la teoría de
   funciones reales, no de Gödel. Para la tesis del vídeo sobre IA: el teorema
   obliga a cualquier sistema formal efectivo, y este instrumento no aporta
   evidencia a favor ni en contra de nada más.
2. **(III), dependiente de la máquina.** Numerar con una posicional en la base
   en que la máquina representa los números cuesta 0 de lectura y de vuelta
   (aquí, base 12); en la otra base, cuadrático (segunda diferencia 1.784) y,
   a la vuelta, mayor. Numerar con potencias de primos cuesta división
   restauradora, 37.366.435 puertas para 6 símbolos frente a 0. No es una
   ventaja de la base 12 en sí.
3. **(III), coincidencia.** Que los dos primeros primos sean los factores de 12
   no compra nada medible en el instrumento (`mod` cuesta igual); lo único real
   son los ceros finales, y cualquier base con esos factores los comparte.

Predicciones: **PG1 se cumple** (con la precisión de que el fragmento aproximado
da un signo erróneo marcado `~`), **PG2 se cumple**, **PG3 se cumple en el punto
registrado y falla para `n ≤ 4`**, **PG4 se cumple**, **PG5 se cumple**, **PG6 se
cumple**.

> **Actualización (§10).** Tras implementar `valuation` y re-medir, **PG3 se
> retira**: la descodificación a 6 símbolos costó 6.254.722 puertas (no
> 37.366.435) y la razón con la codificación es 2,3×, no 13,7×. La jerarquía
> división ≫ lectura posicional se mantiene; el factor 10× registrado no. Y la
> conclusión de §3.4 sobre `mod` se mantiene para `mod` pero ya no para el
> instrumento: `divisible` decide por el último dígito en 36-39 puertas (§10.3).

Los comandos exactos están junto a cada tabla; los scripts que repitieron las
llamadas (descodificación y barrido) no se guardaron en el repositorio, solo sus
totales.

---

## 10. Implementado: lo que se construyó y lo que se re-midió

Después de redactar §1-§9 se implementaron las cuatro propuestas de §6, solo en
CLI, con las reglas del repositorio: ningún valor dozenal pasa por `u64` ni por
base 10; los contadores (el exponente de la valuación, la variable ligada) son
`Dz` que avanzan con `add`; la tabla de dígitos finales se sintetiza con
puertas. `cargo test --workspace --release` da 269 pruebas (42 nuevas), clippy y
el lint de pureza limpios, y `dz-cli eval "1+1" --mode fixed --gates` sigue en
24.083 puertas y profundidad 441.

### 10.1 Dónde está cada cosa

| qué | dónde |
|---|---|
| Circuito de divisibilidad por el último dígito y sus filas sintetizadas | `crates/dz-gates/src/circuits/divisibility.rs` |
| `valuation`, `divisible`, `last_digit_divisible` (genéricos sobre `GateNet`) | `crates/dz-int/src/number_theory.rs` |
| `divrem`, `valuation`, `divisible`, `forall`/`exists` como petición de la API | `crates/dz-api/src/theory.rs` |
| Subcomandos `divrem`, `valuation`, `divisible`, `forall`, `exists` y `--var` | `crates/dz-cli/src/main.rs` |
| Funciones `valuation(n,p)` y `divisible(n,d)` en expresiones (solo CLI) | `crates/dz-eval/src/builtins.rs`, lista `CLI_ONLY_FUNCTIONS` |

Decisiones que conviene dejar escritas:

- **`divrem`**: un solo `div_rem`; el resto toma el signo del dividendo
  (truncado), como en `Dz::div_rem`. Un divisor cero es error de cálculo.
- **`valuation(n, p)`**: `div_rem` repetido hasta que el resto no es cero; el
  signo de `n` se ignora. `n = 0` es error ("todas las potencias dividen a
  cero"), y `p < 2` también (0, 1 y negativos); si se dan ambos, se informa
  primero `p`. Devuelve además el cofactor. Siempre paga un `div_rem` que falla,
  el que prueba que el exponente es máximo. No usa todavía el circuito del
  último dígito como filtro previo.
- **`divisible(n, d)`**: para `d` en `{2, 3, 4, 6, 12}` decide con el último
  dígito; **para cualquier otro `d` cae a `div_rem`** (no lo rechaza) y la
  respuesta dice qué método corrió (`method: last-digit` o `div_rem`). Se eligió
  caer a la división y no rechazar porque la pregunta "¿`d` divide a `n`?" tiene
  respuesta para todo `d`; lo que cambia es el coste, y el campo `method` lo
  hace visible. `d = 0` es error.
- **El circuito**: `MAX_j MIN(lit(j, x), fila[j])` con 12 `lit`, 12 `min` y 11
  `max`: **35 invocaciones y profundidad 13 para cualquier divisor**. La fila
  (qué dígitos son múltiplos de `d`) no está tecleada: se obtiene sumando `d`
  consigo mismo doce veces con `half_adder` y marcando los dígitos visitados con
  `lit`/`max`, una vez, con un `MvlGates` desechable en un `OnceLock` (mismo
  patrón y misma razón que `lo_hi_table`, el error del commit 18565f8). Para `d`
  que divide a 12 el recorrido visita exactamente los múltiplos de `d`; para otros
  `d` visitaría un subgrupo mayor, por eso esas filas nunca se usan. Reconocer qué
  `d` es cuesta 1 a 4 `lit` más (36 a 39 en total).
- **`forall`/`exists`**: `dz-cli forall <N> "<cuerpo>"` y `exists`. La variable
  ligada recorre `0..N-1`. **El cuerpo es verdadero si y solo si su valor es
  distinto de cero**, o una sola relación `lhs = rhs` (también `!=`, `<`, `<=`,
  `>`, `>=`) decidida con `compare` de puertas. El contador es un `Dz` que
  avanza con `add` y la parada la decide `compare` contra `N`; cada candidato es
  el árbol del cuerpo con la variable sustituida por los dígitos del contador (no
  un reemplazo de texto: `abs(i)` ve el contador, `pi` no se toca) y se evalúa en
  modo racional exacto. Un cuerpo aproximado (`sin(i) = 0`) es **error**, por lo
  visto en §2.2: un cuantificador necesita una decisión. Cortocircuita: `exists`
  se detiene en el primer testigo (`witness`), `forall` en el primer
  contraejemplo (`counterexample`); `checked` dice cuántos candidatos corrieron.
  `N = 0` da `forall` verdadero y `exists` falso. Sigue **acotado y por tanto
  decidible**: no hay forma no acotada, y el `μ`-operador no se implementó.
- **Nombre de la variable**: por defecto `i`, cambiable con `--var <nombre>`. Debe
  ser una tira de letras que el lexer lea como **nombre** y no como número: `x`,
  `t`, `e`, `X`, `T`, `E` (y tiras como `xe`) son dígitos y se rechazan con el
  motivo; también se rechaza un nombre del catálogo (`pi`, `sin`) y un nombre que
  el alfabeto de presentación configurado lea como dígito (con `i` como símbolo
  de diez, `i` deja de poder ser variable).
- **Profundidad del cuantificador**: cada candidato abre su propio `DepthScope` y
  se informa el **máximo** de un candidato (con el contador como entrada
  primaria): es la profundidad del bucle desenrollado en paralelo. El árbol
  AND/OR que combinaría los `N` veredictos **no** se cuenta.

### 10.2 Descodificación re-medida con `valuation`

`bash Investigations/12-redecode.sh` (tras `cargo build -p dz-cli --release`):
una llamada `dz-cli valuation <N> <p> --gates` por primo, pasando el cofactor al
siguiente. Se compara con la cota publicada en §3.3 (`mod` + `div` por paso).

| n | llamadas (antes → ahora) | descodificar antes | descodificar ahora | factor | codificar (§3.3) | descod./codif. ahora |
|--:|--:|--:|--:|--:|--:|--:|
| 1 | 8 → 2 | 413.088 | 62.905 | 6,6 | 109.669 | 0,57 |
| 2 | 10 → 2 | 626.724 | 88.073 | 7,1 | 167.851 | 0,52 |
| 3 | 19 → 3 | 2.512.354 | 400.249 | 6,3 | 428.484 | 0,93 |
| 4 | 22 → 4 | 4.049.816 | 684.386 | 5,9 | 542.267 | 1,26 |
| 5 | 33 → 5 | 12.967.748 | 2.231.155 | 5,8 | 1.102.836 | 2,02 |
| 6 | 52 → 6 | 37.366.435 | 6.254.722 | 6,0 | 2.735.092 | 2,29 |

**Corrección visible.** (1) En §3.3 se estimó que un `div_rem` fusionado costaría
"aproximadamente la mitad"; costó **una sexta parte**. `mod` del CLI pasa por
`DzRational::rem`, que cruza productos y calcula un mcd antes de dividir, y eso
pesaba más que la división. (2) **PG3 no sobrevive a la re-medida**: se registró
"descodificar cuesta al menos 10× codificar a 6 símbolos" y se cumplía (13,7×)
solo porque la cifra estaba inflada; con `valuation` la razón a 6 símbolos es
2,3×, y para n ≤ 3 la descodificación es *más barata* que la codificación. Lo que
**sí** sobrevive: la razón crece con `n` (0,57 → 2,29), y frente a la posicional
en base 12 (0 puertas de descodificación) la brecha sigue siendo de millones de
puertas (6.254.722 frente a 0). La lectura honesta de PG3 es "se cumplía en la
medida vieja y se retira"; la jerarquía de coste (división ≫ lectura posicional)
se mantiene, pero ya no con el 10× registrado. Las tablas de §3.3 y el veredicto
de §9 se dejan como se publicaron; esta sección manda donde discrepen.

### 10.3 Divisibilidad por el último dígito frente a la división restauradora

La comparación que en §3.4 no se pudo hacer. Hay que separar dos cosas, porque la
llamada completa del CLI está dominada por **leer los operandos**.

**(a) El circuito solo**, `cargo run -p dz-int --release --example divisibility_cost`
(`MvlGates` fresco, sin evaluador ni lectura de texto; la `n` termina en 0, así
que `d | n` siempre se cumple):

| cifras de `n` | `d` | último dígito | `div_rem` | `div_rem` / último dígito |
|--:|--:|--:|--:|--:|
| 2 | 2 | 36 | 20.563 | 571 |
| 2 | 3 | 37 | 10.396 | 281 |
| 4 | 4 | 38 | 45.334 | 1.193 |
| 4 | 10 (doce) | 38 | 79.461 | 2.091 |
| 8 | 6 | 39 | 104.894 | 2.690 |
| 16 | 2 | 36 | 218.620 | 6.073 |
| 32 | 3 | 37 | 466.164 | 12.599 |
| 32 | 10 (doce) | 38 | 754.677 | 19.860 |

El circuito cuesta **36-39 puertas, independiente del número de cifras** de `n`
(solo lee el último), y su profundidad propia es 13. La división crece con las
cifras, así que la razón sube de ~300-600 a ~12.000-20.000 entre 2 y 32 cifras.
Esto contesta la pregunta de §3.4: **la regla del último dígito es órdenes de
magnitud más barata, y el instrumento no la explotaba porque no existía el
circuito**, no porque no se pudiera.

**(b) La llamada completa del CLI**, `dz-cli divisible <n> <d> --gates`:

| llamada | método | puertas | profundidad |
|---|---|--:|--:|
| `divisible 8820 4` | último dígito | 55.365 | 254 |
| `divisible 8820 6` | último dígito | 58.424 | 254 |
| `divisible 8820 5` | `div_rem` | 109.522 | 294 |
| `divisible 8820 7` | `div_rem` | 113.924 | 294 |
| `eval 8820 --gates` (solo leer `n`) | | 45.001 | 254 |
| `eval 4 --gates` (solo leer `d`) | | 10.518 | 254 |
| `divisible 689↋28↋40↊↋177286↊0 4` (19 cifras) | último dígito | 272.438 | 254 |
| `eval 689↋28↋40↊↋177286↊0 --gates` (solo leer `n`) | | 262.074 | 254 |
| `mod 8820 4` (§3.4, vía `DzRational::rem`) | | 240.415 | 1.178 |

**Lectura.** La profundidad 254 de las llamadas con `method: last-digit` es la
**de leer los operandos**, no la del circuito (13 propios): el evaluador paga ~45k
puertas por leer `8820` y ~10k por `4`, y eso es casi todo lo que cuesta
`divisible 8820 4` (55.365 frente a 45.001 + 10.518; el circuito aporta unas
decenas). Con 19 cifras ocurre lo mismo: 272.438 frente a 262.074 solo de
lectura. Dicho de otro modo, **a nivel de CLI la ganancia contra `mod` es 4,3×
(55.365 frente a 240.415) en 4 cifras, pero contra un `div_rem` desnudo es 571× o
más**; la diferencia es cuánto del coste es leer texto. Con `d = 5` o `7` la
llamada cae a `div_rem` (109.522 y 113.924): el doble, y con profundidad 294, que
sí es la de la división.

El hallazgo de §3.4 **se corrige en su espíritu, no en su cifra**: ahí se midió
que `mod` por 2, 3, 4, 6 cuesta lo mismo que por 5, 7, ↋, y eso sigue siendo
cierto de `mod`. Lo que se afirmó ("el instrumento no explota el hecho") era
cierto entonces y deja de serlo con `divisible`.

**Casilla.** (III): el efecto existe solo por `12 = 2²·3`, y en base 10 el mismo
circuito valdría para divisores de 10 (2, 5), no para 3. No es un resultado sobre
Gödel; mejora una operación que la numeración por primos usa.

### 10.4 Barrido acotado, ahora como una sola llamada

`dz-cli exists 100 "i*i = 8820" --gates` da `holds: false`, `checked: 100`
(100 dozenal = 144 candidatos), 22.032.014 puertas, profundidad 566. Con testigo,
`dz-cli exists 1000 "i*i = 8909" --gates` da `witness: ↊3`, `checked: ↊4`,
19.888.702 puertas, profundidad 566. Y `dz-cli forall 20 "i*i >= i" --gates` da
`holds: true`, 1.915.212 puertas, profundidad 520.

Comparación con §4: el barrido de 144 candidatos costaba 29.844.813 puertas con el
bucle en `bash` (un proceso por candidato, cada uno leyendo sus operandos y
evaluando `sign(...)`) y cuesta 22.032.014 con el bucle dentro de `dz-api`. **No es
una comparación limpia**: el cuerpo y el modo de leer difieren (`sign(x*x-M)`
frente a `i*i = 8820` con comparación racional), así que solo se puede decir que
el orden de magnitud y la linealidad se conservan: `exists 10 "i*i = 8820"` cuesta
1.182.183 y `exists 20 "i*i = 8820"` 2.715.814 (factor 2,30 al duplicar los
candidatos, ligeramente superlineal como en §4). La profundidad informada (476 y
506) es la de un candidato y apenas crece, que es PG5 medida ahora dentro de la
herramienta y no por un guion externo.

### 10.5 Lo que sigue sin hacerse

- `valuation` no usa el circuito del último dígito como filtro previo (para
  `p` en `{2, 3, 4, 6, 12}` evitaría el `div_rem` que falla); sería el siguiente
  paso natural y no se midió.
- No hay `μ`-operador, y es deliberado: su presencia es lo que haría aplicar
  Gödel, y la calculadora se queda decidible.
- Las medidas son del backend MVL; con CRT los contadores primitivos son iguales
  por construcción y solo cambiaría `recon`.
- La profundidad del cuantificador no incluye el árbol que combina los `N`
  veredictos.
