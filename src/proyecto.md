# Proyecto

[Proyecto: **Redis Oxidado** - 1er Cuatrimestre 2021](./proyecto/Proyecto_2021_1C_Redis.pdf)

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
