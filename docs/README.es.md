# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** es un motor de juegos retro para Python.

Las especificaciones están inspiradas en las consolas de juegos retro, como el soporte para solo 16 colores y 4 canales de sonido, lo que te permite disfrutar fácilmente de la creación de juegos con estilo de arte en píxeles.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

El desarrollo de Pyxel se basa en los comentarios de los usuarios. ¡Por favor, dale una estrella a Pyxel en GitHub!

<p>
<a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">
<img src="images/10_platformer.gif" width="290">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/30sec_of_daylight.html">
<img src="images/30sec_of_daylight.gif" width="350">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">
<img src="images/02_jump_game.gif" width="330">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">
<img src="images/megaball.gif" width="310">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_music_editor.gif" width="320">
</a>
</p>

Las especificaciones y APIs de Pyxel están inspiradas en [PICO-8](https://www.lexaloffle.com/pico-8.php) y [TIC-80](https://tic80.com/).

Pyxel es de código abierto bajo la [Licencia MIT](../LICENSE) y es gratuito para usar. ¡Comencemos a crear juegos retro con Pyxel!

## Especificaciones

- Funciona en Windows, Mac, Linux y Web
- Programación en Python
- Tamaño de pantalla personalizable
- Paleta de 16 colores
- 3 bancos de imágenes de 256x256
- 8 mapas de teselas de 256x256
- 4 canales con 64 sonidos definibles
- 8 pistas de música que pueden combinar cualquier sonido
- Entradas de teclado, ratón y gamepad
- Herramientas de edición de imágenes y sonidos
- Colores, canales y bancos ampliables por el usuario

### Paleta de colores

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Cómo instalar

### Windows

Después de instalar [Python3](https://www.python.org/) (versión 3.8 o superior), ejecuta el siguiente comando:

```sh
pip install -U pyxel
```

Al instalar Python usando el instalador oficial, asegúrate de marcar la opción `Add Python 3.x to PATH` para habilitar el comando `pyxel`.

### Mac

Después de instalar [Homebrew](https://brew.sh/), ejecuta los siguientes comandos:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Para actualizar Pyxel después de la instalación, ejecuta `pipx upgrade pyxel`.

### Linux

Después de instalar el paquete SDL2 (`libsdl2-dev` para Ubuntu), [Python3](https://www.python.org/) (versión 3.8 o superior) y `python3-pip`, ejecuta el siguiente comando:

```sh
sudo pip3 install -U pyxel
```

Si el comando anterior falla, considera construir Pyxel desde el código fuente siguiendo las instrucciones en el [Makefile](../Makefile).

### Web

La versión web de Pyxel no requiere instalación de Python o Pyxel y se ejecuta en PCs, smartphones y tabletas con navegadores web compatibles.

Para obtener instrucciones detalladas, consulta [esta página](pyxel-web-en.md).

### Ejecutar ejemplos

Después de instalar Pyxel, puedes copiar los ejemplos al directorio actual con el siguiente comando:

```sh
pyxel copy_examples
```

Los siguientes ejemplos serán copiados a tu directorio actual:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>La aplicación más simple</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Código</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Juego de saltos con archivo de recursos de Pyxel</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Código</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Demostración de las API de dibujo</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Código</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Demostración de las API de sonido</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Código</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>Lista de paletas de colores</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Código</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>Juego de clic con el ratón</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Código</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>Juego de la serpiente con BGM</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Código</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Demostración de las API de dibujo de triángulos</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Código</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Shoot'em up con transiciones de pantalla y MML</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Código</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>Juego de plataformas desplazamiento lateral con mapa</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Código</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Renderizado fuera de pantalla con la clase Image</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Código</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>Animación de ruido de Perlin</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Código</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>Dibujo de una fuente de mapa de bits</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Código</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>Sintetizador utilizando características de expansión de audio</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Código</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>Carga y dibujo de Tiled Map File (.tmx)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Código</a></td>
</tr>
<tr>
<td>16_transform.py</td>
<td>Rotación y escalado de imágenes</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/16_transform.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/16_transform.py">Código</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>Animación con la función flip (solo para plataformas que no son web)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Código</a></td>
</tr>
<tr>
<td>30sec_of_daylight.pyxapp</td>
<td>Juego ganador del 1er Pyxel Jam por <a href="https://x.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30sec_of_daylight.html">Demo</a></td>
<td><a href="https://github.com/kitao/30SecondsOfDaylight">Código</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>Juego de física de pelota arcade por <a href="https://x.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Demo</a></td>
<td><a href="https://github.com/kitao/megaball">Código</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>Generador de música de fondo por <a href="https://x.com/frenchbread1222">frenchbread</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">Demo</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">Código</a></td>
</tr>
</table>

Los ejemplos se pueden ejecutar con los siguientes comandos:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30sec_of_daylight.pyxapp
```

## Cómo usar

### Crear aplicación

En su script de Python, importe el módulo Pyxel, especifique el tamaño de la ventana con la función `init` y luego inicie la aplicación Pyxel con la función `run`.

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

Los argumentos de la función `run` son la función `update`, que procesa las actualizaciones de los fotogramas, y la función `draw`, que maneja el dibujo en la pantalla.

En una aplicación real, se recomienda encapsular el código de Pyxel en una clase, como se muestra a continuación:

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

Para crear gráficos simples sin animación, puede usar la función `show` para simplificar su código.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Ejecutar aplicación

Un script creado se puede ejecutar utilizando el comando `python`:

```sh
python PYTHON_SCRIPT_FILE
```

También se puede ejecutar con el comando `pyxel run`:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Además, el comando `pyxel watch` supervisa los cambios en un directorio especificado y vuelve a ejecutar automáticamente el programa cuando se detectan cambios:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

La supervisión del directorio se puede detener presionando `Ctrl(Command)+C`.

### Operaciones de teclas especiales

Durante la ejecución de una aplicación Pyxel, se pueden realizar las siguientes acciones de teclas especiales:

- `Esc`<br>
  Salir de la aplicación
- `Alt(Option)+1`<br>
  Guardar la captura de pantalla en el escritorio
- `Alt(Option)+2`<br>
  Reiniciar el tiempo de inicio de la grabación del video de captura de pantalla
- `Alt(Option)+3`<br>
  Guardar un video de captura de pantalla en el escritorio (hasta 10 segundos)
- `Alt(Option)+8` o `A+B+X+Y+DL` en el gamepad<br>
  Alterna el escalado de pantalla entre máximo e entero
- `Alt(Option)+9` o `A+B+X+Y+DR` en el gamepad<br>
  Cambiar entre los modos de pantalla (Crisp/Smooth/Retro)
- `Alt(Option)+0` o `A+B+X+Y+DU` en el gamepad<br>
  Alternar el monitor de rendimiento (FPS/`update` tiempo/`draw` tiempo)
- `Alt(Option)+Enter` o `A+B+X+Y+DD` en el gamepad<br>
  Alternar pantalla completa
- `Shift+Alt(Option)+1/2/3`<br>
  Guarda el banco de imágenes 0, 1 o 2 en el escritorio
- `Shift+Alt(Option)+0`<br>
  Guardar la paleta de colores actual en el escritorio

### Cómo crear recursos

Pyxel Editor puede crear imágenes y sonidos utilizados en una aplicación Pyxel.

Puedes iniciar Pyxel Editor con el siguiente comando:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Si el archivo de recursos de Pyxel especificado (.pyxres) existe, se cargará. Si no existe, se creará un nuevo archivo con el nombre especificado. Si se omite el archivo de recursos, se creará un nuevo archivo llamado `my_resource.pyxres`.

Después de iniciar Pyxel Editor, puedes cambiar a otro archivo de recursos arrastrándolo y soltándolo en Pyxel Editor.

El archivo de recursos creado se puede cargar utilizando la función `load`.

Pyxel Editor tiene los siguientes modos de edición.

**Editor de imágenes**

El modo para editar la imagen en cada **banco de imágenes**.

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

Puedes arrastrar y soltar un archivo de imagen (PNG/GIF/JPEG) en el editor de imágenes para cargar la imagen en la banca de imágenes actualmente seleccionada.

**Editor de mapas de teselas**

El modo para editar los **mapas de teselas** que organizan imágenes de los bancos de imágenes en un patrón de teselas.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

Arrastra y suelta un archivo TMX (Tiled Map File) en el editor de mapas de teselas para cargar su capa 0 en el mapa de teselas actualmente seleccionado.

**Editor de sonidos**

El modo para editar los **sonidos** utilizados para melodías y efectos de sonido.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**Editor de música**

El modo para editar **músicas** en el que los sonidos están organizados en orden de reproducción.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### Otras formas de crear recursos

Las imágenes y los mapas de teselas de Pyxel también se pueden crear utilizando los siguientes métodos:

- Crear una imagen a partir de una lista de cadenas utilizando la función `Image.set` o la función `Tilemap.set`
- Cargar un archivo de imagen (PNG/GIF/JPEG) en la paleta de Pyxel con la función `Image.load`

Los sonidos de Pyxel también se pueden crear utilizando el siguiente método:

- Crear un sonido a partir de cadenas con la función `Sound.set` o la función `Music.set`

Consulta la referencia de la API para el uso de estas funciones.

### Cómo distribuir aplicaciones

Pyxel admite un formato de archivo dedicado a la distribución de aplicaciones (archivo de aplicación Pyxel) que es multiplataforma.

Un archivo de aplicación Pyxel (.pyxapp) se crea utilizando el comando `pyxel package`:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Si necesitas incluir recursos o módulos adicionales, colócalos en el directorio de la aplicación.

Los metadatos se pueden mostrar en tiempo de ejecución especificándolos en el siguiente formato dentro del script de inicio. Los campos distintos de `title` y `author` son opcionales.

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

El archivo de aplicación creado se puede ejecutar utilizando el comando `pyxel play`:

```sh
pyxel play PYXEL_APP_FILE
```

Un archivo de aplicación Pyxel también se puede convertir en un archivo ejecutable o un archivo HTML utilizando los comandos `pyxel app2exe` o `pyxel app2html`.

## Referencia de la API

### Sistema

- `width`, `height`<br>
  El ancho y la altura de la pantalla

- `frame_count`<br>
  El número de fotogramas transcurridos

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Inicializa la aplicación Pyxel con el tamaño de la pantalla (`width`, `height`). Se pueden especificar las siguientes opciones: el título de la ventana con `title`, la tasa de fotogramas con `fps`, la tecla para salir de la aplicación con `quit_key`, la escala de la pantalla con `display_scale`, la escala de captura de pantalla con `capture_scale`, y el tiempo máximo de grabación del video de captura de pantalla con `capture_sec`.<br>
  Ejemplo: `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Inicia la aplicación Pyxel y llama a la función `update` para actualizar el fotograma y a la función `draw` para dibujar.

- `show()`<br>
  Muestra la pantalla y espera hasta que se presione la tecla `Esc`.

- `flip()`<br>
  Actualiza la pantalla por un fotograma. La aplicación se cierra cuando se presiona la tecla `Esc`. Esta función no está disponible en la versión web.

- `quit()`<br>
  Cierra la aplicación Pyxel.

### Recursos

- `load(filename, [exclude_images], [exclude_tilemaps], [exclude_sounds], [exclude_musics])`<br>
  Carga el archivo de recursos (.pyxres). Si se establece una opción como `True`, se excluirá el recurso correspondiente de la carga. Si existe un archivo de paleta (.pyxpal) con el mismo nombre en la misma ubicación que el archivo de recursos, los colores de la paleta también se actualizarán. El archivo de paleta contiene entradas hexadecimales para los colores de visualización (ej. `1100ff`), separadas por saltos de línea. El archivo de paleta también puede utilizarse para cambiar los colores mostrados en Pyxel Editor.

- `user_data_dir(vendor_name, app_name)`<br>
  Devuelve el directorio de datos de usuario creado en función de `vendor_name` y `app_name`. Si el directorio no existe, se creará automáticamente. Se utiliza para almacenar puntuaciones altas, el progreso del juego y datos similares.<br>
  Ejemplo: `print(pyxel.user_data_dir("Takashi Kitao", "Pyxel Shooter"))`

### Entrada

- `mouse_x`, `mouse_y`<br>
  La posición actual del cursor del ratón

- `mouse_wheel`<br>
  El valor actual de la rueda del ratón

- `btn(key)`<br>
  Devuelve `True` si se presiona la tecla `key`, de lo contrario, devuelve `False`. ([Lista de definición de teclas](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  Devuelve `True` si se presiona la tecla `key` en ese fotograma, de lo contrario, devuelve `False`. Si se especifican `hold` y `repeat`, después de que la tecla `key` haya sido presionada durante más de `hold` fotogramas, se devolverá `True` cada `repeat` fotogramas.

- `btnr(key)`<br>
  Devuelve `True` si la tecla `key` se ha soltado en ese fotograma, de lo contrario, devuelve `False`.

- `mouse(visible)`<br>
  Muestra el cursor del ratón si `visible` es `True`, y lo oculta si `visible` es `False`. La posición del cursor sigue actualizándose incluso cuando está oculto.

### Gráficos

- `colors`<br>
  Lista de los colores de la paleta. El color de visualización se especifica mediante un valor numérico de 24 bits. Usa `colors.from_list` y `colors.to_list` para asignar y recuperar directamente listas de Python.<br>
  Ejemplo: `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Lista de los bancos de imágenes (instancias de la clase Image) (0-2)<br>
  Ejemplo: `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Lista de los mapas de teselas (instancias de la clase Tilemap) (0-7)

- `clip(x, y, w, h)`<br>
  Establece el área de dibujo de la pantalla desde (`x`, `y`) con un ancho de `w` y una altura de `h`. Llama a `clip()` para restablecer el área de dibujo a la pantalla completa.

- `camera(x, y)`<br>
  Cambia las coordenadas de la esquina superior izquierda de la pantalla a (`x`, `y`). Llama a `camera()` para restablecer las coordenadas de la esquina superior izquierda a (`0`, `0`).

- `pal(col1, col2)`<br>
  Reemplaza el color `col1` con `col2` al dibujar. Llama a `pal()` para restablecer la paleta original.

- `dither(alpha)`<br>
  Aplica tramado (pseudo-transparencia) al dibujar. Establece `alpha` en el rango de `0.0` a `1.0`, donde `0.0` es transparente y `1.0` es opaco.

- `cls(col)`<br>
  Limpia la pantalla con el color `col`.

- `pget(x, y)`<br>
  Obtiene el color del píxel en (`x`, `y`).

- `pset(x, y, col)`<br>
  Dibuja un píxel de color `col` en (`x`, `y`).

- `line(x1, y1, x2, y2, col)`<br>
  Dibuja una línea de color `col` desde (`x1`, `y1`) hasta (`x2`, `y2`).

- `rect(x, y, w, h, col)`<br>
  Dibuja un rectángulo de ancho `w`, alto `h` y color `col` desde (`x`, `y`).

- `rectb(x, y, w, h, col)`<br>
  Dibuja el contorno de un rectángulo de ancho `w`, alto `h` y color `col` desde (`x`, `y`).

- `circ(x, y, r, col)`<br>
  Dibuja un círculo con radio `r` y color `col` en (`x`, `y`).

- `circb(x, y, r, col)`<br>
  Dibuja el contorno de un círculo con radio `r` y color `col` en (`x`, `y`).

- `elli(x, y, w, h, col)`<br>
  Dibuja una elipse de ancho `w`, alto `h` y color `col` desde (`x`, `y`).

- `ellib(x, y, w, h, col)`<br>
  Dibuja el contorno de una elipse de ancho `w`, alto `h` y color `col` desde (`x`, `y`).

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  Dibuja un triángulo con vértices en (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) y color `col`.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  Dibuja el contorno de un triángulo con vértices en (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) y color `col`.

- `fill(x, y, col)`<br>
  Rellena el área conectada con el mismo color que (`x`, `y`) con el color `col`.

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Copia la región de tamaño (`w`, `h`) desde (`u`, `v`) del banco de imágenes `img`(0-2) a (`x`, `y`). Si se asigna un valor negativo a `w` y/o `h`, la región se volteará horizontal y/o verticalmente. Si se especifica `colkey`, será tratado como un color transparente. Si se especifican `rotate` (en grados), `scale` (1.0 = 100%) o ambos, se aplicarán las transformaciones correspondientes.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Copia la región de tamaño (`w`, `h`) desde (`u`, `v`) del mapa de teselas `tm`(0-7) a (`x`, `y`). Si se asigna un valor negativo a `w` y/o `h`, la región se volteará horizontal y/o verticalmente. Si se especifica `colkey`, será tratado como un color transparente. Si se especifican `rotate` (en grados), `scale` (1.0 = 100%) o ambos, se aplicarán las transformaciones correspondientes. El tamaño de una tesela es de 8x8 píxeles y se almacena en un mapa de teselas como una tupla de `(image_tx, image_ty)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  Dibuja una cadena de texto `s` en el color `col` en (`x`, `y`).

### Audio

- `sounds`<br>
  Lista de los sonidos (instancias de la clase Sound) (0-63)<br>
  Ejemplo: `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Lista de las músicas (instancias de la clase Music) (0-7)

- `play(ch, snd, [sec], [loop], [resume])`<br>
  Reproduce el sonido `snd`(0-63) en el canal `ch`(0-3). `snd` puede ser un número de sonido, una lista de números de sonido o una cadena MML. La posición de inicio de la reproducción puede especificarse en segundos con `sec`. Si `loop` se establece en `True`, la reproducción se repetirá. Para reanudar el sonido anterior después de que termine la reproducción, establece `resume` en `True`.

- `playm(msc, [sec], [loop])`<br>
  Reproduce la música `msc`(0-7). La posición de inicio de la reproducción puede especificarse en segundos con `sec`. Si `loop` se establece en `True`, la música se repetirá.

- `stop([ch])`<br>
  Detiene la reproducción del canal especificado `ch`(0-3). Llama a `stop()` para detener todos los canales.

- `play_pos(ch)`<br>
  Obtiene la posición de reproducción del sonido en el canal `ch`(0-3) como una tupla de `(sound_no, note_no)`. Devuelve `None` cuando se ha detenido la reproducción.

### Matemáticas

- `ceil(x)`<br>
  Devuelve el entero más pequeño que es mayor o igual a `x`.

- `floor(x)`<br>
  Devuelve el entero más grande que es menor o igual a `x`.

- `sgn(x)`<br>
  Devuelve `1` si `x` es positivo, `0` si es `0`, y `-1` si es negativo.

- `sqrt(x)`<br>
  Devuelve la raíz cuadrada de `x`.

- `sin(deg)`<br>
  Devuelve el seno de `deg` grados.

- `cos(deg)`<br>
  Devuelve el coseno de `deg` grados.

- `atan2(y, x)`<br>
  Devuelve el arcotangente de `y`/`x` en grados.

- `rseed(seed)`<br>
  Establece la semilla del generador de números aleatorios.

- `rndi(a, b)`<br>
  Devuelve un número entero aleatorio mayor o igual a `a` y menor o igual a `b`.

- `rndf(a, b)`<br>
  Devuelve un número flotante aleatorio mayor o igual a `a` y menor o igual a `b`.

- `nseed(seed)`<br>
  Establece la semilla del ruido de Perlin.

- `noise(x, [y], [z])`<br>
  Devuelve el valor de ruido de Perlin para las coordenadas especificadas.

### Clase Image

- `width`, `height`<br>
  El ancho y la altura de la imagen

- `set(x, y, data)`<br>
  Establece la imagen en (`x`, `y`) utilizando una lista de cadenas de texto.<br>
  Ejemplo: `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Carga un archivo de imagen (PNG/GIF/JPEG) en (`x`, `y`).

- `pget(x, y)`<br>
  Obtiene el color del píxel en (`x`, `y`).

- `pset(x, y, col)`<br>
  Dibuja un píxel con el color `col` en (`x`, `y`).

### Clase Tilemap

- `width`, `height`<br>
  El ancho y la altura del mapa de teselas

- `imgsrc`<br>
  El banco de imágenes (0-2) referenciado por el mapa de teselas

- `set(x, y, data)`<br>
  Establece el mapa de teselas en (`x`, `y`) utilizando una lista de cadenas de texto.<br>
  Ejemplo: `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  Carga la `layer`(0-) desde el archivo TMX (Tiled Map File) en (`x`, `y`).

- `pget(x, y)`<br>
  Obtiene la tesela en (`x`, `y`). Una tesela se representa como una tupla de `(image_tx, image_ty)`.

- `pset(x, y, tile)`<br>
  Dibuja una `tesela` en (`x`, `y`). Una tesela se representa como una tupla de `(image_tx, image_ty)`.

### Clase Sound

- `notes`<br>
  Lista de notas (0-127). Cuanto mayor es el número, mayor es el tono. La nota `33` corresponde a 'A2'(440Hz). Las notas de descanso se representan con `-1`.

- `tones`<br>
  Lista de tonos (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
  Lista de volúmenes (0-7)

- `effects`<br>
  Lista de efectos (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  Velocidad de reproducción. `1` es la más rápida, y cuanto mayor sea el número, más lenta será la reproducción. A `120`, la duración de una nota es de 1 segundo.

- `set(notes, tones, volumes, effects, speed)`<br>
  Establece notas, tonos, volúmenes y efectos utilizando una cadena de texto. Si la longitud de los tonos, volúmenes o efectos es menor que la de las notas, se repetirán desde el principio.

- `set_notes(notes)`<br>
  Establece las notas utilizando una cadena de texto compuesta por `CDEFGAB`+`#-`+`01234` o `R`. No se distingue entre mayúsculas y minúsculas, y los espacios en blanco se ignoran.<br>
  Ejemplo: `pyxel.sounds[0].set_notes("g2b-2d3r rf3f3f3")`

- `set_tones(tones)`<br>
  Establece los tonos con una cadena de texto compuesta por `TSPN`. No se distingue entre mayúsculas y minúsculas, y los espacios en blanco se ignoran.<br>
  Ejemplo: `pyxel.sounds[0].set_tones("ttss pppn")`

- `set_volumes(volumes)`<br>
  Establece los volúmenes con una cadena de texto compuesta por `01234567`. No se distingue entre mayúsculas y minúsculas, y los espacios en blanco se ignoran.<br>
  Ejemplo: `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  Establece los efectos con una cadena de texto compuesta por `NSVFHQ`. No se distingue entre mayúsculas y minúsculas, y los espacios en blanco se ignoran.<br>
  Ejemplo: `pyxel.sounds[0].set_effects("nfnf nvvs")`

- `mml(code)`<br>
  Al pasar una cadena [MML (Music Macro Language)](https://en.wikipedia.org/wiki/Music_Macro_Language), se cambia al modo MML y se reproduce el sonido según su contenido. En este modo, se ignoran los parámetros normales como `notes` y `speed`. Para salir del modo MML, llama a `mml()` sin argumentos. Para más detalles sobre MML, consulta [esta página](faq-en.md).<br>
  Ejemplo: `pyxel.sounds[0].mml("T120 Q90 @1 V100 O5 L8 C4&C<G16R16>C.<G16 >C.D16 @VIB1{10,20,20} E2C2")`

- `save(filename, count, [ffmpeg])`<br>
  Crea un archivo WAV que contiene el sonido repetido `count` veces. Si FFmpeg está instalado y `ffmpeg` se establece en `True`, también se crea un archivo MP4.

- `total_sec()`<br>
  Devuelve la duración de reproducción del sonido en segundos. Devuelve `None` si se utiliza un bucle infinito en MML.

### Clase Music

- `seqs`<br>
  Una lista bidimensional de sonidos (0-63) a través de múltiples canales

- `set(seq0, seq1, seq2, ...)`<br>
  Establece las listas de sonidos (0-63) para cada canal. Si se especifica una lista vacía, ese canal no se utilizará para la reproducción.<br>
  Ejemplo: `pyxel.musics[0].set([0, 1], [], [3])`

- `save(filename, count, [ffmpeg])`<br>
  Crea un archivo WAV que contiene la música repetida `count` veces. Si FFmpeg está instalado y `ffmpeg` se establece en `True`, también se crea un archivo MP4.

### API Avanzada

Pyxel incluye una "API Avanzada" que no se menciona en esta referencia, ya que puede confundir a los usuarios o requerir conocimientos especializados para su uso.

Si confías en tus habilidades, ¡intenta crear obras increíbles usando [esto](../python/pyxel/__init__.pyi) como guía!

## Cómo Contribuir

### Informar Problemas

Utiliza el [Issue Tracker](https://github.com/kitao/pyxel/issues) para enviar informes de errores y solicitudes de funciones o mejoras. Antes de enviar un nuevo problema, asegúrate de que no haya problemas abiertos similares.

### Pruebas Funcionales

¡Cualquier persona que pruebe manualmente el código y reporte errores o sugerencias de mejoras en el [Issue Tracker](https://github.com/kitao/pyxel/issues) es muy bienvenida!

### Enviar Solicitudes de Extracción

Los parches y correcciones se aceptan en forma de solicitudes de extracción (PRs). Asegúrate de que el problema que aborda la solicitud de extracción esté abierto en el Issue Tracker.

Enviar una solicitud de extracción implica que aceptas licenciar tu contribución bajo la [Licencia MIT](../LICENSE).

## Otra Información

- [FAQ](faq-en.md)
- [Ejemplos de Usuarios](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Cuenta X del Desarrollador](https://x.com/kitao)
- [Servidor de Discord (Inglés)](https://discord.gg/Z87eYHN)
- [Servidor de Discord (Japonés)](https://discord.gg/qHA5BCS)

## Licencia

Pyxel está licenciado bajo la [Licencia MIT](../LICENSE). Se puede reutilizar en software propietario, siempre que todas las copias del software o sus partes sustanciales incluyan una copia de los términos de la Licencia MIT y un aviso de copyright.

## Búsqueda de Patrocinadores

Pyxel está buscando patrocinadores en GitHub Sponsors. Por favor, considera patrocinar a Pyxel para apoyar su mantenimiento continuo y desarrollo de funciones. Como beneficio, los patrocinadores pueden consultar directamente con el desarrollador de Pyxel. Para más detalles, visita [esta página](https://github.com/sponsors/kitao).
