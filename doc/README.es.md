# <img src="images/pyxel_logo_152x64.png">

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**NOTE: This manual has not yet been translated for Pyxel version 1.5.0. We are looking for volunteers to translate and check for mistakes!**

**Pyxel** es un motor de videojuegos para Python.

Gracias a sus características simples inspiradas en las consolas de juegos retro, como el solo mostrar 16 colores y el reproducir 4 sonidos al mismo tiempo, puedes sentirte libre de disfrutar creando juegos en estilo pixel art.

<a href="../pyxel/examples/01_hello_pyxel.py" target="_blank">
<img src="images/01_hello_pyxel.gif" width="48%">
</a>

<a href="../pyxel/examples/02_jump_game.py" target="_blank">
<img src="images/02_jump_game.gif" width="48%">
</a>

<a href="../pyxel/examples/03_draw_api.py" target="_blank">
<img src="images/03_draw_api.gif" width="48%">
</a>

<a href="../pyxel/examples/04_sound_api.py" target="_blank">
<img src="images/04_sound_api.gif" width="48%">
</a>

<a href="images/image_tilemap_editor.gif" target="_blank">
<img src="images/image_tilemap_editor.gif" width="48%">
</a>

<a href="images/sound_music_editor.gif" target="_blank">
<img src="images/sound_music_editor.gif" width="48%">
</a>

