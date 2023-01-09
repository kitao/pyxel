# <img src="docs/images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](README.md) | [中文](docs/README.cn.md) | [Deutsch](docs/README.de.md) | [Español](docs/README.es.md) | [Français](docs/README.fr.md) | [Italiano](docs/README.it.md) | [日本語](docs/README.ja.md) | [한국어](docs/README.ko.md) | [Português](docs/README.pt.md) | [Русский](docs/README.ru.md) ]

**Pyxel** is a retro game engine for Python.

Thanks to its simple specifications inspired by retro gaming consoles, such as only 16 colors can be displayed and only 4 sounds can be played back at the same time, you can feel free to enjoy making pixel art style games.

<img src="docs/images/pyxel_300000_downloads.gif" width="480">

The motivation for the development of Pyxel is the feedback from users. Please give Pyxel a star on GitHub!

<p>
<a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">
<img src="docs/images/01_hello_pyxel.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">
<img src="docs/images/02_jump_game.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">
<img src="docs/images/03_draw_api.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">
<img src="docs/images/04_sound_api.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="docs/images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="docs/images/sound_music_editor.gif" width="320">
</a>
</p>

Pyxel's specifications and APIs are inspired by [PICO-8](https://www.lexaloffle.com/pico-8.php) and [TIC-80](https://tic80.com/).

Pyxel is open source and free to use. Let's start making a retro game with Pyxel!

## Specifications

- Runs on Windows, Mac, Linux, and Web
- Programming with Python
- 16 color palette
- 256x256 sized 3 image banks
- 256x256 sized 8 tilemaps
- 4 channels with 64 definable sounds
- 8 musics which can combine arbitrary sounds
- Keyboard, mouse, and gamepad inputs
- Image and sound editor

### Color Palette

<img src="docs/images/05_color_palette.png">

<img src="docs/images/pyxel_palette.png">

## How to Install

### Windows

