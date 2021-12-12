# <img src="images/pyxel_logo_152x64.png">

[ [English](README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]


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

Les spécifications de Pyxel font référence à [PICO-8](https://www.lexaloffle.com/pico-8.php) et [TIC-80](https://tic.computer/).

Pyxel est open source et libre d’usage. Commençons à faire un jeu-vidéo rétro avec Pyxel !

## Spécifications

- Fonctionne sous Windows, Mac et Linux
- Programmable en Python
- Palette de 16 couleurs
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

Il y a deux moyens d’utiliser Pyxel, avec la version packagée ou avec la version standalone.

### Installer la version packagée

La version packagée de Pyxel utilise Pyxel comme un module Python.

Cette version est recommandée pour les personnes qui sont familières avec la gestion de paquets Python et l’utilisatiion de `pip`, ou pour celles qui veulent développer des applications Python.

**Windows**

Après avoir installé [Python3](https://www.python.org/) (version 3.7 ou plus), lancez la commande suivante :

```sh
pip install -U pyxel
```

**Mac**

Après avoir installé [Python3](https://www.python.org/) (version 3.7 ou plus), lancez la commande suivante :

```sh
pip3 install -U pyxel
```

**Linux**

Après avoir installé le paquet SDL2 (`libsdl2-dev` pour Ubuntu), [Python3](https://www.python.org/) (version 3.7 ou plus), et `python3-pip`, lancez la commande suivante :

```sh
pip3 install -U pyxel
```

Si l’installation avec pip ne fonctionne pas, essayez de compiler vous-même en suivant les étapes ci-dessous après avoir installé `cmake` et `rust`:

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make clean all RELEASE=1
pip3 install .
```

### Installer la version standalone

La version standalone de Pyxel utilise Pyxel comme un outil indépendant de Python.

Cette version est recommandée pour les personnes qui commencent à programmer et/ou qui ne veulent pas s’embêter avec les paramètres de Python, ou pour celles qui veulent jouer immédiatement.

**Windows**

Téléchargez et lancez la dernière version de installeur Windows (`pyxel-[version]-windows-setup.exe`) depuis la [page des versions](https://github.com/kitao/pyxel/releases).

**Mac**

Après avoir installé [Homebrew](https://brew.sh/), lancez les commandes suivantes :

```sh
brew tap kitao/pyxel
brew install pyxel
```

**Linux**

Après avoir installé le paquet SDL2 (`libsdl2-dev` pour Ubuntu) et [Homebrew](https://docs.brew.sh/Homebrew-on-Linux), lancez les commandes suivantes :

```sh
brew tap kitao/pyxel
brew install pyxel
```

Si les commandes au-dessus ne fonctionnent pas, vous pouvez essayer de compiler vous-même la version packagée.

### Lancer les exemples de Pyxel

Après l’installation de Pyxel, les exemples de Pyxel seront copiés dans le répertoire courant avec la commande suivante :

```sh
pyxel copy_examples
```

Les exemples copiés sont les suivants :

- [01_hello_pyxel.py](pyxel/examples/01_hello_pyxel.py) - Application simple
- [02_jump_game.py](pyxel/examples/02_jump_game.py) - Jeu de saut avec les fichiers de ressources Pyxel
- [03_draw_api.py](pyxel/examples/03_draw_api.py) - Démonstration des APIs de dessin
- [04_sound_api.py](pyxel/examples/04_sound_api.py) - Démonstration des APIs de son
- [05_color_palette.py](pyxel/examples/05_color_palette.py) - Liste des couleurs de la palette
- [06_click_game.py](pyxel/examples/06_click_game.py) - Jeu de point and click
- [07_snake.py](pyxel/examples/07_snake.py) - Jeu du Snake avec une bande son
- [08_triangle_api.py](pyxel/examples/08_triangle_api.py) - Démonstration des APIs de dessin de triangle
- [09_shooter.py](pyxel/examples/09_shooter.py) - Jeu de shoot'em up avec changement d’écran
- [10_platformer.py](pyxel/examples/10_platformer.py) - Jeu de plateforme avec scroll, et une carte

Les exemples peuvent être lancés avec les commandes suivantes :

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
```

## Comment utiliser

### Créer une application Pyxel

Après avoir importé le module Pyxel dans votre script python, spécifiez d’abord la taille de la fenêtre avec la fonction `init`, puis lancez l’application Pyxel avec la fonction `run`.

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

C’est aussi possible de dessiner de simples images et animatations en utilisant les fonctions `show` et `flip`.

La fonction `show` affiche l’écran jusqu’à ce que la touche `Esc` soit appuyée.

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

### Lancer une application Pyxel

Le script Python créé peut être lancé en utilisant la commande suivante :

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Pour la version packagée, il peut être exécuté comme un script Python classique :

```sh
cd pyxel_examples
python3 PYTHON_SCRIPT_FILE
```

(Pour Windows, utilisez `python` au lieu de `python3`)

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

L’éditeur Pyxel peut créer des images et des sons utilisables dans des applications Pyxel.

Il se lance avec la commande suivante :

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

Si le fichier de ressource Pyxel (.pyxres) existe déjà, le fichier est chargé, sinon, un nouveau fichier avec le nom indiqué est créé.
Si le fichier de ressource n’est pas spécifié, le nom est `my_resource.pyxres`.

Après avoir lancé l’éditeur Pyxel, le fichier peut être changé en glissant-dépossant un autre fichier de ressource. Si le fichier est glissé-déposé en appuyant sur la touche ``Ctrl(Cmd)``, seule le type de ressource actuellement éditée (Image/Tilemap/Son/Musique) sera chargée. Cette opération permet de combiner plusieurs ressources dans un seul fichier.

La ressource créée peut être chargée avec la fonction `load`.

L’éditeur Pyxel a les modes suivants.

**Éditeur d’images :**

Mode pour éditer la banque d’images.

<img src="pyxel/editor/screenshots/image_editor.gif">

En glissant-déposant un fichier images (png/gif/jpeg) dans l’éditeur d’image, l’image peut être chargée dans la banque d’images actuellement sélectionnée.

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

Les images et tilemaps Pyxel peuvent être aussi créées avec les méthodes suivantes :

- Créer une image depuis une liste de chaînes de caractères avec la fonction `Image.set` ou la fonction `Tilemap.set`
- Charger une image (png/gif/jpeg) dans la palette Pyxel avec la fonction `Image.load`

Les sons Pyxel peuvent aussi être créés avec la méthode suivante :

- Créer un son à partir d’une chaîne de caractères avec la fonction `Sound.set` ou la fonction `Music.set`

Référez vous à la documentation de l’API pour l’utilisation de ces fonctions.

### Comment partager une application

Pyxels a un format de fichier spécifique (fichier d’application Pyxel) qui fonctionne sur les différentes plateformes.

Créez le fichier d’application Pyxel (.pyxapp) avec les commandes suivantes :

```sh
pyxel package APP_ROOT_DIR STARTUP_SCRIPT_FILE
```

Si l’application doit inclure des ressources ou des modules additonnels, mettez les dans le dossier de l’application.

L’application créée peut être exécutée avec la commande suivante :

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
Initialise l’application Pyxel avec un écran de taille(`width`, `height`). Il est possible de passer en option : le titre de la fenêtre avec `title`, le nombre de frame par seconde avec `fps`, la touche pour quitter l’application avec `quit_key`, and le temps maximum d’enregistrement vidéo avec `capture_sec`.<br>
Par exemple : `pyxel.init(160, 120, title="Pyxel with Options", fps=60, quit_key=pyxel.KEY_NONE, capture_sec=0)`

- `run(update, draw)`<br>
Lance l’application Pyxel et appelle la fonction `update` et la fonction `draw`.

- `show()`<br>
Affiche l’écran jusqu’à ce que la touche `Esc` soit appuyée. (Ne pas utiliser dans des applications normales)

- `flip()`<br>
Met à jour l’écran une fois. (Ne pas utiliser dans des applications normales)

- `quit()`<br>
Quitte l’application Pyxel à la fin de la frame actuelle.

### Ressources

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Charge la ressource (.pyxres). Si ``False`` est spécifié pour un type de ressource (``image/tilemap/sound/music``), la ressource ne sera pas chargée.

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
Liste les couleurs de la palette. Les couleurs sont spécifiées avec une valeur 24-bit. Vous pouvez utiliser `colors.from_list` et `colors.to_list` pour directement donner et recevoir une liste Python.<br>
Par exemple `org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img)`<br>
Utilise la banque d’images `img` (0-2). (Voir la classe Image)<br>
Par exemple `pyxel.image(0).load(0, 0, "title.png")`

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
Dessine la tilemap `tm` (0-7) à (`x`, `y`) suivant la taille de la tile de (`w`, `h`) à (`u`, `v`). Si `colkey` est spécifié, elle sera traitée comme une couleur transparente. La taille de la tile est de 8x8 pixels et est stockée dans la tilemap comme un tuple `(x in tile, y in tile)`.

- `text(x, y, s, col)`<br>
Dessine une chaîne de caractères `s` de couleur `col` à (`x`, `y`)

### Audio

- `sound(snd, [system])`<br>
Utilise le son `snd`(0-63) (voir la classe Sound). Si `system` est `True`, le son 64 pour le système est accessible<br>
par exemple : `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Utilise la musique `msc`(0-7) (voir la classe Music)

- `play_pos(ch)`<br>
Récupère la position du son du canal `ch` (0-3) comme un tuple `(sound no, note no)`. Renvoie `None` quand le son est arrêté.

- `play(ch, snd, loop=False)`<br>
Joue le son `snd` (0-63) sur le canal `ch` (0-3). Si `snd` est une liste, les sons seront joués dans l’ordre. Si `loop` est à `True`, le son est joué en boucle.

- `playm(msc, loop=False)`<br>
Joue la musique `msc` (0-7). Si `loop` est mis à `True`, la musique est joué en boucle.

- `stop([ch])`<br>
Arrête le son du canal spécifié `ch` (0-3). `stop()` arrête tous les canaux.

### Classe Image

- `width`, `height`<br>
La largeur et la hauteur d’une image

- `data`<br>
Les données de l’image (liste bi-dimentionelle de 256x256)

- `get(x, y)`<br>
Renvoie les données de l’image à (`x`, `y`)

- `set(x, y, data)`<br>
Met la valeur de l’image à (`x`, `y`) suivant une liste de chaînes.<br>
Par exemple `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Charge l’image (png/gif/jpeg) à (`x`, `y`).

### Classe Tilemap

- `width`, `height`<br>
La largeur et la hauteur de la tilemap

- `refimg`<br>
La banque d’image (0-2) référencée par la tilemap

- `set(x, y, data)`<br>
Met la tilemap à (`x`, `y`) suivant une liste de chaînes.<br>
Par exemple `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `pget(x, y)`<br>
Renvoie la tile à (`x`, `y`). Une tile est un tuple `(x in tile, y in tile)`.

- `pset(x, y, tile)`<br>
Dessine une `tile` à (`x`, `y`). Une tile est un tuple `(x in tile, y in tile)`.

### Classe Sound

- `notes`<br>
Liste des notes (0-127). Plus le nombre est haut, plus la note est haute, et à 33 ça devient 'A2' (440Hz). Le reste est à -1.

- `tones`<br>
Liste les tons (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
Liste les volumes (0-7)

- `effects`<br>
Liste les effets (0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`<br>
Vitesse de lecture. 1 est le plus rapide, et plus le nombre est grand, plus la vitesse est lente. à 120, la longueur d’une note est de 1 second.

- `set(notes, tones, volumes, effects, speed)`<br>
Met les valeurs de notes, tones, volumes et effects avec une chaîne. Si les tons, volumes et effets sont plus courts que les notes, ils sont répétés depuis le début.

- `set_notes(notes)`<br>
Met les notes avec une chaîne de 'CDEFGAB'+'#-'+'0123' ou 'R'. Insensible à la casse et les espaces sont ignorés.<br>
Par exemple `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
Met les tons avec une chaîne de 'TSPN'. Insensible à la casse et les espaces sont ignorés.<br>
Par exemple `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volumes(volumes)`<br>
Met les volumes avec une chaîne de '01234567'. Insensible à la casse et les espaces sont ignorés.<br>
Par exemple `pyxel.sound(0).set_volume("7777 7531")`

- `set_effects(effects)`<br>
Met les effets avec une chaîne de 'NSVF'. Insensible à la casse et les espaces sont ignorés.<br>
Par exemple `pyxel.sound(0).set_effect("NFNF NVVS")`

### Classe Music

- `sequences`<br>
Liste à deux dimensions de sons (0-63) listés par leur numéro de canal.

- `set(seq0, seq1, seq2, seq3)`<br>
Met les listes de sons (0-63) de tous les canaux. Si une liste vide est passée, ce canal n’est pas utilisé.<br>
Par exemple `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### Advanced APIs

Pyxel a des "APIs avancées" qui ne sont pas présentées ici car elles peuvent amener de la confusion ou qu’elles nécessitent des connaissances spécifiques pour être utilisées.

Si vous savez ce que vous faîtes, essayez de créer des applications incroyables avec [ceci](pyxel/__init__.pyi) comme indice !

## Comment contribuer

### En ouvrant des tickets

Utilisez [le suivi de tickets](https://github.com/kitao/pyxel/issues) pour rapporter des bugs et demander des nouvelles fonctionnalités ou des améliorations. Avant d’ouvrir un nouveau ticket, regardez si un similaire n’a pas déjà été ouvert.

### Tester manuellement

Toutes les personnes testant le code et rapportant des bugs ou des suggestions d’améliorations dans [le suivi de tickets](https://github.com/kitao/pyxel/issues) sont les bienvenues!

### En ouvrant des pull requests

Les patchs/fixs sont acceptés sous forme de pull requests (PRs). Faites attention à ce que le ticket que la pull request corrige soit ouvert.

En proposant une pull request, vous acceptez qu’elle soit publiée sous la [licence MIT](LICENSE).

## Autres informations

- [Serveur Discord (Anglais)](https://discord.gg/FC7kUZJ)
- [Serveur Discord (Japonais - 日本語版)](https://discord.gg/qHA5BCS)

## License

Pyxel est sous [licence MIT](http://en.wikipedia.org/wiki/MIT_License). Elle peut être réutilisée dans un logiciel propriétaire à condition que toutes les copies du logiciel sous licence comprennent une copie des termes de la licence MIT et de l'avis de copyright.
