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
