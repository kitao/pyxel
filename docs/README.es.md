# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** (/ˈpɪksəl/) es un motor de juegos retro para Python.

Las especificaciones están inspiradas en las consolas de juegos retro, como el soporte para solo 16 colores y 4 canales de sonido, lo que te permite disfrutar fácilmente de la creación de juegos con estilo de arte en píxeles.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

El desarrollo de Pyxel se basa en los comentarios de los usuarios. ¡Por favor, dale una estrella a Pyxel en GitHub!

<p>
<a href="https://kitao.github.io/pyxel/wasm/showcase/examples/10-platformer.html">
<img src="images/10_platformer.gif" width="290">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/apps/30sec-of-daylight.html">
<img src="images/30sec_of_daylight.gif" width="350">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/examples/02-jump-game.html">
<img src="images/02_jump_game.gif" width="330">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/apps/megaball.html">
<img src="images/megaball.gif" width="310">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
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
- Colores, canales de sonido y bancos ampliables por el usuario

### Paleta de colores

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Cómo instalar

### Windows

Después de instalar [Python 3](https://www.python.org/) (versión 3.8 o superior), ejecuta el siguiente comando:

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

Después de instalar [Python 3](https://www.python.org/) (versión 3.8 o superior), ejecuta el siguiente comando:

```sh
pip install -U pyxel
```

Si el comando anterior falla, considera construir Pyxel desde el código fuente siguiendo las instrucciones en el [Makefile](../Makefile).

### Web

La versión web de Pyxel funciona en PCs, smartphones y tablets con un navegador compatible, sin instalar Python ni Pyxel.

La forma más fácil de usarla es a través del IDE en línea [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/).

Para otros patrones de uso, como incrustar aplicaciones Pyxel en tu propio sitio, consulta [esta página](pyxel-web-en.md).

## Uso básico

### Comando Pyxel

Al instalar Pyxel, se añade el comando `pyxel`. Especifique un nombre de comando después de `pyxel` para realizar diversas operaciones.

Ejecútelo sin argumentos para ver la lista de comandos disponibles:

```sh
pyxel
```

```
Pyxel 2.7.1, a retro game engine for Python
usage:
    pyxel run PYTHON_SCRIPT_FILE(.py)
    pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE(.py)
    pyxel play PYXEL_APP_FILE(.pyxapp)
    pyxel edit [PYXEL_RESOURCE_FILE(.pyxres)]
    pyxel package APP_DIR STARTUP_SCRIPT_FILE(.py)
    pyxel app2exe PYXEL_APP_FILE(.pyxapp)
    pyxel app2html PYXEL_APP_FILE(.pyxapp)
    pyxel copy_examples
```

### Ejecutar ejemplos

El siguiente comando copia los ejemplos de Pyxel al directorio actual:

```sh
pyxel copy_examples
```

Los ejemplos se pueden ver y ejecutar en el navegador desde [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/).

En el entorno local, los ejemplos se pueden ejecutar con los siguientes comandos:

```sh
# Ejecutar ejemplo en el directorio examples
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# Ejecutar aplicación en el directorio examples/apps
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

## Creación de aplicaciones

### Crear un programa

En tu script de Python, importa Pyxel, especifica el tamaño de la ventana con `init` y luego inicia la aplicación con `run`.

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

Para crear gráficos simples sin animación, puedes usar la función `show` para simplificar tu código.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Ejecutar un programa

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

Detén la supervisión del directorio presionando `Ctrl(Command)+C`.

### Controles de teclas especiales

Durante la ejecución de una aplicación Pyxel, se pueden realizar las siguientes acciones de teclas especiales:

- `Esc`<br>
  Salir de la aplicación
- `Alt(Option)+R` o `A+B+X+Y+BACK` en el gamepad<br>
  Reiniciar la aplicación
- `Alt(Option)+1`<br>
  Guardar la captura de pantalla en el escritorio
- `Alt(Option)+2`<br>
  Reiniciar el tiempo de inicio de la grabación del video de captura de pantalla
- `Alt(Option)+3`<br>
  Guardar un video de captura de pantalla en el escritorio (hasta 10 segundos)
- `Alt(Option)+8` o `A+B+X+Y+DL` en el gamepad<br>
  Alternar el escalado de pantalla entre máximo e entero
- `Alt(Option)+9` o `A+B+X+Y+DR` en el gamepad<br>
  Cambiar entre los modos de pantalla (Crisp/Smooth/Retro)
- `Alt(Option)+0` o `A+B+X+Y+DU` en el gamepad<br>
  Alternar el monitor de rendimiento (FPS/tiempo de `update`/tiempo de `draw`)
- `Alt(Option)+Enter` o `A+B+X+Y+DD` en el gamepad<br>
  Alternar pantalla completa
- `Shift+Alt(Option)+1/2/3`<br>
  Guardar el banco de imágenes 0, 1 o 2 en el escritorio
- `Shift+Alt(Option)+0`<br>
  Guardar la paleta de colores actual en el escritorio

## Creación de recursos

### Pyxel Editor

Pyxel Editor crea imágenes y sonidos utilizados en una aplicación Pyxel.

Puedes iniciar Pyxel Editor con el siguiente comando:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Si el archivo de recursos de Pyxel especificado (.pyxres) existe, se cargará. Si no existe, se creará un nuevo archivo con el nombre especificado. Si se omite el archivo de recursos, se creará un nuevo archivo llamado `my_resource.pyxres`.

Después de iniciar Pyxel Editor, puedes cambiar a otro archivo de recursos arrastrándolo y soltándolo en el editor.

El archivo de recursos creado se puede cargar utilizando la función `load`.

Pyxel Editor tiene los siguientes modos de edición.

**Editor de imágenes**

El modo para editar imágenes en cada **banco de imágenes**.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_editor.gif">
</a>

Puedes arrastrar y soltar un archivo de imagen (PNG/GIF/JPEG) en el editor de imágenes para cargar la imagen en la banca de imágenes actualmente seleccionada.

**Editor de mapas de teselas**

El modo para editar los **mapas de teselas** que organizan imágenes de los bancos de imágenes en un patrón de teselas.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/tilemap-editor.html">
<img src="images/tilemap_editor.gif">
</a>

Arrastra y suelta un archivo TMX (Tiled Map File) en el editor de mapas de teselas para cargar su capa 0 en el mapa de teselas actualmente seleccionado.

**Editor de sonidos**

El modo para editar los **sonidos** utilizados para melodías y efectos de sonido.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
<img src="images/sound_editor.gif">
</a>

**Editor de música**

El modo para editar **pistas de música** en el que los sonidos están organizados en orden de reproducción.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/music-editor.html">
<img src="images/music_editor.gif">
</a>

### Otros métodos de creación

Las imágenes y los mapas de teselas de Pyxel también se pueden crear utilizando los siguientes métodos:

- Crear imágenes o mapas de teselas a partir de listas de cadenas con las funciones `Image.set` o `Tilemap.set`
- Cargar archivos de imagen compatibles con la paleta de Pyxel (PNG/GIF/JPEG) con la función `Image.load`

Los sonidos y la música de Pyxel también se pueden crear utilizando el siguiente método:

- Crearlos a partir de cadenas con las funciones `Sound.set` o `Music.set`

Consulta la referencia de la API para el uso de estas funciones.

## Distribución de aplicaciones

Pyxel admite un formato de distribución multiplataforma llamado archivo de aplicación Pyxel.

Crea un archivo de aplicación Pyxel (.pyxapp) con el comando `pyxel package`:

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

La lista completa de las API de Pyxel está disponible en [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/).

Pyxel también incluye una "API avanzada" que requiere conocimientos especializados. Puede verla marcando la casilla "Advanced" en la página de referencia.

Si confías en tus habilidades, ¡intenta usar la API avanzada para crear obras verdaderamente sorprendentes!

## Cómo Contribuir

### Informar Problemas

Utiliza el [Issue Tracker](https://github.com/kitao/pyxel/issues) para enviar informes de errores y solicitudes de funciones o mejoras. Antes de enviar un nuevo problema, asegúrate de que no haya problemas abiertos similares.

### Pruebas Funcionales

¡Cualquier persona que pruebe manualmente el código y reporte errores o sugerencias de mejoras en el [Issue Tracker](https://github.com/kitao/pyxel/issues) es muy bienvenida!

### Enviar Solicitudes de Extracción

Los parches y correcciones se aceptan en forma de solicitudes de extracción (PRs). Asegúrate de que el problema que aborda la solicitud de extracción esté abierto en el Issue Tracker.

Enviar una solicitud de extracción implica que aceptas licenciar tu contribución bajo la [Licencia MIT](../LICENSE).

## Herramientas y Ejemplos Web

- [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)
- [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/) [[User Manual](https://qiita.com/kitao/items/b5b3fb28ebf9781eda2e)]
- [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/) [[User Manual](https://qiita.com/kitao/items/a86de4f7d6a0ed656a89)]

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
