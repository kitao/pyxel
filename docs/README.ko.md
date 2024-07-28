# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel**은 Python을 위한 레트로 게임 엔진입니다.

16가지 색상만 사용하거나 동시에 4가지 소리만 재생하는 등 레트로 게임에 나올 법한 사양으로, Python에서 픽셀 아트 스타일의 게임을 마음껏 만들 수 있습니다.

<img src="images/pyxel_message.png" width="480">

Pyxel 개발의 동기는 사용자의 피드백입니다. GitHub에서 Pyxel에 별점을 주세요!

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

Pyxel의 사양 및 API는 [PICO-8](https://www.lexaloffle.com/pico-8.php) 및 [TIC-80](https://tic80.com/) 에서 영감을 받았습니다.

Pyxel은 오픈 소스로, 무료로 자유롭게 사용할 수 있습니다. Pyxel과 함께 레트로 스타일의 게임을 만들어보세요!

## 사양

- Windows, Mac, Linux, Web에서 작동
- Python으로 프로그래밍
- 16색 팔레트
- 256x256 크기의 이미지 뱅크 3개
- 256x256 크기의 타일 맵 8개
- 4개의 사운드 채널 및 64개의 정의 가능한 사운드
- 임의의 사운드를 조합 가능한 8개의 음악
- 키보드, 마우스, 게임패드 입력
- 이미지/사운드 에디터

### 컬러 팔레트

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## 설치 방법

### Windows

[Python3](https://www.python.org/) (버전 3.7 이상) 을 설치한 후, 다음 명령어를 실행합니다.

```sh
pip install -U pyxel
```

공식 설치 프로그램을 사용하여 Python을 설치하는 경우 `pyxel` 명령을 활성화하려면 `Add Python 3.x to PATH` 확인란을 선택하십시오.

### Mac

[Homebrew](https://brew.sh/) 를 설치한 후 다음 명령을 실행합니다:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

파이셀을 설치한 후 버전을 업데이트하려면 `pipx upgrade pyxel`을 실행합니다.

### Linux

SDL2 패키지 (Ubuntu의 경우 `libsdl2-dev`), [Python3](https://www.python.org/) (버전 3.7 이상), `python3-pip`를 설치한 후, 다음 명령어를 실행합니다.

```sh
sudo pip3 install -U pyxel
```

위의 방법이 작동하지 않으면 [Makefile](../Makefile) 의 지침에 따라 자체 빌드를 시도하십시오.

### Web

Pyxel의 웹 버전은 Python 또는 Pyxel 설치가 필요하지 않으며 지원되는 웹 브라우저가 있는 PC는 물론 스마트폰 및 태블릿에서 실행됩니다.

구체적인 지침은 [이 페이지](https://github.com/kitao/pyxel/wiki/How-To-Use-Pyxel-Web) 를 참조하십시오.

### 예제 실행하기

Pyxel 설치 후, 다음 명령어를 사용해 현재 폴더에 Pyxel 예제 파일을 복사할 수 있습니다.

```sh
pyxel copy_examples
```

복사되는 예제 파일은 다음과 같습니다:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>간단한 애플리케이션</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Code</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Pyxel 리소스 파일을 사용한 점프 게임</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Code</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Drawing API 데모</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Code</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Sound API 데모</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Code</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>색상 팔레트 목록</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Code</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>마우스 클릭 게임</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Code</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>BGM이 포함된 스네이크 게임</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Code</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Triangle drawing API 데모</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Code</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>화면 전환이 있는 슈팅 게임</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Code</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>맵이 있는 횡 스크롤 플랫폼 게임</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Code</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Image 클래스를 사용한 오프스크린 렌더링</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Code</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>펄린 노이즈 애니메이션</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Code</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>비트맵 글꼴 그리기</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Code</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>오디오 익스팬션 기능을 활용한 신디사이저</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Code</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>타일 맵 파일 (.tmx) 로드 및 그리기</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Code</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>flip 기능이 있는 애니메이션 (웹 플랫폼이 아닌 경우에만 해당)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Code</a></td>
</tr>
<tr>
<td>30SecondsOfDaylight.pyxapp</td>
<td>제 1회 Pyxel Jam 우승 작품 (<a href="https://twitter.com/helpcomputer0">Adam</a> 제작)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30SecondsOfDaylight.html">Demo</a></td>
<td><a href="https://github.com/kitao/30SecondsOfDaylight">Code</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>아케이드 볼 물리 게임 (<a href="https://twitter.com/helpcomputer0">Adam</a> 제작)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Demo</a></td>
<td><a href="https://github.com/helpcomputer/megaball">Code</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>배경 음악 생성기 (<a href="https://twitter.com/frenchbread1222">frenchbread</a> 제작)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">Demo</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">Code</a></td>
</tr>
</table>

다음 명령어를 사용하여 예제 파일을 실행할 수 있습니다.

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30SecondsOfDaylight.pyxapp
```

## 사용 방법

### Pyxel 애플리케이션 작성 방법

Python 스크립트에서 Pyxel 모듈을 가져온 뒤 `init` 함수로 화면 크기를 지정한 후, `run` 함수로 Pyxel 애플리케이션을 시작합니다.

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

`run` 함수의 인자로는 프레임 갱신을 처리하는 `update` 함수와 필요할 때 화면을 그리는 `draw` 함수가 사용됩니다.

실제 애플리케이션에서는 아래와 같이 클래스에서 Pyxel 코드를 감싸는 것이 좋습니다.

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

애니메이션 없이 간단한 그래픽을 만들 때 `show` 기능을 사용하면 코드를 더 간결하게 만들 수 있습니다.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Pyxel 애플리케이션 실행 방법

생성된 Python 스크립트는 다음 명령어를 사용해 실행할 수 있습니다.

```sh
pyxel run PYTHON_SCRIPT_FILE
```

일반 Python 스크립트처럼 실행할 수도 있습니다.

```sh
python3 PYTHON_SCRIPT_FILE
```

### 특수 조작

Pyxel 애플리케이션 실행 중에, 아래의 특수 조작을 사용할 수 있습니다.

- `Esc`<br>
  애플리케이션 종료
- `Alt(Option)+1`<br>
  바탕 화면에 스크린샷 저장
- `Alt(Option)+2`<br>
  화면 캡처의 녹화 시작 시간 초기화
- `Alt(Option)+3`<br>
  화면 캡처 파일을 바탕 화면에 저장 (최대 10초)
- `Alt(Option)+9`<br>
  화면 모드 (Crisp/Smooth/Retro) 전환하기
- `Alt(Option)+0`<br>
  성능 모니터 (fps, update time, draw time) 의 표시/표시 해제
- `Alt(Option)+Enter`<br>
  전체 화면 전환
- `Shift+Alt(Option)+1/2/3`<br>
  해당 이미지 뱅크를 데스크톱에 저장합니다
- `Shift+Alt(Option)+0`<br>
  현재 색상표를 바탕화면에 저장

### 리소스의 작성 방법

Pyxel Editor는 Pyxel 애플리케이션에서 사용되는 이미지와 사운드를 생성할 수 있습니다.

다음 명령어를 사용해 시작할 수 있습니다.

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

지정한 Pyxel 리소스 파일 (.pyxres) 이 존재하는 경우에는 해당 파일을 불러오고, 존재하지 않는 경우 지정한 이름으로 새 리소스 파일을 생성합니다. 파일 이름을 생략했을 경우, 기본 파일 이름은 `my_resource.pyxres`입니다.

Pyxel Editor 실행 중 다른 리소스 파일을 드래그 앤 드롭하는 것으로 작업 중인 리소스 파일을 변경할 수 있습니다.

만들어진 리소스 파일은 `load` 함수를 통해 불러올 수 있습니다.

Pyxel Editor에는 다음과 같은 편집 모드가 있습니다.

**이미지 편집기**

이미지 뱅크의 이미지를 편집하는 화면입니다.

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

이미지 파일 (PNG/GIF/JPEG) 을 이미지 편집기로 끌어다 놓아 현재 선택한 이미지 뱅크에 이미지를 로드합니다.

**타일 맵 편집기**

이미지 뱅크의 이미지를 타일 모양으로 늘어놓은 타일 맵을 편집하는 화면입니다.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

TMX 파일 (Tiled Map File) 을 타일맵 편집기로 끌어다 놓으면 현재 선택한 타일맵 번호에 해당하는 도면 순서대로 해당 레이어를 로드할 수 있습니다.

**사운드 편집기**

사운드를 편집하는 화면입니다.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**음악 편집기**

사운드를 플레이 순서대로 늘어놓은 음악을 편집하는 화면입니다.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### 기타 리소스 작성 방법

Pyxel의 이미지와 타일 맵은 다음과 같은 방법으로 만들 수도 있습니다.

- `Image.set` 또는 `Tilemap.set` 함수를 사용하여 문자열 리스트에서 이미지 생성
- `Image.load` 함수를 사용하여 Pyxel 팔레트 이미지 파일 (PNG/GIF/JPEG) 불러오기

Pyxel의 사운드와 음악도 다음과 같은 방법으로 만들 수 있습니다.

- `Sound.set` 또는 `Music.set` 함수를 사용하여 문자열에서 사운드 생성

각 함수의 사용법은 API 레퍼런스를 참조해주세요.

### 애플리케이션 배포 방법

Pyxel은 여러 플랫폼에서 작동하는 전용 애플리케이션 배포 파일 형식 (Pyxel 애플리케이션 파일) 을 지원합니다.

Pyxel 애플리케이션 파일 (.pyxapp) 은 다음 명령어를 사용해 생성할 수 있습니다.

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

응용 프로그램에 리소스 또는 추가 모듈이 포함되어야 하는 경우 응용 프로그램 디렉터리에 배치합니다.

생성된 애플리케이션 파일은 다음 명령어를 사용하여 실행할 수 있습니다.

```sh
pyxel play PYXEL_APP_FILE
```

Pyxel 응용 프로그램 파일은 `pyxel app2exe` 또는 `pyxel app2html` 명령을 사용하여 실행 파일 또는 HTML 파일로 변환할 수도 있습니다.

## API 레퍼런스

### 시스템

- `width`, `height`<br>
  화면의 가로/세로 크기

- `frame_count`<br>
  경과한 프레임의 수

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Pyxel 애플리케이션을 화면 크기 (`width`, `height`) 로 초기화합니다. 옵션으로 `title`에 창 제목, `fps`에 프레임 속도, `quit_key`에 애플리케이션 종료 키, `display_scale`을 사용한 디스플레이 크기, `capture_scale`에 화면 캡처의 배율, `capture_sec`에 화면 캡처의 최대 녹화 시간을 지정할 수 있습니다.<br>
  예시: `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Pyxel 애플리케이션을 실행하며, 프레임 갱신 시 `update` 함수를, 화면 그리기에 `draw` 함수를 호출합니다.

- `show()`<br>
  화면을 표시하고 `Esc` 키를 누를 때까지 기다립니다.

- `flip()`<br>
  화면을 한 프레임씩 새로 고칩니다. `Esc` 키를 누르면 애플리케이션이 종료됩니다. 이 기능은 웹 버전에서는 작동하지 않습니다.

- `quit()`<br>
  Pyxel 애플리케이션을 종료합니다.

### 리소스

- `load(filename, [excl_images], [excl_tilemaps], [excl_sounds], [excl_musics])`<br>
  리소스 파일 (.pyxres) 을 불러옵니다. 옵션이 `True`이면 리소스가 로드되지 않습니다. 리소스 파일과 같은 위치에 같은 이름의 팔레트 파일 (.pyxpal) 이 있는 경우 팔레트 표시 색상도 변경됩니다. 색상표 파일은 표시 색상의 16진수 항목 (예: `1100FF`) 을 개행으로 구분한 것입니다. 팔레트 파일은 파이셀 편집기에서 표시되는 색상을 변경하는 데에도 사용할 수 있습니다.

### 입력

- `mouse_x`, `mouse_y`<br>
  마우스 커서의 현재 좌표를 나타냅니다.

- `mouse_wheel`<br>
  마우스 휠의 현재 값을 나타냅니다.

- `btn(key)`<br>
  `key`가 눌리고 있으면 `True`, 눌리고 있지 않으면 `False`를 반환합니다. ([키 정의 리스트](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  해당 프레임에 `key`가 눌리면 `True`, 눌리지 않으면 `False`를 반환합니다. `hold`와 `repeat`를 지정하면, `hold` 프레임 이상 `key`가 눌린 상태인 경우 `repeat` 프레임 간격으로 `True`를 반환합니다.

- `btnr(key)`<br>
  해당 프레임에 `key`가 떼어지면 `True`, 아니면 `False`를 반환합니다.

- `mouse(visible)`<br>
  `visible`이 `True`인 경우 마우스 커서를 표시하고, `False`라면 표시하지 않습니다. 마우스 커서가 보이지 않아도 마우스 커서의 좌표는 갱신됩니다.

### 그래픽

- `colors`<br>
  팔레트의 표시 색상 리스트입니다. 표시 색상은 24-bit 숫자 값으로 지정합니다. `colors.from_list` 및 `colors.to_list`를 사용해 Python 리스트의 형태로 직접 색상을 지정하고 적용할 수 있습니다.<br>
  예시: `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  이미지 뱅크 목록 (0-2). (이미지 클래스 참조)<br>
  예시: `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  타일맵 목록 (0-7). (타일 맵 클래스 참조)

- `clip(x, y, w, h)`<br>
  화면의 드로잉 영역을 (`x`, `y`) 로 설정하고, 너비를 `w`, 높이를 `h`로 설정합니다. `clip()`과 같이 사용하면 드로잉 영역을 초기 상태 (전체 화면) 으로 돌립니다.

- `camera(x, y)`<br>
  화면의 좌측 상단 좌표를 (`x`, `y`) 로 변경합니다. `camera()`로 좌표를 (`0`, `0`) 으로 초기화할 수 있습니다.

- `pal(col1, col2)`<br>
  드로잉 시 `col1`를 `col2`로 대체합니다. `pal()`과 같이 사용하면 초기 상태로 돌립니다.

- `dither(alpha)`<br>
  그리기 시 디더링 (의사 투명도) 을 적용합니다. 0.0-1.0 범위에서 `알파`를 설정합니다 (0.0은 투명, 1.0은 불투명).

- `cls(col)`<br>
  화면을 `col` 색으로 지웁니다.

- `pget(x, y)`<br>
  (`x`, `y`) 좌표의 색상 값을 가져옵니다.

- `pset(x, y, col)`<br>
  `col` 색을 사용해 (`x`, `y`) 좌표에 픽셀을 그립니다.

- `line(x1, y1, x2, y2, col)`<br>
  `col` 색을 사용해 (`x1`, `y1`) 부터 (`x2`, `y2`) 까지 직선을 그립니다.

- `rect(x, y, w, h, col)`<br>
  가로 `w`, 세로 `h`의 크기로 `col` 색을 사용해 직사각형을 (`x`, `y`) 좌표에 그립니다.

- `rectb(x, y, w, h, col)`<br>
  가로 `w`, 세로 `h`의 크기로 `col` 색을 사용해 직사각형 테두리를 (`x`, `y`) 좌표에 그립니다. (테두리 안쪽에 색상을 채우지 않음)

- `circ(x, y, r, col)`<br>
  반경 `r`, `col` 색의 원을 (`x`, `y`) 좌표에 그립니다.

- `circb(x, y, r, col)`<br>
  반경 `r`, `col` 색의 원 테두리를 (`x`, `y`) 좌표에 그립니다. (테두리 안쪽에 색상을 채우지 않음)

- `elli(x, y, w, h, col)`<br>
  (`x`, `y`) 에서 너비 `w`, 높이 `h` 및 색상 `col`의 타원을 그립니다.

- `ellib(x, y, w, h, col)`<br>
  (`x`, `y`) 에서 너비 `w`, 높이 `h` 및 색상 `col`의 타원 윤곽선을 그립니다.

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  꼭짓점 좌표 (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) 를 기준으로 `col` 색상의 삼각형을 그립니다.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  꼭짓점 좌표 (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) 를 기준으로 `col` 색상의 삼각형 테두리를 그립니다. (테두리 안쪽에 색상을 채우지 않음)

- `fill(x, y, col)`<br>
  (`x`, `y`) 에서 너비 `w`, 높이 `h` 및 색상 `col`의 줄임표를 그립니다.

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
  이미지 뱅크 `img`(0-2) 의 (`u`, `v`) 로부터 크기 (`w`, `h`) 까지의 영역을 (`x`, `y`) 좌표에 복사합니다. `w`, `h`의 값을 음수로 설정하면, 각각 수평, 수직 방향으로 반전됩니다. `colkey`로 색을 지정하면 투명 색상으로 처리됩니다.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
  타일 맵 `tm`(0-7) 의 (`u`, `v`) 로부터 크기 (`w`, `h`) 까지의 영역을 (`x`, `y`) 좌표에 복사합니다. `w`, `h`의 값을 음수로 설정하면, 각각 수평, 수직 방향으로 반전됩니다. `colkey`로 색을 지정하면 투명 색상으로 처리됩니다. 타일 하나의 크기는 8x8 픽셀이며 `(tile_x, tile_y)`의 튜플로 타일 맵에 저장되어 있습니다.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  `col` 색을 사용해 문자열 `s`를 (`x`, `y`) 좌표에 그립니다.

### 오디오

- `sounds`<br>
  소리 목록 (0-63). (사운드 클래스 참조)<br>
  예시: `pyxel.sounds[0].speed = 60`

- `musics`<br>
  음악 목록 (0-7). (음악 클래스 참조)

- `play(ch, snd, [tick], [loop], [resume])`<br>
  채널 `ch`(0-3) 에서 사운드 `snd`(0-63) 를 재생합니다. `snd`가 리스트일 경우, 순서대로 재생됩니다. 재생 시작 위치는 `tick`(1 tick = 1/120초) 으로 지정할 수 있습니다. `loop`에 `True`를 지정하면 계속 반복합니다. 재생이 끝난 후 이전 소리를 다시 재생하려면 `resume`을 `True`로 설정하세요.

- `playm(msc, [tick], [loop])`<br>
  음악 `msc`(0-7) 을 재생합니다. 재생 시작 위치는 `tick`(1 tick = 1/120초) 으로 지정할 수 있습니다. `loop`에 `True`를 지정하면 계속 반복합니다.

- `stop([ch])`<br>
  지정된 채널 `ch`(0-3) 의 재생을 중지합니다. `stop()`을 사용해 모든 채널의 재생을 중지할 수도 있습니다.

- `play_pos(ch)`<br>
  채널 `ch`(0-3) 의 사운드 재생 위치를 `(sound no, note no)` 형태의 튜플로 가져옵니다. 재생 중이 아닐 경우 `None`을 반환합니다.

### 수학

- `ceil(x)`<br>
  `x`보다 크거나 같은 가장 작은 정수를 반환합니다.

- `floor(x)`<br>
  `x`보다 작거나 같은 가장 큰 정수를 반환합니다.

- `sgn(x)`<br>
  x가 양수이면 1, 0이면 0, 음수이면 -1을 반환합니다.

- `sqrt(x)`<br>
  `x`의 제곱근을 반환합니다.

- `sin(deg)`<br>
  `deg` 각도의 사인을 반환합니다.

- `cos(deg)`<br>
  `deg` 각도의 코사인을 반환합니다.

- `atan2(y, x)`<br>
  `y`/`x`의 아크탄젠트를 도 단위로 반환합니다.

- `rseed(seed)`<br>
  난수 생성기의 시드를 설정합니다.

- `rndi(a, b)`<br>
  `a`보다 크거나 같고 `b`보다 작거나 같은 임의의 정수를 반환합니다.

- `rndf(a, b)`<br>
  `a`보다 크거나 같고 `b`보다 작거나 같은 임의의 소수를 반환합니다.

- `nseed(seed)`<br>
  Perlin 노이즈의 시드를 설정합니다.

- `noise(x, [y], [z])`<br>
  지정된 좌표에 대한 Perlin 노이즈 값을 반환합니다.

### 이미지 클래스

- `width`, `height`<br>
  이미지의 가로, 세로 크기

- `set(x, y, data)`<br>
  (`x`, `y`) 에 문자열 리스트를 사용해 이미지를 설정합니다.<br>
  예시: `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  (`x`, `y`) 에 이미지 파일 (PNG/GIF/JPEG) 을 불러옵니다.

- `pget(x, y)`<br>
  (`x`, `y`) 에서 픽셀 색상을 가져옵니다.

- `pset(x, y, col)`<br>
  (`x`, `y`) 에 색상 `col`의 픽셀을 그립니다.

### 타일 맵 클래스

- `width`, `height`<br>
  타일 맵의 가로/세로 크기

- `imgsrc`<br>
  타일 맵이 참조하는 이미지 뱅크 (0-2)

- `set(x, y, data)`<br>
  (`x`, `y`) 에 문자열 리스트를 사용해 타일 맵을 설정합니다.<br>
  예시: `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  TMX 파일 (Tiled Map File) 에서 (`x`, `y`) 의 도면 순서 `layer`(0-) 로 레이어를 로드합니다.

- `pget(x, y)`<br>
  (`x`, `y`) 에서 타일을 가져옵니다. 타일은 `(tile_x, tile_y)`의 튜플 형태입니다.

- `pset(x, y, tile)`<br>
  (`x`, `y`) 에 `tile`을 그립니다. 타일은 `(tile_x, tile_y)`의 튜플 형태입니다.

### 사운드 클래스

- `notes`<br>
  음정 리스트 (0-127). 숫자가 높을수록 피치가 높아지며 33에서는 'A2'(440Hz) 가 됩니다. 쉼표는 -1입니다.

- `tones`<br>
  음색 리스트 (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
  음량 리스트 (0-7)

- `effects`<br>
  효과 리스트 (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  재생 속도. 1이 가장 빠르며 숫자가 커질수록 재생 속도가 느려집니다. 120에서는 한 음의 길이가 1초가 됩니다.

- `set(notes, tones, volumes, effects, speed)`<br>
  문자열을 사용해 음정, 음색, 음량 및 효과를 설정합니다. 음색, 음량, 효과의 길이가 음정보다 짧으면 처음부터 반복됩니다.

- `set_notes(notes)`<br>
  'CDEFGAB'+'#-'+'01234' 또는 'R' 문자열로 음정을 설정합니다. 대소문자를 구분하지 않으며 빈칸은 무시됩니다.<br>
  예시: `pyxel.sounds[0].set_notes("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
  'TSPN' 문자열로 음색을 설정합니다. 대소문자를 구분하지 않으며 빈칸은 무시됩니다.<br>
  예시: `pyxel.sounds[0].set_tones("TTSS PPPN")`

- `set_volumes(volumes)`<br>
  '01234567' 문자열로 음량을 설정합니다. 대소문자를 구분하지 않으며 빈칸은 무시됩니다.<br>
  예시: `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  'NSVFHQ' 문자열로 효과를 설정합니다. 대소문자를 구분하지 않으며 빈칸은 무시됩니다.<br>
  예시: `pyxel.sounds[0].set_effects("NFNF NVVS")`

### 음악 클래스

- `seqs`<br>
  채널 수가 포함된 2차원 사운드 목록 (0-63)

- `set(seq0, seq1, seq2, ...)`<br>
  채널의 사운드 목록 (0-63) 을 설정합니다. 빈 리스트가 지정되면 해당 채널은 재생에 사용되지 않습니다.<br>
  예시: `pyxel.musics[0].set([0, 1], [], [3])`

### 고급 사용자용 API

Pyxel에는 "사용자를 혼란스럽게 할 수 있거나", "사용하는 데 전문 지식이 필요함"의 이유로 여기에 언급되지 않은 "고급 사용자용 API"가 존재합니다.

이러한 것을 다루는 데 능숙하시다면, [여기](../python/pyxel/__init__.pyi) 를 참고해 깜짝 놀랄 만한 작품 만들기에 도전해보세요!

## 기여 방법

### 문제 보고

오류 제보나 기능 건의는 [이슈 트래커](https://github.com/kitao/pyxel/issues) 에서 받고 있습니다. 새 이슈를 작성하기 전에 비슷한 내용의 이슈가 없는지 확인 부탁드립니다.

### 매뉴얼 테스팅

코드를 테스트해 주시고, [이슈 트래커](https://github.com/kitao/pyxel/issues) 페이지에서 오류 제보나 개선 제안을 해주시는 분들을 환영합니다!

### 풀 리퀘스트 제출

패치나 수정 요청은 풀 리퀘스트 (PR) 로 받고 있습니다. 제출하기 전에 문제가 이미 해결되지 않았는지 이슈 트래커 페이지에서 확인 부탁드립니다.

제출한 풀 리퀘스트는 [MIT 라이선스](../LICENSE) 에 따라 게시하는 데 동의한 것으로 간주합니다.

## 기타 정보

- [Q&A](https://github.com/kitao/pyxel/wiki/Pyxel-Q&A)
- [User Examples](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Developer's Twitter account](https://twitter.com/kitao)

## 라이선스 정보

Pyxel은 [MIT 라이선스](../LICENSE) 를 따릅니다. 소프트웨어의 모든 사본 또는 그 상당 부분에 MIT 라이선스 조항의 사본 및 저작권 통지가 포함되어 있다면 독점 소프트웨어 내에서 재사용할 수 있습니다.

## 스폰서 모집

Pyxel은 GitHub Sponsors에서 스폰서를 모집하고 있습니다. Pyxel의 유지 보수 및 기능 추가를 위해 스폰서가 되는 것을 고려해보세요. 스폰서가 되면 혜택으로 Pyxel에 대한 상담을 받을 수 있습니다. 자세한 내용은 [여기](https://github.com/sponsors/kitao) 를 참조하세요.
