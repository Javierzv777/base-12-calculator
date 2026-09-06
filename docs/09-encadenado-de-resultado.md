# 09 — Encadenar sobre el resultado

Pedido: después de un Enter o un `=`, si el siguiente carácter que entra es un
operador, se reemplaza todo lo que hay en el input por el resultado. Es el
encadenado clásico de calculadora: `5+3` Enter, luego `*` deja `8*` en la caja.

Estado de partida: rama `feat/phase7-research-panel`, commit `b4ab19d`.

---

## 1. Lo que hay hoy

`evaluate()` (`app/src/main.ts:129`) evalúa, pinta `result`, guarda
`lastResult = response.display` (canónico) y **deja el input intacto**. Así que
hoy, tras `5+3` Enter, teclear `*` da `5+3*`.

`display()` traduce del alfabeto canónico al configurado (`docs/07`), que es lo
que el usuario lee. El texto a reinyectar es **el que se ve**, no el canónico:
si alguien configuró `A`/`B`, la caja debe recibir `A`, que es lo que su lexer
espera tras la traducción de vuelta.

---

## 2. Las dos trampas, verificadas contra el binario

### 2.1 La tilde de aproximado no es texto válido

`dz-api` antepone `~` al display cuando el resultado es aproximado
(`crates/dz-api/src/lib.rs:428`). Ese carácter **no lo acepta el lexer**:

```
$ dz-cli eval "~0;X12" --mode fixed
error: lex error at 0: unexpected character '~'
```

Reinyectar el display tal cual dejaría la caja con una expresión que no evalúa.
La `~` se quita antes de sembrar. Lo que se pierde es la marca de que el número
era una aproximación truncada — y eso hay que decirlo, no esconderlo: el
encadenado sigue con el número truncado, no con el valor exacto. La insignia de
«aproximación» que ya pinta `paintNotes` cubre el aviso mientras el resultado
está en pantalla.

### 2.2 Un resultado compuesto necesita paréntesis

El display no es siempre un número suelto. En modo racional —**el de por
defecto**— es una fracción, y sembrarla cruda cambia el significado:

```
$ dz-cli eval "583/2^2"      → 583/4
$ dz-cli eval "(583/2)^2"    → 284209/4
```

`583/2` seguido de `^2` no es «el resultado al cuadrado». Lo mismo con un
resultado negativo: `-5^2` es `-(5^2)`, no `(-5)^2`.

**Regla: se siembra entre paréntesis salvo que el resultado sea un átomo.** Es
átomo si es solo dígitos, los dos símbolos configurados de diez y once, y el
punto Humphrey `;`. Todo lo demás —barra, signo, cualquier otra cosa— va
envuelto. Envolver de más es inofensivo (`(8)*3` evalúa igual que `8*3`);
envolver de menos da un número equivocado en silencio, que es justo el tipo de
fallo que este proyecto no se puede permitir.

---

## 3. Diseño

### 3.1 Cuándo queda armado

Tras una evaluación **con éxito** (Enter o `=`). Un error no arma nada: no hay
resultado que sembrar.

Se desarma con **el primer carácter que entre**, sea o no operador, y también si
el input cambia por cualquier otra vía. Solo el primero cuenta: `5+3` Enter,
`*`, `2`, `+` debe dar `8*2+`, no volver a sembrar en el segundo `+`.

### 3.2 Qué cuenta como operador

Los operadores binarios del lenguaje: `+`, `-`, `*`, `/`, `^`. El conjunto sale
del parser (`crates/dz-parse/src/parser.rs`), no de una lista inventada aparte.

El `-` es ambiguo a propósito: puede ser «menos el resultado» o «empezar un
número negativo». Se trata como operador, que es lo que hace cualquier
calculadora y lo que el pedido implica.

### 3.3 Por dónde puede llegar el carácter

