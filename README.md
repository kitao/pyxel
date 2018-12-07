# <img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/assets/pyxel_logo_152x64.png">

[ [English](https://github.com/kitao/pyxel/blob/master/README.md) | [日本語](https://github.com/kitao/pyxel/blob/master/README.ja.md) | [Other Languages](https://github.com/kitao/pyxel/wiki) ]

**Pyxel** is a retro game engine for Python.

Thanks to its simple specifications inspired by retro gaming consoles, such as only 16 colors can be displayed and only 4 sounds can be played back at the same time, you can feel free to enjoy making pixel art style games. 

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/01_hello_pyxel.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/01_hello_pyxel.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/02_jump_game.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/02_jump_game.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/03_draw_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/03_draw_api.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/04_sound_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/04_sound_api.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/editor/screenshots/image_tilemap_editor.gif" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/image_tilemap_editor.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/editor/screenshots/sound_music_editor.gif" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/sound_music_editor.gif" width="48%">
</a>

The specifications of the gaming console, APIs, and palettes of Pyxel are referring to awesome [PICO-8](https://www.lexaloffle.com/pico-8.php) and [TIC-80](https://tic.computer/).

Pyxel is open source and free to use. Let's start making a retro game with Pyxel!

## Specifications

- Run on Windows and Mac (Linux version is under development)
- Code writing with Python3
- Fixed 16 color palette
- 256x256 sized 3 image banks
- 256x256 sized 8 tilemaps
- 4 channels with 64 definable sounds
- 8 musics which can combine arbitrary sounds
- Keyboard, mouse, and gamepad inputs
- Image and sound editor

### Color Palette

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/05_color_palette.png">

## How to Install

### Windows

After installing [Python3](https://www.python.org/), the following `pip` command installs Pyxel:

```sh
pip install pyxel
```

### Mac

After installing [Python3](https://www.python.org/) and [glfw](http://www.glfw.org/) (version 3.2.1 or higher), install Pyxel with `pip` command.

If [Homebrew](https://brew.sh/) package manager is ready, the following command installs all the necessary packages:

```sh
brew install python3 glfw
pip3 install pyxel
```

### Linux

**NOTE: Because Linux version is under development, drawing and sound playback do not work properly.**

Install the required packages in a way appropriate for each distribution. [glfw](http://www.glfw.org/) must be version 3.2.1 or higher.

**Arch:**

Install [`python-pyxel`](https://aur.archlinux.org/packages/python-pyxel/) by using your favorite AUR helper:

```sh
yay -S python-pyxel
```

**Debian:**

```sh
apt-get install python3 python3-pip libglfw3 libportaudio2 libasound-dev
pip3 install pyxel
```

**Fedora:**

```sh
dnf install glfw portaudio
pip3 install pyxel
```

### Install examples

After installing Pyxel, the examples of Pyxel will be copied to the current directory with the following command:

```sh
install_pyxel_examples
```

The examples can be executed like normal Python code:

```sh
cd pyxel_examples
python 01_hello_pyxel.py
```

or

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
        pyxel.rect(self.x, 0, self.x + 7, 7, 9)

App()
```

### Special Controls

The following special controls can be performed while a Pyxel application is running:

- `Alt(Option)+1`  
Save the screenshot to the desktop
- `Alt(Option)+2`  
Reset the recording start time of the screen capture video
- `Alt(Option)+3`  
Save the screen capture video (gif) to the desktop (up to 30 seconds)
- `Alt(Option)+0`  
Toggle the performance monitor (fps, update time, and draw time)
- `Alt(Option)+Enter`  
Toggle full screen

### Pyxel Editor

The attached Pyxel Editor can create images and sounds used in a Pyxel application.

Pyxel Editor runs with an arbitrary resource file name.

```sh
pyxeleditor pyxel_resource_file
```

The created resource file (.pyxel) can be loaded with the `load` function.

Pyxel Editor has the following edit modes.

**Image Editor:**

The mode to edit the image banks.

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/image_editor.gif">

**Tilemap Editor:**

The mode to edit tilemaps in which images of the image banks are arranged in a tile pattern.

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/tilemap_editor.gif">

**Sound Editor:**

The mode to edit sounds.

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/sound_editor.gif">

**Music Editor:**

The mode to edit musics in which the sounds are arranged in order of playback.

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/music_editor.gif">

### Other resource creation methods

Pyxel images and tilemaps can also be created in the following way:

- Create an image from a list of strings with `Image.set` or `Tilemap.set` function
- Load a png file in Pyxel palette with `Image.load` function

Because Pyxel uses the same palette as [PICO-8](https://www.lexaloffle.com/pico-8.php), when creating png images for Pyxel, it is recommended to use [Aseprite](https://www.aseprite.org/) in PICO-8 palette mode.

Pyxel sounds can also be created in the following way:

- Create a sound from strings with `Sound.set` or `Music.set` function

Please refer to the API reference for usage of these functions.

## API Reference

### System

- `width`, `height`  
The width and height of the screen

- `frame_count`  
The number of the elapsed frames

- `init(width, height, [caption], [scale], [palette], [fps], [border_width], [border_color])`  
Initialize the Pyxel application with screen size (`width`, `height`). The maximum width and height of the screen is 255  
It is also possible to specify the window title with `caption`, the display magnification with `scale`, the palette color with `palette`, the frame rate with `fps`, and the margin width and color outside the screen with `border_width` and `border_color`. `palette` is specified as a list of 16 elements of 24 bit color, `border_color` as 24 bit color

- `run(update, draw)`  
Start the Pyxel application and call `update` function for frame update and `draw` function for drawing

- `quit()`  
End the Pyxel application at the end of the current frame

### Resource

- `save(filename)`  
Save the resource file (.pyxel) to the directory of the execution script

- `load(filename)`  
Read the resource file (.pyxel) from the directory of the execution script

### Input
- `mouse_x`, `mouse_y`  
The current position of the mouse cursor

- `btn(key)`  
Return `True` if `key` is pressed, otherwise return `False` ([key definition list](https://github.com/kitao/pyxel/blob/master/pyxel/constants.py))

- `btnp(key, [hold], [period])`  
Return `True` if `key` is pressed at that frame, otherwise return `False`. When `hold` and `period` are specified, `True` will be returned at the `period` frame interval when the `key` is held down for more than `hold` frames

- `btnr(key)`  
Return `True` if `key` is released at that frame, otherwise return `False`

- `mouse(visible)`  
If `visible` is `True`, show the mouse cursor. If `False`, hide it. Even if the mouse cursor is not displayed, its position is updated.

### Graphics

- `image(img, [system])`  
Operate the image bank `img`(0-2) (see the Image class). If `system` is `True`, the image bank 3 for system can be accessed  
e.g. `pyxel.image(0).load(0, 0, 'title.png')`

- `tilemap(tm)`  
Operate the tilemap `tm`(0-7) (see the Tilemap class)

- `clip(x1, y1, x2, y2)`  
Set the drawing area of the screen to (`x1`, `y1`)-(`x2`, `y2`). Reset the drawing area with `clip()`

- `pal(col1, col2)`  
Replace color `col1` with `col2` at drawing. `pal()` to reset to the initial palette

- `cls(col)`  
Clear screen with color `col`

- `pix(x, y, col)`  
Draw a pixel of color `col` at (`x`, `y`)

- `line(x1, y1, x2, y2, col)`  
Draw a line of color `col` from (`x1`, `y1`) to (`x2`, `y2`)

- `rect(x1, y1, x2, y2, col)`  
Draw a rectangle of color `col` from (`x1`, `y1`) to (`x2`, `y2`)

- `rectb(x1, y1, x2, y2, col)`  
Draw the outline of a rectangle of color `col` from (`x1`, `y1`) to (`x2`, `y2`)

- `circ(x, y, r, col)`  
Draw a circle of radius `r` and color `col` at (`x`, `y`)

- `circb(x, y, r, col)`  
Draw the outline of a circle of radius `r` and color `col` at (`x`, `y`)

- `blt(x, y, img, u, v, w, h, [colkey])`  
Copy the region of size (`w`, `h`) from (`u`, `v`) of the image bank `img`(0-2) to (`x`, `y`). If negative value is set for `w` and/or `h`, it will reverse horizontally and/or vertically. If `colkey` is specified, treated as transparent color

- `bltm(x, y, tm, u, v, w, h, [colkey])`  
Draw the tilemap `tm`(0-7) to (`x`, `y`) according to the tile information of size (`w`, `h`) from (`u`, `v`). If `colkey` is specified, treated as transparent color. A tile of the tilemap is drawn with a size of 8x8, and if the tile number is 0, indicates the region (0, 0)-(7, 7) of the image bank, if 1, indicates (8, 0)-(15, 0)

- `text(x, y, s, col)`  
Draw a string `s` of color `col` at (`x`, `y`)

### Audio

- `sound(snd, [system])`  
Operate the sound `snd`(0-63) (see the Sound class). If `system` is `True`, the sound 64 for system can be accessed  
e.g. `pyxel.sound(0).speed = 60`

- `music(msc)`  
Operate the music `msc`(0-7) (see the Music class)

- `play(ch, snd, loop=False)`  
Play the sound `snd`(0-63) on channel `ch`(0-3). Play in order when `snd` is a list

- `playm(msc, loop=False)`  
Play the music `msc`(0-7)

- `stop([ch])`  
Stop playback of all channels. If `ch`(0-3) is specified, stop the corresponding channel only

### Image Class

- `width`, `height`  
The width and height of the image

- `data`  
The data of the image (NumPy array)

- `get(x, y)`  
Retrieve the data of the image at (`x`, `y`)

- `set(x, y, data)`  
Set the data of the image at (`x`, `y`) by a value or a list of strings   
e.g. `pyxel.image(0).set(10, 10, ['1234', '5678', '9abc', 'defg'])`

- `load(x, y, filename)`  
Read the png image from the directory of the execution script at (`x`, `y`)

- `copy(x, y, img, u, v, w, h)`  
Copy the region of size (`w`, `h`) from (`u`, `v`) of the image bank `img`(0-2) to (`x`, `y`)

### Tilemap Class

- `width`, `height`  
The width and height of the tilemap

- `data`  
The data of the tilemap (NumPy array)

- `refimg`  
The image bank referenced by the tilemap

- `get(x, y)`  
Retrieve the data of the tilemap at (`x`, `y`)

- `set(x, y, data, [refimg])`  
Set the data of the tilemap at (`x`, `y`) by a value or a list of strings. If `refimg` is specified, the image bank referenced by the tilemap is also set  
e.g. `pyxel.tilemap(0).set(0, 0, ['000102', '202122', 'a0a1a2', 'b0b1b2'], 1)`

- `copy(x, y, tm, u, v, w, h)`  
Copy the region of size (`w`, `h`) from (`u`, `v`) of the tilemap `tm`(0-7) to (`x`, `y`)

### Sound Class

- `note`  
List of note(0-127) (33 = 'A2' = 440Hz)

- `tone`  
List of tone(0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volume`  
List of volume(0-7)

- `effect`  
List of effects(0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`  
The length of one note(120 = 1 second per tone)

- `set(note, tone, volume, effect, speed)`  
Set a note, tone, volume, and effect with a string. If the tone, volume, and effect length are shorter than the note, it is repeated from the beginning

- `set_note(note)`  
Set the note with a string made of 'CDEFGAB'+'#-'+'0123' or 'R'. Case-insensitive and whitespace is ignored  
e.g. `pyxel.sound(0).set_note('G2B-2D3R RF3F3F3')`

- `set_tone(tone)`  
Set the tone with a string made of 'TSPN'. Case-insensitive and whitespace is ignored  
e.g. `pyxel.sound(0).set_tone('TTSS PPPN')`

- `set_volume(volume)`  
Set the volume with a string made of '01234567'. Case-insensitive and whitespace is ignored  
e.g. `pyxel.sound(0).set_volume('7777 7531')`

- `set_effect(effect)`  
Set the effect with a string made of 'NSVF'. Case-insensitive and whitespace is ignored  
e.g. `pyxel.sound(0).set_effect('NFNF NVVS')`

### Music Class

- `ch0`  
List of sound(0-63) play on channel 0

- `ch1`  
List of sound(0-63) play on channel 1

- `ch2`  
List of sound(0-63) play on channel 2

- `ch3`  
List of sound(0-63) play on channel 3

- `set(ch0, ch1, ch2, ch3)`  
Set the list of sound(0-63) of all channels  
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

- `set_ch0(data)`  
Set the list of sound(0-63) of channel 0

- `set_ch1(data)`  
Set the list of sound(0-63) of channel 1

- `set_ch2(data)`  
Set the list of sound(0-63) of channel 2

- `set_ch3(data)`  
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

Submitted pull request is deemed to have agreed to publish under [MIT license](https://github.com/kitao/pyxel/blob/master/LICENSE).

## Other Information

- [Wiki](https://github.com/kitao/pyxel/wiki)
- [Subreddit](https://www.reddit.com/r/pyxel/)

## License

Pyxel is under [MIT license](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software provided that all copies of the licensed software include a copy of the MIT License terms and the copyright notice.
