# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**Pyxel** es un motor de videojuegos para Python.

Gracias a sus características simples inspiradas en las consolas de juegos retro, como el solo mostrar 16 colores y el reproducir 4 sonidos al mismo tiempo, puedes sentirte libre de disfrutar creando juegos en estilo pixel art.

<a href="../pyxel/examples/01_hello_pyxel.py" target="_blank">
<img src="images/01_hello_pyxel.gif" width=48%>
</a>
<a href="../pyxel/examples/02_jump_game.py" target="_blank">
<img src="images/02_jump_game.gif" width=48%>
</a>

<a href="../pyxel/examples/03_draw_api.py" target="_blank">
<img src="images/03_draw_api.gif" width=48%>
</a>
<a href="../pyxel/examples/04_sound_api.py" target="_blank">
<img src="images/04_sound_api.gif" width=48%>
</a>

<a href="images/image_tilemap_editor.gif" target="_blank">
<img src="images/image_tilemap_editor.gif" width=48%>
</a>
<a href="images/sound_music_editor.gif" target="_blank">
<img src="images/sound_music_editor.gif" width=48%>
</a>

Las especificaciones y API de Pyxel se inspiran en [PICO-8](https://www.lexaloffle.com/pico-8.php) y [TIC-80](https://tic80.com/).

Pyxel es de código abierto y gratuito. ¡Empecemos haciendo un juego retro con Pyxel!

## Características

- Se ejecuta en Windows, Mac(Intel y M1) y Linux
- Programación con Python
- Paleta de 16 colores
- 3 bancos de imágenes de 256x256
- 8 mapas de 256x256
- 4 canales con 64 posibles definiciones de sonido
- 8 músicas que pueden combinar sonidos arbitrarios
- Entrada de teclado, ratón y gamepad.
- Editor de imágenes y sonido

### Paleta de colores

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Como instalarlo

Hay dos tipos de Pyxel, una versión como paquete y una versión independiente.

### Instalar la versión empaquetada

La versión empaquetada de Pyxel utiliza Pyxel como un módulo de extensión de Python.

Es lo recomendado para aquellos que estén familiarizados con el manejo de paquetes en Python mediante el comando `pip` o para los que quieran desarrollar una aplicación completa.

**Windows**

Después de instalar [Python3](https://www.python.org/) (versión 3.7 o superior), ejecute el siguiente comando:

```sh
pip install -U pyxel
```

**Mac**

Después de instalar [Python3](https://www.python.org/) (versión 3.7 o superior), ejecute el siguiente comando:

```sh
pip3 install -U pyxel
```

**Linux**

Después de instalar los paquetes SDL2 (`libsdl2-dev` para Ubuntu), [Python3](https://www.python.org/) (versión 3.7 or superior), y `python3-pip`, ejecute el siguiente comando:

```sh
sudo pip3 install -U pyxel
```

Si los pasos mencionados arriba no funcionan, intente compilarla usted mismo, siguiendo los siguientes comandos después de instalar `cmake` and `rust`:

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make clean all
sudo pip3 install .
```

### Instalar la versión independiente

La versión independiente de Pyxel utiliza Pyxel como una herramienta independiente de Python.

Esta instalación es la recomendada para aquellos que quieran empezar a programar sin tener que preocuparse de los ajustes de Python o para aquellos que quieran jugar a juegos de Pyxel inmediatamente.

**Windows**

Descarga y ejecuta la última versión de (`pyxel-[version]-windows-setup.exe`) desde la [Página de Descargas](https://github.com/kitao/pyxel/releases).

**Mac**

Después de instalar [Homebrew](https://brew.sh/), ejecuta el siguiente comando:

```sh
brew tap kitao/pyxel
brew install pyxel
```

**Linux**

Después de instalar el paquete SDL2, (`libsdl2-dev` para Ubuntu) e instalar [Homebrew](https://brew.sh/), ejecuta los siguientes comandos:

```sh
brew tap kitao/pyxel
brew install pyxel
```
Si los pasos mencionados previamente no funcionan, intenta construir / compilar tú la versión empaquetada.

### Prueba los ejemplos de Pyxel

Después de instalar Pyxel, los ejemplos de Pyxel se copiarán a la carpeta actual con el siguiente comando:

```sh
pyxel copy_examples
```
Los ejemplos serán copiados de la siguiente manera:

- [01_hello_pyxel.py](../pyxel/examples/01_hello_pyxel.py) - Aplicación simple
- [02_jump_game.py](../pyxel/examples/02_jump_game.py) - Juego de plataformas con los archivos de recursos Pyxel
- [03_draw_api.py](../pyxel/examples/03_draw_api.py) - Demostración de la API para dibujar
- [04_sound_api.py](../pyxel/examples/04_sound_api.py) - Demostración de la API de sonidos
- [05_color_palette.py](../pyxel/examples/05_color_palette.py) - Lista de la paleta de colores
- [06_click_game.py](../pyxel/examples/06_click_game.py) - Juego con clics del ratón
- [07_snake.py](../pyxel/examples/07_snake.py) - Juego de serpiente con música de fondo
- [08_triangle_api.py](../pyxel/examples/08_triangle_api.py) - Demostración de dibujo de triángulos con la API
- [09_shooter.py](../pyxel/examples/09_shooter.py) - Juego de nave espacial con transiciones de pantalla
- [10_platformer.py](../pyxel/examples/10_platformer.py) - Juego con desplazamiento lateral con plataformas con mapa
- [11_offscreen.py](../pyxel/examples/11_offscreen.py) - Rendimiento fuera de la pantalla con la clase Image
- [12_perlin_noise.py](../pyxel/examples/12_perlin_noise.py) - Animación del ruido Perlin
- [30SecondsOfDaylight.pyxapp](images/30SecondsOfDaylight.gif) - El primer ganador del Jam de Pyxel, por [Adam](https://twitter.com/helpcomputer0)
- [megaball.pyxapp](images/megaball.gif) - Juego arcade de física de bolas, por [Adam](https://twitter.com/helpcomputer0)

Los ejemplos se pueden ejecutar con el siguiente comando:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30SecondsOfDaylight.pyxapp
```

## Como utilizarlo

### Crear una aplicación Pyxel

Tras importar el módulo Pyxel en el código de Python, especifique la dimensión de la ventana con la función `init`, luego, inicie la aplicación con la función `run`.

```python
import pyxel

pyxel.init(160, 120)

def update():
    if pyxel.btnp(pyxel.KEY_Q):
        pyxel.quit()

def draw():
    pyxel.cls(0)
    pyxel.rect(10, 10, 20, 20, 11)

pyxel.run(update, draw)
```

Los argumentos de la función `run` son la función `update` para actualizar cada fotograma y la función `draw` para dibujar la pantalla cuando sea necesario.

En una aplicación, es recomendable envolver el código de pyxel en una clase como la siguiente:

```python
import pyxel

class App:
    def __init__(self):
        pyxel.init(160, 120)
        self.x = 0
        pyxel.run(self.update, self.draw)

    def update(self):
        self.x = (self.x + 1) % pyxel.width

    def draw(self):
        pyxel.cls(0)
        pyxel.rect(self.x, 0, 8, 8, 9)

App()
```

También es posible escribir código simple utilizando la función `show` y la función `flip`, para dibujar gráficos simples y animaciones o la función `show` muestra en pantalla hasta que se pulse la tecla de `Esc`.


```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```
La función `flip` actualiza la pantalla una sola vez.

```python
import pyxel

pyxel.init(120, 80)

while True:
    pyxel.cls(3)
    pyxel.rectb(pyxel.frame_count % 160 - 40, 20, 40, 40, 7)
    pyxel.flip()
```

### Corre aplicaciones Pyxel

El código generado puede ser ejecutado con el siguiente comando:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Para la versión empaquetada, se puede ejecutar como un script normal de Python:

```sh
cd pyxel_examples
python3 PYTHON_SCRIPT_FILE
```

(Para Windows, escriba `python` en vez de `python3`)

### Controles Especiales

Los siguientes controles especiales se pueden utilizar en cualquier aplicación que esté corriendo:

- `Esc`<br>
Salir de la aplicación
- `Alt(Option)+1`<br>
Guardar la captura de pantalla al escritorio
- `Alt(Option)+2`<br>
Restablecer el momento de inicio de la grabación del vídeo de captura de pantalla
- `Alt(Option)+3`<br>
Guarda la captura de video en el escritorio (hasta 10 segundos)
- `Alt(Option)+0`<br>
Activa el monitor de monitorización (fps, el tiempo que tarda en actualizar la pantalla y el tiempo que tarda en dibujar)
- `Alt(Option)+Enter`<br>
Activar el modo de pantalla completa

### Como crear los recursos

El Editor de Pyxel crea imágenes y sonidos que se utilizan en la aplicación de Pyxel.

Se ejecuta con el siguiente comando:

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

Si el archivo de recursos de Pyxel existe, el archivo se carga y si no existe, se crea con el nombre especificado.
Si el archivo de recursos es omitido, el nombre de este será `my_resource.pyxres`.

Tras iniciar el Editor de Pyxel, el archivo puede ser cambiado arrastrando y soltando otro archivo de recursos. Si el archivo se arrastra y se tiene pulsada la tecla ``Ctrl(Cmd)``, solo el recurso concreto se cargará (Imagen/Mapa/Sonido/Música) que se está editando actualmente. Esta operación permite combinar multiples archivos de recursos en uno solo.

Dicho archivo de recursos podrá see cargado con la función ``load``.

EL Editor de Pyxel tiene los siguientes modos de edición:

**Editor de imágenes:**

El modo para editar el banco de imágenes.

<img src="doc/images/image_editor.gif">

Arrastrando y soltando un archivo de imagen (png/jpg/gif) en el editor de imágenes, la imagen será cargada en el banco de imágenes seleccionado.

**Editor de mapa / losas:**

El modo para editar el mapa de losas o mapa en el que las imágenes en el banco de imágenes están organizados en un patrón de azulejos o baldosas

<img src="doc/images/tilemap_editor.gif">

**Editor de sonido:**

El modo para editar el sonido.

<img src="doc/images/sound_editor.gif">

**Editor de música:**

El modo para editar música en el que organiza los sonidos del editor de sonidos para poder reproducirlos.

<img src="doc/images/music_editor.gif">

### Otros recursos en la creación de métodos

Las imágenes de Pyxel y el mapa también se pueden crear de las siguientes maneras:

- Crea una imagen de una lista de strings con la función `Image.set` o la función `Tilemap.set`
- Carga un archivo de imagen (png/gif/jpeg) en la paleta de Pyxel con la función `Image.load`

Los sonidos de Pyxel también se pueden crear ude la siguiente manera:

- Crear el sonido desde strings con las funciones `Sounds.set` o `Music.set`

Por favor, consulte la API para el uso de estas funciones.

### Como distribuir tu aplicación

Pyxel soporta un archivo dedicado para distribuir el código (formato de aplicación de Pyxel) que funciona en todas las plataformas.

Cree la aplicación (.pyxapp) con el siguiente comando:

```sh
pyxel package APP_ROOT_DIR STARTUP_SCRIPT_FILE
```

Si la aplicación debe incluir recursos o algunos módulos adicionales, colóquelo en la carpeta de la aplicación.

La aplicación creada se puede ejecutar con el siguiente comando:

```sh
pyxel play PYXEL_APP_FILE
```

##  Referencias de la API

### Sistema

- `width`, `height`<br>
La anchura y la altura de la pantalla

- `frame_count`<br>
El número de fotogramas que han pasado

- `init(width, height, [title], [fps], [quit_key], [capture_scale], [capture_sec])`<br>
Inicializa la aplicación de Pyxel con el tamaño (`width`, `height`). Los siguientes parámetros pueden ser especificados como opciones: el título con `title`, el ratio de fotogramas por segundo con `fps`, la tecla para salir de la aplicación con `quit_key`, la escala de captura de pantalla con `capture_scale` y el tiempo máximo para grabar la pantalla con `capture_sec`. <br>
por ejemplo: `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
Comienza la aplicación de Pyxel y llama la función `update` para actualizar cada fotograma y la función `draw` para dibujar.

- `show()`<br>
Muestra la pantalla y espera hasta que la tecla `Esc` sea pulsada. (No lo utilice en aplicaciones normales)

- `flip()`<br>
Actualiza la pantalla una vez (No lo utilice en aplicaciones normales)

- `quit()`<br>
Salir de la aplicación

### Recursos

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Carga el archivo de recursos (.pyxres). Si ``False`` es especificado para los recursos (``image/tilemap/sound/music``), los recursos no serán cargados.

### Entrada
- `mouse_x`, `mouse_y`<br>
La posición del cursor

- `mouse_wheel`<br>
EL valor actual de la rueda del ratón

- `btn(key)`<br>
Devuelve True si key es presionada, si no devuelve False (lista de definición de teclas)

- `btnp(key, [hold], [repeat])`<br>
Devuelve `True` si `key` es presionada en ese cuadro, si no devuelve `False`. Cuando `hold` y `repeat` son definidos, devuelve `True` en el intervalo de cuadro `repeat` cuando `key` es sostenida por más cuadros que el valor `hold`.
- `btnr(key)`<br>
Devuelve `True` si se suelta la tecla `key` en ese frame, si no, devuelve `False`.

- `mouse(visible)`<br>
Si `visible` es `True`, muestra el cursor del ratón. Si es `False`, no lo muestra. Incluso si el cursor no se muestra, su posición se actualiza.

### Gráficos

- `colors`<br>
Lista de la paleta de colores que se pueden representar. El color del display se especifica con un valor numérico de 24 bits. Utiliza `colors.from_list` y `colors.to_list` para directamente asignar y leer una lista de Python.
Ejemplo: `org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img)`<br>
Opera la imagen del banco de imágenes `img`(0-2). (Vea la clase Image)<br>
Ejemplo: `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
Opera el mapa `tm` (0-7). (Vea la clase Tilemap)

- `clip(x, y, w, h)`<br>
Establezca el área de dibujo de la pantalla de (`x`, `y`) a una anchura `w` y a una altura `h`. Reinicia el área de dibujo a todo el área de la pantalla con `clip()`.

- `camera(x, y)`<br>
Cambie las coordenadas de la esquina superior izquierda de la pantalla a (`x`,` y`). Restablezca las coordenadas de la esquina superior izquierda a (`0`,` 0`) con `camera()`.

- `pal(col1, col2)`<br>
Reemplaza el color `col1` con `col2` para dibujarlo. Utiliza `pal()` para resetear la paleta de colores y volver a la paleta que viene por defecto por defecto con Pyxel.

- `cls(col)`<br>
Borra la pantalla con el color `col`.

- `pget(x, y)`<br>
Obtiene el color del pixel en la posición (`x`, `y`).

- `pset(x, y, col)`<br>
Dibuja un pixel del color `col` en la posición (`x`, `y`).

- `line(x1, y1, x2, y2, col)`<br>
Dibuja una línea del color `col` desde (`x1`, `y1`) a (`x2`, `y2`).

- `rect(x, y, w, h, col)`<br>
Dibuja un rectángulo de anchura `w`, altura `h` y color `col` desde la posición (`x`, `y`).

- `rectb(x, y, w, h, col)`<br>
Dibuja el perímetro de un rectángulo de anchura `w`, altura `h` y color `col` desde la posición (`x`, `y`).

- `circ(x, y, r, col)`<br>
Dibuja un círculo de radio `r` y color `col` en (`x`, `y`).

- `circb(x, y, r, col)`<br>
Dibuja una circunferencia de radio `r` y color `col` en (`x`, `y`).

- `elli(x, y, w, h, col)`<br>
Dibuja una elipse de anchura `w`, altura `h` y color `col` desde (`x`, `y`).

- `ellib(x, y, w, h, col)`<br>
Dibuja el contorno de una elipse de anchura `w`, altura `h` y color `col` desde (`x`, `y`).

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
Dibuja un triángulo con los vertices (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) y color `col`.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
Dibuja el perímetro de un triángulo con los vertices (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) y color `col`.

- `fill(x, y, col)`<br>
Dibuja una elipsis de anchura `w`, altura `h` y color `col` desde (`x`, `y`).

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
Copia la región de tamaño (`w`, `h`) desde la posición (`u`, `v`) del banco de imágenes `img` (0-2) a (`x`, `y`). Si es negativo el valor para la `w` y/o la `h`, se representará invirtiendo horizontalmente o verticalmente. Si `colkey` se especifica, se tratará ese color como transparente.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Copie la región de tamaño (`w`,` h`) de (`u`,` v`) del mapa de mosaicos `tm` (0-7) a (` x`, `y`). Si se establece un valor negativo para `w` y / o` h`, se invertirá horizontal y / o verticalmente. Si se especifica "colkey", se trata como un color transparente. El tamaño de un mosaico es de 8x8 píxeles y se almacena en un mapa de mosaicos como una tupla de `(tile_x, tile_y)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
Dibuja un string `s` del color`col` en (`x`, `y`).

### Audio

- `sound(snd)`<br>
Opera el sonido `snd` (0-63). (Vea la clase Sound)<br>
Ejemplo: `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Reproduce la música  `msc` (0-7). (Vea la clase Music)

- `play_pos(ch)`<br>
Obtenga la posición de la reproducción de la música de un canal  `ch` (0-3) como la tupla `(sound no, note no)`, no quiere decir número. Devuelve `None` cuando la música cesa.

- `play(ch, snd, [tick], [loop])`<br>
Reproduce el sonido `snd` (0-63) en el canal `ch` (0-3). Si el `snd` está en una lista, será reproducido en orden. La posición de inicio de la reproducción se puede especificar mediante un `tick` (1 tick = 1/120 segundos). Si `True` se especifica para `loop`, la reproducción se realizará en bucle.

- `playm(msc, [tick], [loop])`<br>
Reproduce la música `msc` (0-7). La posición de inicio de la reproducción se puede especificar mediante un `tick` (1 tick = 1/120 segundos). Si `True` se especifica para `loop`, la reproducción en bucle tendrá lugar.

- `stop([ch])`<br>
Para la reproducción del canal `ch` (0-3). `stop()` para detener todos los canales

### Matemáticas

- `ceil(x)`<br>
Devuelve el menor número entero mayor o igual a `x`.

- `floor(x)`<br>
Devuelve el mayor entero menor o igual a `x`.

- `sgn(x)`<br>
Devuelve 1 cuando `x` es positivo, 0 cuando es cero y -1 cuando es negativo.

- `sqrt(x)`<br>
Devuelve la raíz cuadrada de `x`.

- `sin(deg)`<br>
Devuelve el seno de `deg` grados.

- `cos(deg)`<br>
Devuelve el coseno de `deg` grados.

- `atan2(y, x)`<br>
Devuelve la arctangente de `y`/`x` en grados.

- `rseed(seed: int)`<br>
Establece la semilla del generador de números aleatorios.

- `rndi(a, b)`<br>
Devuelve un número entero aleatorio mayor o igual que `a` y menor o igual que `b`.

- `rndf(a, b)`<br>
Devuelve un decimal aleatorio mayor o igual que `a` y menor o igual que `b`.

- `nseed(seed)`<br>
Establece la semilla de ruido Perlin.

- `noise(x, [y], [z])`<br>
Devuelve el valor del ruido Perlin para las coordenadas especificadas.

### Clase Image

- `width`, `height`<br>
La anchura y la altura de una imagen

- `set(x, y, data)`<br>
Define la imagen en (`x`, `y`) por una lista de strings. <br>
Ejemplo: `pyxel.image(0).set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
Carga el archivo con la imagen (png/gif/jpeg) en (`x`, `y`)

- `pget(x, y)`<br>
Obtén el color del pyxel en la posición (`x`, `y`).

- `pset(x, y, col)`<br>
Dibuja un pixel del color `col` en las coordenadas (`x`, `y`).

### Clase Tilemap

- `width`, `height`<br>
La anchura y la altura del mapa

- `refimg`<br>
El banco de imágenes (0-2) que referencia el mapa

- `set(x, y, data)`<br>
Establece el mapa a (`x`, `y`) por una lista de strings<br>
Ejemplo: `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `pget(x, y)`<br>
Obtén la celda del mapa de la posición (`x`, `y`). Una celda es una tupla formada por `(tile_x, tile_y)`

- `pset(x, y, tile)`<br>
Dibuja una `tile` en (`x`, `y`). Una celda es una tupla formada por `(tile_x, tile_y)`

### Clase Sound

- `notes`<br>
Lista de notas (0-127). Cuanto mayor sea el número, mayor será el pitch (más agudo) y a 33, se convierte en la nota 'A2' (440 Hz). El resto es -1

- `tones`<br>
Lista de tonos(0:Triangular / 1:Cuadrada / 2:Pulsada / 3:Ruido)

- `volumes`<br>
- Lista de volúmenes (0-7)

- `effects`<br>
Lista de efectos de sonido (0:Nada / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`<br>
La velocidad de reproducción, 1 es la más rápida y al incrementar este número, la velocidad de reproducción disminuye. Cuando vale 120, la longitud de una nota es de 1 segundo.

- `set(notes, tones, volumes, effects, speed)`<br>
Fija las notas, los tonos, el volumen y los efectos con una string. Si los tonos, el volumen, la longitud de los efectos son más cortos que la nota, se repetirá desde el principio.

- `set_notes(notes)`<br>
Fija las notas con un string hecho por 'CDEFGAB'+'#-'+'0123' o 'R'. Sensible a las mayúsculas y minúsculas y los espacios en blanco serán ignorados.<br>
Ejemplo: `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
Fija las notas con un string hecho con 'TSPN'. Sensible a las mayúsculas y minúsculas y los espacios en blanco serán ignorados.<br>
Ejemplo: `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volumes(volumes)`<br>
Fija el volumen con una string hecha de '01234567'. Sensible a las mayúsculas y minúsculas y los espacios en blanco serán ignorados.<br>
Ejemplo: `pyxel.sound(0).set_volume("7777 7531")`

- `set_effects(effects)`<br>
Fija los efectos con una string hecha con 'NSVF'. Sensible a las mayúsculas y minúsculas y los espacios en blanco serán ignorados.<br>
Ejemplo: `pyxel.sound(0).set_effect("NFNF NVVS")`

### Clase Music

- `snds_list`<br>
Lista bidimensional de sonidos (0-63) con el número de canales.

- `set(snds0, snds1, snds2, snds3)`<br>
Fija la lista de sonidos (0-63) de todos los canales. Si se referencia una lista vacía, ese canal no se utilizará para reproducir el sonido.<br>
Ejemplo: `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### APIs avanzadas

Pyxel tiene "API avanzadas" que no se mencionan en esta referencia porque "puede confundir a los usuarios" o "necesita unos conocimientos especializados para poder utilizarlas".

Si estás familiarizado con tus habilidades. ¡Intenta crear proyectos alucinantes con [esto](../pyxel/__init__.pyi) como pista!

## Como contribuir

### Presentar un problema

Usa el [Rastreador de problemas](https://github.com/kitao/pyxel/issues) para enviar errores y solicitudes de mejora. Antes de presentar un nuevo tema, asegúrese de que no existe uno ya abierto similar.

### Pruebas Manuales

Cualquier persona que compruebe y prueba manualmente el código y reporte errores o sugerencias para mejorar el código en el [Issue Tracker](https://github.com/kitao/pyxel/issues) es bienvenida! <br>

### Envio de Pull Request

Parches o errores son aceptables en forma de pull request (PRs). Asegurate de que el tema de la pull request esté abierta en el Issue Tracker

Los pull request enviados se consideran acordados para poder publicarse bajo la [Licencia MIT](../LICENSE).

## Otra Información

- [Q&A](https://github.com/kitao/pyxel/wiki/Pyxel-Q&A)
- [Ejemplos de usuarios](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Discord Server (English)](https://discord.gg/Z87eYHN)
- [Discord Server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## Licencia

Pyxel esta bajo la [Licencia MIT](../LICENSE). Puede ser reutilizada con el software propietario, siempre y cuando todas las copias del software o sus substanciales porciones del mismo incluyan una copia de los términos de la Licencia MIT y también un aviso de copyright.

## Reclutamiento de patrocinadores

Pyxel está buscando patrocinadores en GitHub Sponsors. Considere patrocinar Pyxel para un mantenimiento continuo y adiciones de funciones. Los patrocinadores pueden consultar sobre Pyxel como un beneficio. Consulte [aquí](https://github.com/sponsors/kitao) para obtener más detalles.
