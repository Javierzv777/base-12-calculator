# 08 — Conversión base 10 → base 12 (herramienta de entrada)

Pedido: al pegar texto en el input, detectar que **posiblemente** está en base 10
y ofrecer convertirlo. `12 + 11` → `10 + ↋`. Muy discreto, porque `12 + 11`
también es válido en base 12. La conversión cambia solo el contenido del input;
se acciona con un botón o un mini popup de sugerencia, nunca sola.

Aclaración del usuario, que gobierna el diseño: **lógica separada**, que no
reutilice lo ya implementado. Puede vivir en el repo, pero **no es parte de la
calculadora**: es una herramienta rápida de conversión.

---

## 1. Dónde vive, y por qué no en Rust

La conversión decimal → dozenal es aritmética (divisiones sucesivas entre doce,
o un Horner). Escribirla en Rust dentro de `crates/` choca de frente con la
regla 4 del lint de pureza, que prohíbe `*`, `%` y `/` nativos fuera de
`backends/` (`tools/lint-purity.ps1`). Esquivarlo pediría tocar el lint, que es
exactamente «lo ya implementado» que el pedido manda no tocar.

Y reutilizar `dz-int` (`mul`/`add` compuestos por puertas) está descartado por
el mismo pedido.

**Decisión: TypeScript, en `app/src/base10.ts`, módulo nuevo y aislado.** No
importa nada del proyecto y nadie lo importa salvo el punto de conexión del
input. Usa `BigInt` nativo, que es exacto para enteros de cualquier tamaño.

### 1.1 La tensión, dicha claro

`CLAUDE.md` dice que el frontend nunca decide semántica dozenal, y aquí un
módulo de JavaScript va a producir dígitos dozenales con `%` nativo — justo lo
que el proyecto entero existe para no hacer.

Lo que lo hace admisible es que **su salida no es un resultado**: es texto en la
caja de entrada, que el usuario todavía tiene que evaluar con el motor de
puertas de verdad. Es un asistente de tecleo, de la misma familia que un
buscar-y-reemplazar, no un camino de cálculo alternativo.

Para que eso no se difumine con el tiempo, tres barreras:

1. El módulo **no toca** `Response`, ni el historial, ni la memoria, ni el panel
   de investigación, ni ningún conteo de puertas. Solo escribe en el `value` del
   input.
2. Cabecera del fichero explicando qué es y qué no, y que su aritmética nativa es
   deliberada y está fuera del núcleo.
3. Nadie más lo importa. Si algún día un resultado sale de aquí, es un bug.

### 1.2 Regla de no-mezcla (la más importante de este documento)

**Esta herramienta no se mezcla con la lógica de la calculadora por ningún
motivo.** No es una capa nueva del proyecto ni una extensión de `dz-parse`: es
una utilidad de edición de texto que casualmente vive en el mismo repositorio.

Lo que eso obliga, en concreto:

- **Un fichero propio**, `app/src/base10.ts`, que **no importa nada** del
  proyecto: ni `api.ts`, ni `alphabet.ts`, ni `glyphs.ts`, ni tipos de Rust. Su
  única entrada es un `string` y su única salida es un `string`. El par de
  símbolos de §3.1 llega **como argumento**, no importando el módulo del
  alfabeto.
- **Comentario de cabecera explícito y en inglés** diciendo, sin rodeos: que es
  una herramienta aparte, que su aritmética con `%`/`/` nativos sobre `BigInt`
  sería una violación de la regla del repositorio si esto fuera código de la
  calculadora, que no lo es, y que **nada de la calculadora puede llamarla ni
  ella puede llamar a nada de la calculadora**.
- **Documentación separada**: este documento. En `CLAUDE.md` va **una sola
  línea** que la nombre y apunte aquí, bajo un epígrafe que diga que no forma
  parte de las capas — no se añade a la tabla de capas, ni a la lista de
  crates, ni a los resultados medidos, porque no es ninguna de esas cosas.
- **Sin punto de contacto lógico**: el único enlace es que un manejador de
  `paste` en `main.ts` le pasa un string y, si el usuario acepta, escribe el
  resultado en el input. Ese enlace es de UI, no de semántica.

Si alguien en el futuro necesita esta conversión *dentro* de un cálculo, la
respuesta correcta no es importar este módulo: es implementarla con puertas en
`dz-int`, como manda el principio del proyecto.

---

## 2. La heurística: cuándo sugerir (la parte delicada)

No existe ninguna señal que **pruebe** que un texto está en base 10: toda tira de
`0`-`9` es válida en las dos bases. Así que la sugerencia se gobierna por
restricción, no por certeza. Se sugiere solo si se cumplen **todas**:

1. **Llegó por un evento `paste`.** Nunca al teclear. Es la señal más fuerte que
   hay, y la dio el propio pedido.
2. **La conversión cambiaría el texto.** `5 + 3` es idéntico en ambas bases, así
   que sugerir ahí es puro ruido. Éste es el filtro que más silencio compra.
3. **El texto parece una expresión aritmética**: dígitos, operadores, paréntesis,
   espacios y nombres conocidos. Un pegado de prosa no dispara nada.

**Que el texto pudiera ser válido en base 12 no impide sugerir.** Quien sabe en
qué base está lo que pegó es el usuario, no la herramienta; `12 + 11` es
legítimo en ambas y aun así la sugerencia aparece. Lo único que la herramienta
decide es cuándo hablar sin molestar, y para eso le basta la condición 2.

### 2.0 Botón permanente, además del popup

El pedido decía «un botón **o** un mini popup». Son las dos cosas, y por la
misma razón: el usuario es quien sabe.

- **Botón siempre disponible** junto al input: convierte bajo demanda, sin
  heurística ninguna, aunque el texto se haya tecleado en vez de pegarse y
  aunque contenga glifos dozenales. Si no hay nada convertible, lo dice y no
  cambia nada.
- **Popup automático** solo cuando se cumplen las condiciones de arriba. Es el
  atajo, no la única puerta.

Un texto con `↊`/`↋` (o con el par configurado) no es interpretable como base 10
en su totalidad: esos símbolos no existen en base 10. El botón convierte
entonces las tiras decimales que sí puede y deja el resto intacto, diciendo qué
hizo. El popup automático no salta en ese caso, porque ahí la señal es débil de
verdad.

Señales que **suben** la confianza y se muestran como el motivo, cuando están:

- un punto decimal `.` usado como separador fraccionario (en dozenal es `;`);
- agrupación de miles en grupos de **tres** con coma (`1,000,000`); en dozenal la
  agrupación es de cuatro.

El popup dice por qué apareció («parece base 10: usa `.` como decimal»), o en su
defecto el motivo genérico. Sin motivo visible, una sugerencia que aparece sobre
un `12 + 11` legítimo es incomprensible.

