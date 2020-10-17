# <img src="images/pyxel_logo_152x64.png">

[ [English](README.md) | [日本語](README.ja.md) | [中文](README.cn.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Português](README.pt.md) | [Italiano](README.it.md) ] 

**Pyxel** es un motor de videojuegos para Python.

Gracias a sus características simples inspiradas en las consolas de juegos retro, como el solo mostrar 16 colores y el reproducir 4 sonidos al mismo tiempo, puedes sentirte libre de disfrutar creando juegos en estilo pixel art.

<a href="pyxel/examples/01_hello_pyxel.py" target="_blank">
<img src="pyxel/examples/screenshots/01_hello_pyxel.gif" width="48%">
</a>

<a href="pyxel/examples/02_jump_game.py" target="_blank">
<img src="pyxel/examples/screenshots/02_jump_game.gif" width="48%">
</a>

<a href="pyxel/examples/03_draw_api.py" target="_blank">
<img src="pyxel/examples/screenshots/03_draw_api.gif" width="48%">
</a>

<a href="pyxel/examples/04_sound_api.py" target="_blank">
<img src="pyxel/examples/screenshots/04_sound_api.gif" width="48%">
</a>

<a href="pyxel/editor/screenshots/image_tilemap_editor.gif" target="_blank">
<img src="pyxel/editor/screenshots/image_tilemap_editor.gif" width="48%">
</a>

<a href="pyxel/editor/screenshots/sound_music_editor.gif" target="_blank">
<img src="pyxel/editor/screenshots/sound_music_editor.gif" width="48%">
</a>

