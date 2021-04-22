# <img src="images/pyxel_logo_152x64.png">

[ [English](README.md) | [中文](README.cn.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

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

Les spécifications de la console de jeux et des APIs pour Pyxel se réfèrent aux incroyables [PICO-8](https://www.lexaloffle.com/pico-8.php) et [TIC-80](https://tic.computer/).

Pyxel est open source et libre d’usage. Commençons à faire un jeu-vidéo rétro avec Pyxel !

## Spécifications

- Fonctionne sous Windows, Mac et Linux
- Code écrit en Python3
- Palette de 16 couleurs fixée
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

### Windows

D’abord, installez [Python3](https://www.python.org/) (version 3.6.8 ou plus).

Quand vous installez Python depuis l’installeur officiel, **ajouter Python au PATH** en cochant le bouton suivant :

<img src="images/python_installer.png">

Ensuite, installez Pyxel avec la commande `pip` suivante dans le terminal :

```sh
pip install -U pyxel
```

### Mac

D’abord, dans l’environnement où le gestionnaire de paquets [Homebrew](https://brew.sh/) est installé, installez [Python3](https://www.python.org/) (version 3.6.8 ou plus) et les paquets requis avec la commande suivante :

```sh
brew install python3 gcc sdl2 sdl2_image gifsicle
```

Vous pouvez installer Python3 de d’autres façon, mais vous devez installer d’autres librairies.

Ensuite, **relancez le terminal** et installez Pyxel avec la commande `pip3` suivante :

```sh
pip3 install -U pyxel
```

### Linux

Installez [Python3](https://www.python.org/) (version 3.6.8 ou plus) et les paquets requis suivant la distribution que vous utilisez.

**Ubuntu:**

```sh
sudo apt install python3 python3-pip libsdl2-dev libsdl2-image-dev gifsicle
sudo -H pip3 install -U pyxel
```

### Autre environment

Pour installer Pyxel dans un environnement autre que ceux vu précédement (Linux 32-bit, Raspberry PI, etc.), suivez les étapes suivantes pour la compilation :

#### Installez les paquets et outils nécessaires

- C++ build toolchain (doit inclure les commandes gcc et make)
- libsdl2-dev et libsdl2-image-dev
- [Python3](https://www.python.org/) (version 3.6.8 ou plus) et la commande pip

#### Éxecutez la commande suivant dans n’importe quel dossier

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make -C pyxel/core clean all
pip3 install .
```

### Installez les exemples

Après l’installation de Pyxel, les exemples de Pyxel seront copiés dans le répertoire courant avec la commande suivante :

```sh
install_pyxel_examples
```

Les exemples copiés sont les suivants :

- [01_hello_pyxel.py](pyxel/examples/01_hello_pyxel.py) - Application simple
- [02_jump_game.py](pyxel/examples/02_jump_game.py) - Jeu de saut avec les fichiers de ressources Pyxel
- [03_draw_api.py](pyxel/examples/03_draw_api.py) - Démonstration de l’API de dessin
- [04_sound_api.py](pyxel/examples/04_sound_api.py) - Démonstration de l’API de son
- [05_color_palette.py](pyxel/examples/05_color_palette.py) - Liste des couleurs de la palette
- [06_click_game.py](pyxel/examples/06_click_game.py) - Jeu de point and click
- [07_snake.py](pyxel/examples/07_snake.py) - Jeu du Snake avec une bande son
- [08_triangle_api.py](pyxel/examples/08_triangle_api.py) - Démonstration de l’API de dessin de triangles
- [09_shooter.py](pyxel/examples/09_shooter.py) - Jeu de shoot'em up avec changement d’écran

Les exemples peuvent être exécutés comme du code Python normal :

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

## Comment utiliser

### Créer une application Pyxel

Après avoir importé le module Pyxel dans votre code python, spécifiez d’abord la taille de la fenêtre avec la fonction `init`, puis lancez l’application Pyxel avec la fonction `run`.

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

Il est aussi possible d’écrire du code simple en utilisant les fonctions `show` et `flip` pour dessiner des formes simples et des animations.

La fonction `show` affiche l’écran et attend jusqu’à ce que la touche `ESC` soit appuyée.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

La fonction `flip` met à jour une fois l’écran.

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
Sauvegarde la capture d’écran (gif) sur le bureau (jusqu’à 30 secondes)
- `Alt(Option)+0`<br>
Bascule vers le moniteur de performance (fps, temps de mise à jour et temps de dessin)
- `Alt(Option)+Enter`<br>
Met en plein écran

### Comment créer une ressource

L’éditeur Pyxel inclus peut créer des images et des sons utilisables dans une application Pyxel.

L’éditeur Pyxel se lance avec la commande suivante :

```sh
pyxeleditor [pyxel_resource_file]
```

Si le fichier de ressource Pyxel (.pyxres) existe déjà, le fichier est chargé, sinon, un nouveau fichier avec le nom indiqué est créé.
Si le fichier de ressource n’est pas spécifié, le nom est `my_resource.pyxres`.

Après avoir lancé l’éditeur Pyxel, le fichier peut être échangé en glissant-déposant un autre fichier de resource. Si le fichier de ressource est glissé-déposé en maintenant appuyé la touche ``Ctrl``(``Cmd``), uniquement la ressource du type (image, tilemap, son, musique) de celle en cours d’édition sera chargée. Cette opération permet de combiner plusieurs fichiers de ressource en un.

Le fichier de ressource créé peut être chargé en utilisant la fonction `load`.

L’éditeur Pyxel a les modes suivants.

**Éditeur d’images :**

Mode pour éditer la banque d’images.

<img src="pyxel/editor/screenshots/image_editor.gif">

En glissant-déposant un fichier png dans l’écran de l’éditeur d’imagese l’image peut être chargée dans la banque d’images sélectionnée.

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

Les images et tilemaps Pyxel peuvent être aussi créé des manières suivantes :

- Créez une image depuis une liste de chaînes de caractères avec la fonction `Image.set` ou `Tilemap.set`
- Chargez un fichier png dans une palette Pyxel avec la fonction `Image.load`

Les sons peuvent être aussi créé de la manière suivante :

- Créez des sons depuis des chaînes de caractères avec la fonction `Sound.set` ou `Music.set`

Référez vous à la documentation de l’API pour l’utilisation de ces fonctions.

### Comment créer un exécutable stand-alone

En utilisant le Pyxel Packager inclus, un exécutable stand-alone pouvant fonctionner dans des environnements où Python n’est pas installé peut être créé.

Pour créer un exécutable stand-alone, dans l’environnement où est installé [PyInstaller](https://www.pyinstaller.org/), spécifiez le fichier Python à utiliser pour lancer l’application avec la commande `pyxelpackager` suivante :

```sh
pyxelpackager python_file
```

Quand le processus est terminé, un exécutable stand-alone est créé dans le dossier `dist`.

Si des ressources comme des fichiers .pyxres ou .png sont nécessaires, mettez les dans le dossier `assets` et ils seront automatiquement inclus.

Il est aussi possible de spécifier une icône avec l’option ``-i icon_file``.

## Documentation de l’API

### Système

- `width`, `height`<br>
La largeur et la hauteur de l’écran

- `frame_count`<br>
Le nombre de frames passées

- `init(width, height, [caption], [scale], [palette], [fps], [quit_key], [fullscreen])`<br>
Initialise l’application Pyxel avec une taille d’écran (`width`, `height`). La largeur et la hauteur maximale sont 256.<br>
Il est possible de définir un titre à la fenêtre avec `caption`, l’agrandissement de la fenêtre avec `scale`, la palette de couleurs avec `palette`, la fréquence de frames avec `fps`, la touche pour quitter l’application avec `quit_key`, et le lancement en plein écran avec `fullscreen`. `palette` est défini comme une liste de 16 éléments de couleur 24 bits.<br>
par exemple : `pyxel.init(160, 120, caption="Pyxel with PICO-8 palette", palette=[0x000000, 0x1D2B53, 0x7E2553, 0x008751, 0xAB5236, 0x5F574F, 0xC2C3C7, 0xFFF1E8, 0xFF004D, 0xFFA300, 0xFFEC27, 0x00E436, 0x29ADFF, 0x83769C, 0xFF77A8, 0xFFCCAA], quit_key=pyxel.KEY_NONE, fullscreen=True)`

- `run(update, draw)`<br>
Lance l’application et appelle la fonction `update` pour la mise à jour de la frame et la fonction `draw` pour le dessin.

- `quit()`<br>
Quitte l’application Pyxel à la fin de frame en cours

- `flip()`<br>
Force le dessin de l’écran (à ne pas utiliser dans des applications normales)

- `show()`<br>
Dessine l’écran et attend indéfiniment (à ne pas utiliser dans des applications normales)

### Ressources

- `save(filename)`<br>
Sauvegarde le fichier ressource (.pyxres) dans le répertoire d’exécution du script

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Lit le fichier de ressource (.pyxres) du répertoire d’exécution du script. Si ``False`` est spécifié pour le type de ressource, la ressource (image, tilemap, son, musique) ne sera pas chargée.

### Entrées
- `mouse_x`, `mouse_y`<br>
La position actuelle du curseur de la souris

- `mouse_wheel`<br>
La valeur actuelle de la molette de la souris

- `btn(key)`<br>
Renvoie `True` si la touche `key` est appuyée, sinon renvoie `False` ([liste des touches](pyxel/__init__.py))

- `btnp(key, [hold], [period])`<br>
Renvoie `True` si la touche `key` est appuyée à cette frame, sinon renvoie `False`. Quand `hold` et `period` sont spécifiés, `True` sera renvoyé à l’intervalle de frame `period` quand la touche `key` est appuyée pendant plus de `hold` frames

- `btnr(key)`<br>
Renvoie `True` si la touche `key` est appuyée à cette frame, sinon renvoie `False`

- `mouse(visible)`<br>
Si `visible` est `True`, affiche le curseur de la souris. Si `False`, le curseur est caché. Même si le curseur n’est pas affiché, sa position est actualisée.

### Graphiques

- `image(img, [system])`<br>
Utilise la banque d’image `img`(0-2) (voir la classe Image). Si `system` est `True`, la banque d’images du système est accessible. 3 est pour la fonte et l’éditeur de ressources. 4 est pour l’affichage de l’écran<br>
par exemple : `pyxel.image(0).load(0, 0, "title.png")`

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
Dessine la tilemap `tm`(0-7) de (`x`, `y`) suivant les information sur la taille de la tuille (`w`, `h`) à (`u`, `v`). Si `colkey` est spécifié, il sera traité comme une couleur transparente. Une tuille d’une tilemap est dessinée avec une taille de 8x8, et si le numéro de la tuile est 0, il indique la région (0, 0)-(7, 7) de la banque d’images, si c’est 1, il indique (8, 0)-(15, 0)

<img src="images/tilemap_mechanism.png">

- `text(x, y, s, col)`<br>
Dessine une chaîne de caractères `s` de couleur `col` à (`x`, `y`)

### Audio

- `sound(snd, [system])`<br>
Utilise le son `snd`(0-63) (voir la classe Sound). Si `system` est `True`, le son 64 pour le système est accessible<br>
par exemple : `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Utilise la musique `msc`(0-7) (voir la classe Music)

- `play_pos(ch)`<br>
Renvoie la position de lecture du son du canal `ch`. Le 100ème et le 1000ème indique le numéro du son et le 1er et le 10ème indique le numéro de la note. Quand la liste de lecture est arrêtée, renvoie `-1`

- `play(ch, snd, loop=False)`<br>
Joue le son `snd`(0-63) sur le canal `ch`(0-3). Joue dans l’ordre quand `snd` est une liste

- `playm(msc, loop=False)`<br>
Joue la musique `msc`(0-7)

- `stop([ch])`<br>
Arrête la liste de lecture sur tous les canaux. Si `ch`(0-3) est défini, arrête la liste de lecture uniquement sur le canal correspondant

### Classe Image

- `width`, `height`<br>
La largeur et la hauteur d’une image

- `data`<br>
Les données de l’image (liste bi-dimentionelle de 256x256)

- `get(x, y)`<br>
Renvoie les données de l’image à (`x`, `y`)

- `set(x, y, data)`<br>
Définit les données de l’image à (`x`, `y`) par la valeur ou une liste de chaînes de caractères<br>
par exemple : `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Lit l’image png depuis le répertoire d’exécution du script à (`x`, `y`)

- `copy(x, y, img, u, v, w, h)`<br>
Copie la région de taille (`w`, `h`) de (`u`, `v`) la banque d’image `img`(0-2) à (`x`, `y`)

### Classe Tilemap

- `width`, `height`<br>
La largeur et la hauteur de la tilemap

- `data`<br>
Les données de la tilemap (liste bi-dimentionelle de 256x256)

- `refimg`<br>
La banque d’images utilisée par la tilemap

- `get(x, y)`<br>
Renvoie les données de la tilemap à (`x`, `y`)

- `set(x, y, data)`<br>
Définit les données de la tilemap à (`x`, `y`) par une valeur ou une liste de chaînes de caractères.<br>
par exemple : `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `copy(x, y, tm, u, v, w, h)`<br>
Copie la région de la taille (`w`, `h`) de (`u`, `v`) de la tilemap `tm`(0-7) à (`x`, `y`)

### Classe Sound

- `note`<br>
Liste des notes (0-127) (33 = 'A2' = 440Hz)

- `tone`<br>
Liste des tons (0:Triangle / 1:Carré / 2:Rythme / 3:Bruit)

- `volume`<br>
Liste des volumes (0-7)

- `effect`<br>
Liste des effets (0:Aucun / 1:Glisse / 2:Vibration / 3:Fondu)

- `speed`<br>
La longueur d’une note (120 = 1 seconde par ton)

- `set(note, tone, volume, effect, speed)`<br>
Définit une note, un ton, un volume, un effet avec une chaine de caractères. Si le ton, le volume et l’effet sont plus courts que la note, ils sont répétés depuis le début

- `set_note(note)`<br>
Définit une note avec une chaine composée de 'CDEFGAB'+'#-'+'0123' ou 'R'. Insensible à la casse et les espaces sont ignorés<br>
par exemple : `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tone(tone)`<br>
Définit un ton avec une chaine composée de 'TSPN'. Insensible à la casse et les espaces sont ignorés<br>
par exemple : `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volume(volume)`<br>
Définit un volume avec une chaine composée de '01234567'. Insensible à la casse et les espaces sont ignorés<br>
par exemple : `pyxel.sound(0).set_volume("7777 7531")`

- `set_effect(effect)`<br>
Définit un effet avec une chaine composée de 'NSVF'. Insensible à la casse et les espaces sont ignorés<br>
par exemple : `pyxel.sound(0).set_effect("NFNF NVVS")`

### Classe Music

- `ch0`<br>
Liste des sons(0-63) joué sur le canal 0. Si une liste vide est spécifiée, le canal n’est pas pris en compte dans la liste de lecture

- `ch1`<br>
Liste des sons(0-63) joué sur le canal 1. Si une liste vide est spécifiée, le canal n’est pas pris en compte dans la liste de lecture

- `ch2`<br>
Liste des sons(0-63) joué sur le canal 2. Si une liste vide est spécifiée, le canal n’est pas pris en compte dans la liste de lecture

- `ch3`<br>
Liste des sons(0-63) joué sur le canal 3. Si une liste vide est spécifiée, le canal n’est pas pris en compte dans la liste de lecture

- `set(ch0, ch1, ch2, ch3)`<br>
Définit la liste des sons(0-63) de tous les canaux. Si une liste vide est spécifiée, le canal n’est pas pris en compte dans la liste de lecture<br>
par exemple : `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

- `set_ch0(data)`<br>
Définit la liste des sons(0-63) du canal 0

- `set_ch1(data)`<br>
Définit la liste des sons(0-63) du canal 1

- `set_ch2(data)`<br>
Définit la liste des sons(0-63) du canal 2

- `set_ch3(data)`<br>
Définit la liste des sons(0-63) du canal 3

## Comment contribuer

### En ouvrant un ticket

Utilisez [le suivi de tickets](https://github.com/kitao/pyxel/issues) pour envoyer des rapports de bug et des proposition de fonctionnalités ou d’améliorations.
Avant d’ouvrir un nouveau ticket, regardez si il n’y en a pas un de similaire déjà ouvert.

Quand vous ouvrez un ticket, choisissez le template approprié depuis [ce lien](https://github.com/kitao/pyxel/issues/new/choose).

### Tester manuellement

Toutes les personnes testant le code et rapportant des bugs ou des suggestions d’améliorations sont les bienvenues!

### En ouvrant une pull request

Les patchs/fixs sont acceptés sous forme de pull requests (PRs). Faites attention à ce que le ticket que la pull request corrige soit ouvert.

En proposant une pull request, vous acceptez qu’elle soit publiée sous la [licence MIT](LICENSE).

## Autres informations

- [Wiki](https://github.com/kitao/pyxel/wiki)
- [Subreddit](https://www.reddit.com/r/pyxel/)
- [Serveur Discord (Anglais)](https://discord.gg/FC7kUZJ)
- [Serveur Discord (Japonais - 日本語版)](https://discord.gg/qHA5BCS)

## License

Pyxel est sous [licence MIT](http://en.wikipedia.org/wiki/MIT_License). Elle peut être réutilisée dans un logiciel propriétaire à condition que toutes les copies du logiciel sous licence comprennent une copie des termes de la licence MIT et de l'avis de copyright.

Pyxel utilise les logiciels suivants :

- [SDL2](https://www.libsdl.org/)
- [miniz-cpp](https://github.com/tfussell/miniz-cpp)
- [Gifsicle](https://www.lcdf.org/gifsicle/)
