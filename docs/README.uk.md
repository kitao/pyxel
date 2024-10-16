# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** — це ретро-ігровий движок для Python.

Специфікації натхнені ретро-ігровими консолями, такими як підтримка лише 16 кольорів та 4 звукових канали, що дозволяє легко насолоджуватися створенням ігор у стилі піксельної графіки.

<img src="images/pyxel_message.png" width="480">

Розробка Pyxel здійснюється на основі відгуків користувачів. Будь ласка, поставте Pyxel зірку на GitHub!

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

Специфікації та API Pyxel були натхненні [PICO-8](https://www.lexaloffle.com/pico-8.php) та [TIC-80](https://tic80.com/).

Pyxel є відкритим програмним забезпеченням під [ліцензією MIT](../LICENSE) та безкоштовний для використання. Давайте почнемо створювати ретро-ігри з Pyxel!

## Специфікації

- Працює на Windows, Mac, Linux та Web
- Програмування на Python
- 16 кольорова палітра
- 3 банки зображень розміром 256x256
- 8 тайлових карт розміром 256x256
- 4 канали з 64 налаштовуваними звуками
- 8 музик, які можуть поєднувати будь-які звуки
- Введення з клавіатури, миші та ігрового контролера
- Інструменти для редагування зображень та звуків
- Розширювані користувачем кольори, канали та банки

### Кольорова палітра

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Як інсталювати

### Windows

Після інсталяції [Python3](https://www.python.org/) (версії 3.8 або вище), виконайте наступну команду:

```sh
pip install -U pyxel
```

Якщо ви встановлюєте Python за допомогою офіційного інсталятора, будь ласка, встановіть прапорець `Add Python 3.x to PATH`, щоб увімкнути команду `pyxel`.

### Mac

Після інсталяції [Homebrew](https://brew.sh/), виконайте наступні команди:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Щоб оновити версію після інсталяції Pyxel, виконайте команду `pipx upgrade pyxel`.

### Linux

Після інсталяції пакету SDL2 (`libsdl2-dev` для Ubuntu), [Python3](https://www.python.org/) (версії 3.8 або вище) та `python3-pip`, виконайте наступну команду:

```sh
sudo pip3 install -U pyxel
```

Якщо вищевказане не допомогло, спробуйте самостійну збірку відповідно до інструкцій в [Makefile](../Makefile).

### Web

Веб-версія Pyxel не вимагає інсталяції Python або Pyxel і працює на ПК, смартфонах і планшетах з підтримуваними веб-браузерами.

Для отримання конкретних вказівок, будь ласка, зверніться до [цієї сторінки](pyxel-web-en.md).

### Спробуйте приклади

Після інсталяції Pyxel ви можете скопіювати приклади до поточного каталогу за допомогою наступної команди:

```sh
pyxel copy_examples
```

Перелік прикладів, які будуть скопійовані:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>Найпростіший додаток</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Код</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Гра зі стрибками з використанням файлу ресурсів Pyxel</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Код</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Демонстрація API для малювання</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Код</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Демонстрація API для звуку</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Код</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>Список кольорових паліт</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Код</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>Гра на клік миші</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Код</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>Гра «Змійка» з BGM</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Код</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Демонстрація API для малювання трикутників</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Код</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Shoot 'em up з переходами між екранами</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Код</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>Горизонтальна платформна гра з картою</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Код</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Відображення поза екраном з класом Image</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Код</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>Анімація Перлін-шуму</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Код</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>Малювання бітмап-шрифта</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Код</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>Синтезатор з використанням функцій розширення звуку</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Код</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>Завантаження та малювання Tiled Map File (.tmx)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Код</a></td>
</tr>
<tr>
<td>16_transform.py</td>
<td>Обертання та масштабування зображень</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/16_transform.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/16_transform.py">Код</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>Анімація за допомогою функції flip (тільки для платформ, що не є веб)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Демонстрація</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Код</a></td>
</tr>
<tr>
<td>30sec_of_daylight.pyxapp</td>
<td>Ігра-переможець 1-го Pyxel Jam від <a href="https://x.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30sec_of_daylight.html">Демонстрація</a></td>
<td><a href="https://github.com/kitao/30sec_of_daylight">Код</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>Аркадна гра з фізикою м'ячів від <a href="https://x.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Демонстрація</a></td>
<td><a href="https://github.com/helpcomputer/megaball">Код</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>Генератор фонової музики від <a href="https://x.com/frenchbread1222">frenchbread</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">Демонстрація</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">Код</a></td>
</tr>
</table>

Приклади можуть бути запущені за допомогою наступних команд:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30sec_of_daylight.pyxapp
```

## Як використовувати

### Створення програми

У вашому скрипті Python імпортуйте модуль Pyxel, вкажіть розмір вікна за допомогою функції `init`, а потім запустіть програму Pyxel за допомогою функції `run`.

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

Аргументами функції `run` є функція `update`, яка обробляє оновлення кадрів, та функція `draw`, яка відповідає за малювання на екрані.

У реальній програмі рекомендується обернути код Pyxel у клас, як показано нижче:

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

Під час створення простої графіки без анімації можна використовувати функцію `show`, щоб зробити код більш лаконічним.

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

Крім того, команда `pyxel watch` дозволяє відстежувати зміни у вказаному каталозі, автоматично перезапускаючи програму при виявленні змін:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

Моніторинг каталогів можна зупинити за допомогою комбінації клавіш `Ctrl(Command)+C`.

### Спеціальні клавіші

Під час виконання програми Pyxel можна виконати наступні спеціальні дії з клавішами:

- `Esc`<br>
  Вийти з програми
- `Alt(Option)+1`<br>
  Зберегти знімок екрану на робочий стіл
- `Alt(Option)+2`<br>
  Скинути час початку запису відео з екрану
- `Alt(Option)+3`<br>
  Зберегти відео захоплення екрану на робочий стіл (до 10 секунд)
- `Alt(Option)+9`<br>
  Перемикати між режимами екрану (Crisp/Smooth/Retro)
- `Alt(Option)+0`<br>
  Перемикати монітор продуктивності (FPS/`update` час/`draw` час)
- `Alt(Option)+Enter`<br>
  Перемикати повноекранний режим
- `Shift+Alt(Option)+1/2/3`<br>
  Зберегти відповідний банк зображень на робочий стіл
- `Shift+Alt(Option)+0`<br>
  Зберегти поточну кольорову палітру на робочий стіл

### Як створити ресурси

Редактор Pyxel може створювати зображення та звуки, які використовуються в програмі Pyxel.

Він запускається за допомогою наступної команди:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Якщо вказаний файл ресурсів Pyxel (.pyxres) існує, файл завантажується, а якщо він не існує, створюється новий файл із зазначеною назвою. Якщо файл ресурсу пропущено, його назва буде `my_resource.pyxres`.

Після запуску редактора Pyxel перемикання між файлами відбувається шляхом перетягування іншого файлу ресурсу.

Створений файл ресурсу можна завантажити за допомогою функції `load`.

Вбудований редактор Pyxel має наступні режими редагування.

**Редактор зображень**

Режим редагування сховищ зображень.

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

Перетягніть файл зображення (PNG/GIF/JPEG) у редактор зображень, щоб завантажити зображення в обране в даний момент сховище зображень.

**Редактор карт тайлів**

Режим редагування карт тайлів, в якому зображення зі сховища зображень упорядковуються у вигляді тайлів.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

Перетягніть файл TMX (файл карти тайлів) у редактор карт тайлів, щоб завантажити його шар у порядку малювання, що відповідає вибраному в даний момент номеру карти тайлів.

**Редактор звуків**

Режим редагування звуків.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**Редактор музики**

Режим редагування музичних композицій, у якому звуки впорядковуються в порядку відтворення.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### Інші методи створення ресурсів

Зображення Pyxel і карти тайлів також можна створювати за допомогою наступних методів:

- Створюйте зображення зі списку рядків за допомогою функції `Image.set` або функції `Tilemap.set`.
- Завантажуйте файл зображення (PNG/GIF/JPEG) у палітру Pyxel за допомогою функції `Image.load`.

Звуки Pyxel також можна створювати наступним способом:

- Створюйте звук із рядків за допомогою функції `Sound.set` або `Music.set`.

Будь ласка, зверніться до довідника API, щоб дізнатися про використання цих функцій.

### Як розповсюджувати додатки

Pyxel підтримує спеціальний формат файлу розповсюдження програми (файл програми Pyxel), який працює на різних платформах.

Файл програми Pyxel (.pyxapp) створюється за допомогою команди `pyxel package`:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Якщо потрібно включити ресурси або додаткові модулі, розмістіть їх у каталозі програми.

Метадані можна відображати під час виконання, вказавши їх у наступному форматі у скрипті запуску. Поля, відмінні від `title` та `author`, можна не вказувати.

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

Файл програми Pyxel також можна перетворити на виконуваний файл EXE або файл HTML за допомогою команд `pyxel app2exe` або `pyxel app2html`.

## Довідник API

### Система

- `width`, `height`<br>
  Ширина та висота екрану

- `frame_count`<br>
  Кількість відображених кадрів

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Ініціалізувати програму Pyxel з введеними розмірами екрану (`width`, `height`). Як параметри можна вказати: заголовок вікна за допомогою `title`, частоту кадрів за допомогою `fps`, ключ для виходу з програми за допомогою `quit_key`, масштаб відображення за допомогою `display_scale`, масштаб захоплення екрану за допомогою `capture_scale`, максимальний час відеозапису екрану за допомогою `capture_sec`.<br>
  напр. `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Запустити програму Pyxel, викликати функцію `update` для оновлення кадрів і функцію `draw` для малювання.

- `show()`<br>
  Показати екран і чекати, доки не буде натиснуто клавішу `Esc`.

- `flip()`<br>
  Оновлення екрана на один кадр. Програма завершує роботу після натискання клавіші `Esc`. Ця функція не працює у web-версії.

- `quit()`<br>
  Вийти із програми Pyxel.

### Ресурси

- `load(filename, [excl_images], [excl_tilemaps], [excl_sounds], [excl_musics])`<br>
  Завантажте файл ресурсів (.pyxres). Якщо параметр має значення `True`, ресурс не буде завантажено. Якщо файл палітри (.pyxpal) з такою ж назвою існує в тому самому місці, що й файл ресурсів, колір відображення палітри також буде змінено. Файл палітри — це шістнадцятковий запис кольорів відображення (наприклад, `1100FF`), розділених символами нового рядка. Файл палітри також можна використовувати для зміни кольорів, які відображаються в редакторі Pyxel.

### Вхід

- `mouse_x`, `mouse_y`<br>
  Поточна позиція курсора миші

- `mouse_wheel`<br>
  Поточне значення колеса миші

- `btn(key)`<br>
  Повертає `True`, якщо клавіша `key` натиснута, інакше повертає `False`. ([Список визначень клавіш](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  Повертає `True`, якщо в цьому кадрі натиснуто клавішу `key`, інакше повертає `False`. Якщо вказано `hold` і `repeat`, значення `True` буде повернено в інтервалі кадрів `repeat`, якщо клавіша `key` утримується натиснутою більше ніж `hold` кадрів.

- `btnr(key)`<br>
  Повертає `True`, якщо клавішу `key` відпущено в цьому кадрі, інакше повертає `False`.

- `mouse(visible)`<br>
  Якщо `visible` має значення `True`, відображається курсор миші. Якщо `False`, приховується. Навіть якщо курсор миші не відображається, його положення оновлюється.

### Графіка

- `colors`<br>
  Список кольорів палітри. Колір відображення визначається 24-бітним числовим значенням. Використовуйте `colors.from_list` і `colors.to_list`, щоб напряму призначати та отримувати списки Python.<br>
  напр. `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Список сховищ зображень (0-2)<br>
  напр. `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Список карт тайлів (0-7)

- `clip(x, y, w, h)`<br>
  Встановіть область малювання екрану від координат (`x`, `y`) до ширини `w` і висоти `h`. Скиньте область малювання на весь екран за допомогою `clip()`.

- `camera(x, y)`<br>
  Змінити координати верхнього лівого кута екрана на (`x`, `y`). Скиньте координати верхнього лівого кута на (`0`, `0`) за допомогою `camera()`.

- `pal(col1, col2)`<br>
  Замінити колір `col1` на `col2` під час малювання. `pal()` для повернення до початкової палітри.

- `dither(alpha)`<br>
  Застосувати дизерінг (псевдопрозорість) під час малювання. Встановіть `alpha` в діапазоні `0.0`-`1.0`, де `0.0` — прозорий, а `1.0` — непрозорий.

- `cls(col)`<br>
  Заповнити/очистити екран із кольоровим `col`.

- `pget(x, y)`<br>
  Отримати колір пікселя в координатах (`x`, `y`).

- `pset(x, y, col)`<br>
  Намалювати піксель кольору `col` в координатах (`x`, `y`).

- `line(x1, y1, x2, y2, col)`<br>
  Намалювати лінію кольору `col` від координат (`x1`, `y1`) до (`x2`, `y2`).

- `rect(x, y, w, h, col)`<br>
  Намалювати прямокутник з шириною `w`, висотою `h` та кольором `col` від координат (`x`, `y`).

- `rectb(x, y, w, h, col)`<br>
  Намалювати контур прямокутника з шириною `w`, висотою `h` та кольором `col` від координат (`x`, `y`).

- `circ(x, y, r, col)`<br>
  Намалювати коло з радіусом `r` та кольором `col` в координатах (`x`, `y`).

- `circb(x, y, r, col)`<br>
  Намалювати контур кола з радіусом `r` та кольором `col` в координатах (`x`, `y`).

- `elli(x, y, w, h, col)`<br>
  Намалювати еліпс з шириною `w`, висотою `h` та кольором `col` від координат (`x`, `y`).

- `ellib(x, y, w, h, col)`<br>
  Намалювати контур еліпса з шириною `w`, висотою `h` та кольором `col` від координат (`x`, `y`).

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  Намалювати трикутник із вершинами в координатах (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) та кольором `col`.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  Намалювати контур трикутника із вершинами в координатах (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) та кольором `col`.

- `fill(x, y, col)`<br>
  Заповнити область, з’єднану тим самим кольором, що й (`x`, `y`), кольором `col`.

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Скопіювати область розмірами (`w`, `h`) з (`u`, `v`) сховища зображень `img`(0-2) до координат (`x`, `y`). Якщо параметри `w` та/або `h` мають від'ємне значення, область буде змінено горизонтально та/або вертикально. Якщо вказано `colkey`, область буде розглядатися як прозорий колір. Якщо вказано `rotate`(в градусах), `scale`(1.0=100%) або обидва параметри, буде застосовано відповідне перетворення.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Скопіювати область розмірами (`w`, `h`) з (`u`, `v`) карти тайлів `tm`(0-7) до координат (`x`, `y`). Якщо параметри `w` та/або `h` мають від'ємне значення, область буде змінено горизонтально та/або вертикально. Якщо вказано `colkey`, область буде розглядатися як прозорий колір. Якщо вказано `rotate`(в градусах), `scale`(1.0=100%) або обидва параметри, буде застосовано відповідне перетворення. Розмір тайлу становить 8x8 пікселів, і він зберігається в карті тайлів як кортеж `(tile_x, tile_y)`.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  Намалювати рядок `s` кольором `col` в координатах (`x`, `y`).

### Аудіо

- `sounds`<br>
  Список звуків (0-63)<br>
  напр. `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Список музики (0-7)

- `play(ch, snd, [tick], [loop], [resume])`<br>
  Відтворити звук `snd`(0-63) на каналі `ch`(0-3). Якщо параметр `snd` це список, звуки відтворюватимуться по порядку. Початкову позицію відтворення можна вказати за допомогою `tick`(1 tick = 1/120 секунди). Якщо параметр `loop` має значення `True`, виконується циклічне відтворення. Щоб відновити попередній звук після завершення відтворення, встановіть для параметра `resume` значення `True`.

- `playm(msc, [tick], [loop])`<br>
  Відтворити музику `msc`(0-7). Початкову позицію відтворення можна вказати за допомогою `tick`(1 tick = 1/120 секунди). Якщо параметр `loop` має значення `True`, виконується циклічне відтворення.

- `stop([ch])`<br>
  Зупинити відтворення вказаного каналу `ch`(0-3). `stop()` без параметрів, щоб зупинити відтворення всіх каналів.

- `play_pos(ch)`<br>
  Отримати позицію відтворення звуку каналу `ch`(0-3) у вигляді кортежу `(sound_no, note_no)`. Повертає `None`, коли відтворення зупинено.

### Математика

- `ceil(x)`<br>
  Повертає найменше ціле число, яке більше або рівне `x`.

- `floor(x)`<br>
  Повертає найбільше ціле число, яке менше або рівне `x`.

- `sgn(x)`<br>
  Повертає `1`, якщо `x` додатне, `0`, коли воно дорівнює `0`, і `-1`, якщо воно від’ємне.

- `sqrt(x)`<br>
  Повертає квадратний корінь з числа `x`.

- `sin(deg)`<br>
  Повертає синус з `deg` градусів.

- `cos(deg)`<br>
  Повертає косинус з `deg` градусів.

- `atan2(y, x)`<br>
  Повертає арктангенс з `y`/`x` у градусах.

- `rseed(seed)`<br>
  Встановити початковий параметр генератора випадкових чисел.

- `rndi(a, b)`<br>
  Повертає випадкове ціле число, яке більше або дорівнює `a` і менше або дорівнює `b`.

- `rndf(a, b)`<br>
  Повертає випадковий десятковий дріб, яке більше або дорівнює `a` і менше або дорівнює `b`.

- `nseed(seed)`<br>
  Встановити початковий параметр шуму Перліна.

- `noise(x, [y], [z])`<br>
  Повертає значення шуму Перліна для вказаних координат.

### Клас Image

- `width`, `height`<br>
  Ширина та висота зображення

- `set(x, y, data)`<br>
  Встановити зображення в координати (`x`, `y`) за допомогою списку рядків.<br>
  напр. `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Завантажити файл зображення (PNG/GIF/JPEG) в координати (`x`, `y`).

- `pget(x, y)`<br>
  Отримати колір пікселя з координат (`x`, `y`).

- `pset(x, y, col)`<br>
  Намалювати піксель кольору `col` в координати (`x`, `y`).

### Клас Tilemap

- `width`, `height`<br>
  Ширина та висота карти тайлів

- `imgsrc`<br>
  Сховище зображень (0-2), на який посилається карта тайлів.

- `set(x, y, data)`<br>
  Встановити карту тайлів в координати (`x`, `y`) за допомогою списку рядків.<br>
  напр. `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  Завантажити шар у порядку малювання `layer`(0-) із файлу TMX (файл карти тайлів) в координатах (`x`, `y`).

- `pget(x, y)`<br>
  Отримати тайл з координат (`x`, `y`). Тайл — це кортеж `(tile_x, tile_y)`.

- `pset(x, y, tile)`<br>
  Намалювати `tile` в координатах (`x`, `y`). Тайл — це кортеж `(tile_x, tile_y)`.

### Клас Sound

- `notes`<br>
  Список нот (0-127). Чим більше число, тим вище висота тону, і при `33` він стає 'A2'(440Hz). Решта становить `-1`.

- `tones`<br>
  Список тонів (0:Трикутник / 1:Квадрат / 2:Пульс / 3:Шум)

- `volumes`<br>
  Список гучностей (0-7)

- `effects`<br>
  Список ефектів (0:Немає / 1:Слайд / 2:Вібрато / 3:Згасання / 4:Напівзгасання / 5:Згасання на чверть)

- `speed`<br>
  Швидкість відтворення. `1` є найшвидшим, і чим більше число, тим повільніше швидкість відтворення. На `120` тривалість однієї ноти стає 1 секундою.

- `set(notes, tones, volumes, effects, speed)`<br>
  Встановити ноти, тони, гучність і ефекти за допомогою рядка. Якщо тони, гучність та довжина ефектів коротші за ноти, вони повторюються з початку.

- `set_notes(notes)`<br>
  Встановити ноти за допомогою рядка, складеного з 'CDEFGAB'+'#-'+'01234' або 'R'. Регістр та пробіли ігноруються.<br>
  напр. `pyxel.sounds[0].set_notes("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
  Встановити тони за допомогою рядка, складеного з 'TSPN'. Регістр та пробіли ігноруються.<br>
  напр. `pyxel.sounds[0].set_tones("TTSS PPPN")`

- `set_volumes(volumes)`<br>
  Встановити гучності за допомогою рядка, складеного з '01234567'. Регістр та пробіли ігноруються.<br>
  напр. `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  Встановити ефекти за допомогою рядка, складеного з 'NSVFHQ'. Регістр та пробіли ігноруються.<br>
  напр. `pyxel.sounds[0].set_effects("NFNF NVVS")`

### Клас Music

- `seqs`<br>
  Двовимірний список звуків (0-63) з кількістю каналів

- `set(seq0, seq1, seq2, ...)`<br>
  Встановити списки звуку (0-63) каналів. Якщо вказано порожній список, цей канал не буде використовуватися для відтворення.<br>
  напр. `pyxel.musics[0].set([0, 1], [], [3])`

### Розширене API

Pyxel має "розширені API", які не згадуються в цьому довіднику, оскільки вони «можуть заплутати користувачів» або «вимагають спеціальних знань для використання».

Якщо ви впевнені у своїх навичках, спробуйте створити дивовижні роботи, використовуючи [це](../python/pyxel/__init__.pyi) як підказку!

## Як внести свій вклад в розвиток проєкту

### Надсилання звітів про проблеми

Використовуйте [систему відстеження проблем](https://github.com/kitao/pyxel/issues), щоб надсилати звіти про помилки та запити щодо функцій/розширень. Перш ніж надсилати нову проблему, переконайтеся, що немає аналогічної відкритої проблеми.

### Ручне тестування

Вітаються всі, хто вручну тестує код і повідомляє про помилки чи пропозиції щодо покращення в [системі відстеження проблем](https://github.com/kitao/pyxel/issues)!

### Надсилання pull-запитів

Патчі/виправлення приймаються у формі pull-запитів на злиття. Переконайтеся, що проблема, яка стосується pull-запиту на злиття, відкрита в [системі відстеження проблем](https://github.com/kitao/pyxel/issues).

Наданий pull request вважається згодою на публікацію за [ліцензією MIT](../LICENSE).

## Інша інформація

- [FAQ](faq-en.md)
- [Приклади користувачів](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Обліковий запис розробника X](https://x.com/kitao)

## Ліцензія

Pyxel розповсюджується за [ліцензією MIT](../LICENSE). Його можна використовувати в закритому програмному забезпеченні за умови, що всі копії програмного забезпечення або його суттєві частини містять копію умов ліцензії MIT, а також повідомлення про авторські права.

## Шукаємо спонсорів

Pyxel шукає спонсорів на GitHub Sponsors. Розгляньте можливість спонсорування Pyxel для подальшого обслуговування та додавання функцій. Спонсорам доступна можливість консультації щодо Pyxel. Будь ласка, перегляньте деталі [тут](https://github.com/sponsors/kitao).