**No solo por el teclado físico.** El teclado en pantalla tiene botones de
operador y escribe por `insert()` (`app/src/main.ts:150`), así que el
encadenado tiene que funcionar igual pulsándolos. Un manejador de `keydown`
solo cubriría la mitad de las vías — el mismo error que la lista negra del
`docs/08` §2.3.

### 3.4 Interacción con el resto

- **El deshacer de la herramienta base 10** se retira solo: sembrar escribe el
  `value` por una vía que no es su propia escritura, y su lista blanca ya cubre
  eso (`docs/08` §2.3). Correcto y deseado.
- **El historial y la memoria** no se tocan.
- **Ningún cálculo nuevo**: sembrar es mover texto. No se llama a `calc`.

---

## 4. Fuera de alcance, y por qué se pregunta

**Qué pasa si el primer carácter tras Enter es un dígito.** Una calculadora de
verdad empieza una entrada nueva y descarta lo anterior. Hoy el input conserva
la expresión, así que teclear `7` tras `5+3` Enter da `5+37`, que no es útil
para nadie. **No se cambia en esta fase**: el pedido habla solo del caso del
operador, y alterar el caso del dígito es una decisión de producto aparte que
conviene tomar a la vista. Queda anotado aquí.

## 5. Lo implementado

Todo el cambio está en `app/src/main.ts`; ni un fichero de `crates/` se tocó, y
tampoco `base10.ts`, `keypad.ts`, `history.ts`, `research.ts`, `alphabet.ts` ni
`api.ts`. No hizo falta: el punto §3.3 se resolvió interceptando en `insert()`,
que es por donde ya pasaban todos los botones.

### 5.1 Las piezas

Un bloque nuevo entre `showError` y `evaluate` (`app/src/main.ts:130-247`):

| Qué | Dónde |
|---|---|
| `OPERATORS`, el conjunto que siembra | `main.ts:144` |
| `chainSeed` / `chainAnchor` / `chainWatching` | `main.ts:147-154` |
| `disarmChain()` | `main.ts:156` |
| `atomicResult()` — la regla del paréntesis | `main.ts:175` |
| `armChain()` — quita la `~`, decide envoltura, ancla | `main.ts:204` |
| `chainOn()` — consume un carácter, devuelve si lo tragó | `main.ts:235` |

Enganches: `evaluate` arma al final del camino de éxito y desarma en el `catch`
(`main.ts:251-276`); `insert()` empieza llamando a `chainOn` (`main.ts:279`); el
`keydown` del input hace lo mismo para el teclado físico (`main.ts:645-655`) y un
`input` listener desarma para todo lo demás (`main.ts:660`).

### 5.2 La tilde (§2.1)

`armChain` recibe **el texto que se ve** —`display(response.display)`, el mismo
string que se pinta en el resultado grande, ya en el alfabeto configurado— y le
quita la `~` inicial con un `replace(/^~/, "")` antes de decidir nada. `lastResult`
(canónico) no se usa para sembrar. Consecuencia asumida y ya dicha en §2.1: el
encadenado continúa desde el número truncado.

### 5.3 La precedencia (§2.2)

`atomicResult` recorre el texto carácter a carácter: pasan `0`-`9`, el punto
Humphrey y los dos símbolos configurados de diez y once. Los símbolos se leen de
la sesión viva **y además en su versión `display()`**, porque en una máquina sin
U+218A/U+218B lo que aparece en el resultado son los alias ASCII y no los glifos
de la sesión; sin esa segunda entrada, un `↊` legítimo se habría envuelto sin
necesidad. Todo lo que no pase —la barra de una fracción, un signo, un exponente—
se siembra entre paréntesis.

### 5.4 Las dos vías de entrada (§3.3)

`chainOn` es un embudo único y las dos vías lo llaman:

- **Teclado en pantalla, pad científico y recuperar memoria** → `insert()`, que
  lo llama antes de tocar el `value`. `keypad.ts` no se modificó.
