# Agregado Final 2023C2: Releases

## Introducción

Durante el cuatrimestre se desarrolló un proyecto de mediana envergadura en Rust, donde se implementó un **Cliente** y un **Servidor** de **Git**. Para este final se propone acoplar a lo realizado durante el cuatrimestre un agregado donde el **Servidor** incorpore una API web para la administración de **Releases**. 


## Requerimientos funcionales

El **Servidor**, además de cumplir con los requerimientos funcionales y no funcionales establecidos en el enunciado del Trabajo Práctico, deberá exponer una API web en un puerto indicado por configuración, por el cual recibirá **Requests** respetando el protocolo **HTTP** para realizar las siguientes operaciones: 
1. Crear un Release: `POST /repos/{repo}/releases`. Ver [especificación](https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#create-a-release) de referencia.
2. Listar Releases: `GET /repos/{repo}/releases`. Ver [especificación](https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#list-releases) de referencia.
3. Obtener el último Release: `GET /repos/{repo}/releases/latest`. Ver [especificación](https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#get-the-latest-release) de referencia.
4. Obtener un Release buscando por Tag: `GET /repos/{repo}/releases/tags/{tag}`. Ver [especificación](https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#get-a-release-by-tag-name) de referencia.
5. Generar notas de Release: `POST /repos/{repo}/releases/generate-notes`. Ver [especificación](https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#generate-release-notes-content-for-a-release) de referencia.
6. (opcional) Modificar un Release: `PATCH /repos/{repo}/releases/{release_id}`. Ver [especificación](https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#update-a-release) de referencia.
7. (opcional) Eliminar un Release: `DELETE /repos/{repo}/releases/{release_id}`. Ver [especificación](https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#delete-a-release) de referencia.
8. (opcional) Soportar mas de un [media type](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types).

Como mínimo se pide implementar el media type: `application/json`


## Requerimientos no funcionales
Los siguientes son los requerimientos no funcionales para la resolución del proyecto:
* El proyecto deberá ser desarrollado en lenguaje **Rust**, usando las herramientas de la biblioteca estándar.
* Se deben implementar **tests unitarios y de integración** de las funcionalidades que se consideren más importantes.
* El código fuente debe compilarse en la versión stable del compilador y no se permite utilizar bloques unsafe.
* La compilación no debe arrojar **warnings** del compilador, ni del linter **clippy**.
* Las funciones y los tipos de datos (**struct**) deben estar documentadas siguiendo el estándar de **cargo doc**.
* El código debe formatearse utilizando **cargo fmt**.
* El programa deberá funcionar en ambiente Unix / Linux.
* Solo se permite el uso **crates** externos listados en la sección correspondiente.


## Presentación
Se deberá realizar una **presentación** explicando la implementación de este agregado, incluyendo las decisiones de diseño y una demostración de la funcionalidad. 
Dentro de los detalles de implementación se deberá explicar la solución adoptada desde el punto de vista de **multi-threading** y performance, con diagramas que faciliten la explicación.
Para cumplir con los requisitos mínimos de este agregado se deberá verificar el funcionamiento todas las APIs listadas en la sección de requerimientos funcionales. Para ello los alumnos podrán elegir una herramienta para realizar los **requests en vivo** o en su defecto utilizar el comando [curl](https://curl.se/).
Los **requests** a realizar durante la demo en vivo deberán estar preparados con anterioridad y guardados en algún archivo del repositorio para poder ser utilizados durante la corrección del presente trabajo.


## Crates externos permitidos
Además de los crates permitidos en el enunciado del Trabajo Práctico se permite el uso de los siguientes crates solo para la implementación del presente agregado:

- [`serde`](https://crates.io/crates/serde): para la serializacion y deserializacion del payload de mensaje HTTP.
- [`serde_json`](https://crates.io/crates/serde_json): para la serializacion y deserializacion utilizando JSON.


## Fechas de final
El grupo deberá presentarse en una de las fechas de examen final, tal como se publica en el calendario respectivo.
