# 10 — Entrada en base 10, convertida con puertas

Pedido: que se pueda **escribir** en base 10 y **ver** el resultado en base 12,
sin que eso abra una segunda vía de cálculo por fuera de las puertas.

Estado de partida: rama `feat/phase7-research-panel`, commit `9916104`.

---

## 1. Por qué esto no traiciona el principio del proyecto

`PLAN.md` §0 dice que lo que separa este proyecto de una calculadora normal es
justamente **no** convertir a base 10 por debajo. Y `docs/08` §1 decidió, para la
herramienta de pegado, escribir la conversión en TypeScript con `BigInt` — fuera
del núcleo, precisamente para no meter aritmética nativa en `crates/`.

Esta fase hace lo contrario, y por eso es admisible: la conversión **también se
hace con puertas**. No hay ningún camino nuevo por el que un valor llegue a la
pantalla sin haber pasado por `DozGates`. Lo único que cambia es *dónde empieza*
la aritmética: antes empezaba en el evaluador, ahora empieza en el lexer.

`docs/08-conversion-base10.md:75-77` dejó esta puerta abierta por escrito:

> Si alguien en el futuro necesita esta conversión *dentro* de un cálculo, la
> respuesta correcta no es importar este módulo: es implementarla con puertas en
> `dz-int`, como manda el principio del proyecto.

Eso es exactamente lo que se hizo.

### 1.1 El hallazgo que justifica la fase

| | acumular *d* cifras | por qué |
|---|--:|---|
| base 12 | **0 puertas** | la posición en el `Vec` *ya es* el valor posicional |
| base 10 | crece con *d²* | ninguna posición significa 10ⁱ; el peso hay que calcularlo |

Base 12 es gratis **porque la representación es la base**. Aceptar decimal
cuesta, y ese coste es medible (§5) y publicable. La fase no diluye la tesis: le
añade un experimento que la mide por su reverso.

---

## 2. Dónde vive cada pieza

Las capas siguen llamando solo hacia abajo:

```
dz-cli  →  dz-api  →  dz-parse::input_base  →  dz-int::{from_base10_digits, to_base10_digits}
```

### 2.1 `crates/dz-int/src/dzint.rs` — la aritmética

```rust
/// Horner: ((d0 x diez + d1) x diez + d2) ...
pub fn from_base10_digits<G: DozGates>(g: &G, digits: &[Digit]) -> Dz;

/// Horner al revés: div_rem repetido entre diez.
pub fn to_base10_digits<G: DozGates>(g: &G, n: &Dz) -> Vec<Digit>;

/// Diez a la k, para el denominador de una fracción decimal.
pub fn pow10<G: DozGates>(g: &G, k: usize) -> Dz;
```

Tres decisiones que no hay que «simplificar»:

- El diez se construye con `Digit::new(10)`, que **nombra un símbolo con un
  literal**; no computa nada. Es el mismo movimiento que `div_rem_net` ya hacía
  en `dzint.rs` para su dígito de tanteo, y el lint de pureza ya lo daba por
  bueno.
- Todo el cuerpo son `Dz::mul` / `Dz::add` / `Dz::div_rem`, es decir puertas
  compuestas. **No hay un `*_net` propio**: componiendo operaciones de `Dz`, el
  nivel de profundidad viaja dentro del propio `Dz` y la medida sale gratis
  (misma razón por la que `dz-num`/`dz-sci`/`dz-eval` no necesitaron cambios en
  la Fase 7).
- Los dígitos llegan **ya mapeados** como `&[Digit]`, igual que en
  `DzFixed::from_digits`. `dz-int` es capa 1 y la tabla carácter ↔ símbolo vive
  en capa 3.

`pow10` no tiene análogo gratuito: doce a la *k* es empujar *k* ceros a una
magnitud (cero puertas), diez a la *k* hay que multiplicarlo.

### 2.2 `crates/dz-parse/src/input_base.rs` — el pre-paso de texto

Hermano de `display_alphabet.rs` y con la misma forma, incluido el **par de
direcciones**:

```rust
pub enum InputBase { Twelve, Ten }              // Twelve es la identidad

pub fn to_dozenal_text<G>(g, base, text)   -> Result<String, InputBaseError>;
pub fn from_dozenal_text<G>(g, base, text) -> Result<String, InputBaseError>;
```

