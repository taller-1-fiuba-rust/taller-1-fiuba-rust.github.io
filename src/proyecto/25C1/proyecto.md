# Proyecto: RustiDocs - 1C 2025

## Introducción
La Universidad de Buenos Aires desea implementar un sistema de trabajo colaborativo en tiempo real que permita confeccionar documentos de texto y planillas de cálculo. El sistema será de uso no solo de toda la comunidad de la UBA sino también por usuarios de otras universidades del mundo.

Dada la carga de usuarios que se espera y su distribución geográfica variada, el sistema se debe construir con un diseño escalable que permita su correcto funcionamiento ante la alta demanda de usuarios y trafico esperado.

Luego de analizar distintas variantes de protocolos se decidió llevar adelante este desarrollo utilizando la versión distribuida de **Redis** (Redis Cluster) tanto para el almacenamiento de información como para el intercambio de mensajes mediante Publisher/Subscriber.

## Objetivo del Proyecto
El objetivo principal del presente Trabajo Practico es desarrollar una versión en el lenguaje Rust del Cluster de Redis, respetando su protocolo cliente-servidor y desarrollando un protocolo interno de intercambio de mensajes entre nodos para implementar las principales características de replicacion y particionado, de manera de poder escalar en forma horizontal ofreciendo altos niveles de disponibilidad y tolerancia a fallos.

El objetivo secundario es desarrollar un proyecto real de software de mediana envergadura aplicando buenas prácticas de desarrollo, incluyendo entregas y revisiones periódicas.

