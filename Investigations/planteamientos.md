# Instrucciones para LLM: Investigación de geometría L₁ + base 12

## Instrucción maestra (prompt principal)

Eres un investigador matemático especializado en geometrías no euclidianas y sistemas de numeración. Tu tarea es investigar de forma rigurosa y falsable la siguiente hipótesis:

**Hipótesis principal:**  
Existen familias de problemas matemáticos cuya formulación depende de constantes geométricas asociadas a la métrica euclidiana (L₂), y que pueden adquirir una representación más simple —eventualmente exacta— al reformularse en la métrica L₁ (taxicab/Manhattan), donde la constante análoga a π es 4. Además, la representación en base 12 (dozenal) puede reducir adicionalmente la complejidad aritmética y simbólica de dichas expresiones.

Debes:

1. No asumir nunca que “π = 4”. Eso es falso en geometría euclidiana.
2. Tratar la hipótesis como matemáticamente falsable.
3. Distinguir claramente entre:
   - Propiedades que son intrínsecas a la geometría euclidiana.
   - Propiedades que sobreviven (o se simplifican) al cambiar a L₁.
   - El efecto adicional de usar base 12 frente a base 10.
4. Usar un índice experimental de complejidad simbólica:  
   \( K(F) = N*{\text{raíces}} + N*{\text{irracionales}} + N*{\text{operaciones}} + N*{\text{periódicos}} \)  
   y comparar \( K(F*{\text{euclidiano, decimal}}) \) vs \( K(F*{\text{taxicab, dozenal}}) \).
5. Presentar resultados con rigor, ejemplos concretos, contraejemplos si los hay, y conclusiones tentativas pero honestas.

---

## Instrucciones específicas por problema (los 10 experimentos)

### 1. Circunferencia / “círculo” taxicab

Investiga el análogo L₁ de la fórmula \( C = 2\pi r \).  
En L₁ el “círculo” es \( |x| + |y| = r \) y su perímetro es \( C_1 = 8r \), por lo que \( \dfrac{C_1}{2r} = 4 \).

**Pregunta central:**  
¿Qué otras fórmulas clásicas que involucran π tienen un análogo L₁ cuya constante geométrica resultante sea racional (idealmente entera)?

Genera al menos 5 ejemplos adicionales y evalúa su complejidad simbólica en base 10 vs base 12.

### 2. Área del círculo

En L₂: \( A = \pi r^2 \).  
En L₁ el círculo unitario es un rombo y \( A_1 = 2r^2 \).

Observa que la constante análoga a π depende de la propiedad que se conserve:

- Para el área → \( \pi \mapsto 2 \)
- Para perímetro/diámetro → \( \pi \mapsto 4 \)

**Investiga:**  
¿Existe un único “π taxicab”? ¿O la constante es siempre dependiente de la propiedad geométrica que se intenta preservar? Documenta ejemplos claros de ambas situaciones.

### 3. Distancia entre dos puntos

Compara  
\( d_2 = \sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2} \)  
con  
\( d_1 = |x_2 - x_1| + |y_2 - y_1| \).

Analiza problemas clásicos de:

- geometría analítica
- optimización
- triangulación
- diagramas de Voronoi
- rutas mínimas

y determina cuántos de ellos se vuelven lineales por tramos (sin raíces) al pasar a L₁. Evalúa también si la base 12 aporta alguna simplificación adicional.

### 4. Elipse

Reformula la definición clásica de elipse  
\( \sqrt{(x - c)^2 + y^2} + \sqrt{(x + c)^2 + y^2} = 2a \)  
usando la distancia L₁:  
\( |x - c| + |y| + |x + c| + |y| = 2a \),  
que se simplifica a  
\( |x - c| + |x + c| + 2|y| = 2a \).

**Pregunta:**  
¿Qué problemas de geometría algebraica que parecen cuadráticos se convierten en problemas lineales por regiones bajo L₁? Genera ejemplos concretos.

### 5. Trigonometría taxicab

Define funciones análogas a \( \sin\theta \) y \( \cos\theta \) sobre el “círculo” L₁: \( |x| + |y| = 1 \).

