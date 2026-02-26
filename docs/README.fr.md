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

Les spécifications et les API de Pyxel s'inspirent de [PICO-8](https://www.lexaloffle.com/pico-8.php) et de [TIC-80](https://tic80.com/).

Pyxel est open source sous la [Licence MIT](../LICENSE) et est gratuit à utiliser. Commençons à créer des jeux rétro avec Pyxel !

## Spécifications

- Fonctionne sur Windows, Mac, Linux et Web
- Programmation en Python
- Taille d'écran personnalisable
- Palette de 16 couleurs
- 3 banques d'images 256x256
- 8 cartes de tuiles 256x256
- 4 canaux avec 64 sons définissables
- 8 pistes de musique capables de combiner n'importe quel son
- Entrées de clavier, de souris et de manette
- Outils d'édition d'images et de sons
- Couleurs, canaux audio et banques extensibles par l'utilisateur

### Palette de couleurs

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Comment installer

### Windows

Après avoir installé [Python 3](https://www.python.org/) (version 3.8 ou supérieure), exécutez la commande suivante :

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

Après avoir installé [Python 3](https://www.python.org/) (version 3.8 ou supérieure), exécutez la commande suivante :

```sh
pip install -U pyxel
```

Si la commande précédente échoue, envisagez de construire Pyxel à partir de la source en suivant les instructions dans le [Makefile](../Makefile).

### Web

La version web de Pyxel fonctionne sur PC, smartphone et tablette avec un navigateur compatible, sans installer Python ou Pyxel.

La façon la plus simple de l'utiliser est via l'IDE en ligne [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/).

Pour d'autres modes d'utilisation, tels que l'intégration d'applications Pyxel sur votre propre site, veuillez vous référer à [cette page](pyxel-web-en.md).

## Utilisation de base

### Commande Pyxel

L'installation de Pyxel ajoute la commande `pyxel`. Spécifiez un nom de commande après `pyxel` pour effectuer diverses opérations.

Exécutez-la sans arguments pour afficher la liste des commandes disponibles :

```sh
pyxel
```

```
Pyxel, a retro game engine for Python
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

### Essayer les exemples

La commande suivante copie les exemples Pyxel dans le répertoire actuel :

```sh
pyxel copy_examples
```

En environnement local, les exemples peuvent être exécutés avec les commandes suivantes :

```sh
# Exécuter l'exemple dans le répertoire examples
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# Exécuter l'application dans le répertoire examples/apps
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

Les exemples peuvent également être consultés et exécutés dans le navigateur depuis [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/).

## Création d'applications

### Créer un programme

Dans votre script Python, importez Pyxel, spécifiez la taille de la fenêtre avec `init` et démarrez l'application avec `run`.

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

### Exécuter un programme

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

Interrompez la surveillance du répertoire en appuyant sur `Ctrl(Command)+C`.

### Commandes spéciales

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
  Alterner le mode de mise à l’échelle de l’écran entre maximal et entier
- `Alt(Option)+9` ou `A+B+X+Y+DR` sur la manette<br>
  Passer d'un mode d'écran à l'autre (Crisp/Smooth/Retro)
- `Alt(Option)+0` ou `A+B+X+Y+DU` sur la manette<br>
  Basculer le moniteur de performance (FPS/temps de `update`/temps de `draw`)
- `Alt(Option)+Enter` ou `A+B+X+Y+DD` sur la manette<br>
  Basculer en plein écran
- `Shift+Alt(Option)+1/2/3`<br>
  Enregistrer la banque d'images 0, 1 ou 2 sur le bureau
- `Shift+Alt(Option)+0`<br>
  Enregistrer la palette de couleurs actuelle sur le bureau

## Création de ressources

### Pyxel Editor

Pyxel Editor crée des images et des sons utilisés dans une application Pyxel.

Vous pouvez démarrer Pyxel Editor avec la commande suivante :

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Si le fichier de ressources Pyxel spécifié (.pyxres) existe, il sera chargé. S'il n'existe pas, un nouveau fichier avec le nom spécifié sera créé. Si le fichier de ressources est omis, un nouveau fichier nommé `my_resource.pyxres` sera créé.

Après avoir démarré Pyxel Editor, vous pouvez passer à un autre fichier de ressources en le faisant glisser et en le déposant sur l'éditeur.

Le fichier de ressources créé peut être chargé en utilisant la fonction `load`.

Pyxel Editor a les modes d'édition suivants.

**Éditeur d'images**

Le mode pour éditer les images dans chaque **banque d'images**.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_editor.gif">
</a>

Vous pouvez faire glisser et déposer un fichier d'image (PNG/GIF/JPEG) dans l'éditeur d'images pour charger l'image dans la banque d'images actuellement sélectionnée.

**Éditeur de cartes de tuiles**

Le mode pour éditer les **cartes de tuiles** qui organisent des images des banques d'images en un motif de tuiles.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/tilemap-editor.html">
<img src="images/tilemap_editor.gif">
</a>

Faites glisser et déposer un fichier TMX (Tiled Map File) dans l'éditeur de cartes de tuiles pour charger sa couche 0 dans la carte de tuiles actuellement sélectionnée.

**Éditeur de sons**

Le mode pour éditer les **sons** utilisés pour les mélodies et les effets sonores.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
<img src="images/sound_editor.gif">
</a>

**Éditeur de musique**

Le mode pour éditer des **pistes de musique** dans lequel les sons sont organisés dans l'ordre de lecture.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/music-editor.html">
<img src="images/music_editor.gif">
</a>

### Autres méthodes de création

Les images et les cartes de tuiles Pyxel peuvent également être créées en utilisant les méthodes suivantes :

- Créer des images ou des cartes de tuiles à partir de listes de chaînes avec les fonctions `Image.set` ou `Tilemap.set`
- Charger des fichiers image compatibles avec la palette Pyxel (PNG/GIF/JPEG) avec la fonction `Image.load`

Les sons et la musique Pyxel peuvent également être créés en utilisant la méthode suivante :

- Les créer à partir de chaînes avec les fonctions `Sound.set` ou `Music.set`

Référez-vous à la documentation de l'API pour l'utilisation de ces fonctions.

## Distribution d'applications

Pyxel prend en charge un format de distribution multiplateforme appelé fichier d'application Pyxel.

Créez un fichier d'application Pyxel (.pyxapp) avec la commande `pyxel package` :

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

Le fichier d'application créé peut être exécuté avec la commande `pyxel play` :

```sh
pyxel play PYXEL_APP_FILE
```

Un fichier d'application Pyxel peut également être converti en un exécutable ou un fichier HTML avec les commandes `pyxel app2exe` ou `pyxel app2html`.

## Référence de l’API

La liste complète des API Pyxel est disponible sur [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/).

Pyxel comprend également une « API avancée » qui nécessite des connaissances spécialisées. Vous pouvez la consulter en cochant la case « Advanced » sur la page de référence.

Si vous êtes confiant dans vos compétences, essayez d’utiliser l’API avancée pour créer des œuvres vraiment impressionnantes !

## Comment Contribuer

### Soumettre des Problèmes

Utilisez le [Issue Tracker](https://github.com/kitao/pyxel/issues) pour soumettre des rapports de bugs et des demandes de fonctionnalités ou d'améliorations. Avant de soumettre un nouveau problème, assurez-vous qu'il n'y a pas de problèmes ouverts similaires.

### Tests Fonctionnels

Toute personne qui teste manuellement le code et signale des bugs ou des suggestions d'améliorations dans le [Issue Tracker](https://github.com/kitao/pyxel/issues) est la bienvenue !

### Soumettre des Demandes de Tirage

Les correctifs et modifications sont acceptés sous forme de demandes de tirage (PRs). Assurez-vous que le problème que la demande de tirage aborde est ouvert dans le Issue Tracker.

Soumettre une demande de tirage implique que vous acceptez de licencier votre contribution sous la [Licence MIT](../LICENSE).

## Outils Web et Exemples

- [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)
- [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/)
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
