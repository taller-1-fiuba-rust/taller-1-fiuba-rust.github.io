# Agregado Final 1C 2024: Reconocimiento de Imágenes

## Introducción
El reconocimiento de imágenes es una tecnología de inteligencia artificial que permite a las computadoras interpretar y comprender el contenido de imágenes digitales. Utiliza algoritmos de aprendizaje automático, particularmente redes neuronales convolucionales (CNN), para identificar y clasificar objetos, personas, escenas y otras características visuales dentro de una imagen. Este proceso comienza con la adquisición y preprocesamiento de grandes volúmenes de imágenes etiquetadas, seguido del entrenamiento del modelo que aprende a reconocer patrones y características específicas. Una vez entrenado, el modelo se evalúa y se implementa en aplicaciones prácticas, como seguridad y vigilancia, diagnóstico médico, vehículos autónomos y comercio electrónico.

## Objetivo
Se deberá incorporar la tecnología de reconocimiento de imágenes al sistema central de cámaras desarrollado durante el Trabajo Práctico del cuatrimestre, utilizando alguno de los proveedores de infraestructura en la nube reconocidos, como por ejemplo:
* [Azure AI Vision](https://learn.microsoft.com/en-us/azure/ai-services/computer-vision/overview)
* [Google Vision AI](https://cloud.google.com/vision)
* [Amazon Rekognition Image](https://aws.amazon.com/rekognition/image-features/)
* Otra, a convenir con los docentes

**Nota:** Se debe seleccionar un proveedor que ofrezca una capa de uso gratuito (free tier) suficiente para probar el trabajo realizado y para realizar la demostración final en vivo con una carga de procesamiento mediana, como mínimo de 10 requests por minuto. No se permite el uso de [OpenAI Vision](https://platform.openai.com/docs/guides/vision) debido a que no cumple con esta caracteristica.

## Requerimientos funcionales
El sistema central de cámaras deberá ser capaz de procesar imágenes generadas por las cámaras que se encuentren en modo pasivo, y utilizar la tecnología de reconocimiento de imágenes para detectar potenciales incidentes. En caso de considerar que la imagen procesada corresponde a un incidente deberá hacer uso del sistema de mensajería para publicar un mensaje que describa esta situación, y por consiguiente la aplicación deberá automáticamente generar el incidente correspondiente y poner en marcha todo el circuito de resolución de incidentes implementado durante el desarrollo del Trabajo Práctico.

Para simular la generación de imágenes por parte de la cámaras se deberá especificar un directorio por configuración en el cual se irán copiando manualmente las imágenes y las mismas deberán ser procesadas por la aplicación inmediatamente, es decir la aplicacion deberá monitorear frecuentemente estos directorios para detectar la presencia de nuevas imágenes para procesar. Este directorio deberá contener subdirectorios por cada una de las cámaras en funcionamiento de manera que la imagen depositada en un directorio correspondiente a una cámara particular se considerará como una imagen tomada por dicha cámara. 

Para comunicarse con el proveedor de servicios de la nube se debe utilizar una estrategia de multithreading que permita ejecutar varios requests en paralelo, ya sea mediante el uso de una [Thread Pool](https://doc.rust-lang.org/book/ch20-02-multithreaded.html#improving-throughput-with-a-thread-pool) o cualquier otro mecanismo implementado con [channels](https://doc.rust-lang.org/book/ch16-02-message-passing.html).

**Nota:** Los alumnos podrán optar por utilizar un modelo local de AI que resuelva el procesamiento de imágenes, en lugar de un proveedor en la nube. 
Se advierte en este caso que deberán investigar su funcionamiento y el modo de entrenamiento necesario para cumplir con las necesidades de este proyecto.

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
Durante la demostración en vivo del sistema se deberá ir copiando distintas imagenes (una a una) a los directorios correspondientes de varias cámaras y a medida que estas imágenes se van copiando y procesando por la aplicacion, se deberá visualizar como las imágenes que den cuenta de potenciales incidentes generan los correspondientes incidentes en el sistema (en tiempo real) y su eventual atención y resolución por parte de los agentes.


## Informe final
Solo se podrán presentar a la fecha de final teniendo completo el informe final del proyecto desarrollado durante el cuatrimestre, incluyendo una sección adicional para describir la implementación de este agregado que contemple los siguientes puntos:
* Los detalles considerados para la elección de proveedor externo de AI.
* El análisis de costos de funcionamiento del proyecto basado en una estimación de carga del sistema en funcionamiento real y utilizando la información de costos del proveedor de servicios seleccionado. 
* Diagramas de diseño correspondientes a la implementación de este agregado que clarifiquen la estrategia de **multi-threading** aplicada (por ej, diagramas de secuencia)


## Fechas de final
El grupo deberá presentarse en una de las fechas de examen final, tal como se publica en el calendario respectivo.

Las fechas de final para el cuatrimestre actual son:
- lunes 1/7
- lunes 15/7
- lunes 29/7
- lunes 5/8
- lunes 12/8
