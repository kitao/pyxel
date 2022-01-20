# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**Pyxel** è un game engine rétro per Python.

Grazie alle sue specifiche limitate ispirate dalle console di videogiochi rétro, come al fatto che solo 16 colori possono essere mostrati e solo 4 suoni possono essere riprodotti allo stesso tempo, puoi sentirti libero di creare giochi stile pixel art.

<a href="../pyxel/examples/01_hello_pyxel.py" target="_blank">
<img src="images/01_hello_pyxel.gif" width="48%">
</a>

<a href="../pyxel/examples/02_jump_game.py" target="_blank">
<img src="images/02_jump_game.gif" width="48%">
</a>

<a href="../pyxel/examples/03_draw_api.py" target="_blank">
<img src="images/03_draw_api.gif" width="48%">
</a>

<a href="../pyxel/examples/04_sound_api.py" target="_blank">
<img src="images/04_sound_api.gif" width="48%">
</a>

<a href="images/image_tilemap_editor.gif" target="_blank">
<img src="images/image_tilemap_editor.gif" width="48%">
</a>

<a href="images/sound_music_editor.gif" target="_blank">
<img src="images/sound_music_editor.gif" width="48%">
</a>

Le specifiche di Pyxel si rifanno alle eccezionali [PICO-8](https://www.lexaloffle.com/pico-8.php) e [TIC-80](https://tic.computer/).

Pyxel è open source e libero da usare. Cominciamo a fare giochi rétro con Pyxel!

## Specifiche

- Funziona su Windows, Mac, e Linux
- Programmazione con Python
- Palette a 16 colori
- 3 banche di immagini di dimensioni 256x256
- 8 tilemap di dimensioni 256x256
- 4 canali con 64 suoni definibili
- 8 musiche che possono combinare suoni arbitrari
- Input di tastiera, mouse, e controller
- Editor suoni e immagini

### Palette colori

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Come installare

Ci sono due tipi di Pyxel, una versione pacchettizzata ed una versione standalone.

### Installare la versione pacchettizzata

La versione pacchettizzata di Pyxel utilizza Pyxel come modulo di estensione Python.

Raccomandata per coloro che hanno familiarità con la gestione dei pacchetti Python utilizzando il comando `pip` o per chi vuole sviluppare applicazioni Python complete.

**Windows**

Dopo aver installato [Python3](https://www.python.org/) (versione 3.7 o superiore), eseguire il seguente comando:

```sh
pip install -U pyxel
```

**Mac**

Dopo aver installato [Python3](https://www.python.org/) (versione 3.7 o superiore), eseguire il seguente comando:

```sh
pip3 install -U pyxel
```

**Linux**

Dopo aver installato il pacchetto SDL2 (`libsdl2-dev` per Ubuntu), [Python3](https://www.python.org/) (versione 3.7 o superiore), e `python3-pip`, eseguire il seguente comando:

```sh
sudo pip3 install -U pyxel
```

Se quanto sopra non funziona, provare a buildare manualmente seguendo i passaggi seguenti dopo aver installato `cmake` e `rust`:

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make clean all
sudo pip3 install .
```

### Installare la versione standalone

La versione standalone di Pyxel usa Pyxel come strumento standalone che non dipende da Python.

Raccomandato per coloro che vogliono iniziare a programmare con facilità senza doversi preoccupare delle impostazioni di Python, o coloro che vogliono eseguire immediatamente giochi Pyxel.

**Windows**

Scaricare e lanciare la versione più recente dell'installer per Windows (`pyxel-[version]-windows-setup.exe`) dalla [pagina di download](https://github.com/kitao/pyxel/releases).

**Mac**

Dopo aver installato [Homebrew](https://brew.sh/), lanciare i seguenti comandi:

```sh
brew tap kitao/pyxel
brew install pyxel
```

**Linux**

Dopo aver installato il pacchetto SDL2 (`libsdl2-dev` per Ubuntu) e [Homebrew](https://brew.sh/), lanciare i seguenti comandi:

```sh
brew tap kitao/pyxel
brew install pyxel
```

Se quanto sopra non funziona, provare a buildare manualmente la versione pacchettizzata.

### Provare gli esempi di Pyxel

Dopo aver installato Pyxel, gli esempi di Pyxel saranno copiati nella corrente cartella con il comando seguente:

```sh
pyxel copy_examples
```

Gli esempi da copiare sono i seguenti:

- [01_hello_pyxel.py](../pyxel/examples/01_hello_pyxel.py) - Applicazione più semplice
- [02_jump_game.py](../pyxel/examples/02_jump_game.py) - Un gioco di salto con file Pyxel di risorsa
- [03_draw_api.py](../pyxel/examples/03_draw_api.py) - Dimostrazione delle API di disegno
- [04_sound_api.py](../pyxel/examples/04_sound_api.py) - Dimostrazione delle API del suono
- [05_color_palette.py](../pyxel/examples/05_color_palette.py) - Lista di colori nella palette
- [06_click_game.py](../pyxel/examples/06_click_game.py) - Gioco punta e clicca
- [07_snake.py](../pyxel/examples/07_snake.py) - Gioco snake con colonna sonora
- [08_triangle_api.py](../pyxel/examples/08_triangle_api.py) - Dimostrazione delle API per il disegno di un triangolo
- [09_shooter.py](../pyxel/examples/09_shooter.py) - Gioco shoot'em up con transizioni schermo
- [10_platformer.py](../pyxel/examples/10_platformer.py) - Gioco a piattaforme a scorrimento orizzontale con mappa
- [11_offscreen.py](../pyxel/examples/11_offscreen.py) - Renderizzazione fuori campo con la classe Image
- [30SecondsOfDaylight.pyxapp](images/30SecondsOfDaylight.gif) - Gioco vincitore del primo Pyxel Jam sviluppato da [Adam](https://twitter.com/helpcomputer0)
- [megaball.pyxapp](images/megaball.gif) - Gioco arcade di palla basato sulla fisica sviluppato da [Adam](https://twitter.com/helpcomputer0)

Un esempio può essere eseguito con i seguenti comandi:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30SecondsOfDaylight.pyxapp
```

## Come usare

### Creare una applicazione Pyxel

Dopo aver importato il modulo Pyxel nel tuo script Python, prima specifica la dimensione della finestra con la funzione `init`, dopodichè lancia l'applicazione Pyxel con la funzione `run`.

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

I parametri della funzione `run` sono passati alla funzione `update` per aggiornare ogni frame e alla funzione `draw` per disegnare lo schermo quando necessario.

In un'applicazione reale, è consigliato includere il codice Pyxel in una classe come qui sotto:

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

È anche possibile scrivere semplice codice utilizzando la funzione `show` e la funzione `flip` per disegnare grafica e animazioni basiche.

La funzione `show` mostra lo schermo e attende fino a quando non viene premuto il pulsante `Esc`.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

La funzione `flip` aggiorna lo schermo una volta sola.

```python
import pyxel

pyxel.init(120, 80)

while True:
    pyxel.cls(3)
    pyxel.rectb(pyxel.frame_count % 160 - 40, 20, 40, 40, 7)
    pyxel.flip()
```

### Eseguire applicazioni Pyxel

Lo script Python creato può essere eseguito con il seguente comando:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Per la versione pacchettizzata, può essere eseguito come un normale script Python:

```sh
cd pyxel_examples
python3 PYTHON_SCRIPT_FILE
```

(Per Windows, digita `python` anzichè `python3`)

### Controlli speciali

I controlli seguenti speciali possono essere eseguite mentre viene eseguita un'applicazione Pyxel:

- `Esc`<br>
Esci dall'applicazione
- `Alt(Option)+1`<br>
Salva uno screenshot sul desktop
- `Alt(Option)+2`<br>
Resetta il tempo d'inizio della registrazione schermo
- `Alt(Option)+3`<br>
Salva la registrazione schermo sul desktop (fino a 10 secondi)
- `Alt(Option)+0`<br>
Alterna il monitor di performance (fps, tempo d'aggiornamento, e tempo di disegno)
- `Alt(Option)+Enter`<br>
Alterna schermo intero

### Come creare una risorsa

L'Editor Pyxel può creare immagini e suoni utilizzati in un'applicazione Pyxel.

Si avvia con il seguente comando:

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

Se il file di risorsa Pyxel (.pyxres) specificato esiste, allora il file viene caricato, e se non esiste, un nuovo file con quel nome viene creato.
Se il file risorsa viene omesso, il nome è `my_resource.pyxres`.

Dopo aver avviato l'Editor Pyxel, il file può essere scambiato trascinando e rilasciando un altro file risorsa. Se il file risorsa è trascinato e rilasciato tenendo premuto il pulsante ``Ctrl(Cmd)``, sarà caricato solo il tipo di risorsa (Immagine/Tilemap/Suono/Musica) che si sta attualmente modificando. Questa operazione consente di combinare più file risorsa in uno.

Il file risorsa creato può essere caricato con la funzione `load`.

L'editor Pyxel ha le seguenti modalità di modifica.

**Editor Immagini:**

La modalità per modificare banche d'immagini.

<img src="images/image_editor.gif">

Trascinando e rilasciando un file immagine (png/gif/jpeg) sullo schermo dell'Editor Immagine, l'immagine può essere caricata all'interno del banco d'immagine attualmente selezionato.

**Editor Tilemap:**

La modalità per modificare tilemap immagini delle banche immagini sono posizionate in un modo a piastrelle.

<img src="images/tilemap_editor.gif">

**Editor Suoni:**

Modalità per modificare suoni.

<img src="images/sound_editor.gif">

**Editor Musica:**

La modalità per modificare musica in cui i suoni sono posizionati in ordine per poi essere risuonati.

<img src="images/music_editor.gif">

### Altri metodi per creare risorse

Le immagini e le tilemap Pyxel possono essere create mediante i seguenti metodi:

- Creare un'immagine da una lista di stringhe con la funzione `Image.set` o la funzione `Tilemap.set`
- Caricare un file immagine (png/gif/jpeg) nella palette di Pyxel con la funzione `Image.load`

I suoni Pyxel possono anche essere creati nel modo seguente:

- Creare un suono con le stringhe con la funzione `Sound.set` o la funzione `Music.set`

Riferirsi al manuale dell'API per l'uso di queste funzioni.

### Come distribuire l'applicazione

Pyxel supporta un formato file dedicato per la distribuzione dell'applicazione (Pyxel application file) che funziona su tutte le piattaforme.

Creare il file applicazione Pyxel (.pyxapp) con il seguente comando:

```sh
pyxel package APP_ROOT_DIR STARTUP_SCRIPT_FILE
```

Se l'applicazione dovrebbe includere risorse o moduli aggiuntivi, posizionarli nella cartella dell'applicazione.

Il file applicazione creato può essere eseguito con il seguente comando:

```sh
pyxel play PYXEL_APP_FILE
```

## Manuale API

### Sistema

- `width`, `height`<br>
Lunghezza e altezza dello schermo

- `frame_count`<br>
Numero di frame passati

- `init(width, height, [title], [fps], [quit_key], [capture_scale], [capture_sec])`<br>
Inizializza l'applicazione Pyxel con la dimensione dello schermo (`width`, `height`). I seguenti possono essere specificati come opzioni: il titolo della finestra con `title`, il frame rate con `fps`, il pulsante per uscire dall'applicazione con `quit_key`, la scala della cattura dello schermo con `capture_scale`, ed il tempo di registrazione massimo del video di cattura dello schermo con `capture_sec`.<br>
e.g. `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
Avvia l'applicazione Pyxel e chiama la funzione `update` per l'aggiornamento del frame e la funzione `draw` per disegnare.

- `show()`<br>
Mostra lo schermo e attende fino a quando il pulsante `Esc` non viene premuto. (Non usare in applicazioni normali)

- `flip()`<br>
Aggiorna lo schermo una volta sola. (Non usare in applicazioni normali)

- `quit()`<br>
Esci dall'applicazione Pyxel.

### Risorse

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Carica il file risorsa (.pyxres). Se ``False`` è specificato per il tipo di risorsa (``image/tilemap/sound/music``), la risorsa non sarà caricata.

### Input
- `mouse_x`, `mouse_y`<br>
La posizione corrente del cursore del mouse

- `mouse_wheel`<br>
Il valore corrente della rotella del mouse

- `btn(key)`<br>
Ritorna `True` se `key` è premuto, altrimenti ritorna `False` ([lista definizione tasti](../pyxel/__init__.pyi))

- `btnp(key, [hold], [period])`<br>
Ritorna `True` se `key` è premuto quel frame, altrimenti ritorna `False`. Quando `hold` e `period` sono specificati, `True` sarà ritornato all'intervallo frame `period` quando `key` è premuto per più di `hold` frame

- `btnr(key)`<br>
Ritorna `True` se `key` è rilasciato quel frame, altrimenti ritorna `False`

- `mouse(visible)`<br>
Se `visible` è `True`, mostra il cursore mouse. Se `False`, nascondilo. Anche se il cursore mouse non è mostrato, la sua posizione è aggiornata.

### Grafica

- `colors`<br>
Lista della palette colori del display. Il colore del display è specificato tramite un valore numerico a 24-bit. Usare `colors.from_list` e `colors.to_list` per assegnare direttamente e recuperare le liste Python.<br>
e.g. `org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img)`<br>
Opera il banco immagine `img` (0-2). (Vedere la classe Image)<br>
e.g. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
Opera la tilemap `tm`(0-7) (Vedere la classe Tilemap)

- `clip(x, y, w, h)`<br>
Imposta l'area di disegno dello schermo da (`x`, `y`) a lunghezza `w` e altezza `h`. Resettare l'area di disegno a schermo intero con `clip()`

- `camera(x, y)`<br>
Cambia le coordinate dell'angolo superiore sinistro dello schermo in (`x`, `y`). Resetta le coordinate dell'angolo superiore sinistro a (`0`, `0`) con `camera()`.

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

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Copia la regione di dimensione (`w`, `h`) da (`u`, `v`) della tilemap `tm` (0-7) a (`x`, `y`). Se un valore negativo è stato impostato per `w` e/o `h`, sarà rovesciata orizzontalmente e/o verticalmente. Se `colkey` è specificato, viene trattato come colore trasparente. La dimensione di una tile tile è di 8x8 pixel ed è memorizzata in una tilemap come una tupla di `(tile_x, tile_y)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
Disegna una stringa `s` di colore `col` su (`x`, `y`)

### Audio

- `sound(snd)`<br>
Opera il suono `snd`(0-63). (Vedere classe Sound).<br>
per esempio: `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Opera la musica `msc`(0-7) (Vedere la classe Music)

- `play_pos(ch)`<br>
Ottiene la posizione del suono in riproduzione del canale `ch` (0-3) come una tupla di `(sound no, note no)`. Ritorna `None` quando la riproduzione è interrotta.

- `play(ch, snd, [loop])`<br>
Riproduce il suono `snd` (0-63) sul canale `ch` (0-3). Se `snd` è una lista, verrà riprodotta in ordine. Se `True` è specificato per `loop`, viene eseguita la riproduzione in loop.

- `playm(msc, [loop])`<br>
Riproduce la musica `msc` (0-7). Se `True` è specificato per `loop`, viene eseguita la riproduzione in loop.

- `stop([ch])`<br>
Interrompe la riproduzione del canale `ch` (0-3) specificato. `stop()` per interrompere tutti i canali.

### Image Class

- `width`, `height`<br>
La lunghezza e l'altezza dell'immagine

- `data`<br>
I dati dell'immagine (lista bidimensionale da 256x256)

- `get(x, y)`<br>
Trova i dati dell'immagine su (`x`, `y`)

- `set(x, y, data)`<br>
Imposta l'immagine a (`x`, `y`) tramite una lista di stringhe.<br>
e.g. `pyxel.image(0).set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
Carica il file immagine (png/gif/jpeg) in (`x`, `y`).

### Classe Tilemap

- `width`, `height`<br>
Lunghezza e altezza della tilemap

- `refimg`<br>
Il banco immagine (0-2) referenziato dalla tilemap

- `set(x, y, data)`<br>
Imposta la tilemap a (`x`, `y`) mediante una lista di stringhe.<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `pget(x, y)`<br>
Prende la tile in (`x`, `y`). Una tile è una tupla di `(tile_x, tile_y)`.

- `pset(x, y, tile)`<br>
Disegna una `tile` in (`x`, `y`). Una tile è una tupla di `(tile_x, tile_y)`.

### Classe Sound

- `notes`<br>
Lista di note (0-127). Più alto il numero, più alto il tono, e a 33 diventa 'A2'(440Hz). Il resto è -1.

- `tones`<br>
Lista di tonalità (0:Triangolo / 1:Quadrato / 2:Ritmo / 3:Rumore)

- `volumes`<br>
Lista di volumi (0-7)

- `effects`<br>
Lista di effetti (0:Nessuno / 1:Scorrimento / 2:Vibrato / 3:Dissolvenza)

- `speed`<br>
Velocità di riproduzione. 1 è il più veloce, e più grande è il numero, più è lenta la velocità di riproduzione. A 120, la lunghezza di una nota diventa 1 secondo.

- `set(notes, tones, volumes, effects, speed)`<br>
Imposta note, tonalità, volumi, ed effetti con una stringa. Se il numero di tonalità, volumi, ed effetti è inferiore alle note, vengono ripetuti dall'inizio.

- `set_notes(notes)`<br>
Imposta le note con una stringa composta da 'CDEFGAB'+'#-'+'0123' o 'R'. Case-insensitive e gli spazi bianchi sono ignorati.<br>
e.g. `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
Imposta le tonalità con una stringa formata da 'TSPN'. Case-insensitive e gli spazi bianchi sono ignorati.<br>
e.g. `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volumes(volumes)`<br>
Imposta i volumi con una stringa formata da '01234567'. Case-insensitive e gli spazi bianchi sono ignorati.<br>
e.g. `pyxel.sound(0).set_volume("7777 7531")`

- `set_effects(effects)`<br>
Imposta gli effetti con una stringa formata da 'NSVF'. Case-insensitive e gli spazi bianchi sono ignorati.<br>
e.g. `pyxel.sound(0).set_effect("NFNF NVVS")`

### Music Class

- `sequences`<br>
Lista bidimensionale di suoni (0-63) elencati in base al numero di canali.

- `set(seq0, seq1, seq2, seq3)`<br>
Imposta l'elenco di suoni (0-63) di tutti i canali. Se è indicata una lista vuota, quel canale non viene utilizzato per la riproduzione.<br>
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### Advanced APIs

Pyxel ha "API avanzate" che non sono menzionate in questa documentazione perchè "potrebbero confondere gli utenti" oppure "necessitano di conoscenze specifiche per poter essere utilizzate".

Se ti senti confidente sulle tue competenze, prova a creare lavori incredibili con [this](../pyxel/__init__.pyi) come idea!

## Come contribuire

### Inviare Issue

Usa l'[Issue Tracker](https://github.com/kitao/pyxel/issues) per inviare segnalazioni su bug e richieste di funzionalità/migliorie. Prima di inviare una nuova issue, assicurati che non ci sia una issue simile aperta.

### Manual Testing

Chiunque è il benvenuto per testare manualmente il codice e riportare bug o suggerimenti per miglioramenti nell'[Issue Tracker](https://github.com/kitao/pyxel/issues)!

### Submitting Pull Request

Patch/fix sono accettati in forma di pull request (PR). Assicurarsi che il problema per cui si emetta una pull request sia aperto nel tracciante di problemi.

Le pull request emesse sono presupposte di accettare di essere pubblicate sotto la [licenza MIT](../LICENSE).

## Altre informazioni

- [Q&A](https://github.com/kitao/pyxel/wiki/Pyxel-Q&A)
- [User Examples](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Discord Server (English)](https://discord.gg/FC7kUZJ)
- [Discord Server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## Licenza

Pyxel è sotto [Licenza MIT](../LICENSE). Può essere riutilizzato all'interno di software proprietario, stabilito che tutte le copie del software o di sue porzioni sostanziali includano una copia dei termini della Licenza MIT ed anche un avviso di copyright.

## Reclutare Sponsor

Pyxel è alla ricerca di sponsor su GitHub Sponsor. Prendi in considerazione la sponsorizzazione di Pyxel per la manutenzione continua e l'aggiunta di funzionalità. Gli sponsor possono consultare Pyxel come vantaggio. Si prega di vedere [qui](https://github.com/sponsors/kitao) per i dettagli.
