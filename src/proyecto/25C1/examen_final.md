# Agregado Final 1C 2025: Microservicio de AI

## Introducción
Los Large Language Models (LLM) son modelos avanzados de inteligencia artificial, basados en arquitecturas de redes neuronales profundas, que han sido entrenados con grandes volúmenes de datos textuales. Estos modelos son capaces de comprender, generar y manipular el lenguaje natural de manera altamente contextualizada, lo que les permite desempeñarse eficientemente en tareas complejas como la redacción automática, traducción, resumen de textos y respuesta a preguntas, entre otras aplicaciones.


## Objetivo
Se deberá incorporar al Trabajo Práctico realizado durante el cuatrimestre un microservicio de LLM utilizando alguno de los proveedores de infraestructura en la nube reconocidos, como por ejemplo:
* [OpenAI](https://platform.openai.com/docs/api-reference/introduction)
* [Gemini API](https://ai.google.dev/gemini-api/docs)
* [AWS Bedrock](https://docs.aws.amazon.com/bedrock/latest/APIReference/welcome.html)
* Otro (a convenir con los docentes)

**Nota:** Se debe seleccionar un proveedor que ofrezca una capa de uso gratuito (free tier) suficiente para probar el trabajo realizado y para realizar la demostración final en vivo.

## Requerimientos funcionales
### Microservicio de LLM
El microservicio de LLM será el responsable de resolver pedidos de generación o modificación de texto introducidos por el usuario mediante un *prompt*. Para ello se debe crear un nuevo canal (channel) de Redis mediante el cual los clientes podrán enviar solicitudes a este microservicio, las respuestas a dichas solicitudes serán enviadas por el microservicio publicando en el canal correspondiente al documento sobre el cual se hizo el pedido. De esta manera el microservicio y los clientes sólo se comunicarán a través de canales de Redis y nunca en forma directa.
Para resolver los pedidos de los clientes el microservicio deberá comunicarse con el servicio externo proveedor de AI que los alumnos hayan elegido para la implementación del presente trabajo.
Se pide utilizar una estrategia de multithreading que permita ejecutar varios requests en paralelo, ya sea mediante el uso de una [Thread Pool](https://doc.rust-lang.org/book/ch20-02-multithreaded.html#improving-throughput-with-a-thread-pool) o cualquier otro mecanismo implementado con [channels](https://doc.rust-lang.org/book/ch16-02-message-passing.html).

**Nota:** Los alumnos podrán optar por utilizar un modelo local de LLM que resuelva la generación y edición de texto, en lugar de un proveedor en la nube. 
Se advierte en este caso que deberán investigar su funcionamiento y el modo de entrenamiento necesario para cumplir con las necesidades de este proyecto.

### Interfaz gráfica
Realizar los cambios en la aplicación para permitir el uso de AI al editar un documento. Se debe poder ingresar el *prompt* con la solicitud deseada apuntando a cualquier parte del documento y al recibir la respuesta de la AI el resultado debe ser aplicado al punto especificado al realizar el comando.
Punto extra: Se debe poder ingresar un *prompt* a nivel general del documento, en el cual se pida realizar cambios sobre el texto completo del documento, por ejemplo, reescribirlo mas formal, o traducirlo a otro idioma. En dicho caso el mensaje a enviar al microservicio debe contener solo el *prompt* y el identificador del documento y el microservicio debe ser capaz de obtener el texto completo y enviarlo al servicio externo de AI para su modificación.

### Docker
Se deberán generar todos los artefactos necesarios para el *deployment*  del cluster de Redis y los dos microservicios del trabajo practico utilizando *Docker*. Incluyendo la generación de imágenes, containers, recursos (por ej almacenamiento) y configuraciones necesarias para la ejecución completa del proyecto.
Como mínimo se deberán incluir en la entrega los siguientes artefactos:
1. **Dockerfile**: para la generación de la imagen del nodo.
2. **Docker-compose**: para la definición de todos los contenedores de la red, incluyendo sus configuraciones y recursos necesarios.
3. Todos los comandos necesarios para construir, iniciar, detener y destruir el cluster.
4. Agregar un archivo README en la carpeta raíz del repositorio indicando todas las instrucciones para construir, iniciar, detener y destruir el ambiente, incluyendo todos los contenedores y sus datos necesarios para funcionar.

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
Durante la demostración en vivo del sistema se deberá poder introducir en cualquier parte de un documento un texto generado por la AI, para lo cual se debe poder ingresar el *prompt* con la solicitud deseada. 
Se debe poder demostrar como la solicitud realizada viaja al microservicio de AI y luego desde el mismo hacia el proveedor de LLM utilizado, para luego responder a esta solicitud mediante el topic correspondiente al documento sobre el que se realizó el comando.


## Informe final
Solo se podrán presentar a la fecha de final teniendo completo el informe final del proyecto desarrollado durante el cuatrimestre, incluyendo una sección adicional para describir la implementación de este agregado que contemple los siguientes puntos:
* Los detalles considerados para la elección de proveedor externo de AI.
* El análisis de costos de funcionamiento del proyecto basado en una estimación de carga del sistema en funcionamiento real y utilizando la información de costos del proveedor de servicios seleccionado. 
* Diagramas de diseño correspondientes a la implementación de este agregado que clarifiquen la estrategia de **multi-threading** aplicada (por ej, diagramas de secuencia)


## Fechas de final
El grupo deberá presentarse en una de las fechas de examen final, tal como se publica en el calendario respectivo.
Las fechas de final para el cuatrimestre actual son:

- lunes 7/7
- lunes 14/7
- lunes 28/7
- lunes 4/8
- lunes 11/8
  
