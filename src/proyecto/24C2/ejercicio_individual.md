# SQL Rustico

## Introducción

[Structured Query Language](https://en.wikibooks.org/wiki/Structured_Query_Language), mejor conocido como SQL, es un lenguaje declarativo específico de dominio utilizado para gestionar y manipular bases de datos relacionales. Fue desarrollado en los años 70 y se ha convertido en el estándar para interactuar con bases de datos relacionales, como [MySQL](https://www.mysql.com), [PostgreSQL](https://www.postgresql.org), [Oracle](https://www.oracle.com/database/), y [Microsoft SQL Server](https://www.microsoft.com/es-ar/sql-server).

Se suele dividir a SQL en *sublenguajes* que encapsulan diferentes operaciones que se pueden realizar con este, siendo estas: Data Query Language (DQL), Data Definition Language (DDL), Data Control Language (DCL), y Data Manipulation Language (DML).

## Ejercicio

El ejercicio propuesto es implementar una versión *rústica* de SQL.

Las tablas serán modeladas como archivos `CSV` (comma separated values), donde la primera fila representará los nombres de las columnas, y las siguientes representaran registros. Por ejemplo:

```
Nombre,Apellido,Edad,Correo electronico,Profesion
Juan,Perez,32,jperez@gmail.com,medico
Maria,Gomez,28,mgomez@gmail.com,abogado
Carlos,Sánchez,45,csanchez@gmail.com,ingeniero
Ana,Ruiz,36,aruiz@gmail.com,arquitecta
Luis,Martínez,29,lmartinez@gmail.com,profesor
Laura,Domínguez,41,ldominguez@gmail.com,enfermera
Pedro,Fernández,33,pfernandez@gmail.com,diseñador
Lucía,Ramos,26,lramos@gmail.com,psicóloga
Diego,Navarro,39,dnavarro@gmail.com,empresario
Paula,Hernández,31,phernandez@gmail.com,publicista
```

Para nuestra version *rústica*, solo soportaremos datos de tipo Integer y String. 

### Operaciones a implementar

El programa tiene que ser capaz de responder a consultas que hagan uso de las operaciones:

- [INSERT](https://en.wikibooks.org/wiki/Structured_Query_Language/INSERT_1)
- [UPDATE](https://en.wikibooks.org/wiki/Structured_Query_Language/UPDATE_1)
- [DELETE](https://en.wikibooks.org/wiki/Structured_Query_Language/DELETE_1)
- [SELECT](https://en.wikibooks.org/wiki/Structured_Query_Language/SELECT:_Fundamentals)

Para probar y experimentar como funcionan estos comandos, se puede utilizar la siguiente herramienta: [SQL Sandbox](https://sandboxsql.com/)

### Select

Para las consultas de tipo SELECT, **solo se debera implementar soporte** para:
- [Restricciones](https://en.wikibooks.org/wiki/Structured_Query_Language/SELECT:_Fundamentals#Restriction_(specify_resulting_rows)) (WHERE).
    - [Operadores de comparación](https://en.wikibooks.org/wiki/Structured_Query_Language/SELECT:_Fundamentals#Comparisons) (sin LENGTH).
    - [Lógica booleana](https://en.wikibooks.org/wiki/Structured_Query_Language/SELECT:_Fundamentals#Boolean_Logic) (sin abreviaciones).
- [Ordenamientos](https://en.wikibooks.org/wiki/Structured_Query_Language/SELECT:_Fundamentals#Sorting) (ORDER BY).

### Formato de Input

Se llamará a nuestro programa pasándole por *primer parámetro* la ruta a una carpeta, en la que dentro se encontrarán archivos  que representarán nuestras tablas. 

Como *segundo parámetro*, se pasará la consulta SQL en sí.
```bash
cargo run -- ruta/a/tablas "<consulta>"
```

### Formato de Output

**SOLO para el caso de las lecturas (SELECT)**, el resultado de la consulta debera imprimirse por salida estandar (STDOUT) en formato `CSV`. En los demas casos no deberá imprimirse nada.

> Nota: Para verificar si la salida respeta el formato CSV se puede utilizar el operador de [redireccion de Bash (>)](https://www.gnu.org/software/bash/manual/html_node/Redirections.html#Redirecting-Output), y redirigir el STDOUT a un nuevo archivo para luego poder verificarlo con cualquier herramienta para tal fin:
>
> ```bash
> cargo run -- ruta/a/tablas "SELECT * FROM table" > output.csv
> ```

Independientemente del tipo de consulta, en caso de ocurrir algun error durante la ejecución, se deberá imprimir tambien por STDOUT con el siguiente formato:
```
[ErrorType]: [Description]
```

Los tipos de errores seran:
- INVALID_TABLE: relacionado a problemas con el procesamiento de las tablas.
- INVALID_COLUMN: relacionado a problemas con el procesamiento de columnas.
- INVALID_SYNTAX: relacionado a problemas con el procesamiento de consultas.
- ERROR: tipo genérico para otros posibles errores detectados.



## Ejemplos

Si tenemos dos tablas:

`clientes.csv`

```csvpreview
id,nombre,apellido,email
1,Juan,Pérez,juan.perez@email.com
2,Ana,López,ana.lopez@email.com
3,Carlos,Gómez,carlos.gomez@email.com
4,María,Rodríguez,maria.rodriguez@email.com
5,José,López,jose.lopez@email.com
6,Laura,Fernández,laura.fernandez@email.com
```

`ordenes.csv`

```csvpreview
id,id_cliente,producto,cantidad
101,1,Laptop,1
103,1,Monitor,1
102,2,Teléfono,2
104,3,Teclado,1
105,4,Mouse,2
106,5,Impresora,1
107,6,Altavoces,1
108,4,Auriculares,1
109,5,Laptop,1
110,6,Teléfono,2
```

### Ejemplo 1

Obtener aquellas ordenes con una cantidad mayor a uno.

```sql
SELECT id, producto, id_cliente
FROM ordenes 
WHERE cantidad > 1;
```

Resultado en STDOUT:

```
id,producto,id_cliente
102,Teléfono,2
105,Mouse,4
110,Teléfono,6
```

### Ejemplo 2

Obtener aquellos clientes cuyo apellido es López, y ordenar por email de manera descendente.

```sql
SELECT id, nombre, email
FROM clientes 
WHERE apellido = 'López'
ORDER BY email DESC;
```

Resultado en STDOUT:

```
id,nombre,email
5,José,jose.lopez@email.com
2,Ana,ana.lopez@email.com
```

### Ejemplo 3

Actualizar el email del cliente cuyo *id* es 4.

```sql
UPDATE clientes
SET email = 'mrodriguez@hotmail.com'
WHERE id = 4;
```

Contenido actualizado del archivo *clientes.csv*:

```
id,nombre,apellido,email
1,Juan,Pérez,juan.perez@email.com
2,Ana,López,ana.lopez@email.com
3,Carlos,Gómez,carlos.gomez@email.com
4,María,Rodríguez,mrodriguez@hotmail.com
5,José,López,jose.lopez@email.com
6,Laura,Fernández,laura.fernandez@email.com
```

### Ejemplo 4

Crear una nueva orden con id 111, hecha por el cliente con id 6, de tres laptops.

```sql
INSERT INTO ordenes (id, id_cliente, producto, cantidad)
VALUES (111, 6, 'Laptop', 3);
```

Contenido actualizado del archivo *ordenes.csv*:

```
id,id_cliente,producto,cantidad
101,1,Laptop,1
103,1,Monitor,1
102,2,Teléfono,2
104,3,Teclado,1
105,4,Mouse,2
106,5,Impresora,1
107,6,Altavoces,1
108,4,Auriculares,1
109,5,Laptop,1
110,6,Teléfono,2
111,6,Laptop,3
```

## Algunas consideraciones
- No está permitido cargar las tablas completas en memoria. La solucion deberá contemplar alguna estrategia para evitar esto.
- Se asume que el [AUTOCOMMIT](https://en.wikipedia.org/wiki/Autocommit) se encuentra activado. Esto quiere decir que cada consulta ingresada se ejecuta inmediatamente en las tablas seleccionadas.
- No es obligatorio implementar aliases para las columnas o tablas.
- Préstese especial atención al orden de los elementos en las consultas de tipo SELECT, dado que normalmente los DBSM respetan este orden:

```SQL
SELECT   <things_to_be_displayed>  -- the so called 'Projection' - mostly a list of columnnames
FROM     <tablename>
WHERE    <where_clause>            -- the so called 'Restriction' or 'search condition'
ORDER BY <order_by_clause>
```

- No se deberá implementar soporte para subqueries.


## Restricciones
* Escribir el programa sin utilizar .unwrap() o .expect(). Todo caso deberá manejarse ideomaticamente con las estructuras y funciones brindadas por el lenguaje.
* No se permite que el programa lance un [panic!()](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html).
* No se permite utilizar la función [exit()](https://doc.rust-lang.org/std/process/fn.exit.html). Se deberá salir del programa finalizando el scope de la función main.
* No se permite utilizar el módulo [mem](https://doc.rust-lang.org/std/mem/) para la manipulación de memoria.
* Para realizar un uso adecuado de memoria y respetar las reglas de ownership se deberá evitar el uso de [.clone()](https://doc.rust-lang.org/std/clone/trait.Clone.html) y [.copy()](https://doc.rust-lang.org/std/marker/trait.Copy.html).
* Todo el programa puede ser resuelto con lo aprendido en clase hasta la presentación de este ejercicio. No se espera que se utilicen estructuras relacionadas a concurrencia o redes para resolución de este ejercicio.

## Requerimientos no funcionales
Los siguientes son los requerimientos no funcionales para la resolución del proyecto:

* El proyecto deberá ser desarrollado en la última version estable de Rust, usando las herramientas de la biblioteca estándar.
* El proyecto deberá realizarse de manera individual. Cualquier tipo de copia significa la expulsión automática de la materia.
* No está permitido el uso de código generado por ninguna IA, ni copiar código de soluciones existentes en internet. 
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


## Fechas de entrega

Primer entrega: Lunes 9 de Septiembre de 2024 hasta las 18hs.

**No cumplir con la primer entrega imposibilitará la continuidad en la materia** 

Luego de la primer entrega se harán las correcciones correspondientes y se podrá volver a entregar el ejercicio en dos oportunidades más.

La forma de entrega se comunicará por el canal de avisos.