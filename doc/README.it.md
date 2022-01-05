# <img src="images/pyxel_logo_152x64.png">

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**NOTE: This manual has not yet been translated for Pyxel version 1.5.0. We are looking for volunteers to translate and check for mistakes!**

**Pyxel** è un game engine rétro per Python.

Grazie alle sue specifiche limitate ispirate dalle console di videogiochi rétro, come al fatto che solo 16 colori possono essere mostrati e solo 4 suoni possono essere suonati allo stesso tempo, puoi sentirti libero di creare giochi stile pixel art.

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

Pyxel è open source e libero da usare. Cominciamo a fare giochi rétro con Pyxel!

## Specifiche

- Funziona su Windows, Mac, e Linux
- Programming with Python
- 16 color palette
- 3 banche di immagini di dimensioni 256x256
- 8 tilemap di dimensioni 256x256
- 4 canali con 64 suoni definibili
- 8 musiche che possono combinare suoni arbitrari
- Input di tastiera, mouse, e controller
- Editor suoni e immagini

### Palette colori

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Come installare

There are two types of Pyxel, a packaged version and a standalone version.

### Install Packaged Version

The packaged version of Pyxel uses Pyxel as a Python extension module.

Recommended for those who are familiar with managing Python packages using the `pip` command or who want to develop full-fledged Python applications.

**Windows**

