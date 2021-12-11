# <img src="images/pyxel_logo_152x64.png">

[ [English](README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) ]

**NOTE: This manual has not yet been translated for Pyxel version 1.5.0. We are looking for volunteers to translate and check for mistakes!**

**Pyxel (픽셀)** 은 Python을 위한 레트로 게임 엔진입니다.

16가지 색상만 사용하거나 동시에 4가지 소리만 재생하는 등 레트로 게임에 나올 법한 사양으로, Python에서 픽셀 아트 스타일의 게임을 마음껏 만들 수 있습니다.

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

Pyxel은 오픈 소스로, 무료로 자유롭게 사용할 수 있습니다. Pyxel과 함께 레트로 스타일의 게임을 만들어보세요!

## 사양

- Windows, Mac, Linux 지원
- Programming with Python
- 16 color palette
- 256x256 크기의 이미지 뱅크 3개
- 256x256 크기의 타일 맵 8개
- 4개의 사운드 동시 재생, 64개의 정의 가능한 사운드
- 임의의 사운드를 조합 가능한 8개의 음악
- 키보드, 마우스, 게임패드 입력
- 이미지/사운드 에디터

### 색상 팔레트

<img src="pyxel/examples/screenshots/05_color_palette.png">
<br><br>
<img src="images/pyxel_palette.png">

## 설치 방법

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

### 예제 설치

Pyxel 설치 후, 아래 명령어를 사용해 현재 폴더에 Pyxel 예제를 복사할 수 있습니다:

```sh
pyxel copy_examples
```

복사되는 예제는 다음과 같습니다:

- [01_hello_pyxel.py](pyxel/examples/01_hello_pyxel.py) - 간단한 애플리케이션
- [02_jump_game.py](pyxel/examples/02_jump_game.py) - Pyxel 리소스 파일을 사용한 점프 게임
- [03_draw_api.py](pyxel/examples/03_draw_api.py) - Demonstration of drawing APIs
- [04_sound_api.py](pyxel/examples/04_sound_api.py) - Demonstration of sound APIs
- [05_color_palette.py](pyxel/examples/05_color_palette.py) - 색상 팔레트 목록
- [06_click_game.py](pyxel/examples/06_click_game.py) - 마우스 클릭 게임
- [07_snake.py](pyxel/examples/07_snake.py) - BGM이 포함된 스네이크 게임
- [08_triangle_api.py](pyxel/examples/08_triangle_api.py) - Demonstration of triangle drawing APIs
- [09_shooter.py](pyxel/examples/09_shooter.py) - 화면 전환으로 슈팅 게임
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

## 사용 방법

### Pyxel 애플리케이션 작성 방법

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

`run` 함수의 인자로는 프레임 갱신을 처리하는 `update` 함수와, 필요할 때 화면을 그리는 `draw` 함수가 사용됩니다.

실제 애플리케이션에서는 아래와 같이 클래스에서 Pyxel 코드를 감싸는 것이 좋습니다:

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


### 특수 조작

Pyxel 애플리케이션 실행 중에, 아래의 특수 조작을 사용할 수 있습니다:

- `Esc`<br>
애플리케이션 종료
- `Alt(Option)+1`<br>
바탕 화면에 스크린샷 저장
- `Alt(Option)+2`<br>
화면 캡쳐의 녹화 시작 시간 초기화
- `Alt(Option)+3`<br>
화면 캡쳐 파일 을 바탕 화면에 저장 (최대 10초)
- `Alt(Option)+0`<br>
성능 모니터 (fps, update time, and draw time)의 표시/표시 해제
- `Alt(Option)+Enter`<br>
전체 화면 전환

### 리소스의 작성 방법

Pyxel Editor can create images and sounds used in a Pyxel application.

It starts with the following command:

```sh
pyxel edit [PYXEL_RESOURCE_FILE]
```

지정한 Pyxel 리소스 파일 (.pyxres)이 존재하는 경우에는 해당 파일을 불러오고, 존재하지 않는 경우 지정한 이름으로 새 리소스 파일을 생성합니다. 파일 이름을 생략했을 경우, 기본 파일 이름은 `my_resource.pyxres`입니다.

After starting Pyxel Editor, the file can be switched by dragging and dropping another resource file. If the resource file is dragged and dropped while holding down ``Ctrl(Cmd)`` key, only the resource type (Image/Tilemap/Sound/Music) that is currently being edited will be loaded. This operation enables to combine multiple resource files into one.

The created resource file can be loaded with `load` function.

Pyxel Editor는 다음과 같은 편집 모드가 있습니다:

**이미지 에디터:**

이미지 뱅크를 편집하는 화면입니다.

<img src="pyxel/editor/screenshots/image_editor.gif">

By dragging and dropping an image file (png/gif/jpeg) onto the Image Editor screen, the image can be loaded into the currently selected image bank.