La diferencia con `DisplayAlphabet` es que ninguna de las dos direcciones es una
sustitución: **las dos son aritmética**, y ni siquiera la misma. La ida son
multiplicaciones, la vuelta son divisiones, y el divisor restaurador es el
circuito más caro del proyecto. Eso se ve en los números de §5.

**Lo que se reutiliza en vez de re-derivarse** (la lección de `docs/07` §2):

- `lexer::is_identifier_run` decide nombre contra número. Una tira de letras que
  el lexer canónico leería como *número* (`E`, `XE`) es exactamente la que base
  10 tiene que rechazar; una que lee como *nombre* (`abs`, `exp`, `tan`) pasa
  intacta. No hay una segunda copia de la regla.
- `lexer::CallParens` decide qué significa una coma. Se extrajo del lexer a un
  tipo `pub(crate)` que ahora **usan los dos**: dentro de una llamada la coma
  separa argumentos y termina el literal, fuera es el separador de agrupación y
  se descarta, así que `1,000,000` se lee como un solo número de siete cifras y
  `log_base(100,10)` sigue siendo dos argumentos.

**La simplificación, dicha como tal:** en base 10 **no** se aplica
`DisplayAlphabet::to_canonical`. Sin letras-dígito, una tira de letras es
inequívocamente un nombre y no hay nada que traducir: `abs` es `abs` sea cual sea
el par configurado para diez y once. Es una simplificación real del pipeline, no
un descuido, y además es la razón de que los dos ajustes compongan sin que
ninguno tenga que conocer al otro.

### 2.3 Las fracciones decimales

Un literal con parte fraccionaria **no** se convierte a una fracción dozenal
truncada: se reescribe como una división entre paréntesis.

```
0.5   →  (5/↊)      que DzRational reduce a 1/2, exacto
0.1   →  (1/↊)      que no termina en base 12, y sigue exacto en modo racional
1.25  →  (↊5/84)    125/100
```

Escribir aquí un literal dozenal truncado tiraría la exactitud antes de que el
evaluador viera el valor. El denominador sale de `pow10`, o sea de una cadena de
multiplicaciones con puertas: no hay potencias de diez gratis.

En modo fijo o DFP la división se trunca a la escala de trabajo, que es lo que
esos modos hacen con cualquier división.

---

## 3. La vuelta, y el fallo que evita

Existe el encadenado de resultado (`docs/09-encadenado-de-resultado.md`): tras
Enter, si el siguiente carácter es un operador, la caja se siembra con el
resultado. En base 12 eso es coherente, porque la caja y el resultado hablan el
mismo idioma.

**En una caja que se lee como base 10, no.** El resultado que se muestra es
dozenal. Sembrarlo crudo haría que la siguiente operación lo leyera como
decimal — un número distinto, sin nada en pantalla que lo avise. Es exactamente
la clase de fallo silencioso que este proyecto existe para no cometer.

Ejemplo real, medido:

```
entrada (base 10):  144 + 11
resultado:          10↋        (155 decimal)
```

`10↋` ni siquiera es tecleable en base 10 — pero un resultado dozenal que *sí*
sea tecleable (`103`, `12`, `1000`) es peor todavía, porque se leería como un
decimal perfectamente válido y el cálculo continuaría desde el número
equivocado.

Por eso `from_dozenal_text` convierte de vuelta, y también con puertas:

```
10↋  →  155
```

### 3.1 Exactitud, por modo

La vuelta parte de texto ya formateado, así que solo hay dos formas posibles y
cada una recibe un trato distinto — porque solo una de las dos puede escribirse
como decimal finito:

- **Racional** (`n/d`): son dos **enteros**. Convertir cada uno por separado es
  exacto siempre. La cara simple está fijada a racional precisamente para no
  salir nunca de este caso.
- **Fijo y DFP** (`i;f`): el valor es `magnitud / 12^escala`. Exacto como
  fracción, pero a menudo no como decimal — `0;4` es un tercio y no tiene forma
  decimal finita. **No se trunca en silencio**: se emite la fracción exacta en
  numerales base 10.

```
0;4   →  4/12       exacto; un tercio no tiene decimal finito
11;6  →  162/12     exacto; trece y medio
1/↋   →  1/11       racional, exacto por los dos lados
```

