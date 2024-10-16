# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** è un motore di gioco retro per Python.

Le specifiche sono ispirate alle console di gioco retro, come il supporto di solo 16 colori e 4 canali audio, permettendoti di divertirti facilmente a creare giochi in stile pixel art.

<img src="images/pyxel_message.png" width="480">

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
- Palette di 16 colori
- 3 banche immagini di 256x256
- 8 tilemap di 256x256
- 4 canali con 64 suoni definibili
- 8 musiche che possono combinare qualsiasi suono
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
<td>Shoot 'em up con transizioni di schermo</td>
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
<td><a href="https://github.com/kitao/30sec_of_daylight">Codice</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>Gioco arcade di fisica della palla di <a href="https://x.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Demo</a></td>
<td><a href="https://github.com/helpcomputer/megaball">Codice</a></td>
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
  Ripristinare il tempo di inizio della registrazione del video di cattura dello schermo
- `Alt(Option)+3`<br>
  Salvare un video di cattura dello schermo sul desktop (fino a 10 secondi)
- `Alt(Option)+9`<br>
  Passare tra le modalità dello schermo (Crisp/Smooth/Retro)
- `Alt(Option)+0`<br>
  Attivare il monitor delle prestazioni (FPS/`update` tempo/`draw` tempo)
- `Alt(Option)+Enter`<br>
  Attivare la modalità a schermo intero
- `Shift+Alt(Option)+1/2/3`<br>
  Salvare la corrispondente banca delle immagini sul desktop
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

Il modo per modificare le banche di immagini.

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

Puoi trascinare e rilasciare un file immagine (PNG/GIF/JPEG) nell'editor di immagini per caricare l'immagine nella banca di immagini attualmente selezionata.

**Editor di Tilemap**

Il modo per modificare i tilemap in cui le immagini delle banche di immagini sono disposte in un modello di piastrelle.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

Trascina e rilascia un file TMX (Tiled Map File) sull'editor di tilemap per caricare il suo strato nell'ordine di disegno corrispondente al numero di tilemap attualmente selezionato.

**Editor di Suoni**

Il modo per modificare i suoni.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**Editor di Musica**

Il modo per modificare le musics in cui i suoni sono disposti in ordine di riproduzione.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### Altri metodi di creazione delle risorse

Le immagini e i tilemap Pyxel possono anche essere creati utilizzando i seguenti metodi:

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
  Lunghezza e altezza dello schermo

