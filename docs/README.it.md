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

### Eseguire esempi

Dopo aver installato Pyxel, puoi copiare gli esempi nella directory corrente con il seguente comando:

```sh
pyxel copy_examples
```

Gli esempi possono essere visualizzati ed eseguiti nel browser da [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/).

In ambiente locale, gli esempi possono essere eseguiti con i seguenti comandi:

```sh
# Eseguire l'esempio nella directory examples
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# Eseguire l'app nella directory examples/apps
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

## Come usare

### Creare un'applicazione

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

Interrompi il monitoraggio della directory premendo `Ctrl(Command)+C`.

### Operazioni dei tasti speciali

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

### Come creare risorse

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

### Altri metodi di creazione delle risorse

Le immagini e le mappe a tessere di Pyxel possono anche essere create utilizzando i seguenti metodi:

- Crea immagini o mappe a tessere da elenchi di stringhe con le funzioni `Image.set` o `Tilemap.set`
- Carica file immagine compatibili con la palette Pyxel (PNG/GIF/JPEG) con la funzione `Image.load`

I suoni e la musica di Pyxel possono anche essere creati utilizzando il seguente metodo:

- Creali da stringhe con le funzioni `Sound.set` o `Music.set`

Fai riferimento alla documentazione dell'API per l'uso di queste funzioni.

### Come distribuire le applicazioni

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

- `reset()`<br>
  Reimposta l'applicazione Pyxel. Le variabili d'ambiente vengono mantenute dopo il reset.

### Risorsa

- `load(filename, [exclude_images], [exclude_tilemaps], [exclude_sounds], [exclude_musics])`<br>
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
  Elenco dei colori della palette. Il colore di visualizzazione è specificato da un valore numerico a 24 bit. Può essere manipolato come una lista Python per aggiungere, rimuovere o sostituire i colori in blocco.<br>
  Esempio: `old_colors = list(pyxel.colors); pyxel.colors[:] = [0x111111, 0x222222, 0x333333]; pyxel.colors[15] = 0x112233`

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
  Riempie l'area connessa con lo stesso colore di (`x`, `y`) con il colore `col`.

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
  Elenco delle tracce musicali (istanze della classe Music) (0-7)

- `play(ch, snd, [sec], [loop], [resume])`<br>
  Riproduce il suono `snd`(0-63) sul canale `ch`(0-3). `snd` può essere un numero di suono, un elenco di numeri di suono o una stringa MML. La posizione di partenza della riproduzione può essere specificata in secondi con `sec`. Se `loop` è impostato su `True`, la riproduzione verrà eseguita in loop. Per riprendere il suono precedente dopo la fine della riproduzione, impostare `resume` su `True`.

- `playm(msc, [sec], [loop])`<br>
  Riproduce la musica `msc`(0-7). La posizione di partenza della riproduzione può essere specificata in secondi con `sec`. Se `loop` è impostato su `True`, la riproduzione verrà eseguita in loop.

- `stop([ch])`<br>
  Interrompe la riproduzione del canale specificato `ch`(0-3). Chiama `stop()` per interrompere tutti i canali.

- `play_pos(ch)`<br>
  Ottiene la posizione di riproduzione del suono sul canale `ch`(0-3) sotto forma di tupla `(sound_no, sec)`. Restituisce `None` quando la riproduzione è terminata.

- `gen_bgm(preset, instr, [seed], [play])`<br>
  Genera una lista MML di BGM con un algoritmo basato su [8bit BGM generator](https://github.com/shiromofufactory/8bit-bgm-generator). `preset` è il numero del preset (0-7), `instr` è il numero di strumentazione (0-3): `0`=melodia+riverbero+basso, `1`=melodia+basso+batteria, `2`=melodia+sub+basso, `3`=melodia+sub+basso+batteria. Se `seed` non è specificato, il risultato è casuale. Se `play` è `True`, viene riprodotto l’MML generato.

### Matematica

- `ceil(x)`<br>
  Restituisce il numero intero più piccolo che è maggiore o uguale a `x`.

- `floor(x)`<br>
  Restituisce il numero intero più grande che è minore o uguale a `x`.

- `clamp(x, lower, upper)`<br>
  Restituisce `x` limitato tra `lower` come valore minimo e `upper` come valore massimo.

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
  Esempio: `pyxel.tilemaps[0].set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  Carica il `layer`(0-) dal file TMX (Tiled Map File) a (`x`, `y`).

- `pget(x, y)`<br>
  Ottiene la tessera a (`x`, `y`). Una tessera è rappresentata come una tupla `(image_tx, image_ty)`.

- `pset(x, y, tile)`<br>
  Disegna una `tessera` a (`x`, `y`). Una tessera è rappresentata come una tupla `(image_tx, image_ty)`.

- `collide(x, y, w, h, dx, dy, walls)`<br>
  Risolve le collisioni dopo aver applicato il movimento (`dx`, `dy`) al rettangolo nella posizione (`x`, `y`) con dimensioni (`w`, `h`), e restituisce i (`dx`, `dy`) corretti. `walls` è una lista di tessere `(image_tx, image_ty)` trattate come muri.

### Classe Sound

- `notes`<br>
  Elenco di note (0-127). Più alto è il numero, più acuto è il suono. La nota `33` corrisponde a 'A2' (440 Hz). Le note di silenzio sono rappresentate da `-1`.

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

- `mml(code)`<br>
  Passando una stringa [MML (Music Macro Language)](https://en.wikipedia.org/wiki/Music_Macro_Language), si passa alla modalità MML e il suono viene riprodotto secondo il suo contenuto. In questa modalità, i parametri normali come `notes` e `speed` vengono ignorati. Per uscire dalla modalità MML, chiama `mml()` senza argomenti. Per maggiori dettagli su MML, consulta [questa pagina](faq-en.md).<br>
  Esempio: `pyxel.sounds[0].mml("T120 Q90 @1 V100 O5 L8 C4&C<G16R16>C.<G16 >C.D16 @VIB1{10,20,20} E2C2")`

- `pcm(filename)`<br>
  Carica un file audio (WAV/OGG) per la riproduzione. Chiama `pcm()` per tornare alla modalità di riproduzione normale.<br>
  Esempio: `pyxel.sounds[0].pcm("sounds/bgm.ogg")`

- `save(filename, sec, [ffmpeg])`<br>
  Crea un file WAV che riproduce il suono per i secondi specificati. Se FFmpeg è installato e `ffmpeg` è impostato su `True`, viene creato anche un file MP4.

- `total_sec()`<br>
  Restituisce la durata di riproduzione del suono in secondi. Restituisce `None` se in MML viene utilizzato un loop infinito.

### Classe Music

- `seqs`<br>
  Un elenco bidimensionale di suoni (0-63) su più canali

- `set(seq0, seq1, seq2, ...)`<br>
  Imposta gli elenchi di suoni (0-63) per ciascun canale. Se viene specificato un elenco vuoto, quel canale non verrà utilizzato per la riproduzione.<br>
  Esempio: `pyxel.musics[0].set([0, 1], [], [3])`

- `save(filename, sec, [ffmpeg])`<br>
  Crea un file WAV che riproduce la musica per i secondi specificati. Se FFmpeg è installato e `ffmpeg` è impostato su `True`, viene creato anche un file MP4.

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

## Strumenti Web e Esempi

- [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)
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
