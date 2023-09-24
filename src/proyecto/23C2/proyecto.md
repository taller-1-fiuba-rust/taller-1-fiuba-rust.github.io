# Proyecto: Git Rústico - 2do Cuatrimestre 2023

## Introducción a Git
Git es un sistema de control de versiones distribuido que se utiliza para rastrear cambios en el código fuente de un proyecto a lo largo del tiempo. Permite a múltiples personas colaborar en un proyecto, realizar seguimiento de cambios y fusionar contribuciones de diferentes desarrolladores de manera eficiente.

## Objetivo del Proyecto
El objetivo principal del presente proyecto de desarrollo consiste en la implementación de un Cliente y Servidor Git con funcionalidades acotadas que se detallan en el presente enunciado. Siguiendo las guías de desarrollo y especificaciones de Git.

El objetivo secundario del proyecto consiste en el desarrollo de un proyecto real de software de mediana envergadura aplicando buenas prácticas de desarrollo de software, incluyendo entregas y revisiones usando un sistema de control de versiones.

Se espera que se haga un uso lo más idiomático posible del lenguaje de programación Rust, siguiendo los estándares que éste promueve.

## Requerimientos funcionales
Los siguientes son los requerimientos funcionales para el desarrollo del Trabajo.

* Clonación de repositorios: El cliente Git deberá ser capaz de clonar repositorios remotos en la máquina local, incluyendo la historia de commits y ramas.
* Comandos básicos: El cliente deberá permitir la ejecución de comandos básicos de Git, como agregar cambios (add), realizar commits, subir cambios (push) y descargar cambios (pull). Los comandos requeridos estan detallados en la sección: Implementación del Cliente Git.
* Servidor: El servidor debera soportar el protocolo [Git transport](https://git-scm.com/docs/pack-protocol#_git_transport) permitiendo al cliente descargar el contenido necesario respetando la implementacion de [upload-pack](https://git-scm.com/docs/pack-protocol#_fetching_data_from_a_server) y [receive-pack](https://git-scm.com/docs/pack-protocol#_pushing_data_to_a_server)
* Interfaz gráfica: Se debe implementar una interfaz gráfica simple para visualizar la historia de commits, gestionar ramas y realizar operaciones básicas. La apariencia puede ser similar a la interfaz de [GitKraken](https://www.gitkraken.com/). 
* Archivo de Configuración: El cliente Git deberá poder ser configurado mediante un archivo de configuración, de manera similar al archivo "gitconfig" de Git. Este archivo deberá incluir información como el nombre y correo del usuario.
* Archivo de Log: El cliente Git debe mantener un registro de las acciones realizadas y mensajes relevantes en un archivo de log. La ubicación del archivo de log estará especificada en el archivo de configuración. Como requerimiento particular del Proyecto, NO se considerará válido que el servidor mantenga un file handle global, aunque esté protegido por un lock, y que se escriba directamente al file handle. La arquitectura deberá contemplar otra solución.


## Implementación del Cliente Git
El cliente Git debe permitir realizar las operaciones basicas de manejo de repositorios, incluyendo crear repositorios, clonar repositorios existentes, actualizar la informacion local del repositorio y enviar las actualizaciones al servidor (commits, branches, etc.). Para cumplir con este objetivo se deberan implementar los comandos detallados en el listado siguiente, en el cual se distinguen comandos requeridos para la entrega **intermedia** y comandos requeridos para la entrega **final**.

Es importante destacar que estos comandos soportan una amplia diversidad de parametros y modificadores que complejizan su implementacion, por este motivo en el presente trabajo nos concentraremos en la funcionalidad basica de cada uno de ellos, siempre que esto sea suficiente para cumplir con los casos de uso tipicos de la herramienta (cada grupo debera analizar y validar con sus correctores las decisiones tomadas respecto a este punto)

A continuacion se listan los comandos requeridos:

#### Comandos requeridos para la entrega intermedia:
* hash-object ([git man page](https://git-scm.com/docs/git-hash-object))
* cat-file ([git man page](https://git-scm.com/docs/git-cat-file))
* init ([git man page](https://git-scm.com/docs/git-init))
* status ([git man page](https://git-scm.com/docs/git-status))
* add ([git man page](https://git-scm.com/docs/git-add))
* rm ([git man page](https://git-scm.com/docs/git-rm))
* commit ([git man page](https://git-scm.com/docs/git-commit))
* checkout ([git man page](https://git-scm.com/docs/git-checkout))
* log ([git man page](https://git-scm.com/docs/git-log))
* clone ([git man page](https://git-scm.com/docs/git-clone))
* fetch ([git man page](https://git-scm.com/docs/git-fetch))
* merge ([git man page](https://git-scm.com/docs/git-merge))
* remote ([git man page](https://git-scm.com/docs/git-remote))
* pull ([git man page](https://git-scm.com/docs/git-pull))
* push ([git man page](https://git-scm.com/docs/git-push))
* branch ([git man page](https://git-scm.com/docs/git-branch))

#### Comandos requeridos para la entrega final:
* check-ignore ([git man page](https://git-scm.com/docs/git-check-ignore))
* ls-files ([git man page](https://git-scm.com/docs/git-ls-files))
* ls-tree ([git man page](https://git-scm.com/docs/git-ls-tree))
* show-ref ([git man page](https://git-scm.com/docs/git-show-ref))
* rebase ([git man page](https://git-scm.com/docs/git-rebase))
* tag ([git man page](https://git-scm.com/docs/git-tag))

**Nota:** Durante el desarrollo del cliente se recomienda utilizar el servicio de [git daemon](https://git-scm.com/docs/git-daemon) para poder realizar pruebas conectando a un servidor git real. 

## Implementación del Servidor Git

El Servidor permitirá al cliente descargar el contenido del repositorio respetando la implementacion de [upload-pack](https://git-scm.com/docs/pack-protocol#_fetching_data_from_a_server) y recibir actualizaciones de contenido por parte de los clientes mediante [receive-pack](https://git-scm.com/docs/pack-protocol#_pushing_data_to_a_server) siguiendo el protocolo [Git Transport](https://git-scm.com/book/en/v2/Git-on-the-Server-The-Protocols#_the_git_protocol).
Estas operaciones serán utilizadas por el cliente para implementar los comandos que interactuan con el Servidor, como por ejemplo fetch, pull, push, etc.
A continuacion se puede observar un diagrama que representa la interaccion entre distintos Clientes que se conectan al Servidor:

![](https://hackmd.io/_uploads/SyaYQsqTn.png)

**Importante:** El Servidor deberá ser capaz de atender solicitudes de multiples clientes al mismo tiempo, para lo cual se pide utilizar multiples **threads** de manera de paralelizar el trabajo del Servidor. No se considerara válida una implementacion en la cual la peticion de un cliente deba esperar a que finalice el procesamiento de peticiones previas de otros clientes.

En los casos en los que se presenten un conflictos entre cambios realizados por un cliente y cambios realizados por otro se deben resolver estos conflictos siguiendo el estandar del protocolo git.  


## Interfaz gráfica
La interfaz gráfica debe permitir al usuario realizar **todas** las operaciones soportadas por el cliente (comandos requeridos) y visualizar la historia de commits y branches en forma grafica de manera similar a como se puede observar en cualquier herramienta comercial, como por ejemplo [GitKraken](https://www.gitkraken.com/) 

![](https://hackmd.io/_uploads/rJ8tKocTn.png)


En los casos en los que se presente un conflicto entre commits la interfaz debe permitir en forma grafica visualizar y resolver estos conflictos.

![](https://hackmd.io/_uploads/rJRxqi5Th.png)



## Requerimientos no funcionales
Los siguientes son los requerimientos no funcionales para la resolución del proyecto:

* El proyecto deberá ser desarrollado en lenguaje Rust, utilizando las herramientas de la biblioteca estándar.
* Se deben implementar pruebas unitarias y de integración de las funcionalidades que se consideren más importantes.
* El código fuente debe compilar en la versión estable del compilador y no se permite el uso de bloques inseguros (unsafe).
* El código deberá funcionar en ambiente Unix / Linux.
* La compilación no debe generar advertencias del compilador ni del linter clippy.
* Las funciones y los tipos de datos (struct) deben estar documentados siguiendo el estándar de cargo doc.
* El código debe formatearse utilizando cargo fmt.
* Las funciones no deben tener una extensión mayor a 30 líneas. Si se requiere una extensión mayor, se debe particionar en varias funciones.
* Cada tipo de dato implementado debe ser colocado en una unidad de compilación (archivo fuente) independiente.

## Entrega intermedia

Los alumnos deberan realizar una entrega intermedia, la cual deberá incluir los siguientes puntos desarrollados en la sección Requerimientos funcionales:

* Clonación de repositorios
* Comandos básicos (init, status, add, commit, push, pull) detallados en el listado de comandos requeridos para la entrega **intermedia**
* Implementación de una interfaz gráfica simple para la gestión de commits y ramas. 

La entrega se realizará en forma de Demostración (Demo) en la cual los alumnos deberán abarcar los siguientes puntos:

Explicación general de la solución, incluyendo diagramas que muestren el diseño desarrollado.
Recorrido por el código fuente escrito, explicando los principales contenidos de cada módulo.
Demo en vivo del programa, en donde se comprobará que el programa cumple con los puntos solicitados. 
Ademas se debe demostrar que el cliente desarrollado puede conectarse indistintamente a su propio servidor como tambien a un servidor iniciado con [git daemon](https://git-scm.com/docs/git-daemon) 

Todos los miembros del grupo deberán participar de la demo y explicar su participación en el proyecto, incluyendo detalles de implementación.

## Crates externos permitidos
Se permite el uso de los siguientes crates solo para los usos mencionados (siempre y cuando se los considere necesario):

*  rand: para la generación de valores aleatorios.
* chrono: para la obtención del timestamp actual.

## Material de consulta
* Sitio oficial de Git: https://git-scm.com/
* Pro Git, Scott Chacon y Ben Straub: https://git-scm.com/book/en/v2
* Documentación de Rust: https://doc.rust-lang.org/

## Criterios de Aceptación y Corrección del Proyecto
Para el desarrollo del proyecto, se deberá observar los siguientes lineamientos generales:
1. **Testing:** Se deberá implementar testing unitario automatizado, utilizando las herramientas de Rust de los métodos y funciones relevantes implementados. Se deberán implementar tests de integración automatizados.
2. **Manejo de Errores:** Deberá hacerse un buen uso y administración de los casos de error, utilizando para ello, las estructuras y herramientas del lenguaje, escribiendo en forma lo más idiomática posible su tratamiento.
3. **Control de versiones:** Se deberá utilizar la herramienta git, siguiendo las recomendaciones de la cátedra. En particular, se deberá utilizar la metodología GitHub Flow para el trabajo con ramas (branches) y la entrega continua del software.
4. **Trabajo en equipo:** Se deberá adecuar, organizar y coordinar el trabajo al equipo, realizando tareas como revisión de código cruzada entre pares de una funcionalidad en un pull request de git.
5. **Merge de Branchs:** Para poder hacer el merge de un branch de una funcionalidad, todos los tests pasan de forma satisfactoria
6. **Informe final:** El trabajo debe acompañarse por un informe que debe incluir diagramas de secuencia de las operaciones más relevantes, diagrama de componentes y módulos de la arquitectura general del diseño desarrollado, todos acompañados de la explicación respectiva.

## Evaluación
El desarrollo del proyecto tendrá un seguimiento directo semanal por parte del docente a cargo del grupo.

Se deberá desarrollar y presentar los avances y progreso del trabajo semana a semana (simulando un sprint de trabajo). Cada semana, cada docente realizará una valoración del estado del trabajo del grupo.

El progreso de cada semana deberá ser acorde a lo que se convenga con el docente para cada sprint. Si el mismo NO cumple con la cantidad de trabajo requerido, el grupo podrá estar desaprobado de forma prematura de la materia, a consideración del docente.

Hacia la mitad del desarrollo del proyecto se deberá entregar una versión preliminar que deberá cumplir con los requisitos mencionados en el apartado *Entrega intermedia* anteriormente enunciado. Estos requisitos **son de cumplimiento mínimo y obligatorio**, aquellos grupos que lo deseen podrán implementar requisitos adicionales.

**Nota importante:** Se deja constancia que las funcionalidades requeridas por este enunciado son un marco de cumplimiento mínimo y que pueden haber agregados o modificaciones durante el transcurso del desarrollo por parte del docente a cargo, que formarán parte de los requerimientos a cumplir. Cabe mencionar que estos desvíos de los requerimientos iniciales se presentan en situaciones reales de trabajo con clientes.

## Finalización del Proyecto
El desarrollo del proyecto finaliza el último día de clases del cuatrimestre. En esa fecha, cada grupo deberá realizar una presentación final y se hará una evaluación global del trabajo.

En dicha presentación se deberá detallar la arquitectura del proyecto, aprendizajes del mismo, y realizar una muestra funcional del desarrollo, esto es una "demo" como si fuera para el usuario final.

El trabajo debe acompañarse por un informe que debe constar de los puntos detallados precedentemente, diagramas de secuencia de las operaciones más relevantes, diagrama de componentes y módulos de la arquitectura general del diseño desarrollado, todos acompañados de la explicación respectiva.

## Fechas de entrega

Entrega intermedia: Lunes 6 de Noviembre de 2023

Entrega final de la cursada:  Lunes 4 de Diciembre de 2023

**Estas entregas serán presenciales en la sede de la Facultad.**
