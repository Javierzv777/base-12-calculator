# Referencia medida — cifras dozenales comunes a las cuatro líneas

Todas las cifras de este fichero salen de `target/release/dz-cli.exe` en este
repositorio, con el comando al lado. Ninguna se convirtió a mano. Las líneas
A–D deben citar de aquí en vez de recalcular, para que los números del informe
final sean consistentes.

Alfabeto: dígitos `0`-`9`, `X` = diez, `E` = once. `;` es el punto fraccionario.
Un `~` inicial significa truncado a la escala de trabajo. `recon` es el número
de reconstrucciones interno→etiqueta, distinto de `total` (§5 del SKILL).

---

## 1. Recíprocos — el discriminante de §2.3 del plan

`target/release/dz-cli.exe eval "1/n" --mode fixed --scale 12`

| n (dozenal) | n (decimal) | factorización | 1/n en base 12 | ¿termina? |
|---|--:|---|---|---|
| 2 | 2 | 2 | `0;6` | sí |
| 3 | 3 | 3 | `0;4` | sí |
| 4 | 4 | 2² | `0;3` | sí |
| 5 | 5 | 5 | `0;2497 2497 2497…` | **no** |
| 6 | 6 | 2·3 | `0;2` | sí |
| 7 | 7 | 7 | `0;186X35 186X35…` | **no** |
| 8 | 8 | 2³ | `0;16` | sí |
| 9 | 9 | 3² | `0;14` | sí |
| X | 10 | 2·5 | `0;1249 7249 7249…` | **no** |
| E | 11 | 11 | `0;1111 1111 1111…` | **no** |
| 10 | 12 | 2²·3 | `0;1` | sí |
| 14 | 16 | 2⁴ | `0;09` | sí |
| 16 | 18 | 2·3² | `0;08` | sí |
| 18 | 20 | 2²·5 | `0;0724 9724 97…` | **no** |
| 20 | 24 | 2³·3 | `0;06` | sí |

Lectura honesta de la tabla, en los dos sentidos:

- **Gana base 12** en 3, 6, 9 y 24: `1/3 = 0;4` es un dígito, frente a
  `0.333…` en decimal. Ésta es la única ventaja dozenal que el plan llama
  «limpia», y es puramente de casilla (III).
- **Pierde base 12** en 5, 10 y 20: `1/5` termina en decimal (`0.2`) y **no**
  termina en base 12. Cualquier familia de problemas con quintos pagará por
  el cambio, no cobrará.
- **Empata** en 7 y 11 (no terminan en ninguna de las dos). Detalle curioso
  y sin consecuencias: `1/E = 0;111111…`, porque once = 12−1, exactamente el
  papel que juega `1/9 = 0.111…` en decimal. Es una coincidencia de
  representación, no un hecho aritmético nuevo.

## 2. Constantes

`target/release/dz-cli.exe eval "<expr>" --mode fixed --scale 8`

| constante | base 12 | comando |
|---|---|---|
| π | `~3;18480949` | `eval "pi" --mode fixed --scale 8` |
| π/2 | `~1;6X2404846` | `eval "pi/2" --mode fixed --scale 8` |
| π/3 | `~1;06968317` | `eval "pi/3" --mode fixed --scale 8` |
| π/4 | `~0;951202423` | `eval "pi/4" --mode fixed --scale 8` |
| π/6 | `~0;634941696` | `eval "pi/6" --mode fixed --scale 8` |
| √2 | `~1;4E79170X` | `eval "sqrt(2)" --mode fixed --scale 8` |
| √3 | `~1;894E98` | `eval "sqrt(3)" --mode fixed --scale 8` |

π no se vuelve racional ni periódico en base 12: es trascendente, y ninguna
base cambia eso. Las constantes análogas de L₁ (4 para perímetro/diámetro,
2 para área) se escriben `4` y `2` en cualquier base — y ahí está justamente
el punto que las líneas A y D deben separar: la simplificación viene de la
**métrica**, no de la base.

`pi` reporta 0 puertas a propósito (se sintetiza en un backend desechable y se
memoiza, para que la medición del usuario no dependa de si su llamada fue la
primera). La profundidad sí se reporta: `depth=15168`.

### 2.1 Aviso: no midas L₂ con `pi` (lo detectó la línea D)

Esa memoización es correcta como decisión de producto y **venenosa como
instrumento de esta investigación**: cualquier comparación L₁ frente a L₂ que
use `pi` sub-factura el lado euclidiano en órdenes de magnitud, porque cobra 0
por la constante más cara de la expresión. Para medir el coste real hay que
pedir la constante sin memoizar:

```
$ ../target/release/dz-cli.exe eval "pi" --mode fixed --scale 8 --gates
gates: ... total=0 depth=15168 recon=0

$ ../target/release/dz-cli.exe eval "4*atan(1)" --mode fixed --scale 8 --gates
gates: cycle=39942408 min=65355724 max=48283155 lit=32829756 thr=7307724 total=193718767 depth=42392 recon=0
```

