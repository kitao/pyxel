# <img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/assets/pyxel_logo_152x64.png">

[ [English](https://github.com/kitao/pyxel/blob/master/README.md) | [日本語](https://github.com/kitao/pyxel/blob/master/README.ja.md) | [中文](https://github.com/YifangSun/pyxel/blob/master/README.cn.md) | [Other Languages](https://github.com/kitao/pyxel/wiki) ]

**Pyxel**是一个python的经典像素风游戏制作引擎。

由于像素风游戏的机制非常简单（如：最多只能显示16种颜色、播放4种声音等），现在你也可以轻松地享受这种游戏的制作过程。

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

Pyxel的设计、API以及调色板参考了经典的[PICO-8](https://www.lexaloffle.com/pico-8.php)以及[TIC-80](https://tic.computer/)。

Pyxel是开源的，大家可以免费使用。现在就让我们一起用Pyxel制作自己的游戏吧！

## 说明

- 需要在Windows、Mac或Linux上运行
- 需要Python3
- 内置16色调色板
- 3个256x256的图像库
- 8个256x256的瓦片地图
- 4个音轨各含有64个可选音调
- 可任意组合8个音乐
- 支持键盘、鼠标及游戏手柄输入
- 图像和音频编辑器

### 调色板

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/05_color_palette.png">

## 如何安装

### Windows

安装[Python3](https://www.python.org/)(3.7或更高版本)之后，输入以下`pip`命令来安装Pyxel：

```sh
pip install -U pyxel
```

### Mac

安装[Python3](https://www.python.org/)(3.7或更高版本)以及[SDL2](https://www.libsdl.org/)之后，输入`pip`命令来安装Pyxel。

如果已安装[Homebrew](https://brew.sh/)，输入以下命令安装所有需要的包。

```sh
brew install python3 sdl2 sdl2_image
pip3 install -U pyxel
```

### Linux

为各linux发行版安装[Python3](https://www.python.org/)(3.7或更高版本)以及依赖包。

**Ubuntu:**

```sh
sudo apt install python3 python3-pip libsdl2-dev libsdl2-image-dev
sudo pip3 install -U pyxel
```

### 其他环境

为除上述外其他环境(32位Linux、树莓派等)安装Pyxel，请按以下步骤进行构建：

#### 安装所需的工具及依赖包

- C++构建工具链（包含gcc和make命令）
- libsdl2-dev和libsdl2-image-dev
- [Python3](https://www.python.org/)(3.7或更高版本)和pip工具

#### 任意文件夹中执行以下命令

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make -C pyxel/core clean all
pip3 install .
```

### 安装例程

安装Pyxel后，可以用以下命令将Pyxe例程复制到当前文件夹：

```sh
install_pyxel_examples
```

例程包含：

- [01_hello_pyxel.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/01_hello_pyxel.py) - 最简单的应用
- [02_jump_game.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/02_jump_game.py) - 用Pyxel制作的跳跃游戏
- [03_draw_api.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/03_draw_api.py) - 绘画API的示例
- [04_sound_api.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/04_sound_api.py) - 声音API的示例
- [05_color_palette.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/05_color_palette.py) - 调色板列表
- [06_click_game.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/06_click_game.py) - 鼠标点击游戏
- [07_snake.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/07_snake.py) - 带BGM的贪吃蛇游戏

这些例程可以像执行正常python程序一样运行：

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

## 使用教程

### 创建Pyxel应用

在python代码导入Pyxel模块后，首先用`init`函数指定窗口大小，然后用`run`函数启动Pyxel应用。

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

有时也可简单使用`show`和`flip`画出简单的画面和动画。

`show`函数可以显示画面直到`ESC`键按下。

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

`flip`函数可以更新一次画面。

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
保存屏幕录制动图（gif）到桌面（最多30秒）
- `Alt(Option)+0`<br>
切换性能监控（fps，更新时间，画面绘制时间）
- `Alt(Option)+Enter`<br>
切换全屏

### 如何创建源文件

内置Pyxel编辑器可以为Pyxel应用创建图片和音频。

输入以下命令启动Pyxel编辑器：

```sh
pyxeleditor [pyxel_resource_file]
```

若指定Pyxel源文件（.pyxres）存在，则加载文件，若不存在，则以指定文件名新建文件。

若未指定源文件，则命名为`my_resource.pyxres`。

启动Pyxel编辑器后，可以通过拖动来切换文件。

创建后的源文件可用`load`函数来加载。

Pyxel编辑器有以下编辑模式。

**图像编辑器：**

此模式用来编辑图像库。

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/image_editor.gif">

通过拖动png文件至图像编辑器界面，可以将图像加载至当选择前图像库。

**瓦片地图编辑器：**

此模式用来编辑瓦片地图，其中图像库的图像以瓦片的样式排列。

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/tilemap_editor.gif">

**音频编辑器：**

此模式用来编辑音频。

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/sound_editor.gif">

**音乐编辑器：**

此模式用来编辑将录音有序编排形成的音乐。

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/music_editor.gif">

### 其他创建源文件的方法

Pyxel图像和瓦片地图还可以通过以下方法创建：

- 在`Image.set`或`Tilemap.set`函数中通过字符串list来生成图像
- 在Pyxel调色板中用`Image.load`函数加载png文件

因为Pyxel使用了与[PICO-8](https://www.lexaloffle.com/pico-8.php)相同的调色板，所以在为Pyxel创建png图像时，建议使用PICO-8调色板模式中的[Aseprite](https://www.aseprite.org/)。

Pyxel音频也可以通过以下方法创建：

- 在`Sound.set`或`Music.set`函数中通过字符串来生成音频

这些函数的具体用法请查阅API参考手册。

### 如何创建独立可执行文件

使用内置的Pyxel  Packager创建独立的可执行文件，在没有python的环境下也可以执行。

使用`pyxelpackager`命令来指定打开启动应用的python文件，就可以创建可执行文件：

```sh
pyxelpackager python_file
```

进程结束后，可执行文件便会生成在`dist`文件夹下。

若应用必须包含.pyxres和.png文件，将其放在`assets`文件夹下，他们便会被打包进可执行文件中。

## API参考手册

### 系统

- `width`, `height`<br>
画面的宽和高

- `frame_count`<br>
经过的帧数

- `init(width, height, [caption], [scale], [palette], [fps], [border_width], [border_color])`<br>
初始化Pyxel应用的画面尺寸。画面的宽和高的最大值是256。<br>
同时可以用`caption`指定窗口标题，`scale`设定放大倍数，`palette`设定色调，`fps`设定帧率，`border_width`和`border_color`设定画面外白边的颜色和宽度。`palette`通过使用16个24位色彩元素的list来设定，`border_color`使用24位的色彩设定。

- `run(update, draw)`<br>
启动Pyxel应用并调用`update`更新帧、`draw`绘制画面。

- `quit()`<br>
当前帧结束后退出Pyxel应用。

- `flip()`<br>
强制绘制画面（通常应用中不会使用）。

- `show()`<br>
绘制画面并一直等待（通常应用中不会使用）。

### 源文件

- `save(filename)`<br>
保存源文件（.pyxres）到执行脚本的目录下。

- `load(filename)`<br>
从执行脚本的目录下读取源文件（.pyxres）。

### 输入
- `mouse_x`, `mouse_y`<br>
当前鼠标指针的位置。

- `btn(key)`<br>
如果`key`被按下则返回`True`，否则返回`False`([key definition list](https://github.com/kitao/pyxel/blob/master/pyxel/__init__.py))。

- `btnp(key, [hold], [period])`<br>
如果`key`被按下则返回`True`。若设置了`hold`和`period`参数，则当`key`被按下持续`hold`帧时，在`period`帧间隙返回`True`。

- `btnr(key)`<br>
如果`key`被松开，则在此帧返回`True`，否则返回`False`。

- `mouse(visible)`<br>
如果`visible`为`True`则显示鼠标指针，为`False`则不显示。即使鼠标指针不显示，其位置同样会被更新。

### 图像

- `image(img, [system])`<br>
操作图像库`img`(0-2)（参考Image类）。若`system`指定为`True`，则图像库可存取。
3对应字体和源文件编辑器，4对应显示画面。<br>
例：`pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
操作瓦片地图`tm`(0-7)（参考Tilemap类）

- `clip(x, y, w, h)`<br>
设置画面绘制区域为从(`x`, `y`)开始的宽度`w`、高度为`h`的区域。`clip()`可以将绘制区域重置为全屏。

- `pal(col1, col2)`<br>
绘制时用`col1`颜色代替`col2`颜色。`pal()`可以重置为初始色调。

- `cls(col)`<br>
用`col`颜色清空画面。

- `pix(x, y, col)`<br>
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

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
Copy the region of size (`w`, `h`) from (`u`, `v`) of the image bank `img`(0-2) to (`x`, `y`). If negative value is set for `w` and/or `h`, it will reverse horizontally and/or vertically. If `colkey` is specified, treated as transparent color

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Draw the tilemap `tm`(0-7) to (`x`, `y`) according to the tile information of size (`w`, `h`) from (`u`, `v`). If `colkey` is specified, treated as transparent color. A tile of the tilemap is drawn with a size of 8x8, and if the tile number is 0, indicates the region (0, 0)-(7, 7) of the image bank, if 1, indicates (8, 0)-(15, 0)

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

Submitted pull request is deemed to have agreed to publish under [MIT license](https://github.com/kitao/pyxel/blob/master/LICENSE).

## Other Information

- [Wiki](https://github.com/kitao/pyxel/wiki)
- [Subreddit](https://www.reddit.com/r/pyxel/)

## License

Pyxel is under [MIT license](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software provided that all copies of the licensed software include a copy of the MIT License terms and the copyright notice.

Pyxel uses the following libraries:

- [SDL2](https://www.libsdl.org/)
- [gif-h](https://github.com/ginsweater/gif-h)
- [miniz-cpp](https://github.com/tfussell/miniz-cpp)
- [PyInstaller](https://www.pyinstaller.org/)
