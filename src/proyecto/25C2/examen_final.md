# Agregado Final 2C 2025: Soporte para audio y envio de archivos

## Introducción
Mediante el uso de WebRTC no solo es posible transmitir video en tiempo real, sino que dada a su versatilidad y extensibilidad es posible adaptarlo para el envio de otros tipos de datos. Una extension natural al sistema desarrollado durante el cuatrimestre es la posibilidad de transmitir audio en tiempo real.
Pero el protocolo tambien para el envio de datos en general, siendo posible enviar archivos, mensajes o incluso [implementar un algoritmo de consenso](https://eevans.co/blog/wraft/) sobre lo ofrecido por WebRTC.

## Objetivo
Se debera incorporar al sistema desarrollado durante el cuatrimestre:
- Transmision de audio en tiempo real.
- Transmision de archivos, mediante el uso de canales de datos.

## Requerimientos funcionales
### Transmision de audio
Se debera implementar la transmision de audio en tiempo real, mediante el uso de WebRTC. El audio debe estar sincronizado con la transmision de video.
En la interfaz grafica se debera incluir un boton para pausar la transmision de audio (mute/unmute).

### Transmision de archivos
Se debera implementar la transmision de archivos, mediante el uso de [data channels](https://webrtcforthecurious.com/docs/07-data-communication/).
En la interfaz gráfica se deberá incluir un botón para enviar archivos a la otra parte de la llamada. Siendo posible el envío de archivos de cualquier tipo y tamaño.
El archivo enviado debe ser recibido por la otra parte de la llamada (siendo posible rechazar su descarga) y guardado en el disco.

# Requerimientos no funcionales
Los siguientes son los requerimientos no funcionales para la resolución del proyecto:
* El proyecto deberá ser desarrollado en lenguaje **Rust**, usando las herramientas de la biblioteca estándar.
* Se deben implementar **tests unitarios y de integración** de las funcionalidades que se consideren más importantes.
* El código fuente debe compilarse en la versión stable del compilador y no se permite utilizar bloques unsafe.
* La compilación no debe arrojar **warnings** del compilador, ni del linter **clippy**.
* Las funciones y los tipos de datos (**struct**) deben estar documentadas siguiendo el estándar de **cargo doc**.
* El código debe formatearse utilizando **cargo fmt**.
* El programa deberá funcionar en ambiente Unix / Linux.
* Solo se permite el uso **crates** externos ligados a la comunicación con los servicios externos (en caso de ser necesarios).

## Presentación
Se deberá realizar una **presentación** explicando la implementación de este agregado, incluyendo las decisiones de diseño y una demostración de la funcionalidad en vivo. 
Dentro de los detalles de implementación se deberá explicar la solución adoptada desde el punto de vista de **multi-threading**, con diagramas que faciliten la explicación.


## Informe final
Solo se podrán presentar a la fecha de final teniendo completo el informe final del proyecto desarrollado durante el cuatrimestre, incluyendo una sección adicional para describir la implementación de este agregado que contemple los siguientes puntos:
* Diagramas de diseño correspondientes a la implementación de este agregado que clarifiquen la estrategia de **multi-threading** aplicada (por ej, diagramas de secuencia)


## Fechas de final
El grupo deberá presentarse en una de las fechas de examen final, tal como se publica en el calendario respectivo.
Las fechas de final para el cuatrimestre actual son:


TBD