### 2.1 Discreción

> **Derogado por el §2.4**: el popup automático se retiró a petición del
> usuario. Lo que sigue vigente de esta sección es que la conversión nunca
> ocurre sola y que tras convertir hay un deshacer a un clic.

- Se descarta con Esc o con la X, y **no vuelve a aparecer para ese mismo
  texto** en la sesión.
- Desaparece sola en cuanto el usuario sigue escribiendo.
- No roba el foco ni desplaza el layout: se ancla bajo el input, superpuesta.
- Nunca convierte automáticamente. Ni con Enter: hace falta un clic o un atajo
  explícito.
- Tras convertir, **deshacer** a un clic, que devuelve el texto pegado tal cual.

### 2.2 Caché de conversiones

Un `Map` de conversiones ya hechas, para no repetir el trabajo cuando el usuario
pega lo mismo dos veces, alterna deshacer/rehacer, o vuelve sobre un texto
largo.

**La clave no es el texto.** El mismo texto convertido con otra configuración da
otro resultado: `0.1` es `1/↊` en modo racional y `0;1249…` en fijo, con tantos
dígitos como diga la escala, y los símbolos de diez y once dependen del alfabeto
configurado. Una clave que fuese solo el texto serviría una respuesta correcta
para *otra* configuración — un caché envenenado, y de los silenciosos.

La clave es la **tupla completa de entrada**: texto + `ten` + `eleven` + `mode` +
`scale`. Es decir, todo lo que entra en `Base10Style`, sin excepción. Si mañana
`Base10Style` gana un campo, la clave lo gana también; conviene derivarla de
forma que olvidarlo sea difícil.

Cota: es un caché de sesión, en memoria, con un tope de entradas (unas 50) y
descarte de la más antigua. No se persiste — no hay nada que ganar guardándolo
entre arranques y sí un formato más que mantener.

El caché es una optimización, **nunca una fuente de verdad**: un acierto tiene
que devolver exactamente lo que devolvería recalcular. Un test debe fijar eso
comparando ambas rutas.

### 2.3 Deshacer caduca al editar

Deshacer solo está disponible **mientras el input no se haya tocado desde la
conversión**. En cuanto el usuario escribe, borra, pega otra cosa o edita de
cualquier forma, la opción desaparece y el botón vuelve a su estado normal.

El motivo es que deshacer restaura el texto de antes de convertir: si el usuario
ya editó encima, restaurar pisaría su trabajo sin avisar. Y un botón «Deshacer»
que lleva media hora ahí, después de veinte ediciones, ya no significa nada
reconocible.

**Cualquier entrada humana, de cualquier tipo, invalida el deshacer.** Y eso se
implementa como **lista blanca, no como lista negra**: el deshacer se arma solo
al convertir, y se desarma ante *cualquier* cambio del `value` del input que no
sea la escritura de la conversión misma.

La diferencia importa. Enumerar las vías a bloquear (teclear, borrar, pegar,
cortar, el teclado dozenal, el historial…) deja fuera las que nadie recordó
—arrastrar y soltar texto, dictado, autocompletar del sistema, una vía nueva que
se añada dentro de seis meses— y cada olvido es un deshacer que pisa el trabajo
del usuario en silencio. Invertida, la regla no tiene agujeros: si el `value`
cambió y no fue la conversión, el deshacer se cae.

Lo único que no cuenta es escribir el resultado de la conversión (o el deshacer
nacería muerto) y mover el cursor o seleccionar, que no cambian el `value`.

### 2.4 Fuera el popup; una sola fila con dirección invertible

Pedido posterior del usuario, y **deroga la sugerencia automática de los §2, §2.1
y §6.2-§6.4**: «el popup no me gusta, se puede ir, en lugar me gustaría que al
lado del pequeño botón de 10→12 hubiera un icono de invertir, similar al que
ponen en los traductores de Google, y si presiono ahí el 12 en el botón ocupa el
lugar del 10, y quitamos el popup que está muy ruidoso».

Lo que queda:

- **Nada aparece solo.** No hay popup, ni al pegar ni nunca. La heurística del §2
  ya no gobierna ninguna aparición.
- **Un botón que convierte bajo demanda** y, a su lado, **un botón de invertir
  (`⇄`)** que cambia la dirección: el rótulo pasa de `10→12` a `12→10` y el botón
  lee la caja al revés. Es el gesto del traductor, y por la misma razón que ahí:
  quien sabe en qué base está el texto es el usuario.
- **La dirección nueva obliga a la conversión inversa**, base 12 → base 10, que
  no existía. Es el espejo exacto de la de ida y vive en el mismo módulo.
- **La herramienta solo habla después de un clic**, y lo hace en una línea corta
  dentro de la misma fila, con el detalle completo en el `title`. Sin caja
  flotante, sin desplazar el layout, sin nada superpuesto al resultado.
- **El deshacer del §2.3 no cambia**: se arma con la conversión, es un botón
  `↺` en la misma fila, y caduca con cualquier cambio del `value`. Invertir la
  dirección también lo retira, porque la oferta era del otro sentido.

#### La conversión inversa

Simétrica en todo, incluida la aritmética:

- Un entero dozenal se lee por Horner en base doce y se escribe en base diez.
- `n / 12^k` **termina en base 10 si y solo si `3^k` divide a `n`**, por el
  espejo del argumento del §3.2: `12^k = 2^2k·3^k` y `10 = 2·5` no lleva ningún
  tres. `0;6` es `0.5` y `0;3` es `0.25`, pero `0;4` es un tercio y no termina.
- Si no termina, el modo decide igual que en la ida: `rational` da la fracción
  exacta (`0;4` → `1/3`) y `fixed`/`dfp` dan la expansión truncada a la escala
  (`0.333333333333`), con el `…` **solo** en la vista previa.
- Los identificadores se respetan con **la regla del propio lexer**
  (`CLAUDE.md`): una tira maximal de letras es nombre si y solo si alguna de sus
  letras no es alias de dígito. Con el par `X`/`E`, `XE` es número (131) y `exp`
  es nombre. Ceros a la izquierda intactos.
- Al leer se aceptan **el par configurado y los glifos canónicos `↊`/`↋`**
  (puede haber texto escrito antes de cambiar el alfabeto); al escribir, la ida
  usa siempre el par configurado.

Sigue siendo una herramienta aparte y sigue sin ser un camino de cálculo: de
hecho ahora es más evidente, porque **una expresión en base 10 no es algo que
esta calculadora pueda evaluar**. La dirección inversa es una ayuda de lectura.

---

## 3. Qué se convierte y qué no

El módulo recorre el texto y sustituye **solo** las tiras de dígitos decimales
que sean números por derecho propio. Se dejan intactos:

- **Los dígitos dentro de un identificador.** Hoy ningún nombre del catálogo
  lleva dígitos (es `log`, no `log12` — verificado contra `dz-cli --functions`),
  así que esto no arregla ningún fallo actual; se hace porque el día que se
  registre un nombre con dígitos, convertirlo lo rompería en silencio.
- **Lo que ya es dozenal** (regla 3 de §2: si aparece, no se sugiere nada).
- Operadores, paréntesis, comas y espacios: se copian tal cual.

### 3.1 Enteros: exacto

Divisiones sucesivas entre doce sobre `BigInt`, dígitos `0`-`9` más el par de
símbolos que la interfaz esté mostrando en ese momento para diez y once (por
defecto `↊`/`↋`; si el usuario configuró `A`/`B` por `docs/07`, se usa ese par,
porque el texto va a una caja que él lee). Sin límite de tamaño.

### 3.2 Fracciones decimales: sí, por escalado

El método es el que indicó el usuario, y es exacto: un decimal con `k` cifras se
multiplica por `10^k` para volverlo entero, se convierte ese entero a base 12, y
se divide por el mismo `10^k` **ya en base 12**. Todo con `BigInt`; la
multiplicación y la división nativas son la excepción deliberada de esta
herramienta (§1.1).

`0.5` → `5 / 10` → cinco es `5`, diez es `↊` → `5/↊`, que es `0;6`. Correcto.

**Pero no todo decimal tiene forma dozenal finita**, y esto decide la salida:
`n / 10^k` termina en base 12 si y solo si el `5^k` del denominador se cancela
contra el numerador, porque `10^k = 2^k·5^k` y `12 = 2²·3` no contiene el cinco.
La prueba es exacta y barata: **termina si y solo si `5^k` divide a `n`**.

**Si termina** (`0.5`, `0.25`, `0.75`): se emite el número dozenal con su punto
Humphrey — `0;6`, `0;3`, `0;9`. Exacto, igual en todos los modos. La división
finita se hace con `BigInt` multiplicando por potencias de doce hasta que sea
divisible.

**Si no termina** (`0.1`, `0.3`): la forma la decide **el modo de la sesión**,
no la herramienta. Es la configuración que el usuario ya eligió para ver
resultados, y esto es texto que va a leer.

| modo | `0.1` queda como | por qué |
|---|---|---|
| `fixed` | `0;12497249724972…` | el modo fijo ya trunca a la escala; ser exacto aquí y truncado al evaluar sería incoherente |
| `rational` | `1/↊` | el modo racional existe para no perder exactitud, y la fracción es exacta |
| `dfp` | como `fixed` | es un formato de dígitos, no de fracciones |

En modo fijo se emiten tantos dígitos como diga la **escala configurada**, la
misma que usa el resto de la interfaz. La escala y el modo llegan **como
argumentos**, igual que el par de símbolos: el módulo sigue sin importar nada
(§1.2).

> **Los puntos suspensivos son solo de la vista previa.** `…` no es un carácter
> que el lexer acepte: si entrara en el input, la expresión dejaría de evaluar.
> El popup muestra `0;12497249724972…` para que se vea que el número sigue, y
> escribe en el input el mismo número **sin** el `…`. El único sitio donde el
> carácter aparece es el texto de la sugerencia.

El popup dice qué caso aplica antes de convertir, para que ni `0.1 → 1/↊` en
racional ni el truncado en fijo sorprendan.

### 3.3 La coma es separador decimal, no de miles

Corrección posterior, alineada con `docs/10` §3.3. La primera versión leía la
coma como agrupación de miles (`/^[0-9]{1,3}(?:,[0-9]{3})+/`, grupos de tres
exactos) porque en dozenal la agrupación es de cuatro y eso la hacía una
**señal** de base 10 útil para la heurística del §2 — que el §2.4 ya derogó.

Sin la heurística esa lectura solo estorba: `,` es el separador decimal de la
mayor parte del mundo y es lo que escribe la tecla decimal de un teclado
numérico español, así que `3,14` significa tres coma catorce, no `314`. El
motor tomó la misma decisión para la entrada en base 10, y dos lecturas
distintas del mismo texto era justo lo que se quería evitar.

Ahora `,` y `.` introducen los dos la parte fraccionaria, y base 10 se queda
sin forma agrupada:

```
3,14           →  igual que 3.14
3,14159265359  →  igual que 3.14159265359
1,000          →  1          (uno, no mil)
```

Base 12 no cambia en ninguna dirección: allí el separador fraccionario es `;`,
así que la coma sigue libre para agrupar de cuatro en cuatro.

---

## 4. Alcance

- **Solo la GUI.** El CLI no tiene evento de pegado ni popup, y la regla de
  no-divergencia de `docs/06` §7 habla de capacidades de la calculadora; esto no
  lo es. Si más adelante se quiere, sería un `dz-cli from10 "12 + 11"`
  independiente.
- **No toca Rust.** Ni un fichero de `crates/`.
- **No toca la lógica existente del frontend.** `main.ts` gana el punto de
  conexión (escuchar `paste`, mostrar la sugerencia, escribir el input) y nada
  más; `keypad.ts`, `history.ts`, `research.ts`, `alphabet.ts` y `api.ts` no se
  modifican.

## 5. Tests

`app/src/base10.test.ts` o el runner que el proyecto ya use; si no hay ninguno,
un fichero de casos ejecutable con `node`. Cubrir:

- `12 + 11` → `10 + ↋`, que es el caso del pedido;
- `5 + 3` → sin sugerencia (no cambia);
- `144` → `100`; `11` → `↋`; `10` → `↊`;
- un entero enorme, más allá de `Number.MAX_SAFE_INTEGER`;
- decimales que terminan: `0.5` → `0;6`, `0.25` → `0;3`, `0.75` → `0;9`;
- decimales que no terminan, **por modo** (§3.2): en `rational`, `0.1` → `1/↊` y
  `0.3` → `3/↊`; en `fixed`, dígitos truncados a la escala configurada, y el
  mismo caso con dos escalas distintas para fijar que la escala manda;
- que el `…` aparece en el texto de la sugerencia y **nunca** en el input;
- la prueba de terminación (`5^k | n`) por sí sola, con varios `k`;
- texto con `↊`/`↋`: el botón convierte lo convertible, el popup no salta;
- dígitos dentro de un identificador → intactos;
- el par de símbolos configurado se respeta en la salida.

## 6. Lo implementado

Fichero nuevo y aislado `app/src/base10.ts` (~330 líneas), más el punto de
conexión en la GUI. Nada de `crates/` se tocó; `keypad.ts`, `history.ts`,
`research.ts`, `alphabet.ts`, `glyphs.ts` y `api.ts` están intactos.

### 6.1 El módulo

