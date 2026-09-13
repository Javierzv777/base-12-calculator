# Plan de investigación — Geometría L₁ + representación dozenal

Derivado de `planteamientos.md`. Este documento fija el **protocolo**: qué se
mide, con qué instrumento, y qué resultado refutaría la hipótesis. Los
hallazgos van a `01-hallazgos-*.md` y la síntesis a `99-sintesis.md`.

Fecha de apertura: 2026-09-12.

---

## 1. Qué se investiga (y qué no)

**H₁ (operativa, falsable).** En familias de problemas geométrico-analíticos
cuya estructura está dominada por particiones en 2, 3, 4, 6 y 12, la
combinación `(L₁ + dozenal)` produce expresiones de **menor complejidad
simbólica y menor coste aritmético medido** que su equivalente
`(L₂ + decimal)`.

**Lo que NO se afirma en ningún punto.** Que π = 4. π es la razón
circunferencia/diámetro **en L₂** y es trascendente. Lo que se estudia es
qué propiedades atribuidas a π son propiedades de la *métrica* y no del
*número*, y qué queda al sustituir la métrica.

**Descomposición obligatoria de todo hallazgo.** Cada resultado se clasifica
en exactamente una de tres casillas, porque confundirlas es el modo de fallo
principal de esta investigación:

| casilla | significado |
|---|---|
| **(I) intrínseco a L₂** | la propiedad *es* la geometría euclidiana; en L₁ no existe análogo, o existe pero es otra cosa |
| **(II) sobrevive a L₁** | la propiedad se traslada, con o sin simplificación; la constante cambia |
| **(III) efecto de base 12** | el cambio es de **representación**, ortogonal a la métrica |

Una simplificación de casilla (III) **nunca** es evidencia geométrica, y una
de casilla (II) **nunca** es evidencia a favor de la base 12. La hipótesis
H₁ es una conjunción, así que se puede refutar por cualquiera de sus dos
factores por separado.

---

## 2. Instrumentos de medida

### 2.1 K(F) — índice de complejidad simbólica

Del planteamiento, operacionalizado para que dos investigadores obtengan el
mismo número:

```
K(F) = N_raices + N_irracionales + N_operaciones + N_periodicos
```

Reglas de conteo, fijadas aquí:

- `N_raices` — número de **ocurrencias** de un radical o exponente
  fraccionario en la forma cerrada (no de radicandos distintos).
- `N_irracionales` — número de **constantes irracionales distintas** que
  aparecen (π, √2, e, γ cuentan una vez cada una, aunque se repitan).
- `N_operaciones` — `+ − × ÷ ^ |·| max min` aplicados en la forma cerrada.
  Un `|·|` cuenta 1. Una suma de k términos cuenta k−1.
- `N_periodicos` — número de constantes racionales de la fórmula cuya
  expansión **no termina** en la base considerada. Éste es el único término
  sensible a la base, y por eso es el que aísla el efecto dozenal.

K(F) es un **índice experimental, no una métrica canónica**. Su valor
absoluto no significa nada; solo se usan diferencias entre las dos
formulaciones de *la misma* fórmula, y se reporta siempre la descomposición
en los cuatro sumandos, nunca el total a secas.

**Sesgo conocido, declarado por adelantado.** K(F) premia lo lineal por
tramos y castiga los radicales, y L₁ es lineal por tramos por construcción.
Un resultado «L₁ baja K» es por tanto *casi tautológico* y tiene poco valor
probatorio por sí solo. Lo informativo es lo contrario: los casos donde L₁
**no** baja K, o donde lo baja al precio de perder una propiedad
(diferenciabilidad, unicidad, invariancia rotacional). Esos casos hay que
buscarlos activamente, no esperar a tropezar con ellos.

### 2.2 Coste en puertas — el instrumento objetivo

Este repositorio aporta algo que la discusión simbólica no tiene: un
**contador de coste independiente del criterio humano**. `dz-cli ... --gates`
devuelve invocaciones de primitivas, profundidad de circuito y
reconstrucciones para cualquier expresión dozenal.

Se usa para dos preguntas distintas, que no hay que mezclar:

1. **¿Cuánto cuesta evaluar la fórmula?** `total` y `depth` de la expresión
   cerrada, misma escala en ambas formulaciones.
2. **¿Cuánto cuesta la representación decimal frente a la dozenal?** El
   diferencial de `--input-base 10` frente a la lectura nativa, y el coste
   de la vuelta con `chain`. Esto mide (III) y **solo** (III).

Regla dura: **ningún número dozenal de esta investigación se calcula a
mano ni con una calculadora decimal.** Todo sale del binario, y cada cifra
publicada va acompañada del comando que la produjo. Motivo: el proyecto
entero existe para no importar significado desde aritmética nativa, y una
investigación que lo hiciera en su propio texto se contradiría.

### 2.3 Período dozenal — el discriminante barato

12 = 2²·3, así que termina en base 12 todo racional cuyo denominador
reducido solo tenga factores 2 y 3. 10 = 2·5 termina con 2 y 5. La
consecuencia es la predicción más nítida y más fácil de refutar de todo el
plan:

