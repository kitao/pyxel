# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** (/ˈpɪksəl/) è un motore di gioco retro per Python.

Le specifiche sono ispirate alle console di gioco retro, come il supporto di solo 16 colori e 4 canali audio, permettendoti di divertirti facilmente a creare giochi in stile pixel art.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

Lo sviluppo di Pyxel è guidato dai feedback degli utenti. Ti preghiamo di dare una stella a Pyxel su GitHub!

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

Le specifiche e le API di Pyxel sono ispirate a [PICO-8](https://www.lexaloffle.com/pico-8.php) e [TIC-80](https://tic80.com/).

Pyxel è open source sotto la [Licenza MIT](../LICENSE) ed è gratuito da usare. Iniziamo a creare giochi retro con Pyxel!

## Specifiche

- Funziona su Windows, Mac, Linux e Web
- Programmazione in Python
- Dimensione dello schermo personalizzabile
- Palette di 16 colori
- 3 banche di immagini 256x256
- 8 mappe a tessere 256x256
- 4 canali con 64 suoni definibili
- 8 tracce musicali in grado di combinare qualsiasi suono
- Input da tastiera, mouse e gamepad
- Strumenti di editing per immagini e suoni
- Colori, canali audio e banche estensibili dall'utente

### Palette colori

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Come installare

### Windows

Dopo aver installato [Python 3](https://www.python.org/) (versione 3.8 o superiore), esegui il seguente comando:

```sh
pip install -U pyxel
```

Quando installi Python usando l'installatore ufficiale, assicurati di selezionare l'opzione `Add Python 3.x to PATH` per abilitare il comando `pyxel`.

### Mac

Dopo aver installato [Homebrew](https://brew.sh/), esegui i seguenti comandi:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Per aggiornare Pyxel dopo l'installazione, esegui `pipx upgrade pyxel`.

### Linux

Dopo aver installato [Python 3](https://www.python.org/) (versione 3.8 o superiore), esegui il seguente comando:

```sh
pip install -U pyxel
```

Se il comando precedente non funziona, considera di costruire Pyxel da sorgente seguendo le istruzioni nel [Makefile](../Makefile).

### Web

La versione web di Pyxel funziona su PC, smartphone e tablet con un browser compatibile, senza installare Python o Pyxel.

Il modo più semplice per utilizzarla è attraverso l'IDE online [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/).

Per altri modelli di utilizzo, come l'incorporamento di app Pyxel nel proprio sito, fai riferimento a [questa pagina](pyxel-web-en.md).

## Utilizzo di base

### Comando Pyxel

L'installazione di Pyxel aggiunge il comando `pyxel`. Specificare un nome di comando dopo `pyxel` per eseguire varie operazioni.

Eseguirlo senza argomenti per visualizzare l'elenco dei comandi disponibili:

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

### Provare gli esempi

Il seguente comando copia gli esempi di Pyxel nella directory corrente:

```sh
pyxel copy_examples
```

In ambiente locale, gli esempi possono essere eseguiti con i seguenti comandi:

```sh
# Eseguire l'esempio nella directory examples
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# Eseguire l'app nella directory examples/apps
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

Gli esempi possono anche essere visualizzati ed eseguiti nel browser da [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/).

## Creazione di applicazioni

### Creare un programma

Nel tuo script Python, importa Pyxel, specifica le dimensioni della finestra con `init` e avvia l'applicazione con `run`.

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

Gli argomenti della funzione `run` sono la funzione `update`, che gestisce gli aggiornamenti dei fotogrammi, e la funzione `draw`, che gestisce il disegno sullo schermo.

In un'applicazione reale, è consigliabile incapsulare il codice Pyxel in una classe, come mostrato di seguito:

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

Per creare grafica semplice senza animazione, puoi utilizzare la funzione `show` per semplificare il tuo codice.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Eseguire un programma

Uno script creato può essere eseguito utilizzando il comando `python`:

```sh
python PYTHON_SCRIPT_FILE
```

Può anche essere eseguito con il comando `pyxel run`:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Inoltre, il comando `pyxel watch` monitora le modifiche in una directory specificata e ri-esegue automaticamente il programma quando vengono rilevati cambiamenti:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

Interrompi il monitoraggio della directory premendo `Ctrl(Command)+C`.

### Comandi speciali dei tasti

Durante l'esecuzione di un'applicazione Pyxel, possono essere eseguite le seguenti operazioni delle chiavi speciali:

- `Esc`<br>
  Uscire dall'applicazione
- `Alt(Option)+R` oppure `A+B+X+Y+BACK` sul gamepad<br>
  Reimpostare l'applicazione
- `Alt(Option)+1`<br>
  Salvare lo screenshot sul desktop
- `Alt(Option)+2`<br>
  Reimpostare il tempo di inizio della registrazione del video di cattura dello schermo
- `Alt(Option)+3`<br>
  Salvare un video di cattura dello schermo sul desktop (fino a 10 secondi)
- `Alt(Option)+8` oppure `A+B+X+Y+DL` sul gamepad<br>
  Cambiare la scala dello schermo tra massima e intera
- `Alt(Option)+9` oppure `A+B+X+Y+DR` sul gamepad<br>
  Passare tra le modalità dello schermo (Crisp/Smooth/Retro)
- `Alt(Option)+0` oppure `A+B+X+Y+DU` sul gamepad<br>
  Attivare/disattivare il monitor delle prestazioni (FPS/tempo di `update`/tempo di `draw`)
- `Alt(Option)+Enter` oppure `A+B+X+Y+DD` sul gamepad<br>
  Visualizzare a schermo intero
- `Shift+Alt(Option)+1/2/3`<br>
  Salvare la banca di immagini 0, 1 o 2 sul desktop
- `Shift+Alt(Option)+0`<br>
  Salvare la palette di colori corrente sul desktop

## Creazione di risorse

### Pyxel Editor

Pyxel Editor crea immagini e suoni utilizzati in un'applicazione Pyxel.

Puoi avviare Pyxel Editor con il seguente comando:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Se il file di risorse Pyxel specificato (.pyxres) esiste, verrà caricato. Se non esiste, verrà creato un nuovo file con il nome specificato. Se il file di risorse viene omesso, verrà creato un nuovo file chiamato `my_resource.pyxres`.

Dopo aver avviato Pyxel Editor, puoi passare a un altro file di risorse trascinandolo e rilasciandolo sull'editor.

Il file di risorse creato può essere caricato utilizzando la funzione `load`.

Pyxel Editor ha i seguenti modi di editing.

**Editor di Immagini**

Il modo per modificare le immagini in ciascuna **banca di immagini**.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_editor.gif">
</a>

Puoi trascinare e rilasciare un file immagine (PNG/GIF/JPEG) nell'editor di immagini per caricare l'immagine nella banca di immagini attualmente selezionata.

**Editor di Mappe a Tessere**

Il modo per modificare le **mappe a tessere** in cui le immagini delle banche di immagini sono disposte in un modello di piastrelle.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/tilemap-editor.html">
<img src="images/tilemap_editor.gif">
</a>

Trascina e rilascia un file TMX (Tiled Map File) nell'editor di mappe a tessere per caricare il suo strato 0 nella mappa a tessere attualmente selezionata.

**Editor di Suoni**

Il modo per modificare i **suoni** utilizzati per le melodie e gli effetti sonori.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
<img src="images/sound_editor.gif">
</a>

**Editor di Musica**

Il modo per modificare le **tracce musicali** in cui i suoni sono disposti in ordine di riproduzione.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/music-editor.html">
<img src="images/music_editor.gif">
</a>

### Altri metodi di creazione

Le immagini e le mappe a tessere di Pyxel possono anche essere create utilizzando i seguenti metodi:

- Crea immagini o mappe a tessere da elenchi di stringhe con le funzioni `Image.set` o `Tilemap.set`
- Carica file immagine compatibili con la palette Pyxel (PNG/GIF/JPEG) con la funzione `Image.load`

I suoni e la musica di Pyxel possono anche essere creati utilizzando il seguente metodo:

- Creali da stringhe con le funzioni `Sound.set` o `Music.set`

Fai riferimento alla documentazione dell'API per l'uso di queste funzioni.

## Distribuzione delle applicazioni

Pyxel supporta un formato di distribuzione multipiattaforma chiamato file dell'applicazione Pyxel.

Crea un file dell'applicazione Pyxel (.pyxapp) con il comando `pyxel package`:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Se hai bisogno di includere risorse o moduli aggiuntivi, posizionali nella directory dell'applicazione.

I metadati possono essere visualizzati durante l'esecuzione specificandoli nel seguente formato all'interno dello script di avvio. I campi diversi da `title` e `author` sono facoltativi.

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

Il file dell'applicazione creato può essere eseguito utilizzando il comando `pyxel play`:

```sh
pyxel play PYXEL_APP_FILE
```

Un file dell'applicazione Pyxel può anche essere convertito in un eseguibile o in un file HTML utilizzando i comandi `pyxel app2exe` o `pyxel app2html`.

## Riferimento API

L’elenco completo delle API di Pyxel è disponibile su [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/).

Pyxel include anche una "API avanzata" che richiede conoscenze specializzate. Puoi visualizzarla selezionando la casella "Advanced" nella pagina di riferimento.

Se sei sicuro delle tue capacità, prova a usare l’API avanzata per creare opere davvero sorprendenti!

## Come Contribuire

### Segnalare Problemi

Utilizza il [Issue Tracker](https://github.com/kitao/pyxel/issues) per inviare rapporti di bug e richieste di funzionalità o miglioramenti. Prima di inviare un nuovo problema, assicurati che non ci siano problemi aperti simili.

### Test Funzionali

Chiunque testi manualmente il codice e segnali bug o suggerimenti per miglioramenti nel [Issue Tracker](https://github.com/kitao/pyxel/issues) è molto benvenuto!

### Inviare Richieste di Pull

Le patch e le correzioni sono accettate sotto forma di richieste di pull (PR). Assicurati che il problema a cui si riferisce la richiesta di pull sia aperto nel Issue Tracker.

Inviare una richiesta di pull implica che accetti di concedere in licenza il tuo contributo sotto la [Licenza MIT](../LICENSE).

## Strumenti Web e Esempi

- [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)
- [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/) [[User Manual](https://qiita.com/kitao/items/b5b3fb28ebf9781eda2e)]
- [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/) [[User Manual](https://qiita.com/kitao/items/a86de4f7d6a0ed656a89)]

## Altre Informazioni

- [FAQ](faq-en.md)
- [Esempi degli Utenti](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Account X dello Sviluppatore](https://x.com/kitao)
- [Server Discord (Inglese)](https://discord.gg/Z87eYHN)
- [Server Discord (Giapponese)](https://discord.gg/qHA5BCS)

## Licenza

Pyxel è concesso in licenza sotto la [Licenza MIT](../LICENSE). Può essere riutilizzato in software proprietari, a condizione che tutte le copie del software o delle sue parti sostanziali includano una copia dei termini della Licenza MIT e un avviso di copyright.

## Ricerca di Sponsor

Pyxel sta cercando sponsor su GitHub Sponsors. Ti preghiamo di considerare l'idea di sponsorizzare Pyxel per supportarne la manutenzione continua e lo sviluppo di nuove funzionalità. Come beneficio, gli sponsor possono consultare direttamente lo sviluppatore di Pyxel. Per ulteriori dettagli, visita [questa pagina](https://github.com/sponsors/kitao).
