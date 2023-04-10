# Proyecto: Bitcoin - 1er Cuatrimestre 2023

**Fechas de entrega:**

* Entrega intermedia: lunes 22 de mayo
* Presentación de la cursada: lunes 26 de junio

## Introduccion a Blockchain

Blockchain es una tecnología descentralizada que se utiliza para registrar transacciones de manera segura e inmutable. La información se organiza en bloques conectados en una cadena y cada participante en la red tiene una copia idéntica del registro completo. La seguridad proviene de la criptografía y la naturaleza descentralizada del sistema. Se utiliza comúnmente para la gestión de criptomonedas, pero también tiene aplicaciones en otras áreas como la gestión de cadenas de suministro y la votación electrónica.

## Introduccion a Bitcoin

Bitcoin es una criptomoneda digital descentralizada basada en una red de nodos que ejecutan software [Bitcoin Core](https://github.com/bitcoin/bitcoin). La red está asegurada mediante el uso de criptografía y un sistema de incentivos llamado "minería de Bitcoin", donde los participantes compiten para resolver un problema matemático complejo y validar transacciones en la red. Las transacciones se registran en una base de datos blockchain, que contiene un registro inmutable de todas las transacciones de Bitcoin desde su creación. El [White Paper](https://bitcoin.org/bitcoin.pdf) original fue escrito en 2009 por una persona o grupo anónimo de personas bajo el seudónimo de Satoshi Nakamoto.

#### Importante: durante el desarrollo del proyecto utilizaremos la red de testing de Bitcoin: 'testnet' la cual no involucra dinero real y permite obtener test coins para realizar pruebas.

## Objetivo del Proyecto

El objetivo principal del presente proyecto de desarrollo consiste en la implementacion de un **Nodo Bitcoin** con funcionalidades acotadas que se detallan en el presente enunciado. Siguiendo las [guias de desarrollo](https://developer.bitcoin.org/devguide/index.html) y [especificaciones](https://developer.bitcoin.org/reference/index.html) de Bitcoin.

El objetivo secundario del proyecto consiste en el desarrollo de un proyecto real de software de mediana envergadura aplicando buenas prácticas de desarrollo de software, incluyendo entregas y revisiones usando un sistema de control de versiones.

Se espera que se haga un uso lo más idiomático posible del lenguaje de programación Rust, siguiendo los estándares que éste promueve.

## Requerimientos funcionales

Los siguientes son los requerimientos funcionales para el desarrollo del Trabajo.

El nodo de Bitcoin deberá ser capaz de descargar y almacenar la cadena completa de headers desde el inicio de la blockchain y los bloques completos a partir de una fecha determinada, que correspondera al inicio del presente proyecto.

A su vez debera mantener actualizada la informacion de nuevos bloques (incluyendo sus headers y las transacciones incluidas) y nuevas transacciones (no confirmadas) que se van trasmitiendo por la red.

Tambien debera mantener la lista de [UTXO](https://developer.bitcoin.org/devguide/transactions.html) (unspent transactions) a partir de la fecha mencionada y permitir al usuario realizar transacciones utilizando dichas UTXO.

### Protocolo

El [protocolo](https://developer.bitcoin.org/devguide/p2p_network.html) de red de Bitcoin permite mantener una red peer to peer (P2P) entre nodos, con el objetivo de intercambiar informacion sobre bloques y transacciones.
![](https://i.imgur.com/QHWYHtV.png)

### Conexion a la red

Los pasos para conectarse a la red estan explicados en la guia de desarrollo [P2P network](https://developer.bitcoin.org/devguide/p2p_network.html). 
Durante todo el desarrollo del proyecto utilizaremos la red de pruebas de Bitcoin: 'testnet'.
Se debera tener en cuenta las siguientes consideraciones:

1. **Peer Discovery**: utilizar alguna de las siguientes direcciones de DNS: 
    * seed.testnet.bitcoin.sprovoost.nl  -recomendado-
    * testnet-seed.bitcoin.jonasschnelli.ch
    * seed.tbtc.petertodd.org
    * testnet-seed.bluematt.me
2. **Handshake**: se debera conectar con la mayor cantidad de nodos posible, obtenidos de la resolucion de los DNS arriba mencionados. Se decidira que valor de [Protocol Version](https://developer.bitcoin.org/reference/p2p_networking.html#protocol-versions) utilizar, el cual estara definido por configuracion.
3. **Initial Block Download**: se sugiere implementar la descarga utilizando la modalidad [Headers first](https://developer.bitcoin.org/devguide/p2p_network.html#headers-first) teniendo en cuenta que se debe descargar la totalidad de los headers pero solo la informacion de bloques a partir de la fecha de inicio del proyecto, la cual debera estar definida por configuracion. Se recomienda el uso de multiples threads para paralelizar la descarga de Bloques.
4. **Block Broadcasting**: se puede optar por recibir 'Direct Headers Announcement' para lo cual se debera especificar dicha modalidad durante el Handshake mediante el mensaje [sendheaders](https://developer.bitcoin.org/reference/p2p_networking.html#sendheaders)
6. **Alerts**: estan discontinuadas y en caso de recibir una de ellas podemos ignorar el mensaje.

#### Nota: los parametros anteriormente definidos no pueden ser hardcodeados y deberan leerse del archivo de configuracion y/o variables de ambiente.

### Comportamiento del Nodo

1. **Block Validations**: ante la llegada de nuevos bloques se debera validar la [Proof Of Work](https://developer.bitcoin.org/devguide/block_chain.html#proof-of-work) de cada bloque recibido y la Proof of Inclusion de las transacciones del bloque, generando el [Merkle Tree](https://developer.bitcoin.org/reference/block_chain.html#merkle-trees) con las transacciones del bloque y comparando el Merkle Root generado con el especificado en el header del Bloque.
2. **UTXO set**: en todo momento se debera mantener la lista de 'unspent transactions' de manera de poder utilizar la misma para realizar transacciones (ver siguiente punto).
3. El nodo, dada una transaccion y un bloque, debe poder devolver una ***merkle proof of inclusion***, para que el usuario pueda verificar la existencia de la transaccion en el bloque.

### Funciones de Wallet

#### Nota: como prerequisito para poder probar estas operaciones se debera crear una cuenta en Testnet y obtener test coins de prueba. Link para [crear una cuenta](https://www.bitaddress.org/bitaddress.org-v3.3.0-SHA256-dec17c07685e1870960903d8f58090475b25af946fe95a734f88408cef4aa194.html?testnet=true) y [obtener coins](https://tbtc.bitaps.com/)

A continuacion se obsevan las funcionalidades provistas por una Wallet y cuales de las mismas estaran dentro del alcance del proyecto.

![](https://i.imgur.com/S9NvpZ0.jpg)
*La zona verde corresponde a las funcionalidades incluidas en el alcance del proyecto.*

Detalle de las funcionalidades:

1. El usuario podra ingresar una o mas cuentas que controla, especificando la clave publica y privada de cada una.
2. Para cada cuenta se debera visualizar el balance de la misma, es decir la suma de todas sus UTXO disponibles al momento.
3. Cada vez que se recibe una Transaccion en la red que involucra una de las cuentas del usuario se debera informar al usuario, aclarando que la misma aun no se encuentra incluida en un Bloque.
4. Cada vez que se recibe un nuevo Bloque generado se debera verificar si alguna de las transacciones del punto anterior se encuentra en dicho bloque y se informara al usuario.
5. En todo momento el usuario podra realizar una [Transaccion](https://developer.bitcoin.org/devguide/transactions.html), ingresando la informacion necesaria para la misma. Como minimo se debera soportar [P2PKH](https://developer.bitcoin.org/devguide/transactions.html#p2pkh-script-validation). La transaccion generada se debera comunicar al resto de los nodos de la red.
6. Adicionalmente, el usuario podra pedir una prueba de inclusión de una transacción en un bloque, y verificarla localmente

#### Importante: luego de realizada una transaccion podremos verificar que la misma se genero correctamente mediante el uso de un blockchain explorer de testnet, por ejemplo [bitaps](https://tbtc.bitaps.com/)

### Archivo de Configuracion
El nodo deber poder ser configurado mediante un archivo de configuración, nombrado `nodo.conf` y cuya ubicación se pasara por argumento de línea de comando: `$ ./nodo-rustico /path/to/nodo.conf`.
Todos los valores de configuracion mencionados en este enunciado y cualquier otro parametro necesario para la ejecucion del programa debera estar definido en este archivo. 
No se permite definir valores hardcodeados en el codigo fuente, ya sean direcciones IP, puertos o cualquier otra informacion necesaria.


### Archivo de Log
El nodo debe mantener un registro de los mensajes recibidos en un archivo de log. La ubicación del archivo de log estará especificada en el archivo de configuración.

Como requerimiento particular del Proyecto, NO se considerará válido que el servidor mantenga un *file handle* global, aunque esté protegido por un lock, y que se escriba directamente al file handle. La arquitectura deberá contemplar otra solución.

## Interfaz gráfica
Se debe implementar una interfaz gráfica utilizando la biblioteca [GTK](https://gtk.org/), mediante el crate [gtk-rs](https://gtk-rs.org/). Se recomienda utilizar [Glade](https://glade.gnome.org/) y GTK3.

La interfaz deberá permitir el uso completo de las funcionalidades solicitadas y su apariencia deberá ser similar a la de [bitcoin-qt](https://bitcoin.org/en/bitcoin-core/features/user-interface).

### Vista general

![](https://i.imgur.com/4HkYKud.png)

### Crear transacciones

![](https://i.imgur.com/F6xo4SC.png)

### Ver transacciones

![](https://i.imgur.com/xd9D0kY.png)


## Requerimientos no funcionales

Los siguientes son los requerimientos no funcionales para la resolución del proyecto:

* El proyecto deberá ser desarrollado en lenguaje **Rust**, usando las herramientas de la biblioteca estándar.
* Se deben implementar **tests unitarios y de integración** de las funcionalidades que se consideren más importantes.
* No se permite utilizar **crates** externos más allá de los mencionados en dicha sección.
* El código fuente debe compilarse en la versión stable del compilador y no se permite utilizar bloques unsafe.
* El código deberá funcionar en ambiente Unix / Linux.
* La compilación no debe arrojar **warnings** del compilador, ni del linter **clippy**.
* Las funciones y los tipos de datos (**struct**) deben estar documentadas siguiendo el estándar de **cargo doc**.
* El código debe formatearse utilizando **cargo fmt**.
* Las funciones no deben tener una extensión mayor a 30 líneas. Si se requiriera una extensión mayor, se deberá particionarla en varias funciones.
* Cada tipo de dato implementado debe ser colocado en una unidad de compilación (archivo fuente) independiente.

### Crates externos permitidos

Se permite el uso de los siguientes crates sólo para los usos mencionados (siempre y cuando se los considere necesario):

* [rand](https://crates.io/crates/rand): para la generación de valores aleatorios.
* [chrono](https://crates.io/crates/chrono): para la obtención del timestamp actual.
* [bitcoin_hashes](https://crates.io/crates/bitcoin_hashes): para utilizar funciones de hash como SHA256, SHA256d y RIPEMD160.
* [k256](https://crates.io/crates/k256): para firma digital con ECDSA (Elliptic Curve Digital Signature Algorithm).
* [secp256k1](https://crates.io/crates/secp256k1): variante para ECDSA (Elliptic Curve Digital Signature Algorithm).
* [bs58](https://crates.io/crates/bs58): para serializacion en base58.

## Criterios de Aceptación y Corrección del Proyecto

Para el desarrollo del proyecto, se deberá observar los siguientes lineamientos generales:

1. **Testing:** Se deberá implementar testing unitario automatizado, utilizando las herramientas de Rust de los métodos y funciones relevantes implementados. Se deberán implementar tests de integración automatizados.
2. **Manejo de Errores:** Deberá hacerse un buen uso y administración de los casos de error, utilizando para ello, las estructuras y herramientas del lenguaje, escribiendo en forma lo más idiomática posible su tratamiento.
3. **Control de versiones:** Se deberá utilizar la herramienta git, siguiendo las recomendaciones de la cátedra. En particular, se deberá utilizar la metodología GitHub Flow para el trabajo con ramas (branches) y la entrega continua del software.
4. **Trabajo en equipo:** Se deberá adecuar, organizar y coordinar el trabajo al equipo, realizando tareas como revisión de código cruzada entre pares de una funcionalidad en un pull request de git.
5. **Merge de Branchs:** Para poder hacer el merge de un branch de una funcionalidad, todos los tests pasan de forma satisfactoria
6. **Informe final:** El trabajo debe acompañarse por un informe que debe incluir diagramas de secuencia de las operaciones más relevantes, diagrama de componentes y módulos de la arquitectura general del diseño desarrollado, todos acompañados de la explicación respectiva.

## Evaluaciones

El desarrollo del proyecto tendrá un seguimiento directo semanal por parte del docente a cargo del grupo.

Se deberá desarrollar y presentar los avances y progreso del trabajo semana a semana (simulando un sprint de trabajo). Cada semana, cada docente realizará una valoración del estado del trabajo del grupo.

El progreso de cada semana deberá ser acorde a lo que se convenga con el docente para cada sprint. Si el mismo NO cumple con la cantidad de trabajo requerido, el grupo podrá estar desaprobado de forma prematura de la materia, a consideración del docente.

Hacia la mitad del desarrollo del proyecto se deberá entregar una versión preliminar que deberá cumplir con un conjunto de requisitos a definir por la cátedra en las próximas semanas. Dichos requisitos serán de cumplimiento mínimo y obligatorio, aquellos grupos que lo deseen podrán implementar requisitos adicionales.

**Nota importante:** Se deja constancia que las funcionalidades requeridas por este enunciado son un marco de cumplimiento mínimo y que pueden haber agregados o modificaciones durante el transcurso del desarrollo por parte del docente a cargo, que formarán parte de los requerimientos a cumplir. Cabe mencionar que estos desvíos de los requerimientos iniciales se presentan en situaciones reales de trabajo con clientes.

## Finalización del Proyecto

El desarrollo del proyecto finaliza el último día de clases del cuatrimestre. En esa fecha, cada grupo deberá realizar una presentación final y se hará una evaluación global del trabajo.

En dicha presentación se deberá detallar la arquitectura del proyecto, aprendizajes del mismo, y realizar una muestra funcional del desarrollo, esto es una "demo" como si fuera para el usuario final.

El trabajo debe acompañarse por un informe que debe constar de los puntos detallados precedentemente, diagramas de secuencia de las operaciones más relevantes, diagrama de componentes y módulos de la arquitectura general del diseño desarrollado, todos acompañados de la explicación respectiva.
