# 07 — Alfabeto de presentación configurable

Pedido: que el teclado de letras acepte un par de símbolos para diez y once en
vez del surtido actual de alias, y que ese par sea **editable por el usuario**
desde un modal de configuración junto a «Sesión». Por defecto se queda como está
(↊/↋). Es un cambio **de presentación**: se traduce justo antes de convertir al
símbolo del backend, y la salida se traduce solo para el resultado visible.

Estado de partida: rama `feat/phase7-research-panel`, commit `8444944`.

---

## 1. Lo que ya existe, y el borde exacto donde encaja esto

| Pieza | Dónde | Qué hace hoy |
|---|---|---|
| Glifos canónicos | `crates/dz-parse/src/alphabet.rs:13` | `GLYPHS`, tabla fija indexada por `tag()` |
| Alias de entrada | `alphabet.rs:27` `char_to_digit` | acepta ↊/↋ más `X`/`x`/`T`/`t` (diez) y `E`/`e` (once) |
| Regla identificador vs. número | `crates/dz-parse/src/lexer.rs:69` `is_identifier_run` | una tira maximal de letras es identificador **si y solo si** alguna de sus letras no es alias de dígito |
| Respaldo tipográfico | `app/src/glyphs.ts` | mide en un canvas si la fuente tiene ↊/↋; si no, cae a `X`/`E` **solo en presentación** |

`glyphs.ts` ya es exactamente este mismo concepto — un alfabeto de presentación
distinto del canónico — pero con una sola alternativa fija y decidida por la
fuente. Lo que se pide es generalizarlo a un par elegido por el usuario. Ése es
el borde: **no** se toca `char_to_digit` ni la tabla `GLYPHS`, que son el
alfabeto canónico del proyecto.

---

## 2. La trampa: esto NO es sustituir caracteres

Si el par configurado es `a`/`b`, un remapeo ingenuo de caracteres convierte
`abs(3)` en `XEs(3)` y rompe la función. El lexer no tiene ese problema porque
aplica la regla de `is_identifier_run`: `ab` es un número (todas sus letras son
alias), `abs` es un identificador (la `s` no lo es).

**Consecuencia de diseño, que manda sobre todo lo demás:** el traductor tiene
que aplicar la misma regla de tiras maximales de letras. Y como esa regla es
semántica dozenal, tiene dos sitios prohibidos y uno correcto:

- **Prohibido en JavaScript.** `CLAUDE.md`: «el frontend nunca decide semántica
  dozenal». Reimplementar `is_identifier_run` en TypeScript es duplicar la regla
  en un lenguaje donde nadie la va a mantener sincronizada.
- **Prohibido duplicarla en `dz-api`.** Sería una segunda copia de la misma
  regla, un piso por encima de su dueño.
- **Correcto: en `dz-parse`**, junto al lexer que ya es el dueño de la regla, y
  reutilizándola en vez de copiarla.

Esto además responde literalmente a lo pedido («se parsearía justo antes de
hacer la conversión al símbolo usado en backend»): la traducción ocurre en la
capa 3, inmediatamente antes del lexer, y a la vuelta sobre el string ya
formateado.

---

## 3. Diseño

### 3.1 `DisplayAlphabet` en `dz-parse`

```rust
// crates/dz-parse/src/display_alphabet.rs

/// The two symbols a user sees for ten and eleven. Presentation only: the
/// canonical alphabet (`alphabet::GLYPHS`, `alphabet::char_to_digit`) is
/// untouched, and nothing below `dz-parse` ever sees these characters.
pub struct DisplayAlphabet { ten: char, eleven: char }

impl DisplayAlphabet {
    /// The default: U+218A / U+218B, i.e. exactly today's behaviour.
    pub fn canonical() -> DisplayAlphabet;

    /// Rejects a pair that would make the alphabet ambiguous or would
    /// swallow a function name -- see section 3.2.
    pub fn new(ten: char, eleven: char) -> Result<DisplayAlphabet, AlphabetError>;

    /// User text -> canonical text, respecting the maximal-letter-run rule
    /// so `abs` stays a function name when `a`/`b` are the chosen pair.
    pub fn to_canonical(&self, text: &str) -> String;

    /// Canonical output -> user text, for display only.
    pub fn from_canonical(&self, text: &str) -> String;
}
```

