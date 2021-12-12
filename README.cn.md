# <img src="images/pyxel_logo_152x64.png">

[ [English](README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

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

Pyxel的设计规范参考了[PICO-8](https://www.lexaloffle.com/pico-8.php)和[TIC-80](https://tic.computer/)。

Pyxel是开源的，大家可以免费使用。现在就让我们一起用Pyxel制作自己的游戏吧！

## 说明

- 需要在Windows、Mac或Linux上运行
- 可以使用python进行编程
- 16色调色板
- 3个256x256的图像库
- 8个256x256的瓦片地图
- 4个音轨，每个各可含有64个音符
- 可任意组合8个音乐
- 支持键盘、鼠标及游戏手柄输入
- 图像和音频编辑器

### 调色板

<img src="pyxel/examples/screenshots/05_color_palette.png">
<br><br>
<img src="images/pyxel_palette.png">

## 如何安装

Pyxel有两种安装版本：Python包版本和独立版本。

### 安装Python包版本

此版本以Python扩展包的方式使用Pyxel。

推荐用户：可以熟练使用pip进行Python包管理，或需要基于已经成熟的Python应用进行开发的用户。

**Windows**

在安装[Python3](https://www.python.org/)（3.7或更高版本）之后，执行以下命令：

```sh
pip install -U pyxel
```

**Mac**

在安装[Python3](https://www.python.org/)（3.7或更高版本）之后，执行以下命令：

```sh
pip3 install -U pyxel
```

**Linux**

安装SDL2（Ubuntu下包名为：`libsdl2-dev`），[Python3](https://www.python.org/)（3.7或更高版本），以及`python3-pip`这三个包之后，执行以下命令：

```sh
pip3 install -U pyxel
```

如果以上步骤无效，可以在安装`cmake`和`rust`后，尝试执行以下步骤自行编译：

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make clean all RELEASE=1
pip3 install .
```

### 安装独立版本

此版本可以直接使用Pyxel而无需依赖Python。

推荐用户：不想进行复杂的Python配置，或想要尽快上手Pyxel游戏的用户。

**Windows**

从[下载页面](https://github.com/kitao/pyxel/releases)下载并运行最新的Windows安装器（`pyxel-[version]-windows-setup.exe`）。

**Mac**

安装[Homebrew](https://brew.sh/)之后，执行以下命令：

```sh
brew tap kitao/pyxel
brew install pyxel
```

**Linux**

安装SDL2（Ubuntu下包名为：`libsdl2-dev`）和[Homebrew](https://docs.brew.sh/Homebrew-on-Linux)之后，执行以下命令：

```sh
brew tap kitao/pyxel
brew install pyxel
```

如果以上步骤无效，可以依前述尝试自行编译。

### 尝试Pyxel例程

以Python包版本为例，安装Pyxel后，用以下命令将Pyxe例程复制到当前文件夹：

```sh
pyxel copy_examples
```

例程包含：

- [01_hello_pyxel.py](pyxel/examples/01_hello_pyxel.py) - 最简单的应用
- [02_jump_game.py](pyxel/examples/02_jump_game.py) - 用Pyxel制作的跳跃游戏
- [03_draw_api.py](pyxel/examples/03_draw_api.py) - 绘画API的使用示例
- [04_sound_api.py](pyxel/examples/04_sound_api.py) - 声音API的使用示例
- [05_color_palette.py](pyxel/examples/05_color_palette.py) - 调色板列表
- [06_click_game.py](pyxel/examples/06_click_game.py) - 鼠标点击游戏
- [07_snake.py](pyxel/examples/07_snake.py) - 带BGM的贪吃蛇游戏
- [08_triangle_api.py](pyxel/examples/08_triangle_api.py) - 三角形绘画API的使用示例
- [09_shooter.py](pyxel/examples/09_shooter.py) - 屏幕过渡射击游戏
- [10_platformer.py](pyxel/examples/10_platformer.py) - 屏幕横向滑动的游戏示例

运行例程，可以使用以下命令：

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
```

## 使用教程

### 创建Pyxel应用

在python文件中导入Pyxel包后，首先使用`init`函数指定窗口大小，然后使用`run`函数启动Pyxel应用。

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

同样可以使用`show`和`flip`函数来设计简单的图形和动画。

`show`函数进行屏幕显示直到`Esc`键被按下。

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

`flip`刷新一次屏幕图像。

```python
import pyxel

pyxel.init(120, 80)

while True:
    pyxel.cls(3)
    pyxel.rectb(pyxel.frame_count % 160 - 40, 20, 40, 40, 7)
    pyxel.flip()
```

### 运行Pyxel应用

创建的Python脚本可以使用以下命令执行：

```sh
pyxel run PYTHON_SCRIPT_FILE
```

对于python包版本，可以像普通Python脚本一样执行：

```sh
cd pyxel_examples
python3 PYTHON_SCRIPT_FILE
```

（在Windows中，使用`python`命令来替代`python3`）

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

在Pyxel应用中使用的图像和音效，可以使用Pyxel编辑器进行制作。

Pyxel编辑器使用以下命令启动：

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

若指定Pyxel源文件（.pyxres）存在，则加载文件，若不存在，则以指定文件名新建文件。

若未指定源文件，则命名为`my_resource.pyxres`。

Pyxel编辑器启动后，可以拖放其他源文件进行切换。如果源文件被拖拽并在按下``Ctrl(Cmd)``键时释放，则只有当前正在编译的类型（图像、瓦片地图、音效、音乐）会被加载。这个操作允许将多种类型的源文件合并入一个源文件中。

创建的源文件可以使用`load`函数加载。

Pyxel编辑器有以下编辑模式。

**图像编辑器：**

此模式用来编辑图像库。

<img src="pyxel/editor/screenshots/image_editor.gif">

通过将图像文件拖放进图像编辑器，图像可以加载进当前的图像库中。

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

Pyxel图像和瓦片地图也可以通过以下方法创建：

- 使用`Image.set`或`Tilemap.set`函数，从字符串列表创建图片
- 使用`Image.load`函数从加载图像文件至pyxel调色板中

Pyxel声音也可以通过以下方法创建：

- 使用`Sound.set`或`Music.set`函数，从字符串列表中创建声音

这些函数的具体用法请查阅API参考手册。

### 如何发布应用

Pyxel支持跨平台的应用文件格式（Pyxel应用文件）。

使用以下命令创建Pyxel应用文件（.pyxapp）：

```sh
pyxel package APP_ROOT_DIR STARTUP_SCRIPT_FILE
```

如果应用需要包含源文件或扩展模块，将他们放在应用文件夹。

创建好的应用文件使用以下命令执行：

```sh
pyxel play PYXEL_APP_FILE
```

## API参考手册

### 系统

- `width`, `height`<br>
画面的宽和高

- `frame_count`<br>
目前为止，经过的总帧数

- `init(width, height, [title], [fps], [quit_key], [capture_sec])`<br>
使用屏幕尺寸（`width`，`height`）初始化Pyxel应用。以下属性为可选配置项：窗口标题`title`，帧率`fps`，应用退出按键`quit_key`，以及屏幕捕获的最长记录时间`capture_sec`。<br>
示例：`pyxel.init(160, 120, title="Pyxel with Options", fps=60, quit_key=pyxel.KEY_NONE, capture_sec=0)`

- `run(update, draw)`<br>
启动Pyxel应用，并调用`update`函数刷新画面帧，并使用`draw`函数渲染画面。

- `show()`<br>
显示屏幕直到`Esc`键被按下。（通常应用中建议不要使用）

- `flip()`<br>
刷新一次屏幕。（通常应用中建议不要使用）

- `quit()`<br>
在当前帧的最后，退出Pyxel应用。

### 源文件

- `load(filename, [image], [tilemap], [sound], [music])`<br>
加载源文件(.pyxres)。如果某文件类型(``image/tilemap/sound/music``)被指定为``False``，则源文件中对应类型不会加载。

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
展示调色板可以显示的颜色列表。颜色以24位数值格式进行展示。使用`colors.from_list`和`colors.to_list`直接指定货检索Python列表。<br>
示例：`org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img)`<br>
直接操作图像库`img` (0-2)。（参考前文Image类）<br>
示例：`pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
操作瓦片地图`tm`(0-7)（参考前文Tilemap类）

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
将坐标瓦片地图`tm` (0-7)中坐标(`u`, `v`)开始的大小为(`w`, `h`)的图像信息绘至坐标(`x`, `y`)处。若参数`colkey`被指定，则其颜色视为透明。每个瓦片大小为8x8像素并以`(x in tile, y in tile)`元组存储在瓦片地图中。

- `text(x, y, s, col)`<br>
用`col`颜色在(`x`, `y`)绘制字符串`s`。

### 声音

- `sound(snd)`<br>
操作音频`snd`(0-63)。（参考Sound类）<br>
示例：`pyxel.sound(0).speed = 60`

- `music(msc)`<br>
操作音乐`msc`(0-7)（参考Music类）

- `play_pos(ch)`<br>
获取通道`ch` (0-3)中音频重播位置`(sound no, note no)`。若重播被停止则返回`None`。

- `play(ch, snd, loop=False)`<br>
播放通道`ch` (0-3)中的声音snd` (0-63)。如果声音`snd`是一个列表，则按顺序播放。如果`loop`被指定为`True`则循环播放。

- `playm(msc, loop=False)`<br>
播放音乐`msc` (0-7)。如果`loop`被指定为`True`则循环播放。

- `stop([ch])`<br>
停止指定通道`ch` (0-3)的重播。`stop()`可以停止所有通道的播放。

### Image类

- `width`, `height`<br>
图像的宽和高。

- `data`<br>
图像中的数据（256x256的二维列表）。

- `get(x, y)`<br>
获取图像中(`x`, `y`)位置的值。

- `set(x, y, data)`<br>
使用字符串列表设置坐标(`x`, `y`)处的图像。<br>
示例：`pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
在(`x`, `y`)处加载图像文件(png/gif/jpeg)。

### Tilemap类

- `width`, `height`<br>
瓦片地图(tilemap)的宽和高。

- `refimg`<br>
被瓦片地图tilemap引用的图像库(0-2)。

- `set(x, y, data)`<br>
使用字符串列表在坐标(`x`, `y`)处设置瓦片地图。<br>
示例：`pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `pget(x, y)`<br>
得到(`x`, `y`)处的瓦片。瓦片数据为元组`(x in tile, y in tile)`。

- `pset(x, y, tile)`<br>
在(`x`, `y`)处画出瓦片`tile`。瓦片数据为元组`(x in tile, y in tile)`。

### Sound类

- `notes`<br>
音符列表(0-127)，数字越高，音调越高。数字达到33时，音调就达到'A2'(440Hz)。其余为-1.

- `tones`<br>
音色列表(0:三角波 / 1:方波 / 2:脉冲 / 3:噪声)

- `volumes`<br>
音量列表(0-7)

- `effects`<br>
音效列表(0:无 / 1:滑动 / 2:颤音 / 3:淡出)

- `speed`<br>
播放速度。1为最快，数字越大，速度越慢。数字120时，每个音符长度为1秒。

- `set(notes, tones, volumes, effects, speed)`<br>
使用字符串设置音符、音色、音量及音效。如果音色、音量及音效的字符串比音符字符串短，则从开头重复。

- `set_notes(notes)`<br>
使用由'CDEFGAB'+'#-'+'0123'或'R'组成的字符串设置音符。大小写不敏感，且空格会被忽略。<br>
示例：`pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
使用由'TSPN'组成的字符串设置音色。大小写不敏感，且空格会被忽略。<br>
示例：`pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volumes(volumes)`<br>
使用由'01234567'组成的字符串设置音量。大小写不敏感，且空格会被忽略。<br>
示例：`pyxel.sound(0).set_volume("7777 7531")`

- `set_effects(effects)`<br>
使用由'NSVF'组成的字符串设置音效。大小写不敏感，且空格会被忽略。<br>
示例：`pyxel.sound(0).set_effect("NFNF NVVS")`

### Music类

- `sequences`<br>
按通道数顺序展示声音(0-63)的二维列表。

- `set(seq0, seq1, seq2, seq3)`<br>
设置所有通道的声音(0-63)列表。如果指定了空列表，则对应通道不会用来播放。<br>
示例：`pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### 高级APIs

Pyxel还有一些“高级API”，出于“可能令用户感到迷惑”、“需要专业知识”等一些原因，在本文尚未提及。

如果你对自己的技术很熟悉，可以参阅[this](pyxel/__init__.pyi)，尝试挑战自己并创造一些神奇的作品！

## 如何参与

### Submitting Issue

使用[Issue Tracker](https://github.com/kitao/pyxel/issues)来提交bug报告或功能需求。在创建新issue之前，请确定没有类似的打开的issue。

### Manual Testing

欢迎任何人在[Issue Tracker](https://github.com/kitao/pyxel/issues)中手动测试代码、上报bug、提交优化建议等！

### Submitting Pull Request

可以通过pull requests(PRs)形式来提交补丁或修复。请确认你的pull request对应的issue地址在issue tracker中依然是open状态。

一旦提交pull request，则默认同意在[MIT License](LICENSE)的许可下发布。

## 其他信息

- [Discord服务器 (English)](https://discord.gg/FC7kUZJ)
- [Discord服务器 (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## 许可证

Pyxel遵循[MIT License](http://en.wikipedia.org/wiki/MIT_License)。您可以在专利软件中重复使用，前提是该软件的所有副本或重要部分均包含 MIT 许可条款的副本和版权声明。