Las características de la consola de juego y APIs de Pyxel son referencia de los increíbles [PICO-8](https://www.lexaloffle.com/pico-8.php) y [TIC-80](https://tic.computer/).

Pyxel es de código abierto y de libre uso. Comencemos a crear juegos retro con Pyxel!

## Características

- Se ejecuta en Windows, Mac y Linux
- código con Python3
- Paleta fija de 16 colores
- 3 bancos de imágenes de tamaño 256x256
- 8 tilemaps de tamaño 256x256
- 4 canales con 64 sonidos configurables
- 8 canciones que pueden combinar sonidos arbitrarios
- Entradas de teclado, mouse, y gamepad
- Editor de imagen y sonido

### Paleta de Colores

<img src="pyxel/examples/screenshots/05_color_palette.png">
<br><br>
<img src="images/pyxel_palette.png">

## Cómo instalar

### Windows

Primero, instalar [Python3](https://www.python.org/) (versión 3.6.8 o superior).

Cuando instale Python con el instalador oficial, **añada Python al PATH** marcando el botón de abajo:

<img src="images/python_installer.png">

A continuación, instale Pyxel con el siguiente comando `pip` desde la línea de comandos:

```sh
pip install -U pyxel
```

### Mac

Primero, en el entorno donde está instalado el administrador de paquetes [Homebrew](https://brew.sh/), instale [Python3](https://www.python.org/) (versión 3.6.8 o superior) y el paquetes requeridos con el siguiente comando:

```sh
brew install python3 gcc sdl2 sdl2_image gifsicle
```

Puedes instalar Python3 de otras maneras, pero ten en cuenta que debes instalar otras bibliotecas.

Luego, **reinicie el terminal** e instale Pyxel con el comando `pip3`:

```sh
pip3 install -U pyxel
```

### Linux

Instala [Python3](https://www.python.org/) (versión 3.6.8 o superior) y los paquetes requeridos en la forma apropiada para cada distribución.

**Ubuntu:**

```sh
sudo apt install python3 python3-pip libsdl2-dev libsdl2-image-dev gifsicle
sudo pip3 install -U pyxel
```

### Otros entornos Linux

Para instalar Pyxel en un entorno diferente a los anteriores (Linux 32-bit, Raspberry PI, etc.), sigue los pasos siguientes  para montarlos:

#### Instalar las herramientas y paquetes necesarios:

- C++ build toolchain (debe incluir los comandos `gcc` y `make`)
- libsdl2-dev y libsdl2-image-dev
- [Python3](https://www.python.org/) (versión 3.6.8 o superior) y el comando `pip`

#### Ejecuta el siguiente comando en cualquier carpeta

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make -C pyxel/core clean all
pip3 install .
```

### Instalar los ejemplos:

Después de instalar Pyxel, los ejemplos serán copiados al directorio actual con el siguiente comando:

```sh
install_pyxel_examples
```

Los ejemplos a ser copiados son los siguientes:

- [01_hello_pyxel.py](pyxel/examples/01_hello_pyxel.py) - Aplicación simple
- [02_jump_game.py](pyxel/examples/02_jump_game.py) - Juego de plataformas con los archivos de recursos Pyxel
- [03_draw_api.py](pyxel/examples/03_draw_api.py) - Demonstración de la API de dibujado
- [04_sound_api.py](pyxel/examples/04_sound_api.py) - Demonstración de la API de sonido
- [05_color_palette.py](pyxel/examples/05_color_palette.py) - Lista de paleta de colores
- [06_click_game.py](pyxel/examples/06_click_game.py) - Juego de click con el mouse
- [07_snake.py](pyxel/examples/07_snake.py) - Juego de serpiente con música de fondo
- [08_triangle_api.py](pyxel/examples/08_triangle_api.py) - Demostración de la API dibujando triangulos
- [09_shooter.py](pyxel/examples/09_shooter.py) - Shoot'em up juego con transición de pantalla

Los ejemplos pueden ser ejecutados como código Python:

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

Luego de importar el módulo Pyxel en tu código Python, primero especifíca el tamaño de la pantalla con la función `init`, luego inicia la aplicación Pyxel con la función `run`.

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

También se puede escribir código simple utilizando las funciones `show` y `flip` para dibujar gráficos y animaciones simples.

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
Reinicia el tiempo de grabación de la captura de video de pantalla al inicial
- `Alt(Option)+3`<br>
Guarda la captura de video de pantalla (gif) en el escritorio (máximo 30 segundos)
- `Alt(Option)+0`<br>
Intercambia el monitor de rendimiento (fps, tiempo de actualización, y tiempo de dibujado)
- `Alt(Option)+Enter`<br>
Intercambia la pantalla completa

### Cómo crear un recurso

La aplicacion "Pyxel Editor" viene adjunta, puede crear imágenes y sonidos usados dentro de Pyxel.

Pyxel Editor se inicia con el siguiente comando:

```sh
pyxeleditor [pyxel_resource_file]
```

Si el archivo de recurso Pyxel especificado (.pyxres) existe, el archivo es cargado, sino existe, un nuevo archivo es creado con el nombre especificado.
Si el archivo de recurso es omitido, el nombre es `my_resource.pyxres`.

Luego de iniciar el Pyxel Editor, el archivo puede ser cambiado arrastrando y soltando otro archivo de recurso. Si el archivo de recurso es arrastrado y soltado mientras se sostiene la tecla ``Ctrl``(``Cmd``), solo el tipo de recurso (imagen/tilemap/sonido/musica) que esta siendo editado será cargado. Esta operación permite combinar multiples archivos de recursos en uno.

El archivo de recurso creado puede ser cargado con la función `load`.

Pyxel Editor tiene las siguientes modalidades de edición.

**Editor de Imágenes:**

El modulo para editar el banco de imágenes.

<img src="pyxel/editor/screenshots/image_editor.gif">

Arrastrando y soltando un archivo png en la pantalla del Editor de Imágenes, la imagen puede ser cargada en el banco de imágenes seleccionado.

**Editor de Tilemap:**

El modulo para editar tilemaps en el que las imágenes del banco de imágenes son organizadas en un patrón de tiles.

<img src="pyxel/editor/screenshots/tilemap_editor.gif">

**Editor de Sonido:**

El modulo para editar sonidos.

<img src="pyxel/editor/screenshots/sound_editor.gif">

**Editor de Música:**

El modulo para editar música en la que los sonidos son organizados en orden de reproducción.

<img src="pyxel/editor/screenshots/music_editor.gif">

### Otros metodos de creación de recursos

La imágenes y tilemaps de Pyxel también pueden ser creadas de la siguiente forma:

- Crea una imagen desde una lista de cadena de texto con la función `Image.set` o `Tilemap.set`
- Carga un archivo png en la paleta de Pyxel con la función `Image.load`

Los sonidos en Pyxel también pueden ser creados de la siguiente forma:

- Crear un sonido desde cadenas de texto con la función `Sound.set` o `Music.set`

Favor acudir a la referencia del API para el uso de estas funciones.

### Cómo crear un ejecutable independiente

Mediante el uso del Pyxel Packager adjunto, se puede crear un ejecutable independiente que funcione incluso en ambientes donde Python no este instalado.

Para crear un ejecutable autónomo, en el entorno en el que está instalado [PyInstaller](https://www.pyinstaller.org/), especifique el archivo Python que se utilizará para lanzar la aplicación con el comando `pyxelpackager` de la siguiente manera:

```sh
pyxelpackager python_file
```

Cuando el proceso termine, el ejecutable independiente se encontrará en la carpeta `dist`.

Si también son necesarios recursos como archivos .pyxres y .png, ponlos dentro de la carpeta `assets` para que sean incluidos.

También es posible especificar un ícono con la opción ``-i icon_file``.

## Referencia de la API

### Sistema

- `width`, `height`<br>
El ancho y alto de la pantalla

- `frame_count`<br>
El número de cuadros transcurridos

- `init(width, height, [caption], [scale], [palette], [fps], [quit_key], [fullscreen])`<br>
Inicializar la aplicación Pyxel con el tamaño de pantalla (`width`, `height`). El ancho y alto máximo de la pantalla es 256<br>
También es posible especificar el título de la ventana con `caption`, el aumento de pantalla con `scale`, la paleta de colores con `palette`, los cuadros por segundo con `fps`, la tecla para salir de la aplicación con `quit_key`, y si se iniciará en modo de pantalla completa con `fullscreen`. `palette` es especificada como una lista de 16 elementos de color a 24 bits<br>
p.ej. `pyxel.init(160, 120, caption="Pyxel with PICO-8 palette", palette=[0x000000, 0x1D2B53, 0x7E2553, 0x008751, 0xAB5236, 0x5F574F, 0xC2C3C7, 0xFFF1E8, 0xFF004D, 0xFFA300, 0xFFEC27, 0x00E436, 0x29ADFF, 0x83769C, 0xFF77A8, 0xFFCCAA], quit_key=pyxel.KEY_NONE, fullscreen=True)`

- `run(update, draw)`<br>
Inicia la aplicación Pyxel y llama a la función `update` para la actualización de cuadros y la función `draw` para el dibujado

- `quit()`<br>
Sale de la aplicación Pyxel al final del cuadro actual

- `flip()`<br>
Fuerza el dibujado en la pantalla (no usarlo en aplicaciones normales)

- `show()`<br>
Dibuja la pantalla y espera indefinidamente (no usarlo en aplicaciones normales)

### Recurso

- `save(filename)`<br>
Guarda el archivo de recurso (.pyxres) en el directorio donde se ejecuta el script

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Lee el archivo de recurso (.pyxres) desde el directorio donde se ejecuta el script. Si el tipo de recurso (image/tilemap/sound/music) se configura como ``False``, el recurso no será cargado.

### Entrada
- `mouse_x`, `mouse_y`<br>
La posición actual del puntero del mouse

- `mouse_wheel`<br>
El valor actual del mouse wheel

- `btn(key)`<br>
Devuelve `True` si `key` es presionada, sino devuelve `False` ([lista de definición de teclas](pyxel/__init__.py))

- `btnp(key, [hold], [period])`<br>
Devuelve `True` si `key` es presionada en ese cuadro, sino devuelve `False`. Cuando `hold` y `period` son definidos, `True` es devuelto en el intervalo de cuadro `period` cuando la `key` es sostenida por mas cuadros que el valor `hold`

- `btnr(key)`<br>
Devuelve `True` si `key` es liberada en ese cuadro, sino devuelve `False`

- `mouse(visible)`<br>
Si `visible` es `True`, muestra el puntero del mouse. Si es `False`, lo esconde. Incluso si el puntero del mouse no es mostrado, su posición si es actualizada.

### Gráficos

- `image(img, [system])`<br>
Opera el banco de imágenes `img`(0-2) (referirse a la clase Image). Si `system` es `True`, el banco de imágenes del sistema puede ser accedido. 3 es para el editor de recurso y fuente. 4 es para la pantalla de visualización<br>
p.ej. `pyxel.image(0).load(0, 0, "title.png")`

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
Dibuja el tilemap `tm`(0-7) en (`x`, `y`) de acuerdo a la información del tile de tamaño (`w`, `h`) de (`u`, `v`). Si `colkey` es definido, ese color se trata como transparencia. Un tile del tilemap es dibujado con un tamaño de 8x8, y si el número de tile es 0, apunta a la región (0, 0)-(7, 7) del banco de imágenes, si es 1, apunta a (8, 0)-(15, 0)

<img src="images/tilemap_mechanism.png">

- `text(x, y, s, col)`<br>
Dibuja una cadena de texto `s` de color `col` en (`x`, `y`)

### Audio

- `sound(snd, [system])`<br>
Opera el sonido `snd`(0-63) (referirse a la clase Sound). Si `system` es `True`, el sonido 64 del sistema puede ser accedido<br>
p.ej. `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Opera la música `msc`(0-7) (referirse a la clase Music)

- `play_pos(ch)`<br>
Obtiene la posición de reproducción de sonido del canal `ch`. Los cienes y miles indican el número de sonido y los unos y dieces inican el número de nota. Cuando la reproducción es detenida, retorna `-1`

- `play(ch, snd, loop=False)`<br>
Toca el sonido `snd`(0-63) en el canal `ch`(0-3). Los toca en orden cuando `snd` es una lista

- `playm(msc, loop=False)`<br>
Toca la música `msc`(0-7)

- `stop([ch])`<br>
Detiene la reproducción de todos los canales. Si `ch`(0-3) es definido, detiene únicamente el canal correspondiente

### Clase Image

- `width`, `height`<br>
El ancho y alto de la imagen

- `data`<br>
La data de la imagen (256x256 lista bidimensional)

- `get(x, y)`<br>
Obtiene la data de la imagen en (`x`, `y`)

- `set(x, y, data)`<br>
Establece la data de la imagen en (`x`, `y`) por un valor o una lista de cadenas de texto<br>
p.ej. `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Lee la imagen png desde el directorio de ejecución del script en (`x`, `y`)

- `copy(x, y, img, u, v, w, h)`<br>
Copia la región de tamaño (`w`, `h`) desde (`u`, `v`) del banco de imágenes `img`(0-2) en (`x`, `y`)

### Clase Tilemap

- `width`, `height`<br>
El ancho y alto del tilemap

- `data`<br>
La data del tilemap (256x256 lista bidimensional)

- `refimg`<br>
El banco de imágenes referenciado por el tilemap

- `get(x, y)`<br>
Obtiene la data del tilemap en (`x`, `y`)

- `set(x, y, data)`<br>
Establece la data del tilemap en (`x`, `y`) por un valor o una lista de cadenas de texto.<br>
p.ej. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `copy(x, y, tm, u, v, w, h)`<br>
Copia la región de tamaño (`w`, `h`) desde (`u`, `v`) del tilemap `tm`(0-7) en (`x`, `y`)

### Clase Sound

- `note`<br>
Lista de notas (0-127) (33 = 'A2' = 440Hz)

- `tone`<br>
Lista de tonos (0:Triangulo / 1:Cuadrado / 2:Pulso / 3:Ruido)

- `volume`<br>
Lista de volumen (0-7)

- `effect`<br>
Lista de efectos (0:Ninguno / 1:Deslisante / 2:Vibración / 3:Desvanecimiento)

- `speed`<br>
La longitud de una nota (120 = 1 segundo por tono)

- `set(note, tone, volume, effect, speed)`<br>
Establece una nota, tono, volumen, y efecto con una cadena de texto. Si la longitud del tono, volumen, y efecto son mas cortos que la nota, se repite desde el principio

- `set_note(note)`<br>
Establece la nota con una cadena hecha con 'CDEFGAB'+'#-'+'0123' o 'R'. No diferencia mayúsculas/minúsculas y los espacios en blanco son ignorados<br>
p.ej. `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tone(tone)`<br>
Establece el tono con una cadena de texto hecha con 'TSPN'. No diferencia mayúsculas/minúsculas y los espacios en blanco son ignorados<br>
p.ej. `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volume(volume)`<br>
Establece el volumen con una cadena de texto hecha con '01234567'. No diferencia mayúsculas/minúsculas y los espacios en blanco son ignorados<br>
p.ej. `pyxel.sound(0).set_volume("7777 7531")`

- `set_effect(effect)`<br>
Establece el efecto con una cadena de texto hecha con 'NSVF'. No diferencia mayúsculas/minúsculas y los espacios en blanco son ignorados<br>
p.ej. `pyxel.sound(0).set_effect("NFNF NVVS")`

### Clase Music

- `ch0`<br>
Lista de sonidos (0-63) tocada en el canal 0. Si se define una lista vacía, el canal no es usado para reproducción

- `ch1`<br>
Lista de sonidos (0-63) tocada en el canal 1. Si se define una lista vacía, el canal no es usado para reproducción

- `ch2`<br>
Lista de sonidos (0-63) tocada en el canal 2. Si se define una lista vacía, el canal no es usado para reproducción

- `ch3`<br>
Lista de sonidos (0-63) tocada en el canal 3. Si se define una lista vacía, el canal no es usado para reproducción

- `set(ch0, ch1, ch2, ch3)`<br>
Define la lista de sonidos (0-63) de todos los canales. Si se define una lista vacía, ese canal no es usado para reproducción<br>
p.ej. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

- `set_ch0(data)`<br>
Define la lista de sonidos (0-63) del canal 0

- `set_ch1(data)`<br>
Define la lista de sonidos (0-63) del canal 1

- `set_ch2(data)`<br>
Define la lista de sonidos (0-63) del canal 2

- `set_ch3(data)`<br>
Define la lista de sonidos (0-63) del canal 3

## Cómo contribuir

### Enviar un issue

Usar el [issue tracker](https://github.com/kitao/pyxel/issues) para enviar reportes de bugs y solicitud de características/mejoras.
Antes de enviar un nuevo issue, buscar en el issue tracker para asegurarse que no existe un issue similar abierto.

Cuando se envíe un reporte, seleccionar la plantilla apropiada de [este enlace](https://github.com/kitao/pyxel/issues/new/choose).

### Testeo manual

Cualquiera puede hacer test manuales del código y enviar reporte de bugs o sugerencias para mejoras en el issue tracker, serán bien recibidos!

### Enviando un pull request

Parches/correcciones son aceptados en forma de pull requests (PRs). Asegurarse que el issue del pull request que apunta este abierto en el issue tracker.

Los pull request enviados se consideran acordados publicarse bajo [licencia MIT](LICENSE).

## Otra Información

- [Wiki](https://github.com/kitao/pyxel/wiki)
- [Subreddit](https://www.reddit.com/r/pyxel/)
- [Discord server (English)](https://discord.gg/FC7kUZJ)
- [Discord server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## Licencia

Pyxel esta bajo [licencia MIT](http://en.wikipedia.org/wiki/MIT_License). Puede ser reutilizado dentro de software propietario proporcionando a todas las copias del software licenciado que incluyan una copia de la licencia MIT, términos y aviso de derechos de autor.

Pyxel utiliza el siguiente software:

- [SDL2](https://www.libsdl.org/)
- [miniz-cpp](https://github.com/tfussell/miniz-cpp)
- [Gifsicle](https://www.lcdf.org/gifsicle/)
