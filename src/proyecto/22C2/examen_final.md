# Examen Final 2do Cuatrimestre 2022

## Enunciado para quienes cursaron en 2do cuatrimestre 2022

Se deberá implementar la extension al protocolo IRC denominada Direct Client-to-Client ([DCC](https://www.irchelp.org/protocol/dccspec.html)), para dar soporte a conversaciones seguras entre clientes y envio de archivos P2P.

Esta funcionalidad deberá poder acceder desde un botón u otro elemento en la interfaz gráfica del cliente desarrollado, en la ventana de chat con otro participante. Se deberá solicitar el archivo a enviar y se procederá a su envío.

Se deberá realizar una presentacion explicando la implementacion este agregado.

### Requisitos funcionales

Se deberá considerar los siguientes requisitos:

1. Mediante el uso de mensajes [CTCP](https://es.wikipedia.org/wiki/CTCP) implementar las extensiones DCC CHAT y DCC SEND.
2. El mensaje [DCC CHAT](https://es.wikipedia.org/wiki/Cliente-a-Cliente_Directo#Chat_DCC) es utilizado para iniciar una conversacion segura punto a punto entre dos clientes. La estructura del mismo es: `DCC CHAT <protocolo><ip><port>` donde protocolo tiene el valor `chat`, `ip` es la direccion IP del cliente que inicia la conversacion y `port` es el puerto del cliente donde esperará por la conexion entrante. Para finalizar la comunicación se utiliza el mensaje `DCC CLOSE`
3. El mensaje [DCC SEND](https://es.wikipedia.org/wiki/Cliente-a-Cliente_Directo#DCC_Send) es utilizado para iniciar la transferencia de un archivo entre clientes. La estructura del mensaje es: `DCC SEND <filename><ip><port><file size>` donde `filename` y `file size` son el nombre y tamano del archivo a enviar, IP es la direccion IP del cliente que inicia la transferencia del archivo y `port` es el puerto del cliente donde esperará por la conexión entrante. En caso de reanudar una transferencia interrumpida el cliente puede enviar la respuesta `DCC RESUME <filename><port><position>` para reanudar la transmision desde el punto indicado en `position`.

### Funcionalidad opcional

Extender el punto 3 para agregar soporte a algunas de las siguientes funcionalidades: encriptación, compresión y control de integridad. Para este punto pueden utilizar los siguientes crates externos:

* [zip](https://crates.io/crates/zip) para compresión.
* [sha](https://crates.io/crates/sha) para generar hashes.
* un crate para realizar encriptación.

### Presentación

Se deberá realizar una presentacion explicando la implementacion este agregado, incluyendo las decisiones de diseño y una demostración de la funcionalidad.

El grupo deberá presentarse en una de las fechas de examen final, tal como se publica en el calendario respectivo.