`to_canonical` **reutiliza** el recorrido de tiras de letras del lexer
(`letter_run`/`is_identifier_run`, hoy privados en `lexer.rs`): se elevan a
`pub(crate)` y se parametrizan por el conjunto de alias en vigor, en vez de
copiarse. Si eso resulta imposible sin retorcer el lexer, la alternativa
aceptable es extraer la regla a una función común que **ambos** llamen — lo que
no es aceptable es una segunda implementación.

### 3.2 Validación de un par: qué se rechaza y por qué

`DisplayAlphabet::new` rechaza:

1. **Dígitos `0`-`9`, el punto Humphrey `;`, la coma, paréntesis y operadores.**
   Redefinirlos hace el texto ambiguo.
2. **Un par con los dos símbolos iguales.**
3. **Un par que haga que algún nombre del catálogo de funciones lexe como
   número.** Éste es el caso real y no obvio: con el par `p`/`i`, la constante
   `pi` pasa a ser una tira de letras todas alias, o sea el número «diez-once»,
   y la constante se vuelve inalcanzable. La validación recorre
   `dz_eval::default_registry()` y rechaza el par si algún nombre queda
   compuesto solo por letras del par.

El mensaje de error dice cuál de las tres reglas falló y, en el tercer caso,
qué nombre concreto se rompería. Un «símbolo no válido» a secas obliga al
usuario a adivinar.

Nota sobre el caso pedido: `a`/`b` **pasa** la validación. Los nombres del
catálogo que empiezan por esas letras (`abs`, `acos`, `atan`) contienen letras
fuera del par, así que siguen siendo identificadores.

### 3.3 Qué se acepta al teclear (el «limitar» del pedido)

Con un par configurado, la entrada acepta:

- los dos símbolos del par, en mayúscula y minúscula;
- **siempre** los glifos canónicos ↊/↋, que no son ambiguos con nada.

Los alias heredados `X`/`x`/`T`/`t`/`E`/`e` dejan de aceptarse **salvo** que el
par configurado sea precisamente el ASCII. Eso es lo que da el «solo acepte a y
b» del pedido: con el par `a`/`b`, teclear `E` es un error de sintaxis, no un
once silencioso.

> **Supuesto explícito.** Esto estrecha lo que hoy se acepta. Si se prefiere que
> `X`/`T`/`E` sigan valiendo siempre por compatibilidad, es una constante de una
> línea en `DisplayAlphabet`. Se implementa como está descrito porque el pedido
> dice «limitar»; cambiarlo después es trivial.

**El lexer no cambia.** Sigue aceptando el alfabeto canónico completo, que es lo
que le llega: la traducción ya ocurrió. Ninguna capa por debajo de `dz-parse` se
entera de nada, y ni un test existente cambia de expectativa.

### 3.4 Cómo cruza a la GUI

`dz-api` gana el par en `Request` (o en la sesión de la interfaz — decisión del
implementador, pero **una sola** de las dos, no ambas), aplica `to_canonical`
antes de lexar y `from_canonical` sobre `Response.display` y sobre el resto de
texto visible. Cuidado con el alcance de la salida: la traza paso a paso, el
CSV y el JSON también llevan glifos. Regla:

- **Se traduce** lo que el usuario lee en pantalla: `display`, los pasos de la
  traza en el visor, las etiquetas del panel.
- **No se traduce** el CSV ni el JSON. Son documentos de intercambio y su
  alfabeto debe ser estable, no depender de una preferencia visual de quien
  exportó. Se anota en el propio documento y en el skill.

El frontend recibe strings ya traducidos y **no traduce nada**. El teclado se
repinta con los símbolos configurados (`app/src/keypad.ts`), y `glyphs.ts`
sigue haciendo su trabajo actual: si la fuente no tiene el símbolo elegido,
avisa; su respaldo automático a `X`/`E` solo aplica cuando el par configurado
es el canónico.

### 3.5 El modal de configuración

Junto a «Sesión» en la barra lateral (`app/index.html`), un modal pequeño:

- dos campos de un carácter, para diez y para once;
- **presets** de un clic: ↊/↋ (por defecto), `X`/`E` (ASCII), `A`/`B`, `T`/`E`;
- vista previa en vivo con un número de ejemplo;
- el error de validación de §3.2 se muestra tal cual llega de Rust;
- botón de restablecer.

Persistencia: `localStorage`. Es una preferencia visual por máquina, no estado
de la sesión de cálculo, así que **no** entra en `AppState` ni en el historial.
Los resultados ya guardados en el historial se repintan con el alfabeto nuevo
al cambiarlo, porque el historial guarda texto canónico y el alfabeto es una
capa de presentación — si se repintara solo lo nuevo, la lista quedaría mezclada.