- Cabecera en inglés que dice sin rodeos lo del §1.2: que es una herramienta
  aparte, que su `*`/`%`/`/` nativos sobre `BigInt` serían una violación de la
  regla del repositorio si esto fuera código de la calculadora, que su salida no
  es un resultado sino texto que el usuario todavía tiene que evaluar con el
  motor de puertas, y que ni la calculadora puede llamarla ni ella a la
  calculadora.
- **Cero imports.** Entra `string`, sale `string`. Todo lo que el módulo sabe de
  la calculadora llega en un argumento `Base10Style { ten, eleven, mode, scale }`:
  el par de símbolos, el modo de la sesión y la escala de trabajo. No importa
  `api.ts` para leer el modo; se lo pasa el punto de conexión de `main.ts`.
- API pública:
  - `convertBase10Text(text, style) -> Base10Conversion` — la transformación,
    sin heurística ninguna. Es lo que ejecuta el botón. Devuelve `text` (lo que
    va al input), `preview` (lo mismo con `…`, solo para el popup) y las marcas
    `changed`, `keptDozenal`, `madeHumphrey`, `madeFraction` y `madeTruncated`,
    que es lo que le permite al botón decir qué hizo.
  - `suggestBase10Conversion(text, symbols) -> Base10Suggestion | null` — la
    heurística del popup. `null` significa «callarse».
  - `terminatesInBase12(n, k)` — la prueba `5^k | n`, expuesta porque se testea
    por sí sola.

### 6.2 Las dos puertas de entrada (§2.0)

- **Botón permanente `10→12`** al lado del input. Sin heurística, sin
  condiciones: convierte lo que haya, esté tecleado o pegado, tenga o no glifos
  dozenales. Si el texto se lee igual en base 12, lo dice («no hay nada que
  convertir») y no toca el input. Si el texto traía `↊`/`↋`/`;`, convierte las
  tiras decimales y lo dice: «los símbolos dozenales se dejaron intactos».
- **Popup automático** solo tras un `paste` y solo si se cumplen las tres
  condiciones del §2. Es el atajo discreto, no la única puerta.

### 6.3 La heurística, tal como quedó

Las tres condiciones del §2 se reparten así: la primera (llegó por `paste`) la
garantiza el llamante, porque `suggestBase10Conversion` solo se invoca desde el
manejador de `paste`; las otras dos viven en el módulo. Que el texto pudiera ser
válido en base 12 **no** bloquea nada: `12 + 11` sugiere, que es el caso del
pedido.

Lo único que sigue callando al popup, además de las tres condiciones, es que el
texto ya traiga un glifo dozenal (`↊`, `↋`, el par configurado, o el punto
Humphrey `;`). Ahí la señal es débil de verdad y el §2.0 lo manda al botón.

«Parece una expresión aritmética» se concretó en: como mucho 200 caracteres,
solo `0-9 A-Za-z + - * / ^ ( ) , . ; espacio`, al menos un dígito, y **todo
identificador tiene que ir seguido de `(`**. Esa última regla es la que impide
que una tira de prosa (`there are 12 cats`, que pasa el filtro de caracteres y
tiene un dígito) se lea como expresión. No es una comprobación contra el
catálogo — el módulo no conoce ni puede conocer ningún nombre de función.

**Precio conocido:** una constante suelta pegada, como `pi*12`, no dispara el
popup, porque `pi` no va seguido de `(`. El botón sí la convierte. Se aceptó a
propósito: entre sugerir de más y callarse, el pedido dice callarse.

### 6.4 Cuatro motivos

El popup nunca aparece sin decir por qué. `Base10Reason`, por orden de
precedencia (gana el desenlace más sorprendente):

- `fraction` — «el decimal no termina en base 12; exacto es → `1/↊`» (modo
  racional);
- `truncated` — «el decimal no termina en base 12; truncado a la escala →
  `0;124972497249…`» (modo fijo o DFP);
- `humphrey` — «el decimal sí termina en base 12 → `0;6`»;
- `thousands` — hay agrupación de miles de tres (`1,000`);
- `generic` — «en base 12 sería → …».

En los cuatro casos el popup enseña además el texto convertido, que es la mitad
de la explicación.

### 6.5 Qué se convierte

- **Enteros** por división sucesiva entre doce sobre `BigInt`. Exacto a
  cualquier tamaño (probado con 2⁵³+1).
- **Decimales**, por el método del §3.2. `terminatesInBase12(n, k)` es
  literalmente `n % 5^k === 0n`.
  - Termina → se busca el menor `m` con `10^k | n·12^m` (existe siempre y nunca
    pasa de `k`), y los `m` últimos dígitos del cociente van detrás del punto
    Humphrey: `0.5` → `0;6`, `0.25` → `0;3`, `0.75` → `0;9`, `1.5` → `1;6`,
    `0.125` → `0;16`. Exacto, igual en los tres modos.
  - No termina y el modo es `rational` → fracción exacta reducida por `gcd`:
    `0.1` → `1/↊`, `0.3` → `3/↊`, `0.2` → `1/5`.
  - No termina y el modo es `fixed` o `dfp` → la expansión **truncada a la
    escala de la sesión**, por el algoritmo de siempre (resto por doce, el
    cociente es el dígito siguiente): con escala 12, `0.1` → `0;124972497249`;
    con escala 4, `0;1249`. Truncado, no redondeado, que es lo que hace el modo
    fijo en el resto de la interfaz.
  - **El `…` es solo de la vista previa.** `convertBase10Text` devuelve dos
    cadenas: `text`, que es lo que se escribe en el input y jamás lleva `…`, y
    `preview`, que es lo que enseña el popup y sí lo lleva. Hay tests que fijan
    las dos cosas.
- **Los ceros a la izquierda se preservan**: si no, `007` se reescribiría como
  `7` y un texto que en las dos bases vale lo mismo parecería haber cambiado,
  disparando una sugerencia vacía. No estaba en el plan; salió de la condición 2.
- Los dígitos dentro de un identificador se copian tal cual (`log12(144)` →
  `log12(100)`). Como dice el §3, hoy ningún nombre del catálogo lleva dígitos,
  así que **esto no arregla ningún fallo actual**.
- La agrupación de miles de tres se lee como un solo número y sale sin agrupar
  (`1,000` → `6↋4`). La agrupación dozenal de cuatro (`1,0000`) no encaja en el
  patrón y se deja intacta, así que no sugiere nada.

**Desviación pequeña y deliberada del §3.2:** una fracción sale entre paréntesis
cuando no es el texto entero. `0.1` en modo racional solo da `1/↊`, como dice el plan, pero
`1/0.1` da `1/(1/↊)` y no `1/1/↊`, que el parser leería como `(1/1)/↊` y valdría
otra cosa. Sin los paréntesis la conversión no sería exacta, que es la razón de
ser de todo el §3.2.

