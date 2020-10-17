# <img src="images/pyxel_logo_152x64.png">

[ [English](README.md) | [中文](README.cn.md) | [Español](README.es.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**Pyxel** é uma engine de jogos retrô para Python.

Graças às suas especificações simples inspiradas em consoles de jogos retrô, assim como permitir a exibição de apenas 16 cores e apenas 4 sons poderem ser reproduzidos ao mesmo tempo, você pode se sentir à vontade para fazer jogos em estilo pixel art.

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

As especificações do console e APIs do Pyxel fazem referência ao incrível [PICO-8](https://www.lexaloffle.com/pico-8.php) e [TIC-80](https://tic.computer/).

Pyxel é open source e livre para utilização. Vamos começar fazendo um jogo retrô com Pyxel!

## Especificações

- Executável no Windows, Mac e Linux
- Código escrito em Python3
- Paleta fixa de 16 cores
- 3 bancos de imagens de tamanho 256x256
- 8 tilemaps de tamanho 256x256
- 4 canais com 64 sons definíveis
- 8 músicas que podem combinar sons arbitrários
- Entrada de teclado, mouse e joystick
- Editor de imagem e som

### Paleta de cores

<img src="pyxel/examples/screenshots/05_color_palette.png">
<br><br>
<img src="images/pyxel_palette.png">

## Como instalar

### Windows

Primeiro, instale o [Python3](https://www.python.org/) (versão 3.6.8 ou superior).

Durante a instalação utilizando o instalador oficial do Python, não esqueça de **adicionar o Python no PATH** selecionando a seguinte opção:

<img src="images/python_installer.png">

Em seguida, instale o Pyxel com o seguinte comando do `pip` na linha de comando:

```sh
pip install -U pyxel
```

### Mac

Primeiro, no ambiente com o [Homebrew] (https://brew.sh/) instalado, instale o [Python3] (https://www.python.org/) (versão 3.6.8 ou superior) e os pacotes necessários com o próximo comando:

```sh
brew install python3 gcc sdl2 sdl2_image gifsicle
```

Você pode instalar o Python3 de outras maneiras, mas lembre-se de que você deve instalar outras bibliotecas.

Em seguida, **reinicie o terminal** e instale o Pyxel com o commando `pip3`:

```sh
pip3 install -U pyxel
```

### Linux

Instale [Python3](https://www.python.org/) (versão 3.6.8 ou superior) e os requisitos específicos para cada distribuição.

**Ubuntu:**

```sh
sudo apt install python3 python3-pip libsdl2-dev libsdl2-image-dev gifsicle
sudo -H pip3 install -U pyxel
```

### Outros ambientes

Para instalar o Pyxel em ambientes diferentes dos anteriores (Linux 32-bit, Raspberry PI, etc.), siga os passos abaixo:

#### Instale as ferramentas e pacotes necessários

- Conjunto de ferramentas C++ (deve incluir os comandos gcc e make)
- libsdl2-dev and libsdl2-image-dev
- [Python3](https://www.python.org/) (versão 3.6.8 ou superior) e o comando pip

#### Execute o comando seguinte em qualquer diretório

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make -C pyxel/core clean all
pip3 install .
```

### Instalando exemplos

Após instalar o Pyxel, os exemplos serão copiados para o diretório atual com o seguinte comando:

```sh
install_pyxel_examples
```

Os exemplos copiados são os seguintes:

- [01_hello_pyxel.py](pyxel/examples/01_hello_pyxel.py) - Aplicação simples
- [02_jump_game.py](pyxel/examples/02_jump_game.py) - Jogo de pulo com o arquivo de recursos do Pyxel
- [03_draw_api.py](pyxel/examples/03_draw_api.py) - Demonstração da API de desenho
- [04_sound_api.py](pyxel/examples/04_sound_api.py) - Demonstração da API de som
- [05_color_palette.py](pyxel/examples/05_color_palette.py) - Lista da paleta de cores
- [06_click_game.py](pyxel/examples/06_click_game.py) - Jogo de clique com mouse
- [07_snake.py](pyxel/examples/07_snake.py) - Jogo Snake com BGM
- [08_triangle_api.py](pyxel/examples/08_triangle_api.py) - Demonstração da API de desenho de triângulo
- [09_shooter.py](pyxel/examples/09_shooter.py) - Jogo de tiro com transição de tela

Os exemplos podem ser executados como um programa Python comum:

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

Também é possível escrever um código simples utilizando as funções `show` e `flip` para desenhar gráficos e animações simples.

A função `show` desenha na tela e espera até a tecla `ESC` ser pressionada.

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

### Controles Especiais

Os seguintes controles especiais podem ser executados quando uma aplicação Pyxel estiver sendo executada:

- `Esc`<br>
Encerra a aplicação
- `Alt(Option)+1`<br>
Salva uma captura de tela para a área de trabalho
- `Alt(Option)+2`<br>
Reinicia o momento inicial do vídeo de captura de tela.
- `Alt(Option)+3`<br>
Salva um vídeo de captura de tela (gif) na área de trabalho (até 30 segundos)
- `Alt(Option)+0`<br>
Ativa/desativa o monitor de performance (fps, tempo de update e tempo de draw)
- `Alt(Option)+Enter`<br>
Ativa/desativa tela cheia

### Como criar um Recurso

O Editor Pyxel pode criar imagens e sons usados em uma aplicação Pyxel.

O Editor Pyxel é iniciado com o seguinte comando:

```sh
pyxeleditor [pyxel_resource_file]
```

Se o arquivo de recursos Pyxel (.pyxres) existir, o arquivo será carregado, e se ele não existir, um novo arquivo com o nome especificado será criado.
Se o arquivo de recursos for omitido, o nome será `my_resource.pyxres`.

Após iniciar o Editor Pyxel, o arquivo pode ser trocado arrastando e soltando outro arquivo de recursos. Se o arquivo de recursos for arrastado segurando a tecla ``Ctrl``(``Cmd``), somente o tipo de recurso (imagem/tilemap/som/musica) que está sendo editado será carregado. Esta operação permite combinar múltiplos arquivos de recurso em um só.

O arquivo de recurso criado pode ser carregado com a função `load`.

O Editor Pyxel possuí os seguintes modos de edição.

**Editor de Imagem:**

O modo para editar bancos de imagem.

<img src="pyxel/editor/screenshots/image_editor.gif">

Ao arrastar e soltar um arquivo png na tela do Editor de Imagens, a imagem pode ser carregada no banco de imagens atualmente selecionado.

**Editor de Tilemap:**

O modo para editar tilemaps em que imagens dos bancos de imagens são organizados em um padrão de tiles.

<img src="pyxel/editor/screenshots/tilemap_editor.gif">

**Editor de Som:**

O modo para editar sons.

<img src="pyxel/editor/screenshots/sound_editor.gif">

**Editor de Musica:**

O modo para editar músicas nas quais os sons são organizados na ordem de execução.

<img src="pyxel/editor/screenshots/music_editor.gif">

### Outros métodos de criação de recursos

Imagens Pyxel e tilemaps também podem ser criadas da seguinte forma:

- Criar uma imagem a partir de uma lista de strings com a função `Image.set` ou `Tilemap.set`
- Carregar um arquivo png na paleta do Pyxel com a função `Image.load`

Sons Pyxel também podem ser criados da seguinte maneira:

- Criar um som a partir de strings com a função `Sound.set` ou `Music.set`

Favor consultar a referência da API para o uso dessas funções.

### Como criar um Executável Autônomo

Usando o Empacotador Pyxel embutido é possível criar um executável autônomo que irá funcionar até em ambientes em que não tenham o Python instalado.

Para criar um executável independente, no ambiente em que [PyInstaller](https://www.pyinstaller.org/) está instalado, especifique o arquivo Python a ser usado para iniciar o aplicativo com o comando `pyxelpackager` da seguinte maneira:

```sh
pyxelpackager python_file
```

Quando o processo estiver completo, um executável será criado na pasta `dist`.

Se recursos como os arquivos .pyxres e .png também forem necessários, coloque os dentro da pasta `assets` que eles também serão inclusos.

Também é possível especificar um icone com a opção ``-i icon_file``.

## Referência da API

### Sistema

- `width`, `height`<br>
A largura e a altura da tela

- `frame_count`<br>
O número dos quadros decorridos

- `init(width, height, [caption], [scale], [palette], [fps], [quit_key], [fullscreen])`<br>
Inicializa a aplicação Pyxel com o tamanho de tela (`width`, `height`). A largura e a altura máxima da tela é 256<br>
Também é possível especificar o título da janela com `caption`, a ampliação da tela com `scale`, a paleta de cores com `palette`, a taxa de quadros com `fps`, a tecla para finalizar a aplicação com `quit_key` e se ela será iniciada em modo tela cheia com `fullscreen`. `palette` é especificada como uma lista de 16 elementos de cor de 24 bits.<br>
e.g. `pyxel.init(160, 120, caption="Pyxel with PICO-8 palette", palette=[0x000000, 0x1D2B53, 0x7E2553, 0x008751, 0xAB5236, 0x5F574F, 0xC2C3C7, 0xFFF1E8, 0xFF004D, 0xFFA300, 0xFFEC27, 0x00E436, 0x29ADFF, 0x83769C, 0xFF77A8, 0xFFCCAA], quit_key=pyxel.KEY_NONE, fullscreen=True)`

- `run(update, draw)`<br>
Inicia a aplicação Pyxel e chama a função `update` para atualização de quadros e a função `draw` para desenhar

- `quit()`<br>
Encerra a aplicação Pyxel no fim do quadro atual

- `flip()`<br>
Força o desenho na tela (não use em aplicações normais)

- `show()`<br>
Desenha na tela e espera para sempre (não use em aplicações normais)

### Recurso

- `save(filename)`<br>
Salva o arquivo de recurso (.pyxres) no diretório do script de execução

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Lê o arquivo de recurso (.pyxres) do diretório do script de execução. Se ``False`` for especificado para o tipo de recurso (imagem/tilemap/som/musica), o recurso não será carregado.

### Entrada
- `mouse_x`, `mouse_y`<br>
A posição atual do cursor do mouse

- `mouse_wheel`<br>
O valor atual da roda de rolagem do mouse

- `btn(key)`<br>
Retorna `True` se `key` é pressionada, caso contrário retorna `False` ([lista de definições de teclas](pyxel/__init__.py))

- `btnp(key, [hold], [period])`<br>
Retorna `True` se `key` for pressionada naquele quadro, caso contrário retorna `False`. Quando `hold` e `period` são especificados, `True` será retornado durante o intervalo de quadros `period`, no qual `key` estiver pressionada por mais que `hold` quadros

- `btnr(key)`<br>
Retorna `True` se `key` for solta naquele quadro, caso contrário retorna `False`

- `mouse(visible)`<br>
Se `visible` for `True`, mostra o cursor do mouse. Se for `False`, esconde. Mesmo se o cursor do mouse não for visível, sua posição é atualizada.

### Gráficos

- `image(img, [system])`<br>
Opera o banco de imagens `img`(0-2) (veja a classe de Imagem). Se `system` for `True`, o banco de imagens do sistema pode ser acessado. 3 é para a fonte e o editor de recursos. 4 é para tela<br>
e.g. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
Opera o tilemap `tm`(0-7) (ver a classe de Tilemap)

- `clip(x, y, w, h)`<br>
Define a área de desenho da tela de (`x`, `y`) para a largura `w` e altura `h`. Redefina a área de desenho para tela cheia com `clip()`

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

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
Desenha um triangulo com os vértices (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e cor `col`

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
Desenha o contorno de um triangulo com os vértices (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e cor `col`

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
Copia a região de tamanho (`w`, `h`) de (`u`, `v`) do banco de imagens `img`(0-2) para (`x`, `y`). Se um valor negativo for definido para `w` e/ou `h`, será invertido horizontalmente e/ou verticalmente. Se `colkey` for especificada, será tratado como cor transparente

<img src="images/image_bank_mechanism.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Desenha o tilemap `tm`(0-7) em (`x`, `y`) de acordo com a informação de tile de tamanho (`w`, `h`) da posição (`u`, `v`). Se `colkey` for especificada, será tratada como cor transparente. Um tile do tilemap será desenhado com tamanho 8x8, e se o número do tile for 0, indica a região (0, 0)-(7, 7) do banco de imagens, se for 1, indica (8, 0)-(15, 0)

<img src="images/tilemap_mechanism.png">

- `text(x, y, s, col)`<br>
Desenha uma string `s` de cor `col` em (`x`, `y`)

### Áudio

- `sound(snd, [system])`<br>
Opera o som `snd`(0-63) (ver a classe de Som). Se `system` é `True`, o som 64 para o sistema, pode ser acessado<br>
e.g. `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Opera a música `msc` (0-7) (ver a classe de Musica)

- `play_pos(ch)`<br>
Recupera a posição de reprodução de som do canal `ch`. As centenas e os milhares indicam o numero do som e as unidades e dezenas indicam o numero da nota. Quando a reprodução termina, retorna `-1`

- `play(ch, snd, loop=False)`<br>
Reproduz o som `snd`(0-63) no canal `ch`(0-3). Tocar em ordem quando `snd` for uma lista

- `playm(msc, loop=False)`<br>
Reproduz a música `msc`(0-7)

- `stop([ch])`<br>
Interrompe a reprodução em todos os canais. Se `ch`(0-3) for especificado, somente este será interrompido

### Classe de Imagem

- `width`, `height`<br>
Largura e altura da imagem

- `data`<br>
Os dados da imagem (lista bidimensional de 256x256)

- `get(x, y)`<br>
Pega os dados da imagem em (`x`, `y`)

- `set(x, y, data)`<br>
Define os dados da imagem em (`x`, `y`) com um valor ou uma lista de strings<br>
e.g. `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Lê a imagem png do diretório do script de execução em (`x`, `y`)

- `copy(x, y, img, u, v, w, h)`<br>
Copia a região do tamanho (`w`, `h`) na posição (`u`, `v`) do banco de imagens `img`(0-2) para (`x`, `y`)

### Classe de Tilemap

- `width`, `height`<br>
A largura e a altura do tilemap

- `data`<br>
Os dados do tilemap (lista bidimensional de 256x256)

- `refimg`<br>
O banco de imagens referenciado pelo tilemap

- `get(x, y)`<br>
Pega os dados do tilemap em (`x`, `y`)

- `set(x, y, data)`<br>
Define os dados do tilemap em (`x`, `y`) com um valor ou uma lista de strings.<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `copy(x, y, tm, u, v, w, h)`<br>
Copia a região de tamanho (`w`, `h`) da posição (`u`, `v`) da tilemap `tm`(0-7) para (`x`, `y`)

### Classe de Som

- `note`<br>
Lista de notas(0-127) (33 = 'A2' = 440Hz)

- `tone`<br>
Lista de tons(0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volume`<br>
Lista de volume(0-7)

- `effect`<br>
Lista de efeitos(0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`<br>
Duração de uma nota(120 = 1 segundo por tom)

- `set(note, tone, volume, effect, speed)`<br>
Define uma nota, tom, volume e efeito com uma string. Se a duração do tom, volume e duração do efeito forem mais curtas que a nota, será repetido do começo

- `set_note(note)`<br>
Define a nota com uma string consistindo de 'CDEFGAB'+'#-'+'0123' ou 'R'. Não diferencia maiúsculas e minúsculas e espaços são ignorados<br>
e.g. `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tone(tone)`<br>
Define um tom com uma string consistindo de 'TSPN'. Não diferencia maiúsculas e minúsculas e espaços são ignorados<br>
e.g. `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volume(volume)`<br>
Define o volume com uma string consistindo de '01234567'. Não diferencia maiúsculas e minúsculas e espaços são ignorados<br>
e.g. `pyxel.sound(0).set_volume("7777 7531")`

- `set_effect(effect)`<br>
Define o efeito com uma string consistindo de 'NSVF'. Não diferencia maiúsculas e minúsculas e espaços são ignorados<br>
e.g. `pyxel.sound(0).set_effect("NFNF NVVS")`

### Classe de Musica

- `ch0`<br>
Lista de som(0-63) a tocar no canal 0. Se uma lista vazia for definida, o canal não será usado para reprodução

- `ch1`<br>
Lista de som(0-63) a tocar no canal 1. Se uma lista vazia for definida, o canal não será usado para reprodução

- `ch2`<br>
Lista de som(0-63) a tocar no canal 2. Se uma lista vazia for definida, o canal não será usado para reprodução

- `ch3`<br>
Lista de som(0-63) a tocar no canal 3. Se uma lista vazia for definida, o canal não será usado para reprodução

- `set(ch0, ch1, ch2, ch3)`<br>
Define a lista de som(0-63) de todos os canais. Se uma lista vazia for definida, o canal não será usado para reprodução<br>
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

- `set_ch0(data)`<br>
Define a lista de som(0-63) do canal 0

- `set_ch1(data)`<br>
Define a lista de som(0-63) do canal 1

- `set_ch2(data)`<br>
Define a lista de som(0-63) do canal 2

- `set_ch3(data)`<br>
Define a lista de som(0-63) do canal 3

## Como Contribuir

### Reportando um problema

Use o [issue tracker](https://github.com/kitao/pyxel/issues) para reportar bugs e enviar pedidos feature/aprimoramento.
Antes de submeter uma nova issue, procure no issue tracker para ter certeza de que não há algo similar em aberto.

Quando for enviar, selecione o template apropriado [neste link](https://github.com/kitao/pyxel/issues/new/choose).

### Testes manuais

Qualquer pessoa é bem vinda a testar manualmente o código e reportar bugs ou enviar sugestões de aprimoramento no issue tracker!

### Enviando um pull request

Patches/correções serão aceitas na forma de pull requests (PRs). Tenha certeza de que o que o pull request tenta resolver esteja em aberto no issue tracker.

Será considerado que todo pull request tenha concordado a ser publicado sob a [licença MIT](LICENSE).

## Outras informações

- [Wiki](https://github.com/kitao/pyxel/wiki)
- [Subreddit](https://www.reddit.com/r/pyxel/)
- [Discord server (English)](https://discord.gg/FC7kUZJ)
- [Discord server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## Licença

Pyxel está sob [MIT license](https://pt.wikipedia.org/wiki/Licen%C3%A7a_MIT). Pode ser reutilizado dentro de um software proprietário desde que todas as cópias do software licenciado incluam uma cópia dos termos da licença MIT e o aviso dos direitos autorais.

Pyxel usa o seguinte software:

- [SDL2](https://www.libsdl.org/)
- [miniz-cpp](https://github.com/tfussell/miniz-cpp)
- [Gifsicle](https://www.lcdf.org/gifsicle/)
