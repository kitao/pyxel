# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** è un motore di gioco retro per Python.

Le specifiche sono ispirate alle console di gioco retro, come il supporto di solo 16 colori e 4 canali audio, permettendoti di divertirti facilmente a creare giochi in stile pixel art.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

Lo sviluppo di Pyxel è guidato dai feedback degli utenti. Ti preghiamo di dare una stella a Pyxel su GitHub!

<p>
<a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">
<img src="images/10_platformer.gif" width="290">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/30sec_of_daylight.html">
<img src="images/30sec_of_daylight.gif" width="350">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">
<img src="images/02_jump_game.gif" width="330">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">
<img src="images/megaball.gif" width="310">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
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
- 3 banche di immagini di 256x256
- 8 mappe a tessere di 256x256
- 4 canali con 64 suoni definibili
- 8 tracce musicali che possono combinare qualsiasi suono
- Input da tastiera, mouse e gamepad
- Strumenti di editing per immagini e suoni
- Colori, canali e banche estensibili dall'utente

### Palette colori

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Come installare

### Windows

Dopo aver installato [Python3](https://www.python.org/) (versione 3.8 o superiore), esegui il seguente comando:

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

Dopo aver installato il pacchetto SDL2 (`libsdl2-dev` per Ubuntu), [Python3](https://www.python.org/) (versione 3.8 o superiore) e `python3-pip`, esegui il seguente comando:

```sh
sudo pip3 install -U pyxel
```

Se il comando precedente non funziona, considera di costruire Pyxel da sorgente seguendo le istruzioni nel [Makefile](../Makefile).

### Web

La versione Web di Pyxel non richiede l'installazione di Python o Pyxel e funziona su PC, smartphone e tablet con browser web supportati.

Per istruzioni dettagliate, fai riferimento a [questa pagina](pyxel-web-en.md).

### Eseguire esempi

Dopo aver installato Pyxel, puoi copiare gli esempi nella directory corrente con il seguente comando:

```sh
pyxel copy_examples
```

I seguenti esempi saranno copiati nella tua directory corrente:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>Applicazione più semplice</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Codice</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Gioco di salti con file di risorse Pyxel</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Codice</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Dimostrazione delle API di disegno</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Codice</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Dimostrazione delle API audio</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Codice</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>Elenco delle palette di colori</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Codice</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>Gioco di clic del mouse</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Codice</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>Gioco della serpente con BGM</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Codice</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Dimostrazione delle API di disegno di triangoli</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Codice</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Shoot'em up con transizioni di schermo e MML</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Codice</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>Gioco di piattaforma a scorrimento laterale con mappa</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Codice</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Rendering offscreen con la classe Image</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Codice</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>Animazione di rumore di Perlin</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Codice</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>Disegnare un font bitmap</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Codice</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>Sintetizzatore che utilizza funzionalità di espansione audio</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Codice</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>Caricamento e disegno di un Tiled Map File (.tmx)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Codice</a></td>
</tr>
<tr>
<td>16_transform.py</td>
<td>Rotazione e ridimensionamento delle immagini</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/16_transform.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/16_transform.py">Codice</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>Animazione con la funzione flip (solo per piattaforme non web)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Codice</a></td>
</tr>
<tr>
<td>30sec_of_daylight.pyxapp</td>
<td>Gioco vincitore del 1° Pyxel Jam di <a href="https://x.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30sec_of_daylight.html">Demo</a></td>
<td><a href="https://github.com/kitao/30SecondsOfDaylight">Codice</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>Gioco arcade di fisica della palla di <a href="https://x.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Demo</a></td>
<td><a href="https://github.com/kitao/megaball">Codice</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>Generatore di musica di sottofondo di <a href="https://x.com/frenchbread1222">frenchbread</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">Demo</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">Codice</a></td>
</tr>
</table>

Gli esempi possono essere eseguiti con i seguenti comandi:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30sec_of_daylight.pyxapp
```

## Come usare

### Creare un'applicazione

Nel tuo script Python, importa il modulo Pyxel, specifica le dimensioni della finestra con la funzione `init`, e poi avvia l'applicazione Pyxel con la funzione `run`.

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

### Eseguire l'applicazione

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

Il monitoraggio della directory può essere interrotto premendo `Ctrl(Command)+C`.

### Operazioni delle chiavi speciali

Durante l'esecuzione di un'applicazione Pyxel, possono essere eseguite le seguenti operazioni delle chiavi speciali:

- `Esc`<br>
  Uscire dall'applicazione
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
  Visualizzare il monitor delle prestazioni (FPS/`update` tempo/`draw` tempo)
- `Alt(Option)+Enter` oppure `A+B+X+Y+DD` sul gamepad<br>
  Visualizzare a schermo intero
- `Shift+Alt(Option)+1/2/3`<br>
  Salvare la banca di immagini 0, 1 o 2 sul desktop
- `Shift+Alt(Option)+0`<br>
  Salvare la palette di colori corrente sul desktop

### Come creare risorse

Pyxel Editor può creare immagini e suoni utilizzati in un'applicazione Pyxel.

Puoi avviare Pyxel Editor con il seguente comando:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Se il file di risorse Pyxel specificato (.pyxres) esiste, verrà caricato. Se non esiste, verrà creato un nuovo file con il nome specificato. Se il file di risorse viene omesso, verrà creato un nuovo file chiamato `my_resource.pyxres`.

Dopo aver avviato Pyxel Editor, puoi passare a un altro file di risorse trascinandolo e rilasciandolo su Pyxel Editor.

Il file di risorse creato può essere caricato utilizzando la funzione `load`.

Pyxel Editor ha i seguenti modi di editing.

**Editor di Immagini**

Il modo per modificare l'immagine in ciascuna **banca di immagini**.

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

Puoi trascinare e rilasciare un file immagine (PNG/GIF/JPEG) nell'editor di immagini per caricare l'immagine nella banca di immagini attualmente selezionata.

**Editor di Mappe a Tessere**

Il modo per modificare le **mappe a tessere** in cui le immagini delle banche di immagini sono disposte in un modello di piastrelle.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

Trascina e rilascia un file TMX (Tiled Map File) nell'editor di mappe a tessere per caricare il suo strato 0 nella mappa a tessere attualmente selezionata.

**Editor di Suoni**

Il modo per modificare i **suoni** utilizzati per le melodie e gli effetti sonori.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**Editor di Musica**

Il modo per modificare le **musiche** in cui i suoni sono disposti in ordine di riproduzione.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### Altri metodi di creazione delle risorse

Le immagini e le mappe a tessere di Pyxel possono anche essere create utilizzando i seguenti metodi:

- Crea un'immagine da un elenco di stringhe utilizzando la funzione `Image.set` o la funzione `Tilemap.set`
- Carica un file immagine (PNG/GIF/JPEG) nella palette Pyxel con la funzione `Image.load`

I suoni Pyxel possono anche essere creati utilizzando il seguente metodo:

- Crea un suono da stringhe con la funzione `Sound.set` o la funzione `Music.set`

Fai riferimento alla documentazione dell'API per l'uso di queste funzioni.

### Come distribuire le applicazioni

Pyxel supporta un formato di file di distribuzione dell'applicazione dedicato (file dell'applicazione Pyxel) che è multipiattaforma.

Un file dell'applicazione Pyxel (.pyxapp) viene creato utilizzando il comando `pyxel package`:

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

### Sistema

- `width`, `height`<br>
  La larghezza e l'altezza dello schermo

- `frame_count`<br>
  Il numero di frame trascorsi

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Inizializza l'applicazione Pyxel con la dimensione dello schermo (`width`, `height`). Le seguenti opzioni possono essere specificate: il titolo della finestra con `title`, il frame rate con `fps`, il tasto per chiudere l'applicazione con `quit_key`, la scala di visualizzazione con `display_scale`, la scala di acquisizione dello schermo con `capture_scale` e il tempo massimo di registrazione del video di acquisizione dello schermo con `capture_sec`.<br>
  Esempio: `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Avvia l'applicazione Pyxel e chiama la funzione `update` per l'aggiornamento del frame e la funzione `draw` per il disegno.

- `show()`<br>
  Mostra lo schermo e attende che venga premuto il tasto `Esc`.

- `flip()`<br>
  Aggiorna lo schermo di un frame. L'applicazione si chiude quando viene premuto il tasto `Esc`. Questa funzione non è disponibile nella versione web.

- `quit()`<br>
  Chiude l'applicazione Pyxel.

### Risorsa

- `load(filename, [excl_images], [excl_tilemaps], [excl_sounds], [excl_musics])`<br>
  Carica il file di risorse (.pyxres). Se un'opzione è impostata su `True`, la risorsa corrispondente sarà esclusa dal caricamento. Se esiste un file di palette (.pyxpal) con lo stesso nome nella stessa posizione del file di risorse, anche i colori della palette verranno aggiornati. Il file di palette contiene voci esadecimali per i colori di visualizzazione (ad esempio `1100ff`), separate da ritorni a capo. Il file di palette può essere utilizzato anche per modificare i colori visualizzati nell'editor Pyxel.

- `user_data_dir(vendor_name, app_name)`<br>
  Restituisce la directory dei dati utente creata in base a `vendor_name` e `app_name`. Se la directory non esiste, verrà creata automaticamente. Viene utilizzata per memorizzare punteggi alti, progressi del gioco e dati simili.<br>
  Esempio: `print(pyxel.user_data_dir("Takashi Kitao", "Pyxel Shooter"))`

### Input

- `mouse_x`, `mouse_y`<br>
  La posizione corrente del cursore del mouse

- `mouse_wheel`<br>
  Il valore corrente della rotella del mouse

- `btn(key)`<br>
  Restituisce `True` se il tasto `key` è premuto, altrimenti restituisce `False`. ([Elenco delle definizioni dei tasti](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  Restituisce `True` se il tasto `key` è premuto in quel frame, altrimenti restituisce `False`. Se vengono specificati `hold` e `repeat`, dopo che il tasto `key` è stato tenuto premuto per `hold` frame o più, `True` viene restituito ogni `repeat` frame.

- `btnr(key)`<br>
  Restituisce `True` se il tasto `key` viene rilasciato in quel frame, altrimenti restituisce `False`.

- `mouse(visible)`<br>
  Mostra il cursore del mouse se `visible` è `True` e lo nasconde se `visible` è `False`. La posizione del cursore continua ad aggiornarsi anche quando è nascosto.

### Grafica

- `colors`<br>
  Elenco dei colori della palette. Il colore di visualizzazione è specificato da un valore numerico a 24 bit. Usa `colors.from_list` e `colors.to_list` per assegnare e recuperare direttamente le liste Python.<br>
  Esempio: `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Elenco delle banche di immagini (istanze della classe Image) (0-2)<br>
  Esempio: `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Elenco delle mappe a tessere (istanze della classe Tilemap) (0-7)

- `clip(x, y, w, h)`<br>
  Imposta l'area di disegno dello schermo da (`x`, `y`) con una larghezza di `w` e un'altezza di `h`. Chiama `clip()` per reimpostare l'area di disegno a tutto schermo.

- `camera(x, y)`<br>
  Cambia le coordinate dell'angolo in alto a sinistra dello schermo a (`x`, `y`). Chiama `camera()` per reimpostare le coordinate dell'angolo in alto a sinistra a (`0`, `0`).

- `pal(col1, col2)`<br>
  Sostituisce il colore `col1` con `col2` durante il disegno. Chiama `pal()` per reimpostare la palette iniziale.

- `dither(alpha)`<br>
  Applica un dithering (pseudo-trasparenza) durante il disegno. Imposta `alpha` nell'intervallo `0.0`-`1.0`, dove `0.0` è trasparente e `1.0` è opaco.

- `cls(col)`<br>
  Pulisce lo schermo con il colore `col`.

- `pget(x, y)`<br>
  Ottiene il colore del pixel a (`x`, `y`).

- `pset(x, y, col)`<br>
  Disegna un pixel con il colore `col` a (`x`, `y`).

- `line(x1, y1, x2, y2, col)`<br>
  Disegna una linea di colore `col` da (`x1`, `y1`) a (`x2`, `y2`).

- `rect(x, y, w, h, col)`<br>
  Disegna un rettangolo di larghezza `w`, altezza `h` e colore `col` da (`x`, `y`).

- `rectb(x, y, w, h, col)`<br>
  Disegna il contorno di un rettangolo di larghezza `w`, altezza `h` e colore `col` da (`x`, `y`).

- `circ(x, y, r, col)`<br>
  Disegna un cerchio di raggio `r` e colore `col` a (`x`, `y`).

- `circb(x, y, r, col)`<br>
  Disegna il contorno di un cerchio di raggio `r` e colore `col` a (`x`, `y`).

- `elli(x, y, w, h, col)`<br>
  Disegna un'ellisse di larghezza `w`, altezza `h` e colore `col` da (`x`, `y`).

- `ellib(x, y, w, h, col)`<br>
  Disegna il contorno di un'ellisse di larghezza `w`, altezza `h` e colore `col` da (`x`, `y`).

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  Disegna un triangolo con vertici (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e colore `col`.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  Disegna il contorno di un triangolo con vertici (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e colore `col`.

- `fill(x, y, col)`<br>
  Riempi l'area connessa con lo stesso colore di (`x`, `y`) con il colore `col`.

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Copia la regione di dimensioni (`w`, `h`) da (`u`, `v`) della banca immagini `img`(0-2) a (`x`, `y`). Se viene assegnato un valore negativo a `w` e/o `h`, la regione verrà capovolta orizzontalmente e/o verticalmente. Se `colkey` è specificato, verrà trattato come un colore trasparente. Se vengono specificati `rotate` (in gradi), `scale` (1.0 = 100%) o entrambi, verranno applicate le trasformazioni corrispondenti.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Copia la regione di dimensioni (`w`, `h`) da (`u`, `v`) della mappa a tessere `tm`(0-7) a (`x`, `y`). Se viene assegnato un valore negativo a `w` e/o `h`, la regione verrà capovolta orizzontalmente e/o verticalmente. Se `colkey` è specificato, verrà trattato come un colore trasparente. Se vengono specificati `rotate` (in gradi), `scale` (1.0 = 100%) o entrambi, verranno applicate le trasformazioni corrispondenti. La dimensione di una tessera è 8x8 pixel ed è memorizzata in una mappa a tessere come una tupla `(image_tx, image_ty)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  Disegna una stringa `s` di colore `col` a (`x`, `y`).

### Audio

- `sounds`<br>
  Elenco dei suoni (istanze della classe Sound) (0-63)<br>
  Esempio: `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Elenco delle musiche (istanze della classe Music) (0-7)

- `play(ch, snd, [tick], [loop], [resume])`<br>
  Riproduce il suono `snd`(0-63) sul canale `ch`(0-3). Se `snd` è un elenco, i suoni verranno riprodotti in sequenza. La posizione di partenza può essere specificata da `tick` (1 tick = 1/120 secondi). Se `loop` è impostato su `True`, la riproduzione verrà eseguita in loop. Per riprendere il suono precedente dopo la fine della riproduzione, impostare `resume` su `True`.

- `playm(msc, [tick], [loop])`<br>
  Riproduce la musica `msc`(0-7). La posizione di partenza può essere specificata da `tick` (1 tick = 1/120 secondi). Se `loop` è impostato su `True`, la riproduzione verrà eseguita in loop.

- `stop([ch])`<br>
  Interrompe la riproduzione del canale specificato `ch`(0-3). Chiama `stop()` per interrompere tutti i canali.

- `play_pos(ch)`<br>
  Ottiene la posizione di riproduzione del suono sul canale `ch`(0-3) sotto forma di tupla `(sound_no, note_no)`. Restituisce `None` quando la riproduzione è terminata.

### Matematica

- `ceil(x)`<br>
  Restituisce il numero intero più piccolo che è maggiore o uguale a `x`.

- `floor(x)`<br>
  Restituisce il numero intero più grande che è minore o uguale a `x`.

- `sgn(x)`<br>
  Restituisce `1` se `x` è positivo, `0` se è `0`, e `-1` se è negativo.

- `sqrt(x)`<br>
  Restituisce la radice quadrata di `x`.

- `sin(deg)`<br>
  Restituisce il seno di `deg` gradi.

- `cos(deg)`<br>
  Restituisce il coseno di `deg` gradi.

- `atan2(y, x)`<br>
  Restituisce l'arcotangente di `y`/`x` in gradi.

- `rseed(seed)`<br>
  Imposta il seme del generatore di numeri casuali.

- `rndi(a, b)`<br>
  Restituisce un numero intero casuale maggiore o uguale a `a` e minore o uguale a `b`.

- `rndf(a, b)`<br>
  Restituisce un numero in virgola mobile casuale maggiore o uguale a `a` e minore o uguale a `b`.

- `nseed(seed)`<br>
  Imposta il seme del rumore Perlin.

- `noise(x, [y], [z])`<br>
  Restituisce il valore del rumore Perlin per le coordinate specificate.

### Classe Image

- `width`, `height`<br>
  La larghezza e l'altezza dell'immagine

- `set(x, y, data)`<br>
  Imposta l'immagine a (`x`, `y`) utilizzando un elenco di stringhe.<br>
  Esempio: `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Carica un file immagine (PNG/GIF/JPEG) a (`x`, `y`).

- `pget(x, y)`<br>
  Ottiene il colore del pixel a (`x`, `y`).

- `pset(x, y, col)`<br>
  Disegna un pixel con il colore `col` a (`x`, `y`).

### Classe Tilemap

- `width`, `height`<br>
  La larghezza e l'altezza della mappa a tessere

- `imgsrc`<br>
  La banca immagini (0-2) a cui fa riferimento la mappa a tessere

- `set(x, y, data)`<br>
  Imposta la mappa a tessere a (`x`, `y`) utilizzando un elenco di stringhe.<br>
  Esempio: `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  Carica il `layer`(0-) dal file TMX (Tiled Map File) a (`x`, `y`).

- `pget(x, y)`<br>
  Ottiene la tessera a (`x`, `y`). Una tessera è rappresentata come una tupla `(image_tx, image_ty)`.

- `pset(x, y, tile)`<br>
  Disegna una `tessera` a (`x`, `y`). Una tessera è rappresentata come una tupla `(image_tx, image_ty)`.

### Classe Sound

- `notes`<br>
  Elenco di note (0-127). Più alto è il numero, più acuto è il suono. La nota `33` corrisponde a 'A2'(440Hz). Le note di silenzio sono rappresentate da `-1`.

- `tones`<br>
  Elenco di toni (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
  Elenco di volumi (0-7)

- `effects`<br>
  Elenco di effetti (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  Velocità di riproduzione. `1` è la più veloce, e più alto è il numero, più lenta è la velocità di riproduzione. A `120`, la durata di una nota diventa 1 secondo.

- `set(notes, tones, volumes, effects, speed)`<br>
  Imposta note, toni, volumi ed effetti utilizzando una stringa. Se la lunghezza di toni, volumi o effetti è inferiore alle note, verranno ripetuti dall'inizio.

- `set_notes(notes)`<br>
  Imposta le note utilizzando una stringa composta da `CDEFGAB`+`#-`+`01234` o `R`. Non fa distinzione tra maiuscole e minuscole, e gli spazi bianchi vengono ignorati.<br>
  Esempio: `pyxel.sounds[0].set_notes("g2b-2d3r rf3f3f3")`

- `set_tones(tones)`<br>
  Imposta i toni con una stringa composta da `TSPN`. Non fa distinzione tra maiuscole e minuscole, e gli spazi bianchi vengono ignorati.<br>
  Esempio: `pyxel.sounds[0].set_tones("ttss pppn")`

- `set_volumes(volumes)`<br>
  Imposta i volumi con una stringa composta da `01234567`. Non fa distinzione tra maiuscole e minuscole, e gli spazi bianchi vengono ignorati.<br>
  Esempio: `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  Imposta gli effetti con una stringa composta da `NSVFHQ`. Non fa distinzione tra maiuscole e minuscole, e gli spazi bianchi vengono ignorati.<br>
  Esempio: `pyxel.sounds[0].set_effects("nfnf nvvs")`

- `mml(mml_str)`<br>
  Imposta i parametri correlati utilizzando [Music Macro Language (MML)](https://en.wikipedia.org/wiki/Music_Macro_Language). I comandi disponibili sono `T`(1-900), `@`(0-3), `O`(0-4), `>`, `<`, `Q`(1-8), `V`(0-7), `X`(0-7), `L`(1/2/4/8/16/32) e `CDEFGABR`+`#+-`+`.~&`. Per maggiori dettagli sui comandi, consulta [questa pagina](faq-en.md).<br>
  Esempio: `pyxel.sounds[0].mml("t120 @1 o3 q6 l8 x0:12345 c4&c<g16r16>c.<g16 v4 >c.&d16 x0 e2~c2~")`

- `save(filename, count, [ffmpeg])`<br>
  Crea un file WAV contenente il suono ripetuto `count` volte. Se FFmpeg è installato e `ffmpeg` è impostato su `True`, viene creato anche un file MP4.

### Classe Music

- `seqs`<br>
  Un elenco bidimensionale di suoni (0-63) su più canali

- `set(seq0, seq1, seq2, ...)`<br>
  Imposta gli elenchi di suoni (0-63) per ciascun canale. Se viene specificato un elenco vuoto, quel canale non verrà utilizzato per la riproduzione.<br>
  Esempio: `pyxel.musics[0].set([0, 1], [], [3])`

- `save(filename, count, [ffmpeg])`<br>
  Crea un file WAV contenente la musica ripetuta `count` volte. Se FFmpeg è installato e `ffmpeg` è impostato su `True`, viene creato anche un file MP4.

### API Avanzata

Pyxel include un'"API Avanzata" che non è menzionata in questo documento, poiché potrebbe confondere gli utenti o richiedere conoscenze specialistiche per l'uso.

Se sei sicuro delle tue capacità, prova a creare opere straordinarie utilizzando [questo](../python/pyxel/__init__.pyi) come guida!

## Come Contribuire

### Segnalare Problemi

Utilizza il [Issue Tracker](https://github.com/kitao/pyxel/issues) per inviare rapporti di bug e richieste di funzionalità o miglioramenti. Prima di inviare un nuovo problema, assicurati che non ci siano problemi aperti simili.

### Test Funzionali

Chiunque testi manualmente il codice e segnali bug o suggerimenti per miglioramenti nel [Issue Tracker](https://github.com/kitao/pyxel/issues) è molto benvenuto!

### Inviare Richieste di Pull

Le patch e le correzioni sono accettate sotto forma di richieste di pull (PR). Assicurati che il problema a cui si riferisce la richiesta di pull sia aperto nel Issue Tracker.

Inviare una richiesta di pull implica che accetti di concedere in licenza il tuo contributo sotto la [Licenza MIT](../LICENSE).

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