- `frame_count`<br>
  Numero di frame passati

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Inizializza l'applicazione Pyxel con la dimensione dello schermo (`width`, `height`). I seguenti possono essere specificati come opzioni: il titolo della finestra con `title`, il frame rate con `fps`, il pulsante per uscire dall'applicazione con `quit_key`, la scala del display con `display_scale`, la scala della cattura dello schermo con `capture_scale`, ed il tempo di registrazione massimo del video di cattura dello schermo con `capture_sec`.<br>
  e.g. `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Avvia l'applicazione Pyxel e chiama la funzione `update` per l'aggiornamento del frame e la funzione `draw` per disegnare.

- `show()`<br>
  Mostra lo schermo e attende fino a quando il pulsante `Esc` non viene premuto.

- `flip()`<br>
  Riavvolge lo schermo di un fotogramma. L'applicazione esce quando viene premuto il tasto `Esc`. Questa funzione non funziona nella versione web.

- `quit()`<br>
  Esci dall'applicazione Pyxel.

### Risorse

- `load(filename, [excl_images], [excl_tilemaps], [excl_sounds], [excl_musics])`<br>
  Carica il file risorsa (.pyxres). Se un'opzione è `True`, la risorsa non verrà caricata. Se esiste un file di tavolozza (.pyxpal) con lo stesso nome nella stessa posizione del file di risorsa, anche il colore di visualizzazione della tavolozza verrà modificato. Il file della tavolozza è una voce esadecimale dei colori di visualizzazione (ad esempio, `1100FF`), separata da newline. Il file della tavolozza può essere usato anche per cambiare i colori visualizzati nell'Editor Pyxel.

### Input

- `mouse_x`, `mouse_y`<br>
  La posizione corrente del cursore del mouse

- `mouse_wheel`<br>
  Il valore corrente della rotella del mouse

- `btn(key)`<br>
  Ritorna `True` se `key` è premuto, altrimenti ritorna `False`. ([lista definizione tasti](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  Ritorna `True` se `key` è premuto quel frame, altrimenti ritorna `False`. Quando `hold` e `repeat` sono specificati, `True` sarà ritornato all'intervallo frame `repeat` quando `key` è premuto per più di `hold` frame.

- `btnr(key)`<br>
  Ritorna `True` se `key` è rilasciato quel frame, altrimenti ritorna `False`

- `mouse(visible)`<br>
  Se `visible` è `True`, mostra il cursore mouse. Se `False`, nascondilo. Anche se il cursore mouse non è mostrato, la sua posizione è aggiornata.

### Grafica

- `colors`<br>
  Lista della palette colori del display. Il colore del display è specificato tramite un valore numerico a 24-bit. Usare `colors.from_list` e `colors.to_list` per assegnare direttamente e recuperare le liste Python.<br>
  e.g. `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Elenco dei banchi immagine (0-2)<br>
  e.g. `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Elenco delle tilemaps (0-7)

- `clip(x, y, w, h)`<br>
  Imposta l'area di disegno dello schermo da (`x`, `y`) a lunghezza `w` e altezza `h`. Resettare l'area di disegno a schermo intero con `clip()`

- `camera(x, y)`<br>
  Cambia le coordinate dell'angolo superiore sinistro dello schermo in (`x`, `y`). Resetta le coordinate dell'angolo superiore sinistro a (`0`, `0`) con `camera()`.

- `pal(col1, col2)`<br>
  Rimpiazza colore `col1` con `col2` al momento di disegno. `pal()` per tornare alla palette iniziale.

- `dither(alpha)`<br>
  Applica il dithering (pseudo-trasparenza) al disegno. Impostare `alpha` nell'intervallo `0,0`-`1,0`, dove `0,0` è trasparente e `1,0` è opaco.

- `cls(col)`<br>
  Riempie lo schermo con `col`

- `pget(x, y)`<br>
  Ritorna il colore del pixel su (`x`, `y`).

- `pset(x, y, col)`<br>
  Disegna un pixel di colore `col` su (`x`, `y`).

- `line(x1, y1, x2, y2, col)`<br>
  Disegna una linea di colore `col` da (`x1`, `y1`) a (`x2`, `y2`).

- `rect(x, y, w, h, col)`<br>
  Disegna un rettangolo con lunghezza `w`, altezza `h` e colore `col` da (`x`, `y`).

- `rectb(x, y, w, h, col)`<br>
  Disegna il contorno di un rettangolo di lunghezza `w`, altezza `h` e colore `col` da (`x`, `y`).

- `circ(x, y, r, col)`<br>
  Disegna un cerchio di raggio `r` e colore `col` su (`x`, `y`).

- `circb(x, y, r, col)`<br>
  Disegna il contorno di un cerchio di raggio `r` e colore `col` su (`x`, `y`).

- `elli(x, y, w, h, col)`<br>
  Disegna un'ellisse di larghezza `w`, altezza `h` e colore `col` da (`x`, `y`).

- `ellib(x, y, w, h, col)`<br>
  Disegna il contorno di un'ellisse di larghezza `w`, altezza `h` e colore `col` da (`x`, `y`).

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  Disegna un triangolo con vertici (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e colore `col`

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  Disegna il contorno di un triangolo con vertici (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e colore `col`

- `fill(x, y, col)`<br>
  Disegna un'ellisse di larghezza `w`, altezza `h` e colore `col` da (`x`, `y`).

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Copia la regione di grandezza (`w`, `h`) da (`u`, `v`) della banca immagini `img`(0-2) a (`x`, `y`). Se un valore negativo è impostato per `w` e/o `h`, sarà invertito orizzontalmente o verticalmente. Se `colkey` è specificato, verrà trattato come colore trasparente. Se viene specificato `rotate`(in gradi), `scale`(1.0=100%) o entrambi, verrà applicata la trasformazione corrispondente.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Copia la regione di dimensione (`w`, `h`) da (`u`, `v`) della tilemap `tm`(0-7) a (`x`, `y`). Se un valore negativo è stato impostato per `w` e/o `h`, sarà rovesciata orizzontalmente e/o verticalmente. Se `colkey` è specificato, viene trattato come colore trasparente. Se viene specificato `rotate`(in gradi), `scale`(1.0=100%) o entrambi, verrà applicata la trasformazione corrispondente. La dimensione di una tile tile è di 8x8 pixel ed è memorizzata in una tilemap come una tupla di `(tile_x, tile_y)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  Disegna una stringa `s` di colore `col` su (`x`, `y`).