### 3.6 El CLI

`docs/06` §7 fija que las interfaces no deben divergir. Como la traducción vive
en `dz-parse`, exponerla en `dz-cli` es un flag: `--alphabet <diez><once>`, más
`:alphabet` en el REPL. Se incluye por esa regla, no porque se pidiera; si
estorba, es la parte más fácil de quitar.

---

## 4. Riesgos

- **El par elegido puede no existir en la fuente del sistema.** `glyphs.ts` ya
  sabe medirlo; debe avisar en el modal en vez de dejar cuadros vacíos.
- **Mayúsculas y minúsculas.** El par se acepta en ambas, pero la salida usa una
  forma fija (la que el usuario escribió en el modal), o `X` y `x` parecerían
  dígitos distintos en el mismo número.
- **La validación depende del catálogo de funciones.** Si mañana se registra una
  función cuyo nombre queda dentro de un par ya configurado por alguien, esa
  configuración pasa a ser inválida a posteriori. El arranque debe validar la
  preferencia guardada y caer al alfabeto canónico avisando, en vez de romper.

## 5. Lo implementado

Implementado sobre `8444944`. Baseline de partida: 171 tests, 0 warnings de
clippy, lint de pureza en verde. Estado final: **190 tests**, 0 warnings,
lint en verde, y `gate_counts` **byte a byte idéntico** al de antes (el control
de que nada salió de la capa de presentación).

### 5.1 Dónde quedó cada cosa

| Pieza | Dónde |
|---|---|
| `DisplayAlphabet`, `AlphabetError`, `to_canonical`/`from_canonical`, `REJECT_LEGACY_ALIASES` | `crates/dz-parse/src/display_alphabet.rs` |
| La regla del lexer, parametrizada y reutilizada | `crates/dz-parse/src/lexer.rs:57-92` (`letter_run`, `is_identifier_run_with`, `is_identifier_run`) |
| `Request.alphabet`, `with_alphabet`, `display_alphabet(ten, eleven)`, `ApiError::Alphabet` | `crates/dz-api/src/lib.rs` |
| `TraceRequest.alphabet`, `compare_backends(..., alphabet)` | `crates/dz-api/src/research.rs` |
| `--alphabet <XY>`, `:alphabet [XY]`, `fn shown` | `crates/dz-cli/src/main.rs` |
| `Session.alphabet`, `set_alphabet`, `reset_alphabet`, traducción de salida | `app/src-tauri/src/{state,commands,research}.rs` |
| El modal, presets, vista previa, `localStorage` | `app/src/alphabet.ts`, `app/index.html`, `app/src/styles.css` |
| Teclado con los dos símbolos configurables | `app/src/keypad.ts` |
| Sonda tipográfica por símbolo | `app/src/glyphs.ts::supportsSymbol` |

### 5.2 Dónde se desvió del plan, y por qué

**El constructor recibe el catálogo; no lo busca.** El §3.2 dice que la
validación «recorre `dz_eval::default_registry()`», pero `dz-eval` es la capa 4
y `dz-parse` la 3: llamarlo desde ahí sería una dependencia circular.
`DisplayAlphabet::new(ten, eleven, reserved_names: &[String])` recibe los
nombres, y `dz_api::display_alphabet(ten, eleven)` —capa 5, donde ambos están a
la vista— es el constructor que las interfaces usan y el que rellena el
catálogo. La regla sigue viviendo en un solo sitio; sólo cambió quién le pasa
la lista. (El test contra el catálogo *real* sí vive en `dz-parse`, porque
`dz-eval` ya era dev-dependency suya.)

**`to_canonical` devuelve `Result`, no `String`.** Hace falta para el §3.3: con
el par `a`/`b`, teclear `E` tiene que ser un error, y un traductor que sólo
devuelve texto no tiene forma de decirlo. El error es
`AlphabetError::LegacyAlias`, y llega al usuario como `ApiError::Alphabet`.

**La traducción de salida no la hace `dz-api`, la hacen las interfaces.** El
§3.4 dejaba la elección abierta; se eligió que `Request.alphabet` sea **sólo de
entrada** y que `Response.display` siga siendo canónico. Eso da gratis las tres
propiedades que el propio plan pedía: el CSV y el JSON no se traducen (se
construyen desde el valor canónico), el historial guarda texto canónico, y por
tanto cambiar de alfabeto **repinta la lista entera** en vez de dejarla mezclada.
La regla quedó: *toda entrada de `dz-api` que acepta texto del usuario acepta
también el alfabeto en que ese texto está escrito* — `Request`, `TraceRequest` y
`compare_backends`.

