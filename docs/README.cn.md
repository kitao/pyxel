# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** (/ˈpɪksəl/) 是一个针对 Python 的复古游戏引擎。

其规格受到复古游戏机的启发，例如仅支持 16 种颜色和 4 个音轨，同时可以轻松享受制作像素艺术风格游戏的乐趣。

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

Pyxel 的开发得益于用户的反馈。请在 GitHub 上给 Pyxel 评分！

<p>
<a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">
<img src="images/10_platformer.gif" width="290">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/30sec_of_daylight.html">
<img src="images/30sec_of_daylight.gif" width="350">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">
<img src="images/02_jump_game.gif" width="330">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">
<img src="images/megaball.gif" width="310">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_music_editor.gif" width="320">
</a>
</p>

Pyxel 的规格和 API 参考了 [PICO-8](https://www.lexaloffle.com/pico-8.php) 和 [TIC-80](https://tic80.com/)。

Pyxel 在 [MIT 许可证](../LICENSE) 下开源并免费使用。让我们开始使用 Pyxel 制作复古游戏吧！

## 规格

- 支持 Windows、Mac、Linux 和 Web
- 使用 Python 编程
- 可自定义屏幕尺寸
- 16 色调色板
- 3 个 256x256 尺寸图像库
- 8 个 256x256 尺寸瓦片地图
- 4 个通道，支持 64 种可定义声音
- 8 个音乐轨道可以组合任意声音
- 支持键盘、鼠标和游戏手柄输入
- 图像和声音编辑工具
- 用户可扩展的颜色、通道和库

### 色彩调色板

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## 如何安装

### Windows

在安装 [Python3](https://www.python.org/)（版本 3.8 或更高）后，运行以下命令：

```sh
pip install -U pyxel
```

在使用官方安装程序安装 Python 时，请确保勾选 `Add Python 3.x to PATH` 选项，以启用 `pyxel` 命令。

### Mac

在安装 [Homebrew](https://brew.sh/) 后，运行以下命令：

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

安装 Pyxel 后，要升级 Pyxel，请运行 `pipx upgrade pyxel`。

### Linux

在安装 SDL2 包（对于 Ubuntu 为 `libsdl2-dev`）、[Python3](https://www.python.org/)（版本 3.8 或更高）和 `python3-pip` 后，运行以下命令：

```sh
sudo pip3 install -U pyxel
```

如果之前的命令失败，请按照 [Makefile](../Makefile) 中的说明考虑从源代码构建 Pyxel。

### Web

Pyxel 的 Web 版本不需要安装 Python 或 Pyxel，可以在支持的 Web 浏览器上运行于 PC、智能手机和平板电脑上。

有关详细说明，请参阅 [此页面](pyxel-web-en.md)。

### 运行示例

在安装 Pyxel 后，您可以使用以下命令将示例复制到当前目录：

```sh
pyxel copy_examples
```

以下示例将被复制到您的当前目录：

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>最简单的应用</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">代码</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>使用 Pyxel 资源文件的跳跃游戏</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">代码</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>绘图 API 的演示</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">代码</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>音频 API 的演示</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">代码</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>颜色调色板列表</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">代码</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>鼠标点击游戏</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">代码</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>带有背景音乐的贪吃蛇游戏</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">代码</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>三角形绘图API的演示</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">代码</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>射击游戏与屏幕切换及MML</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">代码</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>带有地图的横向卷轴平台游戏</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">代码</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>使用 Image 类进行离屏渲染</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">代码</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>柏林噪声动画</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">代码</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>绘制位图字体</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">代码</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>使用音频扩展功能的合成器</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">代码</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>加载和绘制 Tiled Map File (.tmx)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">代码</a></td>
</tr>
<tr>
<td>16_transform.py</td>
<td>图像旋转和缩放</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/16_transform.html">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/16_transform.py">代码</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>使用翻转函数的动画（仅限非网页平台）</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">演示</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">代码</a></td>
</tr>
<tr>
<td>30sec_of_daylight.pyxapp</td>
<td>第 1 届 Pyxel Jam 获胜游戏由 <a href="https://x.com/helpcomputer0">Adam</a> 制作</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30sec_of_daylight.html">演示</a></td>
<td><a href="https://github.com/kitao/30SecondsOfDaylight">代码</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>由 <a href="https://x.com/helpcomputer0">Adam</a> 制作的街机球物理游戏</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">演示</a></td>
<td><a href="https://github.com/kitao/megaball">代码</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>由 <a href="https://x.com/frenchbread1222">frenchbread</a> 制作的背景音乐生成器</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">演示</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">代码</a></td>
</tr>
</table>

这些示例可以通过以下命令执行：

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30sec_of_daylight.pyxapp
```

## 使用方法

### 创建应用程序

在您的 Python 脚本中，导入 Pyxel 模块，通过 `init` 函数指定窗口大小，然后使用 `run` 函数启动 Pyxel 应用程序。

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

`run` 函数的参数是处理帧更新的 `update` 函数和处理屏幕绘制的 `draw` 函数。

在实际应用中，建议将 Pyxel 代码封装在类中，如下所示：

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

要创建没有动画的简单图形，您可以使用 `show` 函数来简化代码。

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### 运行应用程序

创建的脚本可以使用 `python` 命令执行：

```sh
python PYTHON_SCRIPT_FILE
```

它也可以使用 `pyxel run` 命令运行：

```sh
pyxel run PYTHON_SCRIPT_FILE
```

此外，`pyxel watch` 命令监视指定目录中的更改，并在检测到更改时自动重新运行程序：

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

可以通过按 `Ctrl(Command)+C` 停止目录监视。

### 特殊键操作

在运行 Pyxel 应用程序时，可以执行以下特殊键操作：

- `Esc`<br>
  退出应用程序
- `Alt(Option)+1`<br>
  将屏幕截图保存到桌面
- `Alt(Option)+2`<br>
  重置屏幕录像视频的录制开始时间
- `Alt(Option)+3`<br>
  将屏幕录像视频保存到桌面（最多 10 秒）
- `Alt(Option)+8` 或者在游戏手柄上按 `A+B+X+Y+DL`<br>
  在最大和整数倍缩放之间切换屏幕缩放
- `Alt(Option)+9` 或者在游戏手柄上按 `A+B+X+Y+DR`<br>
  在屏幕模式 (Crisp/Smooth/Retro) 之间切换
- `Alt(Option)+0` 或者在游戏手柄上按 `A+B+X+Y+DU`<br>
  切换性能监视器 (FPS/`update` 时间/`draw` 时间)
- `Alt(Option)+Enter` 或者在游戏手柄上按 `A+B+X+Y+DD`<br>
  切换全屏
- `Shift+Alt(Option)+1/2/3`<br>
  将图像库 0、1 或 2 保存到桌面
- `Shift+Alt(Option)+0`<br>
  将当前的调色板保存到桌面

### 如何创建资源

Pyxel Editor 可以创建用于 Pyxel 应用程序的图像和声音。

您可以使用以下命令启动 Pyxel Editor：

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

如果指定的 Pyxel 资源文件 (.pyxres) 存在，则会加载它。如果不存在，则会使用指定的名称创建一个新文件。如果省略资源文件，则会创建一个名为 `my_resource.pyxres` 的新文件。

启动 Pyxel Editor 后，您可以通过将另一个资源文件拖放到 Pyxel Editor 上来切换到该资源文件。

创建的资源文件可以使用 `load` 函数加载。

Pyxel Editor 有以下编辑模式。

**图像编辑器**

用于编辑每个**图像库**中图像的模式。

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

您可以将图像文件 (PNG/GIF/JPEG) 拖放到图像编辑器中，以将图像加载到当前选择的图像库中。

**瓦片地图编辑器**

用于编辑将图像库中的图像按瓦片模式排列的**瓦片地图**的模式。

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

将 TMX 文件（Tiled Map File）拖放到瓦片地图编辑器上，以将其第 0 层加载到当前选择的瓦片地图中。

**声音编辑器**

用于编辑用于旋律和音效的**声音**的模式。

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**音乐编辑器**

用于编辑将声音按播放顺序排列的**音乐**的模式。

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### 其他资源创建方法

Pyxel 图像和瓦片地图还可以通过以下方法创建：

- 使用 `Image.set` 函数或 `Tilemap.set` 函数从字符串列表创建图像
- 使用 `Image.load` 函数加载带有 Pyxel 调色板的图像文件 (PNG/GIF/JPEG)

Pyxel 声音也可以通过以下方法创建：

- 使用 `Sound.set` 函数或 `Music.set` 函数从字符串创建声音

有关这些函数的用法，请参阅 API 参考。

### 如何分发应用程序

Pyxel 支持一种专用的跨平台应用程序分发文件格式（Pyxel 应用程序文件）。

使用 `pyxel package` 命令创建 Pyxel 应用程序文件 (.pyxapp)：

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

如果您需要包括资源或其他模块，请将它们放在应用程序目录中。

通过在启动脚本中指定以下格式，可以在运行时显示元数据。除 `title` 和 `author` 外的字段都是可选的。

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

创建的应用程序文件可以使用 `pyxel play` 命令运行：

```sh
pyxel play PYXEL_APP_FILE
```

Pyxel 应用程序文件还可以使用 `pyxel app2exe` 或 `pyxel app2html` 命令转换为可执行文件或 HTML 文件。

## API 参考

### 系统

- `width`, `height`<br>
  屏幕的宽度和高度

- `frame_count`<br>
  已经过的帧数

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  使用屏幕尺寸 (`width`, `height`) 初始化 Pyxel 应用程序。以下选项可以指定：通过 `title` 设置窗口标题，`fps` 设置帧率，`quit_key` 设置退出应用程序的按键，`display_scale` 设置显示比例，`capture_scale` 设置屏幕捕捉比例，`capture_sec` 设置屏幕捕捉视频的最大录制时间。<br>
  例子：`pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  启动 Pyxel 应用程序，并调用 `update` 函数进行帧更新，调用 `draw` 函数进行绘制。

- `show()`<br>
  显示屏幕并等待按下 `Esc` 键。

- `flip()`<br>
  刷新屏幕一帧。当按下 `Esc` 键时，应用程序退出。此功能在 Web 版中不可用。

- `quit()`<br>
  退出 Pyxel 应用程序。

### 资源

- `load(filename, [exclude_images], [exclude_tilemaps], [exclude_sounds], [exclude_musics])`<br>
  加载资源文件 (.pyxres)。如果某个选项设置为 `True`，相应的资源将被排除加载。如果资源文件所在位置存在同名的调色板文件 (.pyxpal)，调色板显示颜色也将更新。调色板文件包含 16 进制的显示颜色条目（例如 `1100ff`），每行一个。调色板文件也可以用于更改 Pyxel Editor 中的显示颜色。

- `user_data_dir(vendor_name, app_name)`<br>
  返回基于 `vendor_name` 和 `app_name` 创建的用户数据目录。如果目录不存在，它将自动创建。用于存储高分、游戏进度等数据。<br>
  例子：`print(pyxel.user_data_dir("Takashi Kitao", "Pyxel Shooter"))`

### 输入

- `mouse_x`, `mouse_y`<br>
  当前鼠标光标的位置

- `mouse_wheel`<br>
  当前鼠标滚轮的值

- `btn(key)`<br>
  如果按下了 `key` 返回 `True`，否则返回 `False`。([按键定义列表](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  如果在该帧按下了 `key` 返回 `True`，否则返回 `False`。如果指定了 `hold` 和 `repeat`，当按下 `key` 的时间达到 `hold` 帧或更多时，每隔 `repeat` 帧返回一次 `True`。

- `btnr(key)`<br>
  如果在该帧释放了 `key` 返回 `True`，否则返回 `False`。

- `mouse(visible)`<br>
  如果 `visible` 为 `True`，显示鼠标光标。如果 `visible` 为 `False`，隐藏光标。即使光标被隐藏，它的位置仍然会更新。

### 图形

- `colors`<br>
  调色板显示颜色的列表。显示颜色由 24 位数值指定。使用 `colors.from_list` 和 `colors.to_list` 直接分配和获取 Python 列表。<br>
  例子：`old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  图像库的列表（Image 类的实例）(0-2)<br>
  例子：`pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  瓦片地图的列表（Tilemap 类的实例）(0-7)

- `clip(x, y, w, h)`<br>
  设置从 (`x`, `y`) 开始，宽度为 `w`，高度为 `h` 的绘制区域。调用 `clip()` 重置绘制区域为全屏。

- `camera(x, y)`<br>
  将屏幕左上角的坐标更改为 (`x`, `y`)。调用 `camera()` 将左上角坐标重置为 (`0`, `0`)。

- `pal(col1, col2)`<br>
  绘制时将颜色 `col1` 替换为 `col2`。调用 `pal()` 重置为初始调色板。

- `dither(alpha)`<br>
  绘制时应用抖动（伪透明）。设置 `alpha` 范围为 `0.0`-`1.0`，其中 `0.0` 表示透明，`1.0` 表示不透明。

- `cls(col)`<br>
  使用颜色 `col` 清除屏幕。

- `pget(x, y)`<br>
  获取 (`x`, `y`) 处像素的颜色。

- `pset(x, y, col)`<br>
  在 (`x`, `y`) 处绘制颜色为 `col` 的像素。

- `line(x1, y1, x2, y2, col)`<br>
  从 (`x1`, `y1`) 到 (`x2`, `y2`) 绘制颜色为 `col` 的线条。

- `rect(x, y, w, h, col)`<br>
  从 (`x`, `y`) 开始绘制宽度为 `w`，高度为 `h` 的颜色为 `col` 的矩形。

- `rectb(x, y, w, h, col)`<br>
  从 (`x`, `y`) 开始绘制宽度为 `w`，高度为 `h` 的颜色为 `col` 的矩形轮廓。

- `circ(x, y, r, col)`<br>
  在 (`x`, `y`) 处绘制半径为 `r`，颜色为 `col` 的圆。

- `circb(x, y, r, col)`<br>
  在 (`x`, `y`) 处绘制半径为 `r`，颜色为 `col` 的圆轮廓。

- `elli(x, y, w, h, col)`<br>
  从 (`x`, `y`) 开始绘制宽度为 `w`，高度为 `h` 的颜色为 `col` 的椭圆。

- `ellib(x, y, w, h, col)`<br>
  从 (`x`, `y`) 开始绘制宽度为 `w`，高度为 `h` 的颜色为 `col` 的椭圆轮廓。

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  绘制顶点为 (`x1`, `y1`)，(`x2`, `y2`)，(`x3`, `y3`)，颜色为 `col` 的三角形。

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  绘制顶点为 (`x1`, `y1`)，(`x2`, `y2`)，(`x3`, `y3`)，颜色为 `col` 的三角形轮廓。

- `fill(x, y, col)`<br>
  使用颜色 `col` 填充与 (`x`, `y`) 颜色相同的区域。

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  将图像库 `img`(0-2) 中从 (`u`, `v`) 开始的大小为 (`w`, `h`) 的区域复制到 (`x`, `y`)。如果 `w` 和/或 `h` 设置为负值，该区域将水平和/或垂直翻转。如果指定了 `colkey`，则该颜色将被视为透明色。如果指定了 `rotate`（以度为单位），`scale`（1.0 = 100%），或两者，应用相应的转换。

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  将瓦片地图 `tm`(0-7) 中从 (`u`, `v`) 开始的大小为 (`w`, `h`) 的区域复制到 (`x`, `y`)。如果 `w` 和/或 `h` 设置为负值，该区域将水平和/或垂直翻转。如果指定了 `colkey`，则该颜色将被视为透明色。如果指定了 `rotate`（以度为单位），`scale`（1.0 = 100%），或两者，应用相应的转换。瓦片的大小为 8x8 像素，并以 `(image_tx, image_ty)` 元组存储在瓦片地图中。

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  在 (`x`, `y`) 处以颜色 `col` 绘制字符串 `s`。

### 音频

- `sounds`<br>
  声音的列表（Sound 类的实例）(0-63)<br>
  例子：`pyxel.sounds[0].speed = 60`

- `musics`<br>
  音乐的列表（Music 类的实例）(0-7)

- `play(ch, snd, [sec], [loop], [resume])`<br>
  在通道 `ch`(0-3) 上播放声音 `snd`(0-63)。`snd` 可以是声音编号、声音编号的列表，或 MML 字符串。播放的起始位置可以通过 `sec`（秒）指定。如果 `loop` 设置为 `True`，则执行循环播放。要在播放结束后恢复到之前的声音，设置 `resume` 为 `True`。

- `playm(msc, [sec], [loop])`<br>
  播放音乐 `msc`(0-7)。播放的起始位置可以通过 `sec`（秒）指定。如果 `loop` 设置为 `True`，则执行循环播放。

- `stop([ch])`<br>
  停止指定通道 `ch`(0-3) 的播放。调用 `stop()` 停止所有通道。

- `play_pos(ch)`<br>
  获取通道 `ch`(0-3) 的声音播放位置，作为 `(sound_no, sec)` 的元组返回。当播放停止时，返回 `None`。

### 数学

- `ceil(x)`<br>
  返回大于或等于 `x` 的最小整数。

- `floor(x)`<br>
  返回小于或等于 `x` 的最大整数。

- `sgn(x)`<br>
  当 `x` 为正数时返回 `1`，当 `x` 为 `0` 时返回 `0`，当 `x` 为负数时返回 `-1`。

- `sqrt(x)`<br>
  返回 `x` 的平方根。

- `sin(deg)`<br>
  返回 `deg` 度的正弦值。

- `cos(deg)`<br>
  返回 `deg` 度的余弦值。

- `atan2(y, x)`<br>
  返回 `y`/`x` 的反正切值，单位为度。

- `rseed(seed)`<br>
  设置随机数生成器的种子。

- `rndi(a, b)`<br>
  返回大于或等于 `a` 且小于或等于 `b` 的随机整数。

- `rndf(a, b)`<br>
  返回大于或等于 `a` 且小于或等于 `b` 的随机浮点数。

- `nseed(seed)`<br>
  设置 Perlin 噪声的种子。

- `noise(x, [y], [z])`<br>
  返回指定坐标的 Perlin 噪声值。

### Image 类

- `width`, `height`<br>
  图像的宽度和高度

- `set(x, y, data)`<br>
  使用字符串列表在 (`x`, `y`) 处设置图像。<br>
  例子：`pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  在 (`x`, `y`) 处加载图像文件 (PNG/GIF/JPEG)。

- `pget(x, y)`<br>
  获取 (`x`, `y`) 处像素的颜色。

- `pset(x, y, col)`<br>
  在 (`x`, `y`) 处绘制颜色为 `col` 的像素。

### Tilemap 类

- `width`, `height`<br>
  瓦片地图的宽度和高度

- `imgsrc`<br>
  瓦片地图引用的图像库 (0-2)

- `set(x, y, data)`<br>
  使用字符串列表在 (`x`, `y`) 处设置瓦片地图。<br>
  例子：`pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  在 (`x`, `y`) 处从 TMX 文件 (Tiled Map File) 加载 `layer`(0-) 的图层。

- `pget(x, y)`<br>
  获取 (`x`, `y`) 处的瓦片。瓦片表示为 `(image_tx, image_ty)` 的元组。

- `pset(x, y, tile)`<br>
  在 (`x`, `y`) 处绘制一个瓦片。瓦片表示为 `(image_tx, image_ty)` 的元组。

### Sound 类

- `notes`<br>
  音符 (0-127) 的列表。数值越大，音高越高。音符 `33` 对应 'A2'(440Hz)。休止符表示为 `-1`。

- `tones`<br>
  音色的列表 (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
  音量的列表 (0-7)

- `effects`<br>
  效果的列表 (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  播放速度。`1` 是最快的，数值越大，播放速度越慢。当 `120` 时，一个音符的长度为 1 秒。

- `set(notes, tones, volumes, effects, speed)`<br>
  使用字符串设置音符、音色、音量和效果。如果音色、音量或效果的长度比音符短，它们将从头开始重复。

- `set_notes(notes)`<br>
  使用由 `CDEFGAB`+`#-`+`01234` 或 `R` 组成的字符串设置音符。它不区分大小写，并忽略空白。<br>
  例子：`pyxel.sounds[0].set_notes("g2b-2d3r rf3f3f3")`

- `set_tones(tones)`<br>
  使用由 `TSPN` 组成的字符串设置音色。不区分大小写，并忽略空白。<br>
  例子：`pyxel.sounds[0].set_tones("ttss pppn")`

- `set_volumes(volumes)`<br>
  使用由 `01234567` 组成的字符串设置音量。不区分大小写，并忽略空白。<br>
  例子：`pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  使用由 `NSVFHQ` 组成的字符串设置效果。不区分大小写，并忽略空白。<br>
  例子：`pyxel.sounds[0].set_effects("nfnf nvvs")`

- `mml(code)`<br>
  传入 [MML（音乐宏语言）](https://en.wikipedia.org/wiki/Music_Macro_Language) 字符串后，将切换到 MML 模式，并根据内容播放声音。在此模式下，`notes` 和 `speed` 等普通参数将被忽略。要退出 MML 模式，请调用 `mml()`（不带参数）。关于 MML 的详细说明，请参阅[此页面](faq-en.md)。<br>
  例子：`pyxel.sounds[0].mml("T120 Q90 @1 V100 O5 L8 C4&C<G16R16>C.<G16 >C.D16 @VIB1{10,20,20} E2C2")`

- `save(filename, sec, [ffmpeg])`<br>
  创建一个播放指定秒数声音的 WAV 文件。如果已安装 FFmpeg 并将 `ffmpeg` 设置为 `True`，还会创建一个 MP4 文件。

- `total_sec()`<br>
  返回声音的播放时间（秒）。如果 MML 使用了无限循环，则返回 `None`。

### Music 类

- `seqs`<br>
  一个二维列表，包含多个通道的声音 (0-63)

- `set(seq0, seq1, seq2, ...)`<br>
  设置每个通道的声音 (0-63) 列表。如果指定了空列表，则该通道不会用于播放。<br>
  例子：`pyxel.musics[0].set([0, 1], [], [3])`

- `save(filename, sec, [ffmpeg])`<br>
  创建一个播放指定秒数音乐的 WAV 文件。如果已安装 FFmpeg 并将 `ffmpeg` 设置为 `True`，还会创建一个 MP4 文件。

### 高级 API

Pyxel 包含一个“高级 API”，该 API 在本参考中未提及，因为它可能会让用户感到困惑或需要专业知识才能使用。

如果您对自己的技能充满信心，可以尝试使用[这个](../python/pyxel/__init__.pyi)作为指南，创造惊人的作品！

## 如何贡献

### 提交问题

使用 [问题跟踪器](https://github.com/kitao/pyxel/issues) 提交 bug 报告和功能或增强请求。在提交新问题之前，请确保没有类似的开放问题。

### 功能测试

任何手动测试代码并在 [问题跟踪器](https://github.com/kitao/pyxel/issues) 中报告 bug 或增强建议的人都非常欢迎！

### 提交拉取请求

补丁和修复以拉取请求 (PR) 的形式接受。请确保拉取请求所针对的问题在问题跟踪器中是开放的。

提交拉取请求意味着您同意根据 [MIT 许可证](../LICENSE) 授权您的贡献。

## 其他信息

- [常见问题](faq-en.md)
- [用户示例](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [开发者的 X 帐号](https://x.com/kitao)
- [Discord 服务器（英文）](https://discord.gg/Z87eYHN)
- [Discord 服务器（日文）](https://discord.gg/qHA5BCS)

## 许可证

Pyxel 采用 [MIT 许可证](../LICENSE)。它可以在专有软件中重复使用，前提是所有软件或其重要部分的副本都包含 MIT 许可证条款和版权声明的副本。

## 征募赞助者

Pyxel 在 GitHub Sponsors 上寻找赞助者。请考虑赞助 Pyxel，以支持其持续维护和功能开发。作为一种福利，赞助者可以直接咨询 Pyxel 开发者。有关更多详细信息，请访问 [此页面](https://github.com/sponsors/kitao)。