### Audio

- `sounds`<br>
  Elenco dei suoni (0-63)<br>
  per esempio: `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Elenco delle musiche (0-7)

- `play(ch, snd, [tick], [loop], [resume])`<br>
  Riproduce il suono `snd`(0-63) sul canale `ch`(0-3). Se `snd` è una lista, verrà riprodotta in ordine. La posizione di inizio della riproduzione può essere specificata da `tick`(1 tick = 1/120 secondi). Se `True` è specificato per `loop`, viene eseguita la riproduzione in loop. Per riprendere il suono precedente dopo la fine della riproduzione, impostare `resume` su `True`.

- `playm(msc, [tick], [loop])`<br>
  Riproduce la musica `msc`(0-7). La posizione di inizio della riproduzione può essere specificata da `tick`(1 tick = 1/120 secondi). Se `True` è specificato per `loop`, viene eseguita la riproduzione in loop.

- `stop([ch])`<br>
  Interrompe la riproduzione del canale `ch`(0-3) specificato. `stop()` per interrompere tutti i canali.

- `play_pos(ch)`<br>
  Ottiene la posizione del suono in riproduzione del canale `ch`(0-3) come una tupla di `(sound_no, note_no)`. Ritorna `None` quando la riproduzione è interrotta.

### Matematica

- `ceil(x)`<br>
  Restituisce il più piccolo intero maggiore o uguale a `x`.

- `floor(x)`<br>
  Restituisce il più grande intero minore o uguale a `x`.

- `sgn(x)`<br>
  Restituisce `1` quando `x` è positivo, `0` quando è `0` e `-1` quando è negativo.

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
  Restituisce un decimale casuale maggiore o uguale a `a` e minore o uguale a `b`.

- `nseed(seed)`<br>
  Imposta il seme del rumore di Perlin.

- `noise(x, [y], [z])`<br>
  Restituisce il valore del rumore di Perlin per le coordinate specificate.

### Classe Image

- `width`, `height`<br>
  La lunghezza e l'altezza dell'immagine

- `set(x, y, data)`<br>
  Imposta l'immagine a (`x`, `y`) tramite una lista di stringhe.<br>
  e.g. `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Carica il file immagine (PNG/GIF/JPEG) in (`x`, `y`).

- `pget(x, y)`<br>
  Ritorna il colore del pixel su (`x`, `y`).

- `pset(x, y, col)`<br>
  Disegna un pixel di colore `col` su (`x`, `y`).

### Classe Tilemap

- `width`, `height`<br>
  Lunghezza e altezza della tilemap

- `imgsrc`<br>
  Il banco immagine (0-2) referenziato dalla tilemap