- una fórmula cuyos racionales tengan denominadores en `{2,3,4,6,8,9,12,...}`
  gana en dozenal (`N_periodicos` baja);
- una cuyos denominadores sean `{5,10,20,...}` **pierde** (`1/5` no termina
  en base 12);
- una con denominador 7, 11, 13 empata (no termina en ninguna de las dos).

Si al recorrer las diez familias la ganancia dozenal aparece **solo** por
esta vía, la conclusión honesta es que el efecto dozenal es aritmético-trivial
y no estructural. Eso sería una refutación parcial de H₁, y hay que estar
dispuesto a escribirla.

---

## 3. Reparto en cuatro líneas de trabajo

Las diez preguntas de `planteamientos.md` no son independientes; se agrupan
por el **tipo de respuesta** que admiten, no por tema.

| línea | preguntas | qué decide |
|---|---|---|
| **A — Constantes geométricas** | 1, 2, 7 | si hay un «π taxicab» único, o uno por propiedad conservada |
| **B — Lugares geométricos y trigonometría** | 3, 4, 5 | cuánto de lo cuadrático se vuelve lineal por tramos, y a qué precio |
| **C — Análisis** | 6, 8, 9 | dónde L₂ es **estructuralmente** necesaria (casilla I) |
| **D — Dozenal puro** | 10 + H₁ | si el efecto base 12 existe fuera de `N_periodicos` |

La línea **C es la más importante del plan** aunque sea la que menos
promete: es donde la hipótesis puede morir. Si la exponencial compleja, la
transformada de Fourier y la difusión dependen de L₂ de forma esencial —y
hay razón previa para creerlo: la invariancia rotacional, el teorema de
Plancherel y la simetría del laplaciano son propiedades de L₂ y solo de
L₂— entonces H₁ queda confinada a geometría elemental y hay que decirlo con
esas palabras.

---

## 4. Predicciones registradas antes de investigar

Se anotan aquí para que el resultado no se pueda releer a favor:

- **P1.** No existe un «π taxicab» único. La constante depende de la
  propiedad conservada (4 para perímetro/diámetro, 2 para área), y la razón
  es que en L₁ el círculo unitario no es isotrópico. *Confirmada ⇒ el
  hallazgo es de casilla (II) y desinfla la retórica de «π = 4».*
- **P2.** La bola L₁ es la única de la familia `Lp` con `p ∈ (1,∞)` cuyo
  perímetro/diámetro es entero, junto con L∞. Si aparece un `p` intermedio
  racional, P2 cae.
- **P3.** La trigonometría L₁ pierde las fórmulas de suma. `sin`/`cos`
  taxicab no pueden satisfacer una identidad de adición no trivial, porque
  la rotación no es isometría de L₁: el grupo de simetrías del rombo es
  finito (orden 8), no continuo. *Si P3 se confirma, no hay «matemática
  nueva» en el experimento 5, solo una parametrización del rombo.*
- **P4.** Fourier bajo L₁ no da Plancherel. La transformada de la bola L₁
  no es una función de Bessel elemental y **no** se simplifica.
- **P5.** El efecto dozenal medible se reducirá a `N_periodicos` y al coste
  de lectura de literales. No aparecerá ninguna simplificación *estructural*
  atribuible a la base. *Ésta es la predicción que más querría ver refutada,
  y la que la línea D debe atacar en serio.*
- **P6.** Goldbach no cede nada geométrico. Lo máximo defendible es una
  observación sobre residuos módulo 12: todo primo > 3 es ≡ 1, 5, 7, 11
  (mod 12), es decir, **último dígito dozenal ∈ {1, 5, 7, ↋}** — que es una
  reformulación legítima y visualmente limpia, pero es un hecho conocido
  (coprimalidad con 12), no un descubrimiento.

---

## 5. Criterios de cierre

Cada línea entrega un `01-hallazgos-<línea>.md` con, para cada pregunta:

1. enunciado L₂ y enunciado L₁, ambos explícitos;
2. casilla (I)/(II)/(III);
3. tabla K(F) desglosada en sus cuatro sumandos, decimal vs dozenal;
4. las cifras dozenales **con el comando `dz-cli` que las produjo**;
5. veredicto: confirma / refuta / no concluyente, y por qué;
6. una sección **«lo que no funcionó»**, obligatoria y no vacía.

El punto 6 no es adorno. Una investigación sobre una hipótesis atractiva y
vaga que no produzca refutaciones no ha investigado: ha ilustrado.

H₁ se declara:
- **confirmada** solo si hay ≥3 familias con ganancia en K *y* en coste
  medido, **y** al menos una de esas ganancias no proviene de
  `N_periodicos`;
- **parcialmente confirmada** si la ganancia existe pero es toda de
  `N_periodicos` o toda de casilla (II);
- **refutada** si las ganancias en K vienen acompañadas de pérdida de
  propiedades (suavidad, unicidad, invariancia) en todos los casos
  examinados.
