# Logo to ASCII

Prueba de concepto para convertir una imagen en blanco y negro a ascii art (hecho en rust).

A diferencia de otros conversores de imágenes a ASCII, este no usa la luminosidad media de cada bloque, sino que elige el carácter que mejor se aproxima a su forma. No hace diferencia entre colores, solo mide la luminosidad que tiene cada pixel, por lo que funciona mejor con bordes bien definidos.

![Cruz de Calatrava](./images/cruz.png)

## Índice
- [Logo to ASCII](#logo-to-ascii)
  - [Índice](#índice)
  - [Instalación](#instalación)
  - [Uso](#uso)
  - [¿Cómo funciona?](#cómo-funciona)

## Instalación

1. Descarga rust desde https://www.rust-lang.org/tools/installs:
   1. Al terminar la instalación escribe `rustc --version` en la consola para comprobar que todo ha salido bien.
2. Descarga este repositorio.
3. Compllia el repositorio: En la consola de comandos ejecuta `cargo build`. 

## Uso

1. Ejecuta el programa desde la consola. Le tenemos que indicar la imagen que queremos usar, por lo que el comando quedaría así: `./target/debug/Logo_to_ASCII.exe --path <path_imagen>`.

Ese comando imprimirá el texto en la consola.

Otros casos a tener en cuenta:
* Para imprimir la imagen en negativo (imprimiendo donde está el color negro) se deberá añadir `-i` al comando.
* Para cambiar la fuente con la que se hace la comparación se puede usar el argumento `--font <path_fuente>.ttf`. Se tratará como una fuente monoespacio.
* Para guardar el texto en un documento de texto se puede añadir `> <path_archivo>.txt` al final del comando.

![Imagen procesada](./images/image.png)
Logo a ASCII

![Imagen procesada en negativo](./images/image-i.png)
Logo a ASCII en negativo

## ¿Cómo funciona?

La idea surgió de un video en el que se convertía una imagen a ASCII. Sin embargo, se perdía mucha información, los carácteres no tenían la forma que debían.

Este algoritmo opera con píxeles en vez de con bloques.

Primero se procesan los carácteres. En la consola tienen una proporción de 2 de alto por 1 de ancho. Se eligen las dimensiones 8x16 para hacer un mapa de bits de cada carácter. Este indica la luminosidad de cada pixel.

Al calcular la luminosidad se obtiene un valor de 0 a 1. Es importante restarle 0.5 para obtener valores negativos y positivos.

Después se procesa la imagen, dividiéndola en bloques de 8x16 (la misma medida que nuestros carácteres) y se calcula la luminosidad de cada uno de los pixeles (restándole también 0.5). 

Por cada carácter, se multiplica el valor de cada pixel con su homólogo en el bloque, y se suman todos los valores ([0][0] * [0][0] + [0][1] * [0][1] + ...). Al final, se imprime el carácter con la puntuación más alta.

El algoritmo funciona porque al multiplicar dos valores positivos se obtiene un númro positivo, y al multiplicar dos números negativos también. Esto premia las coincidencias de pixeles (y no píxeles) y penaliza las diferencias.