# <img src="images/pyxel_logo_152x64.png">

[ [English](../README.md) | [中文](README.cn.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Deutsch](README.de.md)]

**NOTE: This manual has not yet been translated for Pyxel version 1.5.0. We are looking for volunteers to translate and check for mistakes!**

**Pyxel** -- это игровой движок для Python в стиле ретро.

Благодаря своей простоте, вдохновленной старыми игровыми консолями (например, палитра состоит всего из 16 цветов, и только 4 звука могут быть проиграны одновременно), вы можете легко создавать игры в стиле пиксель-арт.

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

The specifications of Pyxel are referring to awesome [PICO-8](https://www.lexaloffle.com/pico-8.php) and [TIC-80](https://tic.computer/).

Pyxel -- программа с открытым кодом и бесплатна для использовния. За дело!

## Характеристики

- Запускается на Windows, Mac и Linux
- Programming with Python
- 16 color palette
- 3 набора изображений 256x256 пикселей
- 8 тайлмапов 256x256 пикселей
- 4 канала с 64 определяемыми пользователем звуками
- 8 музыкальных композиций
- Ввод с клавиатуры, мышки или игрового контроллера
- Редактор изображений и звука

### Цветовая Палитра

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Как установить

There are two types of Pyxel, a packaged version and a standalone version.

### Install Packaged Version

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

**Linux**

After installing the SDL2 package (`libsdl2-dev` for Ubuntu), [Python3](https://www.python.org/) (version 3.7 or higher), and `python3-pip`, run the following command:

```sh
sudo pip3 install -U pyxel
```

If the above doesn't work, try self-building by following the steps below after installing `cmake` and `rust`:

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make clean all
sudo pip3 install .
```

### Install Standalone Version

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

After installing the SDL2 package (`libsdl2-dev` for Ubuntu) and installing [Homebrew](https://brew.sh/), run the following commands:

```sh
brew tap kitao/pyxel
brew install pyxel
```

If the above doesn't work, try self-building the packaged version.

### Try Pyxel Examples

После установки Pyxel, примеры Pyxel будут скопированы в открытую директорию по выполнении этой команды:

```sh
pyxel copy_examples
```

Список примеров, которые будут скопированы:

- [01_hello_pyxel.py](../pyxel/examples/01_hello_pyxel.py) - Простейшее приложение
- [02_jump_game.py](../pyxel/examples/02_jump_game.py) - Игра прыжков с простейшими ресурсными файлами Pyxel
- [03_draw_api.py](../pyxel/examples/03_draw_api.py) - Demonstration of drawing APIs
- [04_sound_api.py](../pyxel/examples/04_sound_api.py) - Demonstration of sound APIs
- [05_color_palette.py](../pyxel/examples/05_color_palette.py) - Цветовая палитра
- [06_click_game.py](../pyxel/examples/06_click_game.py) - Игра с кликами мышкой
- [07_snake.py](../pyxel/examples/07_snake.py) - Змейка с BGM
- [08_triangle_api.py](../pyxel/examples/08_triangle_api.py) - Демонстрация API по рисованию треугольных полигонов
- [09_shooter.py](../pyxel/examples/09_shooter.py) - Shoot'em up game with screen transition
- [10_platformer.py](../pyxel/examples/10_platformer.py) - Side-scrolling platform game with map

An examples can be executed with the following commands:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
```

## Как использовать Pyxel

### Создание Pyxel-приложения

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

Агрументы функции `run` -- это функции `update` для обновления внутренней игровой логики каждый кадр и функции `draw` для отображения объектов на экране по мере необходимости.

В самом приложении рекомендуется свернуть код Pyxel в один класс (смотрите пример).

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

### Run Pyxel Application

The created Python script can be executed with the following command:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

For the packaged version, it can be executed like a normal Python script:

```sh
cd pyxel_examples
python3 PYTHON_SCRIPT_FILE
```

(For Windows, type `python` instead of `python3`)

### Особые клавиши

Следующие особые клавиши можно применять во время выполнения Pyxel-приложения:

- `Esc`<br>
Выйти из приложения
- `Alt(Option)+1`<br>
Выполнить снимок экрана и сохранить его на рабочий стол
- `Alt(Option)+2`<br>
Начать захват экрана игры
- `Alt(Option)+3`<br>
Сохранить видео, полученное захватом экрана на рабочий стол (до 10 секунд)
- `Alt(Option)+0`<br>
Включить/выключить мониториг производительности (fps, время на update, время на draw)
- `Alt(Option)+Enter`<br>
Войти/выйти из полноэкранного режима

### Как создать ресурсный файл

Pyxel Editor can create images and sounds used in a Pyxel application.

It starts with the following command:

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

Если указанный ресурсный файл (.pyxres) существует, то он будет загружен. В противном случае будет создан файл с указанным именем.
Если имя файла пропущено, то используется стандартное имя `my_resource.pyxres`

After starting Pyxel Editor, the file can be switched by dragging and dropping another resource file. If the resource file is dragged and dropped while holding down ``Ctrl(Cmd)`` key, only the resource type (Image/Tilemap/Sound/Music) that is currently being edited will be loaded. This operation enables to combine multiple resource files into one.

The created resource file can be loaded with `load` function.

Редактор Pyxel Editor оснащем следующими режимами редактирования.

**Редактор изображений:**

Режим редактирования наборов изображений.

<img src="images/image_editor.gif">

By dragging and dropping an image file (png/gif/jpeg) onto the Image Editor screen, the image can be loaded into the currently selected image bank.

**Редактор тайлмапов:**

Режим редактирования тайлмапов, в котором изоражения расположены в плиточном порядке.

<img src="images/tilemap_editor.gif">

**Редактор звука:**

Режим для редактирования звуковых файлов.

<img src="images/sound_editor.gif">

**Редактор музыки:**

Режим для редактирования музыки, в которой звуки расставлены в порядке проигрывания.

<img src="images/music_editor.gif">

### Другие методы создания ресурсов

Pyxel images and tilemaps can also be created by the following methods:

- Create an image from a list of strings with `Image.set` function or `Tilemap.set` function
- Load an image file (png/gif/jpeg) in Pyxel palette with `Image.load` function

Pyxel sounds can also be created in the following method:

- Create a sound from strings with `Sound.set` function or `Music.set` function

Обратитесь к руководству по API (ниже) для получения более подробной информации об использовании этих функций.

### How to Distribute Application

Pyxel supports a dedicated application distribution file format (Pyxel application file) that works across platforms.

Create the Pyxel application file (.pyxapp) with the following command:

```sh
pyxel package APP_ROOT_DIR STARTUP_SCRIPT_FILE
```

If the application should include resources or additional modules, place them in the application folder.

The created application file can be executed with the following command:

```sh
pyxel play PYXEL_APP_FILE
```

## Руководство по API

### Система

- `width`, `height`<br>
Ширина и высота окна

- `frame_count`<br>
Количество отрисованных кадров

- `init(width, height, [title], [fps], [quit_key], [capture_scale], [capture_sec])`<br>
Initialize the Pyxel application with screen size (`width`, `height`). The following can be specified as options: the window title with `title`, the frame rate with `fps`, the key to quit the application with `quit_key`, the scale of the screen capture with `capture_scale`, and the maximum recording time of the screen capture video with `capture_sec`.<br>
e.g. `pyxel.init(160, 120, title="Pyxel with Options", fps=60, quit_key=pyxel.KEY_NONE, capture_sec=0)`

- `run(update, draw)`<br>
Start the Pyxel application and call `update` function for frame update and `draw` function for drawing.

- `show()`<br>
Show the screen and wait until the `Esc` key is pressed. (Do not use in normal applications)

- `flip()`<br>
Updates the screen once. (Do not use in normal applications)

- `quit()`<br>
Quit the Pyxel application.

### Ресурсы

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Load the resource file (.pyxres). If ``False`` is specified for the resource type (``image/tilemap/sound/music``), the resource will not be loaded.

### Ввод

- `mouse_x`, `mouse_y`<br>
Получить положение курсора мышки

- `mouse_wheel`<br>
Получить значение колесика мышки

- `btn(клавиша)`<br>
Получить `Ture`, если `клавиша` нажата, в противном случае получить `False`. ([Список определений клавиш](../pyxel/__init__.pyi))

- `btnp(клавиша, [hold], [period])`<br>
Получить `True`, если `клавиша` нажата в данный кадр, в противном случае получить `False`. В случае, если указаны параметры `hold` и `period`, `True` будет возвращено каждые `period` кадров, когда `key` уже зажата более `hold` кадров

- `btnr(клавиша)`<br>
Получить `True`, если `клавиша` была отпущена в данный кадр, в противном случае получить `False`

- `mouse(видна)`<br>
Установить видимость курсора: если `visible` равно `True`, сделать виндым, если `False`, то невидимым. Даже если курсор не отображается, его позицию всё равно можно получить соответствующими функциями.

### Графика

- `colors`<br>
List of the palette display colors. The display color is specified by a 24-bit numerical value. Use `colors.from_list` and `colors.to_list` to directly assign and retrieve Python lists.<br>
e.g. `org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img, [system])`<br>
Operate the image bank `img` (0-2). (See the Image class)<br>
e.g. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
Оперировать тайлмапом `tm`(0-7) (смотрите класс Tilemap)

- `clip(x, y, w, h)`<br>
Установить площадь рисования экрана с (`x`, `y`) до ширины `w` и высоты `h`. Сбросить площадь рисования до полного экрана можно с помощью `clip()`

- `camera(x, y)`<br>
Change the upper left corner coordinates of the screen to (`x`, `y`). Reset the upper left corner coordinates to (`0`, `0`) with `camera()`.

- `pal(col1, col2)`<br>
Поменять цвет `col1` с цветом `col2` во время рисования. Восстановить изначальную палитру можно с помощью `pal()`

- `cls(col)`<br>
Заполнить (очистить) экран цветом `col`

- `pget(x, y)`<br>
Получить цвет пикселя по координатам (`x`, `y`)

- `pset(x, y, col)`<br>
Нарисовать пиксель цвета `col` по координатам (`x`, `y`)

- `line(x1, y1, x2, y2, col)`<br>
Нарисовать отрезок цвета `col` из (`x1`, `y1`) в (`x2`, `y2`)

- `rect(x, y, w, h, col)`<br>
Нарисовать прямоугольник ширины, высоты `w` и цвета `h` по координатам (`x`, `y`)

- `rectb(x, y, w, h, col)`<br>
Нарисовать контур прямоугольника ширины, высоты `w` и цвета `h` по координатам (`x`, `y`)

- `circ(x, y, r, col)`<br>
Нарисовать круг радиуса `r` и цвета `col` центром в (`x`, `y`)

- `circb(x, y, r, col)`<br>
Нарисовать окружность радиуса `r` и цвета `col` центром в (`x`, `y`)

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
Нарисовать треугольник с вершинами в координатах (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) и цвета `col`

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
Нарисовать контур треугольника с вершинами в координатах (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) и цвета `col`

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
Скопировать область размеров (`w`, `h`), по координатам (`u`, `v`) набора изображений `img`(0-2) по координатам (`x`, `y`) на экране. Если для `w` и/или `h` установлено отрицательное значение, изображение будет развернуто горизонтально и/или вертикально. Если указан параметр `colkey`, соответствующий цвет будет считаться цветом фона (прозрачным цветом)

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Copy the region of size (`w`, `h`) from (`u`, `v`) of the tilemap `tm` (0-7) to (`x`, `y`). If negative value is set for `w` and/or `h`, it will reverse horizontally and/or vertically. If `colkey` is specified, treated as transparent color. The size of a tile is 8x8 pixels and is stored in a tilemap as a tuple of `(x in tile, y in tile)`.

- `text(x, y, s, col)`<br>
Нарисовать строку текста `s` цвета `col` по координате (`x`, `y`)

### Аудио

- `sound(snd)`<br>
Оперировать звуком `snd`(0-63).<br>
Пример: `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Оперировать музыкой `msc`(0-7) (смотрите класс Music)

- `play_pos(ch)`<br>
Get the sound playback position of channel `ch` (0-3) as a tuple of `(sound no, note no)`. Returns `None` when playback is stopped.

- `play(ch, snd, loop=False)`<br>
Play the sound `snd` (0-63) on channel `ch` (0-3). If `snd` is a list, it will be played in order. If `True` is specified for `loop`, loop playback is performed.

- `playm(msc, loop=False)`<br>
Play the music `msc` (0-7). If `True` is specified for `loop`, loop playback is performed.

- `stop([ch])`<br>
Stops playback of the specified channel `ch` (0-3). `stop()` to stop playing all channels.

### Класс Image

- `width`, `height`<br>
Ширина и высота изображения

- `data`<br>
Данные изображения (матрица 256x256)

- `get(x, y)`<br>
Получить данные изображения в точке (`x`, `y`)

- `set(x, y, data)`<br>
Set the image at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Load the image file (png/gif/jpeg) at (`x`, `y`).

### Класс Tilemap

- `width`, `height`<br>
Ширина и высота тайлмапа

- `refimg`<br>
The image bank (0-2) referenced by the tilemap

- `set(x, y, data)`<br>
Set the tilemap at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `pget(x, y)`<br>
Get the tile at (`x`, `y`). A tile is a tuple of `(x in tile, y in tile)`.

- `pset(x, y, tile)`<br>
Draw a `tile` at (`x`, `y`). A tile is a tuple of `(x in tile, y in tile)`.

### Класс Sound

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

### Класс Music

- `sequences`<br>
Two-dimensional list of sounds (0-63) listed by the number of channels

- `set(seq0, seq1, seq2, seq3)`<br>
Set the lists of sound (0-63) of all channels. If an empty list is specified, that channel is not used for playback.<br>
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### Advanced APIs

Pyxel has "advanced APIs" that are not mentioned in this reference because they "may confuse users" or "need specialized knowledge to use".

If you are familiar with your skills, try to create amazing works with [this](../pyxel/__init__.pyi) as a clue!

## Как сделать вклад в развитие проекта?

### Submitting Issue

Use the [Issue Tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature/enhancement requests. Before submitting a new issue, ensure that there is no similar open issue.

### Manual Testing

Anyone manually testing the code and reporting bugs or suggestions for enhancements in the [Issue Tracker](https://github.com/kitao/pyxel/issues) are very welcome!

### Submitting Pull Request

Патчи/фиксы принимаются в форме pull-запросов (PRы). Убедитесь, что проблема, к которой относится pull-запрос, открыта в трекере проблем.

Опубликованный pull-запрос считается опубликованным под лицензией [MIT License](../LICENSE).

## Прочая информация

- [Сервер Discord (Англоязычный)](https://discord.gg/FC7kUZJ)
- [Сервер Discord (Японский - 日本語版)](https://discord.gg/qHA5BCS)

## Лицензия

Pyxel is under [MIT License](../LICENSE). It can be reused within proprietary software, provided that all copies of the software or its substantial portions include a copy of the terms of the MIT License and also a copyright notice.
