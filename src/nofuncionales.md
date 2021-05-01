# Requerimientos no funcionales

Los siguientes son los requerimientos no funcionales para la resolución de los ejercicios:

* El proyecto deberá ser desarrollado en lenguaje Rust, usando las herramientas de la biblioteca estándar.
* No se permite utilizar **crates** externos. El único crate autorizado a ser utilizado es [rand](https://crates.io/crates/rand) en caso de que se quiera generar valores aleatorios.
* El código fuente debe compilarse en la versión stable del compilador y no se permite utilizar bloques unsafe.
* El código deberá funcionar en ambiente Unix / Linux.
* El programa deberá ejecutarse en la línea de comandos.
* La compilación no debe arrojar **warnings** del compilador, ni del linter **clippy**.
* Las funciones y los tipos de datos (**struct**) deben estar documentadas siguiendo el estándar de **cargo doc**.
* El código debe formatearse utilizando **cargo fmt**.
* Las funciones no deben tener una extensión mayor a 30 líneas. Si se requiriera una extensión mayor, se deberá particionarla en varias funciones.
* Cada tipo de dato implementado debe ser colocado en una unidad de compilación (archivo fuente) independiente.
