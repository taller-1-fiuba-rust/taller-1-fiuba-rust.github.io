# Alcance de la entrega intermedia del Trabajo Práctico
 
Se deberá realizar una entrega intermedia el día **6 de Junio**, la cual deberá cumplir con los los siguiente requerimientos:

* El programa deberá recibir por linea de comandos la ruta de un archivo **.torrent**
* El archivo **.torrent** debe ser leído y decodificado según el estándar y su información almacenada en memoria.
* Se deberá conectar al **Tracker** obtenido en el **.torrent**, decodificar la respuesta y obtener la lista de **peers**.
* Dada la lista de **peers**, deberá poder iniciar una conexión con al menos un **peer** y realizar el **Handshake**.
* Realizado el **Handshake** se debe iniciar el intercambio de mensajes para poder descargar una pieza del torrent.
* La pieza descargada se debe validar según el protocolo y almacenar en disco indicando el nro de pieza en el nombre de archivo.
* Implementación del Logger. (**opcional - bonus point**) 

La entrega se realizara en forma de Demostración (**Demo**) en la cual los alumnos deberán abarcar los siguientes puntos:

* Explicación general de la solución, incluyendo diagramas que muestren el diseño desarrollado.
* Recorrido por el código fuente escrito, explicando los principales contenidos de cada módulo.
* Demo en vivo del programa, en donde se comprobará que el programa cumple con los puntos solicitados.
* Verificación en vivo de la integridad de la pieza descargada. (**SHA1**)

**Nota:** Todos los miembros del grupo deberán participar de la demo y explicar su participación en el proyecto, incluyendo detalles de implementación.
