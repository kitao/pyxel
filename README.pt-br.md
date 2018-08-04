# <img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/assets/pyxel_logo_152x64.png">

[ [English](https://github.com/kitao/pyxel/blob/master/README.md) | [Japanese](https://github.com/kitao/pyxel/blob/master/README.ja.md) | [Português](https://github.com/kitao/pyxel/blob/master/README-ptbr.md) ]

**Pyxel** é um ambiente de desenvolvimento de jogos retrôs em Python.

Graças às suas especificações simples inspiradas em consoles de jogos retrô, como apenas 16 cores podem ser exibidas e apenas 4 sons podem ser reproduzidos ao mesmo tempo, você pode se sentir à vontade para fazer jogos em estilo pixel art.

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

As especificações do console, APIs e paletas do Pyxel derivam dos incríveis [PICO-8](https://www.lexaloffle.com/pico-8.php) e [TIC-80](https://tic.computer/).

Pyxel é open source e livre para utilização. Vamos fazer jogos retrôs com Pyxel!

## Especificações

- Executável no Windows, Mac e Linux
- Código escrito em Python3
- Paleta fixa de 16 cores
- 4 bancos de imagens de tamanho 256x256
- 4 canais com 64 bancos de som definíveis
- Entrada de teclado, mouse e joystick(WIP)
- Editor de imagem e som (WIP)

### Paleta de cores

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/05_color_palette.png">

## Como instalar

### Windows

Após instalar o [Python3](https://www.python.org/), o seguinte comando `pip` instala o Pyxel:

```sh
pip install pyxel
```

### Mac

Após instalar o [Python3](https://www.python.org/) e [glfw](http://www.glfw.org/) (versão 3.2.1 ou superior), instale Pyxel com o comando `pip`.

Se o gerenciador de pacotes [Homebrew](https://brew.sh/) estiver instalado, o seguinte comando instala todos os pacotes necessários:

```sh
brew install python3 glfw
pip3 install pyxel
```

### Linux

Instale os pacotes necessários da maneira apropriada para cada distribuição. [glfw](http://www.glfw.org/) deve ser versão 3.2.1 ou superior.

**Arch:**

Instale [`python-pixel`](https://aur.archlinux.org/packages/python-pyxel/) usando o seu assistente AUR favorito:

```sh
yay -S python-pyxel
```

**Debian:**

```sh
apt-get install python3 python3-pip libglfw3 libportaudio2 libasound-dev
pip3 install pyxel
```

### Instalando os exemplos

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
Salva uma captura de tela para a área de trabalho
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

Por favor, consulte a referência do API para uso das funções `Image.set` e` Image.load`.

Como o Pyxel usa a mesma paleta do [PICO-8](https://www.lexaloffle.com/pico-8.php), ao criar imagens png para o Pyxel, é recomendável usar o [Aseprite](https://www.aseprite.org/) no modo de paleta PICO-8.

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
Encerra a aplicação Pyxel no fim do quadro atual

### Entrada
- `mouse_x`, `mouse_y`  
A posição atual do cursor do mouse

- `btn(key)`  
Retorna `True` se `key` é pressionada, caso contrário retorna `False` ([lista de definições de teclas](https://github.com/kitao/pyxel/blob/master/pyxel/constants.py))

- `btnp(key, [hold], [period])`  
Retorna `True` se `key` for pressionada naquele quadro, caso contrário retorna`False`. Quando `hold` e `period` são especificados, `True` será retornado durante o intervalo de quadros `period`, enquanto `key` estiver pressionada por mais que `hold` quadros

- `btnr(key)`  
Retorna `True` se `key` for solta naquele quadro, caso contrário retorna `False`

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
Desenha um pixel de cor `col` em (`x`, `y`)

- `line(x1, y1, x2, y2, col)`  
Desenha uma linha da cor `col` de (`x1`, `y1`) até (`x2`, `y2`)

- `rect(x1, y1, x2, y2, col)`  
Desenha um retângulo da cor `col` de (`x1`, `y1`) até (`x2`, `y2`)

- `rectb(x1, y1, x2, y2, col)`  
Desenha o contorno de um retângulo da cor `col` de (`x1`, `y1`) até (`x2`, `y2`)

- `circ(x, y, r, col)`  
Desenha um círculo de raio `r` e cor `col` em (`x`, `y`)

- `circb(x, y, r, col)`  
Desenha o contorno de um círculo de raio `r` e cor `col` em (`x`, `y`)

- `blt(x, y, img, sx, sy, w, h, [colkey])`  
Copia a região de tamanho (`w`, `h`) de (`sx`, `sy`) do banco de imagens `img`(0-3) para (`x`, `y`). Se um valor negativo for definido para `w` e/ou `h`, será invertido horizontalmente e/ou verticalmente. Se `colkey` for especificado, será tratado como cor transparente.

- `text(x, y, s, col)`  
Desenha uma string `s` de cor `col` em (`x`, `y`)

### Audio

- `sound(snd)`  
Opera o banco de sons `snd`(0-63) (ver a classe Sound)
e.g. `pyxel.sound(0).speed = 60`

- `play(ch, snd, loop=False)`  
Reproduz o banco de som `snd`(0-63) no canal `ch`(0-3). Tocar em ordem quandp `snd` for uma lista

- `stop(ch)`  
Interrompe a reprodução do canal `ch`(0-3)

### Classe Image

- `width`, `height`  
Largura e altura da Image

- `data`  
Os dados da Image (NumPy array)

- `set(x, y, data)`  
Define a imagem como uma lista de strings em (`x`, `y`)
e.g. `pyxel.image(0).set(10, 10, ['1234', '5678', '9abc', 'defg'])`

- `load(x, y, filename)`  
Lê uma imagem png em (`x`, `y`)

- `copy(x, y, img, sx, sy, width, height)`  
Copia a região de tamanho (`width`, `height`) de (`sx`, `sy`) do banco de imagens `img`(0-3) para (`x`, `y`)

### Classe Sound

- `note`  
Lista de notas(0-127) (33 = 'A2' = 440Hz)

- `tone`  
Lista de tons(0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volume`  
List de volume(0-7)

- `effect`  
Lista de efeitos(0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`  
Duração de uma nota(120 = 1 second per tone)

- `set(note, tone, volume, effect, speed)`  
Define uma nota, tom, volume e efeito com uma string. Se o tom, volume e duração do efeito forem mais curtas que a nota, será repetida do começo

- `set_note(note)`  
Define a nota com uma string consistindo de 'CDEFGAB'+'#-'+'0123' ou 'R'. Indiferente a maiúsculas e minúsculas e espaços são ignorados
e.g. `pyxel.sound(0).set_note('G2B-2D3R RF3F3F3')`

- `set_tone(tone)`  
Define um tom com uma string consistindo de 'TSPN'. Indiferente a maiúsculas e minúsculas e espaços são ignorados
e.g. `pyxel.sound(0).set_tone('TTSS PPPN')`

- `set_volume(volume)`  
Define o volume com uma string consistindo de '01234567'. Indiferente a maiúsculas e minúsculas e espaços são ignorados  
e.g. `pyxel.sound(0).set_volume('7777 7531')`

- `set_effect(effect)`  
Define o efeito com uma string consistindo de 'NSVF'. Indiferente a maiúsculas e minúsculas e espaços são ignorados  
e.g. `pyxel.sound(0).set_effect('NFNF NVVS')`

## Licença

Pyxel is under [MIT license](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software provided that all copies of the licensed software include a copy of the MIT License terms and the copyright notice.
