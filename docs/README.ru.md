# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** (/ˈpɪksəl/) — это ретро-игровой движок для Python.

Спецификации вдохновлены ретро-игровыми консолями, такими как поддержка только 16 цветов и 4 звуковых канала, что позволяет легко наслаждаться созданием игр в стиле пиксельной графики.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

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
- Настраиваемый размер экрана
- 16-цветная палитра
- 3 банка изображений размером 256x256
- 8 тайловых карт размером 256x256
- 4 канала с 64 настраиваемыми звуками
- 8 музыкальных треков, которые могут сочетать любые звуки
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
<td>Shoot'em up с переходами между экранами и MML</td>
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
<td>17_app_launcher.py</td>
<td>Pyxel app launcher (вы можете играть в различные игры!)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/17_app_launcher.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/17_app_launcher.py">Code</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>Анимация с помощью функции flip (только для не веб-платформ)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Демонстрация</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Код</a></td>
</tr>
</table>

Примеры можно выполнить с помощью следующих команд:

```sh
# Run sample in examples directory
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# Run app in examples/apps directory
cd apps
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
- `Alt(Option)+R` или `A+B+X+Y+BACK` на геймпаде<br>
  Сбросить приложение
- `Alt(Option)+1`<br>
  Сохранить снимок экрана на рабочий стол
- `Alt(Option)+2`<br>
  Сбросить время начала записи видео с экрана
- `Alt(Option)+3`<br>
  Сохранить видео захвата экрана на рабочий стол (до 10 секунд)
- `Alt(Option)+8` или `A+B+X+Y+DL` на геймпаде<br>
  Переключает масштаб экрана между максимальным и целочисленным
- `Alt(Option)+9` или `A+B+X+Y+DR` на геймпаде<br>
  Переключение между режимами экрана (Crisp/Smooth/Retro)
- `Alt(Option)+0` или `A+B+X+Y+DU` на геймпаде<br>
  Переключить монитор производительности (FPS/`update` время/`draw` время)
- `Alt(Option)+Enter` или `A+B+X+Y+DD` на геймпаде<br>
  Переключить полноэкранный режим
- `Shift+Alt(Option)+1/2/3`<br>
  Сохранить банк изображений 0, 1 или 2 на рабочий стол
- `Shift+Alt(Option)+0`<br>
  Сохранить текущую цветовую палитру на рабочий стол

### Как создавать ресурсы

Pyxel Editor может создавать изображения и звуки, используемые в приложении Pyxel.

Вы можете запустить Pyxel Editor с помощью следующей команды:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Если указанный файл ресурсов Pyxel (.pyxres) существует, он будет загружен. Если его нет, будет создан новый файл с указанным именем. Если файл ресурсов пропущен, будет создан новый файл с именем `my_resource.pyxres`.

После запуска Pyxel Editor вы можете переключаться между файлами ресурсов, перетаскивая и бросая их на Pyxel Editor.

Созданный файл ресурсов можно загрузить с помощью функции `load`.

Pyxel Editor имеет следующие режимы редактирования.

**Редактор изображений**

Режим для редактирования изображения в каждом **банке изображений**.

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

Вы можете перетаскивать и бросать файл изображения (PNG/GIF/JPEG) в редактор изображений, чтобы загрузить изображение в выбранный в данный момент банк изображений.

**Редактор тайловых карт**

Режим для редактирования **тайловых карт**, в которых изображения банков изображений располагаются в плиточном порядке.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

Перетащите файл TMX (Tiled Map File) в редактор тайловых карт, чтобы загрузить его слой 0 в текущую выбранную тайловую карту.

**Редактор звуков**

Режим редактирования **звуков**, используемых для мелодий и звуковых эффектов.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**Редактор музыки**

Режим для редактирования **музыки**, в которых звуки расположены в порядке воспроизведения.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### Другие методы создания ресурсов

Изображения и тайловые карты Pyxel также можно создавать с помощью следующих методов:

- Создайте изображение из списка строк, используя функцию `Image.set` или функцию `Tilemap.set`
- Загрузите файл изображения (PNG/GIF/JPEG) в палитру Pyxel с помощью функции `Image.load`

Звуки Pyxel также можно создать с помощью следующего метода:

- Создайте звук из строк с помощью функции `Sound.set` или функции `Music.set`

Справьтесь к справочнику API для использования этих функций.

### Как распространять приложения

Pyxel поддерживает специальный формат файла для распространения приложений (файл приложения Pyxel), который работает на различных платформах.

Файл приложения Pyxel (.pyxapp) создается с помощью команды `pyxel package`:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Если вам нужно включить ресурсы или дополнительные модули, разместите их в каталоге приложения.

Метаданные могут отображаться во время выполнения, если указать их в следующем формате в стартовом скрипте. Поля, кроме `title` и `author`, являются необязательными.

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

Файл приложения Pyxel также можно преобразовать в исполняемый файл или HTML-файл с помощью команд `pyxel app2exe` или `pyxel app2html`.

## Справочник по API

### Система

- `width`, `height`<br>
  Ширина и высота экрана

- `frame_count`<br>
  Количество прошедших кадров

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Инициализация приложения Pyxel с размером экрана (`width`, `height`). Можно указать следующие параметры: заголовок окна с помощью `title`, частоту кадров с помощью `fps`, клавишу выхода из приложения с помощью `quit_key`, масштаб отображения с помощью `display_scale`, масштаб захвата экрана с помощью `capture_scale` и максимальное время записи видео захвата экрана с помощью `capture_sec`.<br>
  Пример: `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Запуск приложения Pyxel и вызов функции `update` для обновления кадров и функции `draw` для отрисовки.

