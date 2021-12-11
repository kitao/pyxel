# <img src="images/pyxel_logo_152x64.png">

[ [English](README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**NOTE: This manual has not yet been translated for Pyxel version 1.5.0. We are looking for volunteers to translate and check for mistakes!**

**Pyxel** est un moteur de jeu-vidéo rétro pour Python.

Grâce à ses spécifications simples inspirées par les consoles rétro, comme le fait que seulement 16 couleurs peuvent être affichées et que seulement 4 sons peuvent être joués en même temps, vous pouvez vous sentir libres de créer des jeux-vidéos dans le style pixel art.

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

Pyxel est open source et libre d’usage. Commençons à faire un jeu-vidéo rétro avec Pyxel !

## Spécifications

- Fonctionne sous Windows, Mac et Linux
- Programming with Python
- 16 color palette
- 3 banques d’images de taille 256x256
- 8 tilemaps de taille 256x256
- 4 canaux avec 64 sons configurables
- 8 musiques pouvant combiner des sons arbitraires
- Entrées clavier, souris et manettes
- Éditeur d’images et de sons

### Palette de couleurs

<img src="pyxel/examples/screenshots/05_color_palette.png">
<br><br>
<img src="images/pyxel_palette.png">

## Comment installer

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

After installing the SDL2 package (`libsdl2-dev` for Ubuntu), [Python3](https://www.python.org/) (version 3.7 or higher), and `python3-pip`, run the following command:

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

### Installez les exemples

Après l’installation de Pyxel, les exemples de Pyxel seront copiés dans le répertoire courant avec la commande suivante :

```sh
pyxel copy_examples
```

Les exemples copiés sont les suivants :

- [01_hello_pyxel.py](pyxel/examples/01_hello_pyxel.py) - Application simple
- [02_jump_game.py](pyxel/examples/02_jump_game.py) - Jeu de saut avec les fichiers de ressources Pyxel
- [03_draw_api.py](pyxel/examples/03_draw_api.py) - Demonstration of drawing APIs
- [04_sound_api.py](pyxel/examples/04_sound_api.py) - Demonstration of sound APIs
- [05_color_palette.py](pyxel/examples/05_color_palette.py) - Liste des couleurs de la palette
- [06_click_game.py](pyxel/examples/06_click_game.py) - Jeu de point and click
- [07_snake.py](pyxel/examples/07_snake.py) - Jeu du Snake avec une bande son
- [08_triangle_api.py](pyxel/examples/08_triangle_api.py) - Demonstration of triangle drawing APIs
- [09_shooter.py](pyxel/examples/09_shooter.py) - Jeu de shoot'em up avec changement d’écran
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

## Comment utiliser

### Créer une application Pyxel

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

Les arguments de la fonction `run` sont la fonction `update` pour mettre à jour chaque frame et la fonction `draw` pour dessiner sur l’écran quand c’est nécessaire.

Dans une vraie application, il est recommandé de mettre le code pyxel dans une classe comme ci-dessous :

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

### Contrôles spéciaux

Les contrôles spéciaux suivants peuvent être lancés pendant qu’une application Pyxel tourne :

- `Esc`<br>
Quitte l’application
- `Alt(Option)+1`<br>
Sauvegarde la capture d’écran sur le bureau
- `Alt(Option)+2`<br>
Réinitialise le temps de départ de la capture vidéo
- `Alt(Option)+3`<br>
Sauvegarde la capture d’écran sur le bureau (jusqu’à 10 secondes)
- `Alt(Option)+0`<br>
Bascule vers le moniteur de performance (fps, temps de mise à jour et temps de dessin)
- `Alt(Option)+Enter`<br>
Met en plein écran

### Comment créer une ressource

Pyxel Editor can create images and sounds used in a Pyxel application.

It starts with the following command:

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

Si le fichier de ressource Pyxel (.pyxres) existe déjà, le fichier est chargé, sinon, un nouveau fichier avec le nom indiqué est créé.
Si le fichier de ressource n’est pas spécifié, le nom est `my_resource.pyxres`.

After starting Pyxel Editor, the file can be switched by dragging and dropping another resource file. If the resource file is dragged and dropped while holding down ``Ctrl(Cmd)`` key, only the resource type (Image/Tilemap/Sound/Music) that is currently being edited will be loaded. This operation enables to combine multiple resource files into one.

The created resource file can be loaded with `load` function.

L’éditeur Pyxel a les modes suivants.

**Éditeur d’images :**

Mode pour éditer la banque d’images.

<img src="pyxel/editor/screenshots/image_editor.gif">

By dragging and dropping an image file (png/gif/jpeg) onto the Image Editor screen, the image can be loaded into the currently selected image bank.

**Éditeur de tilemap :**

Mode pour éditer les tilemaps dans lesquelles les images des banques d’images sont ordonnées en motif de tuiles.

<img src="pyxel/editor/screenshots/tilemap_editor.gif">

**Éditeur de sons :**

Mode pour éditer les sons.

<img src="pyxel/editor/screenshots/sound_editor.gif">

**Éditeur de musiques :**

Mode pour éditer les musiques dans lesquelles les sons sont ordonnés par ordre de lecture.

<img src="pyxel/editor/screenshots/music_editor.gif">

### Autres méthodes pour créer des ressources

Pyxel images and tilemaps can also be created by the following methods:

- Create an image from a list of strings with `Image.set` function or `Tilemap.set` function
- Load an image file (png/gif/jpeg) in Pyxel palette with `Image.load` function

Pyxel sounds can also be created in the following method:

- Create a sound from strings with `Sound.set` function or `Music.set` function

Référez vous à la documentation de l’API pour l’utilisation de ces fonctions.

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

## Documentation de l’API

### Système

- `width`, `height`<br>
La largeur et la hauteur de l’écran

- `frame_count`<br>
Le nombre de frames passées

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

### Ressources

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Load the resource file (.pyxres). If ``False`` is specified for the resource type (``image/tilemap/sound/music``), the resource will not be loaded.

### Entrées
- `mouse_x`, `mouse_y`<br>
La position actuelle du curseur de la souris

- `mouse_wheel`<br>
La valeur actuelle de la molette de la souris

- `btn(key)`<br>
Renvoie `True` si la touche `key` est appuyée, sinon renvoie `False` ([liste des touches](pyxel/__init__.pyi))

- `btnp(key, [hold], [period])`<br>
Renvoie `True` si la touche `key` est appuyée à cette frame, sinon renvoie `False`. Quand `hold` et `period` sont spécifiés, `True` sera renvoyé à l’intervalle de frame `period` quand la touche `key` est appuyée pendant plus de `hold` frames

- `btnr(key)`<br>
Renvoie `True` si la touche `key` est appuyée à cette frame, sinon renvoie `False`

- `mouse(visible)`<br>
Si `visible` est `True`, affiche le curseur de la souris. Si `False`, le curseur est caché. Même si le curseur n’est pas affiché, sa position est actualisée.

### Graphiques

- `colors`<br>
List of the palette display colors. The display color is specified by a 24-bit numerical value. Use `colors.from_list` and `colors.to_list` to directly assign and retrieve Python lists.<br>
e.g. `org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img)`<br>
Operate the image bank `img` (0-2). (See the Image class)<br>
e.g. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
Utilise la tilemap `tm`(0-7) (voir la classe Tilemap)

- `clip(x, y, w, h)`<br>
Défini la zone de dessin (`x`, `y`) avec une largeur `w` et une hauteur `h`. Réinitialiser la zone de dessin au plein écran avec `clip()`

- `pal(col1, col2)`<br>
Remplace la couleur `col1` avec `col2` au dessin. `pal()` pour réinitialiser la palette de couleurs

- `cls(col)`<br>
Efface l’écran avec la couleur `col`

- `pget(x, y)`<br>
Renvoie la couleur au pixel (`x`, `y`)

- `pset(x, y, col)`<br>
Dessine un pixel de couleur `col` à (`x`, `y`)

- `line(x1, y1, x2, y2, col)`<br>
Dessine une ligne de couleur `col` de (`x1`, `y1`) à (`x2`, `y2`)

- `rect(x, y, w, h, col)`<br>
Dessine un rectangle de largeur `w`, de hauteur `h` et de couleur `col` à partir de (`x`, `y`)

- `rectb(x, y, w, h, col)`<br>
Dessine les contours d’un rectangle de largeur `w`, de hauteur `h` et de couleur `col` à partir de (`x`, `y`)

- `circ(x, y, r, col)`<br>
Dessine un cercle de rayon `r` et de couleur `col` à (`x`, `y`)

- `circb(x, y, r, col)`<br>
Dessine le contour d’un cercle de rayon `r` et de couleur `col` à (`x`, `y`)

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
Dessine un triangle avec les sommets (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) et de couleur `col`

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
Dessine les contours d’un triangle avec les sommets (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) et de couleur `col`

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
Copie la région de taille (`w`, `h`) de (`u`, `v`) de la banque d’image `img`(0-2) à (`x`, `y`). Si une valeur négative est mise pour `w` et/ou `h`, la copie sera inversée horizontalement et/ou verticalement. Si `colkey` est spécifié, il sera traité comme une couleur transparente

<img src="images/image_bank_mechanism.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Draw the tilemap `tm` (0-7) to (`x`, `y`) according to the tile information of size (`w`, `h`) from (`u`, `v`). If `colkey` is specified, treated as transparent color. The size of a tile is 8x8 pixels and is stored in a tilemap as a tuple of `(x in tile, y in tile)`.

- `text(x, y, s, col)`<br>
Dessine une chaîne de caractères `s` de couleur `col` à (`x`, `y`)

### Audio

- `sound(snd, [system])`<br>
Utilise le son `snd`(0-63) (voir la classe Sound). Si `system` est `True`, le son 64 pour le système est accessible<br>
par exemple : `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Utilise la musique `msc`(0-7) (voir la classe Music)

- `play_pos(ch)`<br>
Get the sound playback position of channel `ch` (0-3) as a tuple of `(sound no, note no)`. Returns `None` when playback is stopped.

- `play(ch, snd, loop=False)`<br>
Play the sound `snd` (0-63) on channel `ch` (0-3). If `snd` is a list, it will be played in order. If `True` is specified for `loop`, loop playback is performed.

- `playm(msc, loop=False)`<br>
Play the music `msc` (0-7). If `True` is specified for `loop`, loop playback is performed.

- `stop([ch])`<br>
Stops playback of the specified channel `ch` (0-3). `stop()` to stop playing all channels.

### Classe Image

- `width`, `height`<br>
La largeur et la hauteur d’une image

- `data`<br>
Les données de l’image (liste bi-dimentionelle de 256x256)

- `get(x, y)`<br>
Renvoie les données de l’image à (`x`, `y`)

- `set(x, y, data)`<br>
Set the image at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Load the image file (png/gif/jpeg) at (`x`, `y`).

### Classe Tilemap

- `width`, `height`<br>
La largeur et la hauteur de la tilemap

- `refimg`<br>
The image bank (0-2) referenced by the tilemap

- `set(x, y, data)`<br>
Set the tilemap at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `pget(x, y)`<br>
Get the tile at (`x`, `y`). A tile is a tuple of `(x in tile, y in tile)`.

- `pset(x, y, tile)`<br>
Draw a `tile` at (`x`, `y`). A tile is a tuple of `(x in tile, y in tile)`.

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

### Classe Music

- `sequences`<br>
Two-dimensional list of sounds (0-63) listed by the number of channels

- `set(seq0, seq1, seq2, seq3)`<br>
Set the lists of sound (0-63) of all channels. If an empty list is specified, that channel is not used for playback.<br>
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### Advanced APIs

Pyxel has "advanced APIs" that are not mentioned in this reference because they "may confuse users" or "need specialized knowledge to use".

If you are familiar with your skills, try to create amazing works with [this](pyxel/__init__.pyi) as a clue!

## Comment contribuer

### Submitting an Issue

Use the [Issue Tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature/enhancement requests. Before submitting a new issue, ensure that there is no similar open issue.

### Manual Testing

Anyone manually testing the code and reporting bugs or suggestions for enhancements in the [Issue Tracker](https://github.com/kitao/pyxel/issues) are very welcome!

### Submitting a Pull Request

Les patchs/fixs sont acceptés sous forme de pull requests (PRs). Faites attention à ce que le ticket que la pull request corrige soit ouvert.

En proposant une pull request, vous acceptez qu’elle soit publiée sous la [licence MIT](LICENSE).

## Autres informations

- [Serveur Discord (Anglais)](https://discord.gg/FC7kUZJ)
- [Serveur Discord (Japonais - 日本語版)](https://discord.gg/qHA5BCS)

## License

Pyxel is under [MIT License](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software, provided that all copies of the software or its substantial portions include a copy of the terms of the MIT License and also a copyright notice.
