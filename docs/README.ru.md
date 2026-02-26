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

Спецификации и API Pyxel вдохновлены [PICO-8](https://www.lexaloffle.com/pico-8.php) и [TIC-80](https://tic80.com/).

Pyxel является открытым программным обеспечением под [MIT лицензией](../LICENSE) и бесплатен для использования. Давайте начнем создавать ретро-игры с Pyxel!

## Спецификации

- Работает на Windows, Mac, Linux и Web
- Программирование на Python
- Настраиваемый размер экрана
- 16-цветная палитра
- 3 банка изображений 256x256
- 8 тайловых карт 256x256
- 4 канала с 64 настраиваемыми звуками
- 8 музыкальных треков, способных сочетать любые звуки
- Ввод с клавиатуры, мыши и игрового контроллера
- Инструменты для редактирования изображений и звуков
- Расширяемые пользователем цвета, звуковые каналы и банки

### Цветовая палитра

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Как установить

### Windows

После установки [Python 3](https://www.python.org/) (версии 3.8 или выше), выполните следующую команду:

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

После установки [Python 3](https://www.python.org/) (версии 3.8 или выше) выполните следующую команду:

```sh
pip install -U pyxel
```

Если предыдущая команда не сработала, подумайте о сборке Pyxel из исходного кода, следуя инструкциям в [Makefile](../Makefile).

### Web

Веб-версия Pyxel работает на ПК, смартфонах и планшетах с совместимым браузером, без установки Python или Pyxel.

Самый простой способ использовать её — через онлайн-IDE [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/).

Для других моделей использования, таких как встраивание приложений Pyxel на ваш собственный сайт, обратитесь к [этой странице](pyxel-web-en.md).

## Основы использования

### Команда Pyxel

При установке Pyxel добавляется команда `pyxel`. Укажите имя команды после `pyxel` для выполнения различных операций.

Запустите без аргументов, чтобы увидеть список доступных команд:

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

### Запуск примеров

Следующая команда копирует примеры Pyxel в текущую директорию:

```sh
pyxel copy_examples
```

В локальной среде примеры можно выполнить с помощью следующих команд:

```sh
# Запустить пример в каталоге examples
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# Запустить приложение в каталоге examples/apps
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

Список примеров также можно просмотреть и запустить в браузере на [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/).

## Создание приложений

### Создание программы

В вашем Python-скрипте импортируйте Pyxel, укажите размер окна с помощью `init` и запустите приложение с помощью `run`.

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

### Запуск программы

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

Остановите наблюдение за каталогом, нажав `Ctrl(Command)+C`.

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
  Переключить масштаб экрана между максимальным и целочисленным
- `Alt(Option)+9` или `A+B+X+Y+DR` на геймпаде<br>
  Переключить режим экрана (Crisp/Smooth/Retro)
- `Alt(Option)+0` или `A+B+X+Y+DU` на геймпаде<br>
  Переключить монитор производительности (FPS/время `update`/время `draw`)
- `Alt(Option)+Enter` или `A+B+X+Y+DD` на геймпаде<br>
  Переключить полноэкранный режим
- `Shift+Alt(Option)+1/2/3`<br>
  Сохранить банк изображений 0, 1 или 2 на рабочий стол
- `Shift+Alt(Option)+0`<br>
  Сохранить текущую цветовую палитру на рабочий стол

## Создание ресурсов

### Pyxel Editor

Pyxel Editor позволяет создавать изображения и звуки для приложений Pyxel.

Вы можете запустить Pyxel Editor с помощью следующей команды:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Если указанный файл ресурсов Pyxel (.pyxres) существует, он будет загружен. Если его нет, будет создан новый файл с указанным именем. Если файл ресурсов пропущен, будет создан новый файл с именем `my_resource.pyxres`.

После запуска Pyxel Editor вы можете переключаться между файлами ресурсов, перетащив файл на редактор.

Созданный файл ресурсов можно загрузить с помощью функции `load`.

Pyxel Editor имеет следующие режимы редактирования.

**Редактор изображений**

Режим для редактирования изображений в каждом **банке изображений**.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_editor.gif">
</a>

Вы можете перетаскивать и бросать файл изображения (PNG/GIF/JPEG) в редактор изображений, чтобы загрузить изображение в выбранный в данный момент банк изображений.

**Редактор тайловых карт**

Режим для редактирования **тайловых карт**, в которых изображения банков изображений располагаются в плиточном порядке.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/tilemap-editor.html">
<img src="images/tilemap_editor.gif">
</a>

Перетащите файл TMX (Tiled Map File) в редактор тайловых карт, чтобы загрузить его слой 0 в текущую выбранную тайловую карту.

**Редактор звуков**

Режим редактирования **звуков**, используемых для мелодий и звуковых эффектов.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
<img src="images/sound_editor.gif">
</a>

**Редактор музыки**

Режим для редактирования **музыкальных треков**, в которых звуки расположены в порядке воспроизведения.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/music-editor.html">
<img src="images/music_editor.gif">
</a>

### Другие методы создания

Изображения и тайловые карты Pyxel также можно создавать с помощью следующих методов:

- Создавайте изображения или тайловые карты из списков строк с помощью функций `Image.set` или `Tilemap.set`
- Загружайте файлы изображений, совместимые с палитрой Pyxel (PNG/GIF/JPEG), с помощью функции `Image.load`

Звуки и музыка Pyxel также можно создать с помощью следующего метода:

- Создавайте их из строк с помощью функций `Sound.set` или `Music.set`

См. справочник API, чтобы узнать, как использовать эти функции.

## Распространение приложений

Pyxel поддерживает кроссплатформенный формат распространения под названием файл приложения Pyxel.

Создайте файл приложения Pyxel (.pyxapp) с помощью команды `pyxel package`:

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

Файл приложения Pyxel также можно преобразовать в исполняемый файл или HTML-файл командами `pyxel app2exe` или `pyxel app2html`.

## Справочник API

Полный список API Pyxel доступен на странице [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/).

Pyxel также включает «расширенный API», для использования которого требуются специальные знания. Его можно просмотреть, установив флажок «Advanced» на странице справочника.

Если вы уверены в своих навыках, попробуйте использовать расширенный API для создания по-настоящему впечатляющих работ!

## Как внести вклад

### Подавать проблемы

Используйте [Трекер проблем](https://github.com/kitao/pyxel/issues) для подачи отчетов об ошибках и запросов на функции или улучшения. Перед отправкой новой проблемы убедитесь, что нет похожих открытых вопросов.

### Функциональное тестирование

Любой, кто вручную тестирует код и сообщает об ошибках или предлагает улучшения в [Трекере проблем](https://github.com/kitao/pyxel/issues), очень приветствуется!

### Подавать запросы на изменение

Патчи и исправления принимаются в форме запросов на изменение (PR). Убедитесь, что проблема, на которую ссылается запрос на изменение, открыта в Трекере проблем.

Подача запроса на изменение подразумевает, что вы соглашаетесь лицензировать свой вклад по [Лицензии MIT](../LICENSE).

## Веб-инструменты и примеры

- [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)
- [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/)
- [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/)

## Дополнительная информация

- [Часто задаваемые вопросы](faq-en.md)
- [Примеры пользователей](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [X-аккаунт разработчика](https://x.com/kitao)
- [Сервер Discord (Английский)](https://discord.gg/Z87eYHN)
- [Сервер Discord (Японский)](https://discord.gg/qHA5BCS)

## Лицензия

Pyxel лицензируется по [Лицензии MIT](../LICENSE). Его можно использовать в проприетарном программном обеспечении, при условии, что все копии программного обеспечения или его значительные части содержат копию условий лицензии MIT и уведомление об авторских правах.

## Поиск спонсоров

Pyxel ищет спонсоров на GitHub Sponsors. Пожалуйста, подумайте о том, чтобы стать спонсором Pyxel, чтобы поддержать его дальнейшую поддержку и развитие функций. В качестве преимущества спонсоры могут консультироваться непосредственно с разработчиком Pyxel. Для получения дополнительной информации посетите [эту страницу](https://github.com/sponsors/kitao).