Es la misma decisión que `docs/08` §3.2 ya tomó en la dirección contraria, cuando
`0.1` no terminaba en base 12 y salió como `1/↊`. Coherencia con el precedente,
no una regla nueva.

---

## 4. La frontera de medición, que hubo que mover

`dz_api::execute_with` abría el `DepthScope` y llamaba a `reset_cost()`
**después** de `build_expr`, a propósito: para que quien leyera `gate_count`
viera el coste de la aritmética y no del parseo.

Ese razonamiento daba por hecho que parsear no es aritmética. Es cierto en base
12 —los dígitos de un literal ya *son* su magnitud, y el lexer es un
`Vec::push`— y es falso en base 10. Dejar la ventana donde estaba habría
reportado como cero justo el coste que la fase existe para enseñar.

Así que la ventana se abre ahora **antes** de `build_expr`. Lo mismo en
`research::measured_trace`, que además construye el backend antes de leer los
operandos, porque leerlos es trabajo de puertas.

**Ese movimiento no cambió ni un número publicado, y está comprobado, no
afirmado:**

- `cargo run -p dz-gates --example gate_counts --release` → `diff` **vacío**
  contra la salida previa al cambio;
- `dz-cli eval "1+1" --mode fixed --gates` → **24.083** puertas, profundidad
  **441**;
- `dz-cli eval "4;6*3" --mode fixed --gates` → **120.444** puertas, profundidad
  **1.153**.

---

## 5. Lo que cuesta (medido, backend MVL, modo fijo)

### 5.1 La ida: leer un decimal

Cada fila es **el mismo valor** escrito en las dos bases, así que lo que gasta el
evaluador se cancela y la diferencia es puramente la lectura. Las potencias de
doce hacen el emparejamiento evidente.

| cifras | decimal | dozenal | puertas leyendo base 10 | puertas leyendo base 12 | **diferencia = la lectura** |
|--:|---|---|--:|--:|--:|
| 1 | `1` | `1` | 5.883 | 5.883 | **0** |
| 2 | `12` | `10` | 12.276 | 10.492 | **1.784** |
| 3 | `144` | `100` | 20.405 | 15.053 | **5.352** |
| 4 | `1728` | `1000` | 30.924 | 19.614 | **11.310** |
| 5 | `20736` | `10000` | 43.227 | 24.175 | **19.052** |
| 6 | `248832` | `100000` | 57.920 | 28.736 | **29.184** |

Profundidad en la misma pasada: leyendo base 12 es **254 en todas las filas**
(no hay circuito que atravesar); leyendo base 10 es 254, 254, 478, 816, 1187,
1658 — crece porque Horner encadena una multiplicación por cifra.

**Una cifra sola es gratis.** `0`-`9` denotan el mismo símbolo en las dos bases,
y el primer paso de Horner multiplica un acumulador que es cero, que `Dz::mul`
cortocircuita. El precio empieza en la segunda cifra, que es la primera que
necesita un peso.

La curva es superlineal, como se esperaba: la magnitud del acumulador crece a
cada paso, así que la multiplicación *i*-ésima es más cara que la anterior.

### 5.2 La vuelta: escribir un decimal

| cifras | dozenal | decimal | puertas |
|--:|---|---|--:|
| 1 | `7` | `7` | **48** |
| 2 | `10` | `12` | **11.485** |
| 3 | `100` | `144` | **31.534** |
| 4 | `1000` | `1728` | **69.355** |
| 5 | `10000` | `20736` | **134.060** |
| 6 | `100000` | `248832` | **201.590** |

Y los dos casos de encadenado que no son enteros:

| entrada | modo | display | vuelta | puertas |
|---|---|---|---|--:|
| `1/11` | racional | `1/↋` | `1/11` | **6.282** |
| `1/3` | fijo | `0;4` | `4/12` | **11.533** |

**La vuelta cuesta del orden de siete veces la ida** para el mismo número de
cifras (201.590 contra 29.184 a seis cifras). No es una sorpresa y es el mismo
hallazgo que ya domina `docs/05-funciones-cientificas.md`: la ida son
multiplicaciones y la vuelta es **división restauradora**, que es el circuito más
caro que tiene el proyecto. Una cifra decimal de salida cuesta un `div_rem`
entero sobre la magnitud restante.

