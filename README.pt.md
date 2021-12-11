# <img src="images/pyxel_logo_152x64.png">

[ [English](README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**NOTE: This manual has not yet been translated for Pyxel version 1.5.0. We are looking for volunteers to translate and check for mistakes!**

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

The specifications of Pyxel are referring to awesome [PICO-8](https://www.lexaloffle.com/pico-8.php) and [TIC-80](https://tic.computer/).

Pyxel é open source e livre para utilização. Vamos começar fazendo um jogo retrô com Pyxel!

## Especificações

- Executável no Windows, Mac e Linux
- Programming with Python
- 16 color palette
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

There are two types of Pyxel, a packaged version and a standalone version.

### Install the Packaged Version

The packaged version of Pyxel uses Pyxel as a Python extension module.

Recommended for those who are familiar with managing Python packages using the `pip` command or who want to develop full-fledged Python applications.

**Windows**

After installing [Python3](https://www.python.org/) (version 3.7 or higher), run the following command:

```sh
pip install -U pyxel
```

**Mac**

After installing [Python3](https://www.python.org/) (version 3.7 or higher), run the following command:

```sh
pip3 install -U pyxel
```

### Linux

After installing the SDL2 package (`libsdl2-dev` for Ubuntu), [Python3](https://www.python.org/) (version 3.7 or higher), and `python3-pip`, run the following command:

```sh
pip3 install -U pyxel
```

If the above doesn't work, try self-building by following the steps below after installing `cmake` and `rust`:

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make clean all RELEASE=1
pip3 install .
```

### Install the Standalone Version

The standalone version of Pyxel uses Pyxel as a standalone tool that does not depend on Python.

Recommended for those who want to start programming easily without worrying about Python settings, or those who want to play Pyxel games immediately.

**Windows**

Download and run the latest version of the Windows installer (`pyxel-[version]-windows-setup.exe`) from the [Download Page](https://github.com/kitao/pyxel/releases).

**Mac**

After installing [Homebrew](https://brew.sh/), run the following commands:

```sh
brew tap kitao/pyxel
brew install pyxel
```

**Linux**

After installing the SDL2 package (`libsdl2-dev` for Ubuntu) and installing [Homebrew](https://docs.brew.sh/Homebrew-on-Linux), run the following commands:

```sh
brew tap kitao/pyxel
brew install pyxel
```

If the above doesn't work, try self-building the packaged version.

### Instalando exemplos

Após instalar o Pyxel, os exemplos serão copiados para o diretório atual com o seguinte comando:

```sh
pyxel copy_examples
```

Os exemplos copiados são os seguintes:

- [01_hello_pyxel.py](pyxel/examples/01_hello_pyxel.py) - Aplicação simples
- [02_jump_game.py](pyxel/examples/02_jump_game.py) - Jogo de pulo com o arquivo de recursos do Pyxel
- [03_draw_api.py](pyxel/examples/03_draw_api.py) - Demonstration of drawing APIs
- [04_sound_api.py](pyxel/examples/04_sound_api.py) - Demonstration of sound APIs
- [05_color_palette.py](pyxel/examples/05_color_palette.py) - Lista da paleta de cores
- [06_click_game.py](pyxel/examples/06_click_game.py) - Jogo de clique com mouse
- [07_snake.py](pyxel/examples/07_snake.py) - Jogo Snake com BGM
- [08_triangle_api.py](pyxel/examples/08_triangle_api.py) - Demonstration of triangle drawing APIs
- [09_shooter.py](pyxel/examples/09_shooter.py) - Jogo de tiro com transição de tela
- [10_platformer.py](pyxel/examples/10_platformer.py) - Side-scrolling platform game with map

An examples can be executed with the following commands:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
```

For the packaged version, it can be executed like a normal Python script:

```sh
cd pyxel_examples
python3 01_hello_pyxel.py
```

(For Windows, type `python` instead of `python3`)

## Como usar

### Criando uma aplicação Pyxel

After importing the Pyxel module in your python script, specify the window size with `init` function first, then starts the Pyxel application with `run` function.

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

It is also possible to write simple code using `show` function and `flip` function to draw simple graphics and animations.

`show` function displays the screen and waits until the `Esc` key is pressed.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

`flip` function updates the screen once.

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
Salva um vídeo de captura de tela na área de trabalho (até 10 segundos)
- `Alt(Option)+0`<br>
Ativa/desativa o monitor de performance (fps, tempo de update e tempo de draw)
- `Alt(Option)+Enter`<br>
Ativa/desativa tela cheia

### Como criar um Recurso

Pyxel Editor can create images and sounds used in a Pyxel application.

It starts with the following command:

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

Se o arquivo de recursos Pyxel (.pyxres) existir, o arquivo será carregado, e se ele não existir, um novo arquivo com o nome especificado será criado.
Se o arquivo de recursos for omitido, o nome será `my_resource.pyxres`.

After starting Pyxel Editor, the file can be switched by dragging and dropping another resource file. If the resource file is dragged and dropped while holding down ``Ctrl(Cmd)`` key, only the resource type (Image/Tilemap/Sound/Music) that is currently being edited will be loaded. This operation enables to combine multiple resource files into one.

The created resource file can be loaded with `load` function.

O Editor Pyxel possuí os seguintes modos de edição.

**Editor de Imagem:**

O modo para editar bancos de imagem.

<img src="pyxel/editor/screenshots/image_editor.gif">

By dragging and dropping an image file (png/gif/jpeg) onto the Image Editor screen, the image can be loaded into the currently selected image bank.

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

Pyxel images and tilemaps can also be created by the following methods:

- Create an image from a list of strings with `Image.set` function or `Tilemap.set` function
- Load an image file (png/gif/jpeg) in Pyxel palette with `Image.load` function

Pyxel sounds can also be created in the following method:

- Create a sound from strings with `Sound.set` function or `Music.set` function

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
Retorna `True` se `key` é pressionada, caso contrário retorna `False` ([lista de definições de teclas](pyxel/__init__.pyi))

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
Draw the tilemap `tm` (0-7) to (`x`, `y`) according to the tile information of size (`w`, `h`) from (`u`, `v`). If `colkey` is specified, treated as transparent color. The size of a tile is 8x8 pixels and is stored in a tilemap as a tuple of `(x-in-tile, y-in-tile)`.

- `text(x, y, s, col)`<br>
Desenha uma string `s` de cor `col` em (`x`, `y`)

### Áudio

- `sound(snd)`<br>
Opera o som `snd`(0-63). (ver a classe de Som)<br>
e.g. `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Opera a música `msc` (0-7) (ver a classe de Musica)

- `play_pos(ch)`<br>
Get the sound playback position of channel `ch` (0-3) as a tuple of `(sound-no, note-no)`. Returns `None` when playback is stopped.

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
Get the tile at (`x`, `y`). A tile is a tuple of `(x-in-tile, y-in-tile)`.

- `pset(x, y, tile)`<br>
Draw a `tile` at (`x`, `y`). A tile is a tuple of `(x-in-tile, y-in-tile)`.

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

If you are familiar with your skills, try to create amazing works with [this](pyxel/__init__.pyi) as a clue!

## Como Contribuir

### Submitting an Issue

Use the [Issue Tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature/enhancement requests. Before submitting a new issue, ensure that there is no similar open issue.

### Manual Testing

Anyone manually testing the code and reporting bugs or suggestions for enhancements in the [Issue Tracker](https://github.com/kitao/pyxel/issues) are very welcome!

### Submitting a Pull Request

Patches/correções serão aceitas na forma de pull requests (PRs). Tenha certeza de que o que o pull request tenta resolver esteja em aberto no issue tracker.

Será considerado que todo pull request tenha concordado a ser publicado sob a [licença MIT](LICENSE).

## Outras informações

- [Discord server (English)](https://discord.gg/FC7kUZJ)
- [Discord server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## Licença

Pyxel is under [MIT License](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software, provided that all copies of the software or its substantial portions include a copy of the terms of the MIT License and also a copyright notice.
