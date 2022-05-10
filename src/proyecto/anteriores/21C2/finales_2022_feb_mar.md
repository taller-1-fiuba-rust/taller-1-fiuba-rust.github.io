# Finales Febrero/Marzo 2022

Para la aprobación final de la materia se deberá implementar un agregado al enunciado del Trabajo Práctico y exponerlo en alguna de las fechas de final del llamado febrero/marzo 2022.

Las presentaciones serán de forma **remota** a través de Internet. Los grupos deben inscribirse en el SIU Guaraní para la fecha de final.

Las fechas de examen son:

**9508 // 7542 Taller de Programación I:**

- miércoles 9/2 a las 18 hs
- miércoles 16/2 a las 18 hs
- miércoles 23/2 a las 18 hs
- miércoles 2/3 a las 18 hs
- miércoles 9/3 a las 18 hs

## Enunciado

Se debe implementar el siguiente agregado al trabajo desarrollado:

**Parte A)** Implementar un programa que simule la generación de datos de un dispositivo (por ejemplo, la medición de la temperatura ambiente) cada determinado tiempo y lo publique como cliente al servidor MQTT utilizando un tópico predefinido. Este programa puede ser de consola, sin necesidad de interfaz gráfica, ni de interacción con el usuario.

Se permite utilizar el crate [rand](https://crates.io/crates/rand) para la generación de valores.

**Parte B)** Implementar un programa cliente MQTT que se suscriba al tópico anterior para recibir los datos y que actúe como **servidor HTTP** para publicar y exponer esos datos a través de una web accesible desde el browser. Es decir, este programa debe actuar como servidor HTTP.

Investigar para ello, los lineamientos del protocolo de comunicación HTTP y del formato HTML. No es necesario que la página mostrada se actualice automáticamente.

**Nota importante:** No se permite el uso de crates externos de frameworks HTTP. Se debe implementar la comunicación y el servidor a partir del uso de sockets TCP, como se ha trabajado en el desarrollo del curso.
 