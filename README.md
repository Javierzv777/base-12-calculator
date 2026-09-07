# Investigación Base 12

Calculadora dozenal (base 12) construida **desde puertas lógicas de 12
valores**. Proyecto de investigación: no es una calculadora que muestre
resultados en base 12, es una que los *calcula* en base 12, sin delegar
nunca en la aritmética nativa del lenguaje.

## La regla que gobierna todo el repositorio

> **Ningún operador aritmético nativo de Rust se aplica jamás a un valor
> dozenal.**

`Digit` es `struct Digit(u8)` y **no implementa** `Add`, `Sub`, `Mul`,
`Div` ni `Ord`. El `u8` interno es una etiqueta de símbolo, no una
cantidad. Suma, multiplicación, división y comparación son operaciones
**derivadas**: salen de cinco primitivas de lógica multivaluada
(`cycle`, `min`, `max`, `lit`, `thr`), igual que un sumador real sale de
puertas.

Un lint (`tools/lint-purity.ps1`) verifica las cuatro reglas en cada
cambio. La aritmética nativa sobre índices de `Vec` y contadores de bucle
sí está permitida; sobre el *valor* de un dígito, nunca.

## Qué hay

- **Núcleo en Rust**, por capas, cada una llamando solo hacia abajo:
  puertas → enteros → fraccionarios → funciones científicas → parser →
  evaluador → API.
- **CLI** (`dz-cli`): evaluación, conteo de puertas y profundidad, trazas
  paso a paso de suma/resta/multiplicación, comparación de los tres
  backends, export a CSV y JSON.
- **GUI** (Tauri + TypeScript): dos calculadoras en paralelo —una que se
  teclea en base 10 y otra dozenal— más un panel de investigación.

## Empezar

```bash
cargo test --workspace --release     # la suite completa
cargo run -p dz-cli                  # REPL interactivo
cargo run -p dz-cli -- --help        # modo de una invocación

cd app && npm install
npm run tauri dev                    # GUI en desarrollo
```

Ejemplos:

```
$ dz-cli eval "4;6 * 3" --mode fixed
11;6

$ dz-cli eval "sin(1)" --mode fixed --scale 12 --gates
~0;X1208XX92234
gates: cycle=5743188 min=9039834 max=6706775 lit=4594776 thr=1054968 total=27139541 depth=18397 recon=0

$ dz-cli trace add EEE 1
trace: add  EEE + 1 = 1000
  pos  a  b  cin  sum  cout
    0  E  1   0    0     1
    1  E  0   1    0     1
    2  E  0   1    0     1
result: 1000
```

`;` es el punto Humphrey (`4;6` es cuatro y medio) y los dígitos son
`0-9` más `↊` (diez) y `↋` (once), con alias ASCII `X`/`E`.

## Hallazgos medidos

Lo interesante del proyecto no es que funcione, sino lo que se puede
medir cuando cada operación lleva su factura de puertas.

**La descomposición CRT no compensa.** `Z12 ≅ Z4 × Z3` parece prometedora,
pero destruye el orden: toda comparación (`min`, `max`, `thr`) obliga a
reconstruir el dígito. Los cinco contadores primitivos salen idénticos a
los de MVL —son iguales por construcción— y lo que separa a los backends
es una sexta medida:

| `mul 25 34` | puertas | profundidad | reconstrucciones |
|---|--:|--:|--:|
| MVL | 83 927 | 441 | **0** |
| CRT | 83 927 | 441 | **118 630** |

**El carry-lookahead gana desde n = 1.** Contra lo esperado, no sube el
conteo total: el `full_adder` del ripple encadena dos `half_adder` por
posición, y el lookahead usa uno solo más un mux de 5 puertas.

| n dígitos | ripple | lookahead |
|--:|--:|--:|
| 1 | 303 (prof. 30) | 157 (prof. 17) |
| 16 | 4 848 (prof. 270) | 2 644 (prof. 25) |

**Leer base 10 cuesta; leer base 12 es gratis.** Porque la representación
*es* la base: la posición en el `Vec` ya es el valor posicional. En base
10 cada peso hay que calcularlo. Mismo valor en ambas bases, así que la
diferencia es solo la lectura:

| cifras | decimal / dozenal | base 10 | base 12 | la lectura |
|--:|---|--:|--:|--:|
| 2 | `12` / `10` | 12 276 | 10 492 | **1 784** |
| 6 | `248832` / `100000` | 57 920 | 28 736 | **29 184** |

Y la vuelta cuesta ~7× la ida: escribir un decimal es división
restauradora, el circuito más caro del proyecto.

## Documentación

Las notas de investigación están en `docs/`, en español; el código y sus
comentarios, en inglés. `CLAUDE.md` tiene el mapa de crates y la
arquitectura; `PLAN.md`, el plan por fases.

## Licencia

Apache License 2.0 — ver [LICENSE](LICENSE).

Si copias, adaptas o construyes sobre este trabajo —en código, en un
paper o en material docente— **por favor indica de dónde viene y enlaza
al repositorio original**. El fichero [NOTICE](NOTICE) recoge esa
atribución y la sección 4 de la licencia obliga a arrastrarlo a las obras
derivadas. Para citarlo formalmente, [CITATION.cff](CITATION.cff).
