# <img src="images/pyxel_logo_152x64.png">

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

As especificações do Pyxel fazem referência aos incríveis [PICO-8](https://www.lexaloffle.com/pico-8.php) e [TIC-80](https://tic.computer/).

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
- [03_draw_api.py](../pyxel/examples/03_draw_api.py) - Demonstration of drawing APIs
- [04_sound_api.py](../pyxel/examples/04_sound_api.py) - Demonstration of sound APIs
- [05_color_palette.py](../pyxel/examples/05_color_palette.py) - Lista da paleta de cores
- [06_click_game.py](../pyxel/examples/06_click_game.py) - Jogo de clique com mouse
- [07_snake.py](../pyxel/examples/07_snake.py) - Jogo Snake com BGM
- [08_triangle_api.py](../pyxel/examples/08_triangle_api.py) - Demonstration of triangle drawing APIs
- [09_shooter.py](../pyxel/examples/09_shooter.py) - Jogo de tiro com transição de tela
- [10_platformer.py](../pyxel/examples/10_platformer.py) - Side-scrolling platform game with map

Os exemplos podem ser executados com os seguintes comandos:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
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

(For Windows, type `python` instead of `python3`)

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

**Editor de Musica:**

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

- `init(width, height, [title], [fps], [quit_key], [capture_sec])`<br>
Initialize the Pyxel application with screen size (`width`, `height`). The following can be specified as options: the window title with `title`, the frame rate with `fps`, the key to quit the application with `quit_key`, and the maximum recording time of the screen capture video with `capture_sec`.<br>
e.g. `pyxel.init(160, 120, title="Pyxel with Options", fps=60, quit_key=pyxel.KEY_NONE, capture_sec=0)`

- `run(update, draw)`<br>
Start the Pyxel application and call `update` function for frame update and `draw` function for drawing.

- `show()`<br>
Show the screen and wait until the `Esc` key is pressed. (Do not use in normal applications)

- `flip()`<br>
Updates the screen once. (Do not use in normal applications)

- `quit()`<br>
Quit the Pyxel application at the end of the current frame.

### Recurso

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Load the resource file (.pyxres). If ``False`` is specified for the resource type (``image/tilemap/sound/music``), the resource will not be loaded.

### Entrada
- `mouse_x`, `mouse_y`<br>
A posição atual do cursor do mouse

- `mouse_wheel`<br>
O valor atual da roda de rolagem do mouse

- `btn(key)`<br>
Retorna `True` se `key` é pressionada, caso contrário retorna `False` ([lista de definições de teclas](../pyxel/__init__.pyi))

- `btnp(key, [hold], [period])`<br>
Retorna `True` se `key` for pressionada naquele quadro, caso contrário retorna `False`. Quando `hold` e `period` são especificados, `True` será retornado durante o intervalo de quadros `period`, no qual `key` estiver pressionada por mais que `hold` quadros

- `btnr(key)`<br>
Retorna `True` se `key` for solta naquele quadro, caso contrário retorna `False`

- `mouse(visible)`<br>
Se `visible` for `True`, mostra o cursor do mouse. Se for `False`, esconde. Mesmo se o cursor do mouse não for visível, sua posição é atualizada.

### Gráficos

- `colors`<br>
List of the palette display colors. The display color is specified by a 24-bit numerical value. Use `colors.from_list` and `colors.to_list` to directly assign and retrieve Python lists.<br>
e.g. `org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img)`<br>
Operate the image bank `img` (0-2). (See the Image class)<br>
e.g. `pyxel.image(0).load(0, 0, "title.png")`

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
Draw the tilemap `tm` (0-7) to (`x`, `y`) according to the tile information of size (`w`, `h`) from (`u`, `v`). If `colkey` is specified, treated as transparent color. The size of a tile is 8x8 pixels and is stored in a tilemap as a tuple of `(x in tile, y in tile)`.

- `text(x, y, s, col)`<br>
Desenha uma string `s` de cor `col` em (`x`, `y`)

### Áudio

- `sound(snd)`<br>
Opera o som `snd`(0-63). (ver a classe de Som)<br>
e.g. `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Opera a música `msc` (0-7) (ver a classe de Musica)

- `play_pos(ch)`<br>
Get the sound playback position of channel `ch` (0-3) as a tuple of `(sound no, note no)`. Returns `None` when playback is stopped.

- `play(ch, snd, loop=False)`<br>
Play the sound `snd` (0-63) on channel `ch` (0-3). If `snd` is a list, it will be played in order. If `True` is specified for `loop`, loop playback is performed.

- `playm(msc, loop=False)`<br>
Play the music `msc` (0-7). If `True` is specified for `loop`, loop playback is performed.

- `stop([ch])`<br>
Stops playback of the specified channel `ch` (0-3). `stop()` to stop playing all channels.

### Classe de Imagem

- `width`, `height`<br>
Largura e altura da imagem

- `data`<br>
Os dados da imagem (lista bidimensional de 256x256)

- `get(x, y)`<br>
Pega os dados da imagem em (`x`, `y`)

- `set(x, y, data)`<br>
Set the image at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Load the image file (png/gif/jpeg) at (`x`, `y`).

### Classe de Tilemap

- `width`, `height`<br>
A largura e a altura do tilemap

- `refimg`<br>
The image bank (0-2) referenced by the tilemap

- `set(x, y, data)`<br>
Set the tilemap at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `pget(x, y)`<br>
Get the tile at (`x`, `y`). A tile is a tuple of `(x in tile, y in tile)`.

- `pset(x, y, tile)`<br>
Draw a `tile` at (`x`, `y`). A tile is a tuple of `(x in tile, y in tile)`.

### Classe de Som

- `notes`<br>
List of notes (0-127). The higher the number, the higher the pitch, and at 33 it becomes 'A2'(440Hz). The rest is -1.

- `tones`<br>
List of tones (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
List of volumes (0-7)

- `effects`<br>
List of effects (0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`<br>
Playback speed. 1 is the fastest, and the larger the number, the slower the playback speed. At 120, the length of one note becomes 1 second.

- `set(notes, tones, volumes, effects, speed)`<br>
Set notes, tones, volumes, and effects with a string. If the tones, volumes, and effects length are shorter than the notes, it is repeated from the beginning.

- `set_notes(notes)`<br>
Set the notes with a string made of 'CDEFGAB'+'#-'+'0123' or 'R'. Case-insensitive and whitespace is ignored.<br>
e.g. `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
Set the tones with a string made of 'TSPN'. Case-insensitive and whitespace is ignored.<br>
e.g. `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volumes(volumes)`<br>
Set the volumes with a string made of '01234567'. Case-insensitive and whitespace is ignored.<br>
e.g. `pyxel.sound(0).set_volume("7777 7531")`

- `set_effects(effects)`<br>
Set the effects with a string made of 'NSVF'. Case-insensitive and whitespace is ignored.<br>
e.g. `pyxel.sound(0).set_effect("NFNF NVVS")`

### Classe de Musica

- `sequences`<br>
Two-dimensional list of sounds (0-63) listed by the number of channels

- `set(seq0, seq1, seq2, seq3)`<br>
Set the lists of sound (0-63) of all channels. If an empty list is specified, that channel is not used for playback.<br>
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### Advanced APIs

Pyxel has "advanced APIs" that are not mentioned in this reference because they "may confuse users" or "need specialized knowledge to use".

If you are familiar with your skills, try to create amazing works with [this](../pyxel/__init__.pyi) as a clue!

## Como Contribuir

### Submitting Issue

Use the [Issue Tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature/enhancement requests. Before submitting a new issue, ensure that there is no similar open issue.

### Manual Testing

Anyone manually testing the code and reporting bugs or suggestions for enhancements in the [Issue Tracker](https://github.com/kitao/pyxel/issues) are very welcome!

### Submitting Pull Request

Patches/correções serão aceitas na forma de pull requests (PRs). Tenha certeza de que o que o pull request tenta resolver esteja em aberto no issue tracker.

Será considerado que todo pull request tenha concordado a ser publicado sob a [licença MIT](../LICENSE).

## Outras informações

- [Discord server (English)](https://discord.gg/FC7kUZJ)
- [Discord server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## Licença

Pyxel is under [MIT License](../LICENSE). It can be reused within proprietary software, provided that all copies of the software or its substantial portions include a copy of the terms of the MIT License and also a copyright notice.
