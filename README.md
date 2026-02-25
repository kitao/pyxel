# <img src="docs/images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](README.md) | [中文](docs/README.cn.md) | [Deutsch](docs/README.de.md) | [Español](docs/README.es.md) | [Français](docs/README.fr.md) | [Italiano](docs/README.it.md) | [日本語](docs/README.ja.md) | [한국어](docs/README.ko.md) | [Português](docs/README.pt.md) | [Русский](docs/README.ru.md) | [Türkçe](docs/README.tr.md) | [Українська](docs/README.uk.md) ]

**Pyxel** (/ˈpɪksəl/) is a retro game engine for Python.

With simple specifications inspired by retro gaming consoles, such as displaying only 16 colors and supporting 4 sound channels, you can easily enjoy making pixel-art-style games.

[<img src="docs/images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="docs/images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

The development of Pyxel is driven by user feedback. Please give Pyxel a star on GitHub!

<p>
<a href="https://kitao.github.io/pyxel/wasm/showcase/examples/10-platformer.html">
<img src="docs/images/10_platformer.gif" width="290">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/apps/30sec-of-daylight.html">
<img src="docs/images/30sec_of_daylight.gif" width="350">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/examples/02-jump-game.html">
<img src="docs/images/02_jump_game.gif" width="330">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/apps/megaball.html">
<img src="docs/images/megaball.gif" width="310">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="docs/images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
<img src="docs/images/sound_music_editor.gif" width="320">
</a>
</p>

Pyxel's specifications and APIs are inspired by [PICO-8](https://www.lexaloffle.com/pico-8.php) and [TIC-80](https://tic80.com/).

Pyxel is open source under the [MIT License](LICENSE) and free to use. Let's start making retro games with Pyxel!

## Specifications

- Runs on Windows, Mac, Linux, and Web
- Programming in Python
- Customizable screen size
- 16-color palette
- 3 256x256 image banks
- 8 256x256 tilemaps
- 4 channels with 64 definable sounds
- 8 music tracks composed of sounds
- Keyboard, mouse, and gamepad inputs
- Image and sound editing tools
- User-extensible colors, sound channels, and banks

### Color Palette

<img src="docs/images/05_color_palette.png">

<img src="docs/images/pyxel_palette.png">

## How to Install

### Windows

After installing [Python 3](https://www.python.org/) (version 3.8 or higher), run the following command:

```sh
pip install -U pyxel
```

When installing Python using the official installer, make sure to check the `Add Python 3.x to PATH` option to enable the `pyxel` command.

### Mac

After installing [Homebrew](https://brew.sh/), run the following commands:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

To upgrade Pyxel after installation, run `pipx upgrade pyxel`.

### Linux

After installing [Python 3](https://www.python.org/) (version 3.8 or higher), run the following command:

```sh
pip install -U pyxel
```

If the previous command fails, consider building Pyxel from source by following the instructions in the [Makefile](Makefile).

### Web

The web version of Pyxel works on PCs, smartphones, and tablets with a compatible browser, without installing Python or Pyxel.

The easiest way to use it is through the online IDE [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/).

For other usage patterns, such as embedding Pyxel apps on your own site, please refer to [this page](docs/pyxel-web-en.md).

## Basic Usage

### Pyxel Command

Installing Pyxel adds the `pyxel` command. Specify a command name after `pyxel` to perform various operations.

Run it without arguments to see the list of available commands:

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

### Try Examples

The following command copies Pyxel examples to the current directory:

```sh
pyxel copy_examples
```

Examples can be viewed and run in the browser from [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/).

You can run examples locally with the following commands:

```sh
# Run example in examples directory
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# Run app in examples/apps directory
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

## Creating Applications

### Create a Program

In your Python script, import Pyxel, set the window size with `init`, and start the application with `run`.

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

The arguments of the `run` function are the `update` function, which processes frame updates, and the `draw` function, which handles screen drawing.

In an actual application, it is recommended to wrap Pyxel code in a class, as shown below:

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

For creating simple graphics without animation, you can use the `show` function to simplify your code.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Run a Program

A created script can be executed using the `python` command:

```sh
python PYTHON_SCRIPT_FILE
```

It can also be run with the `pyxel run` command:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Additionally, the `pyxel watch` command monitors changes in a specified directory and automatically re-runs the program when changes are detected:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

Stop directory monitoring by pressing `Ctrl(Command)+C`.

### Special Key Controls

The following special key actions are available while a Pyxel application is running:

- `Esc`<br>
  Quit the application
- `Alt(Option)+R` or `A+B+X+Y+BACK` on gamepad<br>
  Reset the application
- `Alt(Option)+1`<br>
  Save the screenshot to the desktop
- `Alt(Option)+2`<br>
  Reset the recording start time of the screen capture video
- `Alt(Option)+3`<br>
  Save a screen capture video to the desktop (up to 10 seconds)
- `Alt(Option)+8` or `A+B+X+Y+DL` on gamepad<br>
  Toggle screen scaling between maximum and integer
- `Alt(Option)+9` or `A+B+X+Y+DR` on gamepad<br>
  Switch between screen modes (Crisp/Smooth/Retro)
- `Alt(Option)+0` or `A+B+X+Y+DU` on gamepad<br>
  Toggle the performance monitor (FPS/`update` time/`draw` time)
- `Alt(Option)+Enter` or `A+B+X+Y+DD` on gamepad<br>
  Toggle fullscreen
- `Shift+Alt(Option)+1/2/3`<br>
  Save image bank 0, 1, or 2 to the desktop
- `Shift+Alt(Option)+0`<br>
  Save the current color palette to the desktop

## Creating Resources

### Pyxel Editor

Pyxel Editor creates images and sounds used in a Pyxel application.

You can start Pyxel Editor with the following command:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

If the specified Pyxel resource file (.pyxres) exists, it will be loaded. If it does not exist, a new file with the specified name will be created. If the resource file is omitted, a new file named `my_resource.pyxres` will be created.

After starting Pyxel Editor, you can switch to another resource file by dragging and dropping it onto the editor.

The created resource file can be loaded using the `load` function.

Pyxel Editor has the following editing modes.

**Image Editor**

The mode for editing images in each **image bank**.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="docs/images/image_editor.gif">
</a>

You can drag and drop an image file (PNG/GIF/JPEG) into the image editor to load the image into the currently selected image bank.

**Tilemap Editor**

The mode for editing **tilemaps** that arrange images from the image banks in a tile pattern.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/tilemap-editor.html">
<img src="docs/images/tilemap_editor.gif">
</a>

Drag and drop a TMX file (Tiled Map File) onto the tilemap editor to load its layer 0 into the currently selected tilemap.

**Sound Editor**

The mode for editing **sounds** used for melodies and sound effects.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
<img src="docs/images/sound_editor.gif">
</a>

**Music Editor**

The mode for editing **music tracks** in which the sounds are arranged in order of playback.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/music-editor.html">
<img src="docs/images/music_editor.gif">
</a>

### Other Creation Methods

Pyxel images and tilemaps can also be created using the following methods:

- Create images or tilemaps from lists of strings with the `Image.set` or `Tilemap.set` functions
- Load palette-ready image files (PNG/GIF/JPEG) with the `Image.load` function

Pyxel sounds and music can also be created using the following method:

- Create them from strings with the `Sound.set` or `Music.set` functions

Refer to the API reference for the usage of these functions.

## Distributing Applications

Pyxel supports a cross-platform distribution format called a Pyxel application file.

Create a Pyxel application file (.pyxapp) with the `pyxel package` command:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

If you need to include resources or additional modules, place them in the application directory.

Metadata can be displayed at runtime by specifying it in the following format within the startup script. Fields other than `title` and `author` are optional.

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

The created application file can be run using the `pyxel play` command:

```sh
pyxel play PYXEL_APP_FILE
```

A Pyxel application file can also be converted to an executable or an HTML file using the `pyxel app2exe` or `pyxel app2html` commands.

## API Reference

A complete list of Pyxel APIs is available at [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/).

Pyxel also includes an "Advanced API" that requires specialized knowledge. You can view it by checking the "Advanced" checkbox on the reference page.

If you're confident in your skills, try using the Advanced API to create truly amazing works!

## How to Contribute

### Submitting Issues

Use the [Issue Tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature or enhancement requests. Before submitting a new issue, make sure there are no similar open issues.

### Functional Testing

Anyone who manually tests the code and reports bugs or suggestions for enhancements in the [Issue Tracker](https://github.com/kitao/pyxel/issues) is very welcome!

### Submitting Pull Requests

Patches and fixes are accepted in the form of pull requests (PRs). Make sure that the issue the pull request addresses is open in the Issue Tracker.

Submitting a pull request implies that you agree to license your contribution under the [MIT License](LICENSE).

## Web Tools & Examples

- [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)
- [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/) [[User Manual](https://qiita.com/kitao/items/b5b3fb28ebf9781eda2e)]
- [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/) [[User Manual](https://qiita.com/kitao/items/a86de4f7d6a0ed656a89)]

## Other Information

- [FAQ](docs/faq-en.md)
- [User Examples](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Developer's X Account](https://x.com/kitao)
- [Discord Server (English)](https://discord.gg/Z87eYHN)
- [Discord Server (Japanese)](https://discord.gg/qHA5BCS)

## License

Pyxel is licensed under the [MIT License](LICENSE). It can be reused in proprietary software, provided that all copies of the software or its substantial portions include a copy of the MIT License terms and a copyright notice.

## Recruiting Sponsors

Pyxel is looking for sponsors on GitHub Sponsors. Please consider sponsoring Pyxel to support its continued maintenance and feature development. As a benefit, sponsors can consult directly with the Pyxel developer. For more details, please visit [this page](https://github.com/sponsors/kitao).
