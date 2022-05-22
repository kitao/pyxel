# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**Pyxel** é um motor de jogos retrô para Python.

Graças às suas especificações simples inspiradas em consoles de jogos retrô, assim como permitir a exibição de apenas 16 cores e apenas 4 sons poderem ser reproduzidos ao mesmo tempo, você pode se sentir à vontade para fazer jogos em estilo pixel art.

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

As especificações e APIs de Pyxel são inspiradas por [PICO-8](https://www.lexaloffle.com/pico-8.php) e [TIC-80](https://tic80.com/).

Pyxel é open source e livre para utilização. Vamos começar fazendo um jogo retrô com Pyxel!

## Especificações

- Roda em Windows, Mac e Linux
- Programação com Python
- Paleta de 16 cores
- 3 bancos de imagens de tamanho 256x256
- 8 tilemaps de tamanho 256x256
- 4 canais com 64 sons definíveis
- 8 músicas que podem combinar sons arbitrários
- Entradas de teclado, mouse e joystick
- Editor de imagem e som

### Paleta de cores

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Como instalar

Existem dois tipos de Pyxel, uma versão empacotada e uma versão independente.

### Instalando a versão empacotada

A versão empacotada do Pyxel usa o Pyxel como um módulo de extensão do Python.

Recomendado para as pessoas que estejam familiarizadas com o gerenciamento de pacotes Python usando o comando `pip` ou que queiram desenvolver aplicações Python completas.

**Windows**

Após instalar o [Python3](https://www.python.org/) (versão 3.7 ou superior), execute o seguinte comando:

```sh
pip install -U pyxel
```

**Mac**

Após instalar o [Python3](https://www.python.org/) (versão 3.7 ou superior), execute o seguinte comando:

```sh
pip3 install -U pyxel
```

**Linux**

Após instalar o pacote SDL2 (`libsdl2-dev` no Ubuntu), [Python3](https://www.python.org/) (versão 3.7 ou superior), e `python3-pip`, execute o seguinte comando:

```sh
sudo pip3 install -U pyxel
```

Se o comando acima não funcionou, tente a compilação manual seguindo os próximos passos após instalar o `cmake` e o `rust`:

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make clean all
sudo pip3 install .
```

### Instalando a versão independente

A versão independente do Pyxel usa o Pyxel como uma ferramenta independente que não depende do Python.

Recomendado para as pessoas que queiram começar a programar sem se preocupar com as configurações do Python, ou que queiram jogar jogos Pyxel imediatamente.

**Windows**

Baixe e execute a versão mais recente do instalador Windows (`pyxel-[version]-windows-setup.exe`) da [Página de download](https://github.com/kitao/pyxel/releases).

**Mac**

Após instalar o [Homebrew](https://brew.sh/), execute o seguinte comando:

```sh
brew tap kitao/pyxel
brew install pyxel
```

**Linux**

Após instalar o pacote SDL2 (`libsdl2-dev` no Ubuntu) e instalar o [Homebrew](https://brew.sh/), execute os seguintes comandos:

```sh
brew tap kitao/pyxel
brew install pyxel
```

Se os passos acima não funcionarem, tente a compilação manual da versão empacotada.

### Testando os exemplos Pyxel

Após instalar o Pyxel, os exemplos serão copiados para o diretório atual com o seguinte comando:

```sh
pyxel copy_examples
```

Os exemplos copiados são os seguintes:

- [01_hello_pyxel.py](../pyxel/examples/01_hello_pyxel.py) - Aplicação simples
- [02_jump_game.py](../pyxel/examples/02_jump_game.py) - Jogo de pulo com o arquivo de recursos do Pyxel
- [03_draw_api.py](../pyxel/examples/03_draw_api.py) - Demonstração das APIs de desenho
- [04_sound_api.py](../pyxel/examples/04_sound_api.py) - Demonstração das APIs de som
- [05_color_palette.py](../pyxel/examples/05_color_palette.py) - Lista da paleta de cores
- [06_click_game.py](../pyxel/examples/06_click_game.py) - Jogo de clique com mouse
- [07_snake.py](../pyxel/examples/07_snake.py) - Jogo Snake com BGM
- [08_triangle_api.py](../pyxel/examples/08_triangle_api.py) - Demonstração da API de desenho de triângulos
- [09_shooter.py](../pyxel/examples/09_shooter.py) - Jogo de tiro com transição de tela
- [10_platformer.py](../pyxel/examples/10_platformer.py) - Jogo side-scrolling de plataforma com mapa
- [11_offscreen.py](../pyxel/examples/11_offscreen.py) - Renderização fora do ecrã com classe de Image
- [12_perlin_noise.py](../pyxel/examples/12_perlin_noise.py) - Animação sonora Perlin
- [30SecondsOfDaylight.pyxapp](images/30SecondsOfDaylight.gif) - 1º jogo vencedor de Pyxel Jam de [Adam](https://twitter.com/helpcomputer0)
- [megaball.pyxapp](images/megaball.gif) - Jogo de física de bola arcade por [Adam](https://twitter.com/helpcomputer0)

Os exemplos podem ser executados com os seguintes comandos:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30SecondsOfDaylight.pyxapp
```

## Como usar

### Criando uma aplicação Pyxel

Após importar o módulo Pyxel em seu script Python, especifique o tamanho da janela com a função `init`, em seguida inicialize a aplicação Pyxel com a função `run`.

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

Em uma aplicação real, é recomendado colocar código pyxel em uma classe, como feito abaixo:

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

Também é possível escrever um código simples usando a função `show` e a função `flip` para desenhar gráficos básicos e animações.

A função `show` mostra a tela e espera até que a tecla `Esc` seja pressionada.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

A função `flip` atualiza a tela uma vez.

```python
import pyxel

pyxel.init(120, 80)

while True:
    pyxel.cls(3)
    pyxel.rectb(pyxel.frame_count % 160 - 40, 20, 40, 40, 7)
    pyxel.flip()
```

### Executando uma aplicação Pyxel

O script Python criado pode ser executado com o seguinte comando:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Para a versão empacotada, ele também pode ser executado como um script Python comum:

```sh
cd pyxel_examples
python3 PYTHON_SCRIPT_FILE
```

(No Windows, use `python` ao invés de `python3`)

### Controles Especiais

Os seguintes controles especiais podem ser executados quando uma aplicação Pyxel estiver sendo executada:

- `Esc`<br>
Encerra a aplicação
- `Alt(Option)+1`<br>
Salva uma captura de tela para a área de trabalho
- `Alt(Option)+2`<br>
Reinicia o momento inicial do vídeo de captura de tela.
- `Alt(Option)+3`<br>
Salva um vídeo de captura de tela na área de trabalho (até 10 segundos)
- `Alt(Option)+0`<br>
Ativa/desativa o monitor de performance (fps, tempo de update e tempo de draw)
- `Alt(Option)+Enter`<br>
Ativa/desativa tela cheia

### Como criar um Recurso

O Editor Pyxel pode criar imagens e sons usados em uma aplicação Pyxel.

Ele é inicializado com o seguinte comando:

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

Se o arquivo de recursos Pyxel (.pyxres) existir, o arquivo será carregado, e se ele não existir, um novo arquivo com o nome especificado será criado.
Se o arquivo de recursos for omitido, o nome será `my_resource.pyxres`.

Após iniciar o Editor Pyxel, o arquivo pode ser trocado arrastando e soltando outro arquivo de recursos. Se o arquivo for arrastado segurando a tecla ``Ctrl(Cmd)``, somente o recurso (Imagem/Tilemap/Som/Musica) que estiver sendo editado no momento será carregado. Esta operação possibilita a combinar múltiplos arquivos de recursos em um só.

O arquivo recursos criado pode ser carregado através da função `load`.

O Editor Pyxel possuí os seguintes modos de edição.

**Editor de Imagem:**

O modo para editar bancos de imagem.

<img src="images/image_editor.gif">

Arrastando e soltando um arquivo de imagem (png/gif/jpeg) dentro da tela do Editor de Imagens faz com que a imagem possa ser carregada no banco de imagens selecionado no momento.

**Editor de Tilemap:**

O modo para editar tilemaps em que imagens dos bancos de imagens são organizados em um padrão de tiles.

<img src="images/tilemap_editor.gif">

**Editor de Som:**

O modo para editar sons.

<img src="images/sound_editor.gif">

**Editor de Música:**

O modo para editar músicas nas quais os sons são organizados na ordem de execução.

<img src="images/music_editor.gif">

### Outros métodos de criação de recursos

Imagens e tilemaps Pyxel também podem ser criados pelos seguintes métodos:

- Criar uma imagem de uma lista de strings com a função `Image.set` ou com a função `Tilemap.set`
- Carregar um arquivo de imagem (png/gif/jpeg) na paleta Pyxel com a função `Image.load`

Sons Pyxel também podem ser criados com o seguinte método:

- Criar um som de uma strings com a função `Sound.set` ou com a função `Music.set`

Favor consultar a referência da API para o uso dessas funções.

### Como distribuir uma aplicação

O Pyxel suporta um formato de arquivo de distribuição dedicado (arquivo de aplicação Pyxel) que é multiplataforma.

Crie um arquivo de aplicação Pyxel (.pyxapp) com o seguinte comando:

```sh
pyxel package APP_ROOT_DIR STARTUP_SCRIPT_FILE
```

Se a aplicação precisa incluir recursos ou módulos adicionais, coloque eles na pasta da aplicação.

O arquivo de aplicação pode ser executado com o seguinte comando:

```sh
pyxel play PYXEL_APP_FILE
```

## Referência da API

### Sistema

- `width`, `height`<br>
A largura e a altura da tela

- `frame_count`<br>
O número dos quadros decorridos

- `init(width, height, [title], [fps], [quit_key], [capture_scale], [capture_sec])`<br>
Inicializa a aplicação Pyxel com tamanho de tela (`width`, `height`). As seguintes opções podem ser especificadas: o título da janela com `title`, a taxa de quadros com `fps`, a tecla para fechar a aplicação com `quit_key`, a escala da captura de tela com `capture_scale`, o tempo máximo de gravação do vídeo da captura de tela `capture_sec`.<br>
Ex. `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
Roda a aplicação Pyxel e chama a função `update` para atualizar os quadros e a função `draw` para desenhá-los.

- `show()`<br>
Mostra a tela e espera até a tecla `Esc` ser pressionada. (Não utilizar em aplicações normais)

- `flip()`<br>
Atualiza a tela uma vez. (Não utilizar em aplicações normais)

- `quit()`<br>
Feche a aplicação Pyxel.

### Recurso

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Carrega o arquivo de recursos (.pyxres). Se ``False`` for especificado para o tipo de recurso (``image/tilemap/sound/music``), o recurso não será carregado.

### Entrada
- `mouse_x`, `mouse_y`<br>
A posição atual do cursor do mouse

- `mouse_wheel`<br>
O valor atual da roda de rolagem do mouse

- `btn(key)`<br>
Retorna `True` se `key` é pressionada, caso contrário retorna `False` ([lista de definições de teclas](../pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
Retorna `True` se `key` for pressionada naquele quadro, caso contrário retorna `False`. Quando `hold` e `repeat` são especificados, `True` será retornado durante o intervalo de quadros `repeat`, no qual `key` estiver pressionada por mais que `hold` quadros

- `btnr(key)`<br>
Retorna `True` se `key` for solta naquele quadro, caso contrário retorna `False`

- `mouse(visible)`<br>
Se `visible` for `True`, mostra o cursor do mouse. Se for `False`, esconde. Mesmo se o cursor do mouse não for visível, sua posição é atualizada.

### Gráficos

- `colors`<br>
Lista da paleta de cores da tela. A cor da tela é especificada por um valor numérico de 24 bits. Use `colors.from_list` e `colors.to_list` para atribuir e pegar listas do Python.<br>
Ex. `org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img)`<br>
Opera o banco de imagens `img` (0-2). (veja a classe de Imagem)<br>
Ex. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
Opera o tilemap `tm`(0-7) (ver a classe de Tilemap)

- `clip(x, y, w, h)`<br>
Define a área de desenho da tela de (`x`, `y`) para a largura `w` e altura `h`. Redefina a área de desenho para tela cheia com `clip()`

- `camera(x, y)`<br>
Altera as coordenadas do canto superior esquerdo da tela para (`x`, `y`). Redefina as coordenadas do canto superior esquerdo para (`0`, `0`) com `camera()`.

- `pal(col1, col2)`<br>
Substitui a cor `col1` com `col2` ao desenhar. Use `pal()` para voltar para a paleta inicial

- `cls(col)`<br>
Limpar a tela com a cor `col`

- `pget(x, y)`<br>
Captura a cor de um pixel em (`x`, `y`)

- `pset(x, y, col)`<br>
Desenha um pixel de cor `col` em (`x`, `y`)

- `line(x1, y1, x2, y2, col)`<br>
Desenha uma linha da cor `col` de (`x1`, `y1`) até (`x2`, `y2`)

- `rect(x, y, w, h, col)`<br>
Desenha um retângulo de largura `w`, altura `h` e cor `col` a partir de (`x`, `y`)

- `rectb(x, y, w, h, col)`<br>
Desenha o contorno de um retângulo de largura `w`, altura `h` e cor `col` a partir de (`x`, `y`)

- `circ(x, y, r, col)`<br>
Desenha um círculo de raio `r` e cor `col` em (`x`, `y`)

- `circb(x, y, r, col)`<br>
Desenha o contorno de um círculo de raio `r` e cor `col` em (`x`, `y`)

- `elli(x, y, w, h, col)`<br>
Desenhar uma elipse de largura `w`, altura `h` e cor `col` de (`x`, `y`).

- `ellib(x, y, w, h, col)`<br>
Desenhar o contorno de uma elipse de largura `w`, altura `h` e cor `col` de (`x`, `y`).

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
Desenha um triangulo com os vértices (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e cor `col`

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
Desenha o contorno de um triangulo com os vértices (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e cor `col`

- `fill(x, y, col)`<br>
Desenhar uma elipse de largura `w`, altura `h` e cor `col` de (`x`, `y`).

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
Copia a região de tamanho (`w`, `h`) de (`u`, `v`) do banco de imagens `img` (0-2) para (`x`, `y`). Se um valor negativo for definido para `w` e/ou `h`, será invertido horizontalmente e/ou verticalmente. Se `colkey` for especificada, será tratado como cor transparente

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Copia a região de tamanho (`w`, `h`) de (`u`, `v`) do tilemap `tm` (0-7) para (`x`, `y`). Se um valor negativo for definido para `w` e/ou `h`, será invertido horizontalmente e/ou verticalmente. Se `colkey` for especificada, será tratado como cor transparente. O tamanho de um tile é de 8x8 pixels e é armazenado em um tilemap como uma tupla de `(tile_x, tile_y)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
Desenha uma string `s` de cor `col` em (`x`, `y`)

### Áudio

- `sound(snd)`<br>
Opera o som `snd`(0-63). (ver a classe de Som)<br>
Ex. `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Opera a música `msc` (0-7) (ver a classe de Musica)

- `play_pos(ch)`<br>
Obtém a posição do canal `ch` (0-3) da reprodução de som como uma tupla de `(sound no, note no)`. Retorna `None` quando a reprodução para.

- `play(ch, snd, [tick], [loop])`<br>
Reproduz o som `snd` (0-63) no canal `ch` (0-3). Se `snd` é uma lista, os sons serão reproduzidos em ordem. A posição inicial da reprodução pode ser especificada por `tick` (1 tick = 1/120 segundos). Se `True` for especificado para `loop`, a reprodução será feita em laço.

- `playm(msc, [tick], [loop])`<br>
Reproduz a música `msc` (0-7). A posição inicial da reprodução pode ser especificada por `tick` (1 tick = 1/120 segundos). Se `True` for especificado para `loop`, a reprodução será feita em laço.

- `stop([ch])`<br>
Para a reprodução do canal `ch` (0-3). `stop()` para parar a reprodução de todos os canais.

### Matemática

- `ceil(x)`<br>
Devolve o menor inteiro maior ou igual a `x`.

- `floor(x)`<br>
Devolve o maior inteiro menor ou igual a `x`.

- `sgn(x)`<br>
Retorna 1 quando o `x` é positivo, 0 quando é zero, e -1 quando é negativo.

- `sqrt(x)`<br>
Devolve a raiz quadrada de `x`.

- `sin(deg)`<br>
Devolve o seno de `deg` graus.

- `cos(deg)`<br>
Retorna o cosseno de `deg` graus.

- `atan2(y, x)`<br>
Devolve o arctangente de `y`/`x` em graus.

- `rseed(seed: int)`<br>
Define a semente do gerador do número aleatório.

- `rndi(a, b)`<br>
Retorna um inteiro aleatório maior ou igual a `a' e menor ou igual a `b'.

- `rndf(a, b)`<br>
Devolve uma decimal aleatória maior ou igual a `a` e menor ou igual a `b`.

- `nseed(seed)`<br>
Define a semente do ruído de Perlin.

- `noise(x, [y], [z])`<br>
Retorna o valor do ruído Perlin para as coordenadas especificadas.

### Classe de Imagem

- `width`, `height`<br>
Largura e altura da imagem

- `set(x, y, data)`<br>
Define a imagem em (`x`, `y`) por uma lista de strings.<br>
Ex. `pyxel.image(0).set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
Carrega um arquivo de imagem (png/gif/jpeg) em (`x`, `y`).

- `pget(x, y)`<br>
Pega a cor do pixel em (`x`, `y`).

- `pset(x, y, col)`<br>
Desenha um pixel de cor `col` em (`x`, `y`).

### Classe de Tilemap

- `width`, `height`<br>
A largura e a altura do tilemap

- `refimg`<br>
O banco de imagem (0-2) referenciado pelo tilemap

- `set(x, y, data)`<br>
Define o tilemap em (`x`, `y`) por uma lista de strings.<br>
Ex. `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `pget(x, y)`<br>
Pega o tile em (`x`, `y`). Um tile é uma tupla de `(tile_x, tile_y)`.

- `pset(x, y, tile)`<br>
Desenha um `tile` em (`x`, `y`). Um tile é uma tupla de `(tile_x, tile_y)`.

### Classe de Som

- `notes`<br>
Lista de notas (0-127). Quanto maior o número, mais agudo, e ao chegar em 33 ele se torna 'A2'(440Hz). O resto é -1.

- `tones`<br>
Lista de tons (0:Triangular / 1:Quadrada / 2:Pulso / 3:Ruído)

- `volumes`<br>
Lista de volumes (0-7)

- `effects`<br>
Lista de efeitos (0:Nenhum / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`<br>
Velocidade de reprodução. 1 é a velocidade maior, e quanto maior o número, mais lenta ela é. No valor 120, o tempo de uma nota se torna 1 segundo.

- `set(notes, tones, volumes, effects, speed)`<br>
Define as notas, tons, volumes e efeitos com uma string. Se os tons, volumes e efeitos são mais curtos que as notas, elas se repetirão do começo.

- `set_notes(notes)`<br>
Define as notas com uma string 'CDEFGAB'+'#-'+'0123' ou 'R'. É insensível à maiúsculas ou minúsculas e espaços em branco são ignorados.<br>
Ex. `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
Define os tons com uma string composta por 'TSPN'. É insensível à maiúsculas ou minúsculas e espaços em branco são ignorados.<br>
Ex. `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volumes(volumes)`<br>
Define os volumes com uma string composta por '01234567'. É insensível à maiúsculas ou minúsculas e espaços em branco são ignorados.<br>
Ex. `pyxel.sound(0).set_volume("7777 7531")`

- `set_effects(effects)`<br>
Define os efeitos com uma string composta por 'NSVF'. É insensível à maiúsculas ou minúsculas e espaços em branco são ignorados.<br>
Ex. `pyxel.sound(0).set_effect("NFNF NVVS")`

### Classe de Musica

- `snds_list`<br>
Lista bi-dimensional de sons (0-63) com o número de canais.

- `set(snds0, snds1, snds2, snds3)`<br>
Define as listas de sons (0-63) para todos os canais. Se uma lista vazia for especificada, aquele canal não será utilizado para reprodução de sons.<br>
Ex. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### APIs Avançadas

Pyxel possui "APIs avançadas" que não são mencionadas nesse manual de referência pois elas podem "confundir usuários" ou "precisam de conhecimento especializado para usar".

Se você está familiarizado com suas habilidades, tente criar projetos incríveis utilizando [isto](../pyxel/__init__.pyi) como pista!

## Como Contribuir

### Relatando Problemas

Utilize o [Issue Tracker](https://github.com/kitao/pyxel/issues) para relatar bugs e sugerir funcionalidades/melhorias. Antes de relatar uma issue, tenha certeza que não exista uma issue similar aberta.

### Teste Manual

Qualquer um testando o código manualmente e relatando bugs ou sugestões de melhorias no [Issue Tracker](https://github.com/kitao/pyxel/issues) são muito bem vindos!

### Submetendo uma Pull Request

Patches/correções serão aceitas na forma de pull requests (PRs). Tenha certeza de que o que o pull request tenta resolver esteja em aberto no issue tracker.

Será considerado que todo pull request tenha concordado a ser publicado sob a [licença MIT](../LICENSE).

## Outras informações

- [Q&A](https://github.com/kitao/pyxel/wiki/Pyxel-Q&A)
- [User Examples](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Discord Server (English)](https://discord.gg/Z87eYHN)
- [Discord Server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## Licença

Pyxel está sob a [licença MIT](../LICENSE). Ele pode ser reutilizado em software proprietário, contanto que todas as cópias do software ou sua porções substanciais incluam uma cópia dos termos da licença MIT e um aviso de direitos autorais.

## Recrutando Patrocinadores

Pyxel está procurando patrocinadores nos patrocinadores do GitHub. Considere patrocinar o Pyxel para manutenção contínua e acréscimos de recursos. Os patrocinadores podem consultar sobre o Pyxel como um benefício. Por favor, veja [aqui](https://github.com/sponsors/kitao) para detalhes.