### 6.6 El punto de conexión

`app/index.html`: el input pasa a vivir en un `div.expression-row` junto al
botón `10→12`, y entre esa fila y el resultado hay un `div.base10-anchor` de
**altura cero** con el popup dentro en `position: absolute`. Así mostrarlo y
ocultarlo no desplaza el layout (§2.1): flota sobre el resultado.

`app/src/main.ts`: `initBase10Hint()`, una función nueva al final del fichero,
llamada desde `boot()`. Es todo lo que `main.ts` gana. Lo que hace:

- escucha `paste` y vuelve a mirar en un `setTimeout(0)`, porque durante el
  evento el input todavía tiene el texto de antes y lo que se convierte es **todo
  el contenido**;
- lee el par de símbolos de la sesión viva (`session.alphabet_ten/eleven`), así
  que un alfabeto configurado por `docs/07` sale respetado. Solo cruzan dos
  `string`: el módulo nunca ve una `Session`;
- `Esc` y la `✕` descartan y **apuntan ese texto en un `Set` de la sesión**, que
  no vuelve a sugerirse;
- seguir tecleando (`input`) lo oculta, pero sin apuntar el rechazo;
- `Enter` no convierte, y no hay atajo de teclado que convierta: hace falta el
  clic;
- tras convertir, por el botón o por el popup, el mismo sitio ofrece
  **«Deshacer»**, que devuelve el texto anterior literal;
- ningún `focus()` sobre el popup, para no robar el caret.

### 6.7 Verificación

- `app/tests/base10.test.ts`, 63 casos, `node app/tests/base10.test.ts` → 63/63.
  **Desviación del §5:** el fichero está en `app/tests/`, no en `app/src/`. Todo
  lo que hay bajo `src` lo compila `tsc --noEmit` en `npm run build`, e importar
  una ruta `.ts` — que es lo que necesita el borrado de tipos de Node — habría
  obligado a encender `allowImportingTsExtensions` en el tsconfig compartido.
  Sacarlo de `src` no cuesta nada y no toca configuración existente. No se añadió
  ninguna dependencia: Node 24 ejecuta el `.ts` directamente.
- `cd app && npm run build` (incluye `tsc --noEmit`): pasa.
- `cargo test --workspace --release`: 190 tests, 0 fallos — igual que antes, que
  es el control de que no se salió del alcance.
- `pwsh tools/lint-purity.ps1`: verde. El lint solo mira Rust, así que el `*`, el
  `%` y el `/` de `base10.ts` no le constan; lo que los mantiene fuera del núcleo
  es que el fichero es TypeScript y nadie del núcleo lo importa.
- Verificación en pantalla con el `.exe` que produce
  `npm run tauri build -- --no-bundle` (nunca `cargo build --release`, por
  `docs/04` §9.5). Comprobado: al arrancar no hay popup ni desplazamiento del
  layout; pegar `5 + 3` no dice nada; pegar `12 + 11` levanta el popup con su
  motivo; aceptarlo deja `10 + ↋` en el input y ofrece «Deshacer»; deshacer
  restaura `12 + 11` y cierra el popup; `Esc` lo descarta y volver a pegar el
  mismo texto ya no sugiere nada. En modo racional `0.1` ofrece `1/↊`; con el
  modo cambiado a `fixed` el mismo `0.1` ofrece `0;124972497249…`, y al aceptar
  el input queda `0;124972497249` **sin** el `…`, que además evalúa
  (3.113.995 puertas, profundidad 866) — que es la prueba de que el carácter de
  la vista previa no llega al lexer.

Dos fallos que solo se vieron en pantalla, los dos ya arreglados:

1. `.base10-hint { display: flex }` ganaba al atributo `hidden` (misma
   especificidad que `[hidden]`, y la hoja de autor va después de la del
   navegador), así que el popup no se ocultaba nunca. Se añadió
   `.base10-hint[hidden] { display: none }`.
2. El motivo iba en una línea con `text-overflow: ellipsis`, y con una
   expansión truncada de doce dígitos el número quedaba recortado — justo lo
   que el popup existe para enseñar. Ahora envuelve.

### 6.8 Lo que NO se hizo, y por qué

- **CLI.** El §4 lo deja fuera. No hay `dz-cli from10`.
- **Números negativos como signo propio.** El `-` se copia como operador, que es
  lo correcto: `-12` → `-10` sale bien sin tratar el signo.
- **Notación científica, hexadecimal, separadores `_`.** Fuera del alfabeto
  aceptado; el texto se descarta como no-expresión y el popup no salta. El botón
  tampoco los entiende: convertiría las tiras decimales sueltas.
- **Notación de periodo** (una barra sobre los dígitos que se repiten). En modo
  fijo se trunca a la escala y en racional sale la fracción exacta; ninguna de
  las dos es una notación de periodo, y no se pidió.
- **Redondeo** del truncado en modo fijo. Se trunca, como hace el modo fijo en
  el resto de la interfaz; redondear aquí y truncar al evaluar sería la
  incoherencia que el §3.2 quiere evitar.
- **Persistencia de los rechazos.** El `Set` de textos descartados vive en la
  sesión, como pide el §2.1 («en la sesión»); no se guarda en `localStorage`.
- **Un deshacer de más de un paso.** Se guarda solo el texto inmediatamente
  anterior, que es lo que pide el §2.1.
- **Nada del panel de investigación, del historial, de la memoria ni del conteo
  de puertas se tocó.** El módulo solo escribe en `expression.value`.

### 6.9 El caché de conversiones (§2.2)

Vive **dentro de `base10.ts`**, no en el punto de conexión. La razón es la misma
que gobierna todo el módulo: es autocontenido, así que ahí lo aprovechan todos
los llamantes —el botón, la heurística del popup y los tests— sin cablear nada,
y `main.ts` sigue sin saber cómo se produce una conversión. No rompe el §1.2:
sigue habiendo cero imports y sigue entrando un `string` y saliendo un `string`.

- `convertBase10Text` pasó a ser un envoltorio de una función interna
  `computeBase10Text` (la de siempre, intacta). El envoltorio solo hace la
  búsqueda.
- **La clave es la tupla completa**: el texto más *todos* los campos de
  `Base10Style`. Se deriva recorriendo el propio objeto
  (`Object.keys(symbols).sort()`) en vez de enumerar campos a mano, así que un
  campo nuevo entra en la clave solo — olvidarlo no es posible. Un test recorre
  también el objeto y compara contra un recálculo con el caché frío, así que
  falla el día que la clave deje fuera un campo nuevo.
- `Map` de 50 entradas como mucho. En un acierto la entrada se reinserta (cuenta
  como recién usada y sobrevive al descarte) y en una inserción por encima del
  tope se borra la primera clave, que en un `Map` es la más antigua. De sesión,
  en memoria, **no se persiste**.
