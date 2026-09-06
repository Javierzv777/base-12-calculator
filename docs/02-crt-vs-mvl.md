# 02 — CRT vs MVL: cuantificando el costo de reconstruir el orden

## Resumen del resultado

**La descomposición CRT no es más barata que MVL directo para este
conjunto de primitivas, y en la práctica es más cara**, porque cuatro de
las cinco primitivas (`min`, `max`, `thr`, y transitivamente `cycle`)
necesitan reconstruir el dígito completo desde sus residuos antes de poder
compararlo con nada. Esto confirma la sospecha del plan (Sección 2 y
Sección 12): "la CRT destruye el orden [...] Cuantificar ese coste es un
resultado de investigación por sí mismo" y "puede resultar tan cara que
anule la ventaja del backend CRT. Es un resultado válido de la
investigación, no un fallo."

## Por qué

`Z12 ≅ Z4 × Z3` (`crates/dz-gates/src/backends/crt.rs`) representa
internamente un dígito como el par de residuos `(x mod 4, x mod 3)`. La
suma y la multiplicación *módulo 12* se descomponen limpiamente
componente a componente — esa es la ventaja clásica de CRT. Pero el orden
en `Z12` **no** es un orden producto sobre `(Z4, Z3)`: no hay forma de
saber si `x >= j` mirando `q = x mod 4` y `t = x mod 3` por separado sin
reconstruir `x`. Concretamente, en la implementación:

- `lit(j, x)`: **sí** se puede resolver sin reconstruir `x` completo —
  igualdad componente a componente (`q_x == q_j && t_x == t_j`) es
  suficiente, porque CRT es una biyección y la igualdad sí es una
  propiedad "de producto".
- `min`, `max`, `thr` (y `cycle`, que necesita reconstruir para devolver un
  tag válido tras la operación por componentes): **no**. Cada uno paga una
  reconstrucción completa vía una tabla `RECON[q][t]` de 12 entradas antes
  de poder comparar nativamente.

## Medición

`CrtGates` cuenta las reconstrucciones por separado del conteo de puertas
de 5 primitivas (que, por diseño, es idéntico entre backends — ver
`01-diseno-sumador.md`). Medido con
`cargo run -p dz-gates --example gate_counts`:

| Operación | invocaciones de primitivas (igual en MVL y CRT) | reconstrucciones CRT |
|---|--:|--:|
| `half_adder(7, 9)` | 151 | **176** |
| `multiply` shift-add (3 dígitos × 2 dígitos) | 201 243 | **235 266** |

Es decir: **por cada invocación de una primitiva en este conjunto de
circuitos, CRT realiza en promedio más de una reconstrucción completa**
(176/151 ≈ 1.17 para el sumador; 235266/201243 ≈ 1.17 para el
multiplicador — consistente entre operaciones, como se esperaría de que la
proporción de llamadas a `min`/`max`/`thr`/`cycle` frente a `lit` es fija
en estos circuitos). Cada reconstrucción es, como mínimo, dos operaciones
de residuo (`% 4`, `% 3`) más una consulta a tabla de 12 entradas — del
mismo orden de magnitud que la comparación nativa de un solo `u8` que hace
el backend MVL directamente, sin ningún paso previo.

## Conclusión

Para el conjunto de circuitos aquí construidos (que están dominados por
`min`/`max`/`thr` en los decodificadores `MAX_j MIN(...)`), **CRT añade
trabajo, no lo quita**. La ventaja teórica de CRT (suma/multiplicación
módulo 12 "gratis" por componentes) nunca se materializa en un ahorro neto
porque este diseño de circuito no tiene ninguna operación que sea *pura
suma/multiplicación modular sin comparación* — hasta el sumador necesita
`thr` para el acarreo. Un circuito que evitara `min`/`max`/`thr`
enteramente (por ejemplo, una función que solo necesitara módulo 12 sin
comparar magnitudes) sí se beneficiaría de CRT; ninguno de los circuitos de
esta entrega es de ese tipo.

Esto es exactamente el resultado que el plan anticipaba como posible y
pedía documentar en vez de ocultar: la elegancia matemática de "Z12 es
producto de Z4 y Z3" no sobrevive el contacto con "pero necesito saber cuál
de los dos es más grande."

## El hallazgo, ahora observable desde las dos interfaces

