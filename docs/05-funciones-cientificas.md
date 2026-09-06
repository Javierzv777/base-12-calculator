# 05 — Funciones científicas dozenales (`dz-sci`)

Estado: **implementado**. `sqrt`, `exp`, `ln`, `log`, `sin`, `cos`, `tan`,
`asin`, `acos`, `atan`, las constantes `pi`/`tau`/`euler` y el bloque exacto
(`abs`, `sign`, `floor`, `ceil`, `round`, `trunc`, `min`, `max`) están
disponibles en el REPL, en el modo de una invocación del CLI y en la GUI.

PLAN.md §10bis-E dejaba las trascendentes fuera de alcance ("requieren CORDIC
dozenal — es un proyecto propio"). Lo que realmente bloqueaba no era la
matemática sino la **forma del código**: el lenguaje de expresiones no tenía
sintaxis de llamada, `DozOp` operaba solo sobre `DzFixed` y nunca llegaba a
invocarse desde `evaluate`, y no existía una capa donde meter análisis
numérico que no fuera ni un circuito (capa 0) ni una representación numérica
(capa 2). Este documento describe la capa que faltaba y los algoritmos que
ahora viven en ella.

---

## 1. Dónde vive cada cosa

```
dz-cli / GUI Tauri   →  dz-api    (capa 5, sin estado)
                        dz-eval   (capa 4: Expr::Call + FnRegistry)
                        dz-sci    (capa 2.5: análisis numérico)   ← este documento
                        dz-num    (capa 2: DzFixed / DzRational)
                        dz-int, dz-gates
```

- **`crates/dz-sci`** no conoce el AST, ni el REPL, ni la GUI: son funciones
  ordinarias sobre `DzFixed` (`sin(g, x, scale)`).
- **`dz-eval::functions`** define el trait `DozFunction` y el `FnRegistry`;
  **`dz-eval::builtins`** es la *tabla* que ata un nombre y una aridad a cada
  algoritmo de `dz-sci`. Añadir una función es una fila más en esa tabla.
- **`dz-parse`** ganó identificadores y `Expr::Call`; el resto de la cadena
  (api, CLI, GUI) descubre las funciones por el catálogo del registro, así que
  registrar una función la hace aparecer en el `:funcs` del REPL, como
  subcomando (`dz-cli sin 1`) y como tecla de la GUI **sin tocar ninguna
  interfaz**.

---

## 2. La regla de pureza, aplicada al análisis numérico

Ningún operador aritmético nativo toca un valor dozenal, tampoco aquí. El lint
(`tools/lint-purity.ps1`, regla 4) prohíbe `*`, `%` y `/` nativos en todo
`crates/dz-*/src`, y `dz-sci` pasa en verde. Consecuencias concretas y visibles
en el código:

- **Los denominadores de las series son contadores dozenales.** Un término
  como `x^(2k+1)/((2k)(2k+1))` no calcula `2*k+1` en `usize`; lleva un
  `DzFixed` que avanza con `add` gate-compuesto y multiplica sus dos factores
  con `mul` gate-compuesto. El factorial nunca se forma: cada término es el
  anterior por `x²` dividido por el siguiente par de enteros.
- **Dividir entre dos es multiplicar por `0;6`.** Exacto en base 12, un solo
  dígito. Lo mismo `0;3` para un cuarto. En base 10 haría falta `0.5`, en base
  2 un desplazamiento; aquí es un literal.
- **Escalar por una potencia de doce es mover el punto Humphrey**, es decir,
  cambiar el campo `scale`: `Vec`/`usize` bookkeeping, cero puertas. Esto es lo
  que hace barato el paso de reducción de `ln`.
- Los enteros pequeños que las fórmulas necesitan (16 y 239 en Machin, el 144
  del límite de `exp`) se construyen **contando** con `small_usize_to_dz`, la
  misma técnica ya usada en la fase 5 para los exponentes DFP.

---

## 3. Algoritmos

Todas las funciones aceptan una escala objetivo (dígitos dozenales
fraccionarios) y trabajan internamente con `GUARD_DIGITS = 4` dígitos extra,
truncando en cada paso — el truncamiento solo pierde, nunca oscila, y eso
mantiene simple el análisis del error. El **resultado final sí se redondea**
half-to-even a la escala pedida (`util::round_to_scale`): truncar la respuesta
es lo que convertía `tan(pi/4)` en `0;↋↋↋↋↋↋↋↋` en vez de `1`. El empate se
detecta comparando el resto contra media unidad en el último lugar, y media
unidad es exacta en base doce (`0;6` por el ulp), así que la comparación no
necesita ninguna división.

### 3.1 `sqrt` — Newton con semilla por conteo de dígitos

`y ← (y + x/y)/2`, con la semilla tomada del **número de dígitos** de `x`:
`12^⌈m/2⌉` si `|x| ≥ 1` (m = dígitos enteros), `12^-⌊j/2⌋` si `|x| < 1`
(j = ceros fraccionarios a la izquierda). Ambas son cotas superiores de la
raíz, que es lo que Newton quiere: desde arriba la sucesión decrece
monótonamente y el criterio de parada es "dejó de decrecer".

Detalle deliberado: el resultado redondeado se **eleva al cuadrado y se compara**
con la entrada. Si coinciden, la respuesta se marca exacta — `sqrt(9)` da `3`
sin marca de aproximación, que es lo que un usuario espera de una calculadora.

### 3.2 `exp` — reducción por mitades

Se divide el argumento por dos hasta `|x| ≤ 1/2` (multiplicando por `0;6`), se
suma la serie de Taylor ahí, y se eleva al cuadrado tantas veces como
divisiones se hicieron. La precisión de trabajo crece un dígito por cada
cuadrado, porque cada uno duplica el error acumulado.

Los argumentos negativos se calculan como `1/e^|x|`: la serie alternada pierde
dígitos por cancelación, el recíproco de un resultado bien condicionado no.
`|x| > 144` (una gruesa) se rechaza con `Overflow`: `e^144` ya necesita medio
centenar de dígitos enteros.

### 3.3 `ln` — la base 12 hace la mitad del trabajo gratis

`x = m · 12^(±e)` con `m ∈ [1,12)` es **exacto y gratuito** (mover el punto), y
entonces `ln(x) = ln(m) ± e·ln(12)`. Solo `ln(m)` cuesta una serie:

1. raíces cuadradas repetidas hasta que el argumento baja de `1;3` (≈1,25),
   usando `ln(m) = 2^t · ln(m^(1/2^t))`. Tres o cuatro raíces cubren todo
   `[1,12)`, y cada raíz son unos pocos pasos de Newton — mucho más barato que
   los ~175 términos extra que necesitaría la serie sin reducir;
2. la forma `atanh`: `ln(z) = 2(u + u³/3 + u⁵/5 + …)` con `u = (z−1)/(z+1)`,
   cuyos términos ya decaen por un factor `u² < 0;1` en cada paso.

### 3.4 `log` — base doce, exacta en las potencias de doce

`log(x)` es el logaritmo **en base 12** (el equivalente dozenal de la tecla
`log` de una calculadora decimal); `log(b, x)` toma la base explícita.

Las potencias exactas de doce se responden exactamente: `log(100) = 2` (`100`
dozenal es una gruesa). El camino por serie daría `1;↋↋↋…`, correcto al dígito
pero indistinguible de un bug a ojo. La comprobación es estructural —la
magnitud normalizada es un único dígito `1`—, no aritmética.

### 3.5 `sin`, `cos`, `tan`

Reducción del ángulo módulo `2π` (restando un número entero de vueltas, con
`trunc` sobre el cociente) hasta `[−π, π]`, y luego las series alternadas
habituales con el esquema de recurrencia descrito en §2. `tan` es el cociente;
si el coseno trunca a cero a la precisión de trabajo, se devuelve división por
cero, que es la respuesta honesta para un múltiplo impar del ángulo recto.

### 3.6 `atan`, `asin`, `acos`

`atan` pliega los argumentos mayores que uno con `atan(z) = π/2 − atan(1/z)` y
luego aplica la identidad del ángulo mitad
`atan(z) = 2·atan(z/(1+√(1+z²)))` hasta que `z ≤ 0;3`, donde la serie converge
en una docena de términos. `asin(x) = atan(x/√(1−x²))` con los extremos
`±1` respondidos directamente, y `acos(x) = π/2 − asin(x)`.

---

## 4. Constantes: sintetizadas, memoizadas, y con coste cero para el usuario

`pi` sale de la fórmula de Machin `π = 16·atan(1/5) − 4·atan(1/239)`, calculada
con las mismas operaciones `DzFixed` que todo lo demás (nada transcrito de una
tabla decimal). `ln12 = ln(2) + ln(6)` — partir el doce en dos factores dentro
de `[1,12)` rompe la recursión que tendría pedirle `ln(12)` a `ln`, cuya
reducción por radix necesita precisamente esa constante. `euler = exp(1)`.

Se calculan **una vez por escala**, sobre un `MvlGates` desechable cuyo
`GateCount` nunca se lee, y se memoizan. Es el mismo patrón que estableció
`lo_hi_table` para la tabla SOP del multiplicador, por las mismas dos razones:
la medición del usuario no debe contaminarse con la síntesis de una constante,
y no debe depender de si esta llamada fue la primera. Cachear un *resultado* es
legítimo; importar el *significado* de una operación desde aritmética nativa no
lo es (CLAUDE.md).

Efecto medible, y es el que se buscaba:

```
$ dz-cli eval "pi" --mode fixed --scale 12 --unicode --gates
~3;184809493↋92
gates: cycle=0 min=0 max=0 lit=0 thr=0 total=0
```

Cero puertas: el usuario está leyendo una constante ya sintetizada.

---

## 5. Modelo de exactitud

| Clase | Funciones | Qué devuelve |
|---|---|---|
| Exactas siempre | `abs`, `sign`, `floor`, `ceil`, `round`, `trunc`, `min`, `max` | valor exacto, también en modo racional (implementación propia sobre `DzRational`, no pasa por punto fijo) |
| Exactas a veces | `sqrt`, `log` | exacto y verificado en cuadrados/potencias perfectas; aproximado en el resto |
| Aproximadas | `exp`, `ln`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `pi`, `tau`, `euler` | redondeo half-even a la escala de trabajo |

La marca viaja por toda la expresión: `sin(1) + 1/3` es aproximada, `1/3 + 1`
no. Y en modo racional un resultado aproximado **no se imprime como fracción**
—una fracción de doce dígitos afirmaría una exactitud que no existe—; se
muestra en punto fijo con el prefijo `~`. El redondeo de `round` es
half-to-even, igual que el DFP, y el empate se detecta comparando `2·resto`
contra el denominador, sin dividir otra vez (`docs/03-formato-dfp.md`).

Verificación: `crates/dz-sci/tests/accuracy.rs` compara contra `f64` como
oráculo externo (permitido solo en tests, PLAN.md §8.3) para las once
funciones y las cuatro constantes, incluida la reducción de ángulos
(`sin(20)`), los inversos (`ln(exp(4))`) y la coincidencia entre backends.

---

## 6. Resultados medidos

Backend MVL, modo fijo, escala 12 dígitos fraccionarios, `--gates`:

| Expresión | puertas | tiempo (release) |
|---|--:|--:|
| `1+1` | 24.083 | — |
| `4;6*3` | 120.444 | — |
| `sqrt(2)` | 32.657.729 | 141 ms |
| `cos(1)` | 24.428.203 | — |
| `sin(1)` | 27.139.541 | 557 ms |
| `exp(1)` | 35.323.997 | — |
| `ln(2)` | 226.222.343 | 401 ms |
| `atan(1)` | 352.673.718 | 588 ms |
| `pi` | 0 | — |

Los tiempos incluyen el arranque del proceso y la síntesis de las constantes
(que no aparece en el conteo de puertas, §4); `sin(1)` a escala 24 sube a
927 ms. Lectura honesta: una trascendente cuesta entre dos y cuatro órdenes de
magnitud más que una multiplicación, y el reparto no es uniforme —`ln` y
`atan` pagan las raíces cuadradas de su reducción, cada una con su propia
iteración de Newton llena de divisiones restauradoras—. La división sigue
siendo la operación cara de toda la pila, igual que lo era en la fase 5.

---

## 7. ¿Y el CORDIC?

PLAN.md daba por hecho que las trascendentes exigían CORDIC dozenal. No hizo
falta: series con reducción de argumento bastan y reutilizan las operaciones ya
construidas, sin añadir tablas nuevas ni un circuito nuevo. CORDIC sigue siendo
interesante, pero como **experimento comparativo** de la misma clase que
"CRT vs MVL" o "ripple vs lookahead": cambia una multiplicación por
desplazamientos y sumas, así que la pregunta medible es si eso baja el conteo
de puertas de `sin` en base 12, y en cuánto. Queda anotado como trabajo futuro,
no como prerrequisito.

---

## 8. Limitaciones conocidas

- **Reducción de ángulos grandes**: `sin(10^20)` pierde dígitos significativos,
  como en cualquier implementación de precisión finita. La reducción se hace a
  `escala + 4` dígitos; ángulos enormes degradan en vez de rechazarse.
- **`^` sigue aceptando solo exponentes enteros no negativos.** Un exponente
  fraccionario se expresa con `exp(y * ln(x))`. No se cambió el operador para
  no alterar el comportamiento (y los tests) de la fase 5.
- **Componer funciones acumula error**: cada llamada redondea a la escala del
  usuario, así que `exp(ln(5))` vuelve como `4;↋↋↋↋↋↋↋↋` y no como `5`. Se
  arregla pidiendo más dígitos (`--scale`), o —si alguna vez molesta— haciendo
  que el evaluador propague dígitos de guarda entre llamadas anidadas, que es
  un cambio en `dz-eval`, no en los algoritmos.
- **Sin hiperbólicas, sin `atan2`, sin grados.** Son filas nuevas en la tabla
  de `builtins.rs`, no diseño nuevo.
- **`e` no es el número de Euler en el lenguaje**: es el alias ASCII del dígito
  once. La constante se llama `euler`. Misma razón por la que `x` y `t` no
  pueden ser nombres de variable: son alias de dígitos (ver la regla de
  identificadores en `dz-parse/src/lexer.rs`).
