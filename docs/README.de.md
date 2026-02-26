# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** (/ˈpɪksəl/) ist eine Retro-Spiel-Engine für Python.

Die Spezifikationen sind von Retro-Spielkonsolen inspiriert, wie z. B. der Unterstützung von nur 16 Farben und 4 Klangkanälen, sodass Sie ganz einfach pixelartige Spiele erstellen können.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

Die Entwicklung von Pyxel wird durch das Feedback der Benutzer vorangetrieben. Bitte geben Sie Pyxel einen Stern auf GitHub!

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

Die Spezifikationen und APIs von Pyxel sind inspiriert von [PICO-8](https://www.lexaloffle.com/pico-8.php) und [TIC-80](https://tic80.com/).

Pyxel ist unter der [MIT-Lizenz](../LICENSE) Open Source und kostenlos zu verwenden. Lassen Sie uns beginnen, Retro-Spiele mit Pyxel zu erstellen!

## Spezifikationen

- Läuft auf Windows, Mac, Linux und Web
- Programmierung in Python
- Anpassbare Bildschirmgröße
- 16-Farben-Palette
- 3 256x256 Bildbanken
- 8 256x256 Kachelkarten
- 4 Kanäle mit 64 definierbaren Klängen
- 8 Musiktracks, die beliebige Klänge kombinieren können
- Eingaben über Tastatur, Maus und Gamepad
- Werkzeuge zum Bearbeiten von Bildern und Klängen
- Benutzererweiterbare Farben, Sound-Kanäle und Banken

### Farbpalette

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Installation

### Windows

Nachdem Sie [Python 3](https://www.python.org/) (Version 3.8 oder höher) installiert haben, führen Sie den folgenden Befehl aus:

```sh
pip install -U pyxel
```

Wenn Sie Python mit dem offiziellen Installer installieren, stellen Sie sicher, dass Sie die Option `Add Python 3.x to PATH` aktivieren, um den `pyxel` Befehl zu ermöglichen.

### Mac

Nachdem Sie [Homebrew](https://brew.sh/) installiert haben, führen Sie die folgenden Befehle aus:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Um Pyxel nach der Installation zu aktualisieren, führen Sie `pipx upgrade pyxel` aus.

### Linux

Nachdem Sie [Python 3](https://www.python.org/) (Version 3.8 oder höher) installiert haben, führen Sie den folgenden Befehl aus:

```sh
pip install -U pyxel
```

Wenn der vorherige Befehl fehlschlägt, ziehen Sie in Betracht, Pyxel aus dem Quellcode zu bauen, indem Sie die Anweisungen im [Makefile](../Makefile) befolgen.

### Web

Die Webversion von Pyxel funktioniert auf PCs, Smartphones und Tablets mit einem kompatiblen Browser, ohne Python oder Pyxel zu installieren.

Der einfachste Weg, sie zu verwenden, ist über die Online-IDE [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/).

Für andere Nutzungsmuster, wie das Einbetten von Pyxel-Apps auf Ihrer eigenen Website, beziehen Sie sich bitte auf [diese Seite](pyxel-web-en.md).

## Grundlegende Verwendung

### Pyxel-Befehl

Durch die Installation von Pyxel wird der Befehl `pyxel` verfügbar. Geben Sie nach `pyxel` einen Befehlsnamen an, um verschiedene Operationen auszuführen.

Führen Sie ihn ohne Argumente aus, um die Liste der verfügbaren Befehle anzuzeigen:

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

### Beispiele ausführen

Mit dem folgenden Befehl können Sie die Beispiele in das aktuelle Verzeichnis kopieren:

```sh
pyxel copy_examples
```

In der lokalen Umgebung können die Beispiele mit den folgenden Befehlen ausgeführt werden:

```sh
# Beispiel im examples-Verzeichnis ausführen
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# App im examples/apps-Verzeichnis ausführen
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

Die Beispiele können auch auf [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/) im Browser angesehen und ausgeführt werden.

## Anwendungen erstellen

### Programm erstellen

Importieren Sie Pyxel in Ihr Python-Skript, geben Sie die Fenstergröße mit `init` an und starten Sie die Anwendung mit `run`.

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

Die Argumente der `run`-Funktion sind die `update`-Funktion, die die Frame-Aktualisierungen verarbeitet, und die `draw`-Funktion, die das Zeichnen auf dem Bildschirm übernimmt.

In einer tatsächlichen Anwendung wird empfohlen, den Pyxel-Code in einer Klasse zu kapseln, wie im Folgenden gezeigt:

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

Um einfache Grafiken ohne Animation zu erstellen, können Sie die `show`-Funktion verwenden, um Ihren Code zu vereinfachen.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Programm ausführen

Ein erstelltes Skript kann mit dem `python`-Befehl ausgeführt werden:

```sh
python PYTHON_SCRIPT_FILE
```

Es kann auch mit dem `pyxel run`-Befehl ausgeführt werden:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Darüber hinaus überwacht der Befehl `pyxel watch` Änderungen in einem angegebenen Verzeichnis und führt das Programm automatisch erneut aus, wenn Änderungen erkannt werden:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

Beenden Sie die Verzeichnisüberwachung mit `Ctrl(Command)+C`.

### Spezielle Tastenkombinationen

Während eine Pyxel-Anwendung läuft, können die folgenden speziellen Tastenaktionen ausgeführt werden:

- `Esc`<br>
  Die Anwendung beenden
- `Alt(Option)+R` oder `A+B+X+Y+BACK` auf dem Gamepad<br>
  Die Anwendung zurücksetzen
- `Alt(Option)+1`<br>
  Den Screenshot auf dem Desktop speichern
- `Alt(Option)+2`<br>
  Den Startzeitpunkt der Bildschirmaufnahme zurücksetzen
- `Alt(Option)+3`<br>
  Ein Bildschirmaufnahmevideo auf dem Desktop speichern (bis zu 10 Sekunden)
- `Alt(Option)+8` oder `A+B+X+Y+DL` auf dem Gamepad<br>
  Die Bildschirmskalierung zwischen maximal und ganzzahlig umschalten
- `Alt(Option)+9` oder `A+B+X+Y+DR` auf dem Gamepad<br>
  Zwischen Bildschirmmodi (Crisp/Smooth/Retro) wechseln
- `Alt(Option)+0` oder `A+B+X+Y+DU` auf dem Gamepad<br>
  Den Leistungsmonitor (FPS/`update`-Zeit/`draw`-Zeit) umschalten
- `Alt(Option)+Enter` oder `A+B+X+Y+DD` auf dem Gamepad<br>
  Den Vollbildmodus umschalten
- `Shift+Alt(Option)+1/2/3`<br>
  Bildbank 0, 1 oder 2 auf dem Desktop speichern
- `Shift+Alt(Option)+0`<br>
  Die aktuelle Farbpalette auf dem Desktop speichern

## Ressourcen erstellen

### Pyxel Editor

Pyxel Editor erstellt Bilder und Klänge, die in einer Pyxel-Anwendung verwendet werden.

Sie können Pyxel Editor mit dem folgenden Befehl starten:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Wenn die angegebene Pyxel-Ressourcendatei (.pyxres) vorhanden ist, wird sie geladen. Andernfalls wird eine neue Datei mit dem angegebenen Namen erstellt. Wenn die Ressourcendatei weggelassen wird, wird eine neue Datei mit dem Namen `my_resource.pyxres` erstellt.

Nachdem Sie Pyxel Editor gestartet haben, können Sie zu einer anderen Ressourcendatei wechseln, indem Sie sie auf den Editor ziehen und ablegen.

Die erstellte Ressourcendatei kann mit der `load`-Funktion geladen werden.

Pyxel Editor hat die folgenden Bearbeitungsmodi.

**Bildeditor**

Der Modus zum Bearbeiten von Bildern in jeder **Bilderbank**.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_editor.gif">
</a>

Sie können eine Bilddatei (PNG/GIF/JPEG) in den Bildeditor ziehen und ablegen, um das Bild in die aktuell ausgewählte Bilderbank zu laden.

**Kachelkarteeditor**

Der Modus zum Bearbeiten von **Kachelkarten**, in denen Bilder aus den Bilderbanken in einem Kachelmuster angeordnet sind.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/tilemap-editor.html">
<img src="images/tilemap_editor.gif">
</a>

Ziehen Sie eine TMX-Datei (Tiled Map File) in den Kachelkarteneditor, um deren Ebene 0 in die aktuell ausgewählte Kachelkarte zu laden.

**Klangeditor**

Der Modus zum Bearbeiten von **Klängen**, die für Melodien und Effekte verwendet werden.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
<img src="images/sound_editor.gif">
</a>

**Musikeditor**

Der Modus zum Bearbeiten von **Musiktracks**, in denen die Klänge in der Reihenfolge der Wiedergabe angeordnet sind.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/music-editor.html">
<img src="images/music_editor.gif">
</a>

### Andere Erstellungsmethoden

Pyxel-Bilder und Kachelkarten können auch mit folgenden Methoden erstellt werden:

- Erstellen Sie Bilder oder Kachelkarten aus Listen von Zeichenfolgen mit den Funktionen `Image.set` oder `Tilemap.set`
- Laden Sie eine zur Pyxel-Palette passende Bilddatei (PNG/GIF/JPEG) mit der Funktion `Image.load`

Pyxel-Klänge und -Musik können ebenfalls mit der folgenden Methode erstellt werden:

- Erstellen Sie sie aus Zeichenfolgen mit den Funktionen `Sound.set` oder `Music.set`

Bitte beachten Sie die API-Referenz für die Verwendung dieser Funktionen.

## Anwendungen verteilen

Pyxel unterstützt ein plattformübergreifendes Distributionsformat namens Pyxel-Anwendungsdatei.

Erstellen Sie eine Pyxel-Anwendungsdatei (.pyxapp) mit dem Befehl `pyxel package`:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Wenn Sie Ressourcen oder zusätzliche Module einfügen möchten, legen Sie diese im Anwendungsverzeichnis ab.

Metadaten können zur Laufzeit angezeigt werden, indem Sie sie im folgenden Format im Startskript angeben. Felder außer `title` und `author` sind optional.

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

Die erstellte Anwendungsdatei kann mit dem Befehl `pyxel play` ausgeführt werden:

```sh
pyxel play PYXEL_APP_FILE
```

Eine Pyxel-Anwendungsdatei kann auch mit den Befehlen `pyxel app2exe` oder `pyxel app2html` in eine ausführbare Datei oder eine HTML-Datei umgewandelt werden.

## API-Referenz

Eine vollständige Liste der Pyxel-APIs finden Sie unter [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/).

Pyxel enthält auch eine "Advanced API", die Fachwissen erfordert. Sie können sie anzeigen, indem Sie auf der Referenzseite das Kontrollkästchen "Advanced" aktivieren.

Wenn Sie Ihren Fähigkeiten vertrauen, versuchen Sie, mit der Advanced API beeindruckende Werke zu erstellen!

## Wie man beiträgt

### Probleme melden

Verwenden Sie den [Issue Tracker](https://github.com/kitao/pyxel/issues), um Fehlerberichte und Funktions- oder Verbesserungsanfragen einzureichen. Stellen Sie sicher, dass es vor der Einreichung eines neuen Problems keine ähnlichen offenen Probleme gibt.

### Funktionstest

Jeder, der den Code manuell testet und Fehler oder Verbesserungsvorschläge im [Issue Tracker](https://github.com/kitao/pyxel/issues) meldet, ist sehr willkommen!

### Pull-Requests einreichen

Patches und Fixes werden in Form von Pull-Requests (PRs) akzeptiert. Stellen Sie sicher, dass das Problem, das der Pull-Request behandelt, im Issue Tracker offen ist.

Die Einreichung eines Pull-Requests impliziert, dass Sie zustimmen, Ihren Beitrag unter der [MIT-Lizenz](../LICENSE) zu lizenzieren.

## Web-Tools & Beispiele

- [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)
- [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/)
- [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/)

## Weitere Informationen

- [FAQ](faq-en.md)
- [Benutzersamples](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Entwicklers X-Konto](https://x.com/kitao)
- [Discord-Server (Englisch)](https://discord.gg/Z87eYHN)
- [Discord-Server (Japanisch)](https://discord.gg/qHA5BCS)

## Lizenz

Pyxel ist lizenziert unter der [MIT-Lizenz](../LICENSE). Es kann in proprietärer Software wiederverwendet werden, vorausgesetzt, dass alle Kopien der Software oder wesentliche Teile davon eine Kopie der MIT-Lizenzbedingungen und einen Copyright-Hinweis enthalten.

## Sponsoren suchen

Pyxel sucht Sponsoren auf GitHub Sponsors. Bitte ziehen Sie in Betracht, Pyxel zu sponsern, um dessen fortlaufende Wartung und Funktionsentwicklung zu unterstützen. Als Vorteil können Sponsoren direkt mit dem Pyxel-Entwickler beraten. Für weitere Details besuchen Sie bitte [diese Seite](https://github.com/sponsors/kitao).
