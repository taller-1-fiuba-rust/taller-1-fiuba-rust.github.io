# Proyecto: Redis Oxidado - 1er Cuatrimestre 2021

[**Descargar en PDF**](./proyecto/Proyecto_2021_1C_Redis.pdf)

## Introducción

[**Redis**](https://redis.io/) es un almacenamiento principalmente en memoria, usado como una Base de Datos de tipo **clave / valor** en memoria, como también como <em>caché</em> y <em>broker</em> de mensajes, con opción a persistencia de los datos.

Redis soporta distintos tipos de estructuras de datos: strings, listas, hashmaps, sets, sets ordenados, bitmaps, entre varios otros.

Redis tiene una muy buena performance, dado que trabaja con los datos en memoria. Es posible persistir los datos periódicamente a un almacenamiento de disco.

Soporta otras funcionalidades como: transacciones, <em>publishers/suscribers</em>, clave con un tiempo de vida limitado, réplicas asincrónicas distribuidas, entre otras. Se puede utilizar clientes Redis desde la mayoría de los lenguajes de programación. Es un proyecto open source. Es una [base de datos muy popular](https://db-engines.com/en/ranking_trend) (la de mayor uso del tipo clave / valor).

Los usos principales de Redis son como [cache de aplicación](https://redislabs.com/solutions/use-cases/caching/) para mejorar los tiempos de latencia de una aplicación (y aumentar la capacidad de procesamiento de operaciones -<em>requests</em>- por segundo), para almacenar datos de sesión de los usuarios, o funcionalidades como limitar la cantidad de pedidos que puede realizar un cliente en cierto tiempo (<em>rate limiter</em>), para prevenir ataques de denegación de servicio, por ejemplo.

Otros casos de uso de Redis son la implementación del pasaje de mensajes entre publicadores y suscriptores de ciertos tipos de mensajes (que se suscriben a mensajes de algún tópico), o la implementación de colas de tareas para el procesamiento en paralelo de pedidos.

## Objetivo del Proyecto

El objetivo del proyecto es implementar un **Servidor Redis** con funcionalidades acotadas, que se detallan en el presente enunciado.

Se presente emular, en la medida de lo posible, el proceso de desarrollo de la Industria de Software.

@@@@@@@@@@@@@@@@@@
---

## Corriendo Redis y su cliente en Docker

- Instalar docker segun el sistema operativo que estes usando. 
- Descargar y correr una imagen de docker con redis instalado:
  `docker run -d -p 6379:6379 --name redis-taller1 redis`
- Verificar que estar corriedo: 
  `docker ps`
- Acceder a los logs de redis:
  `docker logs redis-taller1`
- Ejecutar otro contenedor con la misma imagen, pero en modo interactivo y una shell:
  `docker exec -it redis-taller1 sh`
- Dentro de este contender, ejecutar el cliente:
  `redis-cli`
- Verificar que esta conectado al servidor redis:
```
127.0.0.1:6379> ping
PONG
```
- Ejecutar comandos redis:
```
127.0.0.1:6379> set name mark
OK
127.0.0.1:6379> get name
"mark"
127.0.0.1:6379> incr counter
(integer) 1
127.0.0.1:6379> incr counter
(integer) 2
127.0.0.1:6379> get counter
"2"
```
- Cerrar el cliente redis:
```
127.0.0.1:6379> exit
# exit
```