After installing [Python3](https://www.python.org/) (version 3.7 or higher), run the following command:

```sh
pip install -U pyxel
```

If you install Python using the official installer, please check the `Add Python 3.x to PATH` checkbox to enable `pyxel` command.

### Mac

After installing [Python3](https://www.python.org/) (version 3.7 or higher), run the following command:

```sh
python3 -m pip install -U pyxel
```

If you use Python3, which is installed by default on Mac, please add `sudo` to the beginning of the above command to enable `pyxel` command.

### Linux

After installing the SDL2 package (`libsdl2-dev` for Ubuntu), [Python3](https://www.python.org/) (version 3.7 or higher), and `python3-pip`, run the following command:

```sh
sudo pip3 install -U pyxel
```

If the above doesn't work, try self-building according to the instructions in [Makefile](Makefile).

### Web

The web version of Pyxel does not require Python or Pyxel installation and runs on PCs as well as smartphones and tablets with supported web browsers.

For specific instructions, please refer to [this page](https://github.com/kitao/pyxel/wiki/How-To-Use-Pyxel-Web).

### Try Pyxel Examples

After installing Pyxel, the examples of Pyxel will be copied to the current directory with the following command:

```sh
pyxel copy_examples
```

The examples to be copied are as follows:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>Simplest application</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Code</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Jump game with Pyxel resource file</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Code</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Demonstration of drawing APIs</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Code</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Demonstration of sound APIs</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Code</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>Color palette list</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Code</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>Mouse click game</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Code</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>Snake game with BGM</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Code</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Demonstration of triangle drawing APIs</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Code</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Shoot'em up game with screen transition</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Code</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>Side-scrolling platform game with map</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Code</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Offscreen rendering with Image class</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Code</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>Perlin noise animation</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Code</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>Animation with flip function (non-web platforms only)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Code</a></td>
</tr>
<tr>
<td>30SecondsOfDaylight.pyxapp</td>
<td>1st Pyxel Jam winning game by Adam</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30SecondsOfDaylight.html">Demo</a></td>
<td><a href="https://github.com/kitao/30SecondsOfDaylight">Code</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>Arcade ball physics game by Adam</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Demo</a></td>
<td><a href="https://github.com/helpcomputer/megaball">Code</a></td>
</tr>
</table>

An examples can be executed with the following commands:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30SecondsOfDaylight.pyxapp
```

## How to Use

### Create Pyxel Application

After importing the Pyxel module in your python script, specify the window size with `init` function first, then starts the Pyxel application with `run` function.

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

When creating simple graphics without animation, `show` function can be used to make the code more concise.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Run Pyxel Application

The created Python script can be executed with the following command:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

It can also be executed like a normal Python script:

```sh
python3 PYTHON_SCRIPT_FILE
```

(For Windows, type `python` instead of `python3`)

### Special Controls

The following special controls can be performed while a Pyxel application is running:

- `Esc`<br>
  Quit the application
- `Alt(Option)+1`<br>
  Save the screenshot to the desktop
- `Alt(Option)+2`<br>
  Reset the recording start time of the screen capture video
- `Alt(Option)+3`<br>
  Save the screen capture video to the desktop (up to 10 seconds)
- `Alt(Option)+0`<br>
  Toggle the performance monitor (fps, update time, and draw time)
- `Alt(Option)+Enter`<br>
  Toggle full screen

### How to Create Resources

Pyxel Editor can create images and sounds used in a Pyxel application.

It starts with the following command:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

If the specified Pyxel resource file (.pyxres) exists, the file is loaded, and if it does not exist, a new file is created with the specified name.
If the resource file is omitted, the name is `my_resource.pyxres`.

After starting Pyxel Editor, the file can be switched by dragging and dropping another resource file. If the resource file is dragged and dropped while holding down `Ctrl(Cmd)` key, only the resource type (Image/Tilemap/Sound/Music) that is currently being edited will be loaded. This operation enables to combine multiple resource files into one.

The created resource file can be loaded with `load` function.

Pyxel Editor has the following edit modes.

**Image Editor:**

The mode to edit the image banks.

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="docs/images/image_editor.gif">
</a>

By dragging and dropping an image file (png/gif/jpeg) onto the Image Editor screen, the image can be loaded into the currently selected image bank.

**Tilemap Editor:**

The mode to edit tilemaps in which images of the image banks are arranged in a tile pattern.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="docs/images/tilemap_editor.gif">
</a>

**Sound Editor:**

The mode to edit sounds.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="docs/images/sound_editor.gif">
</a>

**Music Editor:**

The mode to edit musics in which the sounds are arranged in order of playback.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="docs/images/music_editor.gif">
</a>

### Other Resource Creation Methods

Pyxel images and tilemaps can also be created by the following methods:

- Create an image from a list of strings with `Image.set` function or `Tilemap.set` function
- Load an image file (png/gif/jpeg) in Pyxel palette with `Image.load` function

Pyxel sounds can also be created in the following method:

- Create a sound from strings with `Sound.set` function or `Music.set` function

Please refer to the API reference for usage of these functions.

### How to Distribute Applications

Pyxel supports a dedicated application distribution file format (Pyxel application file) that works across platforms.

Create the Pyxel application file (.pyxapp) with the following command:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

If the application should include resources or additional modules, place them in the application directory.

The created application file can be executed with the following command:

```sh
pyxel play PYXEL_APP_FILE
```

Pyxel application file also can be converted to an executable or an HTML file with the `pyxel app2exe` or `pyxel app2html` commands.

## API Reference

### System

- `width`, `height`<br>
  The width and height of the screen

- `frame_count`<br>
  The number of the elapsed frames

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Initialize the Pyxel application with screen size (`width`, `height`). The following can be specified as options: the window title with `title`, the frame rate with `fps`, the key to quit the application with `quit_key`, the scale of the display with `display_scale`, the scale of the screen capture with `capture_scale`, and the maximum recording time of the screen capture video with `capture_sec`.<br>
  e.g. `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Start the Pyxel application and call `update` function for frame update and `draw` function for drawing.

- `show()`<br>
  Show the screen and wait until the `Esc` key is pressed.

- `flip()`<br>
  Refrech the screen by one frame. The application exits when the `Esc` key is pressed. This function only works on non-web platforms.

- `quit()`<br>
  Quit the Pyxel application.

### Resource

- `load(filename, [image], [tilemap], [sound], [music])`<br>
  Load the resource file (.pyxres). If `False` is specified for the resource type (`image/tilemap/sound/music`), the resource will not be loaded. If a palette file (.pyxpal) of the same name exists in the same location as the resource file, the palette display color will also be changed. The palette file is a hexadecimal entry of the display colors, separated by newlines. The palette file can also be used to change the colors displayed in Pyxel Editor.

### Input

- `mouse_x`, `mouse_y`<br>
  The current position of the mouse cursor

- `mouse_wheel`<br>
  The current value of the mouse wheel

- `btn(key)`<br>
  Return `True` if `key` is pressed, otherwise return `False`. ([Key definition list](python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  Return `True` if `key` is pressed at that frame, otherwise return `False`. When `hold` and `repeat` are specified, `True` will be returned at the `repeat` frame interval when the `key` is held down for more than `hold` frames.

- `btnr(key)`<br>
  Return `True` if `key` is released at that frame, otherwise return `False`.

- `mouse(visible)`<br>
  If `visible` is `True`, show the mouse cursor. If `False`, hide it. Even if the mouse cursor is not displayed, its position is updated.

### Graphics

- `colors`<br>
  List of the palette display colors. The display color is specified by a 24-bit numerical value. Use `colors.from_list` and `colors.to_list` to directly assign and retrieve Python lists.<br>
  e.g. `org_colors = pyxel.colors.to_list(); org_colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img)`<br>
  Operate the image bank `img` (0-2). (See the Image class)<br>
  e.g. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
  Operate the tilemap `tm` (0-7). (See the Tilemap class)

- `clip(x, y, w, h)`<br>
  Set the drawing area of the screen from (`x`, `y`) to width `w` and height `h`. Reset the drawing area to full screen with `clip()`.

- `camera(x, y)`<br>
  Change the upper left corner coordinates of the screen to (`x`, `y`). Reset the upper left corner coordinates to (`0`, `0`) with `camera()`.

- `pal(col1, col2)`<br>
  Replace color `col1` with `col2` at drawing. `pal()` to reset to the initial palette.

- `cls(col)`<br>
  Clear screen with color `col`.

- `pget(x, y)`<br>
  Get the color of the pixel at (`x`, `y`).

- `pset(x, y, col)`<br>
  Draw a pixel of color `col` at (`x`, `y`).

- `line(x1, y1, x2, y2, col)`<br>
  Draw a line of color `col` from (`x1`, `y1`) to (`x2`, `y2`).

- `rect(x, y, w, h, col)`<br>
  Draw a rectangle of width `w`, height `h` and color `col` from (`x`, `y`).

- `rectb(x, y, w, h, col)`<br>
  Draw the outline of a rectangle of width `w`, height `h` and color `col` from (`x`, `y`).

- `circ(x, y, r, col)`<br>
  Draw a circle of radius `r` and color `col` at (`x`, `y`).

- `circb(x, y, r, col)`<br>
  Draw the outline of a circle of radius `r` and color `col` at (`x`, `y`).

- `elli(x, y, w, h, col)`<br>
  Draw an ellipse of width `w`, height `h` and color `col` from (`x`, `y`).

- `ellib(x, y, w, h, col)`<br>
  Draw the outline of an ellipse of width `w`, height `h` and color `col` from (`x`, `y`).

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  Draw a triangle with vertices (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) and color `col`.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  Draw the outline of a triangle with vertices (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) and color `col`.

- `fill(x, y, col)`<br>
  Fill the area connected with the same color as (`x`, `y`) with color `col`.

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
  Copy the region of size (`w`, `h`) from (`u`, `v`) of the image bank `img` (0-2) to (`x`, `y`). If negative value is set for `w` and/or `h`, it will reverse horizontally and/or vertically. If `colkey` is specified, treated as transparent color.

<img src="docs/images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
  Copy the region of size (`w`, `h`) from (`u`, `v`) of the tilemap `tm` (0-7) to (`x`, `y`). If negative value is set for `w` and/or `h`, it will reverse horizontally and/or vertically. If `colkey` is specified, treated as transparent color. The size of a tile is 8x8 pixels and is stored in a tilemap as a tuple of `(tile_x, tile_y)`.

<img src="docs/images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  Draw a string `s` of color `col` at (`x`, `y`).

### Audio

- `sound(snd)`<br>
  Operate the sound `snd` (0-63). (See the Sound class)<br>
  e.g. `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
  Operate the music `msc` (0-7). (See the Music class)

- `play_pos(ch)`<br>
  Get the sound playback position of channel `ch` (0-3) as a tuple of `(sound no, note no)`. Returns `None` when playback is stopped.

- `play(ch, snd, [tick], [loop])`<br>
  Play the sound `snd` (0-63) on channel `ch` (0-3). If `snd` is a list, it will be played in order. The playback start position can be specified by `tick` (1 tick = 1/120 seconds). If `True` is specified for `loop`, loop playback is performed.

- `playm(msc, [tick], [loop])`<br>
  Play the music `msc` (0-7). The playback start position can be specified by `tick` (1 tick = 1/120 seconds). If `True` is specified for `loop`, loop playback is performed.

- `stop([ch])`<br>
  Stops playback of the specified channel `ch` (0-3). `stop()` to stop playing all channels.

### Math

- `ceil(x)`<br>
  Returns the smallest integer greater than or equal to `x`.

- `floor(x)`<br>
  Returns the largest integer less than or equal to `x`.

- `sgn(x)`<br>
  Returns 1 when `x` is positive, 0 when it is zero, and -1 when it is negative.

- `sqrt(x)`<br>
  Returns the square root of `x`.

- `sin(deg)`<br>
  Returns the sine of `deg` degrees.

- `cos(deg)`<br>
  Returns the cosine of `deg` degrees.

- `atan2(y, x)`<br>
  Returns the arctangent of `y`/`x` in degrees.

- `rseed(seed: int)`<br>
  Sets the seed of the random number generator.

- `rndi(a, b)`<br>
  Returns an random integer greater than or equal to `a` and less than or equal to `b`.

- `rndf(a, b)`<br>
  Returns a random decimal greater than or equal to `a` and less than or equal to `b`.

- `nseed(seed)`<br>
  Sets the seed of Perlin noise.

- `noise(x, [y], [z])`<br>
  Returns the Perlin noise value for the specified coordinates.

### Image Class

- `width`, `height`<br>
  The width and height of the image

- `set(x, y, data)`<br>
  Set the image at (`x`, `y`) by a list of strings.<br>
  e.g. `pyxel.image(0).set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Load the image file (png/gif/jpeg) at (`x`, `y`).

- `pget(x, y)`<br>
  Get the pixel color at (`x`, `y`).

- `pset(x, y, col)`<br>
  Draw a pixel of color `col` at (`x`, `y`).

### Tilemap Class

- `width`, `height`<br>
  The width and height of the tilemap

- `refimg`<br>
  The image bank (0-2) referenced by the tilemap

- `set(x, y, data)`<br>
  Set the tilemap at (`x`, `y`) by a list of strings.<br>
  e.g. `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `pget(x, y)`<br>
  Get the tile at (`x`, `y`). A tile is a tuple of `(tile_x, tile_y)`.

- `pset(x, y, tile)`<br>
  Draw a `tile` at (`x`, `y`). A tile is a tuple of `(tile_x, tile_y)`.

### Sound Class

- `notes`<br>
  List of notes (0-127). The higher the number, the higher the pitch, and at 33 it becomes 'A2'(440Hz). The rest is -1.

- `tones`<br>
  List of tones (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
  List of volumes (0-7)

- `effects`<br>
  List of effects (0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`<br>
  Playback speed. 1 is the fastest, and the larger the number, the slower the playback speed. At 120, the length of one note becomes 1 second.

- `set(notes, tones, volumes, effects, speed)`<br>
  Set notes, tones, volumes, and effects with a string. If the tones, volumes, and effects length are shorter than the notes, it is repeated from the beginning.

- `set_notes(notes)`<br>
  Set the notes with a string made of 'CDEFGAB'+'#-'+'0123' or 'R'. Case-insensitive and whitespace is ignored.<br>
  e.g. `pyxel.sound(0).set_notes("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
  Set the tones with a string made of 'TSPN'. Case-insensitive and whitespace is ignored.<br>
  e.g. `pyxel.sound(0).set_tones("TTSS PPPN")`

- `set_volumes(volumes)`<br>
  Set the volumes with a string made of '01234567'. Case-insensitive and whitespace is ignored.<br>
  e.g. `pyxel.sound(0).set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  Set the effects with a string made of 'NSVF'. Case-insensitive and whitespace is ignored.<br>
  e.g. `pyxel.sound(0).set_effects("NFNF NVVS")`

### Music Class

- `snds_list`<br>
  Two-dimensional list of sounds (0-63) with the number of channels

- `set(snds0, snds1, snds2, snds3)`<br>
  Set the lists of sound (0-63) of all channels. If an empty list is specified, that channel is not used for playback.<br>
  e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### Advanced APIs

Pyxel has "advanced APIs" that are not mentioned in this reference because they "may confuse users" or "need specialized knowledge to use".

If you are familiar with your skills, try to create amazing works with [this](python/pyxel/__init__.pyi) as a clue!

## How to Contribute

### Submitting Issues

Use the [Issue Tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature/enhancement requests. Before submitting a new issue, ensure that there is no similar open issue.

### Manual Testing

Anyone manually testing the code and reporting bugs or suggestions for enhancements in the [Issue Tracker](https://github.com/kitao/pyxel/issues) are very welcome!

### Submitting Pull Requests

Patches/fixes are accepted in form of pull requests (PRs). Make sure the issue the pull request addresses is open in the Issue Tracker.

Submitted pull request is deemed to have agreed to publish under [MIT License](LICENSE).

## Other Information

- [Q&A](https://github.com/kitao/pyxel/wiki/Pyxel-Q&A)
- [User Examples](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Discord Server (English)](https://discord.gg/Z87eYHN)
- [Discord Server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)
- [Developer's Twitter account](https://twitter.com/kitao)

## License

Pyxel is under [MIT License](LICENSE). It can be reused within proprietary software, provided that all copies of the software or its substantial portions include a copy of the terms of the MIT License and also a copyright notice.

## Recruiting Sponsors

Pyxel is looking for sponsors on GitHub Sponsors. Consider sponsoring Pyxel for continued maintenance and feature additions. Sponsors can consult about Pyxel as a benefit. Please see [here](https://github.com/sponsors/kitao) for details.
