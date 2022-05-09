# Agregado al proyecto: Monitor de Redis

## Objetivo

Se debe implementar un sitio web para realizar operaciones sobre la base de datos Redis implementada en la primera parte del trabajo práctico.

Este servidor será similar a la demostración del sitio de Redis: <https://try.redis.io/>

## Requerimientos Funcionales

Se debe implementar un servidor web que reciba pedidos (<em>requests</em>) de browsers, comunicándose con los mismos a través del protocolo HTTP/1.1. La descripción de este protocolo es la correspondiente a la [RFC 2616](https://datatracker.ietf.org/doc/html/rfc2616).

El servidor debe escuchar pedidos HTTP en el puerto TCP 8080 y se comunicará con el servidor Redis desarrollado a partir del protocolo implementado.

### Acerca de HTTP

El protocolo HTTP es usado globalmente para el intercambio de información en la Web desde 1990. Es orientado a cadenas de caracteres, 
HTTP es un protocolo de tipo cliente-servidor que opera con mensajes pedido/respuesta
(request/reply). El cliente es el denominado agente de usuario (o user agent, en inglés) y puede ser
un browser, un editor, un crawler u otro software para el usuario final. El servidor es un
programa que acepta conexiones entrantes para responder a los pedidos (requests), con el envı́o
de respuestas (replies).

HTTP provee encabezados (headers) para enviar el pedido, con métodos para indicar el
tipo de pedido y define a la ubicación del recurso (por ejemplo, una "página web") referido a partir de su URI (<em>Uniform Resource
Identifier</em>).

### Funcionalidades a implementar

Al ingresar al sitio principal del sitio, se deberá mostrar una página web que deberá mostrar un recuadro donde el usuario podrá escribir comandos de Redis y debajo del mismo un botón "Enviar" que deberá invocar un comando HTTP **POST**. El servidor web deberá actuar como cliente Redis y conectarse al servidor implementado en la primera parte del Trabajo Práctico.

La respuesta del servidor Redis la deberá re-enviar el servidor web al browser y mostrar en un recuadro.

Se deberá implementar las comunicaciones HTTP entre el servidor web con mensajes del protocolo, respetando los headers.

Se deberá implementar el cliente del protocolo Redis hacia el servidor. **NO** se permite el uso de crates externos para la implementación del cliente Redis ni para el servidor HTTP.


## Requerimientos no funcionales

* Valen los mismos requerimientos que para la primera parte del Trabajo Práctico.



## Condiciones de aprobación

(Serán detalladas en clase)