**타일 맵 에디터:**

이미지 뱅크의 이미지를 타일 모양으로 늘어놓은 타일 맵을 편집하는 화면입니다.

<img src="pyxel/editor/screenshots/tilemap_editor.gif">

**사운드 에디터:**

사운드를 편집하는 화면입니다.

<img src="pyxel/editor/screenshots/sound_editor.gif">

**음악 에디터:**

사운드를 플레이 순서대로 늘어놓은 음악을 편집하는 화면입니다.

<img src="pyxel/editor/screenshots/music_editor.gif">

### 기타 리소스 작성 방법

Pyxel images and tilemaps can also be created by the following methods:

- Create an image from a list of strings with `Image.set` function or `Tilemap.set` function
- Load an image file (png/gif/jpeg) in Pyxel palette with `Image.load` function

Pyxel sounds can also be created in the following method:

- Create a sound from strings with `Sound.set` function or `Music.set` function

각 함수의 사용법은 API 레퍼런스를 참조해주세요.

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

## API 레퍼런스

### 시스템

- `width`, `height`<br>
화면의 가로/세로 크기

- `frame_count`<br>
경과한 프레임의 수

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

### 리소스

- `load(filename, [image], [tilemap], [sound], [music])`<br>
Load the resource file (.pyxres). If ``False`` is specified for the resource type (``image/tilemap/sound/music``), the resource will not be loaded.

### 입력
- `mouse_x`, `mouse_y`<br>
마우스 커서의 현재 좌표를 나타냅니다.

- `mouse_wheel`<br>
마우스 휠의 현재 값을 나타냅니다.

- `btn(key)`<br>
`key`가 눌리고 있으면 `True`, 눌리고 있지 않으면 `False`를 반환합니다. ([키 정의 리스트](pyxel/__init__.pyi))

- `btnp(key, [hold], [period])`<br>
해당 프레임에 `key`가 눌리면 `True`, 눌리지 않으면 `False`를 반환합니다. `hold`와 `period`를 지정하면, `hold` 프레임 이상 `key`가 눌린 상태인 경우 `period` 프레임 간격으로 `True`를 반환합니다.

- `btnr(key)`<br>
해당 프레임에 `key`가 떼어지면 `True`, 아니면 `False`를 반환합니다.

- `mouse(visible)`<br>
`visible`이 `True`인 경우 마우스 커서를 표시하고, `False`라면 표시하지 않습니다. 마우스 커서가 보이지 않아도 마우스 커서의 좌표는 갱신됩니다.

### 그래픽

- `colors`<br>
List of the palette display colors. The display color is specified by a 24-bit numerical value. Use `colors.from_list` and `colors.to_list` to directly assign and retrieve Python lists.<br>
e.g. `org_colors = pyxel.colors.to_list(); pyxel.colors[15] = 0x112233; pyxel.colors.from_list(org_colors)`