**`memory_store` dejó de recibir el valor.** Antes el frontend le devolvía el
string que tenía en pantalla; con un alfabeto configurable eso congelaría el
registro en la ortografía del momento. Ahora el comando no toma argumento y
guarda la copia canónica que ya tiene Rust, igual que `:mem+` en el REPL.

**Se añadió un `reset_alphabet` además de `set_alphabet`.** El botón
«Restablecer» del §3.5 podría haber llamado a `set_alphabet('↊','↋')`, pero un
comando explícito deja el «volver al canónico» fuera del camino de validación.

**Un cuarto motivo de rechazo, no tres.** Además de las tres reglas del §3.2 se
rechaza usar un glifo canónico en la ranura contraria (`↋` como diez), que es
ambigüedad pura y no estaba contemplado.

### 5.3 El supuesto del §3.3, y cómo revertirlo

Implementado tal cual: con un par configurado se aceptan los dos símbolos (en
mayúscula y minúscula) y **siempre** `↊`/`↋`; los alias heredados `X`/`T`/`E`
sólo valen si son el par configurado. Con `a`/`b`, teclear `E` da:

```
error: 'E' is not a number in the current alphabet: ten is 'a', eleven is 'b'
(the canonical glyphs ↊ and ↋ are always accepted too)
```

Revertirlo es **una línea**: `REJECT_LEGACY_ALIASES = false` en
`display_alphabet.rs`. Con el alfabeto por defecto la cuestión ni se plantea:
`is_canonical()` cortocircuita las dos traducciones a la identidad, así que el
comportamiento de siempre —incluidos `X`, `T` y `E`— queda intacto y ningún test
existente cambió de expectativa.

### 5.4 Lo que NO se hizo

- **No se tradujo el CSV ni el JSON**, por el §3.4. Es una decisión, no un
  olvido: son documentos de intercambio.
- **No se tradujo `HistoryEntry.input`.** Es el texto que el usuario tecleó, ya
  en su alfabeto; volver a traducirlo lo aplicaría dos veces. Consecuencia
  visible y aceptada: tras cambiar de alfabeto, la columna de entradas del
  historial conserva la ortografía con que se escribió cada línea y la de
  resultados se repinta entera.
- **No se guarda el alfabeto en el historial ni en `AppState` como preferencia
  persistente.** `Session.alphabet` es el valor de trabajo de Rust; la
  preferencia por máquina vive en `localStorage` y el frontend la re-aplica en
  el arranque, revalidándola contra el catálogo (§4).
- **No se avisa en el modal de un símbolo ausente en la fuente más allá de un
  texto.** `supportsSymbol` está y se usa, pero el aviso es una línea amarilla,
  no un bloqueo: es una preferencia visual y el usuario puede saber lo que hace.
- **No se tocó `glyphs.ts::display`** ni su respaldo automático a `X`/`E`. Sigue
  aplicándose sólo cuando quedan glifos canónicos que pintar, que es exactamente
  el caso del alfabeto por defecto.
- **No se añadió un alfabeto configurable para el resto de los dígitos.** No se
  pidió, y `0`-`9` no son ambiguos con nada.

### 5.5 Verificación visual

Comprobado en pantalla con `npm run tauri build -- --no-bundle` (nunca con
`cargo build --release`, por `docs/04` §9.5), capturando con
`PrintWindow` + `PW_RENDERFULLCONTENT` y clicando con `AttachThreadInput` +
`SetWindowPos(HWND_TOPMOST)` + `mouse_event`:

- el botón «Alfabeto: ↊↋» junto a «Sesión» abre el modal;
- el preset `A B` rellena los dos campos y la vista previa pasa a `1A;B` en vivo;
- «Aplicar» repinta el teclado (`A`/`B` en lugar de `↊`/`↋`) y el botón lateral;
- **`abs(3)` = 3** con el par `a`/`b` — la prueba de fuego del §2 —, y
  `abs(-A)` = `A`, y `A+1` = `B` (entrada y salida traducidas);
- el par `p`/`i` muestra en el modal, literal, el rechazo que escribió Rust:
  «the pair 'p'/'i' would make 'pi' read as a number instead of a name, so the
  function 'pi' could never be called»;
- «Restablecer» devuelve el teclado a `↊`/`↋` y **repinta el historial entero**
  (los resultados `B`/`A` vuelven a `↋`/`↊`);
- la preferencia sobrevive a cerrar y reabrir la aplicación (`localStorage`).