- `set(x, y, data)`<br>
  Imposta la tilemap a (`x`, `y`) mediante una lista di stringhe.<br>
  e.g. `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  Caricare il layer nell'ordine di disegno `layer`(0-) dal file TMX (Tiled Map File) a (`x`, `y`).

- `pget(x, y)`<br>
  Prende la tile in (`x`, `y`). Una tile è una tupla di `(tile_x, tile_y)`.

- `pset(x, y, tile)`<br>
  Disegna una `tile` in (`x`, `y`). Una tile è una tupla di `(tile_x, tile_y)`.

### Classe Sound

- `notes`<br>
  Lista di note (0-127). Più alto il numero, più alto il tono, e a `33` diventa 'A2'(440Hz). Il resto è -1.

- `tones`<br>
  Lista di tonalità (0:Triangolo / 1:Quadrato / 2:Ritmo / 3:Rumore)

- `volumes`<br>
  Lista di volumi (0-7)

- `effects`<br>
  Lista di effetti (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  Velocità di riproduzione. `1` è il più veloce, e più grande è il numero, più è lenta la velocità di riproduzione. A `120`, la lunghezza di una nota diventa 1 secondo.

- `set(notes, tones, volumes, effects, speed)`<br>
  Imposta note, tonalità, volumi, ed effetti con una stringa. Se il numero di tonalità, volumi, ed effetti è inferiore alle note, vengono ripetuti dall'inizio.

- `set_notes(notes)`<br>
  Imposta le note con una stringa composta da 'CDEFGAB'+'#-'+'01234' o 'R'. Case-insensitive e gli spazi bianchi sono ignorati.<br>
  e.g. `pyxel.sounds[0].set_notes("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
  Imposta le tonalità con una stringa formata da 'TSPN'. Case-insensitive e gli spazi bianchi sono ignorati.<br>
  e.g. `pyxel.sounds[0].set_tones("TTSS PPPN")`

- `set_volumes(volumes)`<br>
  Imposta i volumi con una stringa formata da '01234567'. Case-insensitive e gli spazi bianchi sono ignorati.<br>
  e.g. `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  Imposta gli effetti con una stringa formata da 'NSVFHQ'. Case-insensitive e gli spazi bianchi sono ignorati.<br>
  e.g. `pyxel.sounds[0].set_effects("NFNF NVVS")`

### Classe Music

- `seqs`<br>
  Lista bidimensionale di suoni (0-63) con il numero di canali

- `set(seq0, seq1, seq2, ...)`<br>
  Impostare gli elenchi di suoni (0-63) dei canali. Se è indicata una lista vuota, quel canale non viene utilizzato per la riproduzione.<br>
  e.g. `pyxel.musics[0].set([0, 1], [], [3])`

### API avanzata

Pyxel ha "API avanzate" che non sono menzionate in questa documentazione perchè "potrebbero confondere gli utenti" oppure "necessitano di conoscenze specifiche per poter essere utilizzate".

Se ti senti confidente sulle tue competenze, prova a creare lavori incredibili con [this](../python/pyxel/__init__.pyi) come idea!

## Come contribuire

### Segnalare problemi

Usa l'[Issue Tracker](https://github.com/kitao/pyxel/issues) per inviare segnalazioni su bug e richieste di funzionalità/migliorie. Prima di inviare una nuova issue, assicurati che non ci sia una issue simile aperta.

### Test funzionale

Chiunque è il benvenuto per testare manualmente il codice e riportare bug o suggerimenti per miglioramenti nell'[Issue Tracker](https://github.com/kitao/pyxel/issues)!

### Inviare Pull Requests

Patch/fix sono accettati in forma di pull request (PR). Assicurarsi che il problema per cui si emetta una pull request sia aperto nel tracciante di problemi.

La pull request inviata è considerata accettata per la pubblicazione sotto la [Licenza MIT](../LICENSE).

## Altre informazioni

- [FAQ](faq-en.md)
- [Esempi di utenti](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Account dello sviluppatore X](https://x.com/kitao)

## Licenza

Pyxel è sotto [Licenza MIT](../LICENSE). Può essere riutilizzato all'interno di software proprietari, a condizione che tutte le copie del software o delle sue parti sostanziali includano una copia dei termini della Licenza MIT e anche un avviso di copyright.

## Reclutare sponsor

Pyxel è alla ricerca di sponsor su GitHub Sponsor. Prendi in considerazione la sponsorizzazione di Pyxel per la manutenzione continua e l'aggiunta di funzionalità. Gli sponsor possono consultare Pyxel come vantaggio. Si prega di vedere [qui](https://github.com/sponsors/kitao) per i dettagli.
