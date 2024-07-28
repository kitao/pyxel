# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** -- это игровой движок для Python в стиле ретро.

Благодаря своей простоте, вдохновленной старыми игровыми консолями (например, палитра состоит всего из 16 цветов, и только 4 звука могут быть проиграны одновременно), вы можете легко создавать игры в стиле пиксель-арт.

<img src="images/pyxel_message.png" width="480">

Мотивацией для разработки Pyxel является обратная связь от пользователей. Пожалуйста, дайте Pyxel звезду на GitHub!

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

Спецификации и API Pyxel вдохновлены [PICO-8](https://www.lexaloffle.com/pico-8.php) и [TIC-80](https://tic80.com/).

Pyxel -- программа с открытым кодом и бесплатна для использовния. За дело!

## Характеристики

- Работает под управлением Windows, Mac, Linux и Web
- Код пишется на Python
- 16-цветная палитра
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

### Windows

После установки [Python3](https://www.python.org/) (версии 3.7 или выше) необходимо выполнить следующую команду:

```sh
pip install -U pyxel
```

Если вы устанавливаете Python с помощью официального установщика, пожалуйста, установите флажок `Add Python 3.x to PATH`, чтобы включить команду `pyxel`.

### Mac

После установки [Homebrew](https://brew.sh/) выполните следующие команды:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Чтобы обновить версию после установки Pyxel, выполните команду `pipx upgrade pyxel`.

### Linux

После установки пакета SDL2 (`libsdl2-dev` для Ubuntu), [Python3](https://www.python.org/) (версии 3.7 или выше) и `python3-pip` выполните следующую команду:

```sh
sudo pip3 install -U pyxel
```

Если вышеописанное не сработало, попробуйте выполнить самосборку в соответствии с инструкциями в [Makefile](../Makefile).

### Web

Веб-версия Pyxel не требует установки Python или Pyxel и работает на ПК, а также на смартфонах и планшетах с поддерживаемыми веб-браузерами.

Для получения конкретных инструкций, пожалуйста, обратитесь к [этой странице](https://github.com/kitao/pyxel/wiki/How-To-Use-Pyxel-Web).

### Попробуйте примеры

После установки Pyxel, примеры Pyxel будут скопированы в открытую директорию по выполнении этой команды:

```sh
pyxel copy_examples
```

Список примеров, которые будут скопированы:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>Простейшее приложение</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Code</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Игра прыжков с простейшими ресурсными файлами Pyxel</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Code</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Демонстрация API для рисования</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Code</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Демонстрация API для работы со звуком</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Code</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>Цветовая палитра</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Code</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>Игра с кликами мышкой</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Code</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>Змейка с BGM</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Code</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Демонстрация API по рисованию треугольных полигонов</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Code</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Игра жанра «убей всех» с переходом между экранами</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Code</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>Платформер с боковым скроллингом и картой</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Code</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Внеэкранный рендеринг с помощью класса Image</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Code</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>Анимация шума Перлина</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Code</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>Рисование растрового шрифта</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Code</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>Синтезатор, использующий функции расширения звука</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Code</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>Загрузка и рисование файла карты плиток (.tmx)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Code</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>Анимация с функцией flip (только для не-веб-платформ)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Code</a></td>
</tr>
<tr>
<td>30SecondsOfDaylight.pyxapp</td>
<td>1-я победная игра Pyxel Jam от <a href="https://twitter.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30SecondsOfDaylight.html">Demo</a></td>
<td><a href="https://github.com/kitao/30SecondsOfDaylight">Code</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>Аркадная игра с физикой мяча от <a href="https://twitter.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Demo</a></td>
<td><a href="https://github.com/helpcomputer/megaball">Code</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>Генератор фоновой музыки, созданный <a href="https://twitter.com/frenchbread1222">frenchbread</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">Demo</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">Code</a></td>
</tr>
</table>

Эти примеры могут быть запущены следующей командой:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30SecondsOfDaylight.pyxapp
```

## Как использовать Pyxel

### Создание Pyxel-приложения

После импортирования модуля Pyxel в ваш код на Python, сначала укажите размер окна с помощью команды `init`, затем запустите Pyxel-приложение с помощью функции `run`.

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

При создании простой графики без анимации можно использовать функцию `show`, чтобы сделать код более лаконичным.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Запуск Pyxel-приложения

Созданый сценарий на Python может быть запущен путём выполнения следующей команды:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Он также может быть выполнен как обычный сценарий Python:

```sh
python3 PYTHON_SCRIPT_FILE
```

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
- `Alt(Option)+9`<br>
  Переключение между режимами экрана (Crisp/Smooth/Retro)
- `Alt(Option)+0`<br>
  Включить/выключить мониториг производительности (fps, время на update, время на draw)
- `Alt(Option)+Enter`<br>
  Войти/выйти из полноэкранного режима
- `Shift+Alt(Option)+1/2/3`<br>
  Сохраните соответствующий банк изображений на рабочем столе
- `Shift+Alt(Option)+0`<br>
  Сохраните текущую палитру цветов на рабочем столе

### Как создать ресурсный файл

Встроенный Pyxel Editor может создавать изображения и звуки, используемые в Pyxel-приложении.

Он запускается следующей командой:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Если указанный ресурсный файл (.pyxres) существует, то он будет загружен. В противном случае будет создан файл с указанным именем. Если имя файла пропущено, то используется стандартное имя `my_resource.pyxres`

После запуска Pyxel Editor, можно переключаться между различными файлами способом drag-and-drop.

Созданный ресурсный файл может быть загружен в программу с помощью функции `load`.

Редактор Pyxel Editor оснащем следующими режимами редактирования.

**Редактор изображений**

Режим редактирования наборов изображений.

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

Перетащите файл изображения (PNG/GIF/JPEG) в редактор изображений, чтобы загрузить его в выбранный в данный момент банк изображений.

**Редактор тайлмапов**

Режим редактирования тайлмапов, в котором изоражения расположены в плиточном порядке.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

Перетащите файл TMX (Tiled Map File) в редактор плиточных карт, чтобы загрузить его слой в порядке, соответствующем номеру выбранной в данный момент плиточной карты.

**Редактор звука**

Режим для редактирования звуковых файлов.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**Редактор музыки**

Режим для редактирования музыки, в которой звуки расставлены в порядке проигрывания.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### Другие методы создания ресурсов

Изображения и карты тайлов Pyxel могут также быть созданы следующим образом:

- Создайте изображение из списка строк с помощью функций `Image.set` или `Tilemap.set`.
- Загрузите файл изображения (PNG/GIF/JPEG) в палитру Pyxel с помощью функции `Image.load`.

Звуки Pyxel могут также быть созданы следующим образом:

- Создайте звук из строк с помощью функций `Sound.set` или `Music.set`.

Обратитесь к руководству по API (ниже) для получения более подробной информации об использовании этих функций.

### Как распространять приложение

Pyxel предлагает формат распространения приложений (файл Pyxel-приложения), работающий на всех поддерживаемых платформах.

Создать файл Pyxel-приложения (.pyxapp) можно с помощью следующей команды:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Если приложение должно включать в себя дополнительные ресурсы или модули, поместите их в каталог приложения.

Созданный файл приложения может быть запущен следующей командой:

```sh
pyxel play PYXEL_APP_FILE
```

Файл приложения Pyxel также может быть преобразован в исполняемый файл или файл HTML с помощью команд `pyxel app2exe` или `pyxel app2html`.

## Руководство по API

### Система

- `width`, `height`<br>
  Ширина и высота окна

- `frame_count`<br>
  Количество отрисованных кадров

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Инициализирует Pyxel-приложение с указанными размерами экрана (`width`, `height`). Дополнительно могут быть заданы: заголовок окна с помощью параметра `title`, количество кадров в секунду с помощью параметра `fps`, клавиша для выхода из приложения — `quit_key`, масштаб дисплея с помощью `display_scale`, коэффициент масштабирования при захвате экрана — `capture_scale` и максимальное время записи при захвате экрана с помощью `capture_sec`.<br>
  Пример: `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Запустить Pyxel-приложение, использующее функцию `update` для обновления внутренней логики и `draw` для рисования.

- `show()`<br>
  Отрисовать кадр и ждать выхода из приложения по нажатию клавиши `Esc`.

- `flip()`<br>
  Уменьшить экран на один кадр. Приложение завершается при нажатии клавиши `Esc`. Эта функция не работает в веб-версии.

- `quit()`<br>
  Завершить работу Pyxel-приложения.

### Ресурсы

- `load(filename, [excl_images], [excl_tilemaps], [excl_sounds], [excl_musics])`<br>
  Загрузить ресурсный файл (.pyxres). Если опция имеет значение `True`, ресурс не будет загружен. Если файл палитры (.pyxpal) с таким же именем существует в том же месте, что и файл ресурса, цвет отображения палитры также будет изменен. Файл палитры представляет собой шестнадцатеричную запись цветов отображения (например, `1100FF`), разделенных новыми строками. Файл палитры также может быть использован для изменения цветов, отображаемых в Pyxel Editor.

### Ввод

- `mouse_x`, `mouse_y`<br>
  Получить положение курсора мышки

- `mouse_wheel`<br>
  Получить значение колесика мышки

- `btn(клавиша)`<br>
  Получить `Ture`, если `клавиша` нажата, в противном случае получить `False`. ([Список определений клавиш](../python/pyxel/__init__.pyi))

- `btnp(клавиша, [hold], [repeat])`<br>
  Получить `True`, если `клавиша` нажата в данный кадр, в противном случае получить `False`. В случае, если указаны параметры `hold` и `repeat`, `True` будет возвращено каждые `repeat` кадров, когда `key` уже зажата более `hold` кадров.

- `btnr(клавиша)`<br>
  Получить `True`, если `клавиша` была отпущена в данный кадр, в противном случае получить `False`

- `mouse(видна)`<br>
  Установить видимость курсора: если `visible` равно `True`, сделать виндым, если `False`, то невидимым. Даже если курсор не отображается, его позицию всё равно можно получить соответствующими функциями.

### Графика

- `colors`<br>
  Список цветов палитры. Цвет кодируется 24-битным целым числом. Используйте `colors.from_list` и `colors.to_list` для установки и получения списка Python.<br>
  Пример: `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Список банков изображений (0-2). (смотрите класс Image).<br>
  Пример: `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Список тайлмапов (0-7). (смотрите класс Tilemap)

- `clip(x, y, w, h)`<br>
  Установить площадь рисования экрана с (`x`, `y`) до ширины `w` и высоты `h`. Сбросить площадь рисования до полного экрана можно с помощью `clip()`

- `camera(x, y)`<br>
  Изменить координаты левого верхнего угла экрана на (`x`, `y`). Координаты левого верхнего угла экрана могут быть сброшены в (`0`, `0`) вызовом `camera()`.

- `pal(col1, col2)`<br>
  Поменять цвет `col1` с цветом `col2` во время рисования. Восстановить изначальную палитру можно с помощью `pal()`

- `dither(alpha)`<br>
  Применяет дизеринг (псевдопрозрачность) при рисовании. Установите `alpha` в диапазоне 0.0-1.0, где 0.0 - прозрачный, а 1.0 - непрозрачный.

- `cls(col)`<br>
  Заполнить (очистить) экран цветом `col`

- `pget(x, y)`<br>
  Получить цвет пикселя по координатам (`x`, `y`).

- `pset(x, y, col)`<br>
  Нарисовать пиксель цвета `col` по координатам (`x`, `y`).

- `line(x1, y1, x2, y2, col)`<br>
  Нарисовать отрезок цвета `col` из (`x1`, `y1`) в (`x2`, `y2`).

- `rect(x, y, w, h, col)`<br>
  Нарисовать прямоугольник ширины, высоты `w` и цвета `h` по координатам (`x`, `y`).

- `rectb(x, y, w, h, col)`<br>
  Нарисовать контур прямоугольника ширины, высоты `w` и цвета `h` по координатам (`x`, `y`).

- `circ(x, y, r, col)`<br>
  Нарисовать круг радиуса `r` и цвета `col` центром в (`x`, `y`).

- `circb(x, y, r, col)`<br>
  Нарисовать окружность радиуса `r` и цвета `col` центром в (`x`, `y`).

- `elli(x, y, w, h, col)`<br>
  Нарисуйте эллипс шириной `w`, высотой `h` и цветом `col` из (`x`, `y`).

- `ellib(x, y, w, h, col)`<br>
  Нарисуйте контур эллипса шириной `w`, высотой `h` и цветом `col` из (`x`, `y`).

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  Нарисовать треугольник с вершинами в координатах (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) и цвета `col`

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  Нарисовать контур треугольника с вершинами в координатах (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) и цвета `col`

- `fill(x, y, col)`<br>
  Нарисуйте эллипс шириной `w`, высотой `h` и цветом `col` из (`x`, `y`).

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
  Скопировать область размеров (`w`, `h`), по координатам (`u`, `v`) набора изображений `img`(0-2) по координатам (`x`, `y`) на экране. Если для `w` и/или `h` установлено отрицательное значение, изображение будет развернуто горизонтально и/или вертикально. Если указан параметр `colkey`, соответствующий цвет будет считаться цветом фона (прозрачным цветом).

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
  Нарисовать из тайлмапа `tm`(0-7) по координатам (`x`, `y`) тайл размером (`w`, `h`), находящийся по координатам (`u`, `v`). Если переданы отрицательные значения `w` и/или `h`, то изображение будет отражено по горизонтали и/или вертикали. Если указан параметр `colkey`, соответствующий цвет будет считаться цветом фона (прозрачным цветом). Размер тайла равен 8x8 точек и хранится в карте тайлов в виде кортежа `(tile_x, tile_y)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  Нарисовать строку текста `s` цвета `col` по координате (`x`, `y`).

### Аудио

- `sounds`<br>
  Оперировать звуком `snd`(0-63). (См. класс "Звук")<br>
  Пример: `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Список музыкальных произведений (0-7). (смотрите класс Music)

- `play(ch, snd, [tick], [loop], [resume])`<br>
  Проиграть звук `snd`(0-63) на канале `ch`(0-3). Если `snd` — список, он будет проигран по порядку. Позиция начала воспроизведения может быть указана с помощью `tick`(1 tick = 1/120 секунды). Если в в качестве значения `loop` передано `True`, проигрывание будет зациклено. Чтобы возобновить предыдущий звук после завершения воспроизведения, установите `resume` в `True`.

- `playm(msc, [tick], [loop])`<br>
  Проиграть трек `msc`(0-7). Позиция начала воспроизведения может быть указана с помощью `tick`(1 tick = 1/120 секунды). Если в в качестве значения `loop` передано `True`, проигрывание будет зациклено.

- `stop([ch])`<br>
  Остановить воспроизведение на канале `ch`(0-3). `stop()` останавливает воспроизведение на всех каналах.

- `play_pos(ch)`<br>
  Получить позицию канала `ch`(0-3) в виде кортежа `(номер звука, номер ноты)`. Возвращает `None` если проигрывание выключено.

### Математика

- `ceil(x)`<br>
  Возвращает наименьшее целое число, большее или равное `x`.

- `floor(x)`<br>
  Возвращает наибольшее целое число, меньшее или равное `x`.

- `sgn(x)`<br>
  Возвращает 1, если `x` положительно, 0, если оно равно нулю, и -1, если оно отрицательно.

- `sqrt(x)`<br>
  Возвращает квадратный корень из `x`.

- `sin(deg)`<br>
  Возвращает синус градуса `deg`.

- `cos(deg)`<br>
  Возвращает косинус градуса `deg`.

- `atan2(y, x)`<br>
  Возвращает арктангенс угла `y`/`x` в градусах.

- `rseed(seed)`<br>
  Устанавливает затравку генератора случайных чисел.

- `rndi(a, b)`<br>
  Возвращает случайное целое число, большее или равное `a` и меньшее или равное `b`.

- `rndf(a, b)`<br>
  Возвращает случайную десятичную дробь, большую или равную `a` и меньшую или равную `b`.

- `nseed(seed)`<br>
  Устанавливает семя шума Перлина.

- `noise(x, [y], [z])`<br>
  Возвращает значение шума Перлина для указанных координат.

### Класс Image

- `width`, `height`<br>
  Ширина и высота изображения

- `set(x, y, data)`<br>
  Установить данные изображения в точке (`x`, `y`) списком строк.<br>
  Пример: `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Загрузить файл изображения (PNG/GIF/JPEG) в координаты (`x`, `y`).

- `pget(x, y)`<br>
  Получить цвет пикселя по координатам (`x`, `y`).

- `pset(x, y, col)`<br>
  Нарисовать пиксель цвета `col` по координатам (`x`, `y`).

### Класс Tilemap

- `width`, `height`<br>
  Ширина и высота тайлмапа

- `imgsrc`<br>
  Банк изображений (0-2), на который ссылается карта тайлов

- `set(x, y, data)`<br>
  Установить данные карты тайлов в точке (`x`, `y`) списком строк.<br>
  Пример: `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  Загрузите слой в порядке рисования `layer`(0-) из файла TMX (Tiled Map File) по адресу (`x`, `y`).

- `pget(x, y)`<br>
  Получить тайл в координатах (`x`, `y`). Возвращаемое значение представляет собой кортеж `(tile_x, tile_y)`.

- `pset(x, y, tile)`<br>
  Задать тайл в координатах (`x`, `y`). Тайл передаётся в виде кортежа `(tile_x, tile_y)`.

### Класс Sound

- `notes`<br>
  Список нот (0-127). Чем больше значение, тем выше нота. Значение 33 соответствует ноте «ля» второй октавы 'A2' (440Hz). Пауза задаётся значением -1.

- `tones`<br>
  Список тонов (0:Треугольник / 1:Квадрат / 2:Пульс / 3:Шум)

- `volumes`<br>
  Список громкости (0-7)

- `effects`<br>
  Список эффектов (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  Длительность воспроизведения. 1 — самая быстрая, чем выше значение, тем ниже скорость воспроизведения. При значении, равном 120 длительность воспроизведения одной ноты составляет 1 секунду.

- `set(notes, tones, volumes, effects, speed)`<br>
  Установить ноты, тоны, громкость и эффекты с помощью строк. Если длины строк для тона, громкости и эффектов короче строки для нот, они зацикливаются.

- `set_notes(notes)`<br>
  Установить ноты с помощью строки, составленной по форме 'CDEFGAB'+'#-'+'01234' или 'R'. Регистр и пробелы игнорируются.<br>
  Пример: `pyxel.sounds[0].set_notes("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
  Установить тоны строкой, составленной из 'TSPN'. Регистр и пробелы игнорируются.<br>
  Пример: `pyxel.sounds[0].set_tones("TTSS PPPN")`

- `set_volumes(volumes)`<br>
  Установить громкость с помощью строки, составленной из '01234567'. Регистр и пробелы игнорируются.<br>
  Пример: `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  Установить эффекты с помощью строки, составленной из 'NSVFHQ'. Регистр и пробелы игнорируются.<br>
  Пример: `pyxel.sounds[0].set_effects("NFNF NVVS")`

### Класс Music

- `seqs`<br>
  Двумерный список звуков (0-63) с указанием количества каналов

- `set(seq0, seq1, seq2, ...)`<br>
  Установите списки звуков (0-63) каналов. Пустой список означает, что канал не используется для проигрывания.<br>
  Пример: `pyxel.musics[0].set([0, 1], [], [3])`

### Расширенный APIs

Pyxel имеет «расширенные API», не упомянутые в этом документе, так как они «могут смутить пользователя» или «требуют специальных знаний для использования».

Если вы уверены в своих силах, используйте [это](../python/pyxel/__init__.pyi) в качестве подсказки!

## Как сделать вклад в развитие проекта?

### Сообщение о проблемах

Используйте [трекер проблем](https://github.com/kitao/pyxel/issues) для отправки отчётов о проблемах или предложений по улучшению/добавлению новых возможностей. Перед созданием новой задачи, убедитесь что схожие открытые задачи отсутствуют.

### Ручное тестирование

Ручное тестирование кода и написание отчетов о проблемах, предложений по улучшению в [трекере проблем](https://github.com/kitao/pyxel/issues) приветствуется!

### Опубликование запроса на слияние

Патчи/фиксы принимаются в форме запросов на слияние (pull-запрос, PR). Убедитесь, что проблема, к которой относится запрос на слияние изменений, открыта в трекере проблем.

Опубликованный pull-запрос считается опубликованным под лицензией [MIT License](../LICENSE).

## Прочая информация

- [Q&A](https://github.com/kitao/pyxel/wiki/Pyxel-Q&A)
- [User Examples](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Developer's Twitter account](https://twitter.com/kitao)

## Лицензия

Pyxel разпространяется по лицензией [MIT License](../LICENSE). Он может быть использован в проприетарном программном обеспечении при условии того, что все копии этого программного обеспечения или значительные его части содержат копию MIT License terms and the copyright notice.

## Набор Спонсоров

Pyxel ищет спонсоров на GitHub Sponsors. Рассмотрите возможность спонсирования Pyxel для продолжения обслуживания и добавления функций. Спонсоры могут проконсультироваться о Pyxel в качестве преимущества. Подробнее см. [Здесь](https://github.com/sponsors/kitao).