Así que el perímetro del círculo L₂ frente al del «círculo» L₁ no es una
diferencia de factor pequeño: es **193.718.767 puertas contra ~4·10⁴**. Toda
cifra de comparación métrica de este informe debe decir con qué forma de π se
midió, y las que digan `pi` son cota **inferior**, no medición.

Dos rarezas más del instrumento, medidas, que hay que conocer antes de citar
cualquier conteo:

- **El coste no es invariante bajo conmutatividad.** `eval "pi*9"` cuesta
  19.556 puertas y `eval "9*pi"` cuesta 46.220, con el mismo valor y la misma
  profundidad (15.952). El multiplicador recorre los dígitos de un operando, y
  cuál de los dos ocupa esa posición cambia el trabajo. No es un error: es que
  el conteo mide **el circuito que se armó**, no la función matemática.
- **`--scale` no controla la escala de la división**, que la fija
  `DEFAULT_DIV_SCALE = 24`. Dos conteos con `--scale` distinto pueden salir
  iguales si lo que domina es una división.

## 3. El número central del proyecto: coste medido de L₂ frente a L₁

Mismo par de puntos, el triángulo (3,4,5), de modo que **el resultado
euclidiano es un entero exacto** y no hay truncamiento que achacar:

```
$ target/release/dz-cli.exe eval "sqrt(3*3+4*4)" --mode fixed --scale 8 --gates
5
gates: cycle=9212544 min=14542872 max=10787905 lit=7388748 thr=1699896 total=43631965 depth=16001 recon=0

$ target/release/dz-cli.exe eval "3+4" --mode fixed --gates
7
gates: cycle=7920 min=13236 max=9841 lit=6768 thr=1704 total=39469 depth=441 recon=0
```

| | d₂ = √(3²+4²) | d₁ = 3+4 | razón |
|---|--:|--:|--:|
| puertas | 43.631.965 | 39.469 | **≈ 1.105×** |
| profundidad | 16.001 | 441 | ≈ 36× |

Esto es lo más fuerte que este repositorio puede aportar a la hipótesis, y
también hay que decir con precisión **qué** demuestra y qué no:

- **Demuestra** que el coste no está en el valor sino en el algoritmo. El
  resultado `5` es exacto y entero, y aun así cuesta tres órdenes de magnitud
  más que la suma, porque `sqrt` corre Newton con divisiones restauradoras sin
  saber que va a caer en un entero. La medición es ciega al valor, que es
  exactamente lo que la hace un instrumento y no una opinión.
- **No demuestra** que L₁ sea «mejor». Son distancias distintas entre los
  mismos puntos: `5` y `7` no son dos respuestas a la misma pregunta. Comparar
  su coste es legítimo solo cuando el problema admite de verdad cualquiera de
  las dos métricas (rutas en una retícula, por ejemplo), y es ilegítimo cuando
  el problema pide la distancia euclidiana.
- **No es** evidencia dozenal. En base 10 con el mismo circuito el cociente
  sería del mismo orden. Es casilla (II), no (III).

## 4. Diferencial de lectura decimal vs dozenal (aísla la casilla III)

Mismo valor escrito en las dos bases, así que lo que gasta el evaluador se
cancela y la diferencia es solo el coste de **leer**:

```
$ target/release/dz-cli.exe eval "144" --input-base 10 --mode fixed --gates
$ target/release/dz-cli.exe eval "100" --mode fixed --gates
```

| cifras | decimal / dozenal | leyendo base 10 | leyendo base 12 | diferencia | prof. base 10 | prof. base 12 |
|--:|---|--:|--:|--:|--:|--:|
| 2 | `12` / `10` | 12.276 | 10.492 | 1.784 | 254 | 254 |
| 3 | `144` / `100` | 20.405 | 15.053 | 5.352 | 478 | 254 |
| 4 | `1728` / `1000` | 30.924 | 19.614 | 11.310 | 816 | 254 |
| 5 | `20736` / `10000` | 43.227 | 24.175 | 19.052 | 1.187 | 254 |

Dos observaciones, la segunda no publicada antes en el repositorio:

1. Base 12 es gratis **porque la representación es la base**: la posición en el
   `Vec` ya *es* el valor posicional. En base 10 cada peso hay que calcularlo
   (Horner), y la curva es superlineal porque el acumulador crece a cada paso.
2. La **profundidad** en base 12 es constante (254) sea el número de una cifra
   o de cinco, mientras en base 10 crece con las cifras. Horner es secuencial
   por construcción: cada dígito depende del acumulado anterior. Leer dozenal
   no tiene cadena de dependencias en absoluto.

Estas cifras reproducen exactamente la tabla publicada en `CLAUDE.md`, lo que
sirve de control: la instrumentación da hoy lo mismo que cuando se midió.
