# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**Pyxel**是一个 python 的经典像素风游戏制作引擎。

由于像素风游戏的机制非常简单 (如：最多只能显示 16 种颜色、播放 4 种声音等)，现在你也可以轻松地享受这种游戏的制作过程。

<img src="images/pyxel_message.png" width="480">

Pyxel 开发的动力来自于用户的反馈。请在 GitHub 上给 Pyxel 一颗星吧！

<p>
<a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">
<img src="images/01_hello_pyxel.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">
<img src="images/02_jump_game.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">
<img src="images/03_draw_api.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">
<img src="images/04_sound_api.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_music_editor.gif" width="320">
</a>
</p>

Pyxel 的规范和 API 受到[PICO-8](https://www.lexaloffle.com/pico-8.php)和[TIC-80](https://tic80.com/)的启发。

Pyxel 是开源的，大家可以免费使用。现在就让我们一起用 Pyxel 制作自己的游戏吧！

## 说明

- 可在 Windows、Mac、Linux 和 Web 上运行
- 可以使用 python 进行编程
- 16 色调色板
- 3 个 256x256 的图像库
- 8 个 256x256 的瓦片地图
- 4 个音轨，每个各可含有 64 个音符
- 可任意组合 8 个音乐
- 支持键盘、鼠标及游戏手柄输入
- 图像和音频编辑器

### 调色板

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## 如何安装

### Windows

在安装[Python3](https://www.python.org/) (3.7 或更高版本) 之后，执行以下命令：

```sh
pip install -U pyxel
```

如果你使用官方安装程序安装 Python，请勾选`Add Python 3.x to PATH`复选框以启用`pyxel`命令。

### Mac

安装 [Homebrew](https://brew.sh/) 后，运行以下命令：

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

要在安装 Pyxel 后更新版本，请运行 `pipx upgrade pyxel`。

### Linux

安装 SDL2 (Ubuntu 下包名为：`libsdl2-dev`)，[Python3](https://www.python.org/) (3.7 或更高版本)，以及`python3-pip`这三个包之后，执行以下命令：

```sh
sudo pip3 install -U pyxel
```

如果上述方法不奏效，请根据[Makefile](../Makefile)中的说明尝试自我构建。

### Web

网络版 Pyxel 不需要安装 Python 或 Pyxel，可以在 PC 以及支持网络浏览器的智能手机和平板电脑上运行。

具体说明请参考[本页面](https://github.com/kitao/pyxel/wiki/How-To-Use-Pyxel-Web)。

### 尝试 Pyxel 例程

以 Python 包版本为例，安装 Pyxel 后，用以下命令将 Pyxe 例程复制到当前文件夹：

```sh
pyxel copy_examples
```

例程包含：

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>最简单的应用</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Code</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>用 Pyxel 制作的跳跃游戏</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Code</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>绘画 API 的使用示例</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Code</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>声音 API 的使用示例</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Code</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>调色板列表</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Code</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>鼠标点击游戏</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Code</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>带 BGM 的贪吃蛇游戏</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Code</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>三角形绘画 API 的使用示例</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Code</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>屏幕过渡射击游戏</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Code</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>屏幕横向滑动的游戏示例</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Code</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>用 Image 类进行屏外渲染</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Code</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>佩林噪音动画</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Code</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>绘制一个位图字体</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Code</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>利用音频扩展功能的合成器</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Code</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>加载和绘制磁贴地图文件 (.tmx)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Code</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>具有 flip 功能的动画 (仅在非网络平台上)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Code</a></td>
</tr>
<tr>
<td>30SecondsOfDaylight.pyxapp</td>
<td>第 1 届 Pyxel Jam 比赛获胜者是<a href="https://twitter.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30SecondsOfDaylight.html">Demo</a></td>
<td><a href="https://github.com/kitao/30SecondsOfDaylight">Code</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>商场球类物理游戏<a href="https://twitter.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Demo</a></td>
<td><a href="https://github.com/helpcomputer/megaball">Code</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>背景音乐生成器由<a href="https://twitter.com/frenchbread1222">frenchbread</a>制作</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">Demo</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">Code</a></td>
</tr>
</table>

运行例程，可以使用以下命令：

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30SecondsOfDaylight.pyxapp
```

## 使用教程

### 创建 Pyxel 应用

在 python 文件中导入 Pyxel 包后，首先使用`init`函数指定窗口大小，然后使用`run`函数启动 Pyxel 应用。

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

实际应用中，建议将 pyxel 代码封装成如下类：

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

当创建没有动画的简单图形时，可以使用`show`函数来使代码更加简洁。

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### 运行 Pyxel 应用

创建的 Python 脚本可以使用以下命令执行：

```sh
pyxel run PYTHON_SCRIPT_FILE
```

它也可以像普通的 Python 脚本一样被执行：

```sh
python3 PYTHON_SCRIPT_FILE
```

### 快捷键

以下快捷键可以在 Pyxel 运行时使用：

- `Esc`<br>
  退出应用
- `Alt(Option)+1`<br>
  截屏并保存在桌面
- `Alt(Option)+2`<br>
  重置屏幕录制的开始时间
- `Alt(Option)+3`<br>
  保存屏幕录制动图到桌面 (最多 10 秒)
- `Alt(Option)+9`<br>
  切换屏幕模式 (Crisp/Smooth/Retro)
- `Alt(Option)+0`<br>
  切换性能监控 (fps，更新时间，画面绘制时间)
- `Alt(Option)+Enter`<br>
  切换全屏
- `Shift+Alt(Option)+1/2/3`<br>
  将相应的图像库保存到桌面
- `Shift+Alt(Option)+0`<br>
  将当前调色板保存到桌面

### 如何创建源文件

在 Pyxel 应用中使用的图像和音效，可以使用 Pyxel 编辑器进行制作。

Pyxel 编辑器使用以下命令启动：

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

若指定 Pyxel 源文件 (.pyxres) 存在，则加载文件，若不存在，则以指定文件名新建文件。若未指定源文件，则命名为`my_resource.pyxres`。

Pyxel 编辑器启动后，可以拖放其他源文件进行切换。

创建的源文件可以使用`load`函数加载。

Pyxel 编辑器有以下编辑模式。

**图像编辑器**

此模式用来编辑图像库。

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

将图像文件 (PNG/GIF/JPEG) 拖放到图像编辑器上，即可将图像加载到当前选定的图像库中。

**瓦片地图编辑器**

此模式用来编辑瓦片地图，其中图像库的图像以瓦片的样式排列。

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

将 TMX 文件 (Tiled Map File) 拖放到平铺贴图编辑器上，即可按照与当前所选平铺贴图编号相对应的绘图顺序加载其图层。

**音频编辑器**

此模式用来编辑音频。

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**音乐编辑器**

此模式用来编辑将录音有序编排形成的音乐。

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### 其他创建源文件的方法

Pyxel 图像和瓦片地图也可以通过以下方法创建：

- 使用`Image.set`或`Tilemap.set`函数，从字符串列表创建图片
- 使用 `Image.load` 函数在 Pyxel 调色板中加载图像文件 (PNG/GIF/JPEG)

Pyxel 声音也可以通过以下方法创建：

- 使用`Sound.set`或`Music.set`函数，从字符串列表中创建声音

这些函数的具体用法请查阅 API 参考手册。

### 如何发布应用

Pyxel 支持跨平台的应用文件格式 (Pyxel 应用文件)。

使用以下命令创建 Pyxel 应用文件 (.pyxapp)：

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

如果应用程序应包括资源或其他模块，请将它们放在应用程序目录中。

创建好的应用文件使用以下命令执行：

```sh
pyxel play PYXEL_APP_FILE
```

Pyxel 应用程序文件也可以通过`pyxel app2exe`或`pyxel app2html`命令转换为可执行文件或 HTML 文件。

## API 参考手册

### 系统

- `width`，`height`<br>
  画面的宽和高

- `frame_count`<br>
  目前为止，经过的总帧数

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  使用屏幕尺寸 (`width`，`height`) 初始化 Pyxel 应用。以下属性为可选配置项：窗口标题`title`，帧率`fps`，应用退出按键`quit_key`，用 "display_scale "来决定显示的比例，用 "capture_scale "来决定屏幕捕捉的比例，以及屏幕捕获的最长记录时间`capture_sec`。<br>
  示例：`pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  启动 Pyxel 应用，并调用`update`函数刷新画面帧，并使用`draw`函数渲染画面。

- `show()`<br>
  显示屏幕直到`Esc`键被按下。

- `flip()`<br>
  将屏幕重新调整一帧。当按下`Esc`键时，应用程序退出。该功能在网络版中不起作用。

- `quit()`<br>
  退出 Pyxel 应用。

### 源文件

- `load(filename, [excl_images], [excl_tilemaps], [excl_sounds], [excl_musics])`<br>
  加载源文件 (.pyxres)。如果选项为`True`，则不会加载资源。如果在资源文件的同一位置存在同名的调色板文件 (.pyxpal)，调色板的显示颜色也将改变。调色板文件是显示颜色的十六进制条目 (如 `1100FF`)，以换行分隔。调色板文件也可用于更改 Pyxel 编辑器中显示的颜色。

### 输入

- `mouse_x`，`mouse_y`<br>
  当前鼠标指针的位置。

- `mouse_wheel`<br>
  当前鼠标滚轮的值。

- `btn(key)`<br>
  如果`key`被按下则返回`True`，否则返回`False`。([按键定义列表](../python/pyxel/__init__.pyi))。

- `btnp(key, [hold], [repeat])`<br>
  如果`key`被按下则返回`True`。若设置了`hold`和`repeat`参数，则当`key`被按下持续`hold`帧时，在`repeat`帧间隙返回`True`。

- `btnr(key)`<br>
  如果`key`被松开，则在此帧返回`True`，否则返回`False`。

- `mouse(visible)`<br>
  如果`visible`为`True`则显示鼠标指针，为`False`则不显示。即使鼠标指针不显示，其位置同样会被更新。

### 显示

- `colors`<br>
  展示调色板可以显示的颜色列表。颜色以 24 位数值格式进行展示。使用`colors.from_list`和`colors.to_list`直接指定货检索 Python 列表。<br>
  示例：`old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  图像库列表 (0-2)。 (参考前文 Image 类)<br>
  示例：`pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  瓦片贴图列表 (0-7)。 (参考前文 Tilemap 类)

- `clip(x, y, w, h)`<br>
  设置画面绘制区域为从 (`x`, `y`) 开始的宽度`w`、高度为`h`的区域。`clip()`可以将绘制区域重置为全屏。

- `camera(x, y)`<br>
  更改视角等起始位置，使位置 (`x`, `y`) 成为屏幕左上角的起始位置，这将有助于切换视角。若想恢复起始位置，使用`camera()`或 `camera(0, 0)` 即可完成重置。

- `pal(col1, col2)`<br>
  绘制时用`col1`颜色代替`col2`颜色。`pal()`可以重置为初始色调。

- `dither(alpha)`<br>
  在绘制时应用抖动 (伪透明)。在 0.0-1.0 的范围内设置 `alpha`，其中 0.0 表示透明，1.0 表示不透明。

- `cls(col)`<br>
  用`col`颜色清空画面。

- `pget(x, y)`<br>
  获取 (`x`, `y`) 处的像素颜色。

- `pset(x, y, col)`<br>
  用`col`颜色在 (`x`, `y`) 处绘制一个像素点。

- `line(x1, y1, x2, y2, col)`<br>
  用`col`颜色画一条从 (`x1`, `y1`) 到 (`x2`, `y2`) 的直线。

- `rect(x, y, w, h, col)`<br>
  用`col`颜色绘制一个从 (`x`, `y`) 开始的宽为`w`、高为`h`的矩形。

- `rectb(x, y, w, h, col)`<br>
  用`col`颜色绘制从 (`x`, `y`) 开始的宽为`w`、高为`h`的矩形边框。

- `circ(x, y, r, col)`<br>
  用`col`颜色绘制圆心为 (`x`, `y`)，半径为`r`的圆形。

- `circb(x, y, r, col)`<br>
  用`col`颜色绘制圆心为 (`x`, `y`)，半径为`r`的圆形边框。

- `elli(x, y, w, h, col)`<br>
  从 (`x`, `y`) 画一个宽度`w`，高度`h`，颜色`col`的椭圆。

- `ellib(x, y, w, h, col)`<br>
  从 (`x`, `y`) 画出一个宽`w`，高`h`，颜色`col`的椭圆轮廓。

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  用`col`颜色绘制顶点分别为 (`x1`, `y1`)，(`x2`, `y2`)，(`x3`, `y3`) 的三角形。

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  用`col`颜色绘制顶点分别为 (`x1`, `y1`)，(`x2`, `y2`)，(`x3`, `y3`) 的三角形边框。

- `fill(x, y, col)`<br>
  从 (`x`, `y`) 画一个宽度`w`，高度`h`，颜色`col`的椭圆。

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
  将尺寸为 (`w`, `h`) 的区域从图像库的 (`u`, `v`) 复制到 (`x`, `y`)。若`w`或`h`为负值，则在水平或垂直方向上翻转。若指定了`colkey`的值，则视作透明颜色。

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
  从瓦片图`tm` (0-7) 的 (`u`，`v`) 复制大小为 (`w`，`h`) 的区域到 (`x`，`y`)。如果为`w`和/或`h`设置了负值，它将在水平和/或垂直方向上反转。如果指定了 `colkey`，将被视为透明色。瓦片的大小是 8x8 像素，以`(tile_x, tile_y)`的元组形式存储在瓦片图中。

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  用`col`颜色在 (`x`, `y`) 绘制字符串`s`。

### 声音

- `sounds`<br>
  声音列表 (0-63)。 (参考 Sound 类)<br>
  示例：`pyxel.sounds[0].speed = 60`

- `musics`<br>
  音乐列表 (0-7)。 (参考 Music 类)

- `play(ch, snd, [tick], [loop], [resume])`<br>
  播放通道`ch`(0-3) 中的声音`snd`(0-63)。如果声音`snd`是一个列表，则按顺序播放。播放开始位置可以通过 `tick`(1 tick = 1/120 秒) 指定。如果`loop`被指定为`True`则循环播放。播放结束后要恢复之前的声音，请将 `resume` 设置为 `True`。

- `playm(msc, [tick], [loop])`<br>
  播放音乐`msc`(0-7)。播放开始位置可以通过 `tick`(1 tick = 1/120 秒) 指定。如果`loop`被指定为`True`则循环播放。

- `stop([ch])`<br>
  停止指定通道`ch`(0-3) 的重播。`stop()`可以停止所有通道的播放。

- `play_pos(ch)`<br>
  获取通道`ch`(0-3) 中音频重播位置`(sound no, note no)`。若重播被停止则返回`None`。

### 数学

- `ceil(x)`<br>
  返回大于或等于`x`的最小的整数。

- `floor(x)`<br>
  返回小于或等于`x`的最大整数。

- `sgn(x)`<br>
  当`x`是正数时返回 1，当它是零时返回 0，当它是负数时返回 1。

- `sqrt(x)`<br>
  返回`x`的平方根。

- `sin(deg)`<br>
  返回`deg`度的正弦。

- `cos(deg)`<br>
  返回`deg`度的余弦。

- `atan2(y, x)`<br>
  返回`y`/`x`的正切，单位是度。

- `rseed(seed)`<br>
  设置随机数发生器的种子。

- `rndi(a, b)`<br>
  返回一个大于或等于`a`且小于或等于`b`的随机整数。

- `rndf(a, b)`<br>
  返回一个大于或等于`a`且小于或等于`b`的随机小数。

- `nseed(seed)`<br>
  设置佩林噪声的种子。

- `noise(x, [y], [z])`<br>
  返回指定坐标的佩林噪声值。

### Image 类

- `width`，`height`<br>
  图像的宽和高

- `set(x, y, data)`<br>
  使用字符串列表设置坐标 (`x`, `y`) 处的图像。<br>
  示例：`pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  在 (`x`, `y`) 处加载图像文件 (PNG/GIF/JPEG)。

- `pget(x, y)`<br>
  获取 (`x`, `y`) 处的像素颜色。

- `pset(x, y, col)`<br>
  用`col`颜色在 (`x`, `y`) 处绘制一个像素点。

### Tilemap 类

- `width`，`height`<br>
  瓦片地图的宽和高

- `imgsrc`<br>
  被瓦片地图 tilemap 引用的图像库 (0-2)

- `set(x, y, data)`<br>
  使用字符串列表在坐标 (`x`, `y`) 处设置瓦片地图。<br>
  示例：`pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  从位于 (`x`, `y`) 处的 TMX 文件 (Tiled Map File) 中以绘图顺序 `layer`(0-) 加载图层。

- `pget(x, y)`<br>
  得到 (`x`, `y`) 处的瓦片。瓦片数据为元组`(tile_x, tile_y)`。

- `pset(x, y, tile)`<br>
  在 (`x`, `y`) 处画出瓦片`tile`。瓦片数据为元组`(tile_x, tile_y)`。

### Sound 类

- `notes`<br>
  音符列表 (0-127)，数字越高，音调越高。数字达到 33 时，音调就达到'A2'(440Hz)。其余为-1。

- `tones`<br>
  音色列表 (0:三角波 / 1:方波 / 2:脉冲 / 3:噪声)

- `volumes`<br>
  音量列表 (0-7)

- `effects`<br>
  音效列表 (0:无 / 1:滑动 / 2:颤音 / 3:淡出)

- `speed`<br>
  播放速度。1 为最快，数字越大，速度越慢。数字 120 时，每个音符长度为 1 秒。

- `set(notes, tones, volumes, effects, speed)`<br>
  使用字符串设置音符、音色、音量及音效。如果音色、音量及音效的字符串比音符字符串短，则从开头重复。

- `set_notes(notes)`<br>
  使用由'CDEFGAB'+'#-'+'01234'或'R'组成的字符串设置音符。大小写不敏感，且空格会被忽略。<br>
  示例：`pyxel.sounds[0].set_notes("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
  使用由'TSPN'组成的字符串设置音色。大小写不敏感，且空格会被忽略。<br>
  示例：`pyxel.sounds[0].set_tones("TTSS PPPN")`

- `set_volumes(volumes)`<br>
  使用由'01234567'组成的字符串设置音量。大小写不敏感，且空格会被忽略。<br>
  示例：`pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  使用由'NSVF'组成的字符串设置音效。大小写不敏感，且空格会被忽略。<br>
  示例：`pyxel.sounds[0].set_effects("NFNF NVVS")`

### Music 类

- `seqs`<br>
  二维的声音列表 (0-63)，带有通道的数量

- `set(seq0, seq1, seq2, ...)`<br>
  设置通道的声音列表 (0-63)。如果指定了空列表，则对应通道不会用来播放。<br>
  示例：`pyxel.musics[0].set([0, 1], [], [3])`

### 高级 APIs

Pyxel 还有一些“高级 API”，出于“可能令用户感到迷惑”、“需要专业知识”等一些原因，在本文尚未提及。

如果你对自己的技术很熟悉，可以参阅[this](../python/pyxel/__init__.pyi)，尝试挑战自己并创造一些神奇的作品！

## 如何参与

### 向我们报告问题

使用[Issue Tracker](https://github.com/kitao/pyxel/issues)来提交 bug 报告或功能需求。在创建新 issue 之前，请确定没有类似的打开的 issue。

### 参与测试

欢迎任何人在[Issue Tracker](https://github.com/kitao/pyxel/issues)中手动测试代码、上报 bug、提交优化建议等！

### 贡献代码

可以通过 pull requests (PRs) 形式来提交补丁或修复。请确认你的 pull request 对应的 issue 地址在 issue tracker 中依然是 open 状态。

一旦提交 pull request，则默认同意在[MIT License](../LICENSE)的许可下发布。

## 其他信息

- [Q&A](https://github.com/kitao/pyxel/wiki/Pyxel-Q&A)
- [User Examples](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Developer's Twitter account](https://twitter.com/kitao)

## 许可证

Pyxel 遵循[MIT License](../LICENSE)。您可以在专利软件中重复使用，前提是该软件的所有副本或重要部分均包含 MIT 许可条款的副本和版权声明。

## 招募赞助商

Pyxel 正在 GitHub 赞助商上寻找赞助商。 考虑赞助 Pyxel 以进行持续维护和功能添加。 赞助商可以咨询 Pyxel 作为一个好处。 详情请参阅[此处](https://github.com/sponsors/kitao)。