- La entrada se copia al salir (`{ ...hit }`), así que un llamante que mutara su
  resultado no puede corromper el del siguiente. Hay un test de eso.
- **Nunca es fuente de verdad**: todos los casos nuevos comparan la ruta cacheada
  contra un recálculo desde un caché vacío, y hay casos explícitos de que
  cambiar modo, escala o alfabeto no devuelve la entrada del otro.
- `clearBase10Cache()` y `base10CacheSize()` se exportan **solo para los tests**;
  la GUI no los llama y no hay ninguna vía de usuario que vacíe el caché. No hace
  falta: la clave ya lleva la configuración entera, así que no hay nada que
  invalidar.

### 6.10 El deshacer caduca al editar (§2.3)

Implementado como **lista blanca**, tal como manda el §2.3, y por eso no hay
ninguna lista de vías que mantener:

- `anchor` guarda el texto exacto al que pertenece la oferta viva: el texto
  pegado mientras se ofrece convertir, y **la salida de la propia conversión**
  mientras el deshacer está armado. `offerUndo` lo fija leyendo
  `expression.value` justo después de escribir la conversión, que es lo único
  que arma el deshacer.
- Un bucle de `requestAnimationFrame` compara `expression.value` contra `anchor`
  y, en cuanto difieren, cierra el popup. Corre **solo mientras hay una oferta
  viva** y se apaga solo. Hacía falta observar el valor y no los eventos porque
  las escrituras programáticas (`insert`, el borrado, el botón `C`, restaurar
  del historial) **no disparan ningún evento**, ni `input` ni otro.
- Además, al pulsar el botón se vuelve a comparar antes de actuar, por si una
  edición llegó entre el último fotograma y el clic.
- Se conservó el `input` que ya ocultaba el popup: es la respuesta inmediata al
  tecleo y no estorba a la regla general.

**Consecuencia práctica: no se tocó `keypad.ts` ni `history.ts`.** Con un solo
punto de observación del valor, el teclado dozenal en pantalla y la restauración
desde el historial caen por la regla general, igual que el botón `C`, y que
cualquier vía futura. Los ficheros modificados son tres: `app/src/base10.ts`,
`app/src/main.ts` y `app/tests/base10.test.ts`.

### 6.11 Un fallo que solo se vio en pantalla

`offerUndo` no ponía `hint.hidden = false`. Cuando la conversión venía de aceptar
la sugerencia el popup ya estaba abierto y no se notaba, pero cuando venía del
**botón `10→12`** el deshacer se armaba sin que nada se viera: ni el mensaje de
qué se hizo ni el botón «Deshacer». Arreglado con esa línea. Es anterior a estas
dos features; salió al intentar validarlas en pantalla.

### 6.12 Verificación de estas dos features

- `node app/tests/base10.test.ts` → **84/84** (los 63 de antes, sin cambios, más
  21 nuevos del caché).
- `cd app && npm run build`: pasa. `cargo test --workspace --release`: 190 tests,
  0 fallos, igual que antes. `pwsh tools/lint-purity.ps1`: verde.
- En pantalla, con el `.exe` de `npm run tauri build -- --no-bundle`, capturando
  con `PrintWindow` + `PW_RENDERFULLCONTENT` y clicando con `AttachThreadInput` +
  `SetWindowPos(HWND_TOPMOST)` + `mouse_event`:
  - convertir `12 + 11` → `10 + ↋` con «Deshacer»; teclear un `3` encima → el
    popup desaparece y queda `10 + ↋3`;
  - convertir y no tocar nada → «Deshacer» devuelve `12 + 11` y cierra el popup;
  - convertir y pulsar la tecla `7` del teclado dozenal → el popup desaparece
    (queda `10 + ↋7`), **sin código específico para el teclado**;
  - convertir y restaurar `4+2` del historial → el popup desaparece, tampoco con
    código específico;
  - sexta vía, para enseñar que la regla es general: convertir y pulsar `C` → el
    popup desaparece;
  - el caché no cambia lo que se ve: `0.1` en racional da `1/↊`, repetirlo (ya
    cacheado) da `1/↊` otra vez; con el modo cambiado a `fixed` el mismo `0.1` da
    `0;124972497249` —no la entrada racional—, y al volver a racional vuelve a
    salir `1/↊`.

### 6.13 Lo que NO se hizo de estas dos features

- **No se persiste el caché** (§2.2 lo dice), ni se expone su tamaño en la
  interfaz.
- **No se vacía nunca** desde la GUI. Que la clave lleve la configuración entera
  es exactamente lo que hace innecesario invalidarlo.
- **No se cachea la heurística** (`suggestBase10Conversion`) como tal: reutiliza
  el caché de la conversión, que es donde está el trabajo, y sus filtros son
  regulares baratas.
- **Sigue habiendo un solo paso de deshacer**, como en el §6.8.
- **No se cambió `keypad.ts` ni `history.ts`**, y no hizo falta.

### 6.14 El popup fuera y la dirección invertible (§2.4)

Esto **sustituye** a lo descrito en §6.2 (segunda mitad), §6.3 y §6.4: el popup
automático, sus tres condiciones y sus cuatro motivos ya no existen en la
interfaz.

- `app/index.html`: desaparece el bloque `base10-anchor`/`base10-hint` entero. La
  fila del input pasa a tener, además del input: un `span#base10-status`
  (mensaje corto), un `button#base10-undo` (`↺`, oculto salvo cuando el deshacer
  está armado), el `button#base10-convert` (`10→12` / `12→10`) y el
  `button#base10-swap` (`⇄`).
- `app/src/styles.css`: fuera todos los estilos del popup (incluido el
  posicionamiento absoluto y el `[hidden]` que hubo que añadir en su día). El
  estado va en la misma fila, con `text-overflow: ellipsis` y el texto completo
  en el `title`, así que la fila no crece y el display no se mueve.
- `app/src/main.ts`: `initBase10Hint` pasa a ser `initBase10Tool`. Fuera el
  manejador de `paste`, el `Set` de textos descartados, el `Esc`, la `✕` y los
  cuatro motivos. Queda el estado `direction`, el repintado del rótulo del botón,
  la conversión bajo demanda en la dirección activa, el deshacer del §2.3 y la
  línea de estado. La regla de la lista blanca no cambió ni una letra.
- `app/src/base10.ts`: `convertBase12Text` (la dirección inversa) y
  `terminatesInBase10`, espejo de `terminatesInBase12`. El caché es el mismo y su
  clave gana la dirección — `11` es `↋` de ida y `13` de vuelta, así que las dos
  direcciones no pueden verse las entradas.