El proyecto comienza con una investigación sobre como funciona la versión distribuida de Redis ([Redis Cluster](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/)) y sus principales conceptos como [Replication](https://redis.io/docs/latest/operate/oss_and_stack/management/replication/) y [Data Sharding](https://redis.io/docs/latest/operate/oss_and_stack/management/scaling/#redis-cluster-data-sharding), entre otros.


## Requerimientos Funcionales
### Redis Cluster
Para dar soporte a la persistencia de datos y el intercambio de mensajes entre clientes se deberá implementar un cluster de nodos de Redis cumpliendo con los siguientes requerimientos mínimos:

1. **Protocolo Cliente/Servidor**:  El programa deberá implementar un subconjunto del protocolo Redis tal como es especificado en la [documentación](https://redis.io/docs/latest/develop/reference/protocol-spec/).
2. **Comandos de Redis**: Se deben implementar los [comandos](https://redis.io/docs/latest/commands/) de Redis necesarios para poder persistir [string](https://redis.io/docs/latest/commands/?group=string), [sets](https://redis.io/docs/latest/commands/?group=set) y [lists](https://redis.io/docs/latest/commands/?group=list). Ademas se deben soportar los comandos relacionados al intercambio de mensajes [Pub/Sub](https://redis.io/docs/latest/commands/?group=pubsub).
3. **Almacenamiento de datos**: Los datos deberan ser almacenados en disco de manera que un nodo pueda ser reiniciado y recupere la informacion almacenada del archivo en disco.
4. **Pub/Sub:** el servidor debe proveer funcionalidad para soportar el paradigma de mensajería Pub/Sub, en el cual clientes que envían mensajes (publicadores) no necesitan conocer la identidad de los clientes que reciben estos mensajes. En cambio, los mensajes publicados se envían a un canal, y los clientes expresan interés en determinados mensajes subscribiéndose a uno o mas canales, y sólo reciben mensajes de estos canales, sin conocer la identidad de los publicadores.
  Para esto, el servidor debe mantener un registro de canales, publicadores y subscriptores. Para mas detalle, consultar la [documentación de Redis](https://redis.io/topics/pubsub).
5. **Funcionamiento del Cluster**: Se deben desarrollar las funcionalidades principales del Cluster de Redis como lo indica la [documentación](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/). En particular se deberá hacer foco en las siguientes caracteristicas:
   *  Componentes principales del cluster ([main components](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/#overview-of-redis-cluster-main-components)), entre ellos la [distribución de claves](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/#key-distribution-model), la comunicación [inter-nodo](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/#the-cluster-bus), su [topología](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/#cluster-topology), entre otras.
   *  Implementación de la arquitectura master/replica y [replica promotion](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/#replica-election-and-promotion).
   *  Tolerancia a fallos: implementar algun mecanismo de gossip para detección de fallos como se explica en la [documentacion](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/#fault-tolerance).
   *  Soporte a Pub/Sub en múltiples nodos como indica la [documentación](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/#publishsubscribe). A modo opcional se podrá implementar sharded Pub/Sub.
6. **Configuración:** el servidor deber poder ser configurado mediante un archivo de configuración, nombrado `redis.conf` y cuya ubicación se pasa por argumento de línea de comando: `$ ./redis-server /path/to/redis.conf`.
7. **Logs:** el servidor debe mantener un registro de las acciones realizadas y los eventos ocurridos en un archivo de log.
  La ubicación del archivo de log estará especificada en el archivo de configuración.
  Como requerimiento particular del Proyecto, NO se considerará válido que el servidor mantenga un <em>file handle</em> global, aunque esté protegido por un lock, y que se escriba directamente al file handle.
  
8. **Seguridad:**
   - Autenticación y autorización.
   - Encriptación de datos en tránsito (in-transit).
   - Opcional: Encriptación de datos en reposo (at-rest).
  
  Para cumplir con los objetivos mínimos el cluster se debe configurar con al menos tres nodos master principales, cada uno de ellos con sus dos réplicas, como se observa en la imagen siguiente:  
![redis cluster](https://hackmd.io/_uploads/B1gUogRcpyx.png)


### Aplicación Cliente
Se debe construir una aplicación con interfaz grafica (UI) para crear y editar documentos de manera colaborativa en tiempo real entre varios usuarios. Dicha aplicacion se conectara al servicio de Redis para obtener el listado de documentos y permitira agregar y editar documentos. 
Las modificaciones en un documento se deben propagar en tiempo real a todos los usuarios que se encuentran editando o visualizando el mismo documento haciendo uso del servicio de Pub/Sub provisto por el servidor de Redis implementado.
Se deben soportar distintos tipos de documento, como objetivo minimo se pide implementar el documento de texto y la planilla de calculos solo con operaciones básicas.
La aplicación debe tener al menos dos vistas principales, una vista con el listado de documentos existentes por tema, y la vista de edicion de un documento (sesión colaborativa).


### Microservicio de Control y Persistencia
Se debe implementar un servidor liviano que se conectara al cluster de Redis como cualquier cliente pero dará servicio a los clientes respondiendo a mensajes enviados por los mismos mediante el protocolo de Pub/Sub.
Por ejemplo cuando un cliente inicia una sesión para editar un documento, el mismo enviara un mensaje por el channel de dicho documento indicando que se esta uniendo a colaborar en ese documento. Este microservicio respondera al cliente enviando un mensaje de respuesta por el mismo channel incluyendo el estado actual del documento y los datos de la sesion, como por ejemplo quienes estan editando ese documento, etc.
Otra de las funcionalidades de este microservicio será la de persistir frecuentemente el contenido del documento en una entrada de Redis, de manera que el mismo quede guardado.


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


## Crates externos permitidos
Se permite el uso de los siguientes crates solo para los usos mencionados (siempre y cuando se los considere necesario):

* rand: para la generación de valores aleatorios.
* chrono: para la obtención del timestamp actual.

**Nota:** para la implementación de la interfaz gráfica se podrá proponer crates que deseen utilizar, los cuales serán evaluados y autorizados por el grupo docente. 


## Material de Consulta
- [Documentación de Redis Cluster](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/)


## Criterios de Aceptación y Corrección del Proyecto
Para el desarrollo del proyecto, se deberá observar los siguientes lineamientos generales:
1. **Testing:** Se deberá implementar testing unitario automatizado, utilizando las herramientas de Rust de los métodos y funciones relevantes implementados. Se deberán implementar tests de integración automatizados.
2. **Manejo de Errores:** Deberá hacerse un buen uso y administración de los casos de error, utilizando para ello, las estructuras y herramientas del lenguaje, escribiendo en forma lo más idiomática posible su tratamiento.
3. **Control de versiones:** Se deberá utilizar la herramienta git, siguiendo las recomendaciones de la cátedra. En particular, se deberá utilizar la metodología GitHub Flow para el trabajo con ramas (branches) y la entrega continua del software.
4. **Trabajo en equipo:** Se deberá adecuar, organizar y coordinar el trabajo al equipo, realizando tareas como revisión de código cruzada entre pares de una funcionalidad en un pull request de git.
5. **Merge de Branchs:** Para poder hacer el merge de un branch de una funcionalidad, todos los tests pasan de forma satisfactoria
6. **Informe final:** El trabajo debe acompañarse por un informe que debe incluir diagramas de secuencia de las operaciones más relevantes, diagrama de componentes y módulos de la arquitectura general del diseño desarrollado, todos acompañados de la explicación respectiva.


## Evaluación
El desarrollo del proyecto tendrá un seguimiento directo semanal por parte del docente a cargo del grupo.

Se deberá desarrollar y presentar los avances y progreso del trabajo semana a semana (simulando un sprint de trabajo). Cada semana, cada docente realizará una valoración del estado del trabajo del grupo.

El progreso de cada semana deberá ser acorde a lo que se convenga con el docente para cada sprint. Si el mismo NO cumple con la cantidad de trabajo requerido, el grupo podrá estar desaprobado de forma prematura de la materia, a consideración del docente.

Hacia la mitad del desarrollo del proyecto se deberá entregar una versión preliminar que deberá cumplir con los requisitos mencionados en el apartado *Entrega intermedia*. Estos requisitos **son de cumplimiento mínimo y obligatorio**, aquellos grupos que lo deseen podrán implementar requisitos adicionales.

**Nota importante:** Se deja constancia que las funcionalidades requeridas por este enunciado son un marco de cumplimiento mínimo y que pueden haber agregados o modificaciones durante el transcurso del desarrollo por parte del docente a cargo, que formarán parte de los requerimientos a cumplir. Cabe mencionar que estos desvíos de los requerimientos iniciales se presentan en situaciones reales de trabajo con clientes.


## Finalización del Proyecto
El desarrollo del proyecto finaliza el último día de clases del cuatrimestre. En esa fecha, cada grupo deberá realizar una presentación final y se hará una evaluación global del trabajo.

En dicha presentación se deberá detallar la arquitectura del proyecto, aprendizajes del mismo, y realizar una muestra funcional del desarrollo, esto es una "demo" como si fuera para el usuario final, donde se pueda observar todas las funcionalidades pedidas por el presente enunciado. 

Durante la demostración en vivo, se debe poder observar tanto los requerimientos funcionales solicitados en el presente enunciado, como así también los requerimientos no funcionales, es decir, se debe poder observar y comprobar todo lo relacionado a temas como Replication, Data Sharding, Tolerancia a fallos, entre otros. 
Por ejemplo: se debe producir un escenario de falla de alguno de los nodos *master* (desconectando el nodo o cerrando la aplicación) y demostrar el mecanismo de [replica promotion](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/#replica-election-and-promotion) que permitirá a la aplicación continuar funcionando correctamente incluso al escribir datos almacenados en dicho nodo.

El trabajo debe acompañarse por un informe que debe constar de los puntos detallados a continuación: explicación general de la investigación realizada y sus conclusiones, reglas de negocio de la solución y decisiones tomadas durante el desarrollo del proyecto, diagramas de secuencia de las operaciones más relevantes, diagrama de componentes y módulos de la arquitectura general del diseño desarrollado, todos acompañados de la explicación respectiva.

### Entrega Intermedia:
Los alumnos deberán realizar una entrega intermedia, la cual deberá incluir los siguientes puntos del apartado de requerimientos funcionales:

#### Redis Cluster
1. **Protocolo Cliente/Servidor**
2. **Comandos de Redis**
3. **Almacenamiento de datos**
4. **Pub/Sub**
5. **Funcionamiento del Cluster**: como requerimiento obligatorio solo se pide presentar el diseño de como se va a implementar este punto en su proyecto, incluyendo diagramas que ilustren la comunicación entre nodos y los distintos escenarios importantes a soportar, por ejemplo: [replica promotion](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/#replica-election-and-promotion), [gossip](https://redis.io/docs/latest/operate/oss_and_stack/reference/cluster-spec/#fault-tolerance), entre otros.

#### Aplicación Cliente
  Para esta entrega intermedia se pide implementar la aplicación con interfaz grafica (UI) básica que se conecte al menos a un nodo de redis y pueda enviar y recibir mensajes. Este comportamiento se debe poder observar en una demo en vivo ya sea demostrando las funcionalidades solicitadas en la sección de requerimientos funcionales o en su defecto mediante el ingreso de comandos manualmente por parte del usuario.
Además se debe presentar el diseño de como se va a ver la interfaz grafica (UI) final, describiendo las dos vistas principales que se pide implementar.

#### Microservicio de Control y Persistencia
  *Este punto es opcional para la entrega intermedia.*

#### Presentación
La entrega se realizará en forma de **Presentación** en la cual los alumnos deberán abarcar los siguientes puntos:

* Explicación general del desarrollo realizado, incluyendo diagramas de componentes y de secuencia de las funcionalidades mas importantes.
* Diseño de la solución a implementar para completar el proyecto, incluyendo diagramas y su explicación.
* Recorrido por los módulos del código fuente escrito, explicando los principales contenidos.
* Demo en vivo de la aplicación gráfica y el cluster de redis, donde se pueda observar su comunicación a través del protocolo cliente/servidor e incluya las funcionalidades requeridas para la entrega intermedia.  

Todos los miembros del grupo deberán participar de la demo y explicar su participación en el proyecto, incluyendo detalles de implementación.

## Fechas de entrega:
Entrega intermedia: Lunes 19 de Mayo de 2025.

Entrega final de la cursada:  Lunes 23 de Junio de 2025.

**Estas entregas serán presenciales en la sede de la Facultad.**
