# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** (/ˈpɪksəl/) — це ретро-ігровий движок для Python.

Специфікації натхнені ретро-ігровими консолями, такими як підтримка лише 16 кольорів та 4 звукових канали, що дозволяє легко насолоджуватися створенням ігор у стилі піксельної графіки.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

Розробка Pyxel здійснюється на основі відгуків користувачів. Будь ласка, поставте Pyxel зірку на GitHub!

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

Специфікації та API Pyxel були натхненні [PICO-8](https://www.lexaloffle.com/pico-8.php) та [TIC-80](https://tic80.com/).

Pyxel є відкритим програмним забезпеченням під [ліцензією MIT](../LICENSE) та безкоштовний для використання. Давайте почнемо створювати ретро-ігри з Pyxel!

## Специфікації

- Працює на Windows, Mac, Linux та Web
- Програмування на Python
- Налаштовуваний розмір екрану
- 16-кольорова палітра
- 3 банки зображень розміром 256x256
- 8 тайлових карт розміром 256x256
- 4 канали з 64 налаштовуваними звуками
- 8 музичних треків, які можуть поєднувати будь-які звуки
- Введення з клавіатури, миші та ігрового контролера
- Інструменти для редагування зображень та звуків
- Розширювані користувачем кольори, канали та банки

### Кольорова палитра

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Як встановити

### Windows

Після встановлення [Python3](https://www.python.org/) (версія 3.8 або вища) виконайте наступну команду:

```sh
pip install -U pyxel
```

При установці Python за допомогою офіційного установника, переконайтеся, що ви вибрали опцію `Add Python 3.x to PATH`, щоб активувати команду `pyxel`.

### Mac

Після встановлення [Homebrew](https://brew.sh/) виконайте наступні команди:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Щоб оновити Pyxel після установки, виконайте `pipx upgrade pyxel`.

### Linux

Після установки пакета SDL2 (`libsdl2-dev` для Ubuntu), [Python3](https://www.python.org/) (версія 3.8 або вища) та `python3-pip`, виконайте наступну команду:

```sh
sudo pip3 install -U pyxel
```

Якщо попередня команда не спрацювала, подумайте про збірку Pyxel з виходу, дотримуючись інструкцій у [Makefile](../Makefile).

### Web

Веб-версія Pyxel може використовуватися на ПК, а також на смартфонах і планшетах, за умови наявності сумісного веб-браузера, без встановлення Python або Pyxel.

Найпростіший спосіб використовувати її — через онлайн-IDE [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/).

Для інших моделей використання, таких як вбудовування додатків Pyxel на ваш власний сайт, зверніться до [цієї сторінки](pyxel-web-en.md).

### Запустити приклади

Після установки Pyxel ви можете скопіювати приклади в поточний каталог за допомогою наступної команди:

```sh
pyxel copy_examples
```

Наступні приклади будуть скопійовані у ваш поточний каталог:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>Найпростіший додаток</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01-hello-pyxel.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Код</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Гра зі стрибками з використанням файлу ресурсів Pyxel</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02-jump-game.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Код</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Демонстрація API для малювання</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03-draw-api.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Код</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Демонстрація API для звуку</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04-sound-api.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Код</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>Список кольорових паліт</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05-color-palette.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Код</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>Гра на клік миші</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06-click-game.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Код</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>Гра «Змійка» з BGM</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07-snake.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Код</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Демонстрація API для малювання трикутників</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08-triangle-api.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Код</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Shoot'em up з переходами між екранами та MML</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09-shooter.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Код</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>Горизонтальна платформна гра з картою</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10-platformer.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Код</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Відображення поза екраном з класом Image</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11-offscreen.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Код</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>Анімація Перлін-шуму</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12-perlin-noise.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Код</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>Малювання бітмап-шрифта</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13-bitmap-font.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Код</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>Синтезатор з використанням функцій розширення звуку</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14-synthesizer.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Код</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>Завантаження та малювання Tiled Map File (.tmx)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15-tiled-map-file.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Код</a></td>
</tr>
<tr>
<td>16_transform.py</td>
<td>Обертання та масштабування зображень</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/16-transform.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/16_transform.py">Код</a></td>
</tr>
<tr>
<td>17_app_launcher.py</td>
<td>Pyxel app launcher (ви можете грати в різні ігри!)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/17-app-launcher.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/17_app_launcher.py">Code</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>Анімація за допомогою функції flip (тільки для платформ, що не є веб)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Код</a></td>
</tr>
</table>

Приклади можна виконати за допомогою наступних команд:

```sh
# Run example in examples directory
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# Run app in examples/apps directory
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

## Як використовувати

### Створення програми

У вашому Python-скрипті імпортуйте модуль Pyxel, вкажіть розмір вікна за допомогою функції `init`, а потім запустіть програму Pyxel за допомогою функції `run`.

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

Аргументи функції `run` — це функція `update`, яка обробляє оновлення кадрів, та функція `draw`, яка відповідає за малювання на екрані.

У реальному застосуванні рекомендується обернути код Pyxel в клас, як показано нижче:

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

Для створення простих графіків без анімації ви можете використовувати функцію `show`, щоб спростити ваш код.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Запуск програми

Створений скрипт можна виконати за допомогою команди `python`:

```sh
python PYTHON_SCRIPT_FILE
```

Його також можна запустити за допомогою команди `pyxel run`:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Крім того, команда `pyxel watch` відстежує зміни в зазначеній директорії та автоматично перезапускає програму при виявленні змін:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

Спостереження за директорією можна зупинити, натиснувши `Ctrl(Command)+C`.

### Спеціальні клавіші

Під час виконання програми Pyxel можна виконати наступні спеціальні дії з клавішами:

- `Esc`<br>
  Вийти з додатку
- `Alt(Option)+R` або `A+B+X+Y+BACK` на геймпаді<br>
  Скинути додаток
- `Alt(Option)+1`<br>
  Зберегти знімок екрану на робочий стіл
- `Alt(Option)+2`<br>
  Скинути час початку запису відео з екрану
- `Alt(Option)+3`<br>
  Зберегти відео захоплення екрану на робочий стіл (до 10 секунд)
- `Alt(Option)+8` або `A+B+X+Y+DL` на геймпаді<br>
  Перемикати масштаб екрану між максимальним та цілим
- `Alt(Option)+9` або `A+B+X+Y+DR` на геймпаді<br>
  Перемикати режими екрану (Crisp/Smooth/Retro)
- `Alt(Option)+0` або `A+B+X+Y+DU` на геймпаді<br>
  Перемикати монітор продуктивності (FPS/час `update`/час `draw`)
- `Alt(Option)+Enter` або `A+B+X+Y+DD` на геймпаді<br>
  Перемикати повноекранний режим
- `Shift+Alt(Option)+1/2/3`<br>
  Зберегти банк зображень 0, 1 або 2 на робочий стіл
- `Shift+Alt(Option)+0`<br>
  Зберегти поточну кольорову палітру на робочий стіл

### Як створити ресурси

Pyxel Editor може створювати зображення та звуки, що використовуються в програмі Pyxel.

Ви можете запустити Pyxel Editor за допомогою наступної команди:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Якщо зазначений файл ресурсу Pyxel (.pyxres) існує, він буде завантажений. Якщо ні, то буде створено новий файл з вказаним ім'ям. Якщо файл ресурсу пропущено, буде створено новий файл з назвою `my_resource.pyxres`.

Після запуску Pyxel Editor ви можете перемикатися на інший файл ресурсу, перетягуючи його на Pyxel Editor.

Створений файл ресурсу можна завантажити за допомогою функції `load`.

Pyxel Editor має такі режими редагування.

**Редактор зображень**

Режим для редагування зображення в кожному **банку зображень**.

<a href="https://kitao.github.io/pyxel/wasm/examples/image-editor.html">
<img src="images/image_editor.gif">
</a>

Ви можете перетягувати файл зображення (PNG/GIF/JPEG) у редактор зображень, щоб завантажити зображення в поточний вибраний банк зображень.

**Редактор тайлових карт**

Режим для редагування **карти плиток**, де зображення з банків зображень розташовані в плитковому шаблоні.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap-editor.html">
<img src="images/tilemap_editor.gif">
</a>

Перетягніть файл TMX (Tiled Map File) на редактор карт плиток, щоб завантажити його шар 0 у поточну вибрану карту плиток.

**Редактор звуку**

Режим для редагування **звуків**, що використовуються для мелодій і звукових ефектів。

<a href="https://kitao.github.io/pyxel/wasm/examples/sound-editor.html">
<img src="images/sound_editor.gif">
</a>

**Редактор музики**

Режим для редагування **музик**, в яких звуки розташовані в порядку відтворення.

<a href="https://kitao.github.io/pyxel/wasm/examples/music-editor.html">
<img src="images/music_editor.gif">
</a>

### Інші методи створення ресурсів

Зображення та тайлових карт Pyxel також можна створювати за допомогою таких методів:

- Створіть зображення з списку рядків за допомогою функції `Image.set` або `Tilemap.set`
- Завантажте файл зображення (PNG/GIF/JPEG) з палітрою Pyxel за допомогою функції `Image.load`

Звуки Pyxel також можна створити за допомогою наступного методу:

- Створіть звук з рядків за допомогою функції `Sound.set` або `Music.set`

Зверніться до документації API для використання цих функцій.

### Як розповсюджувати програми

Pyxel підтримує спеціальний формат файлу для розподілу програм (файл програми Pyxel), який є кросплатформеним.

Файл програми Pyxel (.pyxapp) створюється за допомогою команди `pyxel package`:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Якщо потрібно включити ресурси або додаткові модулі, розмістіть їх у каталозі програми.

Метадані можна відображати під час виконання, вказуючи їх у наступному форматі в скрипті запуску. Поля, крім `title` і `author`, є необов'язковими.

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

Створений файл програми можна запустити за допомогою команди `pyxel play`:

```sh
pyxel play PYXEL_APP_FILE
```

Файл програми Pyxel також можна конвертувати в виконуваний файл або HTML-файл за допомогою команд `pyxel app2exe` або `pyxel app2html`.

## Довідка з API

### Система

- `width`, `height`<br>
  Ширина та висота екрану

- `frame_count`<br>
  Кількість пройдених кадрів

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Ініціалізує Pyxel-додаток з розміром екрану (`width`, `height`). Можна вказати такі параметри: заголовок вікна через `title`, частоту кадрів через `fps`, клавішу для завершення програми через `quit_key`, масштаб відображення через `display_scale`, масштаб захоплення екрану через `capture_scale` та максимальний час запису відео через `capture_sec`.<br>
  Приклад: `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Запускає Pyxel-додаток і викликає функцію `update` для оновлення кадрів та функцію `draw` для малювання.

- `show()`<br>
  Відображає екран та очікує на натискання клавіші `Esc`.

- `flip()`<br>
  Оновлює екран на один кадр. Програма завершується при натисканні клавіші `Esc`. Ця функція недоступна у веб-версії.

- `quit()`<br>
  Завершує Pyxel-додаток.

- `reset()`<br>
  Скидає Pyxel-додаток. Змінні середовища зберігаються після скидання.

### Ресурси

- `load(filename, [exclude_images], [exclude_tilemaps], [exclude_sounds], [exclude_musics])`<br>
  Завантажує файл ресурсу (.pyxres). Якщо для параметра вказано `True`, відповідний ресурс буде виключено з завантаження. Якщо файл палітри (.pyxpal) з таким самим іменем знаходиться у тому самому місці, що й файл ресурсу, кольори відображення палітри також будуть оновлені. Файл палітри містить кольори у шістнадцятковому форматі (наприклад, `1100ff`), розділені новими рядками. Файл палітри також можна використовувати для зміни кольорів у редакторі Pyxel.

- `user_data_dir(vendor_name, app_name)`<br>
  Повертає каталог користувацьких даних, створений на основі `vendor_name` і `app_name`. Якщо каталог не існує, він буде створений автоматично. Використовується для збереження результатів, прогресу в грі та подібних даних.<br>
  Приклад: `print(pyxel.user_data_dir("Takashi Kitao", "Pyxel Shooter"))`

### Введення

- `mouse_x`, `mouse_y`<br>
  Поточна позиція курсору миші

- `mouse_wheel`<br>
  Поточне значення колеса миші

- `btn(key)`<br>
  Повертає `True`, якщо клавішу `key` натиснуто, інакше повертає `False`. ([Список визначень клавіш](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  Повертає `True`, якщо клавішу `key` було натиснуто в цьому кадрі, інакше повертає `False`. Якщо вказано `hold` та `repeat`, після того, як клавішу `key` було натиснуто протягом більше ніж `hold` кадрів, `True` буде повертатися кожні `repeat` кадрів.

- `btnr(key)`<br>
  Повертає `True`, якщо клавішу `key` було відпущено в цьому кадрі, інакше повертає `False`.

- `mouse(visible)`<br>
  Показує курсор миші, якщо параметр `visible` встановлено у `True`, та приховує, якщо у `False`. Позиція курсору продовжує оновлюватися, навіть якщо він прихований.

### Графіка

- `colors`<br>
  Список кольорів палітри. Кольори відображення вказуються у 24-бітному числовому значенні. Використовуйте `colors.from_list` та `colors.to_list` для безпосереднього присвоєння та отримання списків у Python.<br>
  Приклад: `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Список банків зображень (екземпляри класу Image) (0-2)<br>
  Приклад: `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Список тайлмапів (екземпляри класу Tilemap) (0-7)
  Список тайлмапів (0-7)

- `clip(x, y, w, h)`<br>
  Встановлює область малювання на екрані від (`x`, `y`) з шириною `w` та висотою `h`. Викличте `clip()`, щоб скинути область малювання на весь екран.

- `camera(x, y)`<br>
  Змінює координати верхнього лівого кута екрана на (`x`, `y`). Викличте `camera()`, щоб скинути координати верхнього лівого кута на (`0`, `0`).

- `pal(col1, col2)`<br>
  Заміщує колір `col1` на `col2` при малюванні. Викличте `pal()`, щоб скинути палітру на початкову.

- `dither(alpha)`<br>
  Застосовує дезеринг (імітацію прозорості) при малюванні. Встановіть `alpha` у діапазоні від `0.0` до `1.0`, де `0.0` є прозорим, а `1.0` — непрозорим.

- `cls(col)`<br>
  Очищає екран кольором `col`.

- `pget(x, y)`<br>
  Повертає колір пікселя на позиції (`x`, `y`).

- `pset(x, y, col)`<br>
  Малює піксель кольору `col` на позиції (`x`, `y`).

- `line(x1, y1, x2, y2, col)`<br>
  Малює лінію кольору `col` від точки (`x1`, `y1`) до точки (`x2`, `y2`).

- `rect(x, y, w, h, col)`<br>
  Малює прямокутник шириною `w`, висотою `h` і кольором `col` від позиції (`x`, `y`).

- `rectb(x, y, w, h, col)`<br>
  Малює контур прямокутника шириною `w`, висотою `h` і кольором `col` від позиції (`x`, `y`).

- `circ(x, y, r, col)`<br>
  Малює коло радіусом `r` і кольором `col` на позиції (`x`, `y`).

- `circb(x, y, r, col)`<br>
  Малює контур кола радіусом `r` і кольором `col` на позиції (`x`, `y`).

- `elli(x, y, w, h, col)`<br>
  Малює еліпс шириною `w`, висотою `h` і кольором `col` від позиції (`x`, `y`).

- `ellib(x, y, w, h, col)`<br>
  Малює контур еліпса шириною `w`, висотою `h` і кольором `col` від позиції (`x`, `y`).

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  Малює трикутник із вершинами в точках (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) і кольором `col`.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  Малює контур трикутника із вершинами в точках (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) і кольором `col`.

- `fill(x, y, col)`<br>
  Заповнює область, яка має той самий колір, що й позиція (`x`, `y`), кольором `col`.

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Копіює область розміром (`w`, `h`) із позиції (`u`, `v`) банку зображень `img`(0-2) у позицію (`x`, `y`). Якщо негативне значення присвоєно для `w` та/або `h`, область буде віддзеркалена по горизонталі та/або вертикалі. Якщо вказано `colkey`, цей колір буде трактуватися як прозорий. Якщо вказано `rotate` (у градусах), `scale` (1.0 = 100%) або обидва параметри, будуть застосовані відповідні трансформації.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Копіює область розміром (`w`, `h`) із позиції (`u`, `v`) тайлмапу `tm` (0-7) у позицію (`x`, `y`). Якщо негативне значення присвоєно для `w` та/або `h`, область буде віддзеркалена по горизонталі та/або вертикалі. Якщо вказано `colkey`, цей колір буде трактуватися як прозорий. Якщо вказано `rotate` (у градусах), `scale` (1.0 = 100%) або обидва параметри, будуть застосовані відповідні трансформації. Розмір одного тайла становить 8x8 пікселів, і він зберігається в тайлмапі у вигляді кортежу `(image_tx, image_ty)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  Малює рядок `s` кольором `col` на позиції (`x`, `y`).

### Аудіо

- `sounds`<br>
  Список звуків (екземпляри класу Sound) (0-63)<br>
  Приклад: `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Список музик (екземпляри класу Music) (0-7)

- `play(ch, snd, [sec], [loop], [resume])`<br>
  Відтворює звук `snd`(0-63) на каналі `ch`(0-3). `snd` може бути номером звуку, списком номерів звуків або MML-рядком. Початкову позицію відтворення можна вказати в секундах за допомогою параметра `sec`. Якщо параметр `loop` встановлено в значення `True`, звук буде відтворюватися по колу. Щоб відновити попередній звук після завершення відтворення, встановіть параметр `resume` у значення `True`.

- `playm(msc, [sec], [loop])`<br>
  Відтворює музику `msc`(0-7). Початкову позицію відтворення можна вказати в секундах за допомогою параметра `sec`. Якщо параметр `loop` встановлено в значення `True`, музика буде відтворюватися по колу.

- `stop([ch])`<br>
  Зупиняє відтворення на вказаному каналі `ch` (0-3). Виклик `stop()` зупиняє відтворення на всіх каналах.

- `play_pos(ch)`<br>
  Повертає позицію відтворення звуку на каналі `ch` (0-3) у вигляді кортежу `(sound_no, sec)`. Повертає `None`, коли відтворення зупинено.

### Математика

- `ceil(x)`<br>
  Повертає найменше ціле число, більше або рівне `x`.

- `floor(x)`<br>
  Повертає найбільше ціле число, менше або рівне `x`.

- `sgn(x)`<br>
  Повертає `1`, якщо `x` додатний, `0`, якщо він дорівнює нулю, і `-1`, якщо від'ємний.

- `sqrt(x)`<br>
  Повертає квадратний корінь з `x`.

- `sin(deg)`<br>
  Повертає синус кута `deg` у градусах.

- `cos(deg)`<br>
  Повертає косинус кута `deg` у градусах.

- `atan2(y, x)`<br>
  Повертає арктангенс відношення `y` до `x` у градусах.

- `rseed(seed)`<br>
  Встановлює насіння генератора випадкових чисел.

- `rndi(a, b)`<br>
  Повертає випадкове ціле число від `a` до `b` включно.

- `rndf(a, b)`<br>
  Повертає випадкове дійсне число від `a` до `b` включно.

- `nseed(seed)`<br>
  Встановлює насіння для шуму Перліна.

- `noise(x, [y], [z])`<br>
  Повертає значення шуму Перліна для заданих координат.

### Клас Image

- `width`, `height`<br>
  Ширина та висота зображення

- `set(x, y, data)`<br>
  Встановлює зображення на позиції (`x`, `y`), використовуючи список рядків.<br>
  Приклад: `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Завантажує файл зображення (PNG/GIF/JPEG) на позицію (`x`, `y`).

- `pget(x, y)`<br>
  Повертає колір пікселя на позиції (`x`, `y`).

- `pset(x, y, col)`<br>
  Малює піксель кольору `col` на позиції (`x`, `y`).

### Клас Tilemap

- `width`, `height`<br>
  Ширина та висота тайлмапу

- `imgsrc`<br>
  Банк зображень (0-2), на який посилається тайлмап

- `set(x, y, data)`<br>
  Встановлює тайлмап на позиції (`x`, `y`), використовуючи список рядків.<br>
  Приклад: `pyxel.tilemaps[0].set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  Завантажує `layer` (0-) з файлу TMX (Tiled Map File) на позицію (`x`, `y`).

- `pget(x, y)`<br>
  Повертає тайл на позиції (`x`, `y`). Тайл представлений у вигляді кортежу `(image_tx, image_ty)`.

- `pset(x, y, tile)`<br>
  Малює `tile` на позиції (`x`, `y`). Тайл представлений у вигляді кортежу `(image_tx, image_ty)`.

### Клас Sound

- `notes`<br>
  Список нот (0-127). Чим більше число, тим вища нота. Нота `33` відповідає 'A2' (440Hz). Пауза позначається як `-1`.

- `tones`<br>
  Список тонів (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
  Список гучностей (0-7)

- `effects`<br>
  Список ефектів (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  Швидкість відтворення. `1` — найшвидша швидкість, чим більше число, тим повільніше відтворення. При `120` тривалість однієї ноти становить 1 секунду.

- `set(notes, tones, volumes, effects, speed)`<br>
  Встановлює ноти, тони, гучності та ефекти за допомогою рядка. Якщо довжина тонів, гучностей або ефектів коротша за ноти, вони будуть повторюватися з початку.

- `set_notes(notes)`<br>
  Встановлює ноти за допомогою рядка, що складається з `CDEFGAB`+`#-`+`01234` або `R`. Регістр не має значення, пробіли ігноруються.<br>
  Приклад: `pyxel.sounds[0].set_notes("g2b-2d3r rf3f3f3")`

- `set_tones(tones)`<br>
  Встановлює тони за допомогою рядка, що складається з `TSPN`. Регістр не має значення, пробіли ігноруються.<br>
  Приклад: `pyxel.sounds[0].set_tones("ttss pppn")`

- `set_volumes(volumes)`<br>
  Встановлює гучності за допомогою рядка, що складається з `01234567`. Регістр не має значення, пробіли ігноруються.<br>
  Приклад: `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  Встановлює ефекти за допомогою рядка, що складається з `NSVFHQ`. Регістр не має значення, пробіли ігноруються.<br>
  Приклад: `pyxel.sounds[0].set_effects("nfnf nvvs")`

- `mml(code)`<br>
  Передача рядка [MML (Music Macro Language)](https://en.wikipedia.org/wiki/Music_Macro_Language) перемикає в режим MML і відтворює звук відповідно до його вмісту. У цьому режимі звичайні параметри, такі як `notes` і `speed`, ігноруються. Щоб вийти з режиму MML, викличте `mml()` без аргументів. Для отримання додаткової інформації про MML дивіться [цю сторінку](faq-en.md).<br>
  Приклад: `pyxel.sounds[0].mml("T120 Q90 @1 V100 O5 L8 C4&C<G16R16>C.<G16 >C.D16 @VIB1{10,20,20} E2C2")`

- `save(filename, sec, [ffmpeg])`<br>
  Створює WAV-файл, який відтворює звук протягом вказаної кількості секунд. Якщо FFmpeg встановлено та `ffmpeg` встановлено в значення `True`, також створюється MP4-файл.

- `total_sec()`<br>
  Повертає час відтворення звуку в секундах. Повертає `None`, якщо в MML використовується нескінченний цикл.

### Клас Music

- `seqs`<br>
  Двовимірний список звуків (0-63) по кількох каналах

- `set(seq0, seq1, seq2, ...)`<br>
  Встановлює списки звуків (0-63) для кожного каналу. Якщо вказано порожній список, цей канал не використовуватиметься для відтворення.<br>
  Приклад: `pyxel.musics[0].set([0, 1], [], [3])`

- `save(filename, sec, [ffmpeg])`<br>
  Створює WAV-файл, який відтворює музику протягом вказаної кількості секунд. Якщо FFmpeg встановлено та `ffmpeg` встановлено в значення `True`, також створюється MP4-файл.

### Розширене API

Pyxel включає "Розширене API", яке не згадується в цьому довіднику, оскільки воно може заплутати користувачів або вимагати спеціальних знань для використання.

Якщо ви впевнені у своїх силах, спробуйте створити дивовижні роботи, використовуючи [цей](../python/pyxel/__init__.pyi) посібник!

## Як зробити внесок

### Подання проблем

Використовуйте [Трекер проблем](https://github.com/kitao/pyxel/issues) для подання звітів про помилки та запитів на функції або покращення. Перед поданням нової проблеми переконайтеся, що немає подібних відкритих проблем.

### Функціональне тестування

Будь-хто, хто тестує код вручну та повідомляє про помилки або пропозиції щодо покращення в [Трекері проблем](https://github.com/kitao/pyxel/issues), дуже вітається!

### Подання запитів на витягування

Патчі та виправлення приймаються у вигляді запитів на витягування (PR). Переконайтеся, що проблема, яку вирішує запит на витягування, відкрита в Трекері проблем.

Подання запиту на витягування означає, що ви погоджуєтеся ліцензувати свій внесок відповідно до [ліцензії MIT](../LICENSE).

## Веб-інструменти та приклади

- [Pyxel Web Examples](https://kitao.github.io/pyxel/wasm/examples/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/)
- [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/)

## Інша інформація

- [Часті запитання](faq-en.md)
- [Приклади користувачів](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [X-акаунт розробника](https://x.com/kitao)
- [Сервер Discord (Англійська)](https://discord.gg/Z87eYHN)
- [Сервер Discord (Японська)](https://discord.gg/qHA5BCS)

## Ліцензія

Pyxel ліцензовано під [ліцензією MIT](../LICENSE). Його можна використовувати в власному програмному забезпеченні, за умови, що всі копії програмного забезпечення або його істотні частини містять копію умов ліцензії MIT та повідомлення про авторські права.

## Пошук спонсорів

Pyxel шукає спонсорів на GitHub Sponsors. Розгляньте можливість спонсорування Pyxel, щоб підтримати його подальше обслуговування та розвиток функцій. Як перевага, спонсори можуть безпосередньо консультуватися з розробником Pyxel. Для отримання додаткової інформації відвідайте [цю сторінку](https://github.com/sponsors/kitao).
