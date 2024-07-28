# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** é um motor de jogos retrô para Python.

Graças às suas especificações simples inspiradas em consoles de jogos retrô, assim como permitir a exibição de apenas 16 cores e apenas 4 sons poderem ser reproduzidos ao mesmo tempo, você pode se sentir à vontade para fazer jogos em estilo pixel art.

<img src="images/pyxel_message.png" width="480">

A motivação para o desenvolvimento do Pyxel é o feedback dos utilizadores. Por favor, dê a Pyxel uma estrela em GitHub!

<p>
<a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">
<img src="images/01_hello_pyxel.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">
<img src="images/02_jump_game.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">
<img src="images/03_draw_api.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">
<img src="images/04_sound_api.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_music_editor.gif" width="320">
</a>
</p>

As especificações e APIs de Pyxel são inspiradas por [PICO-8](https://www.lexaloffle.com/pico-8.php) e [TIC-80](https://tic80.com/).

Pyxel é open source e livre para utilização. Vamos começar fazendo um jogo retrô com Pyxel!

## Especificações

- Funciona em Windows, Mac, Linux, e Web
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

### Windows

Após instalar o [Python3](https://www.python.org/) (versão 3.7 ou superior), execute o seguinte comando:

```sh
pip install -U pyxel
```

Se instalar Python utilizando o instalador oficial, verifique a caixa de verificação `Add Python 3.x to PATH` para activar o comando `pyxel`.

### Mac

Depois de instalar o [Homebrew](https://brew.sh/), execute os seguintes comandos:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Para atualizar a versão após a instalação do Pyxel, execute `pipx upgrade pyxel`.

### Linux

Após instalar o pacote SDL2 (`libsdl2-dev` no Ubuntu), [Python3](https://www.python.org/) (versão 3.7 ou superior), e `python3-pip`, execute o seguinte comando:

```sh
sudo pip3 install -U pyxel
```

Se o acima mencionado não funcionar, tente auto-construir-se de acordo com as instruções em [Makefile](../Makefile).

### Web

A versão web do Pyxel não requer instalação de Python ou Pyxel e corre em PCs, bem como smartphones e tablets com navegadores web suportados.

Para instruções específicas, por favor consulte [esta página](https://github.com/kitao/pyxel/wiki/How-To-Use-Pyxel-Web).

### Testando os exemplos Pyxel

Após instalar o Pyxel, os exemplos serão copiados para o diretório atual com o seguinte comando:

```sh
pyxel copy_examples
```

Os exemplos copiados são os seguintes:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>Aplicação simples</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Code</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Jogo de pulo com o arquivo de recursos do Pyxel</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Code</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Demonstração das APIs de desenho</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Code</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Demonstração das APIs de som</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Code</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>Lista da paleta de cores</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Code</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>Jogo de clique com mouse</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Code</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>Jogo Snake com BGM</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Code</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Demonstração da API de desenho de triângulos</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Code</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Jogo de tiro com transição de tela</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Code</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>Jogo side-scrolling de plataforma com mapa</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Code</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Renderização fora do ecrã com classe de Image</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Code</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>Animação sonora Perlin</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Code</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>Desenho de uma fonte bitmap</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Code</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>Sintetizador que utiliza funcionalidades de expansão de áudio</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Code</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>Carregar e desenhar um ficheiro de mapa de azulejos (.tmx)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Code</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>Animação com função flip (apenas plataformas não-web)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Code</a></td>
</tr>
<tr>
<td>30SecondsOfDaylight.pyxapp</td>
<td>1º jogo vencedor de Pyxel Jam de <a href="https://twitter.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30SecondsOfDaylight.html">Demo</a></td>
<td><a href="https://github.com/kitao/30SecondsOfDaylight">Code</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>Jogo de física de bola arcade por <a href="https://twitter.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Demo</a></td>
<td><a href="https://github.com/helpcomputer/megaball">Code</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>Gerador de música de fundo feito por <a href="https://twitter.com/frenchbread1222">frenchbread</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">Demo</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">Code</a></td>
</tr>
</table>

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

Ao criar gráficos simples sem animação, a função `show` pode ser utilizada para tornar o código mais conciso.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Executando uma aplicação Pyxel

O script Python criado pode ser executado com o seguinte comando:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Também pode ser executado como um guião Python normal:

```sh
python3 PYTHON_SCRIPT_FILE
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
  Salva um vídeo de captura de tela na área de trabalho (até 10 segundos)
- `Alt(Option)+9`<br>
  Alternar entre os modos de ecrã (Crisp/Smooth/Retro)
- `Alt(Option)+0`<br>
  Ativa/desativa o monitor de performance (fps, tempo de update e tempo de draw)
- `Alt(Option)+Enter`<br>
  Ativa/desativa tela cheia
- `Shift+Alt(Option)+1/2/3`<br>
  Guardar o banco de imagens correspondente no ambiente de trabalho
- `Shift+Alt(Option)+0`<br>
  Guardar a paleta de cores atual no ambiente de trabalho

### Como criar um Recurso

O Editor Pyxel pode criar imagens e sons usados em uma aplicação Pyxel.

Ele é inicializado com o seguinte comando:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Se o arquivo de recursos Pyxel (.pyxres) existir, o arquivo será carregado, e se ele não existir, um novo arquivo com o nome especificado será criado. Se o arquivo de recursos for omitido, o nome será `my_resource.pyxres`.

Após iniciar o Editor Pyxel, o arquivo pode ser trocado arrastando e soltando outro arquivo de recursos.

O arquivo recursos criado pode ser carregado através da função `load`.

O Editor Pyxel possuí os seguintes modos de edição.

**Editor de Imagem**

O modo para editar bancos de imagem.

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

Arrastar e largar um ficheiro de imagem (PNG/GIF/JPEG) no Editor de imagens para carregar a imagem no banco de imagens atualmente selecionado.

**Editor de Tilemap**

O modo para editar tilemaps em que imagens dos bancos de imagens são organizados em um padrão de tiles.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

Arraste e largue um ficheiro TMX (Tiled Map File) no Tilemap Editor para carregar a sua camada na ordem de desenho que corresponde ao número de tilemap atualmente selecionado.

**Editor de Som**

O modo para editar sons.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**Editor de Música**

O modo para editar músicas nas quais os sons são organizados na ordem de execução.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### Outros métodos de criação de recursos

Imagens e tilemaps Pyxel também podem ser criados pelos seguintes métodos:

- Criar uma imagem de uma lista de strings com a função `Image.set` ou com a função `Tilemap.set`
- Carregar um arquivo de imagem (PNG/GIF/JPEG) na paleta Pyxel com a função `Image.load`

Sons Pyxel também podem ser criados com o seguinte método:

- Criar um som de uma strings com a função `Sound.set` ou com a função `Music.set`

Favor consultar a referência da API para o uso dessas funções.

### Como distribuir uma aplicação

O Pyxel suporta um formato de arquivo de distribuição dedicado (arquivo de aplicação Pyxel) que é multiplataforma.

Crie um arquivo de aplicação Pyxel (.pyxapp) com o seguinte comando:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Se a candidatura deve incluir recursos ou módulos adicionais, coloque-os no directório da candidatura.

O arquivo de aplicação pode ser executado com o seguinte comando:

```sh
pyxel play PYXEL_APP_FILE
```

O ficheiro de aplicação Pyxel também pode ser convertido num ficheiro executável ou HTML com os comandos `pyxel app2exe` ou `pyxel app2html`.

## Referência da API

### Sistema

- `width`, `height`<br>
  A largura e a altura da tela

- `frame_count`<br>
  O número dos quadros decorridos

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Inicializa a aplicação Pyxel com tamanho de tela (`width`, `height`). As seguintes opções podem ser especificadas: o título da janela com `title`, a taxa de quadros com `fps`, a tecla para fechar a aplicação com `quit_key`, a escala da exposição com `display_scale`, a escala da captura de tela com `capture_scale`, o tempo máximo de gravação do vídeo da captura de tela `capture_sec`.<br>
  Ex. `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Roda a aplicação Pyxel e chama a função `update` para atualizar os quadros e a função `draw` para desenhá-los.

- `show()`<br>
  Mostra a tela e espera até a tecla `Esc` ser pressionada.

- `flip()`<br>
  Refrear o ecrã por uma moldura. A aplicação sai quando a tecla `Esc` é premida. Esta função não funciona na versão Web.

- `quit()`<br>
  Feche a aplicação Pyxel.

### Recurso

- `load(filename, [excl_images], [excl_tilemaps], [excl_sounds], [excl_musics])`<br>
  Carrega o arquivo de recursos (.pyxres). Se uma opção for `True`, o recurso não será carregado. Se um arquivo de paleta (.pyxpal) com o mesmo nome existir no mesmo local que o arquivo de recurso, a cor de exibição da paleta também será alterada. O arquivo de paleta é uma entrada hexadecimal das cores de exibição (por exemplo, `1100FF`), separada por novas linhas. O arquivo de paleta também pode ser usado para alterar as cores exibidas no Pyxel Editor.

### Entrada

- `mouse_x`, `mouse_y`<br>
  A posição atual do cursor do mouse

- `mouse_wheel`<br>
  O valor atual da roda de rolagem do mouse

- `btn(key)`<br>
  Retorna `True` se `key` é pressionada, caso contrário retorna `False`. ([lista de definições de teclas](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  Retorna `True` se `key` for pressionada naquele quadro, caso contrário retorna `False`. Quando `hold` e `repeat` são especificados, `True` será retornado durante o intervalo de quadros `repeat`, no qual `key` estiver pressionada por mais que `hold` quadros.

- `btnr(key)`<br>
  Retorna `True` se `key` for solta naquele quadro, caso contrário retorna `False`

- `mouse(visible)`<br>
  Se `visible` for `True`, mostra o cursor do mouse. Se for `False`, esconde. Mesmo se o cursor do mouse não for visível, sua posição é atualizada.

### Gráficos

- `colors`<br>
  Lista da paleta de cores da tela. A cor da tela é especificada por um valor numérico de 24 bits. Use `colors.from_list` e `colors.to_list` para atribuir e pegar listas do Python.<br>
  Ex. `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Lista dos bancos de imagens (0-2). (veja a classe de Imagem)<br>
  Ex. `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Lista dos mapas de azulejos (0-7). (ver a classe de Tilemap)

- `clip(x, y, w, h)`<br>
  Define a área de desenho da tela de (`x`, `y`) para a largura `w` e altura `h`. Redefina a área de desenho para tela cheia com `clip()`

- `camera(x, y)`<br>
  Altera as coordenadas do canto superior esquerdo da tela para (`x`, `y`). Redefina as coordenadas do canto superior esquerdo para (`0`, `0`) com `camera()`.

- `pal(col1, col2)`<br>
  Substitui a cor `col1` com `col2` ao desenhar. Use `pal()` para voltar para a paleta inicial.

- `dither(alpha)`<br>
  Aplica dithering (pseudo-transparência) no desenho. Define `alpha` no intervalo 0.0-1.0, onde 0.0 é transparente e 1.0 é opaco.

- `cls(col)`<br>
  Limpar a tela com a cor `col`

- `pget(x, y)`<br>
  Captura a cor de um pixel em (`x`, `y`).

- `pset(x, y, col)`<br>
  Desenha um pixel de cor `col` em (`x`, `y`).

- `line(x1, y1, x2, y2, col)`<br>
  Desenha uma linha da cor `col` de (`x1`, `y1`) até (`x2`, `y2`).

- `rect(x, y, w, h, col)`<br>
  Desenha um retângulo de largura `w`, altura `h` e cor `col` a partir de (`x`, `y`).

- `rectb(x, y, w, h, col)`<br>
  Desenha o contorno de um retângulo de largura `w`, altura `h` e cor `col` a partir de (`x`, `y`).

- `circ(x, y, r, col)`<br>
  Desenha um círculo de raio `r` e cor `col` em (`x`, `y`).

- `circb(x, y, r, col)`<br>
  Desenha o contorno de um círculo de raio `r` e cor `col` em (`x`, `y`).

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
  Copia a região de tamanho (`w`, `h`) de (`u`, `v`) do banco de imagens `img`(0-2) para (`x`, `y`). Se um valor negativo for definido para `w` e/ou `h`, será invertido horizontalmente e/ou verticalmente. Se `colkey` for especificada, será tratado como cor transparente.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
  Copia a região de tamanho (`w`, `h`) de (`u`, `v`) do tilemap `tm`(0-7) para (`x`, `y`). Se um valor negativo for definido para `w` e/ou `h`, será invertido horizontalmente e/ou verticalmente. Se `colkey` for especificada, será tratado como cor transparente. O tamanho de um tile é de 8x8 pixels e é armazenado em um tilemap como uma tupla de `(tile_x, tile_y)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  Desenha uma string `s` de cor `col` em (`x`, `y`).

### Áudio

- `sounds`<br>
  Lista dos sons (0-63). (ver a classe de Som)<br>
  Ex. `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Lista das músicas (0-7). (ver a classe de Musica)

- `play(ch, snd, [tick], [loop], [resume])`<br>
  Reproduz o som `snd`(0-63) no canal `ch`(0-3). Se `snd` é uma lista, os sons serão reproduzidos em ordem. A posição inicial da reprodução pode ser especificada por `tick`(1 tick = 1/120 segundos). Se `True` for especificado para `loop`, a reprodução será feita em laço. Para retomar o som anterior após o término da reprodução, defina `resume` como `True`.

- `playm(msc, [tick], [loop])`<br>
  Reproduz a música `msc`(0-7). A posição inicial da reprodução pode ser especificada por `tick`(1 tick = 1/120 segundos). Se `True` for especificado para `loop`, a reprodução será feita em laço.

- `stop([ch])`<br>
  Para a reprodução do canal `ch`(0-3). `stop()` para parar a reprodução de todos os canais.

- `play_pos(ch)`<br>
  Obtém a posição do canal `ch`(0-3) da reprodução de som como uma tupla de `(sound no, note no)`. Retorna `None` quando a reprodução para.

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

- `rseed(seed)`<br>
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
  Ex. `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Carrega um arquivo de imagem (PNG/GIF/JPEG) em (`x`, `y`).

- `pget(x, y)`<br>
  Pega a cor do pixel em (`x`, `y`).

- `pset(x, y, col)`<br>
  Desenha um pixel de cor `col` em (`x`, `y`).

### Classe de Tilemap

- `width`, `height`<br>
  A largura e a altura do tilemap

- `imgsrc`<br>
  O banco de imagem (0-2) referenciado pelo tilemap

- `set(x, y, data)`<br>
  Define o tilemap em (`x`, `y`) por uma lista de strings.<br>
  Ex. `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  Carregar a camada na ordem de desenho `layer`(0-) a partir do ficheiro TMX (Tiled Map File) em (`x`, `y`).

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
  Lista de efeitos (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  Velocidade de reprodução. 1 é a velocidade maior, e quanto maior o número, mais lenta ela é. No valor 120, o tempo de uma nota se torna 1 segundo.

- `set(notes, tones, volumes, effects, speed)`<br>
  Define as notas, tons, volumes e efeitos com uma string. Se os tons, volumes e efeitos são mais curtos que as notas, elas se repetirão do começo.

- `set_notes(notes)`<br>
  Define as notas com uma string 'CDEFGAB'+'#-'+'01234' ou 'R'. É insensível à maiúsculas ou minúsculas e espaços em branco são ignorados.<br>
  Ex. `pyxel.sounds[0].set_notes("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
  Define os tons com uma string composta por 'TSPN'. É insensível à maiúsculas ou minúsculas e espaços em branco são ignorados.<br>
  Ex. `pyxel.sounds[0].set_tones("TTSS PPPN")`

- `set_volumes(volumes)`<br>
  Define os volumes com uma string composta por '01234567'. É insensível à maiúsculas ou minúsculas e espaços em branco são ignorados.<br>
  Ex. `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  Define os efeitos com uma string composta por 'NSVFHQ'. É insensível à maiúsculas ou minúsculas e espaços em branco são ignorados.<br>
  Ex. `pyxel.sounds[0].set_effects("NFNF NVVS")`

### Classe de Musica

- `seqs`<br>
  Lista bi-dimensional de sons (0-63) com o número de canais

- `set(seq0, seq1, seq2, ...)`<br>
  Define as listas de sons (0-63) para todos os canais. Se uma lista vazia for especificada, aquele canal não será utilizado para reprodução de sons.<br>
  Ex. `pyxel.musics[0].set([0, 1], [], [3])`

### APIs Avançadas

Pyxel possui "APIs avançadas" que não são mencionadas nesse manual de referência pois elas podem "confundir usuários" ou "precisam de conhecimento especializado para usar".

Se você está familiarizado com suas habilidades, tente criar projetos incríveis utilizando [isto](../python/pyxel/__init__.pyi) como pista!

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
- [Developer's Twitter account](https://twitter.com/kitao)

## Licença

Pyxel está sob a [licença MIT](../LICENSE). Ele pode ser reutilizado em software proprietário, contanto que todas as cópias do software ou sua porções substanciais incluam uma cópia dos termos da licença MIT e um aviso de direitos autorais.

## Recrutando Patrocinadores

Pyxel está procurando patrocinadores nos patrocinadores do GitHub. Considere patrocinar o Pyxel para manutenção contínua e acréscimos de recursos. Os patrocinadores podem consultar sobre o Pyxel como um benefício. Por favor, veja [aqui](https://github.com/sponsors/kitao) para detalhes.
