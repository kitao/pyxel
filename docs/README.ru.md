# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** — это ретро-игровой движок для Python.

Спецификации вдохновлены ретро-игровыми консолями, такими как поддержка только 16 цветов и 4 звуковых канала, что позволяет легко наслаждаться созданием игр в стиле пиксельной графики.

<img src="images/pyxel_message.png" width="480">

Разработка Pyxel осуществляется на основе отзывов пользователей. Пожалуйста, поставьте Pyxel звезду на GitHub!

<p>
<a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">
<img src="images/10_platformer.gif" width="290">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/30sec_of_daylight.html">
<img src="images/30sec_of_daylight.gif" width="350">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">
<img src="images/02_jump_game.gif" width="330">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">
<img src="images/megaball.gif" width="310">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_music_editor.gif" width="320">
</a>
</p>

Спецификации и API Pyxel вдохновлены [PICO-8](https://www.lexaloffle.com/pico-8.php) и [TIC-80](https://tic80.com/).

Pyxel является открытым программным обеспечением под [MIT лицензией](../LICENSE) и бесплатен для использования. Давайте начнем создавать ретро-игры с Pyxel!

## Спецификации

- Работает на Windows, Mac, Linux и Web
- Программирование на Python
- 16-цветная палитра
- 3 банка изображений размером 256x256
- 8 тайловых карт размером 256x256
- 4 канала с 64 настраиваемыми звуками
- 8 музыкальных произведений, которые могут сочетать любые звуки
- Ввод с клавиатуры, мыши и игрового контроллера
- Инструменты для редактирования изображений и звуков
- Расширяемые пользователем цвета, каналы и банки

### Цветовая палитра

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Как установить

### Windows

После установки [Python3](https://www.python.org/) (версии 3.8 или выше), выполните следующую команду:

```sh
pip install -U pyxel
```

При установке Python с помощью официального установщика убедитесь, что вы отметили опцию `Add Python 3.x to PATH`, чтобы активировать команду `pyxel`.

### Mac

После установки [Homebrew](https://brew.sh/) выполните следующие команды:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Чтобы обновить Pyxel после установки, выполните `pipx upgrade pyxel`.

### Linux

После установки пакета SDL2 (`libsdl2-dev` для Ubuntu), [Python3](https://www.python.org/) (версии 3.8 или выше) и `python3-pip` выполните следующую команду:

```sh
sudo pip3 install -U pyxel
```

Если предыдущая команда не сработала, подумайте о сборке Pyxel из исходного кода, следуя инструкциям в [Makefile](../Makefile).

### Web

Веб-версия Pyxel не требует установки Python или Pyxel и работает на ПК, смартфонах и планшетах с поддерживаемыми веб-браузерами.

Для получения подробных инструкций обратитесь к [этой странице](pyxel-web-en.md).

### Запуск примеров

После установки Pyxel вы можете скопировать примеры в текущую директорию с помощью следующей команды:

```sh
pyxel copy_examples
```

Следующие примеры будут скопированы в вашу текущую директорию:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>Самое простое приложение</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Код</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Игра с прыжками с использованием файла ресурсов Pyxel</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Код</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Демонстрация API для рисования</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Код</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Демонстрация API для звука</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Код</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>Список цветовых палитр</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Код</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>Игра на клик мыши</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Код</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>Игра «Змейка» с BGM</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Код</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Демонстрация API для рисования треугольников</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Код</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Shoot 'em up с переходами между экранами</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Код</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>Горизонтальная платформенная игра с картой</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Код</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Отрисовка вне экрана с классом Image</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Код</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>Анимация Перлин-шума</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Код</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>Рисование битмап-шрифта</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Код</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>Синтезатор с использованием расширенных функций аудио</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Код</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>Загрузка и отрисовка Tiled Map File (.tmx)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Код</a></td>
</tr>
<tr>
<td>16_transform.py</td>
<td>Поворот и масштабирование изображений</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/16_transform.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/16_transform.py">Код</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>Анимация с помощью функции flip (только для не веб-платформ)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Код</a></td>
</tr>
<tr>
<td>30sec_of_daylight.pyxapp</td>
<td>Победившая игра 1-го Pyxel Jam от <a href="https://x.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30sec_of_daylight.html">Демонстрация</a></td>
<td><a href="https://github.com/kitao/30sec_of_daylight">Код</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>Аркадная игра с физикой мячей от <a href="https://x.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Демонстрация</a></td>
<td><a href="https://github.com/helpcomputer/megaball">Код</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>Генератор фоновой музыки от <a href="https://x.com/frenchbread1222">frenchbread</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">Демонстрация</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">Код</a></td>
</tr>
</table>

Примеры можно выполнить с помощью следующих команд:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30sec_of_daylight.pyxapp
```

## Как использовать

### Создание приложения

В вашем Python-скрипте импортируйте модуль Pyxel, укажите размер окна с помощью функции `init`, а затем запустите приложение Pyxel с помощью функции `run`.

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

Аргументы функции `run` — это функция `update`, которая обрабатывает обновления кадров, и функция `draw`, которая отвечает за отрисовку на экране.

В реальном приложении рекомендуется обернуть код Pyxel в класс, как показано ниже:

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

Для создания простых графиков без анимации вы можете использовать функцию `show`, чтобы упростить ваш код.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Запуск приложения

Созданный скрипт можно выполнить с помощью команды `python`:

```sh
python PYTHON_SCRIPT_FILE
```

Его также можно запустить с помощью команды `pyxel run`:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Кроме того, команда `pyxel watch` отслеживает изменения в указанном каталоге и автоматически перезапускает программу при обнаружении изменений:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

Наблюдение за каталогом можно остановить, нажав `Ctrl(Command)+C`.

### Специальные клавиши

Во время выполнения приложения Pyxel можно выполнить следующие специальные действия с клавишами:

- `Esc`<br>
  Выйти из приложения
- `Alt(Option)+1`<br>
  Сохранить снимок экрана на рабочий стол
- `Alt(Option)+2`<br>
  Сбросить время начала записи видео с экрана
- `Alt(Option)+3`<br>
  Сохранить видео захвата экрана на рабочий стол (до 10 секунд)
- `Alt(Option)+9`<br>
  Переключение между режимами экрана (Crisp/Smooth/Retro)
- `Alt(Option)+0`<br>
  Переключить монитор производительности (FPS/`update` время/`draw` время)
- `Alt(Option)+Enter`<br>
  Переключить полноэкранный режим
- `Shift+Alt(Option)+1/2/3`<br>
  Сохранить соответствующий банк изображений на рабочий стол
- `Shift+Alt(Option)+0`<br>
  Сохранить текущую цветовую палитру на рабочий стол

### Как создать ресурсы

Pyxel Editor может создавать изображения и звуки, используемые в Pyxel-приложении.

Он запускается следующей командой:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Если указанный ресурсный файл (.pyxres) существует, то он будет загружен. В противном случае будет создан файл с указанным именем. Если имя файла пропущено, то используется стандартное имя `my_resource.pyxres`

После запуска Pyxel Editor можно переключаться между ресурсными файлами, перетаскивая другой ресурсный файл.

Созданный ресурсный файл можно загрузить с помощью функции `load`.

После запуска Pyxel Editor можно переключаться между ресурсными файлами, перетаскивая другой ресурсный файл.

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

**Редактор звуков**

Режим для редактирования звуков.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**Редактор музыки**

Режим для редактирования музыки, в которой звуки расставлены в порядке воспроизведения.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### Другие методы создания ресурсов

Изображения и тайлмапы Pyxel могут также быть созданы следующим образом:

- Создайте изображение из списка строк с помощью функции `Image.set` или тайлмап с помощью функции `Tilemap.set`.
- Загрузите файл изображения (PNG/GIF/JPEG) в палитру Pyxel с помощью функции `Image.load`.

Звуки в Pyxel могут также быть созданы следующим образом:

- Создайте звук из строк с помощью функции `Sound.set` или `Music.set`.

Обратитесь к руководству по API (ниже) для получения более подробной информации об использовании этих функций.

### Как распространять приложения

Pyxel поддерживает специальный формат файла для распространения приложений (файл Pyxel-приложения), работающий на всех платформах.

Создайте файл приложения Pyxel (.pyxapp) с помощью команды `pyxel package`:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Если необходимо включить ресурсы или дополнительные модули, поместите их в каталог приложения.

Метаданные можно отобразить во время выполнения, указав их в следующем формате в сценарии запуска. Поля, отличные от `title` и `author`, могут быть опущены.

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

Созданный файл приложения можно запустить с помощью команды `pyxel play`:

```sh
pyxel play PYXEL_APP_FILE
```

Файл приложения Pyxel также можно преобразовать в исполняемый файл или файл HTML с помощью команд `pyxel app2exe` или `pyxel app2html`.

## Руководство по API

### Система

- `width`, `height`<br>
  Ширина и высота экрана

- `frame_count`<br>
  Количество прошедших кадров

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Инициализирует Pyxel-приложение с указанными размерами экрана (`width`, `height`). Дополнительно могут быть заданы: заголовок окна с помощью параметра `title`, количество кадров в секунду с помощью параметра `fps`, клавиша для выхода из приложения — `quit_key`, масштаб дисплея с помощью `display_scale`, коэффициент масштабирования при захвате экрана — `capture_scale` и максимальное время записи при захвате экрана с помощью `capture_sec`.<br>
  Пример: `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Запустить Pyxel-приложение, использующее функцию `update` для обновления внутренней логики и `draw` для рисования.

- `show()`<br>
  Отрисовать кадр и ждать выхода из приложения по нажатию клавиши `Esc`.

- `flip()`<br>
  Обновить экран на один кадр. Приложение завершится при нажатии клавиши `Esc`. Эта функция не работает в веб-версии.

- `quit()`<br>
  Завершить работу Pyxel-приложения.

### Ресурсы

- `load(filename, [excl_images], [excl_tilemaps], [excl_sounds], [excl_musics])`<br>
  Загрузить ресурсный файл (.pyxres). Если опция имеет значение `True`, соответствующий ресурс не будет загружен. Если файл палитры (.pyxpal) с таким же именем существует в том же месте, что и файл ресурса, цвета отображения палитры также будут обновлены. Файл палитры представляет собой шестнадцатеричную запись цветов отображения (например, `1100FF`), разделенных новыми строками. Файл палитры также может быть использован для изменения цветов, отображаемых в Pyxel Editor.

### Ввод

- `mouse_x`, `mouse_y`<br>
  Получить положение курсора мышки

- `mouse_wheel`<br>
  Получить значение колесика мышки

- `btn(клавиша)`<br>
  Получить `True`, если `клавиша` нажата, в противном случае получить `False`. ([Список определений клавиш](../python/pyxel/__init__.pyi))

- `btnp(клавиша, [hold], [repeat])`<br>
  Получить `True`, если `клавиша` нажата в данный кадр, в противном случае вернуть `False`. Если указаны параметры `hold` и `repeat`, `True` будет возвращаться через каждые `repeat` кадров, если клавиша удерживается более чем `hold` кадров.

- `btnr(клавиша)`<br>
  Вернуть `True`, если `клавиша` была отпущена в данный кадр, в противном случае вернуть `False`.

- `mouse(видна)`<br>
  Установить видимость курсора: если `visible` равно `True`, сделать виндым, если `False`, то невидимым. Даже если курсор не отображается, его позицию всё равно можно получить соответствующими функциями.

### Графика

- `colors`<br>
  Список цветов палитры. Цвет кодируется 24-битным целым числом. Используйте `colors.from_list` и `colors.to_list` для установки и получения списка Python.<br>
  Пример: `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Список банков изображений (0-2)<br>
  Пример: `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Список тайлмапов (0-7)

- `clip(x, y, w, h)`<br>
  Установить область рисования экрана от (`x`, `y`) шириной `w` и высотой `h`. Сбросить область рисования на весь экран можно с помощью вызова `clip()`.

- `camera(x, y)`<br>
  Установить верхний левый угол экрана на координаты (`x`, `y`). Сбросить на (`0`, `0`) можно с помощью вызова `camera()`.

- `pal(col1, col2)`<br>
  Поменять цвет `col1` с цветом `col2` во время рисования. Восстановить изначальную палитру можно с помощью `pal()`

- `dither(alpha)`<br>
  Применить дизеринг (псевдопрозрачность) при рисовании. Установите `alpha` в диапазоне от `0.0` до `1.0`, где `0.0` — прозрачный, а `1.0` — непрозрачный.

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

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Скопировать область размеров (`w`, `h`), по координатам (`u`, `v`) набора изображений `img`(0-2) по координатам (`x`, `y`) на экране. Если для `w` и/или `h` установлено отрицательное значение, изображение будет развернуто горизонтально и/или вертикально. Если указан параметр `colkey`, соответствующий цвет будет считаться цветом фона (прозрачным цветом). Если указаны `rotate`(в градусах), `scale`(1.0=100%) или оба значения, то будет применено соответствующее преобразование.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Нарисовать из тайлмапа `tm`(0-7) по координатам (`x`, `y`) тайл размером (`w`, `h`), находящийся по координатам (`u`, `v`). Если переданы отрицательные значения `w` и/или `h`, то изображение будет отражено по горизонтали и/или вертикали. Если указан параметр `colkey`, соответствующий цвет будет считаться цветом фона (прозрачным цветом). Если указаны `rotate`(в градусах), `scale`(1.0=100%) или оба значения, то будет применено соответствующее преобразование. Размер тайла равен 8x8 точек и хранится в карте тайлов в виде кортежа `(tile_x, tile_y)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  Нарисовать строку текста `s` цвета `col` по координате (`x`, `y`).

### Аудио

- `sounds`<br>
  Список звуков (0-63)<br>
  Пример: `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Список музыки (0-7)

- `play(ch, snd, [tick], [loop], [resume])`<br>
  Проигрывать звук `snd`(0-63) на канале `ch`(0-3). Если `snd` — список, звуки будут воспроизведены по порядку. Позиция начала воспроизведения может быть указана с помощью `tick`(1 тик = 1/120 секунды). Если в качестве значения `loop` передано `True`, проигрывание будет зациклено. Чтобы возобновить предыдущий звук после завершения воспроизведения, установите `resume` в `True`.

- `playm(msc, [tick], [loop])`<br>
  Проигрывает музыку `msc`(0-7). Позиция начала воспроизведения может быть указана с помощью `tick`(1 тик = 1/120 секунды). Если в качестве значения `loop` передано `True`, проигрывание будет зациклено.

- `stop([ch])`<br>
  Остановить воспроизведение на канале `ch`(0-3). Вызовите `stop()`, чтобы остановить воспроизведение на всех каналах.

- `play_pos(ch)`<br>
  Возвращает позицию воспроизведения на канале `ch`(0-3) в виде кортежа `(sound_no, note_no)`. Возвращает `None`, если воспроизведение остановлено.

### Математика

- `ceil(x)`<br>
  Возвращает наименьшее целое число, большее либо равное `x`.

- `floor(x)`<br>
  Возвращает наибольшее целое число, меньшее либо равное `x`.

- `sgn(x)`<br>
  Возвращает `1`, если `x` положительное, `0`, если оно равно `0`, и `-1`, если оно отрицательное.

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
  Установить изображение в точке (`x`, `y`) с помощью списка строк.<br>
  Пример: `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Загрузить файл изображения (PNG/GIF/JPEG) в точку (`x`, `y`).

- `pget(x, y)`<br>
  Получить цвет пикселя по координатам (`x`, `y`).

- `pset(x, y, col)`<br>
  Нарисовать пиксель цвета `col` по координатам (`x`, `y`).

### Класс Tilemap

- `width`, `height`<br>
  Ширина и высота тайлмапа

- `imgsrc`<br>
  Банк изображений (0-2), на который ссылается тайлмап

- `set(x, y, data)`<br>
  Установить тайлмап в точке (`x`, `y`) с помощью списка строк.<br>
  Пример: `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  Загрузить слой в порядке рисования `layer`(0-) из файла TMX (Tiled Map File) в точку (`x`, `y`).

- `pget(x, y)`<br>
  Получить тайл в координатах (`x`, `y`). Возвращаемое значение представляет собой кортеж `(tile_x, tile_y)`.

- `pset(x, y, tile)`<br>
  Задать тайл в координатах (`x`, `y`). Тайл передаётся в виде кортежа `(tile_x, tile_y)`.

### Класс Sound

- `notes`<br>
  Список нот (0-127). Чем больше значение, тем выше нота. Значение `33` соответствует ноте «ля» второй октавы 'A2' (440Hz). Паузы обозначаются значением `-1`.

- `tones`<br>
  Список тонов (0:Треугольник / 1:Квадрат / 2:Пульс / 3:Шум)

- `volumes`<br>
  Список громкости (0-7)

- `effects`<br>
  Список эффектов (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  Длительность воспроизведения. `1` — самая быстрая, чем выше значение, тем ниже скорость воспроизведения. При значении, равном `120` длительность воспроизведения одной ноты составляет 1 секунду.

- `set(notes, tones, volumes, effects, speed)`<br>
  Установить ноты, тоны, громкости и эффекты с помощью строки. Если длины тонов, громкостей или эффектов меньше длины нот, они будут повторяться с начала.

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
  Двумерный список музыки (0-63) с указанием количества каналов

- `set(seq0, seq1, seq2, ...)`<br>
  Установить списки музыки (0-63) для каждого канала. Если указан пустой список, этот канал не используется для воспроизведения.<br>
  Пример: `pyxel.musics[0].set([0, 1], [], [3])`

### Расширенный API

Pyxel включает в себя "Расширенный API", который не упоминается в этом руководстве, так как он может запутать пользователей или требует специализированных знаний для использования.

Если вы уверены в своих силах, попробуйте создать удивительные работы, используя [это](../python/pyxel/__init__.pyi) в качестве подсказки!

## Как внести вклад

### Отправка отчетов о проблемах

Используйте [трекер проблем](https://github.com/kitao/pyxel/issues) для отправки отчётов о проблемах или предложений по улучшению/добавлению новых возможностей. Перед созданием новой задачи убедитесь, что схожие открытые задачи отсутствуют.

### Функциональное тестирование

Ручное тестирование кода и написание отчетов о проблемах, предложений по улучшению в [трекере проблем](https://github.com/kitao/pyxel/issues) приветствуется!

### Отправка pull-запросов

Патчи и исправления принимаются в форме запросов на слияние (pull-запрос, PR). Убедитесь, что проблема, к которой относится запрос на слияние, открыта в трекере проблем.

Отправленный pull request считается согласием на публикацию по [лицензии MIT](../LICENSE)

## Прочая информация

- [FAQ](faq-en.md)
- [Примеры пользователей](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Учетная запись разработчика на X](https://x.com/kitao)

## Лицензия

Pyxel распространяется под [лицензией MIT](../LICENSE). Его можно использовать в проприетарном программном обеспечении при условии, что все копии программного обеспечения или его значительные части включают копию условий лицензии MIT и уведомление об авторских правах.

## Набор спонсоров

Pyxel ищет спонсоров на GitHub Sponsors. Рассмотрите возможность спонсирования Pyxel для поддержки его дальнейшего обслуживания и разработки новых функций. Спонсоры могут проконсультироваться напрямую с разработчиком Pyxel. Подробнее см. [Здесь](https://github.com/sponsors/kitao).
