# Agregado Final 2C 2024: Containerization

## Introducción
*Containerization* es un enfoque de virtualización que permite ejecutar aplicaciones en entornos aislados llamados contenedores. A diferencia de las máquinas virtuales tradicionales, que requieren un sistema operativo completo para cada instancia, los contenedores comparten el mismo núcleo del sistema operativo, lo que los hace más ligeros y eficientes en términos de recursos.

### Docker
Docker es una plataforma de software que permite crear, implementar y ejecutar aplicaciones en contenedores, empaquetando el código de la aplicación junto con todas sus dependencias, bibliotecas y configuraciones necesarias para que funcione correctamente.

## Objetivo
El objetivo del presente agregado es implementar el servidor distribuido desarrollado durante el cuatrimestre en una serie de contenedores de Docker, utilizando un contenedor para cada nodo del cluster de base de datos.
Además se incorporaran algunos requerimientos para hacer mas flexible la distribución de nodos (instancias) del cluster y el logging de intercambio de mensajes entre nodos.


## Requerimientos funcionales

### Docker
Se deberán generar todos los artefactos necesarios para el *deployment*  del cluster de nodos utilizando *Docker*. Incluyendo la generación de imágenes, containers, recursos (por ej almacenamiento) y configuraciones necesarias para la ejecución del servidor de base de datos distribuido.
Como mínimo se deberán incluir en la entrega los siguientes artefactos:
1. **Dockerfile**: para la generación de la imágen del nodo.
2. **Docker-compose**: para la definición de todos los contenedores de la red, incluyendo sus configuraciones y recursos necesarios.
3. Todos los comandos necesarios para construir, iniciar, detener y destruir el cluster.
4. Agregar un archivo README en la carpeta raíz del repositorio indicando todas las instrucciones para construir, iniciar, detener y destruir el ambiente, incluyendo todos los nodos y sus datos necesarios para funcionar.

### Reconfiguración dinámica del cluster
Se pide realizar todos los cambios necesarios para que el sistema distribuido de DB soporte la incorporación y/o desvinculación de un nodo de la red. Es decir, dado un cluster de N nodos se deberá poder iniciar e incorporar un nuevo nodo a la red, de manera que este mismo reciba un rango de particionamiento para cada una de las tablas de la DB y los nodos existentes le envíen la información correspondiente a los datos del segmento de partición asignado al nuevo nodo. El nuevo nodo deberá entonces recibir la información de tablas, particiones y datos almacenados (de su propio segmento de particiones unicamente)

### Logger
De manera de poder visualizar facilmente el funcionamiento de los nodos en *Docker* se pide implementar un *Logguer* que registre todos los mensajes enviados y recibidos por el nodo. Dicho logguer deberá poder escribir en un archivo y en la salida estándar (**stdout**) al mismo tiempo, de manera de poder visualizar el log utilizando el comando: **docker logs**.


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
Se deberá realizar una **presentación** explicando la implementación de este agregado, incluyendo las decisiones de diseño y una demostración de la funcionalidad en vivo. 
Dentro de los detalles de implementación se deberá explicar la solución adoptada desde el punto de vista de **multi-threading**, con diagramas que faciliten la explicación.
Durante la demostración en vivo, se debe poder observar tanto los requerimientos funcionales solicitados en el presente enunciado, como así también los requerimientos no funcionales, es decir, se debe demostrar como se agrega un nuevo nodo a la red y el mismo recibe toda la información necesaria para funcionar, así como también el funcionamiento de la base de datos antes y después de agregar el nuevo nodo, utilizando las aplicaciones implementadas durante el cuatrimestre (visualización de vuelos y simulación de vuelos en curso) 
De la misma manera se deberá demostrar como se construyen todos los contenedores utilizando *Docker* y como se inicia, se agrega un nodo a la red, se quita un nodo de la red, se detienen los contenedores; utilizando y explicando los correspondientes comandos de *Docker* empleados. 


## Informe final
Solo se podrán presentar a la fecha de final teniendo completo el informe final del proyecto desarrollado durante el cuatrimestre, incluyendo una sección adicional para describir la implementación de este agregado con sus
diagramas de diseño (por ej, diagramas de secuencia) y documentación relacionada.


## Fechas de final
El grupo deberá presentarse en una de las fechas de examen final, tal como se publica en el calendario respectivo.
Las fechas de final para el cuatrimestre actual son:

- lunes 9/12/2024
- lunes 16/12/2024
- lunes 3/2/2025
- lunes 10/2/2025
- lunes 24/2/2025