Consecuencia práctica para la interfaz: encadenar en base 10 no es gratis, y la
cara simple hace bien en enseñar el número de puertas — el encadenado tiene su
propia factura, separada de la del cálculo.

---

## 6. Interfaz de terminal

```
dz-cli eval "144+11" --input-base 10            # → 10E   (155 decimal)
dz-cli eval "0.5"    --input-base 10            # → 1/2
dz-cli eval "144+11" --input-base 10 --gates    # la factura incluye la lectura
```

En el REPL:

```
:inputbase 10       cambia la base de lo que se teclea
:inputbase          la muestra
:session            la incluye
```

`:inputbase`, a diferencia de `:alphabet`, **no repinta nada**: cambia cómo se
lee la *siguiente* línea. Lo que ya está en el historial se leyó en la base que
estuviera vigente entonces, y la salida es dozenal en cualquier caso.

---

## 7. Primera oleada: lo implementado, y en qué se desvió

Se implementó todo el núcleo Rust y toda la interfaz de terminal. La interfaz
gráfica (la tercera pestaña, la cara simple, las dos sesiones en paralelo) es de
la segunda oleada y **no se tocó** ningún fichero de `app/`.

Desviaciones respecto del plan, todas conscientes:

1. **`CallParens` se extrajo del lexer.** El plan decía reutilizar la regla de
   tiras maximales de letras, y eso se hizo; pero hacía falta *también* la regla
   de la coma, que estaba escrita a mano dentro de `lex`. Copiarla habría sido
   exactamente el fallo que `docs/07` §2 vigila, así que se sacó a un tipo
   `pub(crate)` que usan los dos. Sin ese paso, `1,000,000` en base 10 se habría
   leído como `100` en silencio.

2. **Una cifra decimal sola cuesta 0 puertas.** El plan pedía fijar por test que
   «el coste de la conversión es > 0 y crece con el número de cifras». Es cierto
   a partir de dos cifras; con una sola es honestamente cero, porque `0`-`9` son
   el mismo símbolo en las dos bases. El test fija las dos cosas por separado en
   vez de forzar un `> 0` que sería falso.

3. **Los ejemplos del pedido de la vuelta no salían.** El encargo ilustraba el
   fallo con «`144 + 11` da `103`, que volvería como `147`». El valor real es
   `10↋` (155 decimal): `144 + 11` en base 10 es ciento cincuenta y cinco, no
   ciento cuarenta y siete. El fallo que describe es real y está arreglado; solo
   los números del ejemplo estaban mal, y los tests usan los verdaderos.

4. **`compare_backends` y `TraceRequest` ganaron el ajuste.** Por la regla de
   no-divergencia de `docs/06` §7: si `Request` acepta una base de entrada,
   todas las puertas de entrada de `dz-api` la aceptan. Eso cambia la firma de
   `compare_backends` y añade un campo a `TraceRequest`, lo cual **rompe la
   compilación de `app/src-tauri`** en dos líneas (`research.rs:207` y
   `research.rs:225`). Se dejó roto a propósito en vez de tocar `app/`, que es
   de la segunda oleada.

5. **`chain_operand` en vez de un campo de `Response`.** La vuelta se expone
   como función de `dz-api`, no como un campo más de la respuesta, porque
   encadenar es un gesto *posterior* del usuario: el resultado se muestra
   primero y solo se convierte si luego se teclea un operador. Meterlo en
   `Response` habría cargado a toda llamada con una conversión que nadie pidió,
   contaminando `gate_count` con trabajo que la petición no encargó.

### Lo que no se hizo

- **No hay separador de miles en base 10 dentro de una llamada.** Fuera de una
  llamada `1,000,000` funciona; dentro, la coma separa argumentos y hay que
  escribir `1000000`. Es la misma limitación que base 12 ya tenía con `1,0000`,
  por la misma razón posicional.
- **La vuelta en modo fijo emite una fracción, no un decimal finito.** Cuando el
  valor *sí* tiene forma decimal finita (`0;6` es exactamente `0.5`) se emite
  igualmente `6/12`. Detectarlo pediría factorizar el denominador, y la fracción
  ya es exacta y válida como expresión; se prefirió una sola regla a dos.
