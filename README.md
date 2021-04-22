# <img src="images/pyxel_logo_152x64.png">

[ [English](README.md) | [中文](README.cn.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**Pyxel** is a retro game engine for Python.

Thanks to its simple specifications inspired by retro gaming consoles, such as only 16 colors can be displayed and only 4 sounds can be played back at the same time, you can feel free to enjoy making pixel art style games.

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

The specifications of the gaming console and APIs for Pyxel are referring to awesome [PICO-8](https://www.lexaloffle.com/pico-8.php) and [TIC-80](https://tic.computer/).

Pyxel is open source and free to use. Let's start making a retro game with Pyxel!

## Specifications

- Run on Windows, Mac, and Linux
- Code writing with Python3
- Fixed 16 color palette
- 256x256 sized 3 image banks
- 256x256 sized 8 tilemaps
- 4 channels with 64 definable sounds
- 8 musics which can combine arbitrary sounds
- Keyboard, mouse, and gamepad inputs
- Image and sound editor

### Color Palette

<img src="pyxel/examples/screenshots/05_color_palette.png">
<br><br>
<img src="images/pyxel_palette.png">

## How to Install

### Windows

First, install [Python3](https://www.python.org/) (version 3.6.8 or higher).

When you install Python with the official installer, **add Python to PATH** by checking the button below:

<img src="images/python_installer.png">

Next, install Pyxel with the following `pip` command from the command prompt:

```sh
pip install -U pyxel
```

### Mac

First, in the environment where [Homebrew](https://brew.sh/) package manager is installed, install [Python3](https://www.python.org/) (version 3.6.8 or higher) and the required packages with the following command:

```sh
brew install python3 gcc sdl2 sdl2_image gifsicle
```

You can install Python3 in other ways, but be aware that you must install other libraries.

Next, **restart the terminal** and install Pyxel with the `pip3` command:

```sh
pip3 install -U pyxel
```

### Linux

Install [Python3](https://www.python.org/) (version 3.6.8 or higher) and the required packages in a way appropriate for each distribution.

**Ubuntu:**

```sh
sudo apt install python3 python3-pip libsdl2-dev libsdl2-image-dev gifsicle
sudo -H pip3 install -U pyxel
```

### Other environment

To install Pyxel in an environment other than the above (32-bit Linux, Raspberry PI, etc.), follow the steps below for building:

#### Install necessary tools and packages

- C++ build toolchain (should include gcc and make command)
- libsdl2-dev and libsdl2-image-dev
- [Python3](https://www.python.org/) (version 3.6.8 or highter) and pip command

#### Execute the following command in any folder

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make -C pyxel/core clean all
pip3 install .
```

### Install examples

After installing Pyxel, the examples of Pyxel will be copied to the current directory with the following command:

```sh
install_pyxel_examples
```

The examples to be copied are as follows:

- [01_hello_pyxel.py](pyxel/examples/01_hello_pyxel.py) - Simplest application
- [02_jump_game.py](pyxel/examples/02_jump_game.py) - Jump game with Pyxel resource file
- [03_draw_api.py](pyxel/examples/03_draw_api.py) - Demonstration of drawing API
- [04_sound_api.py](pyxel/examples/04_sound_api.py) - Demonstration of sound API
- [05_color_palette.py](pyxel/examples/05_color_palette.py) - Color palette list
- [06_click_game.py](pyxel/examples/06_click_game.py) - Mouse click game
- [07_snake.py](pyxel/examples/07_snake.py) - Snake game with BGM
- [08_triangle_api.py](pyxel/examples/08_triangle_api.py) - Demonstration of triangle drawing API
- [09_shooter.py](pyxel/examples/09_shooter.py) - Shoot'em up game with screen transition

The examples can be executed like normal Python code:

**Windows:**

```sh
cd pyxel_examples
python 01_hello_pyxel.py
```

**Mac / Linux:**

```sh
cd pyxel_examples
python3 01_hello_pyxel.py
```

## How to Use

### Create a Pyxel Application

After importing the Pyxel module in your python code, specify the window size with `init` function first, then starts the Pyxel application with `run` function.

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

The arguments of `run` function are `update` function to update each frame and `draw` function to draw screen when necessary.

In an actual application, it is recommended to wrap pyxel code in a class as below:

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

It is also possible to write simple code using `show` and `flip` functions to draw simple graphics and animations.

The `show` function displays the screen and waits until the `ESC` key is pressed.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

The `flip` function updates the screen once.

```python
import pyxel

pyxel.init(120, 80)

while True:
    pyxel.cls(3)
    pyxel.rectb(pyxel.frame_count % 160 - 40, 20, 40, 40, 7)
    pyxel.flip()
```

### Special Controls

The following special controls can be performed while a Pyxel application is running:

- `Esc`<br>
Quit the application
- `Alt(Option)+1`<br>
Save the screenshot to the desktop
- `Alt(Option)+2`<br>
Reset the recording start time of the screen capture video
- `Alt(Option)+3`<br>
Save the screen capture video (gif) to the desktop (up to 30 seconds)
- `Alt(Option)+0`<br>
Toggle the performance monitor (fps, update time, and draw time)
- `Alt(Option)+Enter`<br>
Toggle full screen

### How to Create a Resource

The attached Pyxel Editor can create images and sounds used in a Pyxel application.

Pyxel Editor starts with the following command:

```sh
pyxeleditor [pyxel_resource_file]
```

If the specified Pyxel resource file (.pyxres) exists, the file is loaded, and if it does not exist, a new file is created with the specified name.
If the resource file is omitted, the name is `my_resource.pyxres`.

After starting Pyxel Editor, the file can be switched by dragging and dropping another resource file. If the resource file is dragged and dropped while holding down ``Ctrl``(``Cmd``) key, only the resource type (image/tilemap/sound/music) that is currently being edited will be loaded. This operation enables to combine multiple resource file into one.

The created resource file can be loaded with the `load` function.

Pyxel Editor has the following edit modes.

**Image Editor:**

The mode to edit the image banks.

<img src="pyxel/editor/screenshots/image_editor.gif">

By dragging and dropping a png file onto the Image Editor screen, the image can be loaded into the currently selected image bank.

**Tilemap Editor:**

The mode to edit tilemaps in which images of the image banks are arranged in a tile pattern.

<img src="pyxel/editor/screenshots/tilemap_editor.gif">

**Sound Editor:**

The mode to edit sounds.

<img src="pyxel/editor/screenshots/sound_editor.gif">

**Music Editor:**

The mode to edit musics in which the sounds are arranged in order of playback.

<img src="pyxel/editor/screenshots/music_editor.gif">

### Other resource creation methods

Pyxel images and tilemaps can also be created in the following way:

- Create an image from a list of strings with `Image.set` or `Tilemap.set` function
- Load a png file in Pyxel palette with `Image.load` function

Pyxel sounds can also be created in the following way:

- Create a sound from strings with `Sound.set` or `Music.set` function

Please refer to the API reference for usage of these functions.

### How to Create a Stand-Alone Executable

By using the attached Pyxel Packager, a stand-alone executable that will work even in environments where Python is not installed can be created.

To create a stand-alone executable, in the environment where [PyInstaller](https://www.pyinstaller.org/) is installed, specify the Python file to be used to launch the application with the `pyxelpackager` command as follows:

```sh
pyxelpackager python_file
```

When the process is completed, a stand-alone executable is created in the `dist` folder.

If resources such as .pyxres and .png files are also necessary, put them under the `assets` folder and they will be included.

It is also possible to specify an icon with the ``-i icon_file`` option.

## API Reference

### System

- `width`, `height`<br>
The width and height of the screen

- `frame_count`<br>
The number of the elapsed frames

- `init(width, height, [caption], [scale], [palette], [fps], [quit_key], [fullscreen])`<br>
Initialize the Pyxel application with screen size (`width`, `height`). The maximum width and height of the screen is 256<br>
It is also possible to specify the window title with `caption`, the display magnification with `scale`, the palette color with `palette`, the frame rate with `fps`, the key to quit the application with `quit_key`, and whether to start in full screen with `fullscreen`. `palette` is specified as a list of 16 elements of 24 bit color.<br>
e.g. `pyxel.init(160, 120, caption="Pyxel with PICO-8 palette", palette=[0x000000, 0x1D2B53, 0x7E2553, 0x008751, 0xAB5236, 0x5F574F, 0xC2C3C7, 0xFFF1E8, 0xFF004D, 0xFFA300, 0xFFEC27, 0x00E436, 0x29ADFF, 0x83769C, 0xFF77A8, 0xFFCCAA], quit_key=pyxel.KEY_NONE, fullscreen=True)`

- `run(update, draw)`<br>
Start the Pyxel application and call `update` function for frame update and `draw` function for drawing

- `quit()`<br>
Quit the Pyxel application at the end of the current frame

- `flip()`<br>
Force drawing the screen (do not use in normal applications)

- `show()`<br>
Draw the screen and wait forever (do not use in normal applications)

### Resource

- `save(filename)`<br>
Save the resource file (.pyxres) to the directory of the execution script

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Read the resource file (.pyxres) from the directory of the execution script. If ``False`` is specified for the resource type (image/tilemap/sound/music), the resource will not be loaded.

### Input
- `mouse_x`, `mouse_y`<br>
The current position of the mouse cursor

- `mouse_wheel`<br>
The current value of the mouse wheel

- `btn(key)`<br>
Return `True` if `key` is pressed, otherwise return `False` ([key definition list](pyxel/__init__.py))

- `btnp(key, [hold], [period])`<br>
Return `True` if `key` is pressed at that frame, otherwise return `False`. When `hold` and `period` are specified, `True` will be returned at the `period` frame interval when the `key` is held down for more than `hold` frames

- `btnr(key)`<br>
Return `True` if `key` is released at that frame, otherwise return `False`

- `mouse(visible)`<br>
If `visible` is `True`, show the mouse cursor. If `False`, hide it. Even if the mouse cursor is not displayed, its position is updated.

### Graphics

- `image(img, [system])`<br>
Operate the image bank `img`(0-2) (see the Image class). If `system` is `True`, the image bank for system can be accessed. 3 is for the font and resource editor. 4 is for the display screen<br>
e.g. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
Operate the tilemap `tm`(0-7) (see the Tilemap class)

- `clip(x, y, w, h)`<br>
Set the drawing area of the screen from (`x`, `y`) to width `w` and height `h`. Reset the drawing area to full screen with `clip()`

- `pal(col1, col2)`<br>
Replace color `col1` with `col2` at drawing. `pal()` to reset to the initial palette

- `cls(col)`<br>
Clear screen with color `col`

- `pget(x, y)`<br>
Get the color of the pixel at (`x`, `y`)

- `pset(x, y, col)`<br>
Draw a pixel of color `col` at (`x`, `y`)

- `line(x1, y1, x2, y2, col)`<br>
Draw a line of color `col` from (`x1`, `y1`) to (`x2`, `y2`)

- `rect(x, y, w, h, col)`<br>
Draw a rectangle of width `w`, height `h` and color `col` from (`x`, `y`)

- `rectb(x, y, w, h, col)`<br>
Draw the outline of a rectangle of width `w`, height `h` and color `col` from (`x`, `y`)

- `circ(x, y, r, col)`<br>
Draw a circle of radius `r` and color `col` at (`x`, `y`)

- `circb(x, y, r, col)`<br>
Draw the outline of a circle of radius `r` and color `col` at (`x`, `y`)

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
Draw a triangle with vertices (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) and color `col`

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
Draw the outline of a triangle with vertices (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) and color `col`

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
Copy the region of size (`w`, `h`) from (`u`, `v`) of the image bank `img`(0-2) to (`x`, `y`). If negative value is set for `w` and/or `h`, it will reverse horizontally and/or vertically. If `colkey` is specified, treated as transparent color

<img src="images/image_bank_mechanism.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Draw the tilemap `tm`(0-7) to (`x`, `y`) according to the tile information of size (`w`, `h`) from (`u`, `v`). If `colkey` is specified, treated as transparent color. A tile of the tilemap is drawn with a size of 8x8, and if the tile number is 0, indicates the region (0, 0)-(7, 7) of the image bank, if 1, indicates (8, 0)-(15, 0)

<img src="images/tilemap_mechanism.png">

- `text(x, y, s, col)`<br>
Draw a string `s` of color `col` at (`x`, `y`)

### Audio

- `sound(snd, [system])`<br>
Operate the sound `snd`(0-63) (see the Sound class). If `system` is `True`, the sound 64 for system can be accessed<br>
e.g. `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
Operate the music `msc`(0-7) (see the Music class)

- `play_pos(ch)`<br>
Get the sound playback position of channel `ch`. The 100's and 1000's indicate the sound number and the 1's and 10's indicate the note number. When playback is stopped, return `-1`

- `play(ch, snd, loop=False)`<br>
Play the sound `snd`(0-63) on channel `ch`(0-3). Play in order when `snd` is a list

- `playm(msc, loop=False)`<br>
Play the music `msc`(0-7)

- `stop([ch])`<br>
Stop playback of all channels. If `ch`(0-3) is specified, stop the corresponding channel only

### Image Class

- `width`, `height`<br>
The width and height of the image

- `data`<br>
The data of the image (256x256 two-dimentional list)

- `get(x, y)`<br>
Retrieve the data of the image at (`x`, `y`)

- `set(x, y, data)`<br>
Set the data of the image at (`x`, `y`) by a value or a list of strings<br>
e.g. `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Read the png image from the directory of the execution script at (`x`, `y`)

- `copy(x, y, img, u, v, w, h)`<br>
Copy the region of size (`w`, `h`) from (`u`, `v`) of the image bank `img`(0-2) to (`x`, `y`)

### Tilemap Class

- `width`, `height`<br>
The width and height of the tilemap

- `data`<br>
The data of the tilemap (256x256 two-dimentional list)

- `refimg`<br>
The image bank referenced by the tilemap

- `get(x, y)`<br>
Retrieve the data of the tilemap at (`x`, `y`)

- `set(x, y, data)`<br>
Set the data of the tilemap at (`x`, `y`) by a value or a list of strings.<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `copy(x, y, tm, u, v, w, h)`<br>
Copy the region of size (`w`, `h`) from (`u`, `v`) of the tilemap `tm`(0-7) to (`x`, `y`)

### Sound Class

- `note`<br>
List of note(0-127) (33 = 'A2' = 440Hz)

- `tone`<br>
List of tone(0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volume`<br>
List of volume(0-7)

- `effect`<br>
List of effects(0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`<br>
The length of one note(120 = 1 second per tone)

- `set(note, tone, volume, effect, speed)`<br>
Set a note, tone, volume, and effect with a string. If the tone, volume, and effect length are shorter than the note, it is repeated from the beginning

- `set_note(note)`<br>
Set the note with a string made of 'CDEFGAB'+'#-'+'0123' or 'R'. Case-insensitive and whitespace is ignored<br>
e.g. `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tone(tone)`<br>
Set the tone with a string made of 'TSPN'. Case-insensitive and whitespace is ignored<br>
e.g. `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volume(volume)`<br>
Set the volume with a string made of '01234567'. Case-insensitive and whitespace is ignored<br>
e.g. `pyxel.sound(0).set_volume("7777 7531")`

- `set_effect(effect)`<br>
Set the effect with a string made of 'NSVF'. Case-insensitive and whitespace is ignored<br>
e.g. `pyxel.sound(0).set_effect("NFNF NVVS")`

### Music Class

- `ch0`<br>
List of sound(0-63) play on channel 0. If an empty list is specified, the channel is not used for playback

- `ch1`<br>
List of sound(0-63) play on channel 1. If an empty list is specified, the channel is not used for playback

- `ch2`<br>
List of sound(0-63) play on channel 2. If an empty list is specified, the channel is not used for playback

- `ch3`<br>
List of sound(0-63) play on channel 3. If an empty list is specified, the channel is not used for playback

- `set(ch0, ch1, ch2, ch3)`<br>
Set the list of sound(0-63) of all channels. If an empty list is specified, that channel is not used for playback<br>
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

- `set_ch0(data)`<br>
Set the list of sound(0-63) of channel 0

- `set_ch1(data)`<br>
Set the list of sound(0-63) of channel 1

- `set_ch2(data)`<br>
Set the list of sound(0-63) of channel 2

- `set_ch3(data)`<br>
Set the list of sound(0-63) of channel 3

## How to Contribute

### Submitting an issue

Use the [issue tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature/enhancement requests.
Before submitting a new issue, search the issue tracker to ensure that there is no similar open issue.

When submitting a report, select the appropriate template from [this link](https://github.com/kitao/pyxel/issues/new/choose).

### Manual testing

Anyone manually testing the code and reporting bugs or suggestions for enhancements in the issue tracker are very welcome!

### Submitting a pull request

Patches/fixes are accepted in form of pull requests (PRs). Make sure the issue the pull request addresses is open in the issue tracker.

Submitted pull request is deemed to have agreed to publish under [MIT license](LICENSE).

## Other Information

- [Wiki](https://github.com/kitao/pyxel/wiki)
- [Subreddit](https://www.reddit.com/r/pyxel/)
- [Discord server (English)](https://discord.gg/FC7kUZJ)
- [Discord server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## License

Pyxel is under [MIT license](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software provided that all copies of the licensed software include a copy of the MIT License terms and the copyright notice.

Pyxel uses the following software:

- [SDL2](https://www.libsdl.org/)
- [miniz-cpp](https://github.com/tfussell/miniz-cpp)
- [Gifsicle](https://www.lcdf.org/gifsicle/)
