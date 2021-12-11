# <img src="images/pyxel_logo_152x64.png">

[ [English](README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**NOTE: This manual has not yet been translated for Pyxel version 1.5.0. We are looking for volunteers to translate and check for mistakes!**

**Pyxel** ist eine Retro-Spielengine für Python.

Dank seiner einfachen, von Retro-Spielkonsolen inspirierten, Spezifikationen, wie z. B. dass nur 16 Farben angezeigt werden können und nur 4 Töne gleichzeitig wiedergegeben werden können, kannst du dich frei fühlen, um Spiele im Pixel-Art-Stil zu entwickeln.

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

The specifications of Pyxel are referring to awesome [PICO-8](https://www.lexaloffle.com/pico-8.php) and [TIC-80](https://tic.computer/).

Pyxel ist quelloffen und kostenlos zu verwenden. Fang an, ein Retro-Spiel mit Pyxel zu entwickeln!

## Spezifikationen

- Läuft unter Windows, Mac, und Linux
- Programming with Python
- 16 color palette
- 256x256 große 3 Image Banks
- 256x256 große 8 Tilemaps
- 4 Kanäle mit 64 definierbaren Tönen
- 8 Musikspuren, die beliebige Klänge kombinieren können
- Tastatur-, Maus- und Gamepad-Eingaben
- Bild- und Toneditor

### Farbpalette

<img src="pyxel/examples/screenshots/05_color_palette.png">
<br><br>
<img src="images/pyxel_palette.png">

## Installationsanleitung

There are two types of Pyxel, a packaged version and a standalone version.

### Install the Packaged Version

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

### Linux

After installing the SDL2 package (`libsdl2-dev` for Ubuntu), [Python3](https://www.python.org/) (version 3.7 or higher), and `pip3`, run the following command:

```sh
pip3 install -U pyxel
```

If the above doesn't work, try self-building by following the steps below after installing `cmake` and `rust`:

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make clean all RELEASE=1
pip3 install .
```

### Install the Standalone Version

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

After installing the SDL2 package (`libsdl2-dev` for Ubuntu) and installing [Homebrew](https://docs.brew.sh/Homebrew-on-Linux), run the following commands:

```sh
brew tap kitao/pyxel
brew install pyxel
```

If the above doesn't work, try self-building the packaged version.

### Installiere Beispiele

Nach der Installation von Pyxel kannst du die Beispiele von Pyxel mit dem folgenden Befehl in das aktuelle Verzeichnis kopieren:

```sh
pyxel copy_examples
```

Die zu kopierenden Beispiele lauten wie folgt:

- [01_hello_pyxel.py](pyxel/examples/01_hello_pyxel.py) - Einfaches Fenster
- [02_jump_game.py](pyxel/examples/02_jump_game.py) - Spring Spiel mit Pyxel-Ressource-Datei
- [03_draw_api.py](pyxel/examples/03_draw_api.py) - Demonstration of drawing APIs
- [04_sound_api.py](pyxel/examples/04_sound_api.py) - Demonstration of sound APIs
- [05_color_palette.py](pyxel/examples/05_color_palette.py) - Farbpalleten Liste
- [06_click_game.py](pyxel/examples/06_click_game.py) - Maus-Klick-Spiel
- [07_snake.py](pyxel/examples/07_snake.py) - Snake mit BGM
- [08_triangle_api.py](pyxel/examples/08_triangle_api.py) - Demonstration of triangle drawing APIs
- [09_shooter.py](pyxel/examples/09_shooter.py) - Shoot'em up with Displayübergängen
- [10_platformer.py](pyxel/examples/10_platformer.py) - Side-scrolling platform game with map

An examples can be executed with the following commands:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
```

For the packaged version, it can be executed like a normal Python script:

```sh
cd pyxel_examples
python3 01_hello_pyxel.py
```

(For Windows, type `python` instead of `python3`)

## Verwendung

### Eine Pyxel-Anwendung erstellen

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

Die Argumente der Funktion `run` sind die `update` Funktion, um jedes Bild zu aktualisieren, und die Funktion `draw`, um den Bildschirm bei Bedarf zu zeichnen.

In einer tatsächlichen Anwendung ist es empfehlenswert, den Pyxel-Code in eine Klasse zu verpacken, wie unten dargestellt:

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

### Shortcuts

Die folgenden Shortcuts können eingegeben werden, während eine Pyxel-Anwendung läuft:

- `Esc`<br>
Schließt die Anwendung
- `Alt(Option)+1`<br>
Speichert einen Screenshot zum Desktop
- `Alt(Option)+2`<br>
Setzt die Startzeit für die Aufnahme des Bildschirmaufzeichnung zurück
- `Alt(Option)+3`<br>
Speichert die Bildschirmaufzeichnung zum Desktop (bis zu 10 Sekunden)
- `Alt(Option)+0`<br>
Umschalten des Leistungsmonitors (fps, Updatezeit und Framezeit)
- `Alt(Option)+Enter`<br>
Fullscreen umschalten

### Wie man eine Ressource erstellt

Pyxel Editor can create images and sounds used in a Pyxel application.

It starts with the following command:

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

Wenn die angegebene Pyxel-Datei (.pyxres) existiert, wird die Datei geladen, wenn nicht, wird eine neue Datei mit dem angegebenen Namen erstellt. Wenn die Datei nicht angegeben wird, lautet der Name my_resource.pyxres.

After starting Pyxel Editor, the file can be switched by dragging and dropping another resource file. If the resource file is dragged and dropped while holding down ``Ctrl(Cmd)`` key, only the resource type (Image/Tilemap/Sound/Music) that is currently being edited will be loaded. This operation enables to combine multiple resource files into one.

The created resource file can be loaded with `load` function.

Der Pyxel Editor hat die folgenden Bearbeitungsmodi

**Bildeditor:**

Der Modus zum Editieren von Image Banks.

<img src="pyxel/editor/screenshots/image_editor.gif">

By dragging and dropping an image file (png/gif/jpeg) onto the Image Editor screen, the image can be loaded into the currently selected image bank.

**Tilemap-Editor:**

Der Modus zum editieren von Tilemaps in welcher Bilder aus der Image Bank in einem tile pattern arrangiert sind.

<img src="pyxel/editor/screenshots/tilemap_editor.gif">

**Sound-Editor:**

Der Modus um Sounds zu editieren.

<img src="pyxel/editor/screenshots/sound_editor.gif">

**Musik-Editor:**

Der Modus um Sounds in Wiedergabereihenfolge zu Musik zusammenzufügen.

<img src="pyxel/editor/screenshots/music_editor.gif">

### Andere Methoden der Ressourcenerstellung

Pyxel images and tilemaps can also be created by the following methods:

- Create an image from a list of strings with `Image.set` function or `Tilemap.set` function
- Load an image file (png/gif/jpeg) in Pyxel palette with `Image.load` function

Pyxel sounds can also be created in the following method:

- Create a sound from strings with `Sound.set` function or `Music.set` function

Bitte lesen Sie die API-Referenz für die Verwendung dieser Funktionen.

### How to Distribute an Application

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

## API-Referenz

### System

- `width`, `height`<br>
Die Breite und Höhe des Fensters

- `frame_count`<br>
Die Anzahl der bereits gezeigten Bilder

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

### Ressource

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Load the resource file (.pyxres). If ``False`` is specified for the resource type (``image/tilemap/sound/music``), the resource will not be loaded.

### Eingabe
- `mouse_x`, `mouse_y`<br>
Die aktuelle Position der Maus

- `mouse_wheel`<br>
Der aktuelle Wert des Scrollrads

- `btn(key)`<br>
Return `True` falls `key` gedrückt ist, sonst return `False` ([key definition list](pyxel/__init__.pyi))

- `btnp(key, [hold], [period])`<br>
Return `True` falls `key` gedrückt ist, sonst return `False`. Wenn `hold` und `period` angegeben sind, wird `True` am `period` Bildintervall returned, falls `key` für mehr als `hold` Frames gedrückt ist

- `btnr(key)`<br>
Return `True` falls `key` in dem Frame losgelassen wird, sonst return `False`

- `mouse(visible)`<br>
Falls `visible`  `True` ist, zeige den Mauscursor. Falls `False`, verstecke ihn. Obwohl man den Cursor dann nicht sehen kann, wird seine Position geupdated

### Grafiken

- `colors`<br>
List of the palette display colors. The display color is specified by a 24-bit numerical value. Use `colors.from_list` and `colors.to_list` to directly assign and retrieve Python lists.<br>
e.g. `org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img)`<br>
Operate the image bank `img` (0-2). (See the Image class)<br>
e.g. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
Bediene die Tilemap `tm`(0-7) (siehe die Tilemap class)

- `clip(x, y, w, h)`<br>
Setze den Bildausschnitt von (`x`, `y`) zu Breite `w` und Höhe `h`. Setze den Bildausschnitt zurück zum Follbild mit `clip()`

- `pal(col1, col2)`<br>
Ersetze Farbe `col1` mit `col2` beim zeichnen. Mit `pal()` lässt sich die Pallete auf die initiale zurücksetzen

- `cls(col)`<br>
Das Fenster mit der Farbe `col` füllen

- `pget(x, y)`<br>
Erhalte den Pixel an der Position (`x`, `y`).

- `pset(x, y, col)`<br>
Zeichne einen Pixel der Farbe `col` an der Position (`x`, `y`)

- `line(x1, y1, x2, y2, col)`<br>
Zeichne eine Linie der Farbe `col` von (`x1`, `y1`) bis (`x2`, `y2`)

- `rect(x, y, w, h, col)`<br>
Zeichne ein Rechteck der Breite `w`, Höhe `h` und Farbe `col` ausgehend von (`x`, `y`)

- `rectb(x, y, w, h, col)`<br>
Zeichne die Umrisse eines Rechtecks der Breite `w`, Höhe `h` und Farbe `col` ausgehend von (`x`, `y`)

- `circ(x, y, r, col)`<br>
Zeichne einen Kreis mit dem Radius `r` und Farbe `col` an der Stelle (`x`, `y`)

- `circb(x, y, r, col)`<br>
Zeichne die Umrisse eines Kreises mit dem Radius `r` und Farbe `col` an der Stelle (`x`, `y`)

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
Zeichne ein Dreieck mit den Scheitelpunkten (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) und Farbe `col`

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
Zeichne die Umrisse eines Dreiecks mit den Scheitelpunkten (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) und Farbe `col`

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
Kopiere eine Region der Größe (`w`, `h`) von (`u`, `v`) des Image Banks `img`(0-2) zur Position (`x`, `y`). Falls `w` und/oder `h` negativ ist, wird der Ausschnitt horizontal und/oder vertical gespiegelt. Falls `colkey` angegeben ist, wird der Auschnitt als transparentes Farbe behandelt

<img src="images/image_bank_mechanism.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Draw the tilemap `tm` (0-7) to (`x`, `y`) according to the tile information of size (`w`, `h`) from (`u`, `v`). If `colkey` is specified, treated as transparent color. The size of a tile is 8x8 pixels and is stored in a tilemap as a tuple of `(x-in-tile, y-in-tile)`.

- `text(x, y, s, col)`<br>
Zeichne einen String `s` der Farbe `col` bei (`x`, `y`)

### Audio

- `sound(snd)`<br>
Bediene den Ton `snd`(0-63). (siehe die Sound class).<br>
z.B. `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Bediene die Musik `msc`(0-7) (siehe die Music class)

- `play_pos(ch)`<br>
Get the sound playback position of channel `ch` (0-3) as a tuple of `(sound-no, note-no)`. Returns `None` when playback is stopped.

- `play(ch, snd, loop=False)`<br>
Play the sound `snd` (0-63) on channel `ch` (0-3). If `snd` is a list, it will be played in order. If `True` is specified for `loop`, loop playback is performed.

- `playm(msc, loop=False)`<br>
Play the music `msc` (0-7). If `True` is specified for `loop`, loop playback is performed.

- `stop([ch])`<br>
Stops playback of the specified channel `ch` (0-3). `stop()` to stop playing all channels.

- `play_pos(ch)`<br>
Erhalte die Sound-Playback-Position des Kanals `ch`. Die 100's und 1000's zeigen die Soundnummer und die 1's und 10's zeigen die Notennummer. Wenn playback gestoppt ist, return `-1`

### Image Klasse

- `width`, `height`<br>
Die Breite und Höhe des Bildes

- `data`<br>
Die Daten des Bildes (256x256 zweidimensionale list)

- `get(x, y)`<br>
Erhalte die Daten des Bildes an der Position (`x`, `y`)

- `set(x, y, data)`<br>
Set the image at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Load the image file (png/gif/jpeg) at (`x`, `y`).

### Tilemap Klasse

- `width`, `height`<br>
Die Breite und Höhe der Tilemap

- `refimg`<br>
The image bank (0-2) referenced by the tilemap

- `set(x, y, data)`<br>
Set the tilemap at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `pget(x, y)`<br>
Get the tile at (`x`, `y`). A tile is a tuple of `(x-in-tile, y-in-tile)`.

- `pset(x, y, tile)`<br>
Draw a `tile` at (`x`, `y`). A tile is a tuple of `(x-in-tile, y-in-tile)`.

### Sound Klasse

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

### Music Klasse

- `sequences`<br>
Two-dimensional list of sounds (0-63) listed by the number of channels

- `set(seq0, seq1, seq2, seq3)`<br>
Set the lists of sound (0-63) of all channels. If an empty list is specified, that channel is not used for playback.<br>
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### Advanced APIs

Pyxel has "advanced APIs" that are not mentioned in this reference because they "may confuse users" or "need specialized knowledge to use".

If you are familiar with your skills, try to create amazing works with [this](pyxel/__init__.pyi) as a clue!

## Wie du beitragen kannst

### Submitting an Issue

Use the [Issue Tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature/enhancement requests. Before submitting a new issue, ensure that there is no similar open issue.

### Manual Testing

Anyone manually testing the code and reporting bugs or suggestions for enhancements in the [Issue Tracker](https://github.com/kitao/pyxel/issues) are very welcome!

### Submitting a Pull Request

Patches/Fixes werden in Form von Pull Requests (PRs) akzeptiert. Stellen Sie sicher, dass das Problem, auf das sich der Pull Request bezieht, im Issue Tracker offen ist.

Bei einem eingereichten Pull-Request wird davon ausgegangen, dass der Veröffentlichung unter der [MIT Lizenz](LICENSE) zugestimmt wird.

## Other Information

- [Discord Server (English)](https://discord.gg/FC7kUZJ)
- [Discord Server (Japanisch - 日本語版)](https://discord.gg/qHA5BCS)

## License

Pyxel is under [MIT License](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software, provided that all copies of the software or its substantial portions include a copy of the terms of the MIT License and also a copyright notice.