- **No se convierte de vuelta el display por defecto.** `Response.display` es
  siempre dozenal, en el mismo sentido en que es siempre canónico respecto del
  alfabeto. Convertirlo sin que nadie lo pida sería una segunda vía de cálculo
  sin medir, que es justo lo que la fase evita.
- **La GUI.** Segunda oleada — §8.

---

## 8. Interfaz gráfica: dos calculadoras en paralelo

La barra de pestañas pasa de dos a tres:

```
[ Calculadora dozenal desde base 10 ]  [ Calculadora dozenal ]  [ Investigación ]
            (por defecto)                  (la de siempre)
```

Las dos primeras son **calculadoras enteras puestas una al lado de la otra**, no
dos vistas de una. Cada una conserva su entrada a medias, su último resultado, su
historial, su memoria y sus ajustes; se puede tener una cuenta larga en la
dozenal y un tanteo aparte en la de base 10 sin que se pisen.

### 8.1 El estado sigue en Rust, duplicado, no movido

`app/src-tauri/src/state.rs` guardaba una `Session`. Ahora guarda dos, en
`Sessions { simple, advanced }`, y cada comando de `commands.rs` **nombra la cara
sobre la que opera** con un parámetro `face: Face` (`"simple"` / `"advanced"`).
Son doce comandos con un argumento más; el cuerpo de cada uno no cambió.

La alternativa —que la cara simple llevara su estado en JavaScript— se descartó
por lo que ya avisa `docs/06` §7: el historial guarda **texto canónico** y la
traducción de alfabeto la hace Rust, así que una copia en el frontend sería una
segunda autoridad sobre semántica dozenal. Con dos sesiones, la cara nueva no
añade ninguna regla: reusa la que ya existía.

Deliberadamente **no hay «cara actual» en Rust**. Si la hubiera sería un tercer
estado que las dos visibles podrían contradecir; que cada llamada diga de quién
habla es más largo de escribir y no puede desincronizarse.

Ajustes fijos de la sesión simple, y ninguno alcanzable desde su cara:
`input_base = Ten`, modo `rational`, backend MVL, escala por defecto. El modo
racional es el que carga peso: es el exacto, así que el display siempre es `n` o
`n/d` —dos enteros— y la vuelta de §3.1 nunca cae en el caso de punto fijo.

### 8.2 El encadenado inverso, que es la razón de ser de esta oleada

En la cara dozenal el encadenado (`docs/09`) es movimiento de texto: la caja y el
resultado hablan el mismo idioma. En la cara de base 10 **no**, y sembrar el
dozenal crudo sería un número equivocado sin nada en pantalla que lo avise.

`app/src/simple.ts` lo resuelve con `dz_api::chain_operand`, a través del comando
`chain_operand(face)`. Tres decisiones:

- **No lleva valor.** Igual que `memory_store`, lee la copia **canónica** que
  vive en Rust. Que el frontend devolviera el string de pantalla congelaría la
  conversión en el alfabeto que estuviera puesto.
- **Es perezoso.** No se convierte al calcular, sino al teclear el operador. Por
  eso `chainOn` es asíncrono aquí y síncrono allí: hay un viaje a Rust de por
  medio. Convertir por adelantado gastaría puertas en todo cálculo del que nadie
  encadena.
- **Su coste se enseña aparte.** La línea del pie pasa a decir
  `80.043 puertas · 33.366 puertas al reescribirlo en base 10 · ver`. La factura
  del cálculo y la del encadenado son dos cosas distintas y §5.2 explica por qué
  la segunda es cara.

Medido en pantalla:

```
144 + 11   →  10↋                        80.043 puertas
pulsar *   →  la caja queda en  155*     33.366 puertas
155 * 2    →  21↊                        81.985 puertas
```

### 8.3 El anzuelo

Al pie del resultado, una línea discreta con el número de puertas y un `ver` que
lleva a Investigación. Alguien teclea `144 + 11`, lee `10↋`, y esa línea le hace
preguntarse qué pasa por debajo. No es decoración: es la única vía por la que
esta cara enseña que hay un proyecto de investigación detrás.

### 8.4 A qué sesión mira Investigación

