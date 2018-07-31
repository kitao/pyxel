# <img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/assets/pyxel_logo_152x64.png">

[ [English](https://github.com/kitao/pyxel/blob/master/README.md) | [Japanese](https://github.com/kitao/pyxel/blob/master/README.ja.md) ]

**Pyxel** é um ambiente de desenvolvimento de jogos retro em Python.

Graças às suas especificações simples inspiradas em consoles de jogos retrô, como apenas 16 cores poderem ser exibidas e apenas 4 sons podem ser reproduzidos ao mesmo tempo, você pode se sentir à vontade para fazer jogos em estilo pixel art.

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/01_hello_pyxel.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/01_hello_pyxel.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/02_jump_game.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/02_jump_game.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/03_draw_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/03_draw_api.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/04_sound_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/04_sound_api.gif" width="48%">
</a>

As especificações do console de jogos, APIs e paletts do Pyxel estão se referindo ao incrível [PICO-8](https://www.lexaloffle.com/pico-8.php) e [TIC-80](https://tic.computer/).

Pyxel is open souce and free to use. Let's start making a retro game with Pyxel!

## Specifications

- Executável no Windows, Mac e Linux
- Código escrito em Python3
- Paleta fixa de 16 cores
- 4 bancos de imagens de tamanho 256x256
- 4 canais com 64 bancos de som definíveis
- Entrada de teclado, mouse e joystick(WIP)
- Editor de imagem e som (WIP)

## Como instalar

### Windows

Após instalar o [Python3](https://www.python.org/), o seguinte comando `pip` instala o Pyxel:

```sh
pip install pyxel
```

### Mac

Após instalar o [Python3](https://www.python.org/) e [glfw](http://www.glfw.org/), instale Pyxel com o comando `pip`.

Se o gerenciador de pacotes [Homebrew](https://brew.sh/) estiver instalado, o seguinte comando instala todos os pacotes necessários:

```sh
brew install python3 glfw
pip3 install pyxel
```

### Linux

Instale os pacotes necessários da maneira apropriada para cada distribuição.

**Arch:**

```sh
pacman -S python python-pip glfw portaudio
pip install pyxel
```

**Debian:**

```sh
apt-get install python3 python3-pip glfw libportaudio2 libasound-dev
pip3 install pyxel
```

### Instalando exemplos

Depois de instalar o Pyxel, os exemplos serão copiados para o atual diretório com o seguinte comando:

```sh
install_pyxel_examples
```

## Como usar

### Criando uma aplicação Pyxel

Depois de importar o módulo Pyxel para o seu código Python, primeiro especifique o tamanho da janela com a função `init`, depois inicie a aplicação Pyxel com a função `run`.

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

Os argumentos da função `run` são as funções `update`, para atualizar cada frame, e `draw` para desenhar a tela quando for necessário. 

Em um programa de verdade, é recomendado embrulhar o código pyxel em uma classe como feito abaixo:

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
        pyxel.rect(self.x, 0, self.x + 7, 7, 9)

App()
```

### Controles Especiais

Os seguintes controles especiais podem ser executados quando uma aplicação Pyxel estiver sendo executada.

- `Alt(Option)+1`  
Salva uma printscreen para a área de trabalho
- `Alt(Option)+2`  
Reinicia o momento inicial do vídeo de captura de tela.
- `Alt(Option)+3`  
Salva um vídeo de captura de tela (gif) para a área de trabalho (até 30 segundos)
- `Alt(Option)+0`  
Ativa/desativa o monitor de performance (fps, update time, e draw time)
- `Alt(Option)+Enter`  
Ativa/desativa tela cheia

### Criando Imagens

Existem os seguintes métodos para criar imagens para o Pyxel:

- Criar uma imagem a partir de uma lista de strings com a função `Image.set`
- Carregar um arquivo png na paleta de cores do Pyxel com a função `Image.load`
- Criar imagens com o Pyxel Editor (WIP)

Por favor, consulte a referência da API para uso das funções `Image.set` e` Image.load`.

Como o Pyxel usa a mesma paleta do [PICO-8](https://www.lexaloffle.com/pico-8.php), ao criar imagens png para o Pyxel, é recomendável usar o [Aseprite](https: // www.aseprite.org/) no modo de paleta PICO-8.

## Referência da API

### Sistema

- `width`, `height`  
A largura e a altura da tela.

- `frame_count`  
O número dos quadros decorridos

- `init(width, height, [caption], [scale], [palette], [fps], [border_width], [border_color])`  
Inicializa a aplicação Pyxel com o tamanho de tela (`width`, `height`)
Também é possível especificar o título da janela com `caption`, a ampliação do display com `scale`, a cor da paleta com `palette`, a taxa de quadros com `fps` e a largura e cor da margem de fora da tela com `border_width `e` border_color`

- `run(update, draw)`  
Inicia a aplicação Pyxel e chama a função `update` para atualização de quadros e a função `draw` para desenhar

- `quit()`  
Encerra a aplicação Pyel no fim do frame atual

### Entrada
- `mouse_x`, `mouse_y`  
A posição atual do cursor do mouse

- `btn(key)`  
Retorna `True` se `key` é pressionada, caso contrário retorna `False` ([lista de definições de teclas](https://github.com/kitao/pyxel/blob/master/pyxel/constants.py))

- `btnp(key, [hold], [period])`  
Retorna `True` se `key` é pressionada naquele frame, caso contrário retorna`False`. Quando `hold` e `period` são especificados, `True` será retornado durante o intervalo de quadros `period` enquanto `key` estiver pressionada por mais que `hold` frames

- `btnr(key)`  
Retorna `True` se `key` for solta naquele frame, caso contrário retorna `False`

### Gráficos

- `image(img)`  
Opera o banco de imagens `img`(0-3) (veja a classe Image)
e.g. `pyxel.image(0).load(0, 0, 'title.png')`

- `clip(x1, y1, x2, y2)`  
Define a área de desenho da tela para (`x1`, `y1`)-(`x2`, `y2`). Reseta a área de desenho com `clip()`

- `pal(col1, col2)`  
Substitui a cor `col1` com `col2` ao desenhar. Use `pal()` para resetar para a paleta inicial

- `cls(col)`  
Limpar a tela com a cor `col`

- `pix(x, y, col)`  
Draw a pixel of color `col` at (`x`, `y`)

- `line(x1, y1, x2, y2, col)`  
Draw a line of color `col` from (`x1`, `y1`) to (`x2`, `y2`)

- `rect(x1, y1, x2, y2, col)`  
Draw a rectanble of color `col` from (`x1`, `y1`) to (`x2`, `y2`)

- `rectb(x1, y1, x2, y2, col)`  
Draw the outline of a rectangle of color `col` from (`x1`, `y1`) to (`x2`, `y2`)

- `circ(x, y, r, col)`  
Draw a circle of radius `r` and color `col` at (`x`, `y`)

- `circb(x, y, r, col)`  
Draw the outline of a circle of radius `r` and color `col` at (`x`, `y`)

- `blt(x, y, img, sx, sy, w, h, [colkey])`  
Copy the region of size (`w`, `h`) from (`sx`, `sy`) of the image bank `img`(0-3) to (`x`, `y`). If negative value is set for `w` and/or `h`, it will reverse horizontally and/or vertically. If `colkey` is speficied, treated as transparent color

- `text(x, y, s, col)`  
Draw a string `s` of color `col` at (`x`, `y`)

### Audio

- `sound(snd)`  
Operate the sound bank `snd`(0-63) (see the Sound class)
e.g. `pyxel.sound(0).speed = 60`

- `play(ch, snd, loop=False)`  
Play the sound bank `snd`(0-63) on channel `ch`(0-3). Play in order when `snd` is a list

- `stop(ch)`  
Stop playback of channel `ch`(0-3)

### Image Class

- `width`, `height`  
The width and height of the Image

- `data`  
The data of the Image (NumPy array)

- `set(x, y, data)`  
Set the image as a list of strings at (`x`, `y`)   
e.g. `pyxel.image(0).set(10, 10, ['1234', '5678', '9abc', 'defg'])`

- `load(x, y, filename)`  
Read png image at (`x`, `y`)

- `copy(x, y, img, sx, sy, width, height)`  
Copy the region of size (`width`, `height`) from (`sx`, `sy`) of the image bank `img`(0-3) to (`x`, `y`)

### Sound Class

- `note`  
List of note(0-127) (33 = 'A2' = 440Hz)

- `tone`  
List of tone(0:Triagnle / 1:Square / 2:Pulse / 3:Noise)

- `volume`  
List of volume(0-7)

- `effect`  
List of effects(0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`  
The length of one note(120 = 1 second per tone)

- `set(note, tone, volume, effect, speed)`  
Set a note, tone, volume, and efffect with a string. If the tone, volume, and effect length are shorter than the note, it is repeated from the beginning

- `set_note(note)`  
Set the note with a string consists of 'CDEFGAB'+'#-'+'0123' or 'R'. Case-insensitive and whitespace is ignored  
e.g. `pyxel.sound(0).set_note('G2B-2D3R RF3F3F3')`

- `set_tone(tone)`  
Set the tone with a string consists of 'TSPN'. Case-insensitive and whitespace is ignored  
e.g. `pyxel.sound(0).set_tone('TTSS PPPN')`

- `set_volume(volume)`  
Set the volume with a string consists of '01234567'. Case-insensitive and whitespace is ignored  
e.g. `pyxel.sound(0).set_volume('7777 7531')`

- `set_effect(effect)`  
Set the effect with a string consists of 'NSVF'. Case-insensitive and whitespace is ignored  
e.g. `pyxel.sound(0).set_effect('NFNF NVVS')`

## License

Pyxel is under [MIT license](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software provided that all copies of the licensed software include a copy of the MIT License terms and the copyright notice.