- `show()`<br>
  Отображение экрана и ожидание нажатия клавиши `Esc`.

- `flip()`<br>
  Обновление экрана на один кадр. Приложение завершится при нажатии клавиши `Esc`. Эта функция недоступна в веб-версии.

- `quit()`<br>
  Завершение работы приложения Pyxel.

- `reset()`<br>
  Сбрасывает приложение Pyxel. Переменные окружения сохраняются после сброса.

### Ресурсы

- `load(filename, [exclude_images], [exclude_tilemaps], [exclude_sounds], [exclude_musics])`<br>
  Загрузка файла ресурсов (.pyxres). Если опция установлена в `True`, соответствующий ресурс будет исключен из загрузки. Если файл палитры (.pyxpal) с таким же именем существует в той же директории, что и файл ресурсов, цвета палитры также будут обновлены. Файл палитры содержит шестнадцатеричные значения для цветов отображения (например, `1100ff`), разделенные новыми строками. Файл палитры также можно использовать для изменения цветов, отображаемых в Pyxel Editor.

- `user_data_dir(vendor_name, app_name)`<br>
  Возвращает каталог для сохранения пользовательских данных, созданный на основе `vendor_name` и `app_name`. Если каталог не существует, он будет создан автоматически. Используется для сохранения рекордов, прогресса в игре и аналогичных данных.<br>
  Пример: `print(pyxel.user_data_dir("Takashi Kitao", "Pyxel Shooter"))`

### Ввод

- `mouse_x`, `mouse_y`<br>
  Текущие координаты курсора мыши

- `mouse_wheel`<br>
  Текущее значение колесика мыши