El panel vive al lado de la calculadora dozenal y su formulario de trazas es
suyo: `trace_operation` lee siempre la sesión avanzada.

La **comparación de backends**, en cambio, lleva `face`. El desglose de «última
operación» es el de quien calculó por último —los números que ya venían en su
`Response`, sin recalcular nada—, y si esa fue la cara simple, re-ejecutarla en
los tres backends solo significa lo mismo si sus operandos se vuelven a leer en
base 10. `144 + 11` es otra cuenta según la base. Lo que **no** se hace es
mezclar los dos historiales en una tabla: cada cara enseña el suyo.

`last_comparison` / `last_trace` se guardan siempre en la sesión avanzada: el
panel es un sitio con un par de botones de export, así que lo último que produjo
tiene un solo sitio donde estar.

### 8.5 Desviaciones de esta oleada

1. **La pestaña activa se recuerda, la cara activa no.** El plan pedía persistir
   la pestaña en `localStorage`, y así es (`dz.view`, junto a `dz.alphabet`).
   Pero eso es una preferencia de esta máquina sobre *qué se ve*, no estado de
   cálculo: el estado de cálculo vive en Rust y muere con el proceso, igual que
   antes de esta fase.

2. **El alfabeto se aplica a las dos sesiones.** Es un ajuste **visual por
   máquina**, no por cara, y dos caras dibujadas en la misma pantalla deletreando
   el once de dos maneras sería una diferencia que nadie pidió. El diálogo sigue
   estando solo en la cara avanzada, así que esto no le añade un ajuste a la
   simple; `alphabet.ts` llama a `set_alphabet` una vez por cara y `simple.ts`
   se repinta con el gancho `onOtherFace`.

3. **`↊`/`↋`/`;` se rechazan en la caja, no solo faltan del teclado.** El plan
   decía «sin `↊`/`↋`». El teclado no los tiene, pero pegar sí podía meterlos, y
   entonces la promesa se apoyaba en el mensaje de error de Rust. La caja los
   filtra en `keydown` y otra vez en `input` (que sí cubre pegar y dictado). No
   es una segunda copia de la regla de base 10: es la promesa del propio teclado.
   Todo lo demás —`X`, `E`, un nombre de función, dos puntos decimales— sigue
   yendo a Rust para que lo juzgue él, con su mensaje.

4. **`compare_backends` ganó `face`.** El plan decía que Investigación mira la
   sesión avanzada. Se cumple para las trazas y para dónde se guarda el export,
   pero la comparación tenía que aceptar la cara de la operación que compara, o
   habría re-leído en base 12 unos operandos escritos en base 10 y comparado otra
   cuenta. Es la misma regla de no-divergencia de `docs/06` §7 aplicada un nivel
   más arriba.

5. **La cara simple enfoca su caja después de calcular.** La dozenal no lo hace
   —tras `=` en el teclado, el foco se queda en el botón—, pero allí no importa
   tanto: aquí el encadenado es la interacción que da sentido a la cara, y solo
   es alcanzable desde la caja. Es una línea que se añadió a `simple.ts` y no
   toca la cara de siempre.

### 8.6 Lo que no se hizo

- **La cara simple no tiene memoria (`M+`/`MR`/`MC`).** El plan la listaba entre
  el estado que cada cara conserva, y la sesión la tiene, pero el teclado que
  pedía es `0-9 . operadores = C`: añadir tres botones más habría contradicho el
  encargo. El registro está ahí para cuando se pida.
- **La cara simple no tiene el `10→12`.** Es la herramienta de texto de
  `docs/08`, y en una caja que ya se lee como base 10 no significaría nada. Sigue
  en la cara dozenal, intacta.
- **No hay científicas en la cara simple.** Ni teclado científico ni catálogo.
  Escribir `sqrt(2)` en su caja funciona —el evaluador es el mismo—, pero nada en
  la interfaz lo sugiere.
- **La verificación visual de la cara dozenal es por comparación, no por
  captura de referencia.** No existía una captura previa contra la que hacer
  `diff`; lo que se comprobó es que la pestaña «Calculadora dozenal» muestra el
  mismo teclado dozenal, los mismos selectores, el mismo `10→12`/`⇄`, el mismo
  botón de alfabeto y las mismas secciones de coste, memoria e historial.