**`suggestBase10Conversion` se borró en el commit siguiente al de la herramienta.**
Era la heurística documentada del §2, sin llamante desde que el §2.4 retiró el
popup. Se commiteó primero con el resto y se quitó después, en un commit propio,
para que el trabajo quede recuperable en el historial en vez de perderse: quien
quiera resucitar el popup tiene ahí el código y sus casos. Con ella salieron
`Base10Reason`, `Base10Suggestion`, `looksLikeExpression` y los cuatro tests que
solo probaban el silencio de la heurística (prosa y texto ya dozenal); los demás
casos siguen vivos apuntando a `convertBase10Text`, que es lo que la interfaz
llama de verdad.

### 6.15 Verificación de §2.4

- `node app/tests/base10.test.ts` → **117/117** (**108** tras borrar la heurística
  en el commit siguiente): los 63 originales, los 21 del
  caché y 33 nuevos de la dirección inversa (enteros, `0;6`→`0.5`, `0;3`→`0.25`,
  `0;16`→`0.125`, el tercio por modo y por escala, el `…` solo en la vista
  previa, la fracción entre paréntesis dentro de una expresión, identificadores,
  el par `X`/`E` con `XE`=131 y `exp` como nombre, los glifos canónicos siempre
  legibles, la prueba `3^k | n` suelta, el viaje de ida y vuelta de un entero
  mayor que 2⁵³, y que las dos direcciones no comparten entradas de caché).
- `cd app && npm run build`: pasa. `cargo test --workspace --release`: 190, 0
  fallos. `pwsh tools/lint-purity.ps1`: verde.
- En pantalla, con el `.exe` de `npm run tauri build -- --no-bundle`:
  - al arrancar, la fila muestra `10→12` y `⇄`, y **nada más**;
  - pegar `12 + 11` ya **no levanta ningún popup**;
  - `12 + 11` + botón → `10 + ↋`, con «base 12» en la fila y el `↺` a la
    izquierda del botón;
  - pulsar `⇄` cambia el rótulo a `12→10` y retira el deshacer; pulsar el botón
    devuelve `12 + 11`, ahora con «base 10»;
  - teclear encima de una conversión, o pulsar una tecla del teclado dozenal,
    quita el `↺` y el mensaje (la lista blanca del §2.3 sigue en pie con la
    interfaz nueva);
  - `0;4` con `12→10` en modo racional da `1/3`; volver a pulsar el botón dice
    «nada que convertir» en la fila y no toca el input.

### 6.16 Lo que NO se hizo de §2.4

- **No hay conversión de vuelta en el CLI**, igual que no la había de ida (§4).
- **No se evalúa lo convertido a base 10.** No se puede: el motor es dozenal. La
  dirección inversa deja texto en la caja para leerlo, y si el usuario lo evalúa,
  el lexer lo leerá como dozenal —que es lo mismo que pasa si lo teclea a mano.
- **No se recuerda la dirección entre arranques.** Empieza siempre en `10→12`.
- **No se avisa de la ambigüedad de la vuelta**: `12 + 11` en la caja puede
  querer decir dos cosas y la herramienta no lo adivina, exactamente igual que en
  la ida (§2). Por eso el control es un botón que se pulsa, no una heurística.
- **No se tocó el teclado, el historial, la memoria, el panel de investigación ni
  ningún conteo de puertas.** Los ficheros del cambio son `app/index.html`,
  `app/src/styles.css`, `app/src/main.ts`, `app/src/base10.ts` y
  `app/tests/base10.test.ts`.

---

## 2.5 Convertir solo la selección

Pedido: cuando haya una parte del input seleccionada, aparece junto al botón de
convertir un texto «convertir selección», y al pulsar el botón se convierte
**solo** ese trozo.

### El rótulo

Vive en la fila, junto a `10→12`, y aparece **solo** mientras hay una selección
no vacía dentro del input. Es una etiqueta de estado, no un segundo botón: el
botón sigue siendo uno y lo que cambia es su alcance. Dos botones que convierten
obligarían a elegir cuál mirar cada vez.

La selección cambia por muchas vías (arrastrar con el ratón, `shift`+flechas,
`ctrl`+`a`, doble clic, la barra de menú del sistema), así que el rótulo se
gobierna observando **el estado de la selección**, no enumerando eventos —
misma lección que dejó la lista blanca del §2.3.

### La trampa: el foco

Pulsar un botón le quita el foco al input. `selectionStart`/`selectionEnd`
sobreviven al desenfoque, pero el manejador actual termina con
`expression.focus()`, y volver a enfocar puede reposicionar el cursor. Así que
el trozo a convertir se lee **al empezar** el manejador, antes de tocar nada, y
al terminar se restaura la selección sobre el texto ya convertido, para que el
usuario vea qué cambió y pueda encadenar otra acción.

### Qué se convierte, exactamente

El trozo seleccionado, tal cual, como si fuera un texto independiente: se le
pasa a `convertBase10Text`/`convertBase12Text` y su resultado sustituye a la
selección dentro del valor completo. El módulo `base10.ts` **no cambia**; esto
es recorte y pegado alrededor de una llamada que ya existe.

Consecuencia que hay que aceptar y decir: **una selección que parte un número
por la mitad convierte las mitades por separado.** Con `12 + 11`, seleccionar
`12 + 1` da `10 + 1` y el `1` que queda fuera no se toca, así que la caja acaba
en `10 + 11`. Es lo que pidió el pedido —convertir lo seleccionado— y es
predecible; la alternativa (extender la selección a los límites del número) sería
convertir cosas que el usuario no marcó, que es peor. La línea de estado dice
que se convirtió la selección, para que el resultado no parezca un fallo.

Si la conversión del trozo no cambia nada, se dice igual que hoy («nada que
convertir»), sin tocar el input.

### Deshacer

Sin cambios de regla: `undoTo` guarda el valor **completo** de antes, y el
ancla sigue siendo el valor completo de después. La lista blanca del §2.3 no se
entera de que la conversión fue parcial.

---

### 6.17 Convertir solo la selección (§2.5)

Ficheros tocados: `app/index.html`, `app/src/styles.css` y `app/src/main.ts`.
**`app/src/base10.ts` no cambió ni una línea**, que era la prueba de que el
diseño no se torcía: esto es recorte y pegado alrededor de una llamada que ya
existía.

- `app/index.html` — un `span#base10-selection` con el texto «convertir
  selección», `hidden` de nacimiento, entre la línea de estado y el botón
  `10→12`. Es lo primero que el usuario lee a la izquierda del botón, que es
  donde mira antes de pulsarlo.
- `app/src/styles.css` — `.base10-selection`, en el color de acento (`--approx`),
  sin borde, sin cursor de mano y sin `display` propio. Lo del `display` es
  deliberado: el fallo del §6.7 fue justo ese, una regla de autor que ganaba al
  `[hidden]` del navegador. Sin `display` en la regla, `hidden` funciona solo.
