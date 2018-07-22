# Pyxel

[ [English](https://github.com/kitao/pyxel/blob/master/README.md) | [Japanese](https://github.com/kitao/pyxel/blob/master/README.ja.md) ]

Pyxel is a game development environment in Python.

Thanks to its simple specifications inspired by retro gaming consoles, such as only 16 colors can be displayed and only 4 sounds can be played back at the same time, you can feel free to enjoy making pixel art style games. 

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/02_simple_game.py" target="_blank">
<img
src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/02_simple_game.gif" width="32%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/03_draw_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/03_draw_api.gif" width="32%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/04_sound_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/04_sound_api.gif" width="32%">
</a>

The specifications of the gaming console, APIs, and paletts of Pyxel are reffering to awesome [PICO-8](https://www.lexaloffle.com/pico-8.php) and [TIC-80](https://tic.computer/)

Pyxel is open souce and free to use. Let's start making a retro game with Pyxel!

## Specifications

- Run on both Windows and Mac
- Code writing with Python3
- Fixed 16 color palette
- 128x128 sized 4 image banks
- 4 channels with 64 definable sounds
- Keyboard, mouse, and joystick(WIP) inputs
- Image and sound editor(WIP)

## How to Install

### Windows

After installing [Python3](https://www.python.org/), the following `pip` command installs Pyxel:

```sh
pip install pyxel
```

### Mac

After installing [Python3](https://www.python.org/) and [glfw](http://www.glfw.org/), install Pyxel with `pip` command.

If [Homebrew](https://brew.sh/) package manager is ready, the following command installs all the necessary packages:

```sh
brew install python3 glfw
pip3 install pyxel
```

### Install examples

After installing Pyxel, the examples of Pyxel can be copied to the current directory with the folloing command:

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

In an actual application, it is recommended to wrap pyxel commands in a class as below:

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

The following special controls can be performed while a Pyxel application is running:

- `Alt(Option)+1`  
Saves the screenshot to the desktop
- `Alt(Option)+2`  
Resets the recording start time of the screen capture video
- `Alt(Option)+3`  
Saves the screen capture video (gif) to the desktop
- `Alt(Option)+0`  
Toggles the performance monitor (fps, update time, and draw time)
- `Alt(Option)+Enter`  
Toggles full screen

### Create Images for Pyxel

There are the following methods to create images for Pyxel:

- Create image from a list of strings with `Image.set` command
- Load a png file of Pyxel palette with `Image.load` command
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
Initialize the Pyxel application with screen size (width, height).

- `run(update, draw)`  
Start the Pyxel application and call `update` function for frame update and `draw` function for drawing.

- `quit()`  
End the Pyxel application at the end of the current frame.

### Input
- `mouse_x`, `mouse_y`  
The current position of the mouse cursor

- `btn(key)`  
keyが押されていたらTrue、押されていなければFalseを返す([キー定義一覧](https://github.com/kitao/pyxel/blob/master/pyxel/constants.py))

- `btnp(key, [hold], [period])`  
そのフレームにキーが押されたらTrue、押されなければFalseを返す。holdとperiodを指定すると、holdフレーム以上ボタンを押し続けた際にperiodフレーム間隔でTrueが返る

- `btnr(key)`  
Returns True is the key is pressed at the frame, otherwise returns False.

### Graphics

- `image(no)`  
イメージno(0-3)を操作する(イメージクラスを参照のこと)  
例：`pyxel.image(0).load(0, 0, "title.png")`

- `clip(x1, y1, x2, y2)`  
画面の描画領域を(x1, y1)-(x2, y2)にする。`clip()`で描画領域をリセットする

- `pal(col1, col2)`  
描画時に色col1をcol2に置き換える。`pal()`で初期状態にリセットする

- `cls(col)`  
画面を色colでクリアする

- `pix(x, y, col)`  
(x, y)に色colのピクセルを描画する

- `line(x1, y1, x2, y2, col)`  
(x1, y1)-(x2, y2)に色colの直線を描画する

- `rect(x1, y1, x2, y2, col)`  
(x1, y1)-(x2, y2)に色colの矩形を描画する

- `rectb(x1, y1, x2, y2, col)`  
(x1, y1)-(x2, y2)に色colの矩形の輪郭を描画する

- `circ(x, y, r, col)`  
(x, y)に半径r、色colの円を描画する

- `circb(x, y, r, col)`  
(x, y)に半径r、色colの円の輪郭を描画する

- `blt(x, y, no, sx, sy, w, h, [colkey])`  
(x, y)にイメージno(0-3)の(sx, sy)からサイズ(w, h)の画像をコピーする。colkeyに色を指定すると透明色として扱われる

- `text(x, y, s, col)`  
(x, y)に色colで文字列sを描画する

### Audio

- `sound(no)`  
サウンドno(0-63)を操作する(サウンドクラスを参照のこと)  
例：`pyxel.sound(0).speed = 60`

- `play(ch, no, loop=False)`  
チャンネルch(0-3)でサウンドno(0-63)を再生する。noがリストの場合順に再生する

- `stop(ch)`  
チャンネルch(0-3)の再生を停止する

### Image Class

- `width`, `height`  
イメージの幅と高さ

- `data`  
イメージのデータ(NumPy配列)

- `resize(width, height)`  
イメージのサイズを(width, height)に変更する

- `set(x, y, data)`  
(x, y)に文字列のリストでイメージを設定する   
例：`pyxel.image(0).set(10, 10, ['1234', '5678', '9abc', 'defg'])`

- `load(x, y, filename)`  
(x, y)にpngファイルを読み込む

- `copy(x, y, no, sx, sy, width, height)`  
(x, y)にイメージno(0-3)の(sx, sy)からサイズ(width, height)の画像をコピーする

### Sound Class

- `note`  
音程(0-127)のリスト(33='A2'=440Hz)

- `tone`  
音色(0:Triagnle/1:Square/2:Pulse/3:Noise)のリスト

- `volume`  
音量(0-7)のリスト

- `effect`  
エフェクト(0:None/1:Slide/2:Vibrato/3:FadeOut)のリスト

- `speed`  
1音の長さ(120=1音1秒)

- `set(note, tone, volume, effect, speed)`  
文字列で音程、音色、音量、エフェクトを設定する。音色、音量、エフェクトの長さが音程より短い場合は、先頭から繰り返される

- `set_note(note)`  
'CDEFGAB'+'#-'+'0123'または'R'の文字列で音程を設定する  
例：`pyxel.sound(0).set_note('G2B-2RD3RF3')`

- `set_tone(tone)`  
'TSPN'の文字列で音色を設定する

- `set_volume(volume)`  
'01234567'の文字列で音量を設定する

- `set_effect(effect)`  
'NSVF'の文字列でエフェクトを設定する

## License

Pyxel is under [MIT license](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software provided that all copies of the licensed software include a copy of the MIT License terms and the copyright notice.