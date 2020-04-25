# <img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/images/pyxel_logo_152x64.png">

[ [English](https://github.com/kitao/pyxel/blob/master/README.md) | [日本語](https://github.com/kitao/pyxel/blob/master/README.ja.md) | [中文](https://github.com/kitao/pyxel/blob/master/README.cn.md) | [한국어](https://github.com/kitao/pyxel/blob/master/README.ko.md) | [Español](https://github.com/kitao/pyxel/blob/master/README.es.md) | [Português](https://github.com/kitao/pyxel/blob/master/README.pt.md) ]

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

游戏控制台以及API的设计参考了经典的[PICO-8](https://www.lexaloffle.com/pico-8.php)以及[TIC-80](https://tic.computer/)。

Pyxel是开源的，大家可以免费使用。现在就让我们一起用Pyxel制作自己的游戏吧！

## 说明

- 需要在Windows、Mac或Linux上运行
- 需要Python3
- 内置16色调色板
- 3个256x256的图像库
- 8个256x256的瓦片地图
- 4个声道各含有64个可选音调
- 可任意组合8个音乐
- 支持键盘、鼠标及游戏手柄输入
- 图像和音频编辑器

### 调色板

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/05_color_palette.png">
<br><br>
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/assets/pyxel_palette.png">

## 如何安装

### Windows

第一步，安装[Python3](https://www.python.org/)(3.6.9或更高版本)。

如果使用官方安装器来安装python，不要忘记勾选下图选项**将python添加到环境变量：**

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/python_installer.png">

第二步, 在命令提示符中输入以下`pip`指令直接安装pyxel：

```sh
pip install -U pyxel
```

### Mac

安装[Python3](https://www.python.org/)(3.6.9或更高版本)以及[SDL2](https://www.libsdl.org/)之后，输入`pip`命令来安装Pyxel。

如果已安装[Homebrew](https://brew.sh/)，输入以下命令安装所有需要的包。

```sh
brew install python3 sdl2 sdl2_image
```

**重启终端**之后：

```sh
pip3 install -U pyxel
```

### Linux

为各linux发行版安装[Python3](https://www.python.org/)(3.6.9或更高版本)及其依赖包。

**Ubuntu:**

```sh
sudo apt install python3 python3-pip libsdl2-dev libsdl2-image-dev
sudo -H pip3 install -U pyxel
```

### 其他环境

为除上述外其他环境(32位Linux、树莓派等)安装Pyxel，请按以下步骤进行构建：

#### 安装所需的工具及依赖包

- C++构建工具链（包含gcc和make命令）
- libsdl2-dev和libsdl2-image-dev
- [Python3](https://www.python.org/)(3.6.9或更高版本)和pip工具

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
- [08_triangle_api.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/08_triangle_api.py) - 三角形绘图示例

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

启动Pyxel编辑器后，可以通过拖放来切换文件。若在按下``Ctrl``(``Cmd``)键的同时拖放源文件，则只有当前正在编辑的类型(image/tilemap/sound/music)会被加载。通过本操作可以将多个源文件合并为一个。

创建后的源文件可用`load`函数来加载。

Pyxel编辑器有以下编辑模式。

**图像编辑器：**

此模式用来编辑图像库。

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/image_editor.gif">

通过拖动png文件至图像编辑器界面，可以将图像加载至当选择前图像库。

**瓦片地图(Tilemap)编辑器：**

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

可以使用``-i icon_file``指令自定义应用图标。

## API参考手册

### 系统

- `width`, `height`<br>
画面的宽和高

- `frame_count`<br>
经过的帧数

- `init(width, height, [caption], [scale], [palette], [fps], [quit_key], [fullscreen])`<br>
初始化Pyxel应用的画面尺寸。画面的宽和高的最大值是256。<br>
同时可以用`caption`指定窗口标题，`scale`设定放大倍数，`palette`设定色调，`fps`设定帧率，`quit_key`可指定退出键, `fullscreen`设置是否全屏启动。其中`palette`为16个24位真彩色元素的list。<br>
例：`pyxel.init(160, 120, caption="Pyxel with PICO-8 palette", palette=[0x000000, 0x1D2B53, 0x7E2553, 0x008751, 0xAB5236, 0x5F574F, 0xC2C3C7, 0xFFF1E8, 0xFF004D, 0xFFA300, 0xFFEC27, 0x00E436, 0x29ADFF, 0x83769C, 0xFF77A8, 0xFFCCAA], quit_key=pyxel.KEY_NONE, fullscreen=True)`

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

- `load(filename, [image], [tilemap], [sound], [music])`<br>
从执行脚本的目录下读取源文件（.pyxres）。如果某一源文件类型指定为False，则对应类型不会被加载。

### 输入
- `mouse_x`, `mouse_y`<br>
当前鼠标指针的位置。

- `mouse_wheel`<br>
当前鼠标滚轮的值。

- `btn(key)`<br>
如果`key`被按下则返回`True`，否则返回`False`([按键定义列表](https://github.com/kitao/pyxel/blob/master/pyxel/__init__.py))。

- `btnp(key, [hold], [period])`<br>
如果`key`被按下则返回`True`。若设置了`hold`和`period`参数，则当`key`被按下持续`hold`帧时，在`period`帧间隙返回`True`。

- `btnr(key)`<br>
如果`key`被松开，则在此帧返回`True`，否则返回`False`。

- `mouse(visible)`<br>
如果`visible`为`True`则显示鼠标指针，为`False`则不显示。即使鼠标指针不显示，其位置同样会被更新。

### 显示

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

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
根据从(`u`, `v`)开始的尺寸为(`w`, `h`)的tail信息，将瓦片地图(tilemap)`tm`(0-7)绘制到(`x`, `y`)处。若指定了`colkey`的值，则视作透明颜色。瓦片地图(tilemap)中一个tail尺寸为8x8。若tail编号为0，代表图像库中(0, 0)-(7, 7)的区域，若编号为1，代表(8, 0)-(15, 0)的区域。

- `text(x, y, s, col)`<br>
用`col`颜色在(`x`, `y`)绘制字符串`s`。

### 声音

- `sound(snd, [system])`<br>
操作音频`snd`(0-63)（参考Sound类）。若`system`为`True`，则sound 64可存取<br>
示例：`pyxel.sound(0).speed = 60`

- `music(msc)`<br>
操作音乐`msc`(0-7)（参考Music类）

- `play_pos(ch)`<br>
获取`ch`声道的音频当前播放到的位置。个位数和十位数表示note的值，百位数和千位数表示sound的数字。当播放停止时，返回-1。

- `play(ch, snd, loop=False)`<br>
在声道`ch`(0-3)播放音频`snd`(0-63)。当`snd`是列表时，按顺序播放。

- `playm(msc, loop=False)`<br>
播放音乐`msc`(0-7)

- `stop([ch])`<br>
停止所有声道的播放。若指定了`ch`(0-3)，则只停止对应声道。

### Image类

- `width`, `height`<br>
图像的宽和高。

- `data`<br>
图像中的数据（256x256的二维列表）。

- `get(x, y)`<br>
获取图像中(`x`, `y`)位置的值。

- `set(x, y, data)`<br>
将图像中(`x`, `y`)位置的值设置为字符串列表的值。<br>
示例：`pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
从执行脚本所在的文件夹加载png文件到(`x`, `y`)

- `copy(x, y, img, u, v, w, h)`<br>
将图像库`img`(0-2)中从(`u`, `v`)开始的尺寸为(`w`, `h`)的区域复制到(`x`, `y`)

### Tilemap类

- `width`, `height`<br>
瓦片地图(tilemap)的宽和高。

- `data`<br>
瓦片地图中的数据（256x256的二维列表）

- `refimg`<br>
瓦片地图中引用的图像库。

- `get(x, y)`<br>
获取瓦片地图中(`x`, `y`)位置的值。

- `set(x, y, data)`<br>
将瓦片地图中(`x`, `y`)位置的值设置为字符串列表的值。<br>
示例：`pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `copy(x, y, tm, u, v, w, h)`<br>
将瓦片地图`tm`(0-7)中从(`u`, `v`)开始的尺寸为(`w`, `h`)的区域复制到(`x`, `y`)

### Sound类

- `note`<br>
note（音符）列表(0-127) (33 = 'A2' = 440Hz)

- `tone`<br>
tone（音调）列表(0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volume`<br>
volume（音量）列表(0-7)

- `effect`<br>
effect（音效）列表(0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`<br>
一个note（音符）的长度(120 = 1 second per tone)

- `set(note, tone, volume, effect, speed)`<br>
用字符串来设置note，tone，volume和effect。若tone，volume，和effect的长度比note短，则将其循环处理。

- `set_note(note)`<br>
用'CDEFGAB'+'#-'+'0123'或'R'组成的字符串来设置note。不区分大小写，不计入空格。<br>
示例：`pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tone(tone)`<br>
用'TSPN'组成的字符串设置tone。不区分大小写，不计入空格。<br>
示例：`pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volume(volume)`<br>
用'01234567'组成的字符串设置volume。不区分大小写，不计入空格。<br>
示例：`pyxel.sound(0).set_volume("7777 7531")`

- `set_effect(effect)`<br>
用'NSVF'组成的字符串设置effect。不区分大小写，不计入空格。<br>
示例：`pyxel.sound(0).set_effect("NFNF NVVS")`

### Music类

- `ch0`<br>
声道0中播放的sound(0-63)列表。若列表为空，则此声道未被使用。

- `ch1`<br>
声道1中播放的sound(0-63)列表。若列表为空，则此声道未被使用。

- `ch2`<br>
声道2中播放的sound(0-63)列表。若列表为空，则此声道未被使用。

- `ch3`<br>
声道3中播放的sound(0-63)列表。若列表为空，则此声道未被使用。

- `set(ch0, ch1, ch2, ch3)`<br>
设置所有声道的音频sound(0-63)播放列表。若指定了空列表，则对应声道未被使用。<br>
示例：`pyxel.music(0).set([0, 1], [2, 3], [4], [])`

- `set_ch0(data)`<br>
设置声道0的音频sound(0-63)播放列表。

- `set_ch1(data)`<br>
设置声道1的音频sound(0-63)播放列表。

- `set_ch2(data)`<br>
设置声道2的音频sound(0-63)播放列表。

- `set_ch3(data)`<br>
设置声道3的音频sound(0-63)播放列表。

## 如何参与

### 提交问题
使用[issue tracker](https://github.com/kitao/pyxel/issues)来提交bug报告或功能需求。
提交问题之前，请搜索issue tracker以确认没有人提出过类似的问题。

提交报告时，从[这里](https://github.com/kitao/pyxel/issues/new/choose)选取合适的模板。

### 手动测试

欢迎大家手动测试代码并提交bug，或者提出改进意见！

### 提交pull request

可以通过pull requests(PRs)形式来提交补丁或修复。请确认你的pull request对应的issue地址在issue tracker中依然是open状态。

一旦提交pull request，则默认同意在[MIT license](https://github.com/kitao/pyxel/blob/master/LICENSE)的许可下发布。

## 其他信息

- [Wiki](https://github.com/kitao/pyxel/wiki)
- [Subreddit](https://www.reddit.com/r/pyxel/)
- [Discord server](https://discord.gg/FC7kUZJ)

## 许可证

Pyxel开源在[MIT license](http://en.wikipedia.org/wiki/MIT_License)下，你可以将pyxel用在你的软件中，但同时所述软件的所有版本都必须包含MIT License许可条款及版权声明。

Pyxel使用了以下库：

- [SDL2](https://www.libsdl.org/)
- [miniz-cpp](https://github.com/tfussell/miniz-cpp)
- [PyInstaller](https://www.pyinstaller.org/)