Hasta la Fase 9 este documento era la única forma de saber lo anterior: la
tabla de `dz-cli compare` y la tabla de tres backends del panel de
investigación mostraban los **mismos cinco contadores** para MVL, CRT y
`table`, porque `GateCount` cuenta invocaciones de primitivas y los tres
backends implementan las mismas primitivas para el mismo circuito. Era un
dato correcto que invitaba a una lectura falsa («los tres cuestan lo
mismo»).

`DozGates` ganó `reconstruction_ops()` con cuerpo por defecto `0` —
adición no rompedora, el mismo patrón que usó la Fase 7 con
`GateNet::lift_at`. `CrtGates` lo sobrescribe; MVL y el oráculo se quedan
con el 0, que **es la verdad**: su representación interna ya es el tag, así
que no reconstruyen nada. `DynGates` lo reenvía como cualquier otro
contador. `dz-api` lo sube a `Response.reconstruction_ops` y a
`BackendCost.reconstruction_ops` — campo hermano de `depth`, nunca un
sexto contador dentro de `GateCount`, que se mantiene deliberadamente
agnóstico del backend al nivel de las cinco primitivas.

Salida real (`target/release/dz-cli.exe compare mul 25 34`):

```
result: 808
  backend     cycle      min      max      lit      thr      total    depth    recon
  mvl         16896    28156    20923    14376     3576      83927      441        0
  crt         16896    28156    20923    14376     3576      83927      441   118630
  table       16896    28156    20923    14376     3576      83927      441        0
  (table is the truth-table oracle, not a production backend)
  verdict: mvl is cheapest. mvl and crt tie on all five primitive counters
  (83927) and on depth (441), so the five headline numbers cannot separate
  them -- they are equal by construction, since every backend implements the
  same primitives for the same circuit. The tie is broken by
  reconstructions: mvl 0, crt 118630. Cost model: primitive invocations +
  internal->tag reconstructions, ranked lexicographically by (total, depth,
  reconstructions). The table backend is listed but not ranked: it is the
  truth-table oracle, not a production backend.
```

Y para una suma (`compare sum 1 1`), misma forma: 23.780 invocaciones y
profundidad 441 en los tres, 33.244 reconstrucciones en CRT y 0 en los
otros dos. La proporción reconstrucciones/invocaciones sigue siendo la de
la sección anterior (118.630/83.927 ≈ 1,41; 33.244/23.780 ≈ 1,40 — algo
más alta que el 1,17 de los circuitos aislados de `gate_counts`, porque
una expresión completa pasa por `div_rem` y por el comparador, que son
todavía más ricos en `min`/`max`/`thr`).

### Qué dice el veredicto y qué no puede decir

El veredicto lo redacta `dz_api::verdict` (`crates/dz-api/src/research.rs`)
y **solo ahí**: el CLI lo imprime plegado a 76 columnas y el frontend lo
inserta como texto. Dos autores serían dos respuestas, que es justo el
riesgo que fija `docs/06-fase9-paridad-cli.md` §7.

- **`table` no entra en el ranking.** Es el oráculo por tabla de verdad, no
  un backend de producción: indexa memoria en vez de componer puertas, así
  que «más barato que el oráculo» no es una respuesta de investigación. Su
  fila se muestra porque su dato es real, y la salida dice en voz alta que
  no se le clasifica.
- **El modelo de coste va escrito, no implícito**: «coste = invocaciones de
  primitivas + reconstrucciones interno→tag, ordenado lexicográficamente
  por (total, profundidad, reconstrucciones)». Sin eso, el veredicto sería
  otro oráculo.
- **No hay fórmula ponderada.** No existe tipo de cambio medido entre una
  invocación de primitiva y una reconstrucción, así que sumarlas con un
  coeficiente inventaría una precisión que el experimento no tiene. El
  orden lexicográfico dice exactamente lo que se sabe.
- **El empate se nombra.** Con estos circuitos las dos primeras claves
  siempre empatan, así que el veredicto lo declara («tie on all five
  primitive counters ... and on depth») antes de desempatar con la tercera.
  Un «mvl gana» a secas escondería precisamente lo interesante.

El resultado es consistente con la conclusión de arriba: MVL gana siempre,
CRT nunca. Si alguna vez esta tabla diera CRT como ganador, o el análisis
de este documento está mal o lo está la medición — sería un hallazgo, no un
detalle a maquillar.