- `image(img)`<br>
Operate the image bank `img` (0-2). (See the Image class)<br>
e.g. `pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
타일 맵 `tm`(0-7)을 조작합니다(타일 맵 클래스를 참조).

- `clip(x, y, w, h)`<br>
화면의 드로잉 영역을 (`x`, `y`)로 설정하고, 폭을 `w`, 높이를 `h`로 설정합니다. `clip()`과 같이 사용하면 드로잉 영역을 초기 상태(전체 화면)으로 돌립니다.

- `pal(col1, col2)`<br>
드로잉 시 `col1`를 `col2`로 대체합니다. `pal()`과 같이 사용하면 초기 상태로 돌립니다.

- `cls(col)`<br>
화면을 `col` 색으로 지웁니다.

- `pget(x, y)`<br>
(`x`, `y`) 좌표의 색상 값을 가져옵니다.

- `pset(x, y, col)`<br>
`col` 색을 사용해 (`x`, `y`) 좌표에 픽셀을 찍습니다.

- `line(x1, y1, x2, y2, col)`<br>
`col` 색을 사용해 (`x1`, `y1`)부터 (`x2`, `y2`)까지 직선을 그립니다.

- `rect(x, y, w, h, col)`<br>
가로 `w`, 세로 `h`의 크기로 `col` 색을 사용해 직사각형을 (`x`, `y`) 좌표에 그립니다.

- `rectb(x, y, w, h, col)`<br>
가로 `w`, 세로 `h`의 크기로 `col` 색을 사용해 직사각형 테두리를 (`x`, `y`) 좌표에 그립니다. (테두리 안쪽에 색상을 채우지 않음)

- `circ(x, y, r, col)`<br>
반경 `r`, `col` 색의 원을 (`x`, `y`) 좌표에 그립니다.

- `circb(x, y, r, col)`<br>
반경 `r`, `col` 색의 원 테두리를 (`x`, `y`) 좌표에 그립니다. (테두리 안쪽에 색상을 채우지 않음)

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
꼭짓점 좌표 (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`)를 기준으로 `col` 색상의 삼각형을 그립니다.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
꼭짓점 좌표 (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`)를 기준으로 `col` 색상의 삼각형 테두리를 그립니다. (테두리 안쪽에 색상을 채우지 않음)

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
이미지 뱅크 `img`(0-2)의 (`u`, `v`)부터 (`w`, `h`)까지의 영역을 (`x`, `y`) 좌표에 복사합니다. `w`, `h`의 값을 마이너스로 설정하면, 각각 수평, 수직 방향으로 반전됩니다. `colkey`로 색을 지정하면 투명 색상으로 처리됩니다.

<img src="images/image_bank_mechanism.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
Draw the tilemap `tm` (0-7) to (`x`, `y`) according to the tile information of size (`w`, `h`) from (`u`, `v`). If `colkey` is specified, treated as transparent color. The size of a tile is 8x8 pixels and is stored in a tilemap as a tuple of `(x in tile, y in tile)`.

- `text(x, y, s, col)`<br>
`col` 색을 사용해 문자열 `s`를 (`x`, `y`) 좌표에 그립니다.

### 오디오

- `sound(snd)`<br>
사운드 `snd`(0-63) 를 조작합니다. (사운드 클래스를 참조)<br>
예: `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
음악 `msc`(0-7) 를 조작합니다(음악 클래스를 참조).

- `play_pos(ch)`<br>
Get the sound playback position of channel `ch` (0-3) as a tuple of `(sound no, note no)`. Returns `None` when playback is stopped.

- `play(ch, snd, loop=False)`<br>
Play the sound `snd` (0-63) on channel `ch` (0-3). If `snd` is a list, it will be played in order. If `True` is specified for `loop`, loop playback is performed.

- `playm(msc, loop=False)`<br>
Play the music `msc` (0-7). If `True` is specified for `loop`, loop playback is performed.

- `stop([ch])`<br>
Stops playback of the specified channel `ch` (0-3). `stop()` to stop playing all channels.

### 이미지 클래스

- `width`, `height`<br>
이미지의 가로/세로 크기

- `data`<br>
이미지의 데이터 (256x256 크기의 2차원 리스트)

- `get(x, y)`<br>
이미지의 (`x`,`y`) 데이터를 가져옵니다.

- `set(x, y, data)`<br>
Set the image at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
Load the image file (png/gif/jpeg) at (`x`, `y`).

### 타일 맵 클래스

- `width`, `height`<br>
타일 맵의 가로/세로 크기

- `refimg`<br>
The image bank (0-2) referenced by the tilemap

- `set(x, y, data)`<br>
Set the tilemap at (`x`, `y`) by a list of strings.<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `pget(x, y)`<br>
Get the tile at (`x`, `y`). A tile is a tuple of `(x in tile, y in tile)`.

- `pset(x, y, tile)`<br>
Draw a `tile` at (`x`, `y`). A tile is a tuple of `(x in tile, y in tile)`.

### 사운드 클래스

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

### 음악 클래스

- `sequences`<br>
Two-dimensional list of sounds (0-63) listed by the number of channels

- `set(seq0, seq1, seq2, seq3)`<br>
Set the lists of sound (0-63) of all channels. If an empty list is specified, that channel is not used for playback.<br>
e.g. `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

### Advanced APIs

Pyxel has "advanced APIs" that are not mentioned in this reference because they "may confuse users" or "need specialized knowledge to use".

If you are familiar with your skills, try to create amazing works with [this](pyxel/__init__.pyi) as a clue!

## 컨트리뷰션 방법

### Submitting an Issue

Use the [Issue Tracker](https://github.com/kitao/pyxel/issues) to submit bug reports and feature/enhancement requests. Before submitting a new issue, ensure that there is no similar open issue.

### Manual Testing

Anyone manually testing the code and reporting bugs or suggestions for enhancements in the [Issue Tracker](https://github.com/kitao/pyxel/issues) are very welcome!

### Submitting a Pull Request

패치나 수정 요청은 풀 리퀘스트(PR)로 받고 있습니다. 제출하기 전에 문제가 이미 해결되지 않았는지 [Issue Tracker](https://github.com/kitao/pyxel/issues) 페이지에서 확인 부탁드립니다.

제출한 풀 리퀘스트는 [MIT License](LICENSE)에 따라 게시하기를 동의한 것으로 간주됩니다.

## 기타 정보

- [Discord server (English)](https://discord.gg/FC7kUZJ)
- [Discord server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## 라이선스 정보

Pyxel is under [MIT License](http://en.wikipedia.org/wiki/MIT_License). It can be reused within proprietary software, provided that all copies of the software or its substantial portions include a copy of the terms of the MIT License and also a copyright notice.