Investiga si existen identidades equivalentes a \( \sin^2\theta + \cos^2\theta = 1 \) (adaptadas a L₁) y otras identidades trigonométricas clásicas.

Determina si aparece matemática genuinamente nueva o solo versiones simplificadas de resultados conocidos.

### 6. Números complejos bajo L₁

Define el módulo L₁ como \( |z|\_1 = |x| + |y| \) en lugar de \( \sqrt{x^2 + y^2} \).

Estudia la forma polar \( z = r e^{i\theta} \) bajo esta norma.

**Pregunta clave:**  
¿Qué partes de la teoría de la función exponencial compleja dependen esencialmente de la geometría euclidiana y cuáles sobreviven (o se simplifican) en L₁?

### 7. Integrales que contienen π

Toma la integral clásica  
\( \displaystyle\int\_{-1}^{1} \sqrt{1 - x^2}\, dx = \dfrac{\pi}{2} \)  
(media circunferencia).

Sustituye el dominio circular por el dominio L₁ \( |x| + |y| \le 1 \) y estudia  
\( \displaystyle\int\_{-1}^{1} (1 - |x|)\, dx \),  
que vale 1.

Analiza el fenómeno conceptual: no se ha “demostrado que \( \pi = 2 \)”; se ha cambiado la geometría que generaba el π. Extiende el análisis a otras integrales clásicas que involucran π.

### 8. Transformadas de Fourier

Investiga qué ocurre con las transformadas de Fourier cuando la geometría subyacente utiliza la norma L₁ en lugar de L₂.

Analiza el efecto sobre:

- bolas unitarias
- superficies
- frecuencias
- convoluciones
- kernels

en una y varias dimensiones. Señala si aparecen estructuras más simples o discretas.

### 9. Ecuaciones diferenciales

Compara operadores basados en \( \|\nabla f\|\_2 \) frente a \( \|\nabla f\|\_1 \).

Estudia el comportamiento de problemas clásicos de:

- difusión
- optimización
- superficies mínimas
- regularización

Determina si en L₁ aparecen soluciones más sencillas o estructuras discretas.

### 10. Problema abierto (Goldbach) + base 12

No intentes “cambiar π por 4” (π no es central en Goldbach).

Investiga si existe una representación geométrica L₁ combinada con base 12 que revele algún patrón en \( n = p + q \) (conjetura de Goldbach).

Explora distribuciones de primos usando coordenadas o estructuras discretas compatibles con base 12.

**Objetivo realista:** no resolver la conjetura, sino buscar una nueva formulación geométrico-dozenal que pueda resultar iluminadora.

---

## Hipótesis H₁ (versión operativa)

**Hipótesis H₁ (falsable):**  
En determinadas familias de problemas geométricos y analíticos cuya estructura está dominada por particiones en 2, 3, 4, 6 y 12, la combinación de una formulación L₁ + representación dozenal produce expresiones con menor complejidad simbólica que su equivalente euclidiano-decimal.

**Tarea:**

1. Identifica al menos 8–10 familias de problemas donde las particiones 2, 3, 4, 6, 12 sean naturales.
2. Reformúlalas en L₁.
3. Exprésalas en base 12.
4. Calcula el índice de complejidad \( K(F) \) para ambas versiones.
5. Reporta en qué casos H₁ se confirma, en qué casos se refuta y por qué.

---

## Pregunta verdaderamente poderosa (versión final)

En lugar de preguntar “¿podemos hacer que π sea 4?”, investiga:

> ¿Qué propiedades matemáticas actualmente expresadas mediante π son realmente propiedades de la geometría euclidiana (L₂), y cuáles sobreviven cuando sustituimos L₂ por L₁? ¿Puede la representación dozenal hacer más evidente la estructura algebraica resultante?

Busca casos en los que fórmulas que parecen analíticamente complejas en \( (L_2 + \text{base 10}) \) se convierten en expresiones sorprendentemente simples en \( (L_1 + \text{base 12}) \). Documenta tanto los éxitos como los fracasos.
