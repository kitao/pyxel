# <img src="https://raw.githubusercontent.com/kitao/pyxel/master/images/pyxel_logo_152x64.png">

[ [English](https://github.com/kitao/pyxel/blob/master/README.md) | [日本語](https://github.com/kitao/pyxel/blob/master/README.ja.md) | [中文](https://github.com/kitao/pyxel/blob/master/README.cn.md) | [한국어](https://github.com/kitao/pyxel/blob/master/README.ko.md) | [Español](https://github.com/kitao/pyxel/blob/master/README.es.md) | [Português](https://github.com/kitao/pyxel/blob/master/README.pt.md) ]

**Pyxel (픽셀)** 은 Python을 위한 레트로 게임 엔진입니다.

16가지 색상만 사용하거나 동시에 4가지 소리만 재생하는 등 레트로 게임에 나올 법한 사양으로, Python에서 픽셀 아트 스타일의 게임을 마음껏 만들 수 있습니다.

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

Pyxel의 게이밍 콘솔, API의 사양은 [PICO-8](https://www.lexaloffle.com/pico-8.php)과 [TIC-80](https://tic.computer/)의 디자인을 참고하고 있습니다.

Pyxel은 오픈 소스로, 무료로 자유롭게 사용할 수 있습니다. Pyxel과 함께 레트로 스타일의 게임을 만들어보세요!

## 사양

- Windows, Mac, Linux 지원
- Python3으로 코드 작성
- 16색 고정 팔레트
- 256x256 크기의 이미지 뱅크 3개
- 256x256 크기의 타일 맵 8개
- 4개의 사운드 동시 재생, 64개의 정의 가능한 사운드
- 임의의 사운드를 조합 가능한 8개의 음악
- 키보드, 마우스, 게임패드 입력
- 이미지/사운드 에디터

### 색상 팔레트

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/05_color_palette.png">
<br><br>
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/images/pyxel_palette.png">

## 설치 방법

### Windows

먼저 [Python3](https://www.python.org/) (버전 3.6.9 이상)을 설치하십시오.

공식 설치 프로그램으로 Python을 설치할 때 아래 버튼을 확인하여 **PATH에 Python을 추가** 하십시오:

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/images/python_installer.png">

다음, 명령 프롬프트에서 다음`pip` 명령으로 Pyxel을 설치하십시오:

```sh
pip install -U pyxel
```

### Mac

[Python3](https://www.python.org/) (버전 3.6.9 이상)과 [SDL2](https://www.libsdl.org/)를 설치한 후, 아래의 `pip` 명령어를 통해 Pyxel을 설치합니다.

[Homebrew](https://brew.sh/) 패키지 관리자를 사용할 수 있다면, 아래의 명령으로 필요한 패키지를 모두 설치할 수 있습니다:

```sh
brew install python3 sdl2 sdl2_image
```

**터미널을 다시 시작**한 후,

```sh
pip3 install -U pyxel
```

### Linux

각 distribution에 적합한 방법으로 [Python3](https://www.python.org/) (버전 3.6.9 이상)과 필요한 패키지를 설치하시면 됩니다.

**Ubuntu:**

```sh
sudo apt install python3 python3-pip libsdl2-dev libsdl2-image-dev
sudo -H pip3 install -U pyxel
```

### 기타 환경

이외의 환경(32-Bit 리눅스, 라즈베리 파이 등)에서 Pyxel을 설치하려면 아래 단계를 통해 직접 빌드할 수 있습니다:

#### 필요한 툴과 패키지 설치

- C++ build toolchain (gcc 및 make 명령어를 포함해야 함)
- libsdl2-dev
- libsdl2-image-dev
- [Python3](https://www.python.org/) (버전 3.6.9 이상), pip

#### 임의의 폴더에서 아래 명령어 실행

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make -C pyxel/core clean all
pip3 install .
```

### 예제 설치

Pyxel 설치 후, 아래 명령어를 사용해 현재 폴더에 Pyxel 예제를 복사할 수 있습니다:

```sh
install_pyxel_examples
```

복사되는 예제는 다음과 같습니다:

- [01_hello_pyxel.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/01_hello_pyxel.py) - 간단한 애플리케이션
- [02_jump_game.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/02_jump_game.py) - Pyxel 리소스 파일을 사용한 점프 게임
- [03_draw_api.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/03_draw_api.py) - Drawing API를 사용한 그리기 데모
- [04_sound_api.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/04_sound_api.py) - 사운드 API 데모
- [05_color_palette.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/05_color_palette.py) - 색상 팔레트 목록
- [06_click_game.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/06_click_game.py) - 마우스 클릭 게임
- [07_snake.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/07_snake.py) - BGM이 포함된 스네이크 게임
- [08_triangle_api.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/08_triangle_api.py) - Triangle Drawing API를 사용한 삼각형 그리기 데모

예제 파일은 일반적인 Python 코드와 같이 실행할 수 있습니다:

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

## 사용 방법

### Pyxel 애플리케이션 작성 방법

먼저 Python 코드 내에서 Pyxel 모듈을 import한 뒤, `init` 함수로 화면 크기를 지정한 후에, `run` 함수로 Pyxel 애플리케이션을 실행합니다.

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

`show`나 `flip` 함수를 이용해 간단한 그래픽이나 애니메이션을 그리는 것도 가능합니다.

`show` 함수는 화면을 표시하고 `ESC` 키가 눌릴 때까지 대기합니다.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

`flip` 함수는 화면을 한 번 갱신하는 함수입니다.

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
화면 캡쳐 파일(gif)을 바탕 화면에 저장 (최대 30초)
- `Alt(Option)+0`<br>
성능 모니터 (fps, update time, and draw time)의 표시/표시 해제
- `Alt(Option)+Enter`<br>
전체 화면 전환

### 리소스의 작성 방법

Pyxel Editor는 Pyxel 애플리케이션에 사용되는 이미지와 사운드를 제작할 수 있습니다.

Pyxel Editor는 아래 명령어를 사용해 시작할 수 있습니다:

```sh
pyxeleditor [Pyxel 리소스 파일]
```

지정한 Pyxel 리소스 파일 (.pyxres)이 존재하는 경우에는 해당 파일을 불러오고, 존재하지 않는 경우 지정한 이름으로 새 리소스 파일을 생성합니다. 파일 이름을 생략했을 경우, 기본 파일 이름은 `my_resource.pyxres`입니다.

Pyxel Editor 실행 중 다른 리소스 파일을 Drag & Drop하는 것으로 작업 중인 리소스 파일을 변경할 수 있습니다.
또한 ``Ctrl``(``Cmd``) 키를 누르면서 리소스 파일을 Drag & Drop 하면, 현재 편집 중인 리소스 유형(이미지/타일 맵/사운드/뮤직)만 불러올 수 있습니다. 이를 통해 여러 개의 리소스 파일을 하나로 결합할 수 있습니다.

작성한 리소스 파일은 Pyxel 애플리케이션에서 `load` 함수를 사용해 불러올 수 있습니다.

Pyxel Editor는 다음과 같은 편집 모드가 있습니다:

**이미지 에디터:**

이미지 뱅크를 편집하는 화면입니다.

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/image_editor.gif">

이미지 에디터 화면에 png 파일을 Drag & Drop하면, 이미지 파일을 선택 중인 이미지 뱅크에 추가할 수 있습니다.

**타일 맵 에디터:**

이미지 뱅크의 이미지를 타일 모양으로 늘어놓은 타일 맵을 편집하는 화면입니다.

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/tilemap_editor.gif">

**사운드 에디터:**

사운드를 편집하는 화면입니다.

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/sound_editor.gif">

**음악 에디터:**

사운드를 플레이 순서대로 늘어놓은 음악을 편집하는 화면입니다.

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/music_editor.gif">

### 기타 리소스 작성 방법

Pyxel을 위한 이미지나 타일 맵은 아래와 같은 방법으로 편집할 수도 있습니다:

- `Image.set`나 `Tilemap.set` 함수를 사용해 문자열 리스트에서 이미지 생성
- `Image.load` 함수를 사용해 Pyxel 색상 팔레트로 png 파일 불러오기

Pyxel을 위한 사운드나 음악은 아래의 방법으로 작성할 수도 있습니다:

- `Sound.set`이나 `Music.set` 함수로 문자열에서 사운드 생성

각 함수의 사용법은 API 레퍼런스를 참조해주세요.

### Stand-Alone 실행 파일 작성 방법

Pyxel Packager를 사용해 Python이 설치되지 않은 환경에서도 실행 가능한 독립 실행 파일을 생성할 수 있습니다.

실행 파일을 작성하려면 다음과 같이 `pyxelpackager` 명령어로 애플리케이션의 실행에 사용하는 Python 파일을 지정합니다:

```sh
pyxelpackager python_file
```

처리가 완료되면 dist 폴더에 실행 가능한 파일이 생성됩니다.

.pyxres 파일이나 .png 파일 등의 리소스도 필요한 경우 리소스를 `assets` 폴더 내에 넣으면 포함할 수 있습니다.

``-i icon_file`` 옵션으로 애플리케이션의 아이콘을 지정할 수도 있습니다.

## API 레퍼런스

### 시스템

- `width`, `height`<br>
화면의 가로/세로 크기

- `frame_count`<br>
경과한 프레임의 수

- `init(width, height, [caption], [scale], [palette], [fps], [quit_key], [fullscreen])`<br>
Pyxel 애플리케이션을 (`width`, `height`) 크기로 초기화합니다. 화면의 가로/세로 최대 크기는 256입니다.<br>
`caption`으로 창 제목, `scale`로 표시 배율, `palette`로 팔레트 색, `fps`로 동작 프레임 레이트, `quit_key`로 애플리케이션의 종료 키를 지정할 수 있습니다, 그리고 `fullscreen`으로 전체 화면으로 시작할지 여부. `palette`는 24비트 색상 중 16개의 요소로 지정합니다.<br>
예: `pyxel.init(160, 120, caption="Pyxel with PICO-8 palette", palette=[0x000000, 0x1D2B53, 0x7E2553, 0x008751, 0xAB5236, 0x5F574F, 0xC2C3C7, 0xFFF1E8, 0xFF004D, 0xFFA300, 0xFFEC27, 0x00E436, 0x29ADFF, 0x83769C, 0xFF77A8, 0xFFCCAA], quit_key=pyxel.KEY_NONE, fullscreen=True)`

- `run(update, draw)`<br>
Pyxel 애플리케이션을 실행하며, 프레임 갱신 시 `update` 함수를, 화면 그리기에 `draw` 함수를 호출합니다.

- `quit()`<br>
현재 프레임 종료 시에 Pyxel 애플리케이션을 종료합니다.

- `flip()`<br>
강제로 화면을 그립니다. (일반적인 애플리케이션에선 사용하지 않음)

- `show()`<br>
화면을 그린 후 계속 기다립니다. (일반적인 애플리케이션에선 사용하지 않음)

### 리소스

- `save(filename)`<br>
실행 스크립트가 위치한 폴더에 리소스 파일 (.pyxres)을 저장합니다.

- `load(filename, [image], [tilemap], [sound], [music])`<br>
실행 스크립트가 위치한 폴더에서 리소스 파일 (.pyxres)을 불러옵니다. 리소스 타입(image/tilemap/sound/music)에 False를 지정하면, 해당 리소스는 불러오지 않습니다.

### 입력
- `mouse_x`, `mouse_y`<br>
현재의 마우스 커서 좌표를 나타냅니다.

- `mouse_wheel`<br>
마우스 휠의 현재 값

- `btn(key)`<br>
`key`가 눌리고 있으면 `True`, 눌리고 있지 않으면 `False`를 반환합니다. ([키 정의 리스트](https://github.com/kitao/pyxel/blob/master/pyxel/__init__.py))

- `btnp(key, [hold], [period])`<br>
해당 프레임에 `key`가 눌리면 `True`, 눌리지 않으면 `False`를 반환합니다. `hold`와 `period`를 지정하면, `hold` 프레임 이상 `key`가 눌린 상태인 경우 `period` 프레임 간격으로 `True`를 반환합니다.

- `btnr(key)`<br>
해당 프레임에 `key`가 떼어지면 `True`, 아니면 `False`를 반환합니다.

- `mouse(visible)`<br>
`visible`이 `True`인 경우 마우스 커서를 표시하고, `False`라면 표시하지 않습니다. 마우스 커서가 보이지 않아도 마우스 커서의 좌표는 갱신됩니다.

### 그래픽

- `image(img, [system])`<br>
이미지 뱅크 `img`(0-2) 를 조작합니다(이미지 클래스 참조). `system`에 `True`를 지정하면 시스템용 이미지 뱅크에 접근할 수 있습니다. 3은 폰트 및 리소스 에디터이며, 4는 화면 표시용입니다.<br>
예: `pyxel.image(0).load(0, 0, "title.png")`

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

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
타일 맵 `tm`(0-7)을 (`u`, `v`)부터 (`w`, `h`)까지의 타일 정보에 따라 (`x`, `y`) 좌표에 그립니다. `colkey`로 색을 지정하면 투명 색상으로 처리됩니다. 타일 맵의 타일은 8x8 크기로 그려지며, 타일 번호가 0이면 이미지 뱅크의 (0, 0)-(7, 7), 1이면 (8, 0)-(15, 0) 영역을 나타냅니다.

- `text(x, y, s, col)`<br>
`col` 색을 사용해 문자열 `s`를 (`x`, `y`) 좌표에 그립니다.

### 오디오

- `sound(snd, [system])`<br>
사운드 `snd`(0-63) 를 조작합니다(사운드 클래스를 참조). `system`에 `True`를 지정하면, 시스템용 사운드인 64에 접근할 수 있습니다.<br>
예: `pyxel.sound(0).speed = 60`

- `music(msc)`<br>
음악 `msc`(0-7) 를 조작합니다(음악 클래스를 참조).

- `play_pos(ch)`<br>
채널 `ch`(0-3)의 사운드 재생 위치를 가져옵니다. 100, 1000 단위는 사운드 번호, 1, 10 단위는 노트 번호를 의미하며, 사운드 재생이 중지 상태이면 `-1`를 반환합니다.

- `play(ch, snd, loop=False)`<br>
채널 `ch`(0-3)로 사운드 `snd`(0-63)를 재생합니다. `snd`가 리스트면 순서대로 재생합니다.

- `playm(msc, loop=False)`<br>
음악 `msc`(0-7)를 재생합니다.

- `stop([ch])`<br>
모든 채널의 사운드 재생을 중지합니다. 채널 `ch`(0-3)을 지정하면 해당 채널만 중지됩니다.

### 이미지 클래스

- `width`, `height`<br>
이미지의 가로/세로 크기

- `data`<br>
이미지의 데이터 (256x256 크기의 2차원 리스트)

- `get(x, y)`<br>
이미지의 (`x`,`y`) 데이터를 가져옵니다.

- `set(x, y, data)`<br>
이미지의 (`x`, `y`) 데이터를 값 또는 문자열 리스트를 사용해 설정합니다.<br>
예: `pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
실행 스크립트가 위치한 폴더에서 png 파일을 (`x`, `y`) 좌표에 불러옵니다.

- `copy(x, y, img, u, v, w, h)`<br>
이미지 뱅크 `img`(0-2)의 (`u`, `v`)부터 (`w`, `h`)까지의 영역을 (`x`, `y`) 좌표에 복사합니다.

### 타일 맵 클래스

- `width`, `height`<br>
타일 맵의 가로/세로 크기

- `data`<br>
타일 맵의 데이터 (256x256 크기의 2차원 리스트)

- `refimg`<br>
타일 맵이 참조하는 이미지 뱅크

- `get(x, y)`<br>
타일 맵의 (`x`,`y`) 데이터를 가져옵니다.

- `set(x, y, data)`<br>
타일 맵의 (`x`, `y`) 데이터를 값 또는 문자열 리스트를 사용해 설정합니다.<br>
예: `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `copy(x, y, tm, u, v, w, h)`<br>
타일 맵 `tm`(0-7)의 (`u`, `v`)부터 (`w`, `h`)까지의 영역을 (`x`, `y`) 좌표에 복사합니다.

### 사운드 클래스

- `note`<br>
음정 (0-127) 리스트 (33 = 'A2' = 440Hz)

- `tone`<br>
음색 리스트 (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volume`<br>
음량 리스트 (0-7)

- `effect`<br>
이펙트 리스트 (0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`<br>
1 노트의 길이 (120 = 음색(Tone) 당 1초)

- `set(note, tone, volume, effect, speed)`<br>
문자열을 사용해 음정, 음색, 음량, 이펙트를 설정합니다. 음색, 음량, 이펙트의 길이가 음정보다 짧으면 처음부터 반복합니다.

- `set_note(note)`<br>
'CDEFGAB'+'#-'+'0123' 또는 'R'의 문자열을 사용해 음정을 설정합니다. (대소문자 구별 없음/공백 무시)<br>
예: `pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tone(tone)`<br>
'TSPN' 문자열을 사용해 음색을 설정합니다. (대소문자 구별 없음/공백 무시)<br>
예: `pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volume(volume)`<br>
'01234567' 문자열을 사용해 음량을 설정합니다. (대소문자 구별 없음/공백 무시)<br>
예: `pyxel.sound(0).set_volume("7777 7531")`

- `set_effect(effect)`<br>
'NSVF' 문자열을 사용해 이펙트를 설정합니다. (대소문자 구별 없음/공백 무시)<br>
예: `pyxel.sound(0).set_effect("NFNF NVVS")`

### 음악 클래스

- `ch0`<br>
채널 0에서 재생하는 사운드(0-63) 리스트. 빈 리스트를 지정하면 사운드 재생 시 이 채널을 사용하지 않습니다.

- `ch1`<br>
채널 1에서 재생하는 사운드(0-63) 리스트. 빈 리스트를 지정하면 사운드 재생 시 이 채널을 사용하지 않습니다.

- `ch2`<br>
채널 2에서 재생하는 사운드(0-63) 리스트. 빈 리스트를 지정하면 사운드 재생 시 이 채널을 사용하지 않습니다.

- `ch3`<br>
채널 3에서 재생하는 사운드(0-63) 리스트. 빈 리스트를 지정하면 사운드 재생 시 이 채널을 사용하지 않습니다.

- `set(ch0, ch1, ch2, ch3)`<br>
모든 채널에서 재생하는 사운드(0-63)의 리스트를 지정합니다. 빈 리스트를 지정하면 사운드 재생 시 해당 채널을 사용하지 않습니다.<br>
예: `pyxel.music(0).set([0, 1], [2, 3], [4], [])`

- `set_ch0(data)`<br>
채널 0의 사운드(0-63) 리스트를 지정합니다.

- `set_ch1(data)`<br>
채널 1의 사운드(0-63) 리스트를 지정합니다.

- `set_ch2(data)`<br>
채널 2의 사운드(0-63) 리스트를 지정합니다.

- `set_ch3(data)`<br>
채널 3의 사운드(0-63) 리스트를 지정합니다.

## 컨트리뷰션 방법

### 문제 보고

오류 제보나 기능 건의는 [Issue Tracker](https://github.com/kitao/pyxel/issues)에서 받고 있습니다.
새 이슈를 작성하기 전에 비슷한 내용의 이슈가 없는지 확인 부탁드립니다.

새로운 리포트를 작성할 때는, [여기](https://github.com/kitao/pyxel/issues/new/choose)에서 내용에 맞는 템플릿을 선택해 주세요.

### 매뉴얼 테스트

코드를 테스트 해주시고, [Issue Tracker](https://github.com/kitao/pyxel/issues) 페이지에서 오류 제보나 개선 제안을 해주시는 분은 대환영입니다!

### 풀 리퀘스트 (Pull Request)

패치나 수정 요청은 풀 리퀘스트(PR)로 받고 있습니다. 제출하기 전에 문제가 이미 해결되지 않았는지 [Issue Tracker](https://github.com/kitao/pyxel/issues) 페이지에서 확인 부탁드립니다.

제출한 풀 리퀘스트는 [MIT license](https://github.com/kitao/pyxel/blob/master/LICENSE)에 따라 게시하기를 동의한 것으로 간주됩니다.

## 기타 정보

- [Wiki](https://github.com/kitao/pyxel/wiki)
- [Subreddit](https://www.reddit.com/r/pyxel/)
- [Discord server (English)](https://discord.gg/FC7kUZJ)
- [Discord server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## 라이선스 정보

Pyxel은 [MIT license](http://en.wikipedia.org/wiki/MIT_License)를 따릅니다. 라이선스가 부여된 소프트웨어의 모든 사본에 MIT 라이선스 조항의 사본 및 저작권 통지가 포함되어 있다면 독점 소프트웨어 내에서 재사용할 수 있습니다.

Pyxel은 아래와 같은 라이브러리를 사용하고 있습니다:

- [SDL2](https://www.libsdl.org/)
- [miniz-cpp](https://github.com/tfussell/miniz-cpp)
- [PyInstaller](https://www.pyinstaller.org/)