- `app/src/main.ts` — dentro de `initBase10Tool`: `selectedRange()`,
  `paintSelection()`, el observador de la selección, y el manejador del botón
  reescrito para trabajar sobre un rango.

#### Cómo se observa la selección

`selectedRange()` lee `selectionStart`/`selectionEnd` y devuelve `null` cuando
el rango está vacío. `paintSelection()` enseña u oculta el rótulo según eso, y
**nunca según qué evento pasó**. El rótulo se repinta desde dos disparadores:

1. `document.addEventListener("selectionchange", …)`, la vía moderna. Se
   comprobó en pantalla que WebView2 (Chromium) la dispara para `<input>`.
2. Un bucle de `requestAnimationFrame` que llama a lo mismo. Cuesta dos
   comparaciones de enteros por fotograma y sólo toca el DOM cuando el estado
   cambia de verdad (`selectionShown`), así que es gratis en la práctica.

El segundo no es redundancia por miedo: es la misma lección de la lista blanca
del §2.3. Lo que se lee siempre es **el rango**, no el evento, así que una vía
que no dispare `selectionchange` — una futura, o el menú del sistema en una
versión distinta del WebView — la coge el fotograma siguiente en vez de dejar el
rótulo mintiendo. Enumerar eventos (ratón, `shift`+flechas, `ctrl`+`a`, doble
clic…) es exactamente la lista negra que este documento ya decidió no volver a
escribir.

#### El foco, que era la trampa

El manejador del botón lee el rango en su **primera** línea, antes de tocar
nada, porque para cuando corre el input ya está desenfocado y todo lo que viene
después (`focus()`, escribir `value`) puede mover el caret.
`selectionStart`/`selectionEnd` sobreviven al desenfoque, así que leer temprano
basta y no hizo falta ningún `mousedown`/`preventDefault` sobre el botón para
retener el foco — que era la otra salida, y habría cambiado el comportamiento de
un control que ya funcionaba.

Al terminar, en vez del `expression.focus()` de siempre, se llama a
`restore(start, end)`: enfoca y vuelve a marcar el trozo, ahora sobre el **texto
convertido**, que tiene otra longitud (`range.start + conversion.text.length`).
El usuario ve marcado exactamente lo que cambió, el rótulo sigue en pie y puede
encadenar otra acción. Cuando no había selección, se conserva el `focus()` seco
de antes. En el camino de «nada que convertir» la selección también se restaura,
para que el input quede como estaba.

#### El alcance y el mensaje

Con selección, el trozo va a `convertBase10Text`/`convertBase12Text` **como
texto independiente** y su resultado se pega en el hueco:
`previous.slice(0, start) + conversion.text + previous.slice(end)`. Sin
selección, todo sigue igual que antes.

La línea de estado dice `base 12 (selección)` en vez de `base 12`, y el
`title` gana «solo se convirtió la parte seleccionada». Es lo que evita que el
caso del §2.5 —`12 + 11` con `12 + 1` marcado, que deja `10 + 11`— se lea como
un fallo: las dos mitades del número se convierten por separado porque eso es lo
que se pidió convertir. **No se extiende la selección a los límites del
número**, que convertiría texto que el usuario no marcó.

#### Deshacer

Sin cambios de regla, tal como manda el §2.5. `undoTo` guarda `previous`, el
valor **completo** de antes, y `anchor` es el valor completo de después. La
lista blanca del §2.3 compara valores enteros y no se entera de que la
conversión fue parcial; ni el bucle de vigilancia ni la comprobación previa al
clic necesitaron una línea nueva.

### 6.18 Verificación de §2.5

- `node app/tests/base10.test.ts` → **108/108**, los mismos de antes y sin
  ninguno nuevo. **Desviación consciente:** no se añadieron tests. La lógica
  nueva es entera de DOM —leer `selectionStart`/`selectionEnd`, enseñar un
  `span`, devolver el foco— y ese fichero prueba el módulo, que no cambió;
  montar un andamiaje de DOM falso sólo para `main.ts` sería más código de
  prueba que de producción y probaría el andamiaje. La comprobación de esta
  feature es la de pantalla, que es donde vive.
- `cd app && npm run build` (incluye `tsc --noEmit`): pasa.
- `cargo test --workspace --release`: **190 tests, 0 fallos**, igual que antes —
  el control de que no se salió del alcance. `pwsh tools/lint-purity.ps1`: verde.
- En pantalla, con el `.exe` de `npm run tauri build -- --no-bundle`, capturando
  con `PrintWindow` + `PW_RENDERFULLCONTENT` y actuando con `AttachThreadInput` +
  `SetWindowPos(HWND_TOPMOST)` + `mouse_event`/`SendKeys`:
  - al arrancar, la fila muestra `10→12` y `⇄` y **ningún rótulo**;
  - `12 + 11` y `shift`+`←`+`←` → aparece «convertir selección» junto al botón;
  - `→` (selección vacía) → el rótulo desaparece, y el deshacer no se retira,
    porque mover el caret no cambia el `value` (§2.3);
  - `12 + 11` con `11` marcado + botón → `12 + ↋`, con «base 12 (selección)», el
    `↺` armado y el `↋` marcado;
  - `↺` devuelve `12 + 11` entero; volver a convertir y teclear un `3` retira el
    `↺` y el mensaje;
  - `12 + 11` con `12 + 1` marcado + botón → **`10 + 11`**, sin excepción ni
    texto raro, con el mensaje diciendo que fue la selección;
  - `ctrl`+`a` + botón → `10 + ↋`, idéntico a convertir todo;
  - **doble clic** sobre `12` (vía puramente de ratón, sin teclado) → el rótulo
    aparece igual, que es la prueba de que se observa el estado y no una lista
    de teclas;
  - selección sin nada que convertir (`5 + 3` con `ctrl`+`a`) → «nada que
    convertir», el input intacto y la selección conservada.

### 6.19 Lo que NO se hizo de §2.5

- **No hay un segundo botón.** El rótulo es una etiqueta de estado; el botón
  sigue siendo uno y lo único que cambia es su alcance, como pide el §2.5.
- **No se extiende la selección a los límites del número**, ni se avisa de que
  la selección parte uno por la mitad. Se convierte lo marcado y se dice que fue
  la selección; adivinar sería convertir lo que el usuario no pidió.
- **El botón `⇄` y el `↺` no cambian de alcance**: invertir la dirección sigue
  retirando la oferta, y el deshacer sigue restaurando el valor completo.
- **No se toca la selección cuando no hay conversión**, más allá de devolverla
  tal cual estaba.
- **No se tocó `base10.ts`**, ni `keypad.ts`, `history.ts`, `research.ts`,
  `alphabet.ts` o `api.ts`, ni nada de `crates/`.
