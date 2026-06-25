# Agregado Final 2C 2024: Containerization

## Introducción
*Containerization* es un enfoque de virtualización que permite ejecutar aplicaciones en entornos aislados llamados contenedores. A diferencia de las máquinas virtuales tradicionales, que requieren un sistema operativo completo para cada instancia, los contenedores comparten el mismo núcleo del sistema operativo, lo que los hace más ligeros y eficientes en términos de recursos.

### Docker
Docker es una plataforma de software que permite crear, implementar y ejecutar aplicaciones en contenedores, empaquetando el código de la aplicación junto con todas sus dependencias, bibliotecas y configuraciones necesarias para que funcione correctamente.

## Objetivo

El objetivo principal es facilitar el posterior despliegue por parte de los clientes, haciendo foco en una transición fluida de la administración de la aplicación. Se opta por un formato estandar como entregable ya que asegura la existencia de multiples herramientas de despliegue que trabajan con este formato, permitiendo flexibilidad y portabilidad. 
Se busca tambien brindar visibilidad del estado de la aplicacion con la incorporacion del modulo de loggeo, que permitira a los clientes encontrar errores y arreglarlos en el futuro. 


## Requerimientos funcionales

### Docker
Se deberán generar todos los artefactos necesarios para el *deployment*  del proyecto utilizando *Docker*. Incluyendo la generación de imágenes, containers, recursos  y configuraciones necesarias para la ejecución del proyecto.
Como mínimo se deberán incluir en la entrega los siguientes artefactos:
1.**Archivo de configuracin**: Separar del código fuente todas las variables configurables, incluyendo secretos y parámetros de entorno en un archivo de configuracion.
2. **Dockerfile**: para la generación de la imágen de cada uno de los componentes.
3. **Docker-compose**: para la definición de todos los contenedores de la red, incluyendo sus configuraciones y recursos necesarios.
4. Todos los comandos necesarios para construir, iniciar, detener y destruir el proyecto.
5. Agregar un archivo README en la carpeta raíz del repositorio indicando todas las instrucciones para construir, iniciar, detener y destruir el ambiente, incluyendo todos las variable necesarias para funcionar.


### Logger
De manera de poder visualizar facilmente el funcionamiento del sistema en *Docker* se pide implementar un *Logger* que registre los eventos relevantes. Dicho logger deberá poder escribir en un archivo y en la salida estándar (**stdout**) al mismo tiempo, de manera de poder visualizar el log utilizando el comando: **docker logs**.


## Requerimientos no funcionales
Los siguientes son los requerimientos no funcionales para la resolución del proyecto:

* El proyecto deberá ser desarrollado en lenguaje Rust, utilizando las herramientas de la biblioteca estándar.
* Se deben implementar pruebas unitarias y de integración de las funcionalidades que se consideren más importantes.
* El código fuente debe compilar en la versión estable del compilador y no se permite el uso de bloques inseguros (unsafe).
* El código deberá funcionar en ambiente Unix / Linux.
* La compilación no debe generar advertencias del compilador ni del linter clippy.
* El programa no puede contener ningún [Busy-Wait](https://en.wikipedia.org/wiki/Busy_waiting), ni puede consumir recursos de CPU y/o memoria indiscriminadamente. Se debe hacer un uso adecuado tanto de la memoria como del CPU. 
* Las funciones y los tipos de datos (struct) deben estar documentados siguiendo el estándar de cargo doc.
* El código debe formatearse utilizando cargo fmt.
* Las funciones no deben tener una extensión mayor a 30 líneas. Si se requiere una extensión mayor, se debe particionar en varias funciones.
* Cada tipo de dato implementado debe ser colocado en una unidad de compilación (archivo fuente) independiente.


## Presentación
Se deberá realizar una **presentación** explicando la implementación de esta nueva consigna, incluyendo las decisiones de diseño y una demostración de la funcionalidad en vivo. 
Dentro de los detalles de implementación se deberá explicar la solución adoptada desde el punto de vista de **multi-threading**, con diagramas que faciliten la explicación.
Durante la demostración en vivo, se debe poder observar tanto los requerimientos funcionales solicitados en el presente enunciado, como así también los requerimientos no funcionales.
De la misma manera se deberá demostrar como se construyen todos los contenedores utilizando *Docker* y explicando los correspondientes comandos de *Docker* empleados. 


## Informe final
Solo se podrán presentar a la fecha de final teniendo completo el informe final completo del proyecto desarrollado durante el cuatrimestre. La seccion de solucion propuesta y plan de actividades debe reflejar el trabajo realizado para el examen final. 


## Fechas de final
El grupo deberá presentarse en una de las fechas de examen final, tal como se publica en el calendario respectivo.
Las fechas de final para el cuatrimestre actual son:

* Lunes 29/6
* Lunes 13/7
* Lunes 27/7
* Lunes 3/8
* Lunes 10/8