After installing [Python3](https://www.python.org/) (version 3.7 or higher), run the following command:

```sh
pip install pyxel
```

**Mac**

After installing [Python3](https://www.python.org/) (version 3.7 or higher), run the following command:

```sh
pip3 install pyxel
```

**Linux**

After installing the SDL2 package (`libsdl2-dev` for Ubuntu), [Python3](https://www.python.org/) (version 3.7 or higher), and `python3-pip`, run the following command:

```sh
sudo pip3 install pyxel
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

Dopo aver installato Pyxel, gli esempi di Pyxel saranno copiati nella corrente cartella con il comando seguente:

```sh
pyxel copy_examples
```

Gli esempi da copiare sono i seguenti:

- [01_hello_pyxel.py](../pyxel/examples/01_hello_pyxel.py) - Applicazione più semplice
- [02_jump_game.py](../pyxel/examples/02_jump_game.py) - Un gioco di salto con file Pyxel di risorsa
- [03_draw_api.py](../pyxel/examples/03_draw_api.py) - Demonstration of drawing APIs
- [04_sound_api.py](../pyxel/examples/04_sound_api.py) - Demonstration of sound APIs
- [05_color_palette.py](../pyxel/examples/05_color_palette.py) - Lista di colori nella palette
- [06_click_game.py](../pyxel/examples/06_click_game.py) - Gioco punta e clicca
- [07_snake.py](../pyxel/examples/07_snake.py) - Gioco snake con colonna sonora
- [08_triangle_api.py](../pyxel/examples/08_triangle_api.py) - Demonstration of triangle drawing APIs
- [09_shooter.py](../pyxel/examples/09_shooter.py) - Gioco shoot'em up con transizioni schermo
- [10_platformer.py](../pyxel/examples/10_platformer.py) - Side-scrolling platform game with map
- [11_offscreen.py](../pyxel/examples/11_offscreen.py) - Offscreen rendering with Image class

An examples can be executed with the following commands:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
```

## Come usare

### Creare una applicazione Pyxel

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

I parametri della funzione `run` sono passate alla funzione `update` per aggiornare ogni frame e alla funzione `draw` per disegnare lo schermo quando necessario.

In una effettiva applicazione, è consigliato ricoprire codice pyxel in una classe come qui sotto:

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

### Controlli speciali

I controlli seguenti speciali possono essere eseguite mentre viene eseguita un'applicazione Pyxel:

- `Esc`<br>
Esci dall'applicazione
- `Alt(Option)+1`<br>
Salva uno screenshot sul desktop
- `Alt(Option)+2`<br>
Resetta il tempo d'inizio della registrazione schermo
- `Alt(Option)+3`<br>
Salva la registrazione schermo sul desktop (fino a 10 secondi)
- `Alt(Option)+0`<br>
Alterna il monitor di performance (fps, tempo d'aggiornamento, e tempo di disegno)
- `Alt(Option)+Enter`<br>
Alterna schermo intero

### Come creare una risorsa

Pyxel Editor can create images and sounds used in a Pyxel application.

It starts with the following command:

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

Se il file di risorsa Pyxel (.pyxres) specificato esiste, allora il file viene caricato, e se non esiste, un nuovo file con quel nome viene creato.
Se il file risorsa viene omesso, il nome è `my_resource.pyxres`.

After starting Pyxel Editor, the file can be switched by dragging and dropping another resource file. If the resource file is dragged and dropped while holding down ``Ctrl(Cmd)`` key, only the resource type (Image/Tilemap/Sound/Music) that is currently being edited will be loaded. This operation enables to combine multiple resource files into one.

The created resource file can be loaded with `load` function.

L'editor Pyxel ha le seguenti modalità di modifica.

**Editor Immagini:**

La modalità per modificare banche d'immagini.

<img src="images/image_editor.gif">

By dragging and dropping an image file (png/gif/jpeg) onto the Image Editor screen, the image can be loaded into the currently selected image bank.

**Editor Tilemap:**

La modalità per modificare tilemap immagini delle banche immagini sono posizionate in un modo a piastrelle.

<img src="images/tilemap_editor.gif">

**Editor Suoni:**

Modalità per modificare suoni.

<img src="images/sound_editor.gif">

**Editor Musica:**

La modalità per modificare musica in cui i suoni sono posizionati in ordine per poi essere risuonati.

<img src="images/music_editor.gif">

### Altri metodi per creare risorse

Pyxel images and tilemaps can also be created by the following methods:

- Create an image from a list of strings with `Image.set` function or `Tilemap.set` function
- Load an image file (png/gif/jpeg) in Pyxel palette with `Image.load` function

Pyxel sounds can also be created in the following method:

- Create a sound from strings with `Sound.set` function or `Music.set` function

Riferirsi al manuale dell'API per l'uso di queste funzioni.

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

## Manuale API

### Sistema

- `width`, `height`<br>
Lunghezza e altezza dello schermo

- `frame_count`<br>
Numero di frame passati

- `init(width, height, [title], [fps], [quit_key], [capture_scale], [capture_sec])`<br>
Initialize the Pyxel application with screen size (`width`, `height`). The following can be specified as options: the window title with `title`, the frame rate with `fps`, the key to quit the application with `quit_key`, the scale of the screen capture with `capture_scale`, and the maximum recording time of the screen capture video with `capture_sec`.<br>
e.g. `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
Start the Pyxel application and call `update` function for frame update and `draw` function for drawing.

- `show()`<br>
Show the screen and wait until the `Esc` key is pressed. (Do not use in normal applications)

- `flip()`<br>
Updates the screen once. (Do not use in normal applications)

- `quit()`<br>
Quit the Pyxel application.

### Risorse

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Load the resource file (.pyxres). If ``False`` is specified for the resource type (``image/tilemap/sound/music``), the resource will not be loaded.

### Input
- `mouse_x`, `mouse_y`<br>
La posizione corrente del cursore del mouse

- `mouse_wheel`<br>
Il valore corrente della rotella del mouse

- `btn(key)`<br>
Ritorna `True` se `key` è premuto, altrimenti ritorna `False` ([lista definizione tasti](../pyxel/__init__.pyi))

- `btnp(key, [hold], [period])`<br>
Ritorna `True` se `key` è premuto quel frame, altrimenti ritorna `False`. Quando `hold` e `period` sono specificati, `True` sarà ritornato all'intervallo frame `period` quando `key` è premuto per più di `hold` frame

- `btnr(key)`<br>
Ritorna `True` se `key` è rilasciato quel frame, altrimenti ritorna `False`

- `mouse(visible)`<br>
Se `visible` è `True`, mostra il cursore mouse. Se `False`, nascondilo. Anche se il cursore mouse non è mostrato, la sua posizione è aggiornata.

### Grafica

- `colors`<br>
List of the palette display colors. The display color is specified by a 24-bit numerical value. Use `colors.from_list` and `colors.to_list` to directly assign and retrieve Python lists.<br>
e.g. `org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img)`<br>
Operate the image bank `img` (0-2). (See the Image class)<br>
e.g. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
Opera la tilemap `tm`(0-7) (vedere la classe Tilemap)

- `clip(x, y, w, h)`<br>
Imposta l'area di disegno dello schermo da (`x`, `y`) a lunghezza `w` e altezza `h`. Resettare l'area di disegno a schermo intero con `clip()`

- `camera(x, y)`<br>
Change the upper left corner coordinates of the screen to (`x`, `y`). Reset the upper left corner coordinates to (`0`, `0`) with `camera()`.

- `pal(col1, col2)`<br>
Rimpiazza colore `col1` con `col2` al momento di disegno. `pal()` per tornare alla palette iniziale

- `cls(col)`<br>
Riempie lo schermo con `col`

- `pget(x, y)`<br>
Ritorna il colore del pixel su (`x`, `y`)

- `pset(x, y, col)`<br>
Disegna un pixel di colore `col` su (`x`, `y`)

- `line(x1, y1, x2, y2, col)`<br>
Disegna una linea di colore `col` da (`x1`, `y1`) a (`x2`, `y2`)

- `rect(x, y, w, h, col)`<br>
Disegna un rettangolo con lunghezza `w`, altezza `h` e colore `col` da (`x`, `y`)

- `rectb(x, y, w, h, col)`<br>
Disegna il contorno di un rettangolo di lunghezza `w`, altezza `h` e colore `col` da (`x`, `y`)

- `circ(x, y, r, col)`<br>
Disegna un cerchio di raggio `r` e colore `col` su (`x`, `y`)

- `circb(x, y, r, col)`<br>
Disegna il contorno di un cerchio di raggio `r` e colore `col` su (`x`, `y`)

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
Disegna un triangolo con vertici (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e colore `col`

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
Disegna il contorno di un triangolo con vertici (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e colore `col`

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
Copia la regione di grandezza (`w`, `h`) da (`u`, `v`) della banca immagini `img`(0-2) a (`x`, `y`). Se un valore negativo è impostato per `w` e/o `h`, sarà invertito orizzontalmente o verticalmente. Se `colkey` è specificato, verrà trattato come colore trasparente

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Copy the region of size (`w`, `h`) from (`u`, `v`) of the tilemap `tm` (0-7) to (`x`, `y`). If negative value is set for `w` and/or `h`, it will reverse horizontally and/or vertically. If `colkey` is specified, treated as transparent color. The size of a tile is 8x8 pixels and is stored in a tilemap as a tuple of `(tile_x, tile_y)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
Disegna una stringa `s` di colore `col` su (`x`, `y`)

### Audio

- `sound(snd)`<br>
Opera il suono `snd`(0-63). (Vedere classe Sound).<br>
per esempio: `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Opera la musica `msc`(0-7) (vedere la classe Music)

- `play_pos(ch)`<br>
Get the sound playback position of channel `ch` (0-3) as a tuple of `(sound no, note no)`. Returns `None` when playback is stopped.

- `play(ch, snd, loop=False)`<br>
Play the sound `snd` (0-63) on channel `ch` (0-3). If `snd` is a list, it will be played in order. If `True` is specified for `loop`, loop playback is performed.

- `playm(msc, loop=False)`<br>
Play the music `msc` (0-7). If `True` is specified for `loop`, loop playback is performed.

- `stop([ch])`<br>
Stops playback of the specified channel `ch` (0-3). `stop()` to stop playing all channels.

### Image Class

- `width`, `height`<br>
La lunghezza e l'altezza dell'immagine

- `data`<br>
I dati dell'immagine (lista bidimensionale da 256x256)

- `get(x, y)`<br>
Trova i dati dell'immagine su (`x`, `y`)

- `set(x, y, data)`<br>
Set the image at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.image(0).set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
Load the image file (png/gif/jpeg) at (`x`, `y`).

### Classe Tilemap

- `width`, `height`<br>
Lunghezza e altezza della tilemap

- `refimg`<br>
The image bank (0-2) referenced by the tilemap

- `set(x, y, data)`<br>
Set the tilemap at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `pget(x, y)`<br>
Get the tile at (`x`, `y`). A tile is a tuple of `(tile_x, tile_y)`.

- `pset(x, y, tile)`<br>
Draw a `tile` at (`x`, `y`). A tile is a tuple of `(tile_x, tile_y)`.

### Classe Sound

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

### Music Class

- `sequences`<br>
Two-dimensional list of sounds (0-63) listed by the number of channels

- `set(seq0, seq1, seq2, seq3)`<br>
Set the lists of sound (0-63) of all channels. If an empty list is specified, that channel is not used for playback.<br>
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### Advanced APIs

Pyxel has "advanced APIs" that are not mentioned in this reference because they "may confuse users" or "need specialized knowledge to use".

If you are familiar with your skills, try to create amazing works with [this](../pyxel/__init__.pyi) as a clue!

## Come contribuire

### Submitting Issue

Use the [Issue Tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature/enhancement requests. Before submitting a new issue, ensure that there is no similar open issue.

### Manual Testing

Anyone manually testing the code and reporting bugs or suggestions for enhancements in the [Issue Tracker](https://github.com/kitao/pyxel/issues) are very welcome!

### Submitting Pull Request

Patch/fix sono accettati in forma di pull request (PR). Assicurarsi che il problema per cui si emetta una pull request sia aperto nel tracciante di problemi.

Le pull request emesse sono presupposte di accettare di essere pubblicate sotto la [licenza MIT](../LICENSE).

## Altre informazioni

- [Server Discord (Inglese)](https://discord.gg/FC7kUZJ)
- [Server Discord (Giapponese - 日本語版)](https://discord.gg/qHA5BCS)

## Licenza

Pyxel is under [MIT License](../LICENSE). It can be reused within proprietary software, provided that all copies of the software or its substantial portions include a copy of the terms of the MIT License and also a copyright notice.
