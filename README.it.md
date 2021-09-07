# <img src="images/pyxel_logo_152x64.png">

[ [English](README.md) | [中文](README.cn.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Deutsch](README.de.md)]

**Pyxel** è un game engine rétro per Python.

Grazie alle sue specifiche limitate ispirate dalle console di videogiochi rétro, come al fatto che solo 16 colori possono essere mostrati e solo 4 suoni possono essere suonati allo stesso tempo, puoi sentirti libero di creare giochi stile pixel art.

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

Le specifiche della console di gioco e API per Pyxel si riferiscono ai mozzafiato [PICO-8](https://www.lexaloffle.com/pico-8.php) e [TIC-80](https://tic.computer/).

Pyxel è open source e libero da usare. Cominciamo a fare giochi rétro con Pyxel!

## Specifiche

- Funziona su Windows, Mac, e Linux
- Codice si scrive con Python3
- Palette di 16 colori fissi
- 3 banche di immagini di dimensioni 256x256
- 8 tilemap di dimensioni 256x256
- 4 canali con 64 suoni definibili
- 8 musiche che possono combinare suoni arbitrari
- Input di tastiera, mouse, e controller
- Editor suoni e immagini

### Palette colori

<img src="pyxel/examples/screenshots/05_color_palette.png">
<br><br>
<img src="images/pyxel_palette.png">

## Come installare

### Windows

Prima di tutto, installa [Python3](https://www.python.org/) (versione 3.6.8 o maggiore).

Una volta che hai installato Python con l'installer ufficiale, **aggiungi Python alla PATH** selezionando il tasto mostrato qui:

<img src="images/python_installer.png">

Dopo, installa Pyxel con il comando `pip` seguente dalla linea di comando:

```sh
pip install -U pyxel
```

### Mac

Prima di tutto, nell'ambiente dove è installato [Homebrew](https://brew.sh/) installa [Python3](https://www.python.org/) (versione 3.6.8 o maggiore) e i pacchetti necessari con il comando seguente:

```sh
brew install python3 gcc sdl2 sdl2_image gifsicle
```

Si può installare Python3 in altri modi, ma tieni conto che bisognerebbe installare altre librerie.

Dopo, **riavvia il terminale** e installa Pyxel con il comando `pip3`:

```sh
pip3 install -U pyxel
```

### Linux

Installa [Python3](https://www.python.org/) (versione 3.6.8 o maggiore) e i pacchetti necessari nel modo appropriato per ogni distribuzione.

**Ubuntu:**

```sh
sudo apt install python3 python3-pip libsdl2-dev libsdl2-image-dev gifsicle
sudo -H pip3 install -U pyxel
```

### Altri ambienti

Per installare Pyxel in un ambiente diverso da quelli precedenti (32-bit Linux, Raspberry PI, ecc.), segui i procedimenti seguenti per compilare:

#### Installa strumenti e pacchetti necessari

- Toolchain compilazione C++ (dovrebbe includere i comandi gcc e make)
- libsdl2-dev e libsdl2-image-dev
- [Python3](https://www.python.org/) (versione 3.6.8 o maggiore) e comando pip

#### Esegui il comando seguente in qualsiasi cartella

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make -C pyxel/core clean all
pip3 install .
```

### Installa esempi

Dopo aver installato Pyxel, gli esempi di Pyxel saranno copiati nella corrente cartella con il comando seguente:

```sh
install_pyxel_examples
```

Gli esempi da copiare sono i seguenti:

- [01_hello_pyxel.py](pyxel/examples/01_hello_pyxel.py) - Applicazione più semplice
- [02_jump_game.py](pyxel/examples/02_jump_game.py) - Un gioco di salto con file Pyxel di risorsa
- [03_draw_api.py](pyxel/examples/03_draw_api.py) - Dimostrazione dell'API di disegno
- [04_sound_api.py](pyxel/examples/04_sound_api.py) - Dimostrazione dell'API di suono
- [05_color_palette.py](pyxel/examples/05_color_palette.py) - Lista di colori nella palette
- [06_click_game.py](pyxel/examples/06_click_game.py) - Gioco punta e clicca
- [07_snake.py](pyxel/examples/07_snake.py) - Gioco snake con colonna sonora
- [08_triangle_api.py](pyxel/examples/08_triangle_api.py) - Dimostrazione dell'API di disegno triangoli
- [09_shooter.py](pyxel/examples/09_shooter.py) - Gioco shoot'em up con transizioni schermo

Gli esempi possono essere eseguiti come qualsiasi file python:

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

## Come usare

### Creare una applicazione Pyxel

Dopo aver importato il modulo Pyxel nel tuo codice python, Specifica la dimensione della finestra con la funzione `init`, poi avvia l'applicazione Pyxel con la funzione `run`.

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

I parametri della funzione `run` sono passate alla funzione `update` per aggiornare ogni frame e alla funzione `draw` per disegnare lo schermo quando necessario.

In una effettiva applicazione, è consigliato ricoprire codice pyxel in una classe come qui sotto:

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

Si può anche scrivere codice semplice usando le funzioni `show` e `flip` per disegnare grafiche e animazioni semplici.

La funzione `show` mostra lo schermo e aspetta fino a che il tasto `ESC` non sia premuto.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

La funzione `flip` aggiorna lo schermo una volta.

```python
import pyxel

pyxel.init(120, 80)

while True:
    pyxel.cls(3)
    pyxel.rectb(pyxel.frame_count % 160 - 40, 20, 40, 40, 7)
    pyxel.flip()
```

### Controlli speciali

I controlli seguenti speciali possono essere eseguite mentre viene eseguita un'applicazione Pyxel:

- `Esc`<br>
Esci dall'applicazione
- `Alt(Option)+1`<br>
Salva uno screenshot sul desktop
- `Alt(Option)+2`<br>
Resetta il tempo d'inizio della registrazione schermo
- `Alt(Option)+3`<br>
Salva la registrazione schermo (gif) sul desktop (fino a 30 secondi)
- `Alt(Option)+0`<br>
Alterna il monitor di performance (fps, tempo d'aggiornamento, e tempo di disegno)
- `Alt(Option)+Enter`<br>
Alterna schermo intero

### Come creare una risorsa

Il Pyxel editor incluso può creare suoni ed immagini usate in un'applicazione Pyxel.

L'editor Pyxel è avviato con il comando seguente:

```sh
pyxeleditor [pyxel_resource_file]
```

Se il file di risorsa Pyxel (.pyxres) specificato esiste, allora il file viene caricato, e se non esiste, un nuovo file con quel nome viene creato.
Se il file risorsa viene omesso, il nome è `my_resource.pyxres`.

Dopo aver avviato l'editor Pyxel, il file può essere cambiato trascinando un'altro file risorsa. Se il file risorsa viene trascinato quando il tasto ``Ctrl``(``Cmd``) è premuto, solo il tipo di risorsa corrente (immagine/tilemap/suono/musica) che sta venendo modificata sarà caricato. Questa operazione permette di combinare multipli file risorsa in uno.

Il file risorsa creato può essere caricato con la funzione `load`.

L'editor Pyxel ha le seguenti modalità di modifica.

**Editor Immagini:**

La modalità per modificare banche d'immagini.

<img src="pyxel/editor/screenshots/image_editor.gif">

Trascinando un file png sullo schermo dell'editor immagini, l'immagine può essere caricata nella banca immagini selezionata.

**Editor Tilemap:**

La modalità per modificare tilemap immagini delle banche immagini sono posizionate in un modo a piastrelle.

<img src="pyxel/editor/screenshots/tilemap_editor.gif">

**Editor Suoni:**

Modalità per modificare suoni.

<img src="pyxel/editor/screenshots/sound_editor.gif">

**Editor Musica:**

La modalità per modificare musica in cui i suoni sono posizionati in ordine per poi essere risuonati.

<img src="pyxel/editor/screenshots/music_editor.gif">

### Altri metodi per creare risorse

Immagini e tilemap Pyxel possono anche essere creati nei modi seguenti:

- Crea un'immagine da una lista di stringhe con le funzioni `Image.set` o `Tilemap.set`
- Carica un file png nella palette Pyxel con la funzione `Image.load`

Suoni Pyxel possono anche essere creati nel modo seguente:

- Creare un suono da una stringa con le funzioni `Sound.set` o `Music.set`

Riferirsi al manuale dell'API per l'uso di queste funzioni.

### Come creare un eseguibile stand-alone

Usando il Pyxel Packager incluso, un eseguibile stand-alone che funzionerà anche in ambienti dove python non è installato può essere creato.

Per creare un eseguibile stand-alone, nell'ambiente dove è installato [PyInstaller](https://www.pyinstaller.org/) , specificare il file Python da essere usato per avviare l'applicazione con il comando `pyxelpackager` come segue:

```sh
pyxelpackager python_file
```

Quando il procedimento è completo, un eseguibile stand-alone è creato nella cartella `dist`

Se le risorce come file .pyxres e .png sono necessari, metterli nella cartella `assets` e saranno inclusi.

Si può anche specificare un'icona con l'opzione ``-i icon_file``.

## Manuale API

### Sistema

- `width`, `height`<br>
Lunghezza e altezza dello schermo

- `frame_count`<br>
Numero di frame passati

- `init(width, height, [caption], [scale], [palette], [fps], [quit_key], [fullscreen])`<br>
Inizializza l'applicazione Pyxel con la grandezza schermo (`width`, `height`). La grandezza massima dello schermo è 256<br>
Si può anche specificare il titolo della finestra con `caption`, la magnificazione del display con `scale`, il colore di palette con `palette`, il framerate con `fps`, la chiave per uscire dall'applicazione con `quit_key`, e se iniziare l'applicazione a schermo intero con `fullscreen`. `palette` è definita come una lista di 16 elementi di colori a 24 bit.<br>
per esempio: `pyxel.init(160, 120, caption="Pyxel with PICO-8 palette", palette=[0x000000, 0x1D2B53, 0x7E2553, 0x008751, 0xAB5236, 0x5F574F, 0xC2C3C7, 0xFFF1E8, 0xFF004D, 0xFFA300, 0xFFEC27, 0x00E436, 0x29ADFF, 0x83769C, 0xFF77A8, 0xFFCCAA], quit_key=pyxel.KEY_NONE, fullscreen=True)`

- `run(update, draw)`<br>
Inizia l'applicazione Pyxel e chiama la funzione `update` per aggiornare il frame e la funzione `draw` per disegnare

- `quit()`<br>
Uscire dall'applicazione Pyxel alla fine del frame corrente

- `flip()`<br>
Forza il disegno dello schermo (non usare in applicazioni normali)

- `show()`<br>
Disegna lo schermo e aspetta per sempre (non usare in applicazioni normali)

### Risorse

- `save(filename)`<br>
Salva il file risorsa (.pyxres) nella cartella d'esecuzione dello script

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Leggi il file risorsa (.pyxres) dalla cartella d'esecuzione dello script. Se ``False`` è specificato per il tipo di risorsa (immagine/tilemap/suono/musica), la risorsa non sarà caricata.

### Input
- `mouse_x`, `mouse_y`<br>
La posizione corrente del cursore del mouse

- `mouse_wheel`<br>
Il valore corrente della rotella del mouse

- `btn(key)`<br>
Ritorna `True` se `key` è premuto, altrimenti ritorna `False` ([lista definizione tasti](pyxel/__init__.py))

- `btnp(key, [hold], [period])`<br>
Ritorna `True` se `key` è premuto quel frame, altrimenti ritorna `False`. Quando `hold` e `period` sono specificati, `True` sarà ritornato all'intervallo frame `period` quando `key` è premuto per più di `hold` frame

- `btnr(key)`<br>
Ritorna `True` se `key` è rilasciato quel frame, altrimenti ritorna `False`

- `mouse(visible)`<br>
Se `visible` è `True`, mostra il cursore mouse. Se `False`, nascondilo. Anche se il cursore mouse non è mostrato, la sua posizione è aggiornata.

### Grafica

- `image(img, [system])`<br>
Opera la banca immagini `img`(0-2) (vedere la classe Image). Se `system` è `True`, la banca immagine per il sistema può essere acquisito. 3 è per il font e l'editor risorse. 4 è per lo schermo del display<br>
per esempio: `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
Opera la tilemap `tm`(0-7) (vedere la classe Tilemap)

- `clip(x, y, w, h)`<br>
Imposta l'area di disegno dello schermo da (`x`, `y`) a lunghezza `w` e altezza `h`. Resettare l'area di disegno a schermo intero con `clip()`

- `pal(col1, col2)`<br>
Rimpiazza colore `col1` con `col2` al momento di disegno. `pal()` per tornare alla palette iniziale

- `cls(col)`<br>
Riempie lo schermo con `col`

- `pget(x, y)`<br>
Ritorna il colore del pixel su (`x`, `y`)

- `pset(x, y, col)`<br>
Disegna un pixel di colore `col` su (`x`, `y`)

- `line(x1, y1, x2, y2, col)`<br>
Disegna una linea di colore `col` da (`x1`, `y1`) a (`x2`, `y2`)

- `rect(x, y, w, h, col)`<br>
Disegna un rettangolo con lunghezza `w`, altezza `h` e colore `col` da (`x`, `y`)

- `rectb(x, y, w, h, col)`<br>
Disegna il contorno di un rettangolo di lunghezza `w`, altezza `h` e colore `col` da (`x`, `y`)

- `circ(x, y, r, col)`<br>
Disegna un cerchio di raggio `r` e colore `col` su (`x`, `y`)

- `circb(x, y, r, col)`<br>
Disegna il contorno di un cerchio di raggio `r` e colore `col` su (`x`, `y`)

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
Disegna un triangolo con vertici (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e colore `col`

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
Disegna il contorno di un triangolo con vertici (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e colore `col`

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
Copia la regione di grandezza (`w`, `h`) da (`u`, `v`) della banca immagini `img`(0-2) a (`x`, `y`). Se un valore negativo è impostato per `w` e/o `h`, sarà invertito orizzontalmente o verticalmente. Se `colkey` è specificato, verrà trattato come colore trasparente

<img src="images/image_bank_mechanism.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Disegna la tilemap `tm`(0-7) su (`x`, `y`) seguendo l'informazione delle piastrelle di grandezza (`w`, `h`) da (`u`, `v`). Se `colkey` è specificato, verrà trattato come colore trasparente. Una piastrella della tilemap è disegnata con una grandezza di 8x8, e se il numero di piastrella è 0, indica la regione (0, 0)-(7, 7) della banca immagini, se 1, indica (8, 0)-(15, 0)

<img src="images/tilemap_mechanism.png">

- `text(x, y, s, col)`<br>
Disegna una stringa `s` di colore `col` su (`x`, `y`)

### Audio

- `sound(snd, [system])`<br>
Opera il suono `snd`(0-63) (Vedere classe Sound). Se `system` è `True`, the sound 64 for system can be accessed<br>
per esempio: `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Opera la musica `msc`(0-7) (vedere la classe Music)

- `play_pos(ch)`<br>
Dà la posizione del playback sonoro di canale `ch`. I 100 e 1000 indicano il numero di suono e gli 1 e 10 indicano il numero della nota. Quando il playback è finito, ritorna `-1`

- `play(ch, snd, loop=False)`<br>
Suona `snd`(0-63) sul canale `ch`(0-3). Suona in ordine quando `snd` è una lista

- `playm(msc, loop=False)`<br>
Suona la musica `msc`(0-7)

- `stop([ch])`<br>
Ferma playback di tutti i canali. Se `ch`(0-3) è specificato, ferma solo il canale corrispondente

### Image Class

- `width`, `height`<br>
La lunghezza e l'altezza dell'immagine

- `data`<br>
I dati dell'immagine (lista bidimensionale da 256x256)

- `get(x, y)`<br>
Trova i dati dell'immagine su (`x`, `y`)

- `set(x, y, data)`<br>
Imposta i dati dell'immagine su (`x`, `y`) da un valore o una lista di stringhe<br>
per esempio: `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Leggi l'immagine png dalla cartella d'esecuzione dello script su (`x`, `y`)

- `copy(x, y, img, u, v, w, h)`<br>
Copia la regione di grandezza (`w`, `h`) da (`u`, `v`) della banca immagini `img`(0-2) a (`x`, `y`)

### Classe Tilemap

- `width`, `height`<br>
Lunghezza e altezza della tilemap

- `data`<br>
I dati della tilemap (lista bidimensionale da 256x256)

- `refimg`<br>
La banca immagini che la tilemap prende in riferimento

- `get(x, y)`<br>
Trova i dati della tilemap su (`x`, `y`)

- `set(x, y, data)`<br>
Imposta i dati della tilemap su (`x`, `y`) da un valore o una lista di stringhe.<br>
per esempio: `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `copy(x, y, tm, u, v, w, h)`<br>
Copia la regione di grandezza (`w`, `h`) da (`u`, `v`) della tilemap `tm`(0-7) a (`x`, `y`)

### Classe Sound

- `note`<br>
Lista di note(0-127) (33 = 'A2' = 440Hz)

- `tone`<br>
Lista di toni(0:Triangolo / 1:Quadrato / 2:Impulso / 3:Rumore)

- `volume`<br>
Lista di volume(0-7)

- `effect`<br>
Lista di effetti(0:Nessuno / 1:Scivolo / 2:Vibrato / 3:Sfumato in uscita)

- `speed`<br>
Lunghezza di una nota(120 = 1 second per tone)

- `set(note, tone, volume, effect, speed)`<br>
Imposta una nota, tono, volume, ed effetto con una stringa. Se la lunghezza del tono, volume o effetto è minore della lunghezza della nota, è ripetuto dall'inizio

- `set_note(note)`<br>
Imposta la nota con una stringa fatta da 'CDEFGAB'+'#-'+'0123' o 'R'. Non sensibile al maiuscolo e gli spazi sono ignorati<br>
per esempio: `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tone(tone)`<br>
Imposta il tono con una stringa fatta da 'TSPN'. Non sensibile al maiuscolo e gli spazi sono ignorati<br>
e.g. `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volume(volume)`<br>
Imposta il volume con una stringa fatta da '01234567'. Non sensibile al maiuscolo e gli spazi sono ignorati<br>
e.g. `pyxel.sound(0).set_volume("7777 7531")`

- `set_effect(effect)`<br>
Imposta l'effetto con una stringa fatta da 'NSVF'. Non sensibile al maiuscolo e gli spazi sono ignorati<br>
e.g. `pyxel.sound(0).set_effect("NFNF NVVS")`

### Music Class

- `ch0`<br>
Lista di sound(0-63) suonata su canale 0. Se è specificata una lista vuota, il canale non è usato per playback

- `ch1`<br>
Lista di sound(0-63) suonata su canale 1. Se è specificata una lista vuota, il canale non è usato per playback

- `ch2`<br>
Lista di sound(0-63) suonata su canale 2. Se è specificata una lista vuota, il canale non è usato per playback

- `ch3`<br>
Lista di sound(0-63) suonata su canale 3. Se è specificata una lista vuota, il canale non è usato per playback

- `set(ch0, ch1, ch2, ch3)`<br>
Imposta la lista di sound(0-63) di tutti i canali. Se è specificata una lista vuota, il canale non è usato per playback<br>
per esempio: `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

- `set_ch0(data)`<br>
Imposta la lista di sound(0-63) di canale 0

- `set_ch1(data)`<br>
Imposta la lista di sound(0-63) di canale 1

- `set_ch2(data)`<br>
Imposta la lista di sound(0-63) di canale 2

- `set_ch3(data)`<br>
Imposta la lista di sound(0-63) di canale 3

## Come contribuire

### Emettere un nuovo problema

Usare [il tracciante di problemi](https://github.com/kitao/pyxel/issues) per segnalare bug e richieste di miglioramenti/capacità nuove.
Prima di emettere un nuovo problema, cerca nel tracciante di problemi per assicurarti che non ci siano problemi simili già aperti.

Quando emettendo un problema, selezionare un template da [questo link](https://github.com/kitao/pyxel/issues/new/choose).

### Testare manualmente

Chiunque che testi il codice manualmente e segnali bug o consigli per miglioramenti nel tracciante di problemi sono benvenuti!

### Emettere una pull request

Patch/fix sono accettati in forma di pull request (PR). Assicurarsi che il problema per cui si emetta una pull request sia aperto nel tracciante di problemi.

Le pull request emesse sono presupposte di accettare di essere pubblicate sotto la [licenza MIT](LICENZA).

## Altre informazioni

- [Wiki](https://github.com/kitao/pyxel/wiki)
- [Subreddit](https://www.reddit.com/r/pyxel/)
- [Server Discord (Inglese)](https://discord.gg/FC7kUZJ)
- [Server Discord (Giapponese - 日本語版)](https://discord.gg/qHA5BCS)

## Licenza

Pyxel è sotto la [licenza MIT](http://en.wikipedia.org/wiki/MIT_License). Può essere riusato in software proprietario affinche tutte le copie del software includano una copia dei termini della licenza MIT e dei notice di copyright.

Pyxel usa il software seguente:

- [SDL2](https://www.libsdl.org/)
- [miniz-cpp](https://github.com/tfussell/miniz-cpp)
- [Gifsicle](https://www.lcdf.org/gifsicle/)
