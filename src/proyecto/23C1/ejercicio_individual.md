# Captura inminente

**Fecha límite de entrega:** lunes 3 de Abril de 2023 a las 18 hs

## Introducción

El [ajedrez](https://es.wikipedia.org/wiki/Ajedrez) es un juego de tablero entre dos contrincantes en el que cada uno dispone al inicio de 16 piezas móviles que se colocan sobre un tablero dividido en 64 casillas alternadas en colores blanco y negro, que constituyen las 64 posibles posiciones entre las que se mueven las piezas durante el desarrollo del juego.

![Tablero damas](https://i.imgur.com/fVVOAaC.png)

Inicialmente, cada jugador cuenta con 16 piezas:

* Un **[Rey](https://es.wikipedia.org/wiki/Rey_(ajedrez))**: Puede moverse en cualquier dirección (vertical, horizontal y diagonal), avanzando siempre una casilla (exceptuando el caso del enroque).
* Una **[Dama](https://es.wikipedia.org/wiki/Reina_(ajedrez))**: Tambien puede moverse en cualquier dirección (vertical, horizontal y diagonal), avanzando tantas casillas como se desee.
* Dos **[Alfiles](https://es.wikipedia.org/wiki/Alfil_(ajedrez))**: Solo pueden moverse en direcciones diagonales, avanzando tantas casillas como se desee.
* Dos **[Caballos](https://es.wikipedia.org/wiki/Caballo_(ajedrez))**: Se mueve avanzando dos casillas en vertical y una horizontal, o viceversa (simplificando, se mueve en patron de L), siendo capaz de saltar por encima de otras piezas.
* Dos **[Torres](https://es.wikipedia.org/wiki/Torre_(ajedrez))**: Solo pueden moverse en direcciones verticales y horizontales, avanzando tantas casillas como se desee.
* Ocho **[peones](https://es.wikipedia.org/wiki/Pe%C3%B3n_(ajedrez))**: Puede avanzar una o dos casillas hacia adelante en su primer movimiento, y avanzar solo una en los siguientes. A diferencia de las demas piezas, el peon no puede retroceder, y solo puede capturar piezas que se encuentren a una casilla de distancia en direccion diagonal (hacia adelante). Para este ejercicio, ignoraremos la [captura de peon al paso](https://es.wikipedia.org/wiki/Captura_al_paso).

## Consigna

Dada la posicion de dos piezas (una negra y una blanca), **y solo dos piezas**, determinar si:

* La pieza **negra** puede capturar a la blanca en su siguiente movimiento.
* La pieza blanca puede capturar a la **negra** en su siguiente movimiento.
* Ambas piezas pueden capturar a la otra en su siguiente movimiento.
* Ninguna de las piezas puede capturar a la otra.

> **Aclaración**: es indistinto qué color se movera en el siguiente turno.

Para identificar cada pieza se utilizaran letras: Rey [R], Dama [D], Alfil [A], Caballo [C], Torre [T], Peon [P]. Las blancas se identificaran con letras minúsculas, mientras que las negras con letras MAYÚSCULAS. Cualquier casilla vacia sera representada con guion bajo [_]. Para cada fila del tablero, cada casillero se encuentra separado por un espacio [ ].

### Ejemplo 1

``` text
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ D _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ t _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
```

En este caso, la dama negra esta en posicion para capturar a la torre blanca en su siguiente turno. Por ende, **las negras capturan**.

### Ejemplo 2

``` text
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ P _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ a _ _ _ _ _
_ _ _ _ _ _ _ _
```

En este caso, el alfil blanco esta en posicion para capturar al peon negro en su siguiente turno. Por ende, **las blancas capturan**.

### Ejemplo 3

``` text
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ K _ _ _ _ _
_ _ t _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
```

En este caso, la torre blanca esta en posicion de para capturar al rey negro (curiosamente, acabando en un jaque mate), pero el rey negro tambien podria capturar a la torre blanca. Por ende, **es un empate**.

### Ejemplo 4

``` text
_ _ _ _ _ _ _ _
_ _ _ _ _ P _ _
_ _ _ _ _ _ _ _
_ _ d _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
```

En este caso, ni la dama blanca ni el peon negro estan en posicion de capturar a la otra pieza. Por ende, **todos pierden**.

## Formato de input

El input es un archivo en el filesystem con el formato de entrada del tablero. En la invocación del programa se debe proveer la ruta a ese archivo. **Solo deberá ingresarse este argumento**.

```shell
cargo run -- table.txt
```

## Formato de output

El output sera un caracter impreso por terminal:

* **B**: indica que solo la pieza blanca pueden capturar.
* **N**: indica que solo la pieza negra pueden capturar.
* **E**: indica que ambas piezas pueden capturar.
* **P**: indica que ninguna pieza puede capturar.

En caso de que un error ocurriese, se deberá imprimir un mensaje con el siguiente formato: **ERROR: [descripcion_del_error]**.

## Algunas consideraciones

* La orientación del tablero dispondrá a las piezas blancas en la parte de abajo, y a las piezas negras en la parte de arriba (precisamente, igual que en la imagen presentada al principio de la consigna).
  * Esto quiere decir que los peones blancos se moverán y capturarán hacia "arriba", mientras que los peones negros hacia "abajo".
* Escribir el programa sin clonar (*.clone()*) el input.
* **Solamente** debe imprimirse por terminal el caracter de output, o el error lanzado junto a su descripción.

## Requerimientos no funcionales

**Nota importante:** El proyecto deberá realizarse de manera individual. Si se detecta una copia del trabajo entregado con respecto a la de otro/a estudiante, se podrá decidir hasta la exclusión del curso por considerarse conducta deshonestas y anti-académica.

Los siguientes son los requerimientos no funcionales para la resolución del proyecto:

* El proyecto deberá ser desarrollado en lenguaje Rust y debe compilarse con la versión 1.68 del compilador, usando las herramientas de la biblioteca estándar.
* Se deben implementar tests unitarios y de integración de las funcionalidades que se consideren más importantes.
* No se permite utilizar crates externos.
* El código fuente debe compilarse en la versión estable del compilador y no se permite utilizar bloques unsafe.
* El código deberá funcionar en ambiente Unix / Linux.
* El programa deberá ejecutarse en la línea de comandos.
* La compilación no debe arrojar warnings del compilador, ni del linter clippy.
* Las funciones y los tipos de datos (struct) deben estar documentados siguiendo el estándar de cargo doc.
* El código debe formatearse utilizando cargo fmt.
* Las funciones no deben tener una extensión mayor a 30 líneas. Si se requiriera una extensión mayor, se deberá particionarla en varias funciones.
* Cada tipo de dato implementado debe ser colocado en una unidad de compilación (archivo fuente) independiente.
