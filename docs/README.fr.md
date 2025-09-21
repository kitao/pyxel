# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** (/ˈpɪksəl/) est un moteur de jeu rétro pour Python.

Les spécifications sont inspirées des consoles de jeux rétro, comme le fait de n'afficher que 16 couleurs et de prendre en charge 4 canaux audio, vous permettant ainsi de profiter facilement de la création de jeux au style pixel art.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

Le développement de Pyxel est motivé par les retours des utilisateurs. Merci de donner une étoile à Pyxel sur GitHub !

<p>
<a href="https://kitao.github.io/pyxel/wasm/examples/10-platformer.html">
<img src="images/10_platformer.gif" width="290">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/30sec-of-daylight.html">
<img src="images/30sec_of_daylight.gif" width="350">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/02-jump-game.html">
<img src="images/02_jump_game.gif" width="330">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">
<img src="images/megaball.gif" width="310">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/image-editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/sound-editor.html">
<img src="images/sound_music_editor.gif" width="320">
</a>
</p>

Les spécifications et les API de Pyxel s'inspirent de [PICO-8](https://www.lexaloffle.com/pico-8.php) et de [TIC-80](https://tic80.com/).

Pyxel est open source sous la [Licence MIT](../LICENSE) et est gratuit à utiliser. Commençons à créer des jeux rétro avec Pyxel !

## Spécifications

- Fonctionne sur Windows, Mac, Linux et Web
- Programmation en Python
- Taille d'écran personnalisable
- Palette de 16 couleurs
- 3 banques d'images de 256x256
- 8 cartes de tuiles de 256x256
- 4 canaux avec 64 sons définissables
- 8 pistes de musique qui peuvent combiner n'importe quel son
- Entrées de clavier, de souris et de manette
- Outils d'édition d'images et de sons
- Couleurs, canaux et banques extensibles par l'utilisateur

### Palette de couleurs

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Comment installer

### Windows

