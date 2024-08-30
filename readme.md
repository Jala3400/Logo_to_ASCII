# Logo to ASCII art

Prueba de concepto para convertir una imagen en blanco y negro a ascii art (hecho en rust).

![Imagen procesada](./images/image.png)
Imagen a ASCII

![Imagen procesada en negativo](./images/image-i.png)
imagen a ASCII en negativo

# ¿Cómo se usa?

1. Descarga y compila el programa 
2. En la consola ejecuta el comando `./target/debug/Logo_to_ASCII.exe --path <path_imagen>`
3. Se escribirá en la consola el ascii art de la imagen (la parte que está en blanco. El transparente cuenta como negro)

El texto se imprimirá en la consola. Para guardar el resultado en un archivo, redirige la salida a un archivo con el comando `./target/debug/Logo_to_ASCII.exe --path <path_imagen> > <nombre_archivo>.txt`

Para dibujar la imagen en negativo, se puede añadir el flag `-i` al final del comando.

Cada carácter ocupa 8x16 píxeles de la imágen original. Con el flag `-w6` ocuparán 6x12. Se recomienda usar imágenes con anchuras divisibles entre estos números (6 u 8 dependiendo de cuál se use).

# ¿Cómo funciona?

Se procesa la imágen por bloques, comparándolos con cada carácter. Para cada carácter tenemos una matriz (lista de listas) que contiene 1 y -1 donde la letra tiene o no tiene un píxel. Estos se han puesto a mano, por lo que no encajan perfectamente, pero hacen el trabajo. 

Por cada píxel de la imágen se calcula su iluminación, entre 0 y 1, y se le resta 0.5, de modo que abarca un rango de -0.5 a 0.5. Lo único que importa es que la mitad de los píxeles tendrán valor negativo y la otra positivo.

Cada bloque se compara con todos los caracteres, y se elige el que más se parezca. Para evaluar cada carácter, se multiplica el valor de cada píxel con el valor del píxel correspondiente de la matriz del carácter, y se suman todos los valores. Se elige el carácter con la suma más alta.

De esta forma, el carácter que tenga más píxeles que coincidan con el bloque y menos donde el bloque no tenga, tendrá la suma más alta.
Se puede ver mejor así: + · + = + | - · - = + | - · + = -
