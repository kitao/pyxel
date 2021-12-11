# <img src="images/pyxel_logo_152x64.png">

[ [English](README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**NOTE: This manual has not yet been translated for Pyxel version 1.5.0. We are looking for volunteers to translate and check for mistakes!**

**Pyxel**是一个python的经典像素风游戏制作引擎。

由于像素风游戏的机制非常简单（如：最多只能显示16种颜色、播放4种声音等），现在你也可以轻松地享受这种游戏的制作过程。

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

The specifications of Pyxel are referring to awesome [PICO-8](https://www.lexaloffle.com/pico-8.php) and [TIC-80](https://tic.computer/).

Pyxel是开源的，大家可以免费使用。现在就让我们一起用Pyxel制作自己的游戏吧！

## 说明

- 需要在Windows、Mac或Linux上运行
- Programming with Python
- 16 color palette
- 3个256x256的图像库
- 8个256x256的瓦片地图
- 4个声道各含有64个可选音调
- 可任意组合8个音乐
- 支持键盘、鼠标及游戏手柄输入
- 图像和音频编辑器

### 调色板

<img src="pyxel/examples/screenshots/05_color_palette.png">
<br><br>
<img src="images/pyxel_palette.png">

## 如何安装

There are two types of Pyxel, a packaged version and a standalone version.

### Install the Packaged Version

The packaged version of Pyxel uses Pyxel as a Python extension module.

Recommended for those who are familiar with managing Python packages using the `pip` command or who want to develop full-fledged Python applications.

**Windows**

After installing [Python3](https://www.python.org/) (version 3.7 or higher), run the following command:

```sh
pip install -U pyxel
```

**Mac**

After installing [Python3](https://www.python.org/) (version 3.7 or higher), run the following command:

```sh
pip3 install -U pyxel
```

### Linux

After installing the SDL2 package (`libsdl2-dev` for Ubuntu), [Python3](https://www.python.org/) (version 3.7 or higher), and `python3-pip`, run the following command:

```sh
pip3 install -U pyxel
```

If the above doesn't work, try self-building by following the steps below after installing `cmake` and `rust`:

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make clean all RELEASE=1
pip3 install .
```

### Install the Standalone Version

The standalone version of Pyxel uses Pyxel as a standalone tool that does not depend on Python.

Recommended for those who want to start programming easily without worrying about Python settings, or those who want to play Pyxel games immediately.

**Windows**

Download and run the latest version of the Windows installer (`pyxel-[version]-windows-setup.exe`) from the [Download Page](https://github.com/kitao/pyxel/releases).

**Mac**

After installing [Homebrew](https://brew.sh/), run the following commands:

```sh
brew tap kitao/pyxel
brew install pyxel
```

**Linux**

After installing the SDL2 package (`libsdl2-dev` for Ubuntu) and installing [Homebrew](https://docs.brew.sh/Homebrew-on-Linux), run the following commands:

```sh
brew tap kitao/pyxel
brew install pyxel
```

If the above doesn't work, try self-building the packaged version.

### 安装例程

安装Pyxel后，可以用以下命令将Pyxe例程复制到当前文件夹：

```sh
pyxel copy_examples
```

例程包含：

- [01_hello_pyxel.py](pyxel/examples/01_hello_pyxel.py) - 最简单的应用
- [02_jump_game.py](pyxel/examples/02_jump_game.py) - 用Pyxel制作的跳跃游戏
- [03_draw_api.py](pyxel/examples/03_draw_api.py) - Demonstration of drawing APIs
- [04_sound_api.py](pyxel/examples/04_sound_api.py) - Demonstration of sound APIs
- [05_color_palette.py](pyxel/examples/05_color_palette.py) - 调色板列表
- [06_click_game.py](pyxel/examples/06_click_game.py) - 鼠标点击游戏
- [07_snake.py](pyxel/examples/07_snake.py) - 带BGM的贪吃蛇游戏
- [08_triangle_api.py](pyxel/examples/08_triangle_api.py) - Demonstration of triangle drawing APIs
- [09_shooter.py](pyxel/examples/09_shooter.py) - 屏幕过渡射击游戏
- [10_platformer.py](pyxel/examples/10_platformer.py) - Side-scrolling platform game with map

An examples can be executed with the following commands:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
```

For the packaged version, it can be executed like a normal Python script:

```sh
cd pyxel_examples
python3 01_hello_pyxel.py
```

(For Windows, type `python` instead of `python3`)

## 使用教程

### 创建Pyxel应用

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

`run`函数的两个参数`update`函数和`draw`函数分别用来在需要时更新帧和绘制画面。

实际应用中，建议将pyxel代码封装成如下类：

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

It is also possible to write simple code using `show` function and `flip` function to draw simple graphics and animations.

`show` function displays the screen and waits until the `Esc` key is pressed.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

`flip` function updates the screen once.

```python
import pyxel

pyxel.init(120, 80)

while True:
    pyxel.cls(3)
    pyxel.rectb(pyxel.frame_count % 160 - 40, 20, 40, 40, 7)
    pyxel.flip()
```

### 快捷键

以下快捷键可以在Pyxel运行时使用：

- `Esc`<br>
退出应用
- `Alt(Option)+1`<br>
截屏并保存在桌面
- `Alt(Option)+2`<br>
重置屏幕录制的开始时间
- `Alt(Option)+3`<br>
保存屏幕录制动图到桌面（最多10秒）
- `Alt(Option)+0`<br>
切换性能监控（fps，更新时间，画面绘制时间）
- `Alt(Option)+Enter`<br>
切换全屏

### 如何创建源文件

Pyxel Editor can create images and sounds used in a Pyxel application.

It starts with the following command:

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

若指定Pyxel源文件（.pyxres）存在，则加载文件，若不存在，则以指定文件名新建文件。

若未指定源文件，则命名为`my_resource.pyxres`。

After starting Pyxel Editor, the file can be switched by dragging and dropping another resource file. If the resource file is dragged and dropped while holding down ``Ctrl(Cmd)`` key, only the resource type (Image/Tilemap/Sound/Music) that is currently being edited will be loaded. This operation enables to combine multiple resource files into one.

The created resource file can be loaded with `load` function.

Pyxel编辑器有以下编辑模式。

**图像编辑器：**

此模式用来编辑图像库。

<img src="pyxel/editor/screenshots/image_editor.gif">

By dragging and dropping an image file (png/gif/jpeg) onto the Image Editor screen, the image can be loaded into the currently selected image bank.

**瓦片地图(Tilemap)编辑器：**

此模式用来编辑瓦片地图，其中图像库的图像以瓦片的样式排列。

<img src="pyxel/editor/screenshots/tilemap_editor.gif">

**音频编辑器：**

此模式用来编辑音频。

<img src="pyxel/editor/screenshots/sound_editor.gif">

**音乐编辑器：**

此模式用来编辑将录音有序编排形成的音乐。

<img src="pyxel/editor/screenshots/music_editor.gif">

### 其他创建源文件的方法

Pyxel images and tilemaps can also be created by the following methods:

- Create an image from a list of strings with `Image.set` function or `Tilemap.set` function
- Load an image file (png/gif/jpeg) in Pyxel palette with `Image.load` function

Pyxel sounds can also be created in the following method:

- Create a sound from strings with `Sound.set` function or `Music.set` function

这些函数的具体用法请查阅API参考手册。

### How to Distribute an Application

Pyxel supports a dedicated application distribution file format (Pyxel application file) that works across platforms.

Create the Pyxel application file (.pyxapp) with the following command:

```sh
pyxel package APP_ROOT_DIR STARTUP_SCRIPT_FILE
```

If the application should include resources or additional modules, place them in the application folder.

The created application file can be executed with the following command:

```sh
pyxel play PYXEL_APP_FILE
```

## API参考手册

### 系统

- `width`, `height`<br>
画面的宽和高

- `frame_count`<br>
经过的帧数

- `init(width, height, [title], [fps], [quit_key], [capture_sec])`<br>
Initialize the Pyxel application with screen size (`width`, `height`). The following can be specified as options: the window title with `title`, the frame rate with `fps`, the key to quit the application with `quit_key`, and the maximum recording time of the screen capture video with `capture_sec`.<br>
e.g. `pyxel.init(160, 120, title="Pyxel with Options", fps=60, quit_key=pyxel.KEY_NONE, capture_sec=0)`

- `run(update, draw)`<br>
Start the Pyxel application and call `update` function for frame update and `draw` function for drawing.

- `show()`<br>
Show the screen and wait until the `Esc` key is pressed. (Do not use in normal applications)

- `flip()`<br>
Updates the screen once. (Do not use in normal applications)

- `quit()`<br>
Quit the Pyxel application at the end of the current frame.

### 源文件

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Load the resource file (.pyxres). If ``False`` is specified for the resource type (``image/tilemap/sound/music``), the resource will not be loaded.

### 输入
- `mouse_x`, `mouse_y`<br>
当前鼠标指针的位置。

- `mouse_wheel`<br>
当前鼠标滚轮的值。

- `btn(key)`<br>
如果`key`被按下则返回`True`，否则返回`False`([按键定义列表](pyxel/__init__.pyi))。

- `btnp(key, [hold], [period])`<br>
如果`key`被按下则返回`True`。若设置了`hold`和`period`参数，则当`key`被按下持续`hold`帧时，在`period`帧间隙返回`True`。

- `btnr(key)`<br>
如果`key`被松开，则在此帧返回`True`，否则返回`False`。

- `mouse(visible)`<br>
如果`visible`为`True`则显示鼠标指针，为`False`则不显示。即使鼠标指针不显示，其位置同样会被更新。

### 显示

- `colors`<br>
List of the palette display colors. The display color is specified by a 24-bit numerical value. Use `colors.from_list` and `colors.to_list` to directly assign and retrieve Python lists.<br>
e.g. `org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img)`<br>
Operate the image bank `img` (0-2). (See the Image class)<br>
e.g. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
操作瓦片地图`tm`(0-7)（参考Tilemap类）

- `clip(x, y, w, h)`<br>
设置画面绘制区域为从(`x`, `y`)开始的宽度`w`、高度为`h`的区域。`clip()`可以将绘制区域重置为全屏。

- `pal(col1, col2)`<br>
绘制时用`col1`颜色代替`col2`颜色。`pal()`可以重置为初始色调。

- `cls(col)`<br>
用`col`颜色清空画面。

- `pget(x, y)`<br>
获取(`x`, `y`)处的像素颜色。

- `pset(x, y, col)`<br>
用`col`颜色在(`x`, `y`)处绘制一个像素点。

- `line(x1, y1, x2, y2, col)`<br>
用`col`颜色画一条从(`x1`, `y1`)到(`x2`, `y2`)的直线。

- `rect(x, y, w, h, col)`<br>
用`col`颜色绘制一个从(`x`, `y`)开始的宽为`w`、高为`h`的矩形。

- `rectb(x, y, w, h, col)`<br>
用`col`颜色绘制从(`x`, `y`)开始的宽为`w`、高为`h`的矩形边框。

- `circ(x, y, r, col)`<br>
用`col`颜色绘制圆心为(`x`, `y`)，半径为`r`的圆形。

- `circb(x, y, r, col)`<br>
用`col`颜色绘制圆心为(`x`, `y`)，半径为`r`的圆形边框。

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
用`col`颜色绘制顶点分别为(`x1`, `y1`)，(`x2`, `y2`)，(`x3`, `y3`)的三角形。

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
用`col`颜色绘制顶点分别为(`x1`, `y1`)，(`x2`, `y2`)，(`x3`, `y3`)的三角形边框。

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
将尺寸为(`w`, `h`)的区域从图像库的(`u`, `v`)复制到(`x`, `y`)。若`w`或`h`为负值，则在水平或垂直方向上翻转。若指定了`colkey`的值，则视作透明颜色。

<img src="images/image_bank_mechanism.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Draw the tilemap `tm` (0-7) to (`x`, `y`) according to the tile information of size (`w`, `h`) from (`u`, `v`). If `colkey` is specified, treated as transparent color. The size of a tile is 8x8 pixels and is stored in a tilemap as a tuple of `(x-in-tile, y-in-tile)`.

- `text(x, y, s, col)`<br>
用`col`颜色在(`x`, `y`)绘制字符串`s`。

### 声音

- `sound(snd)`<br>
操作音频`snd`(0-63)。（参考Sound类）<br>
示例：`pyxel.sound(0).speed = 60`

- `music(msc)`<br>
操作音乐`msc`(0-7)（参考Music类）

- `play_pos(ch)`<br>
Get the sound playback position of channel `ch` (0-3) as a tuple of `(sound-no, note-no)`. Returns `None` when playback is stopped.

- `play(ch, snd, loop=False)`<br>
Play the sound `snd` (0-63) on channel `ch` (0-3). If `snd` is a list, it will be played in order. If `True` is specified for `loop`, loop playback is performed.

- `playm(msc, loop=False)`<br>
Play the music `msc` (0-7). If `True` is specified for `loop`, loop playback is performed.

- `stop([ch])`<br>
Stops playback of the specified channel `ch` (0-3). `stop()` to stop playing all channels.

### Image类

- `width`, `height`<br>
图像的宽和高。

- `data`<br>
图像中的数据（256x256的二维列表）。

- `get(x, y)`<br>
获取图像中(`x`, `y`)位置的值。

- `set(x, y, data)`<br>
Set the image at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Load the image file (png/gif/jpeg) at (`x`, `y`).

### Tilemap类

- `width`, `height`<br>
瓦片地图(tilemap)的宽和高。

- `refimg`<br>
The image bank (0-2) referenced by the tilemap

- `set(x, y, data)`<br>
Set the tilemap at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `pget(x, y)`<br>
Get the tile at (`x`, `y`). A tile is a tuple of `(x-in-tile, y-in-tile)`.

- `pset(x, y, tile)`<br>
Draw a `tile` at (`x`, `y`). A tile is a tuple of `(x-in-tile, y-in-tile)`.

### Sound类

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
e.g. `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
Set the tones with a string made of 'TSPN'. Case-insensitive and whitespace is ignored.<br>
e.g. `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volumes(volumes)`<br>
Set the volumes with a string made of '01234567'. Case-insensitive and whitespace is ignored.<br>
e.g. `pyxel.sound(0).set_volume("7777 7531")`

- `set_effects(effects)`<br>
Set the effects with a string made of 'NSVF'. Case-insensitive and whitespace is ignored.<br>
e.g. `pyxel.sound(0).set_effect("NFNF NVVS")`

### Music类

- `sequences`<br>
Two-dimensional list of sounds (0-63) listed by the number of channels

- `set(seq0, seq1, seq2, seq3)`<br>
Set the lists of sound (0-63) of all channels. If an empty list is specified, that channel is not used for playback.<br>
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### Advanced APIs

Pyxel has "advanced APIs" that are not mentioned in this reference because they "may confuse users" or "need specialized knowledge to use".

If you are familiar with your skills, try to create amazing works with [this](pyxel/__init__.pyi) as a clue!

## 如何参与

### Submitting an Issue

Use the [Issue Tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature/enhancement requests. Before submitting a new issue, ensure that there is no similar open issue.

### Manual Testing

Anyone manually testing the code and reporting bugs or suggestions for enhancements in the [Issue Tracker](https://github.com/kitao/pyxel/issues) are very welcome!

### Submitting a Pull Request

可以通过pull requests(PRs)形式来提交补丁或修复。请确认你的pull request对应的issue地址在issue tracker中依然是open状态。

一旦提交pull request，则默认同意在[MIT License](LICENSE)的许可下发布。

## 其他信息

- [Discord server (English)](https://discord.gg/FC7kUZJ)
- [Discord server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## 许可证

Pyxel is under [MIT License](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software, provided that all copies of the software or its substantial portions include a copy of the terms of the MIT License and also a copyright notice.
