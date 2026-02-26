# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** (/ˈpɪksəl/) — це ретро-ігровий движок для Python.

Специфікації натхнені ретро-ігровими консолями, зокрема обмеженням до 16 кольорів і 4 звукових каналів, що дозволяє легко насолоджуватися створенням ігор у стилі піксельної графіки.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

Розробка Pyxel здійснюється на основі відгуків користувачів. Будь ласка, поставте Pyxel зірку на GitHub!

<p>
<a href="https://kitao.github.io/pyxel/wasm/showcase/examples/10-platformer.html">
<img src="images/10_platformer.gif" width="290">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/apps/30sec-of-daylight.html">
<img src="images/30sec_of_daylight.gif" width="350">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/examples/02-jump-game.html">
<img src="images/02_jump_game.gif" width="330">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/apps/megaball.html">
<img src="images/megaball.gif" width="310">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
<img src="images/sound_music_editor.gif" width="320">
</a>
</p>

Специфікації та API Pyxel були натхненні [PICO-8](https://www.lexaloffle.com/pico-8.php) та [TIC-80](https://tic80.com/).

Pyxel є відкритим програмним забезпеченням під [ліцензією MIT](../LICENSE) та безкоштовним для використання. Давайте почнемо створювати ретро-ігри з Pyxel!

## Специфікації

- Працює на Windows, Mac, Linux та Web
- Програмування на Python
- Налаштовуваний розмір екрану
- 16-кольорова палітра
- 3 банки зображень 256x256
- 8 тайлмапів 256x256
- 4 канали з 64 налаштовуваними звуками
- 8 музичних треків, здатних поєднувати будь-які звуки
- Введення з клавіатури, миші та ігрового контролера
- Інструменти для редагування зображень та звуків
- Розширювані користувачем кольори, звукові канали та банки

### Кольорова палітра

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Як встановити

### Windows

Після встановлення [Python 3](https://www.python.org/) (версія 3.8 або вища) виконайте наступну команду:

```sh
pip install -U pyxel
```

Під час встановлення Python за допомогою офіційного установника переконайтеся, що ви вибрали опцію `Add Python 3.x to PATH`, щоб активувати команду `pyxel`.

### Mac

Після встановлення [Homebrew](https://brew.sh/) виконайте наступні команди:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Щоб оновити Pyxel після установки, виконайте `pipx upgrade pyxel`.

### Linux

Після встановлення [Python 3](https://www.python.org/) (версія 3.8 або вища) виконайте наступну команду:

```sh
pip install -U pyxel
```

Якщо попередня команда не спрацювала, подумайте про збірку Pyxel з вихідного коду, дотримуючись інструкцій у [Makefile](../Makefile).

### Web

Веб-версія Pyxel працює на ПК, смартфонах і планшетах із сумісним браузером, без встановлення Python або Pyxel.

Найпростіший спосіб використовувати її — через онлайн-IDE [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/).

Для інших моделей використання, таких як вбудовування додатків Pyxel на ваш власний сайт, зверніться до [цієї сторінки](pyxel-web-en.md).

## Основи використання

### Команда Pyxel

Встановлення Pyxel додає команду `pyxel`. Вкажіть назву команди після `pyxel` для виконання різних операцій.

Запустіть без аргументів, щоб побачити список доступних команд:

```sh
pyxel
```

```
Pyxel 2.7.1, a retro game engine for Python
usage:
    pyxel run PYTHON_SCRIPT_FILE(.py)
    pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE(.py)
    pyxel play PYXEL_APP_FILE(.pyxapp)
    pyxel edit [PYXEL_RESOURCE_FILE(.pyxres)]
    pyxel package APP_DIR STARTUP_SCRIPT_FILE(.py)
    pyxel app2exe PYXEL_APP_FILE(.pyxapp)
    pyxel app2html PYXEL_APP_FILE(.pyxapp)
    pyxel copy_examples
```

### Запуск прикладів

Наступна команда копіює приклади Pyxel у поточний каталог:

```sh
pyxel copy_examples
```

У локальному середовищі приклади можна виконати за допомогою наступних команд:

```sh
# Запустити приклад у каталозі examples
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# Запустити застосунок у каталозі examples/apps
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

Список прикладів також можна переглянути та запустити у браузері на [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/).

## Створення додатків

### Створення програми

У вашому Python-скрипті імпортуйте Pyxel, вкажіть розмір вікна за допомогою `init` і запустіть програму за допомогою `run`.

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

Крім того, команда `pyxel watch` відстежує зміни в зазначеному каталозі та автоматично перезапускає програму при виявленні змін:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

Зупиніть спостереження за каталогом, натиснувши `Ctrl(Command)+C`.

### Спеціальні клавіші

Під час виконання програми Pyxel можна виконати наступні спеціальні дії з клавішами:

- `Esc`<br>
  Вийти з застосунку
- `Alt(Option)+R` або `A+B+X+Y+BACK` на геймпаді<br>
  Скинути застосунок
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

## Створення ресурсів

### Pyxel Editor

Pyxel Editor дозволяє створювати зображення та звуки для програм Pyxel.

Ви можете запустити Pyxel Editor за допомогою наступної команди:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Якщо зазначений файл ресурсу Pyxel (.pyxres) існує, він буде завантажений. Якщо ні, то буде створено новий файл з вказаним ім'ям. Якщо файл ресурсу пропущено, буде створено новий файл з назвою `my_resource.pyxres`.

Після запуску Pyxel Editor ви можете перемикатися між файлами ресурсів, перетягнувши файл на редактор.

Створений файл ресурсу можна завантажити за допомогою функції `load`.

Pyxel Editor має такі режими редагування.

**Редактор зображень**

Режим для редагування зображень у кожному **банку зображень**.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_editor.gif">
</a>

Ви можете перетягувати файл зображення (PNG/GIF/JPEG) у редактор зображень, щоб завантажити зображення в поточний вибраний банк зображень.

**Редактор тайлмапів**

Режим для редагування **тайлмапів**, де зображення з банків зображень розташовані у вигляді плиток.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/tilemap-editor.html">
<img src="images/tilemap_editor.gif">
</a>

Перетягніть файл TMX (Tiled Map File) на редактор тайлмапів, щоб завантажити шар 0 у поточний вибраний тайлмап.

**Редактор звуку**

Режим для редагування **звуків**, що використовуються для мелодій і звукових ефектів.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
<img src="images/sound_editor.gif">
</a>

**Редактор музики**

Режим для редагування **музичних треків**, у яких звуки розташовані в порядку відтворення.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/music-editor.html">
<img src="images/music_editor.gif">
</a>

### Інші методи створення

Зображення та тайлмапи Pyxel також можна створювати за допомогою таких методів:

- Створюйте зображення або тайлмапи зі списків рядків за допомогою функцій `Image.set` або `Tilemap.set`
- Завантажуйте файли зображень, сумісні з палітрою Pyxel (PNG/GIF/JPEG), за допомогою функції `Image.load`

Звуки та музику Pyxel також можна створити за допомогою наступного методу:

- Створюйте їх з рядків за допомогою функцій `Sound.set` або `Music.set`

Зверніться до документації API для використання цих функцій.

## Розповсюдження додатків

Pyxel підтримує кросплатформений формат розповсюдження, який називається файлом програми Pyxel.

Створіть файл програми Pyxel (.pyxapp) за допомогою команди `pyxel package`:

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

Файл програми Pyxel також можна конвертувати у виконуваний файл або HTML-файл за допомогою команд `pyxel app2exe` або `pyxel app2html`.

## Довідник API

Повний список API Pyxel доступний на сторінці [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/).

Pyxel також містить «розширений API», для використання якого потрібні спеціальні знання. Його можна переглянути, встановивши прапорець «Advanced» на сторінці довідника.

Якщо ви впевнені у своїх навичках, спробуйте використати розширений API для створення справді вражаючих робіт!

## Як зробити внесок

### Подання проблем

Використовуйте [Трекер проблем](https://github.com/kitao/pyxel/issues) для подання звітів про помилки та запитів на функції або покращення. Перед поданням нової проблеми переконайтеся, що немає подібних відкритих проблем.

### Функціональне тестування

Будь-хто, хто тестує код вручну та повідомляє про помилки або пропозиції щодо покращення в [Трекері проблем](https://github.com/kitao/pyxel/issues), дуже вітається!

### Подання запитів на витягування

Патчі та виправлення приймаються у вигляді запитів на витягування (PR). Переконайтеся, що проблема, яку вирішує запит на витягування, відкрита в Трекері проблем.

Подання запиту на витягування означає, що ви погоджуєтеся ліцензувати свій внесок відповідно до [ліцензії MIT](../LICENSE).

## Веб-інструменти та приклади

- [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)
- [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/) [[User Manual](https://qiita.com/kitao/items/b5b3fb28ebf9781eda2e)]
- [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/) [[User Manual](https://qiita.com/kitao/items/a86de4f7d6a0ed656a89)]

## Інша інформація

- [Часті запитання](faq-en.md)
- [Приклади користувачів](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [X-акаунт розробника](https://x.com/kitao)
- [Сервер Discord (Англійська)](https://discord.gg/Z87eYHN)
- [Сервер Discord (Японська)](https://discord.gg/qHA5BCS)

## Ліцензія

Pyxel ліцензовано під [ліцензією MIT](../LICENSE). Його можна використовувати у власному програмному забезпеченні за умови, що всі копії програмного забезпечення або його істотні частини містять копію умов ліцензії MIT та повідомлення про авторські права.

## Пошук спонсорів

Pyxel шукає спонсорів на GitHub Sponsors. Розгляньте можливість спонсорування Pyxel, щоб підтримати його подальше обслуговування та розвиток функцій. Як перевага, спонсори можуть безпосередньо консультуватися з розробником Pyxel. Для отримання додаткової інформації відвідайте [цю сторінку](https://github.com/sponsors/kitao).