The specifications of Pyxel are referring to awesome [PICO-8](https://www.lexaloffle.com/pico-8.php) and [TIC-80](https://tic.computer/).

Pyxel es de código abierto y de libre uso. Comencemos a crear juegos retro con Pyxel!

## Características

- Se ejecuta en Windows, Mac y Linux
- Programming with Python
- 16 color palette
- 3 bancos de imágenes de tamaño 256x256
- 8 tilemaps de tamaño 256x256
- 4 canales con 64 sonidos configurables
- 8 canciones que pueden combinar sonidos arbitrarios
- Entradas de teclado, mouse, y gamepad
- Editor de imagen y sonido

### Paleta de Colores

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Cómo instalar

There are two types of Pyxel, a packaged version and a standalone version.

### Install Packaged Version

The packaged version of Pyxel uses Pyxel as a Python extension module.

Recommended for those who are familiar with managing Python packages using the `pip` command or who want to develop full-fledged Python applications.

**Windows**

After installing [Python3](https://www.python.org/) (version 3.7 or higher), run the following command:

```sh
pip install -U pyxel
```

**Mac**

After installing [Python3](https://www.python.org/) (version 3.7 or higher), run the following command:

```sh
pip3 install -U pyxel
```

**Linux**

After installing the SDL2 package (`libsdl2-dev` for Ubuntu), [Python3](https://www.python.org/) (version 3.7 or higher), and `python3-pip`, run the following command:

```sh
sudo pip3 install -U pyxel
```

If the above doesn't work, try self-building by following the steps below after installing `cmake` and `rust`:

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make clean all
sudo pip3 install .
```

### Install Standalone Version

The standalone version of Pyxel uses Pyxel as a standalone tool that does not depend on Python.

Recommended for those who want to start programming easily without worrying about Python settings, or those who want to play Pyxel games immediately.

**Windows**

Download and run the latest version of the Windows installer (`pyxel-[version]-windows-setup.exe`) from the [Download Page](https://github.com/kitao/pyxel/releases).

**Mac**

After installing [Homebrew](https://brew.sh/), run the following commands:

```sh
brew tap kitao/pyxel
brew install pyxel
```

**Linux**

After installing the SDL2 package (`libsdl2-dev` for Ubuntu) and installing [Homebrew](https://brew.sh/), run the following commands:

```sh
brew tap kitao/pyxel
brew install pyxel
```

If the above doesn't work, try self-building the packaged version.

### Try Pyxel Examples

Después de instalar Pyxel, los ejemplos serán copiados al directorio actual con el siguiente comando:

```sh
pyxel copy_examples
```

Los ejemplos a ser copiados son los siguientes:

- [01_hello_pyxel.py](../pyxel/examples/01_hello_pyxel.py) - Aplicación simple
- [02_jump_game.py](../pyxel/examples/02_jump_game.py) - Juego de plataformas con los archivos de recursos Pyxel
- [03_draw_api.py](../pyxel/examples/03_draw_api.py) - Demonstration of drawing APIs
- [04_sound_api.py](../pyxel/examples/04_sound_api.py) - Demonstration of sound APIs
- [05_color_palette.py](../pyxel/examples/05_color_palette.py) - Lista de paleta de colores
- [06_click_game.py](../pyxel/examples/06_click_game.py) - Juego de click con el mouse
- [07_snake.py](../pyxel/examples/07_snake.py) - Juego de serpiente con música de fondo
- [08_triangle_api.py](../pyxel/examples/08_triangle_api.py) - Demonstration of triangle drawing APIs
- [09_shooter.py](../pyxel/examples/09_shooter.py) - Shoot'em up juego con transición de pantalla
- [10_platformer.py](../pyxel/examples/10_platformer.py) - Side-scrolling platform game with map

An examples can be executed with the following commands:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
```

## Cómo usarlo

### Crear una aplicación Pyxel

After importing the Pyxel module in your python script, specify the window size with `init` function first, then starts the Pyxel application with `run` function.

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

Los argumentos de la función `run` son la función `update` para actualizar cada cuadro, y la función `draw` para dibujar la escena cuando es necesario.

En la aplicación actual, es recomendable envolver el código Pyxel dentro de una clase como a continuación:

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

It is also possible to write simple code using `show` function and `flip` function to draw simple graphics and animations.

`show` function displays the screen and waits until the `Esc` key is pressed.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

`flip` function updates the screen once.

```python
import pyxel

pyxel.init(120, 80)

while True:
    pyxel.cls(3)
    pyxel.rectb(pyxel.frame_count % 160 - 40, 20, 40, 40, 7)
    pyxel.flip()
```

### Run Pyxel Application

The created Python script can be executed with the following command:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

For the packaged version, it can be executed like a normal Python script:

```sh
cd pyxel_examples
python3 PYTHON_SCRIPT_FILE
```

(For Windows, type `python` instead of `python3`)

### Controles especiales

Los siguientes controles especiales pueden ser usados cuando una aplicación Pyxel se esta ejecutando:

- `Esc`<br>
Sale de la aplicación
- `Alt(Option)+1`<br>
Guarda la captura de pantalla en el escritorio
- `Alt(Option)+2`<br>
Reinicia el tiempo de grabación de la captura de video de pantalla al inicial
- `Alt(Option)+3`<br>
Guarda la captura de video de pantalla en el escritorio (máximo 10 segundos)
- `Alt(Option)+0`<br>
Intercambia el monitor de rendimiento (fps, tiempo de actualización, y tiempo de dibujado)
- `Alt(Option)+Enter`<br>
Intercambia la pantalla completa

### Cómo crear un recurso

Pyxel Editor can create images and sounds used in a Pyxel application.

It starts with the following command:

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

Si el archivo de recurso Pyxel especificado (.pyxres) existe, el archivo es cargado, sino existe, un nuevo archivo es creado con el nombre especificado.
Si el archivo de recurso es omitido, el nombre es `my_resource.pyxres`.

After starting Pyxel Editor, the file can be switched by dragging and dropping another resource file. If the resource file is dragged and dropped while holding down ``Ctrl(Cmd)`` key, only the resource type (Image/Tilemap/Sound/Music) that is currently being edited will be loaded. This operation enables to combine multiple resource files into one.

The created resource file can be loaded with `load` function.

Pyxel Editor tiene las siguientes modalidades de edición.

**Editor de Imágenes:**

El modulo para editar el banco de imágenes.

<img src="images/image_editor.gif">

By dragging and dropping an image file (png/gif/jpeg) onto the Image Editor screen, the image can be loaded into the currently selected image bank.

**Editor de Tilemap:**

El modulo para editar tilemaps en el que las imágenes del banco de imágenes son organizadas en un patrón de tiles.

<img src="images/tilemap_editor.gif">

**Editor de Sonido:**

El modulo para editar sonidos.

<img src="images/sound_editor.gif">

**Editor de Música:**

El modulo para editar música en la que los sonidos son organizados en orden de reproducción.

<img src="images/music_editor.gif">

### Otros metodos de creación de recursos

Pyxel images and tilemaps can also be created by the following methods:

- Create an image from a list of strings with `Image.set` function or `Tilemap.set` function
- Load an image file (png/gif/jpeg) in Pyxel palette with `Image.load` function

Pyxel sounds can also be created in the following method:

- Create a sound from strings with `Sound.set` function or `Music.set` function

Favor acudir a la referencia del API para el uso de estas funciones.

### How to Distribute Application

Pyxel supports a dedicated application distribution file format (Pyxel application file) that works across platforms.

Create the Pyxel application file (.pyxapp) with the following command:

```sh
pyxel package APP_ROOT_DIR STARTUP_SCRIPT_FILE
```

If the application should include resources or additional modules, place them in the application folder.

The created application file can be executed with the following command:

```sh
pyxel play PYXEL_APP_FILE
```

## Referencia de la API

### Sistema

- `width`, `height`<br>
El ancho y alto de la pantalla

- `frame_count`<br>
El número de cuadros transcurridos

- `init(width, height, [title], [fps], [quit_key], [capture_sec])`<br>
Initialize the Pyxel application with screen size (`width`, `height`). The following can be specified as options: the window title with `title`, the frame rate with `fps`, the key to quit the application with `quit_key`, and the maximum recording time of the screen capture video with `capture_sec`.<br>
e.g. `pyxel.init(160, 120, title="Pyxel with Options", fps=60, quit_key=pyxel.KEY_NONE, capture_sec=0)`

- `run(update, draw)`<br>
Start the Pyxel application and call `update` function for frame update and `draw` function for drawing.

- `show()`<br>
Show the screen and wait until the `Esc` key is pressed. (Do not use in normal applications)

- `flip()`<br>
Updates the screen once. (Do not use in normal applications)

- `quit()`<br>
Quit the Pyxel application at the end of the current frame.

### Recurso

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Load the resource file (.pyxres). If ``False`` is specified for the resource type (``image/tilemap/sound/music``), the resource will not be loaded.

### Entrada
- `mouse_x`, `mouse_y`<br>
La posición actual del puntero del mouse

- `mouse_wheel`<br>
El valor actual del mouse wheel

- `btn(key)`<br>
Devuelve `True` si `key` es presionada, sino devuelve `False` ([lista de definición de teclas](../pyxel/__init__.pyi))

- `btnp(key, [hold], [period])`<br>
Devuelve `True` si `key` es presionada en ese cuadro, sino devuelve `False`. Cuando `hold` y `period` son definidos, `True` es devuelto en el intervalo de cuadro `period` cuando la `key` es sostenida por mas cuadros que el valor `hold`

- `btnr(key)`<br>
Devuelve `True` si `key` es liberada en ese cuadro, sino devuelve `False`

- `mouse(visible)`<br>
Si `visible` es `True`, muestra el puntero del mouse. Si es `False`, lo esconde. Incluso si el puntero del mouse no es mostrado, su posición si es actualizada.

### Gráficos

- `colors`<br>
List of the palette display colors. The display color is specified by a 24-bit numerical value. Use `colors.from_list` and `colors.to_list` to directly assign and retrieve Python lists.<br>
e.g. `org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img, [system])`<br>
Operate the image bank `img` (0-2). (See the Image class)<br>
e.g. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
Opera el tilemap `tm`(0-7) (referirse a la clase Tilemap)

- `clip(x, y, w, h)`<br>
Prepara el área de dibujado de la pantalla en (`x`, `y`) para el ancho `w` y alto `h`. Reinicia el área de dibujado para pantalla completa con `clip()`

- `pal(col1, col2)`<br>
Remplaza el color `col1` con `col2` en el dibujado. `pal()` para reiniciar a la paleta inicial

- `cls(col)`<br>
Limpia la pantalla con el color `col`

- `pget(x, y)`<br>
Obtiene el color del pixel en (`x`, `y`)

- `pset(x, y, col)`<br>
Dibuja un pixel de color `col` en (`x`, `y`)

- `line(x1, y1, x2, y2, col)`<br>
Dibuja una línea de color `col` desde (`x1`, `y1`) hasta (`x2`, `y2`)

- `rect(x, y, w, h, col)`<br>
Dibuja un rectángulo de ancho `w`, alto `h` y color `col` en (`x`, `y`)

- `rectb(x, y, w, h, col)`<br>
Dibuja el contorno de un rectángulo de ancho `w`, alto `h` y color `col` en (`x`, `y`)

- `circ(x, y, r, col)`<br>
Dibuja un círculo de radio `r` y color `col` en (`x`, `y`)

- `circb(x, y, r, col)`<br>
Dibuja el contorno de un círculo de radio `r` y color `col` en (`x`, `y`)

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
Dibuja un triangulo con los vértices (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) y color `col`

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
Dibuja el controno de un triangulo con los vértices (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) y color `col`

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
Copia la región de tamaño (`w`, `h`) de (`u`, `v`) del banco de imágenes `img`(0-2) en (`x`, `y`). Si se establece un valor negativo para `w` y/o `h`, será invertido horizontal y/o verticalmente. Si `colkey` es especificado, ese color se trata como transparencia

<img src="images/image_bank_mechanism.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Draw the tilemap `tm` (0-7) to (`x`, `y`) according to the tile information of size (`w`, `h`) from (`u`, `v`). If `colkey` is specified, treated as transparent color. The size of a tile is 8x8 pixels and is stored in a tilemap as a tuple of `(x in tile, y in tile)`.

- `text(x, y, s, col)`<br>
Dibuja una cadena de texto `s` de color `col` en (`x`, `y`)

### Audio

- `sound(snd)`<br>
Opera el sonido `snd`(0-63). (referirse a la clase Sound).<br>
p.ej. `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Opera la música `msc`(0-7) (referirse a la clase Music)

- `play_pos(ch)`<br>
Get the sound playback position of channel `ch` (0-3) as a tuple of `(sound no, note no)`. Returns `None` when playback is stopped.

- `play(ch, snd, loop=False)`<br>
Play the sound `snd` (0-63) on channel `ch` (0-3). If `snd` is a list, it will be played in order. If `True` is specified for `loop`, loop playback is performed.

- `playm(msc, loop=False)`<br>
Play the music `msc` (0-7). If `True` is specified for `loop`, loop playback is performed.

- `stop([ch])`<br>
Stops playback of the specified channel `ch` (0-3). `stop()` to stop playing all channels.

### Clase Image

- `width`, `height`<br>
El ancho y alto de la imagen

- `data`<br>
La data de la imagen (256x256 lista bidimensional)

- `get(x, y)`<br>
Obtiene la data de la imagen en (`x`, `y`)

- `set(x, y, data)`<br>
Set the image at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Load the image file (png/gif/jpeg) at (`x`, `y`).

### Clase Tilemap

- `width`, `height`<br>
El ancho y alto del tilemap

- `refimg`<br>
The image bank (0-2) referenced by the tilemap

- `set(x, y, data)`<br>
Set the tilemap at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `pget(x, y)`<br>
Get the tile at (`x`, `y`). A tile is a tuple of `(x in tile, y in tile)`.

- `pset(x, y, tile)`<br>
Draw a `tile` at (`x`, `y`). A tile is a tuple of `(x in tile, y in tile)`.

### Clase Sound

- `notes`<br>
List of notes (0-127). The higher the number, the higher the pitch, and at 33 it becomes 'A2'(440Hz). The rest is -1.

- `tones`<br>
List of tones (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
List of volumes (0-7)

- `effects`<br>
List of effects (0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`<br>
Playback speed. 1 is the fastest, and the larger the number, the slower the playback speed. At 120, the length of one note becomes 1 second.

- `set(notes, tones, volumes, effects, speed)`<br>
Set notes, tones, volumes, and effects with a string. If the tones, volumes, and effects length are shorter than the notes, it is repeated from the beginning.

- `set_notes(notes)`<br>
Set the notes with a string made of 'CDEFGAB'+'#-'+'0123' or 'R'. Case-insensitive and whitespace is ignored.<br>
e.g. `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
Set the tones with a string made of 'TSPN'. Case-insensitive and whitespace is ignored.<br>
e.g. `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volumes(volumes)`<br>
Set the volumes with a string made of '01234567'. Case-insensitive and whitespace is ignored.<br>
e.g. `pyxel.sound(0).set_volume("7777 7531")`

- `set_effects(effects)`<br>
Set the effects with a string made of 'NSVF'. Case-insensitive and whitespace is ignored.<br>
e.g. `pyxel.sound(0).set_effect("NFNF NVVS")`

### Clase Music

- `sequences`<br>
Two-dimensional list of sounds (0-63) listed by the number of channels

- `set(seq0, seq1, seq2, seq3)`<br>
Set the lists of sound (0-63) of all channels. If an empty list is specified, that channel is not used for playback.<br>
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### Advanced APIs

Pyxel has "advanced APIs" that are not mentioned in this reference because they "may confuse users" or "need specialized knowledge to use".

If you are familiar with your skills, try to create amazing works with [this](../pyxel/__init__.pyi) as a clue!

## Cómo contribuir

### Submitting Issue

Use the [Issue Tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature/enhancement requests. Before submitting a new issue, ensure that there is no similar open issue.

### Manual Testing

Anyone manually testing the code and reporting bugs or suggestions for enhancements in the [Issue Tracker](https://github.com/kitao/pyxel/issues) are very welcome!

### Submitting Pull Request

Parches/correcciones son aceptados en forma de pull requests (PRs). Asegurarse que el issue del pull request que apunta este abierto en el issue tracker.

Los pull request enviados se consideran acordados publicarse bajo [licencia MIT](../LICENSE).

## Otra Información

- [Discord server (English)](https://discord.gg/FC7kUZJ)
- [Discord server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## Licencia

Pyxel is under [MIT License](../LICENSE). It can be reused within proprietary software, provided that all copies of the software or its substantial portions include a copy of the terms of the MIT License and also a copyright notice.