- `btn(key)`<br>
  Возвращает `True`, если клавиша `key` нажата, иначе возвращает `False`. ([Список определений клавиш](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  Возвращает `True`, если клавиша `key` нажата в данном кадре, иначе возвращает `False`. Если указаны `hold` и `repeat`, то после удержания клавиши `key` в течение `hold` кадров или более, `True` будет возвращаться каждые `repeat` кадров.

- `btnr(key)`<br>
  Возвращает `True`, если клавиша `key` была отпущена в данном кадре, иначе возвращает `False`.

- `mouse(visible)`<br>
  Показывает курсор мыши, если `visible` равно `True`, и скрывает его, если `visible` равно `False`. Координаты курсора продолжают обновляться даже при его скрытии.

### Графика

- `colors`<br>
  Список цветов палитры отображения. Цвет отображения задается 24-битным числовым значением. Используйте `colors.from_list` и `colors.to_list` для прямого назначения и извлечения списков Python.<br>
  Пример: `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Список банков изображений (экземпляры класса Image) (0-2)<br>
  Пример: `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Список карт тайлов (экземпляры класса Tilemap) (0-7)

- `clip(x, y, w, h)`<br>
  Устанавливает область рисования на экране от координат (`x`, `y`) с шириной `w` и высотой `h`. Вызов `clip()` сбрасывает область рисования на весь экран.

- `camera(x, y)`<br>
  Изменяет координаты верхнего левого угла экрана на (`x`, `y`). Вызов `camera()` сбрасывает координаты верхнего левого угла на (`0`, `0`).

- `pal(col1, col2)`<br>
  Заменяет цвет `col1` на `col2` при рисовании. Вызов `pal()` сбрасывает палитру к исходному состоянию.

- `dither(alpha)`<br>
  Применяет дизеринг (псевдо-прозрачность) при рисовании. Установите значение `alpha` в диапазоне от `0.0` до `1.0`, где `0.0` — это прозрачность, а `1.0` — это непрозрачность.

- `cls(col)`<br>
  Очищает экран цветом `col`.

- `pget(x, y)`<br>
  Получает цвет пикселя в координатах (`x`, `y`).

- `pset(x, y, col)`<br>
  Рисует пиксель цвета `col` в координатах (`x`, `y`).

- `line(x1, y1, x2, y2, col)`<br>
  Рисует линию цвета `col` от координат (`x1`, `y1`) до (`x2`, `y2`).

- `rect(x, y, w, h, col)`<br>
  Рисует прямоугольник шириной `w`, высотой `h` и цветом `col` от координат (`x`, `y`).

- `rectb(x, y, w, h, col)`<br>
  Рисует контур прямоугольника шириной `w`, высотой `h` и цветом `col` от координат (`x`, `y`).

- `circ(x, y, r, col)`<br>
  Рисует круг радиусом `r` и цветом `col` в координатах (`x`, `y`).

- `circb(x, y, r, col)`<br>
  Рисует контур круга радиусом `r` и цветом `col` в координатах (`x`, `y`).

- `elli(x, y, w, h, col)`<br>
  Рисует эллипс шириной `w`, высотой `h` и цветом `col` от координат (`x`, `y`).

- `ellib(x, y, w, h, col)`<br>
  Рисует контур эллипса шириной `w`, высотой `h` и цветом `col` от координат (`x`, `y`).

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  Рисует треугольник с вершинами в координатах (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) и цветом `col`.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  Рисует контур треугольника с вершинами в координатах (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) и цветом `col`.

- `fill(x, y, col)`<br>
  Заполняет область, соединенную с таким же цветом, как и в координатах (`x`, `y`), цветом `col`.

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Копирует регион размером (`w`, `h`) от координат (`u`, `v`) из банка изображений `img`(0-2) в координаты (`x`, `y`). Если задано отрицательное значение для `w` и/или `h`, регион будет перевернут по горизонтали и/или вертикали. Если указан `colkey`, он будет считаться прозрачным цветом. Если заданы `rotate` (в градусах), `scale` (1.0 = 100%) или оба параметра, будут применены соответствующие преобразования.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Копирует регион размером (`w`, `h`) от координат (`u`, `v`) из карты тайлов `tm`(0-7) в координаты (`x`, `y`). Если задано отрицательное значение для `w` и/или `h`, регион будет перевернут по горизонтали и/или вертикали. Если указан `colkey`, он будет считаться прозрачным цветом. Если заданы `rotate` (в градусах), `scale` (1.0 = 100%) или оба параметра, будут применены соответствующие преобразования. Размер одного тайла составляет 8x8 пикселей и хранится в карте тайлов в виде кортежа `(image_tx, image_ty)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  Рисует строку `s` цвета `col` в координатах (`x`, `y`).

### Аудио

- `sounds`<br>
  Список звуков (экземпляры класса Sound) (0-63)<br>
  Пример: `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Список музыки (экземпляры класса Music) (0-7)

- `play(ch, snd, [sec], [loop], [resume])`<br>
  Воспроизводит звук `snd`(0-63) на канале `ch`(0-3). `snd` может быть номером звука, списком номеров звуков или MML-строкой. Начальная позиция воспроизведения может быть указана в секундах с помощью параметра `sec`. Если параметр `loop` установлен в значение `True`, воспроизведение будет зациклено. Чтобы возобновить предыдущий звук после завершения воспроизведения, установите `resume` в значение `True`.

- `playm(msc, [sec], [loop])`<br>
  Воспроизводит музыку `msc`(0-7). Начальная позиция воспроизведения может быть указана в секундах с помощью параметра `sec`. Если параметр `loop` установлен в значение `True`, музыка будет воспроизводиться в цикле.

- `stop([ch])`<br>
  Останавливает воспроизведение на указанном канале `ch`(0-3). Вызов функции `stop()` останавливает воспроизведение на всех каналах.

- `play_pos(ch)`<br>
  Возвращает текущую позицию воспроизведения звука на канале `ch`(0-3) в виде кортежа `(sound_no, sec)`. Возвращает `None`, если воспроизведение остановлено.

### Математика

- `ceil(x)`<br>
  Возвращает наименьшее целое число, большее или равное `x`.

- `floor(x)`<br>
  Возвращает наибольшее целое число, меньшее или равное `x`.

- `sgn(x)`<br>
  Возвращает `1`, если `x` положительное, `0`, если равно `0`, и `-1`, если отрицательное.

- `sqrt(x)`<br>
  Возвращает квадратный корень числа `x`.

- `sin(deg)`<br>
  Возвращает синус угла в градусах `deg`.

- `cos(deg)`<br>
  Возвращает косинус угла в градусах `deg`.

- `atan2(y, x)`<br>
  Возвращает арктангенс отношения `y` к `x` в градусах.

- `rseed(seed)`<br>
  Устанавливает значение семени для генератора случайных чисел.

- `rndi(a, b)`<br>
  Возвращает случайное целое число от `a` до `b` включительно.

- `rndf(a, b)`<br>
  Возвращает случайное вещественное число от `a` до `b` включительно.

- `nseed(seed)`<br>
  Устанавливает значение семени для Perlin-шума.

- `noise(x, [y], [z])`<br>
  Возвращает значение Perlin-шума для указанных координат.

### Класс Image

- `width`, `height`<br>
  Ширина и высота изображения

- `set(x, y, data)`<br>
  Устанавливает изображение в координатах (`x`, `y`) с использованием списка строк.<br>
  Пример: `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Загружает файл изображения (PNG/GIF/JPEG) в координаты (`x`, `y`).

- `pget(x, y)`<br>
  Получает цвет пикселя в координатах (`x`, `y`).

- `pset(x, y, col)`<br>
  Рисует пиксель цвета `col` в координатах (`x`, `y`).

### Класс Tilemap

- `width`, `height`<br>
  Ширина и высота карты тайлов

- `imgsrc`<br>
  Банк изображений (0-2), на который ссылается карта тайлов

- `set(x, y, data)`<br>
  Устанавливает карту тайлов в координатах (`x`, `y`) с использованием списка строк.<br>
  Пример: `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  Загружает `layer`(0-) из файла TMX (Tiled Map File) в координаты (`x`, `y`).

- `pget(x, y)`<br>
  Получает тайл в координатах (`x`, `y`). Тайлы представлены в виде кортежа `(image_tx, image_ty)`.

- `pset(x, y, tile)`<br>
  Рисует `tile` в координатах (`x`, `y`). Тайл представлен в виде кортежа `(image_tx, image_ty)`.

### Класс Sound

- `notes`<br>
  Список нот (0-127). Чем больше число, тем выше звук. Нота `33` соответствует 'A2'(440Hz). Паузы обозначаются значением `-1`.

- `tones`<br>
  Список тонов (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
  Список громкостей (0-7)

- `effects`<br>
  Список эффектов (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  Скорость воспроизведения. `1` — самая быстрая, и чем больше число, тем медленнее воспроизведение. При значении `120` длительность одной ноты составляет 1 секунду.

- `set(notes, tones, volumes, effects, speed)`<br>
  Устанавливает ноты, тоны, громкости и эффекты с помощью строки. Если длина тонов, громкостей или эффектов меньше, чем у нот, они будут повторяться с начала.

- `set_notes(notes)`<br>
  Устанавливает ноты с помощью строки, состоящей из символов `CDEFGAB`+`#-`+`01234` или `R`. Регистр не имеет значения, а пробелы игнорируются.<br>
  Пример: `pyxel.sounds[0].set_notes("g2b-2d3r rf3f3f3")`

- `set_tones(tones)`<br>
  Устанавливает тоны с помощью строки, состоящей из символов `TSPN`. Регистр не имеет значения, а пробелы игнорируются.<br>
  Пример: `pyxel.sounds[0].set_tones("ttss pppn")`

- `set_volumes(volumes)`<br>
  Устанавливает громкости с помощью строки, состоящей из символов `01234567`. Регистр не имеет значения, а пробелы игнорируются.<br>
  Пример: `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  Устанавливает эффекты с помощью строки, состоящей из символов `NSVFHQ`. Регистр не имеет значения, а пробелы игнорируются.<br>
  Пример: `pyxel.sounds[0].set_effects("nfnf nvvs")`

- `mml(code)`<br>
  Передача строки [MML (Music Macro Language)](https://en.wikipedia.org/wiki/Music_Macro_Language) переводит в режим MML и воспроизводит звук в соответствии с её содержимым. В этом режиме обычные параметры, такие как `notes` и `speed`, игнорируются. Чтобы выйти из режима MML, вызовите `mml()` без аргументов. Подробнее о MML см. [на этой странице](faq-en.md).<br>
  Пример: `pyxel.sounds[0].mml("T120 Q90 @1 V100 O5 L8 C4&C<G16R16>C.<G16 >C.D16 @VIB1{10,20,20} E2C2")`

- `save(filename, sec, [ffmpeg])`<br>
  Создает WAV-файл, который воспроизводит звук в течение указанного количества секунд. Если FFmpeg установлен и `ffmpeg` установлен в значение `True`, также создается MP4-файл.

- `total_sec()`<br>
  Возвращает время воспроизведения звука в секундах. Возвращает `None`, если в MML используется бесконечный цикл.

### Класс Music

- `seqs`<br>
  Двумерный список звуков (0-63) для нескольких каналов

- `set(seq0, seq1, seq2, ...)`<br>
  Устанавливает списки звуков (0-63) для каждого канала. Если указан пустой список, этот канал не будет использоваться для воспроизведения.<br>
  Пример: `pyxel.musics[0].set([0, 1], [], [3])`

- `save(filename, sec, [ffmpeg])`<br>
  Создает WAV-файл, который воспроизводит музыку в течение указанного количества секунд. Если FFmpeg установлен и `ffmpeg` установлен в значение `True`, также создается MP4-файл.

### Расширенный API

Pyxel включает в себя "Расширенный API", который не упоминается в данной справке, так как он может сбивать пользователей с толку или требовать специализированных знаний для использования.

Если вы уверены в своих навыках, попробуйте создать удивительные 作品, используя [это](../python/pyxel/__init__.pyi) в качестве руководства!

## Как внести вклад

### Подавать проблемы

Используйте [Трекер проблем](https://github.com/kitao/pyxel/issues) для подачи отчетов об ошибках и запросов на функции или улучшения. Перед отправкой новой проблемы убедитесь, что нет похожих открытых вопросов.

### Функциональное тестирование

Любой, кто вручную тестирует код и сообщает об ошибках или предлагает улучшения в [Трекере проблем](https://github.com/kitao/pyxel/issues), очень приветствуется!

### Подавать запросы на изменение

Патчи и исправления принимаются в форме запросов на изменение (PR). Убедитесь, что проблема, на которую ссылается запрос на изменение, открыта в Трекере проблем.

Подача запроса на изменение подразумевает, что вы соглашаетесь лицензировать свой вклад по [Лицензии MIT](../LICENSE).

## Дополнительная информация

- [Часто задаваемые вопросы](faq-en.md)
- [Примеры пользователей](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [X-аккаунт разработчика](https://x.com/kitao)
- [Сервер Discord (Английский)](https://discord.gg/Z87eYHN)
- [Сервер Discord (Японский)](https://discord.gg/qHA5BCS)

## Лицензия

Pyxel лицензируется по [Лицензии MIT](../LICENSE). Его можно использовать в проприетарном программном обеспечении, при условии, что все копии программного обеспечения или его значительные части содержат копию условий лицензии MIT и уведомление о авторских правах.

## Поиск спонсоров

Pyxel ищет спонсоров на GitHub Sponsors. Пожалуйста, подумайте о том, чтобы стать спонсором Pyxel, чтобы поддержать его дальнейшую поддержку и развитие функций. В качестве преимущества спонсоры могут консультироваться непосредственно с разработчиком Pyxel. Для получения дополнительной информации посетите [эту страницу](https://github.com/sponsors/kitao).