- **Teclado físico** → el `keydown` del input, para cualquier `event.key` de
  longitud 1 sin `ctrl`/`meta`/`alt` (una combinación con modificador es un
  atajo, no un carácter).

Un operador siembra y devuelve `true` (el llamador no escribe además el
carácter: el `keydown` hace `preventDefault`, `insert` sale antes); cualquier
otro carácter no siembra, pero gasta la oferta igual.

### 5.5 Desarmado sin agujeros

Misma forma que la lista blanca de `docs/08` §2.3, y por la misma razón: el
`input` listener cubre teclear, borrar y pegar de forma **síncrona** (por eso
`5+3` Enter, `*`, `2`, `+` da `8*2+` aunque las teclas lleguen más rápido que un
frame), y un bucle de `requestAnimationFrame` comparando contra `chainAnchor`
cubre las escrituras programáticas, que no disparan ningún evento: restaurar del
historial, la tecla `C`, una conversión de base 10, o la ruta que alguien añada
dentro de seis meses. El bucle solo corre mientras hay oferta armada.

### 5.6 Desviaciones

- **El conjunto de operadores es más ancho que los cinco del enunciado.** El
  pedido decía `+ - * / ^`, pero la instrucción operativa era sacarlo del parser,
  y el parser tiene un binario más: `%` (`Token::Percent` → `Expr::Mod`,
  `parser.rs:73`), que además es una tecla del teclado en pantalla. Están también
  las dos grafías unicode que el lexer acepta (`×` y `÷`, `lexer.rs:190-191`),
  para que encadenar no dependa de cuál de dos sinónimos llegó. Dejar `%` fuera
  habría sido justo la «lista inventada aparte» que §3.2 prohíbe.
- Nada más. El caso del dígito (§4) sigue igual: `5+3` Enter y `7` da `5+37`.

### 5.7 Validado en pantalla

Con `npm run tauri build -- --no-bundle` y la ventana real (`PrintWindow` con
`PW_RENDERFULLCONTENT`, `SendKeys` para teclear, `mouse_event` para clicar):

| Caso | Resultado |
|---|---|
| (a) `5+3` Enter, `*` físico | caja `8*`; con `2` Enter → `14` |
| (b) lo mismo con el `×` del teclado en pantalla | caja `8*`, luego `8*2` |
| (c) racional, `23X/E` Enter (`23↊/↋`), `^`, `2` | caja `(23↊/↋)^2` → `54684/↊1` |
| (d) fijo, `sin(1)` Enter (`~0;↊1208↊↊92234`), `*`, `2` | caja sin `~` y sin paréntesis → `1;824159964468` |
| (e) tras Enter, `2` | `5+32`: no siembra, desarma |
| (f) `5+3` Enter, `*`, `2`, `+` | `8*2+`: solo el primero siembra |
| (g) `1/0` Enter (error), `*` | `1/0*`: no siembra nada |
| (h) alfabeto `A`/`B`, `5+6` Enter (`B`), `*`, `2` | caja `B*2` → `1A` |

Comprobado además, no asumido (§3.4): con el deshacer de base 10 armado
(`25` → `21`, con su flecha visible), evaluar y sembrar retira la oferta —
la etiqueta «base 12» y la flecha desaparecen en la misma pulsación.

Limitación del arnés, no del código: `SendKeys "{^}"` con la distribución de
teclado de esta máquina entrega `&`, así que el caso (c) se tecleó con la tecla
`^` del teclado en pantalla. El `&` que llegó por error dio, de paso, una
comprobación gratis de (e): no sembró y gastó la oferta.

### 5.8 Verificación

`node app/tests/base10.test.ts` → 108/108. `npm run build` (typecheck + bundle)
limpio. `cargo test --workspace --release` → 190 pasados, 0 fallos (control de
alcance: no se tocó Rust). `pwsh tools/lint-purity.ps1` verde.
