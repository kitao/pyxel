# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** (/ˈpɪksəl/) é um motor de jogos retro para Python.

As especificações são inspiradas em consoles de jogos retro, como o suporte para apenas 16 cores e 4 canais de som, permitindo que você desfrute facilmente da criação de jogos com estilo de arte em pixel.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

O desenvolvimento do Pyxel é impulsionado pelo feedback dos usuários. Por favor, dê uma estrela ao Pyxel no GitHub!

<p>
<a href="https://kitao.github.io/pyxel/wasm/examples/10-platformer.html">
<img src="images/10_platformer.gif" width="290">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/30sec-of-daylight.html">
<img src="images/30sec_of_daylight.gif" width="350">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/02-jump-game.html">
<img src="images/02_jump_game.gif" width="330">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">
<img src="images/megaball.gif" width="310">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/image-editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/sound-editor.html">
<img src="images/sound_music_editor.gif" width="320">
</a>
</p>

As especificações e APIs do Pyxel são inspiradas no [PICO-8](https://www.lexaloffle.com/pico-8.php) e no [TIC-80](https://tic80.com/).

O Pyxel é de código aberto sob a [Licença MIT](../LICENSE) e é gratuito para usar. Vamos começar a criar jogos retrô com o Pyxel!

## Especificações

- Funciona no Windows, Mac, Linux e Web
- Programação em Python
- Tamanho de tela personalizável
- Paleta de 16 cores
- 3 bancos de imagem de 256x256
- 8 mapas de blocos de 256x256
- 4 canais com 64 sons definíveis
- 8 faixas de música que podem combinar quaisquer sons
- Entradas de teclado, mouse e gamepad
- Ferramentas de edição de imagens e sons
- Cores, canais e bancos extensíveis pelo usuário

### Paleta de cores

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Como instalar

### Windows

Após instalar o [Python3](https://www.python.org/) (versão 3.8 ou superior), execute o seguinte comando:

```sh
pip install -U pyxel
```

Ao instalar o Python usando o instalador oficial, certifique-se de marcar a opção `Add Python 3.x to PATH` para habilitar o comando `pyxel`.

### Mac

Após instalar o [Homebrew](https://brew.sh/), execute os seguintes comandos:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Para atualizar o Pyxel após a instalação, execute `pipx upgrade pyxel`.

### Linux

Após instalar o pacote SDL2 (`libsdl2-dev` para Ubuntu), [Python3](https://www.python.org/) (versão 3.8 ou superior) e `python3-pip`, execute o seguinte comando:

```sh
sudo pip3 install -U pyxel
```

Se o comando anterior falhar, considere construir o Pyxel a partir do código-fonte seguindo as instruções no [Makefile](../Makefile).

### Web

A versão web do Pyxel não requer a instalação do Python ou do Pyxel e funciona em PCs, smartphones e tablets com navegadores web compatíveis.

Para instruções detalhadas, consulte [esta página](pyxel-web-en.md).

### Executar exemplos

Após instalar o Pyxel, você pode copiar os exemplos para o diretório atual com o seguinte comando:

```sh
pyxel copy_examples
```

Os seguintes exemplos serão copiados para o seu diretório atual:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>Aplicativo mais simples</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01-hello-pyxel.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Código</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Jogo de salto com arquivo de recursos Pyxel</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02-jump-game.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Código</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Demonstração das APIs de desenho</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03-draw-api.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Código</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Demonstração das APIs de som</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04-sound-api.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Código</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>Lista de paletas de cores</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05-color-palette.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Código</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>Jogo de clique do mouse</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06-click-game.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Código</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>Jogo da cobrinha com BGM</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07-snake.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Código</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Demonstração das APIs de desenho de triângulos</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08-triangle-api.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Código</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Shoot'em up com transições de tela e MML</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09-shooter.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Código</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>Jogo de plataforma em rolagem lateral com mapa</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10-platformer.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Código</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Renderização offscreen com a classe Image</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11-offscreen.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Código</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>Animação de ruído de Perlin</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12-perlin-noise.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Código</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>Desenho de uma fonte bitmap</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13-bitmap-font.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Código</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>Sintetizador utilizando recursos de expansão de áudio</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14-synthesizer.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Código</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>Carregando e desenhando um Tiled Map File (.tmx)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15-tiled-map-file.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Código</a></td>
</tr>
<tr>
<td>16_transform.py</td>
<td>Rotação e dimensionamento de imagens</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/16-transform.html">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/16_transform.py">Código</a></td>
</tr>
<tr>
<td>17_app_launcher.py</td>
<td>Launcher de aplicativos Pyxel (você pode jogar vários jogos!)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/17-app-launcher.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/17_app_launcher.py">Code</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>Animação com a função flip (apenas para plataformas não web)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Demonstração</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Código</a></td>
</tr>
</table>

Os exemplos podem ser executados com os seguintes comandos:

```sh
# Run example in examples directory
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# Run app in examples/apps directory
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

## Como usar

### Criar um aplicativo

No seu script Python, importe o módulo Pyxel, especifique o tamanho da janela com a função `init` e, em seguida, inicie o aplicativo Pyxel com a função `run`.

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

Os argumentos da função `run` são a função `update`, que processa as atualizações de quadro, e a função `draw`, que lida com a exibição na tela.

Em um aplicativo real, é recomendável encapsular o código Pyxel em uma classe, como mostrado abaixo:

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

Para criar gráficos simples sem animação, você pode usar a função `show` para simplificar seu código.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Executar o aplicativo

Um script criado pode ser executado usando o comando `python`:

```sh
python PYTHON_SCRIPT_FILE
```

Ele também pode ser executado com o comando `pyxel run`:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Além disso, o comando `pyxel watch` monitora alterações em um diretório especificado e executa automaticamente o programa quando mudanças são detectadas:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

A vigilância do diretório pode ser interrompida pressionando `Ctrl(Command)+C`.

### Operações de Teclas Especiais

Durante a execução de uma aplicação Pyxel, as seguintes operações de teclas especiais podem ser realizadas:

- `Esc`<br>
  Sair do aplicativo
- `Alt(Option)+R` ou `A+B+X+Y+BACK` no gamepad<br>
  Reiniciar o aplicativo
- `Alt(Option)+1`<br>
  Salvar a captura de tela na área de trabalho
- `Alt(Option)+2`<br>
  Reiniciar o tempo de início da gravação do vídeo de captura de tela
- `Alt(Option)+3`<br>
  Salvar um vídeo de captura de tela na área de trabalho (até 10 segundos)
- `Alt(Option)+8` ou `A+B+X+Y+DL` no gamepad<br>
  Alterna o modo de escala da tela entre máximo e inteiro
- `Alt(Option)+9` ou `A+B+X+Y+DR` no gamepad<br>
  Alternar entre os modos de tela (Crisp/Smooth/Retro)
- `Alt(Option)+0` ou `A+B+X+Y+DU` no gamepad<br>
  Alternar o monitor de desempenho (FPS/`update` tempo/`draw` tempo)
- `Alt(Option)+Enter` ou `A+B+X+Y+DD` no gamepad<br>
  Alternar para tela cheia
- `Shift+Alt(Option)+1/2/3`<br>
  Salvar o banco de imagens 0, 1 ou 2 na área de trabalho
- `Shift+Alt(Option)+0`<br>
  Salvar a paleta de cores atual na área de trabalho

### Como Criar Recursos

O Pyxel Editor pode criar imagens e sons usados em uma aplicação Pyxel.

Você pode iniciar o Pyxel Editor com o seguinte comando:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Se o arquivo de recurso Pyxel especificado (.pyxres) existir, ele será carregado. Se não existir, um novo arquivo com o nome especificado será criado. Se o arquivo de recurso for omitido, um novo arquivo chamado `my_resource.pyxres` será criado.

Após iniciar o Pyxel Editor, você pode alternar para outro arquivo de recurso arrastando e soltando-o no Pyxel Editor.

O arquivo de recurso criado pode ser carregado usando a função `load`.

O Pyxel Editor tem os seguintes modos de edição.

**Editor de Imagem**

O modo para editar a imagem em cada **banco de imagens**.

<a href="https://kitao.github.io/pyxel/wasm/examples/image-editor.html">
<img src="images/image_editor.gif">
</a>

Você pode arrastar e soltar um arquivo de imagem (PNG/GIF/JPEG) no editor de imagem para carregar a imagem no banco de imagens atualmente selecionado.

**Editor de Mapas de Ladrilhos**

O modo para editar os **mapas de ladrilhos** que organizam imagens dos bancos de imagens em um padrão de ladrilhos.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap-editor.html">
<img src="images/tilemap_editor.gif">
</a>

Arraste e solte um arquivo TMX (Tiled Map File) no editor de mapas de ladrilhos para carregar sua camada 0 no mapa de ladrilhos atualmente selecionado.

**Editor de Som**

O modo para editar os **sons** utilizados para melodias e efeitos sonoros.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound-editor.html">
<img src="images/sound_editor.gif">
</a>

**Editor de Música**

O modo para editar **músicas** em que os sons são organizados em ordem de reprodução.

<a href="https://kitao.github.io/pyxel/wasm/examples/music-editor.html">
<img src="images/music_editor.gif">
</a>

### Outros Métodos de Criação de Recursos

Imagens e mapas de ladrilhos do Pyxel também podem ser criados usando os seguintes métodos:

- Crie uma imagem a partir de uma lista de strings usando a função `Image.set` ou a função `Tilemap.set`
- Carregue um arquivo de imagem (PNG/GIF/JPEG) na paleta do Pyxel com a função `Image.load`

Os sons do Pyxel também podem ser criados usando o seguinte método:

- Crie um som a partir de strings com a função `Sound.set` ou a função `Music.set`

Consulte a referência da API para o uso dessas funções.

### Como Distribuir Aplicações

O Pyxel suporta um formato de arquivo de distribuição de aplicativo dedicado (arquivo de aplicativo Pyxel) que é multiplataforma.

Um arquivo de aplicativo Pyxel (.pyxapp) é criado usando o comando `pyxel package`:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Se você precisar incluir recursos ou módulos adicionais, coloque-os no diretório do aplicativo.

Os metadados podem ser exibidos em tempo de execução, especificando-os no seguinte formato dentro do script de inicialização. Os campos além de `title` e `author` são opcionais.

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

O arquivo de aplicativo criado pode ser executado usando o comando `pyxel play`:

```sh
pyxel play PYXEL_APP_FILE
```

Um arquivo de aplicativo Pyxel também pode ser convertido em um executável ou um arquivo HTML usando os comandos `pyxel app2exe` ou `pyxel app2html`.

## Referência da API

### Sistema

- `width`, `height`<br>
  A largura e altura da tela

- `frame_count`<br>
  O número de quadros transcorridos

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Inicializa a aplicação Pyxel com o tamanho da tela (`width`, `height`). As seguintes opções podem ser especificadas: o título da janela com `title`, a taxa de quadros com `fps`, a tecla para encerrar a aplicação com `quit_key`, a escala de exibição com `display_scale`, a escala de captura de tela com `capture_scale` e o tempo máximo de gravação do vídeo de captura de tela com `capture_sec`.<br>
  Exemplo: `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Inicia a aplicação Pyxel e chama a função `update` para atualização de quadros e a função `draw` para desenhar.

- `show()`<br>
  Mostra a tela e aguarda até que a tecla `Esc` seja pressionada.

- `flip()`<br>
  Atualiza a tela em um quadro. A aplicação encerra quando a tecla `Esc` é pressionada. Esta função não está disponível na versão web.

- `quit()`<br>
  Encerra a aplicação Pyxel.

- `reset()`<br>
  Reinicia a aplicação Pyxel. As variáveis de ambiente são mantidas após o reinício.

### Recurso

- `load(filename, [exclude_images], [exclude_tilemaps], [exclude_sounds], [exclude_musics])`<br>
  Carrega o arquivo de recursos (.pyxres). Se uma opção for definida como `True`, o recurso correspondente será excluído do carregamento. Se um arquivo de paleta (.pyxpal) com o mesmo nome existir no mesmo local do arquivo de recursos, as cores da paleta de exibição também serão atualizadas. O arquivo de paleta contém entradas hexadecimais para as cores de exibição (por exemplo, `1100ff`), separadas por novas linhas. O arquivo de paleta também pode ser usado para alterar as cores exibidas no Pyxel Editor.

- `user_data_dir(vendor_name, app_name)`<br>
  Retorna o diretório de dados do usuário criado com base em `vendor_name` e `app_name`. Se o diretório não existir, ele será criado automaticamente. Ele é usado para armazenar pontuações altas, progresso do jogo e dados semelhantes.<br>
  Exemplo: `print(pyxel.user_data_dir("Takashi Kitao", "Pyxel Shooter"))`

### Entrada

- `mouse_x`, `mouse_y`<br>
  A posição atual do cursor do mouse

- `mouse_wheel`<br>
  O valor atual da roda do mouse

- `btn(key)`<br>
  Retorna `True` se a tecla `key` estiver pressionada, caso contrário, retorna `False`. ([Lista de definições de teclas](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  Retorna `True` se a tecla `key` for pressionada naquele quadro, caso contrário, retorna `False`. Se `hold` e `repeat` forem especificados, depois que a tecla `key` for mantida pressionada por `hold` quadros ou mais, `True` será retornado a cada `repeat` quadros.

- `btnr(key)`<br>
  Retorna `True` se a tecla `key` for liberada naquele quadro, caso contrário, retorna `False`.

- `mouse(visible)`<br>
  Mostra o cursor do mouse se `visible` for `True` e o oculta se `visible` for `False`. A posição do cursor continua a ser atualizada mesmo quando ele está oculto.

### Gráficos

- `colors`<br>
  Lista de cores da paleta de exibição. A cor de exibição é especificada por um valor numérico de 24 bits. Use `colors.from_list` e `colors.to_list` para atribuir e recuperar diretamente listas Python.<br>
  Exemplo: `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Lista dos bancos de imagens (instâncias da classe Image) (0-2)<br>
  Exemplo: `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Lista dos mapas de ladrilhos (instâncias da classe Tilemap) (0-7)

- `clip(x, y, w, h)`<br>
  Define a área de desenho da tela de (`x`, `y`) com uma largura de `w` e uma altura de `h`. Chame `clip()` para redefinir a área de desenho para a tela inteira.

- `camera(x, y)`<br>
  Altera as coordenadas do canto superior esquerdo da tela para (`x`, `y`). Chame `camera()` para redefinir as coordenadas do canto superior esquerdo para (`0`, `0`).

- `pal(col1, col2)`<br>
  Substitui a cor `col1` por `col2` ao desenhar. Chame `pal()` para redefinir para a paleta inicial.

- `dither(alpha)`<br>
  Aplica dithering (pseudo-transparência) ao desenhar. Defina `alpha` na faixa de `0.0`-`1.0`, onde `0.0` é transparente e `1.0` é opaco.

- `cls(col)`<br>
  Limpa a tela com a cor `col`.

- `pget(x, y)`<br>
  Obtém a cor do pixel em (`x`, `y`).

- `pset(x, y, col)`<br>
  Desenha um pixel da cor `col` em (`x`, `y`).

- `line(x1, y1, x2, y2, col)`<br>
  Desenha uma linha da cor `col` de (`x1`, `y1`) a (`x2`, `y2`).

- `rect(x, y, w, h, col)`<br>
  Desenha um retângulo de largura `w`, altura `h` e cor `col` de (`x`, `y`).

- `rectb(x, y, w, h, col)`<br>
  Desenha o contorno de um retângulo de largura `w`, altura `h` e cor `col` de (`x`, `y`).

- `circ(x, y, r, col)`<br>
  Desenha um círculo de raio `r` e cor `col` em (`x`, `y`).

- `circb(x, y, r, col)`<br>
  Desenha o contorno de um círculo de raio `r` e cor `col` em (`x`, `y`).

- `elli(x, y, w, h, col)`<br>
  Desenha uma elipse de largura `w`, altura `h` e cor `col` de (`x`, `y`).

- `ellib(x, y, w, h, col)`<br>
  Desenha o contorno de uma elipse de largura `w`, altura `h` e cor `col` de (`x`, `y`).

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  Desenha um triângulo com vértices em (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e cor `col`.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  Desenha o contorno de um triângulo com vértices em (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) e cor `col`.

- `fill(x, y, col)`<br>
  Preenche a área conectada com a mesma cor de (`x`, `y`) com a cor `col`.

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Copia a região de tamanho (`w`, `h`) de (`u`, `v`) do banco de imagens `img`(0-2) para (`x`, `y`). Se um valor negativo for atribuído a `w` e/ou `h`, a região será invertida horizontalmente e/ou verticalmente. Se `colkey` for especificado, ele será tratado como uma cor transparente. Se `rotate`(em graus), `scale`(1.0 = 100%) ou ambos forem especificados, as transformações correspondentes serão aplicadas.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Copia a região de tamanho (`w`, `h`) de (`u`, `v`) do mapa de ladrilhos `tm`(0-7) para (`x`, `y`). Se um valor negativo for atribuído a `w` e/ou `h`, a região será invertida horizontalmente e/ou verticalmente. Se `colkey` for especificado, ele será tratado como uma cor transparente. Se `rotate`(em graus), `scale`(1.0 = 100%) ou ambos forem especificados, as transformações correspondentes serão aplicadas. O tamanho de um ladrilho é 8x8 pixels e é armazenado em um mapa de ladrilhos como uma tupla de `(image_tx, image_ty)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  Desenha uma string `s` com a cor `col` em (`x`, `y`).

### Áudio

- `sounds`<br>
  Lista dos sons (instâncias da classe Sound) (0-63)<br>
  Exemplo: `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Lista das músicas (instâncias da classe Music) (0-7)

- `play(ch, snd, [sec], [loop], [resume])`<br>
  Reproduz o som `snd`(0-63) no canal `ch`(0-3). `snd` pode ser um número de som, uma lista de números de som ou uma string MML. A posição inicial da reprodução pode ser especificada em segundos com `sec`. Se `loop` for definido como `True`, a reprodução será em loop. Para retomar o som anterior após o término da reprodução, defina `resume` como `True`.

- `playm(msc, [sec], [loop])`<br>
  Reproduz a música `msc`(0-7). A posição inicial da reprodução pode ser especificada em segundos com `sec`. Se `loop` for definido como `True`, a música será reproduzida em loop.

- `stop([ch])`<br>
  Interrompe a reprodução do canal especificado `ch`(0-3). Chame `stop()` para interromper todos os canais.

- `play_pos(ch)`<br>
  Obtém a posição de reprodução do som do canal `ch`(0-3) como uma tupla de `(sound_no, sec)`. Retorna `None` quando a reprodução for interrompida.

### Matemática

- `ceil(x)`<br>
  Retorna o menor número inteiro maior ou igual a `x`.

- `floor(x)`<br>
  Retorna o maior número inteiro menor ou igual a `x`.

- `sgn(x)`<br>
  Retorna `1` quando `x` é positivo, `0` quando é `0` e `-1` quando é negativo.

- `sqrt(x)`<br>
  Retorna a raiz quadrada de `x`.

- `sin(deg)`<br>
  Retorna o seno de `deg` graus.

- `cos(deg)`<br>
  Retorna o cosseno de `deg` graus.

- `atan2(y, x)`<br>
  Retorna a arctangente de `y`/`x` em graus.

- `rseed(seed)`<br>
  Define a semente do gerador de números aleatórios.

- `rndi(a, b)`<br>
  Retorna um número inteiro aleatório maior ou igual a `a` e menor ou igual a `b`.

- `rndf(a, b)`<br>
  Retorna um número flutuante aleatório maior ou igual a `a` e menor ou igual a `b`.

- `nseed(seed)`<br>
  Define a semente do ruído Perlin.

- `noise(x, [y], [z])`<br>
  Retorna o valor do ruído Perlin para as coordenadas especificadas.

### Classe Image

- `width`, `height`<br>
  A largura e altura da imagem

- `set(x, y, data)`<br>
  Define a imagem em (`x`, `y`) usando uma lista de strings.<br>
  Exemplo: `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Carrega um arquivo de imagem (PNG/GIF/JPEG) em (`x`, `y`).

- `pget(x, y)`<br>
  Obtém a cor do pixel em (`x`, `y`).

- `pset(x, y, col)`<br>
  Desenha um pixel com a cor `col` em (`x`, `y`).

### Classe Tilemap

- `width`, `height`<br>
  A largura e altura do mapa de ladrilhos

- `imgsrc`<br>
  O banco de imagens (0-2) referenciado pelo mapa de ladrilhos

- `set(x, y, data)`<br>
  Define o mapa de ladrilhos em (`x`, `y`) usando uma lista de strings.<br>
  Exemplo: `pyxel.tilemaps[0].set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  Carrega a `layer`(0-) do arquivo TMX (Tiled Map File) em (`x`, `y`).

- `pget(x, y)`<br>
  Obtém o ladrilho em (`x`, `y`). Um ladrilho é representado como uma tupla de `(image_tx, image_ty)`.

- `pset(x, y, tile)`<br>
  Desenha um `ladrilho` em (`x`, `y`). Um ladrilho é representado como uma tupla de `(image_tx, image_ty)`.

### Classe Sound

- `notes`<br>
  Lista de notas (0-127). Quanto maior o número, mais alta a nota. A nota `33` corresponde a 'A2'(440Hz). As pausas são representadas por `-1`.

- `tones`<br>
  Lista de tons (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
  Lista de volumes (0-7)

- `effects`<br>
  Lista de efeitos (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  Velocidade de reprodução. `1` é a mais rápida e, quanto maior o número, mais lenta a reprodução. Em `120`, a duração de uma nota é de 1 segundo.

- `set(notes, tones, volumes, effects, speed)`<br>
  Define notas, tons, volumes e efeitos usando uma string. Se o comprimento de tons, volumes ou efeitos for menor que o de notas, eles serão repetidos desde o início.

- `set_notes(notes)`<br>
  Define as notas usando uma string composta por `CDEFGAB`+`#-`+`01234` ou `R`. Não diferencia maiúsculas de minúsculas e ignora espaços em branco.<br>
  Exemplo: `pyxel.sounds[0].set_notes("g2b-2d3r rf3f3f3")`

- `set_tones(tones)`<br>
  Define os tons com uma string composta por `TSPN`. Não diferencia maiúsculas de minúsculas e ignora espaços em branco.<br>
  Exemplo: `pyxel.sounds[0].set_tones("ttss pppn")`

- `set_volumes(volumes)`<br>
  Define os volumes com uma string composta por `01234567`. Não diferencia maiúsculas de minúsculas e ignora espaços em branco.<br>
  Exemplo: `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  Define os efeitos com uma string composta por `NSVFHQ`. Não diferencia maiúsculas de minúsculas e ignora espaços em branco.<br>
  Exemplo: `pyxel.sounds[0].set_effects("nfnf nvvs")`

- `mml(code)`<br>
  Ao passar uma string [MML (Music Macro Language)](https://en.wikipedia.org/wiki/Music_Macro_Language), o modo MML é ativado e o som é reproduzido conforme seu conteúdo. Nesse modo, parâmetros normais como `notes` e `speed` são ignorados. Para sair do modo MML, chame `mml()` sem argumentos. Para mais detalhes sobre MML, consulte [esta página](faq-en.md).<br>
  Exemplo: `pyxel.sounds[0].mml("T120 Q90 @1 V100 O5 L8 C4&C<G16R16>C.<G16 >C.D16 @VIB1{10,20,20} E2C2")`

- `save(filename, sec, [ffmpeg])`<br>
  Cria um arquivo WAV que reproduz o som durante os segundos especificados. Se o FFmpeg estiver instalado e `ffmpeg` for definido como `True`, um arquivo MP4 também será criado.

- `total_sec()`<br>
  Retorna o tempo de reprodução do som em segundos. Retorna `None` se um loop infinito for usado no MML.

### Classe Music

- `seqs`<br>
  Uma lista bidimensional de sons (0-63) em vários canais

- `set(seq0, seq1, seq2, ...)`<br>
  Define as listas de sons (0-63) para cada canal. Se uma lista vazia for especificada, esse canal não será usado para reprodução.<br>
  Exemplo: `pyxel.musics[0].set([0, 1], [], [3])`

- `save(filename, sec, [ffmpeg])`<br>
  Cria um arquivo WAV que reproduz a música durante os segundos especificados. Se o FFmpeg estiver instalado e `ffmpeg` for definido como `True`, um arquivo MP4 também será criado.

### API Avançada

O Pyxel inclui uma "API Avançada" que não é mencionada nesta referência, pois pode confundir os usuários ou exigir conhecimento especializado para uso.

Se você tem confiança em suas habilidades, tente criar obras incríveis usando [isto](../python/pyxel/__init__.pyi) como guia!

## Como Contribuir

### Enviando Problemas

Use o [Rastreador de Problemas](https://github.com/kitao/pyxel/issues) para enviar relatórios de bugs e solicitações de recursos ou melhorias. Antes de enviar um novo problema, verifique se não há problemas semelhantes abertos.

### Testes Funcionais

Qualquer pessoa que teste manualmente o código e relate bugs ou sugestões de melhorias no [Rastreador de Problemas](https://github.com/kitao/pyxel/issues) é muito bem-vinda!

### Enviando Pull Requests

Patches e correções são aceitos na forma de pull requests (PRs). Certifique-se de que o problema que o pull request aborda está aberto no Rastreador de Problemas.

Enviar um pull request implica que você concorda em licenciar sua contribuição sob a [Licença MIT](../LICENSE).

## Ferramentas e Exemplos Web

- [Pyxel Web Examples](https://kitao.github.io/pyxel/wasm/examples/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/)
- [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/)

## Outras Informações

- [FAQ](faq-en.md)
- [Exemplos de Usuários](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Conta X do Desenvolvedor](https://x.com/kitao)
- [Servidor do Discord (Inglês)](https://discord.gg/Z87eYHN)
- [Servidor do Discord (Japonês)](https://discord.gg/qHA5BCS)

## Licença

O Pyxel é licenciado sob a [Licença MIT](../LICENSE). Pode ser reutilizado em software proprietário, desde que todas as cópias do software ou suas partes substanciais incluam uma cópia dos termos da Licença MIT e um aviso de copyright.

## Recrutamento de Patrocinadores

O Pyxel está buscando patrocinadores no GitHub Sponsors. Considere patrocinar o Pyxel para apoiar sua manutenção contínua e desenvolvimento de recursos. Como benefício, os patrocinadores podem consultar diretamente o desenvolvedor do Pyxel. Para mais detalhes, por favor, visite [esta página](https://github.com/sponsors/kitao).
