# <img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/assets/pyxel_logo_152x64.png">

[ [English](https://github.com/kitao/pyxel/blob/master/README.md) | [日本語](https://github.com/kitao/pyxel/blob/master/README.ja.md) | [中文](https://github.com/kitao/pyxel/blob/master/README.cn.md) | [한국어](https://github.com/kitao/pyxel/blob/master/README.ko.md) ]

**Pyxel** es un motor de videojuegos para Python.

Gracias a sus especificaciones simples inspiradas por las consolas de juegos retro, como el solo mostrar 16 colores y el solo reproducir 4 sonidos al mismo tiempo, puedes sentirte libre de disfrutar creando juegos con estilo pixel art.

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/01_hello_pyxel.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/01_hello_pyxel.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/02_jump_game.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/02_jump_game.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/03_draw_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/03_draw_api.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/04_sound_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/04_sound_api.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/editor/screenshots/image_tilemap_editor.gif" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/image_tilemap_editor.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/editor/screenshots/sound_music_editor.gif" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/sound_music_editor.gif" width="48%">
</a>

Las especificaciones de la consola de juego, APIs, y paletas de Pyxel son referencia de el increíble [PICO-8](https://www.lexaloffle.com/pico-8.php) y [TIC-80](https://tic.computer/).

Pyxel es de codigo abierto y de uso libre. Comencemos a crear juegos retro con Pyxel!

## Especificaciones

- Se ejecuta en Windows, Mac y Linux
- Escritura de código con Python3
- Paleta fija de 16 colores
- 3 bancos de imagenes de tamaño 256x256
- 8 tilemaps de tamaño 256x256
- 4 canales con 64 sonidos definibles
- 8 musicas que pueden combinar sonidos arbitrarios
- Entradas de teclado, mouse, y gamepad
- Editor de imagen y sonido

### Paleta de Colores

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/05_color_palette.png">

## Cómo instalar

### Windows

Despues de instalar [Python3](https://www.python.org/) (versión 3.7 o superior), los siguientes comandos `pip` instalan Pyxel:

```sh
pip install -U pyxel
```

### Mac

Despues de instalar [Python3](https://www.python.org/) (versión 3.7 superior) y [SDL2](https://www.libsdl.org/), instalar Pyxel con el comando `pip`.

Si el gestor de paquetes [Homebrew](https://brew.sh/) esta listo, el siguiente comando instala todos los paquetes necesarios:

```sh
brew install python3 sdl2 sdl2_image
pip3 install -U pyxel
```

### Linux

Instala [Python3](https://www.python.org/) (version 3.7 o superior) y los paquetes requeridos en la forma apropiada para cada distribución.

**Ubuntu:**

```sh
sudo apt install python3 python3-pip libsdl2-dev libsdl2-image-dev
sudo pip3 install -U pyxel
```

### Otros entornos

Para instalar Pyxel en un entorno diferente a los anteriores (Linux 32-bit, Raspberry PI, etc.), sigue los pasos descritos debajo para construirlo:

#### Instalar las herramientas y paquetes necesarios

- C++ build toolchain (should include gcc and make command)
- Cadena de herramientas de construcción C++ (debe incluir los comandos `gcc` y `make`)
- libsdl2-dev y libsdl2-image-dev
- [Python3](https://www.python.org/) (version 3.7 o superior) y el comando `pip`

#### Ejecuta el siguiente comando en cualquier carpeta

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make -C pyxel/core clean all
pip3 install .
```

### Instalar ejemplos

After installing Pyxel, the examples of Pyxel will be copied to the current directory with the following command:
Despuest de instalar Pyxel, los ejemplos de Pyxel serán copiados al directorio actual con el siguiente comando:

```sh
install_pyxel_examples
```

Los ejemplos a ser copiados son los siguientes:

- [01_hello_pyxel.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/01_hello_pyxel.py) - Aplicación mas simple
- [02_jump_game.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/02_jump_game.py) - Juego de saltos con el archivo de recursos Pyxel
- [03_draw_api.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/03_draw_api.py) - Demonstración del API de dibujado
- [04_sound_api.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/04_sound_api.py) - Demonstración del API de sonido
- [05_color_palette.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/05_color_palette.py) - Lista de paleta de colores
- [06_click_game.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/06_click_game.py) - Juego de click con el Mouse
- [07_snake.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/07_snake.py) - Juego de serpiente con música de fondo

Los ejemplos pueden ser ejecutados como código Python normal:

**Windows:**

```sh
cd pyxel_examples
python 01_hello_pyxel.py
```

**Mac / Linux:**

```sh
cd pyxel_examples
python3 01_hello_pyxel.py
```

## Cómo usarlo

### Crear una aplicación Pyxel

Luego de importar el modulo Pyxel en tu código Python, primero especifíca el tamaño de la pantalla con la función `init`, luego inicia la aplicación Pyxel con la func `run`.

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

Los argumentos de la función `run` son la función `update` para actualizar cada cuadro y la función `draw` para dibujar la pantalla cuando es necesario.

En una aplicación actual, es recomendable envolver el código Pyxel en una clase como a continuación:

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

También es posible escribir código simple utilizando las funciones `show` y `flip` para dibujar graficos y animaciones simples.

La función `show` muestra la pantalla y espera hasta que la tecla `ESC` es pulsada.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

La función `flip` actualiza la pantalla una vez.

```python
import pyxel

pyxel.init(120, 80)

while True:
    pyxel.cls(3)
    pyxel.rectb(pyxel.frame_count % 160 - 40, 20, 40, 40, 7)
    pyxel.flip()
```

### Controles especiales

Los siguientes controles especiales pueden ser usados cuando una aplicación Pyxel se esta ejecutando:

- `Esc`<br>
Sale de la aplicación
- `Alt(Option)+1`<br>
Guarda la captura de pantalla en el escritorio
- `Alt(Option)+2`<br>
Reinicia el tiempo inicial de grabación de la captura de video de pantalla
- `Alt(Option)+3`<br>
Guarda la captura de video de pantalla (gif) en el escritorio (máximo 30 segundos)
- `Alt(Option)+0`<br>
Intercambia el monitor de rendimiento (fps, tiempo de actualización, y tiempo de dibujado)
- `Alt(Option)+Enter`<br>
Intercambia la pantalla completa

### Cómo crear un recurso

El Editor Pyxel adjunto puede crear imagenes y sonidos usados en la aplicación Pyxel.

El Editor Pyxel inicia con el siguiente comando:

```sh
pyxeleditor [pyxel_resource_file]
```

Si el archivo de recurso Pyxel especificado (.pyxres) existe, el archivo es cargado, y sino existe, un nuevo archivo es creado con el nombre especificado.
Si el archivo de recurso es omitido, el nombre es `my_resource.pyxres`.

Luego de iniciar el Editor Pyxel, el archivo puede ser cambiado arrastrando y soltando otro archivo de recurso.

El archivo de recurso creado puede ser cargado con la función `load`.

El Editor Pyxel Editor tiene los siguientes modos de edición.

**Editor de Imagenes:**

El modo para editar el banco de imagenes.

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/image_editor.gif">

Arrastrando y soltando un archivo png en la pantalla del Editor de Imagenes, la imagen puede ser cargada en el banco de imagenes actualmente seleccionado.

**Editor de Tilemap:**

El modo para editar tilemaps en el que imagenes del banco de imagenes son organizados en un patrón de tiles.

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/tilemap_editor.gif">

**Editor de Sonido:**

El modo para editar sonidos.

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/sound_editor.gif">

**Editor de Música:**

The mode to edit musics in which the sounds are arranged in order of playback.
El modo para editar música en el que los sonidos son organizados en orden de reproducción.

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/music_editor.gif">

### Other resource creation methods

Pyxel images and tilemaps can also be created in the following way:

- Create an image from a list of strings with `Image.set` or `Tilemap.set` function
- Load a png file in Pyxel palette with `Image.load` function

Because Pyxel uses the same palette as [PICO-8](https://www.lexaloffle.com/pico-8.php), when creating png images for Pyxel, it is recommended to use [Aseprite](https://www.aseprite.org/) in PICO-8 palette mode.

Pyxel sounds can also be created in the following way:

- Create a sound from strings with `Sound.set` or `Music.set` function

Please refer to the API reference for usage of these functions.

### How to Create a Stand-Alone Executable

By using the attached Pyxel Packager, a stand-alone executable that will work even in environments where Python is not installed can be created.

To create a stand-alone executable, specify the Python file to be used to launch the application with the `pyxelpackager` command as follows:

```sh
pyxelpackager python_file
```

When the process is complete, a stand-alone executable is created in the `dist` folder.

If resources such as .pyxres and .png files are also necessary, put them under the `assets` folder and they will be included.

It is also possible to specify an icon with the ``-i icon_file`` option.

## API Reference

### System

- `width`, `height`<br>
The width and height of the screen

- `frame_count`<br>
The number of the elapsed frames

- `init(width, height, [caption], [scale], [palette], [fps], [border_width], [border_color])`<br>
Initialize the Pyxel application with screen size (`width`, `height`). The maximum width and height of the screen is 256<br>
It is also possible to specify the window title with `caption`, the display magnification with `scale`, the palette color with `palette`, the frame rate with `fps`, and the margin width and color outside the screen with `border_width` and `border_color`. `palette` is specified as a list of 16 elements of 24 bit color, `border_color` as 24 bit color

- `run(update, draw)`<br>
Start the Pyxel application and call `update` function for frame update and `draw` function for drawing

- `quit()`<br>
Quit the Pyxel application at the end of the current frame

- `flip()`<br>
Force drawing the screen (do not use in normal applications)

- `show()`<br>
Draw the screen and wait forever (do not use in normal applications)

### Resource

- `save(filename)`<br>
Save the resource file (.pyxres) to the directory of the execution script

- `load(filename)`<br>
Read the resource file (.pyxres) from the directory of the execution script

### Input
- `mouse_x`, `mouse_y`<br>
The current position of the mouse cursor

- `btn(key)`<br>
Return `True` if `key` is pressed, otherwise return `False` ([key definition list](https://github.com/kitao/pyxel/blob/master/pyxel/__init__.py))

- `btnp(key, [hold], [period])`<br>
Return `True` if `key` is pressed at that frame, otherwise return `False`. When `hold` and `period` are specified, `True` will be returned at the `period` frame interval when the `key` is held down for more than `hold` frames

- `btnr(key)`<br>
Return `True` if `key` is released at that frame, otherwise return `False`

- `mouse(visible)`<br>
If `visible` is `True`, show the mouse cursor. If `False`, hide it. Even if the mouse cursor is not displayed, its position is updated.

### Graphics

- `image(img, [system])`<br>
Operate the image bank `img`(0-2) (see the Image class). If `system` is `True`, the image bank for system can be accessed. 3 is for the font and resource editor. 4 is for the display screen<br>
e.g. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
Operate the tilemap `tm`(0-7) (see the Tilemap class)

- `clip(x, y, w, h)`<br>
Set the drawing area of the screen from (`x`, `y`) to width `w` and height `h`. Reset the drawing area to full screen with `clip()`

- `pal(col1, col2)`<br>
Replace color `col1` with `col2` at drawing. `pal()` to reset to the initial palette

- `cls(col)`<br>
Clear screen with color `col`

- `pix(x, y, col)`<br>
Draw a pixel of color `col` at (`x`, `y`)

- `line(x1, y1, x2, y2, col)`<br>
Draw a line of color `col` from (`x1`, `y1`) to (`x2`, `y2`)

- `rect(x, y, w, h, col)`<br>
Draw a rectangle of width `w`, height `h` and color `col` from (`x`, `y`)

- `rectb(x, y, w, h, col)`<br>
Draw the outline of a rectangle of width `w`, height `h` and color `col` from (`x`, `y`)

- `circ(x, y, r, col)`<br>
Draw a circle of radius `r` and color `col` at (`x`, `y`)

- `circb(x, y, r, col)`<br>
Draw the outline of a circle of radius `r` and color `col` at (`x`, `y`)

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
Copy the region of size (`w`, `h`) from (`u`, `v`) of the image bank `img`(0-2) to (`x`, `y`). If negative value is set for `w` and/or `h`, it will reverse horizontally and/or vertically. If `colkey` is specified, treated as transparent color

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Draw the tilemap `tm`(0-7) to (`x`, `y`) according to the tile information of size (`w`, `h`) from (`u`, `v`). If `colkey` is specified, treated as transparent color. A tile of the tilemap is drawn with a size of 8x8, and if the tile number is 0, indicates the region (0, 0)-(7, 7) of the image bank, if 1, indicates (8, 0)-(15, 0)

- `text(x, y, s, col)`<br>
Draw a string `s` of color `col` at (`x`, `y`)

### Audio

- `sound(snd, [system])`<br>
Operate the sound `snd`(0-63) (see the Sound class). If `system` is `True`, the sound 64 for system can be accessed<br>
e.g. `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Operate the music `msc`(0-7) (see the Music class)

- `play_pos(ch)`<br>
Get the sound playback position of channel `ch`. The 100's and 1000's indicate the sound number and the 1's and 10's indicate the note number. When playback is stopped, return `-1`

- `play(ch, snd, loop=False)`<br>
Play the sound `snd`(0-63) on channel `ch`(0-3). Play in order when `snd` is a list

- `playm(msc, loop=False)`<br>
Play the music `msc`(0-7)

- `stop([ch])`<br>
Stop playback of all channels. If `ch`(0-3) is specified, stop the corresponding channel only

### Image Class

- `width`, `height`<br>
The width and height of the image

- `data`<br>
The data of the image (256x256 two-dimentional list)

- `get(x, y)`<br>
Retrieve the data of the image at (`x`, `y`)

- `set(x, y, data)`<br>
Set the data of the image at (`x`, `y`) by a value or a list of strings<br>
e.g. `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Read the png image from the directory of the execution script at (`x`, `y`)

- `copy(x, y, img, u, v, w, h)`<br>
Copy the region of size (`w`, `h`) from (`u`, `v`) of the image bank `img`(0-2) to (`x`, `y`)

### Tilemap Class

- `width`, `height`<br>
The width and height of the tilemap

- `data`<br>
The data of the tilemap (256x256 two-dimentional list)

- `refimg`<br>
The image bank referenced by the tilemap

- `get(x, y)`<br>
Retrieve the data of the tilemap at (`x`, `y`)

- `set(x, y, data)`<br>
Set the data of the tilemap at (`x`, `y`) by a value or a list of strings.<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `copy(x, y, tm, u, v, w, h)`<br>
Copy the region of size (`w`, `h`) from (`u`, `v`) of the tilemap `tm`(0-7) to (`x`, `y`)

### Sound Class

- `note`<br>
List of note(0-127) (33 = 'A2' = 440Hz)

- `tone`<br>
List of tone(0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volume`<br>
List of volume(0-7)

- `effect`<br>
List of effects(0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`<br>
The length of one note(120 = 1 second per tone)

- `set(note, tone, volume, effect, speed)`<br>
Set a note, tone, volume, and effect with a string. If the tone, volume, and effect length are shorter than the note, it is repeated from the beginning

- `set_note(note)`<br>
Set the note with a string made of 'CDEFGAB'+'#-'+'0123' or 'R'. Case-insensitive and whitespace is ignored<br>
e.g. `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tone(tone)`<br>
Set the tone with a string made of 'TSPN'. Case-insensitive and whitespace is ignored<br>
e.g. `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volume(volume)`<br>
Set the volume with a string made of '01234567'. Case-insensitive and whitespace is ignored<br>
e.g. `pyxel.sound(0).set_volume("7777 7531")`

- `set_effect(effect)`<br>
Set the effect with a string made of 'NSVF'. Case-insensitive and whitespace is ignored<br>
e.g. `pyxel.sound(0).set_effect("NFNF NVVS")`

### Music Class

- `ch0`<br>
List of sound(0-63) play on channel 0. If an empty list is specified, the channel is not used for playback

- `ch1`<br>
List of sound(0-63) play on channel 1. If an empty list is specified, the channel is not used for playback

- `ch2`<br>
List of sound(0-63) play on channel 2. If an empty list is specified, the channel is not used for playback

- `ch3`<br>
List of sound(0-63) play on channel 3. If an empty list is specified, the channel is not used for playback

- `set(ch0, ch1, ch2, ch3)`<br>
Set the list of sound(0-63) of all channels. If an empty list is specified, that channel is not used for playback<br>
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

- `set_ch0(data)`<br>
Set the list of sound(0-63) of channel 0

- `set_ch1(data)`<br>
Set the list of sound(0-63) of channel 1

- `set_ch2(data)`<br>
Set the list of sound(0-63) of channel 2

- `set_ch3(data)`<br>
Set the list of sound(0-63) of channel 3

## How to Contribute

### Submitting an issue

Use the [issue tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature/enhancement requests.
Before submitting a new issue, search the issue tracker to ensure that there is no similar open issue.

When submitting a report, select the appropriate template from [this link](https://github.com/kitao/pyxel/issues/new/choose).

### Manual testing

Anyone manually testing the code and reporting bugs or suggestions for enhancements in the issue tracker are very welcome!

### Submitting a pull request

Patches/fixes are accepted in form of pull requests (PRs). Make sure the issue the pull request addresses is open in the issue tracker.

Submitted pull request is deemed to have agreed to publish under [MIT license](https://github.com/kitao/pyxel/blob/master/LICENSE).

## Other Information

- [Wiki](https://github.com/kitao/pyxel/wiki)
- [Subreddit](https://www.reddit.com/r/pyxel/)

## License

Pyxel is under [MIT license](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software provided that all copies of the licensed software include a copy of the MIT License terms and the copyright notice.

Pyxel uses the following libraries:

- [SDL2](https://www.libsdl.org/)
- [gif-h](https://github.com/ginsweater/gif-h)
- [miniz-cpp](https://github.com/tfussell/miniz-cpp)
- [PyInstaller](https://www.pyinstaller.org/)
