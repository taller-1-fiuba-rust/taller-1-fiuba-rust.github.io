
# Agregado Final 2023C1: Nodo como servidor

## Introducción

Durante el cuatrimestre se desarrolló un proyecto de mediana envergadura en Rust, donde se implementó un **Nodo de Bitcoin** con funcionalidades acotadas. Para este final se propone acoplar a lo realizado durante el cuatrimestre un agregado donde el **Nodo** tambien tenga la funcionalidad de actuar como servidor, recibiendo conexiones entrantes de otros nodos clientes por el puerto que indica el **protocolo**. 


## Requerimientos funcionales

El **Nodo** Bitcoin, además de cumplir con los requerimientos funcionales y no funcionales establecidos en el enunciado del Trabajo Practico, debera actuar como servidor, recibiendo conexiones entrantes de otros nodos y respondiendo a las solicitudes que estos realicen. 
Para ello debera cumplir con las siguientes funcionalidades: 
1. Recibir conexiones entrantes en el puerto definido para la red `testnet`, el cual sera establecido en el archivo de configuracion.
2. Atender a estas conexiones como indica el protocolo, es decir realizando el `Handshake`.
3. Responder a las solicitudes de `Headers` realizadas mediante el mensaje `getheaders` como indica el protocolo.
4. Responder a los mensajes `getdata` segun indica el protocolo. 
5. (opcional) Notificar al nodo cliente sobre nuevos bloques y transacciones recibidas desde otros nodos.
6. Se debe poder visualizar el estado de la descarga en la interfaz grafica.


## Requerimientos no funcionales
Los siguientes son los requerimientos no funcionales para la resolución del proyecto:
* El proyecto deberá ser desarrollado en lenguaje **Rust**, usando las herramientas de la biblioteca estándar.
* Se deben implementar **tests unitarios y de integración** de las funcionalidades que se consideren más importantes.
* El código fuente debe compilarse en la versión stable del compilador y no se permite utilizar bloques unsafe.
* La compilación no debe arrojar **warnings** del compilador, ni del linter **clippy**.
* Las funciones y los tipos de datos (**struct**) deben estar documentadas siguiendo el estándar de **cargo doc**.
* El código debe formatearse utilizando **cargo fmt**.
* El programa deberá funcionar en ambiente Unix / Linux.
* No se permite utilizar **crates** externos.


## Presentación
Se deberá realizar una presentacion explicando la implementacion de este agregado, incluyendo las decisiones de diseño y una demostración de la funcionalidad.
Para cumplir con los requisitos minimos de este agregado se debera verificar el funcionamiento del nodo actuando como servidor. Para ello los alumnos deberan demostrar durante su presentacion la correcta ejecucion del programa y la descarga completa del **Initial Block Download** (IBD).
La prueba se realizara ejecutando dos instancias de la aplicacion: la primera de ellas conectada a la red principal y teniendo previamente descargada **una parte de la blockchain** y la segunda, sin datos descargados, tendra su archivo de configuracion modificado para conectarse a la primera e iniciar la **descarga de la blockchain**.
Deberan contemplar las modificaciones necesarias para que el nodo se pueda conectar tanto a direcciones DNS como a direcciones IP definidas en el archivo de configuracion.

**Nota:** Se permite realizar la demostracion ejecutando las dos instancias en el mismo **Host**, en cuyo caso la segunda instancia se conectara a `localhost` o `127.0.0.1`. 

**Importante:** cada grupo debera hacer uso de las herramientas de que dispone para demostrar en vivo la correcta descarga completa de la bockchain, ya sea comparando archivos de headers y bloques y/o mediante el uso de la interfaz grafica.


## Fechas de final
El grupo deberá presentarse en una de las fechas de examen final, tal como se publica en el calendario respectivo.

* 03/07
* 24/07
* 31/07
* 07/08
* 14/08