Après avoir installé [Python3](https://www.python.org/) (version 3.8 ou supérieure), exécutez la commande suivante :

```sh
pip install -U pyxel
```

Lors de l'installation de Python à l'aide de l'installateur officiel, assurez-vous de cocher l'option `Add Python 3.x to PATH` pour activer la commande `pyxel`.

### Mac

Après avoir installé [Homebrew](https://brew.sh/), exécutez les commandes suivantes :

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Pour mettre à jour Pyxel après l'installation, exécutez `pipx upgrade pyxel`.

### Linux

Après avoir installé le paquet SDL2 (`libsdl2-dev` pour Ubuntu), [Python3](https://www.python.org/) (version 3.8 ou supérieure) et `python3-pip`, exécutez la commande suivante :

```sh
sudo pip3 install -U pyxel
```

Si la commande précédente échoue, envisagez de construire Pyxel à partir de la source en suivant les instructions dans le [Makefile](../Makefile).

### Web

La version Web de Pyxel ne nécessite pas d'installation de Python ou de Pyxel et fonctionne sur des PC, des smartphones et des tablettes avec des navigateurs Web compatibles.

Pour des instructions détaillées, veuillez vous référer à [cette page](pyxel-web-en.md).

### Exécuter des exemples

Après avoir installé Pyxel, vous pouvez copier les exemples dans le répertoire actuel avec la commande suivante :

```sh
pyxel copy_examples
```

Les exemples suivants seront copiés dans votre répertoire actuel :

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>L'application la plus simple</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01-hello-pyxel.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Code</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Jeu de saut avec fichier de ressources Pyxel</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02-jump-game.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Code</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Démo des APIs de dessin</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03-draw-api.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Code</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Démo des APIs de son</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04-sound-api.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Code</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>Liste de palettes de couleurs</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05-color-palette.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Code</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>Jeu de clic avec la souris</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06-click-game.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Code</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>Jeu de serpent avec BGM</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07-snake.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Code</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Démo des APIs de dessin de triangles</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08-triangle-api.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Code</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Shoot'em up avec transitions d'écran et MML</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09-shooter.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Code</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>Jeu de plateforme défilant horizontalement avec carte</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10-platformer.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Code</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Rendu hors écran avec la classe Image</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11-offscreen.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Code</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>Animation de bruit de Perlin</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12-perlin-noise.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Code</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>Dessin d'une police bitmap</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13-bitmap-font.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Code</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>Synthétiseur utilisant des fonctions d'extension audio</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14-synthesizer.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Code</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>Chargement et dessin d'un Tiled Map File (.tmx)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15-tiled-map-file.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Code</a></td>
</tr>
<tr>
<td>16_transform.py</td>
<td>Rotation et mise à l'échelle d'images</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/16-transform.html">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/16_transform.py">Code</a></td>
</tr>
<tr>
<td>17_app_launcher.py</td>
<td>Lanceur d'applications Pyxel (vous pouvez jouer à divers jeux !)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/17-app-launcher.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/17_app_launcher.py">Code</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>Animation avec la fonction flip (uniquement pour les plateformes non-web)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Démonstration</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Code</a></td>
</tr>
</table>

Les exemples peuvent être exécutés avec les commandes suivantes :

```sh
# Run example in examples directory
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# Run app in examples/apps directory
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

## Comment utiliser

### Créer une application

Dans votre script Python, importez le module Pyxel, spécifiez la taille de la fenêtre avec la fonction `init`, puis démarrez l'application Pyxel avec la fonction `run`.

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

Les arguments de la fonction `run` sont la fonction `update`, qui traite les mises à jour de trames, et la fonction `draw`, qui gère le dessin à l'écran.

Dans une application réelle, il est recommandé d'encapsuler le code Pyxel dans une classe, comme montré ci-dessous :

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

Pour créer des graphiques simples sans animation, vous pouvez utiliser la fonction `show` pour simplifier votre code.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Exécuter l'application

Un script créé peut être exécuté en utilisant la commande `python` :

```sh
python PYTHON_SCRIPT_FILE
```

Il peut également être exécuté avec la commande `pyxel run` :

```sh
pyxel run PYTHON_SCRIPT_FILE
```

De plus, la commande `pyxel watch` surveille les modifications dans un répertoire spécifié et relance automatiquement le programme lorsque des changements sont détectés :

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

La surveillance du répertoire peut être arrêtée en appuyant sur `Ctrl(Command)+C`.

### Opérations de touches spéciales

Pendant l'exécution d'une application Pyxel, les opérations de touches spéciales suivantes peuvent être effectuées :

- `Esc`<br>
  Quitter l'application
- `Alt(Option)+R` ou `A+B+X+Y+BACK` sur la manette<br>
  Réinitialiser l'application
- `Alt(Option)+1`<br>
  Enregistrer la capture d'écran sur le bureau
- `Alt(Option)+2`<br>
  Réinitialiser le temps de début de l'enregistrement de la vidéo de capture d'écran
- `Alt(Option)+3`<br>
  Enregistrer une vidéo de capture d'écran sur le bureau (jusqu'à 10 secondes)
- `Alt(Option)+8` ou `A+B+X+Y+DL` sur la manette<br>
  Alterne le mode de mise à l’échelle de l’écran entre maximal et entier
- `Alt(Option)+9` ou `A+B+X+Y+DR` sur la manette<br>
  Passer d'un mode d'écran à l'autre (Crisp/Smooth/Retro)
- `Alt(Option)+0` ou `A+B+X+Y+DU` sur la manette<br>
  Basculer le moniteur de performance (FPS/`update` temps/`draw` temps)
- `Alt(Option)+Enter` ou `A+B+X+Y+DD` sur la manette<br>
  Basculer en plein écran
- `Shift+Alt(Option)+1/2/3`<br>
  Enregistrer la banque d'images 0, 1 ou 2 sur le bureau
- `Shift+Alt(Option)+0`<br>
  Enregistrer la palette de couleurs actuelle sur le bureau

### Comment créer des ressources

Pyxel Editor peut créer des images et des sons utilisés dans une application Pyxel.

Vous pouvez démarrer Pyxel Editor avec la commande suivante :

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Si le fichier de ressources Pyxel spécifié (.pyxres) existe, il sera chargé. S'il n'existe pas, un nouveau fichier avec le nom spécifié sera créé. Si le fichier de ressources est omis, un nouveau fichier nommé `my_resource.pyxres` sera créé.

Après avoir démarré Pyxel Editor, vous pouvez passer à un autre fichier de ressources en le faisant glisser et en le déposant sur Pyxel Editor.

Le fichier de ressources créé peut être chargé en utilisant la fonction `load`.

Pyxel Editor a les modes d'édition suivants.

**Éditeur d'images**

Le mode pour éditer l'image dans chaque **banque d'images**.

<a href="https://kitao.github.io/pyxel/wasm/examples/image-editor.html">
<img src="images/image_editor.gif">
</a>

Vous pouvez faire glisser et déposer un fichier d'image (PNG/GIF/JPEG) dans l'éditeur d'images pour charger l'image dans la banque d'images actuellement sélectionnée.

**Éditeur de cartes de tuiles**

Le mode pour éditer les **cartes de tuiles** qui organisent des images des banques d'images en un motif de tuiles.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap-editor.html">
<img src="images/tilemap_editor.gif">
</a>

Faites glisser et déposer un fichier TMX (Tiled Map File) dans l'éditeur de cartes de tuiles pour charger sa couche 0 dans la carte de tuiles actuellement sélectionnée.

**Éditeur de sons**

Le mode pour éditer les **sons** utilisés pour les mélodies et les effets sonores.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound-editor.html">
<img src="images/sound_editor.gif">
</a>

**Éditeur de musique**

Le mode pour éditer des **musiques** dans lequel les sons sont organisés dans l'ordre de lecture.

<a href="https://kitao.github.io/pyxel/wasm/examples/music-editor.html">
<img src="images/music_editor.gif">
</a>

### Autres méthodes de création de ressources

Les images et les cartes de tuiles Pyxel peuvent également être créées en utilisant les méthodes suivantes :

- Créer une image à partir d'une liste de chaînes en utilisant la fonction `Image.set` ou la fonction `Tilemap.set`
- Charger un fichier d'image (PNG/GIF/JPEG) avec la palette Pyxel en utilisant la fonction `Image.load`

Les sons Pyxel peuvent également être créés en utilisant la méthode suivante :

- Créer un son à partir de chaînes avec la fonction `Sound.set` ou la fonction `Music.set`

Référez-vous à la documentation de l'API pour l'utilisation de ces fonctions.

### Comment distribuer des applications

Pyxel prend en charge un format de fichier de distribution d'application dédié (fichier d'application Pyxel) qui est multiplateforme.

Un fichier d'application Pyxel (.pyxapp) est créé en utilisant la commande `pyxel package` :

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Si vous avez besoin d'inclure des ressources ou des modules supplémentaires, placez-les dans le répertoire de l'application.

Les métadonnées peuvent être affichées à l'exécution en les spécifiant dans le format suivant dans le script de démarrage. Les champs autres que `title` et `author` sont facultatifs.

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

Le fichier d'application créé peut être exécuté en utilisant la commande `pyxel play` :

```sh
pyxel play PYXEL_APP_FILE
```

Un fichier d'application Pyxel peut également être converti en un exécutable ou un fichier HTML en utilisant les commandes `pyxel app2exe` ou `pyxel app2html`.

## Référence de l'API

### Système

- `width`, `height`<br>
  La largeur et la hauteur de l'écran

- `frame_count`<br>
  Le nombre d'images écoulées

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Initialise l'application Pyxel avec la taille de l'écran (`width`, `height`). Les options suivantes peuvent être spécifiées : le titre de la fenêtre avec `title`, le taux de rafraîchissement avec `fps`, la touche pour quitter l'application avec `quit_key`, l'échelle d'affichage avec `display_scale`, l'échelle de capture d'écran avec `capture_scale`, et le temps maximum d'enregistrement de la vidéo de capture d'écran avec `capture_sec`.<br>
  Exemple : `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Démarre l'application Pyxel et appelle la fonction `update` pour la mise à jour de l'image et la fonction `draw` pour le dessin.

- `show()`<br>
  Affiche l'écran et attend que la touche `Esc` soit enfoncée.

- `flip()`<br>
  Rafraîchit l'écran d'une image. L'application se ferme lorsque la touche `Esc` est enfoncée. Cette fonction n'est pas disponible dans la version web.

- `quit()`<br>
  Ferme l'application Pyxel.

- `reset()`<br>
  Réinitialise l'application Pyxel. Les variables d'environnement sont conservées après la réinitialisation.

### Ressources

- `load(filename, [exclude_images], [exclude_tilemaps], [exclude_sounds], [exclude_musics])`<br>
  Charge le fichier de ressources (.pyxres). Si une option est définie sur `True`, la ressource correspondante sera exclue du chargement. Si un fichier de palette (.pyxpal) portant le même nom existe au même endroit que le fichier de ressources, les couleurs de la palette seront également mises à jour. Le fichier de palette contient des entrées hexadécimales pour les couleurs d'affichage (par ex. `1100ff`), séparées par des sauts de ligne. Le fichier de palette peut également être utilisé pour changer les couleurs affichées dans Pyxel Editor.

- `user_data_dir(vendor_name, app_name)`<br>
  Renvoie le répertoire de données utilisateur créé en fonction de `vendor_name` et `app_name`. Si le répertoire n'existe pas, il sera créé automatiquement. Il est utilisé pour stocker des scores élevés, la progression du jeu et des données similaires.<br>
  Exemple : `print(pyxel.user_data_dir("Takashi Kitao", "Pyxel Shooter"))`

### Entrée

- `mouse_x`, `mouse_y`<br>
  La position actuelle du curseur de la souris

- `mouse_wheel`<br>
  La valeur actuelle de la molette de la souris

- `btn(key)`<br>
  Renvoie `True` si la touche `key` est enfoncée, sinon renvoie `False`. ([Liste des définitions de touches](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  Renvoie `True` si la touche `key` est enfoncée à cette image, sinon renvoie `False`. Si `hold` et `repeat` sont spécifiés, après que la touche `key` ait été enfoncée pendant `hold` images ou plus, `True` est renvoyé toutes les `repeat` images.

- `btnr(key)`<br>
  Renvoie `True` si la touche `key` est relâchée à cette image, sinon renvoie `False`.

- `mouse(visible)`<br>
  Affiche le curseur de la souris si `visible` est `True`, et le masque si `visible` est `False`. La position du curseur continue de se mettre à jour même lorsqu'il est masqué.

### Graphiques

- `colors`<br>
  Liste des couleurs de la palette. La couleur d'affichage est spécifiée par une valeur numérique de 24 bits. Utilisez `colors.from_list` et `colors.to_list` pour affecter et récupérer directement des listes Python.<br>
  Exemple : `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Liste des banques d'images (instances de la classe Image) (0-2)<br>
  Exemple : `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Liste des cartes de tuiles (instances de la classe Tilemap) (0-7)
  Liste des cartes de tuiles (0-7)

- `clip(x, y, w, h)`<br>
  Définit la zone de dessin de l'écran à partir de (`x`, `y`) avec une largeur de `w` et une hauteur de `h`. Appelez `clip()` pour réinitialiser la zone de dessin à plein écran.

- `camera(x, y)`<br>
  Change les coordonnées du coin supérieur gauche de l'écran en (`x`, `y`). Appelez `camera()` pour réinitialiser les coordonnées du coin supérieur gauche à (`0`, `0`).

- `pal(col1, col2)`<br>
  Remplace la couleur `col1` par `col2` lors du dessin. Appelez `pal()` pour réinitialiser la palette initiale.

- `dither(alpha)`<br>
  Applique un tramage (pseudo-transparence) lors du dessin. Réglez `alpha` dans la plage de `0.0` à `1.0`, où `0.0` est transparent et `1.0` est opaque.

- `cls(col)`<br>
  Efface l'écran avec la couleur `col`.

- `pget(x, y)`<br>
  Obtient la couleur du pixel à (`x`, `y`).

- `pset(x, y, col)`<br>
  Dessine un pixel de couleur `col` à (`x`, `y`).

- `line(x1, y1, x2, y2, col)`<br>
  Dessine une ligne de couleur `col` de (`x1`, `y1`) à (`x2`, `y2`).

- `rect(x, y, w, h, col)`<br>
  Dessine un rectangle de largeur `w`, hauteur `h` et couleur `col` à partir de (`x`, `y`).

- `rectb(x, y, w, h, col)`<br>
  Dessine le contour d'un rectangle de largeur `w`, hauteur `h` et couleur `col` à partir de (`x`, `y`).

- `circ(x, y, r, col)`<br>
  Dessine un cercle de rayon `r` et de couleur `col` à (`x`, `y`).

- `circb(x, y, r, col)`<br>
  Dessine le contour d'un cercle de rayon `r` et de couleur `col` à (`x`, `y`).

- `elli(x, y, w, h, col)`<br>
  Dessine une ellipse de largeur `w`, hauteur `h` et couleur `col` à partir de (`x`, `y`).

- `ellib(x, y, w, h, col)`<br>
  Dessine le contour d'une ellipse de largeur `w`, hauteur `h` et couleur `col` à partir de (`x`, `y`).

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  Dessine un triangle avec des sommets à (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) et de couleur `col`.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  Dessine le contour d'un triangle avec des sommets à (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) et de couleur `col`.

- `fill(x, y, col)`<br>
  Remplit la zone connectée avec la même couleur que (`x`, `y`) avec la couleur `col`.

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Copie la région de taille (`w`, `h`) de (`u`, `v`) de la banque d'images `img`(0-2) vers (`x`, `y`). Si une valeur négative est assignée à `w` et/ou `h`, la région sera retournée horizontalement et/ou verticalement. Si `colkey` est spécifié, il sera traité comme une couleur transparente. Si `rotate` (en degrés), `scale` (1.0 = 100%) ou les deux sont spécifiés, les transformations correspondantes seront appliquées.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Copie la région de taille (`w`, `h`) de (`u`, `v`) de la carte de tuiles `tm`(0-7) vers (`x`, `y`). Si une valeur négative est assignée à `w` et/ou `h`, la région sera retournée horizontalement et/ou verticalement. Si `colkey` est spécifié, il sera traité comme une couleur transparente. Si `rotate` (en degrés), `scale` (1.0 = 100%) ou les deux sont spécifiés, les transformations correspondantes seront appliquées. La taille d'une tuile est de 8x8 pixels et est stockée dans une carte de tuiles sous forme de tuple `(image_tx, image_ty)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  Dessine une chaîne `s` de couleur `col` à (`x`, `y`).

### Audio

- `sounds`<br>
  Liste des sons (instances de la classe Sound) (0-63)<br>
  Exemple : `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Liste des musiques (instances de la classe Music) (0-7)

- `play(ch, snd, [sec], [loop], [resume])`<br>
  Joue le son `snd`(0-63) sur le canal `ch`(0-3). `snd` peut être un numéro de son, une liste de numéros de son ou une chaîne MML. La position de départ de la lecture peut être spécifiée en secondes avec `sec`. Si `loop` est défini sur `True`, le son sera lu en boucle. Pour reprendre le son précédent après la fin de la lecture, définissez `resume` sur `True`.

- `playm(msc, [sec], [loop])`<br>
  Joue la musique `msc`(0-7). La position de départ de la lecture peut être spécifiée en secondes avec `sec`. Si `loop` est défini sur `True`, la musique sera lue en boucle.

- `stop([ch])`<br>
  Arrête la lecture du canal spécifié `ch`(0-3). Appelez `stop()` pour arrêter tous les canaux.

- `play_pos(ch)`<br>
  Obtient la position de lecture du son sur le canal `ch`(0-3) sous forme de tuple `(sound_no, sec)`. Renvoie `None` lorsque la lecture est arrêtée.

### Mathématiques

- `ceil(x)`<br>
  Renvoie l'entier le plus petit qui est supérieur ou égal à `x`.

- `floor(x)`<br>
  Renvoie l'entier le plus grand qui est inférieur ou égal à `x`.

- `sgn(x)`<br>
  Renvoie `1` si `x` est positif, `0` s'il est égal à `0`, et `-1` s'il est négatif.

- `sqrt(x)`<br>
  Renvoie la racine carrée de `x`.

- `sin(deg)`<br>
  Renvoie le sinus de `deg` degrés.

- `cos(deg)`<br>
  Renvoie le cosinus de `deg` degrés.

- `atan2(y, x)`<br>
  Renvoie l'arc-tangente de `y`/`x` en degrés.

- `rseed(seed)`<br>
  Définit la graine du générateur de nombres aléatoires.

- `rndi(a, b)`<br>
  Renvoie un entier aléatoire compris entre `a` et `b` inclus.

- `rndf(a, b)`<br>
  Renvoie un nombre flottant aléatoire compris entre `a` et `b` inclus.

- `nseed(seed)`<br>
  Définit la graine du bruit de Perlin.

- `noise(x, [y], [z])`<br>
  Renvoie la valeur du bruit de Perlin pour les coordonnées spécifiées.

### Classe Image

- `width`, `height`<br>
  La largeur et la hauteur de l'image

- `set(x, y, data)`<br>
  Définit l'image à (`x`, `y`) à l'aide d'une liste de chaînes.<br>
  Exemple : `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Charge un fichier image (PNG/GIF/JPEG) à (`x`, `y`).

- `pget(x, y)`<br>
  Obtient la couleur du pixel à (`x`, `y`).

- `pset(x, y, col)`<br>
  Dessine un pixel de couleur `col` à (`x`, `y`).

### Classe Tilemap

- `width`, `height`<br>
  La largeur et la hauteur de la carte de tuiles

- `imgsrc`<br>
  La banque d'images (0-2) référencée par la carte de tuiles

- `set(x, y, data)`<br>
  Définit la carte de tuiles à (`x`, `y`) à l'aide d'une liste de chaînes.<br>
  Exemple : `pyxel.tilemaps[0].set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  Charge la `layer`(0-) à partir du fichier TMX (Tiled Map File) à (`x`, `y`).

- `pget(x, y)`<br>
  Obtient la tuile à (`x`, `y`). Une tuile est représentée sous forme de tuple `(image_tx, image_ty)`.

- `pset(x, y, tile)`<br>
  Dessine une `tuile` à (`x`, `y`). Une tuile est représentée sous forme de tuple `(image_tx, image_ty)`.

### Classe Sound

- `notes`<br>
  Liste de notes (0-127). Plus le nombre est élevé, plus la hauteur du son est aiguë. La note `33` correspond à 'A2'(440Hz). Les notes de silence sont représentées par `-1`.

- `tones`<br>
  Liste de tons (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
  Liste de volumes (0-7)

- `effects`<br>
  Liste d'effets (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  Vitesse de lecture. `1` est la plus rapide, et plus le nombre est élevé, plus la vitesse de lecture est lente. À `120`, la durée d'une note est de 1 seconde.

- `set(notes, tones, volumes, effects, speed)`<br>
  Définit les notes, les tons, les volumes et les effets à l'aide d'une chaîne. Si la longueur des tons, volumes ou effets est inférieure à celle des notes, ils seront répétés à partir du début.

- `set_notes(notes)`<br>
  Définit les notes à l'aide d'une chaîne composée de `CDEFGAB`+`#-`+`01234` ou `R`. Insensible à la casse, et les espaces sont ignorés.<br>
  Exemple : `pyxel.sounds[0].set_notes("g2b-2d3r rf3f3f3")`

- `set_tones(tones)`<br>
  Définit les tons avec une chaîne composée de `TSPN`. Insensible à la casse, et les espaces sont ignorés.<br>
  Exemple : `pyxel.sounds[0].set_tones("ttss pppn")`

- `set_volumes(volumes)`<br>
  Définit les volumes avec une chaîne composée de `01234567`. Insensible à la casse, et les espaces sont ignorés.<br>
  Exemple : `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  Définit les effets avec une chaîne composée de `NSVFHQ`. Insensible à la casse, et les espaces sont ignorés.<br>
  Exemple : `pyxel.sounds[0].set_effects("nfnf nvvs")`

- `mml(code)`<br>
  En passant une chaîne [MML (Music Macro Language)](https://en.wikipedia.org/wiki/Music_Macro_Language), on passe en mode MML et le son est joué selon son contenu. Dans ce mode, les paramètres normaux comme `notes` et `speed` sont ignorés. Pour quitter le mode MML, appelez `mml()` sans argument. Pour plus de détails sur MML, voir [cette page](faq-en.md).<br>
  Exemple : `pyxel.sounds[0].mml("T120 Q90 @1 V100 O5 L8 C4&C<G16R16>C.<G16 >C.D16 @VIB1{10,20,20} E2C2")`

- `save(filename, sec, [ffmpeg])`<br>
  Crée un fichier WAV qui lit le son pendant le nombre de secondes spécifié. Si FFmpeg est installé et que `ffmpeg` est défini sur `True`, un fichier MP4 est également créé.

- `total_sec()`<br>
  Renvoie la durée de lecture du son en secondes. Renvoie `None` si une boucle infinie est utilisée dans MML.

### Classe Music

- `seqs`<br>
  Une liste bidimensionnelle de sons (0-63) sur plusieurs canaux

- `set(seq0, seq1, seq2, ...)`<br>
  Définit les listes de sons (0-63) pour chaque canal. Si une liste vide est spécifiée, ce canal ne sera pas utilisé pour la lecture.<br>
  Exemple : `pyxel.musics[0].set([0, 1], [], [3])`

- `save(filename, sec, [ffmpeg])`<br>
  Crée un fichier WAV qui lit la musique pendant le nombre de secondes spécifié. Si FFmpeg est installé et que `ffmpeg` est défini sur `True`, un fichier MP4 est également créé.

### API Avancée

Pyxel inclut une "API Avancée" qui n'est pas mentionnée dans cette référence, car elle peut confondre les utilisateurs ou nécessiter des connaissances spécialisées pour son utilisation.

Si vous êtes confiant dans vos compétences, essayez de créer des œuvres étonnantes en utilisant [cela](../python/pyxel/__init__.pyi) comme guide !

## Comment Contribuer

### Soumettre des Problèmes

Utilisez le [Issue Tracker](https://github.com/kitao/pyxel/issues) pour soumettre des rapports de bugs et des demandes de fonctionnalités ou d'améliorations. Avant de soumettre un nouveau problème, assurez-vous qu'il n'y a pas de problèmes ouverts similaires.

### Tests Fonctionnels

Toute personne qui teste manuellement le code et signale des bugs ou des suggestions d'améliorations dans le [Issue Tracker](https://github.com/kitao/pyxel/issues) est la bienvenue !

### Soumettre des Demandes de Tirage

Les correctifs et modifications sont acceptés sous forme de demandes de tirage (PRs). Assurez-vous que le problème que la demande de tirage aborde est ouvert dans le Issue Tracker.

Soumettre une demande de tirage implique que vous acceptez de licencier votre contribution sous la [Licence MIT](../LICENSE).

## Outils Web et Exemples

- [Pyxel Web Examples](https://kitao.github.io/pyxel/wasm/examples/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/)

## Autres Informations

- [FAQ](faq-en.md)
- [Exemples d'Utilisateurs](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Compte X du Développeur](https://x.com/kitao)
- [Serveur Discord (Anglais)](https://discord.gg/Z87eYHN)
- [Serveur Discord (Japonais)](https://discord.gg/qHA5BCS)

## Licence

Pyxel est sous la [Licence MIT](../LICENSE). Il peut être réutilisé dans des logiciels propriétaires, à condition que toutes les copies du logiciel ou de ses parties substantielles incluent une copie des termes de la Licence MIT et un avis de droit d'auteur.

## Recherche de Sponsors

Pyxel recherche des sponsors sur GitHub Sponsors. Veuillez envisager de soutenir Pyxel pour soutenir sa maintenance continue et le développement de fonctionnalités. En tant qu'avantage, les sponsors peuvent consulter directement le développeur de Pyxel. Pour plus de détails, veuillez visiter [cette page](https://github.com/sponsors/kitao).
