# Proyecto: MQTT Rústico - 2do Cuatrimestre 2021

## Introducción

MQTT es un protocolo de mensajería basado en el patrón de comunicación <em>publisher-suscriber</em> y la arquitectura <em>cliente-servidor</em>. Dentro de sus principales características se puede destacar que es un protocolo liviano, simple y sencillo de implementar. Es un protocolo de capa de aplicación binario construido sobre TCP/IP, lo cual lo convierte en una forma de comunicarse sumamente eficiente con un <em>overhead</em> mínimo en la cantidad de paquetes que se envían a través de la red, a diferencia de otros protocolos de capa de aplicación, como por ejemplo HTTP. 

Existen distintos estándares debido a que el protocolo ha ido evolucionando a través del tiempo (1.2, 3.1, 3.1.1, 5, etc). En particular, este proyecto estará centrado en la versión 3.1.1 del protocolo, cuya especificación puede encontrarse en el siguiente link: [MQTT-v3.1.1](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.pdf)

Su principal utilidad se da para implementar aplicaciones de IoT (Internet of Things), y se ha convertido en una forma estándar de comunicar dispositivos a través de la red. La principal causa de su éxito es que, gracias a sus características y funcionalidades ofrecidas, es un protocolo extremadamente simple, eficiente, sencillo de comprender e implementar y permite transferir datos a través de internet de forma confiable incluso por medio de canales de comunicación poco confiables.

Los requerimientos con los cuáles fue implementado el protocolo han sido son los siguientes:

- Que sea sencillo de implementar.
- Que se pueda especificar un nivel de calidad de servicio (<em>Quality of Service, QoS</em>) para la entrega de datos.
- Que sea liviano y eficiente en la red.
- Que sea agnóstico respecto de los datos que quieren comunicar las aplicaciones.
- Que permita a los clientes establecer sesiones para poder reconectarse en caso de sufrir una desconexión.

MQTT está basado en el patrón de comunicación <em>publisher-subscriber</em>. Esencialmente, es un patrón en el que existen clientes que quieren comunicar mensajes a través de <em>tópicos</em>, los cuáles son entregados a otros clientes que se encuentran suscritos a estos <em>tópicos</em>. Los clientes (publicadores y suscriptores) no se conocen entre sí, sino que envían los mensajes pertinentes con su respectivo tópico únicamente al servidor, cuya principal tarea es entregar los mensajes a los clientes que corresponda. Una buena ilustración que ejemplifica dicho patrón es la siguiente:

![](https://mqtt.org/assets/img/mqtt-publish-subscribe.png)

Como es posible observar, uno de los clientes envía un mensaje al servidor con el tópico temperatura, el cual llega al <em>broker</em> y es entregado a otros clientes que se encuentran suscritos a dicho tópico.

En la actualidad, existen distintas implementaciones de servidores MQTT, entre ellas:

- [mosquitto](https://mosquitto.org/), <em>open-source</em> y una de las más utilizadas.
- [HiveMQ](https://www.hivemq.com/), que posee una versión abierta y otra paga.
- [verneMQ](https://vernemq.com/).

A su vez, existen una serie de distintas implementaciones de clientes MQTT para distintos lenguajes.

Por último, se recomienda mirar los videos de la siguiente lista de reproducción, la cual explica a nivel general el funcionamientio del protocolo: [MQTT Essentials](https://www.youtube.com/watch?v=jTeJxQFD8Ak&list=PLRkdoPznE1EMXLW6XoYLGd4uUaB6wB0wd) 


## Objetivo del Proyecto

El objetivo del proyecto consiste en investigar el protocolo MQTT y realizar la implementación de una aplicación cliente y una aplicación servidor en el lenguaje de programación Rust. Además, la aplicación cliente deberá poseer una interfaz gráfica que permita interactuar con las distintas funcionalidades.

El objetivo secundario del proyecto consiste en el desarrollo de un proyecto real de software de mediana envergadura aplicando buenas prácticas de desarrollo de software, incluyendo entregas y revisiones usando un sistema de control de versiones.

Se espera que se haga un uso lo más idiomático posible del lenguaje de programación, siguiendo los estándares que éste promueve.

<!-- 
## Criterios de Aceptación y Corrección del Proyecto
-->

## Evaluaciones

El desarrollo del proyecto tendrá un seguimiento directo semanal por parte del docente a cargo del grupo.

Se deberá desarrollar y presentar los avances y progreso del trabajo semana a semana (simulando un <em>sprint</em> de trabajo). 
Cada semana, o el o los docentes asignados a cada grupos realizarán una valoración del estado del trabajo del grupo.

El progreso de cada semana deberá ser acorde a lo que se convenga con el docente para cada sprint.
Si el mismo NO cumple con la cantidad de trabajo requerido, el grupo podrá estar desaprobado de forma prematura de la materia, a consideración del docente.

**Nota importante:** Se deja constancia que las funcionalidades requeridas por este enunciado son un marco de cumplimiento mínimo y que pueden haber agregados o modificaciones durante el transcurso del desarrollo por parte del docente a cargo, que formarán parte de los requerimientos a cumplir.
Cabe mencionar que estos desvíos de los requerimientos iniciales se presentan en situaciones reales de trabajo con clientes.

## Finalización del Proyecto

El desarrollo del proyecto finaliza el último día de clases del cuatrimestre. En esa fecha, cada grupo deberá realizar una presentación final y se hará una evaluación global del trabajo.

{{#include nofuncionales.md}}

## Requerimientos Funcionales

### Protocolo

#### Configuración

El servidor MQTT tiene que poder ser inicializado con ciertos parámetros de configuración. Dichos parámetros deberán encontrarse especificados en un archivo de configuración con algún formato conveniente (no se puede utilizar ningún crate para parsear el archivo). Dentro de los parámetros de configuración, deben encontrarse:

- Puerto en cual el servidor escuchará por solicitudes (<em>port</em>).
- Path de archivo sobre el cuál se realizará un <em>dump</em> (ver más adelante).
- Intervalo de tiempo para el cual se realizará el <em>dump</em> (ver más adelante).
- Path de archivo de <em>log</em> (ver más adelante).

<!-- - @@@ ver qué mas. -->

El path del archivo de configuración debe ser pasado como parámetro al ejecutable compilado del servidor.

#### Logging

Es necesario que el servidor loguee las distintas solicitudes que van llegando al mismo, así como también las distintas acciones que va realizando. El path del archivo donde se irán almacenando estos registros será especificado en el archivo de configuración. 

El cumplimiento de este requerimiento puede ser implementado por dos vías: 

- Implementando un log propio (No se considerará válido que el servidor mantenga un file handle global, aunque esté protegido por un lock, y que se escriba directamente al file handle. La arquitectura deberá contemplar otra solución.)
- Investigando y utilizando el crate <em>[tracing](https://docs.rs/tracing/0.1.28/tracing/)</em>.

#### Autenticación

Una de las ventajas del protocolo MQTT es que ofrece la posibilidad de que los clientes deban autenticarse al conectarse al servidor. Para esto, dentro del mensaje de tipo CONNECT, deben especificar valores para los campos <em>username</em> y <em>password</em>. Para implementar esta funcionalidad, el servidor puede poseer un archivo de texto con los usuarios y sus contraseñas en texto plano.

#### Tipos de Paquetes

Se debe investigar e implementar los siguientes tipos de paquetes del protocolo:

- [connect](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718028): Luego de establecida una conexión de red por el cliente hacia el servidor, el primer paquete enviado por el cliente es el CONNECT.
- [connack](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718033): Paquete enviado por el servidor en respuesta a un paquete CONNECT recibido de un cliente.
- [publish](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718037)
- [puback](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718043)
- [pubrel](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718053) 
- [pubcomp](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718058)
- [subscribe](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718063)
- [suback](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718068)
- [unsubscribe](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718072)
- [unsuback](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718077)
- [pingreq](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718081)
- [pingresp](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718086)
- [disconnect](http://docs.oasis-open.org/mqtt/mqtt/v3.1.1/os/mqtt-v3.1.1-os.html#_Toc398718090)
    
#### <em> Quality of service </em> (QoS)

- QoS 0 y 1 (de esta forma se encolan los mensajes de los clientes que están offline porque se desconectaron ungracefully y cuando se vuelven a conectar con el mismo clientId se le pueden enviar los mensajes).

Nota: no se implementará dentro del alcance del presente trabajo los mensajes de QoS 2.

#### Wilcards

- tiene que implementar las wildcards que se pueden utilizar para conformar un topic.

#### Sesiones persistentes

- tienen que implementar la persistent session (no hace falta que implementen el reenvío para los mensajes que no refieron el ACK, pero sí para los mensajes encolados).

#### Retained messages

- tienen implementar retained messages.

#### Last will & testament

- tienen que implementar last will & testament.
- se debe persistir la data necesaria para hacer el delivery de mensajes que no hayan podido ser entregados (respetando QoS nivel 1). Esto implica persistir id + mensajes no entregados.


### Interfaz gráfica

La aplicación cliente deberá contar con una interfaz gráfica que permita interactuar con el servidor. Para realizar dicha aplicación, se debe utilizar el crate [GTK](https://gtk-rs.org/). El grupo deberá investigar cómo utilizar dicho crate.

#### Requerimientos

La interfaz gráfica deberá contar con tres pestañas:

1) **Pestaña de conexión:** Esta pestaña deberá contar con una serie de campos que permitan especificar los parámetros de conexión, y un botón para efectuar la misma. Los parámetros son:

- IP del servidor.
- Puerto del servidor al que hay que conectarse.
- clientId (el servidor debe poder discriminar entre clientes).
- Parámetros que deben ser incluidos en el paquete de tipo CONNECT:
  - Nombre de usuario (<em>username</em>).
  - Contraseña (<em>password</em>).
  - <em>lastWillMessage</em>.
  - <em>lastWillTopic</em>.

2) **Pestaña de publicación:** Esta pestaña permitirá enviar mensajes al servidor. En ella, se deberá poder ingresar un mensaje, un tópico, y tendrá que existir un botón para efectuar el envío. 
Una vez que el servidor confirme la recepción del mensaje, se deberá señalizar el éxito de la operación.

3) **Pestaña de suscripción:** Esta pestaña permitirá suscribirse a distintos tópicos y escuchar por los mensajes que sean publicados a estos.
Para esto, se debe poder:
  - Incluir un tópico del cual se desea escuchar por mensajes.
  - Eliminar un tópico del cual no se desea seguir escuchando.
  - Observar en tiempo real los mensajes que van llegando a los tópicos a los cuales se está suscrito, indicando cuál es tópico en cuestión.
 
**Aclaración importante**: El diseño de la interfaz y cómo se acomodan los componentes de la misma estará a cargo de cada grupo y será validada por los tutores asignados. Dicha interfaz debe cumplir con la cualidad de usabilidad, por lo que se recomienda hacer un diseño simple pero riguroso para cumplir con este requerimiento.  
