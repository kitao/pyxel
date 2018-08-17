# <img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/assets/pyxel_logo_152x64.png">

[ [English](https://github.com/kitao/pyxel/blob/master/README.md) | [日本語](https://github.com/kitao/pyxel/blob/master/README.ja.md) | [Other Languages](https://github.com/kitao/pyxel/wiki) ]

**Pyxel** is a retro game development environment in Python.

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

The specifications of the gaming console, APIs, and palettes of Pyxel are referring to awesome [PICO-8](https://www.lexaloffle.com/pico-8.php) and [TIC-80](https://tic.computer/).

Pyxel is open source and free to use. Let's start making a retro game with Pyxel!

## Specifications

- Run on Windows, Mac, and Linux
- Code writing with Python3
- Fixed 16 color palette
- 256x256 sized 3 image banks
- 4 channels with 64 definable sound banks
- Keyboard, mouse, and joystick(WIP) inputs
- Image and sound editor (WIP)

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

Install the required packages in a way appropriate for each distribution. [glfw](http://www.glfw.org/) must be version 3.2.1 or higher.

**Arch:**

Install [`python-pixel`](https://aur.archlinux.org/packages/python-pyxel/) by using your favorite AUR helper:

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

## How to Use

### Create Pyxel Application

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

### Create Images

There are the following methods to create images for Pyxel:

- Create an image from a list of strings with `Image.set` function
- Load a png file in Pyxel palette with `Image.load` function
- Create images with Pyxel Editor (WIP)

Please refer to the API reference for usage of `Image.set` and `Image.load`.

Because Pyxel uses the same palette as [PICO-8](https://www.lexaloffle.com/pico-8.php), when creating png images for Pyxel, it is recommended to use [Aseprite](https://www.aseprite.org/) in PICO-8 palette mode.

## API Reference

### System

- `width`, `height`  
The width and height of the screen

- `frame_count`  
The number of the elapsed frames

- `init(width, height, [caption], [scale], [palette], [fps], [border_width], [border_color])`  
Initialize the Pyxel application with screen size (`width`, `height`). The maximum width and height of the screen is 256  
It is also possible to specify the window title with `caption`, the display magnification with `scale`, the palette color with `palette`, the frame rate with `fps`, and the margin width and color outside the screen with `border_width` and `border_color`. `palette` is specified as a list of 16 elements of 24 bit color, `border_color` as 24 bit color

- `run(update, draw)`  
Start the Pyxel application and call `update` function for frame update and `draw` function for drawing

- `quit()`  
End the Pyxel application at the end of the current frame

### Input
- `mouse_x`, `mouse_y`  
The current position of the mouse cursor

- `btn(key)`  
Return `True` if `key` is pressed, otherwise return `False` ([key definition list](https://github.com/kitao/pyxel/blob/master/pyxel/constants.py))

- `btnp(key, [hold], [period])`  
Return `True` if `key` is pressed at that frame, otherwise return `False`. When `hold` and `period` are specified, `True` will be returned at the `period` frame interval when the `key` is held down for more than `hold` frames

- `btnr(key)`  
Return `True` if `key` is released at that frame, otherwise return `False`

### Graphics

- `image(img, [system])`  
Operate the image bank `img`(0-2) (see the Image class). If `system` is `True`, the image bank 3 for system can be accessed  
e.g. `pyxel.image(0).load(0, 0, 'title.png')`

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

- `blt(x, y, img, sx, sy, w, h, [colkey])`  
Copy the region of size (`w`, `h`) from (`sx`, `sy`) of the image bank `img`(0-2) to (`x`, `y`). If negative value is set for `w` and/or `h`, it will reverse horizontally and/or vertically. If `colkey` is specified, treated as transparent color

- `text(x, y, s, col)`  
Draw a string `s` of color `col` at (`x`, `y`)

### Audio

- `sound(snd)`  
Operate the sound bank `snd`(0-63) (see the Sound class)
e.g. `pyxel.sound(0).speed = 60`

- `play(ch, snd, loop=False)`  
Play the sound bank `snd`(0-63) on channel `ch`(0-3). Play in order when `snd` is a list

- `stop(ch)`  
Stop playback of channel `ch`(0-3)

### Image Class

- `width`, `height`  
The width and height of the Image

- `data`  
The data of the Image (NumPy array)

- `set(x, y, data)`  
Set the image as a list of strings at (`x`, `y`)   
e.g. `pyxel.image(0).set(10, 10, ['1234', '5678', '9abc', 'defg'])`

- `load(x, y, filename, [dirname])`  
Read the png image from the directory of the execution script or `dirname` at (`x`, `y`)

- `copy(x, y, img, sx, sy, width, height)`  
Copy the region of size (`width`, `height`) from (`sx`, `sy`) of the image bank `img`(0-2) to (`x`, `y`)

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
Set the note with a string consists of 'CDEFGAB'+'#-'+'0123' or 'R'. Case-insensitive and whitespace is ignored  
e.g. `pyxel.sound(0).set_note('G2B-2D3R RF3F3F3')`

- `set_tone(tone)`  
Set the tone with a string consists of 'TSPN'. Case-insensitive and whitespace is ignored  
e.g. `pyxel.sound(0).set_tone('TTSS PPPN')`

- `set_volume(volume)`  
Set the volume with a string consists of '01234567'. Case-insensitive and whitespace is ignored  
e.g. `pyxel.sound(0).set_volume('7777 7531')`

- `set_effect(effect)`  
Set the effect with a string consists of 'NSVF'. Case-insensitive and whitespace is ignored  
e.g. `pyxel.sound(0).set_effect('NFNF NVVS')`

## Other Information

- [Pyxel Wiki](https://github.com/kitao/pyxel/wiki)

## License

Pyxel is under [MIT license](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software provided that all copies of the licensed software include a copy of the MIT License terms and the copyright notice.
