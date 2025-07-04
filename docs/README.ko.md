# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel**은 Python을 위한 복고풍 게임 엔진입니다.

사양은 복고풍 게임 콘솔에서 영감을 받아 16색만 지원하고 4개의 사운드 채널을 사용할 수 있어 픽셀 아트 스타일의 게임 제작을 쉽게 즐길 수 있습니다.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

Pyxel 개발은 사용자 피드백에 의해 이루어집니다. GitHub에서 Pyxel에 별을 주세요!

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

Pyxel의 사양 및 API는 [PICO-8](https://www.lexaloffle.com/pico-8.php)와 [TIC-80](https://tic80.com/)에서 영감을 받았습니다.

Pyxel은 [MIT 라이센스](../LICENSE) 하에 오픈 소스이며 무료로 사용할 수 있습니다. Pyxel로 레트로 게임을 만들어 봅시다!

## 사양

- Windows, Mac, Linux 및 Web에서 실행
- Python으로 프로그래밍
- 사용자 정의 화면 크기
- 16색 팔레트
- 3개의 256x256 크기 이미지 뱅크
- 8개의 256x256 크기 타일 맵
- 64개의 정의 가능한 사운드와 4개의 채널
- 결합 가능한 8개의 음악 트랙
- 키보드, 마우스 및 게임패드 입력
- 이미지 및 사운드 편집 도구
- 사용자 확장 가능한 색상, 채널 및 뱅크

### 색상 팔레트

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## 설치 방법

### Windows

[Python3](https://www.python.org/) (버전 3.8 이상)을 설치한 후, 다음 명령어를 실행하세요:

```sh
pip install -U pyxel
```

공식 설치 프로그램을 사용하여 Python을 설치할 때 `Add Python 3.x to PATH` 옵션을 체크하여 `pyxel` 명령을 활성화하세요.

### Mac

[Homebrew](https://brew.sh/)를 설치한 후, 다음 명령어를 실행하세요:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

설치 후 Pyxel을 업그레이드하려면 `pipx upgrade pyxel`을 실행하세요.

### Linux

SDL2 패키지(`libsdl2-dev`는 Ubuntu의 경우), [Python3](https://www.python.org/) (버전 3.8 이상), `python3-pip`를 설치한 후, 다음 명령어를 실행하세요:

```sh
sudo pip3 install -U pyxel
```

이전 명령이 작동하지 않으면 [Makefile](../Makefile)에 있는 지침에 따라 소스에서 Pyxel을 빌드하는 것을 고려하세요.

### Web

Pyxel의 웹 버전은 Python이나 Pyxel 설치가 필요 없으며, 지원되는 웹 브라우저를 사용하는 PC, 스마트폰 및 태블릿에서 실행됩니다.

자세한 지침은 [이 페이지](pyxel-web-en.md)를 참조하세요.

### 예제 실행하기

Pyxel을 설치한 후, 다음 명령어로 예제를 현재 디렉토리로 복사할 수 있습니다:

```sh
pyxel copy_examples
```

다음 예제가 현재 디렉토리에 복사됩니다:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>가장 간단한 애플리케이션</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">코드</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Pyxel 리소스 파일을 이용한 점프 게임</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">코드</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>그리기 API의 시연</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">코드</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>사운드 API의 시연</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">코드</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>색상 팔레트 목록</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">코드</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>마우스 클릭 게임</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">코드</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>BGM이 있는 스네이크 게임</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">코드</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>삼각형 그리기 API의 시연</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">코드</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Shoot'em up 게임과 화면 전환 및 MML</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">코드</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>맵이 있는 횡스크롤 플랫폼 게임</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">코드</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Image 클래스를 이용한 오프스크린 렌더링</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">코드</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>퍼린 노이즈 애니메이션</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">코드</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>비트맵 폰트 그리기</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">코드</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>오디오 확장 기능을 활용한 신시사이저</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">코드</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>Tiled Map File (.tmx) 로드 및 그리기</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">코드</a></td>
</tr>
<tr>
<td>16_transform.py</td>
<td>이미지 회전 및 크기 조정</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/16_transform.html">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/16_transform.py">코드</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>flip 함수로 애니메이션 (비 웹 플랫폼 전용)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">데모</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">코드</a></td>
</tr>
<tr>
<td>30sec_of_daylight.pyxapp</td>
<td>1회 Pyxel Jam 우승 게임 (<a href="https://x.com/helpcomputer0">Adam</a> 제작)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30sec_of_daylight.html">데모</a></td>
<td><a href="https://github.com/kitao/30SecondsOfDaylight">코드</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>아케이드 볼 물리 게임 (<a href="https://x.com/helpcomputer0">Adam</a> 제작)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">데모</a></td>
<td><a href="https://github.com/kitao/megaball">코드</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>BGM 자동 생성 도구 (<a href="https://x.com/frenchbread1222">frenchbread</a> 제작)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">데모</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">코드</a></td>
</tr>
</table>

예제는 다음 명령어로 실행할 수 있습니다:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30sec_of_daylight.pyxapp
```

## 사용 방법

### 애플리케이션 만들기

Python 스크립트에서 Pyxel 모듈을 가져오고, `init` 함수로 창 크기를 지정한 후, `run` 함수로 Pyxel 애플리케이션을 시작합니다.

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

`run` 함수의 인자는 프레임 업데이트를 처리하는 `update` 함수와 화면 그리기를 처리하는 `draw` 함수입니다.

실제 애플리케이션에서는 Pyxel 코드를 클래스에 감싸는 것이 좋습니다. 아래와 같이 작성할 수 있습니다:

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

애니메이션 없는 간단한 그래픽을 만들고 싶다면 `show` 함수를 사용하여 코드를 간단하게 작성할 수 있습니다.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### 애플리케이션 실행하기

작성한 스크립트는 `python` 명령어로 실행할 수 있습니다:

```sh
python PYTHON_SCRIPT_FILE
```

`pyxel run` 명령어로도 실행할 수 있습니다:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

또한, `pyxel watch` 명령어를 사용하면 지정한 디렉터리의 변경 사항을 모니터링하고, 변경 사항이 감지되면 프로그램을 자동으로 재실행합니다:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

디렉터리 모니터링은 `Ctrl(Command)+C`를 눌러 중지할 수 있습니다.

### 특수 키 조작

Pyxel 애플리케이션이 실행 중일 때, 다음의 특수 키 조작을 수행할 수 있습니다:

- `Esc`<br>
  애플리케이션 종료
- `Alt(Option)+1`<br>
  화면 캡처를 데스크탑에 저장
- `Alt(Option)+2`<br>
  화면 캡처 비디오의 시작 시간을 재설정
- `Alt(Option)+3`<br>
  화면 캡처 비디오를 데스크탑에 저장 (최대 10초)
- `Alt(Option)+8` 또는 게임패드에서 `A+B+X+Y+DL`<br>
  화면 확대 방법을 최대 및 정수 배율로 전환
- `Alt(Option)+9` 또는 게임패드에서 `A+B+X+Y+DR`<br>
  화면 모드 (Crisp/Smooth/Retro) 전환
- `Alt(Option)+0` 또는 게임패드에서 `A+B+X+Y+DU`<br>
  성능 모니터 (FPS/`update` 시간/`draw` 시간) 전환
- `Alt(Option)+Enter` 또는 게임패드에서 `A+B+X+Y+DD`<br>
  전체 화면 전환
- `Shift+Alt(Option)+1/2/3`<br>
  이미지 뱅크 0, 1 또는 2를 데스크탑에 저장
- `Shift+Alt(Option)+0`<br>
  현재 색상 팔레트를 데스크탑에 저장

### 리소스 생성 방법

Pyxel Editor를 사용하여 Pyxel 애플리케이션에서 사용할 이미지와 사운드를 생성할 수 있습니다.

다음 명령으로 Pyxel Editor를 시작할 수 있습니다:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

지정된 Pyxel 리소스 파일(.pyxres)이 존재하면 해당 파일이 로드됩니다. 존재하지 않는 경우 지정된 이름으로 새 파일이 생성됩니다. 리소스 파일이 생략된 경우 `my_resource.pyxres`라는 새 파일이 생성됩니다.

Pyxel Editor를 시작한 후 다른 리소스 파일로 전환하려면 해당 파일을 Pyxel Editor로 드래그 앤 드롭하면 됩니다.

생성된 리소스 파일은 `load` 함수를 사용하여 로드할 수 있습니다.

Pyxel Editor에는 다음과 같은 편집 모드가 있습니다.

**이미지 편집기**

각 **이미지 뱅크**의 이미지를 편집하는 모드입니다.

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

이미지 편집기로 PNG/GIF/JPEG 파일을 드래그 앤 드롭하면 현재 선택된 이미지 뱅크에 이미지를 로드합니다.

**타일맵 편집기**

이미지 뱅크의 이미지를 타일 패턴으로 배열하여 **타일맵**을 편집하는 모드입니다.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

TMX 파일(Tiled Map File)을 타일맵 편집기로 드래그 앤 드롭하면 현재 선택된 타일맵에 레이어 0이 로드됩니다.

**사운드 편집기**

멜로디와 **사운드** 효과에 사용되는 사운드를 편집하는 모드입니다.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**뮤직 편집기**

재생 순서에 따라 사운드를 배열한 **뮤직**을 편집하는 모드입니다.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### 기타 리소스 생성 방법

Pyxel 이미지와 타일맵은 다음 방법을 사용하여 생성할 수도 있습니다:

- `Image.set` 함수 또는 `Tilemap.set` 함수를 사용하여 문자열 목록에서 이미지를 생성합니다.
- `Image.load` 함수를 사용하여 Pyxel 팔레트의 이미지 파일(PNG/GIF/JPEG)을 로드합니다.

Pyxel 사운드는 다음 방법을 사용하여 생성할 수도 있습니다:

- `Sound.set` 함수 또는 `Music.set` 함수를 사용하여 문자열에서 사운드를 생성합니다.

이 함수의 사용법은 API 참조를 참조하십시오.

### 애플리케이션 배포 방법

Pyxel은 플랫폼에 관계없이 작동하는 전용 애플리케이션 배포 파일 형식(Pyxel 애플리케이션 파일)을 지원합니다.

Pyxel 애플리케이션 파일(.pyxapp)은 `pyxel package` 명령을 사용하여 생성됩니다:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

리소스나 추가 모듈을 포함해야 하는 경우, 애플리케이션 디렉토리에 배치하십시오.

메타데이터는 다음 형식으로 시작 스크립트 내에 지정하여 실행 중에 표시될 수 있습니다. `title`과 `author`를 제외한 필드는 선택 사항입니다.

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

생성된 애플리케이션 파일은 `pyxel play` 명령을 사용하여 실행할 수 있습니다:

```sh
pyxel play PYXEL_APP_FILE
```

Pyxel 애플리케이션 파일은 `pyxel app2exe` 또는 `pyxel app2html` 명령을 사용하여 실행 파일이나 HTML 파일로 변환할 수도 있습니다.

## API Reference

### System

- `width`, `height`<br>
  화면의 너비와 높이

- `frame_count`<br>
  경과한 프레임 수

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  화면 크기 (`width`, `height`)로 Pyxel 애플리케이션을 초기화합니다. 다음 옵션을 지정할 수 있습니다: `title`로 창 제목, `fps`로 프레임 속도, `quit_key`로 애플리케이션 종료 키, `display_scale`로 화면 표시 배율, `capture_scale`로 화면 캡처 배율, `capture_sec`로 화면 캡처 비디오의 최대 녹화 시간을 지정합니다.<br>
  예시: `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Pyxel 애플리케이션을 시작하고 프레임 업데이트를 위해 `update` 함수를, 화면 그리기를 위해 `draw` 함수를 호출합니다.

- `show()`<br>
  화면을 표시하고 `Esc` 키가 눌릴 때까지 대기합니다.

- `flip()`<br>
  화면을 한 프레임 갱신합니다. `Esc` 키가 눌리면 애플리케이션이 종료됩니다. 이 함수는 웹 버전에서는 사용할 수 없습니다.

- `quit()`<br>
  Pyxel 애플리케이션을 종료합니다.

### Resource

- `load(filename, [ignore_images], [ignore_tilemaps], [ignore_sounds], [ignore_musics])`<br>
  리소스 파일 (.pyxres)을 로드합니다. 옵션이 `True`로 설정되면 해당 리소스는 로드에서 제외됩니다. 동일한 이름의 팔레트 파일 (.pyxpal)이 리소스 파일과 동일한 위치에 존재하는 경우, 팔레트 표시 색상도 업데이트됩니다. 팔레트 파일은 각 색상을 16진수로 나타낸 값을 줄바꿈으로 구분하여 입력합니다 (예: `1100ff`). 팔레트 파일을 사용하면 Pyxel Editor에서 표시되는 색상도 변경할 수 있습니다.

- `user_data_dir(vendor_name, app_name)`<br>
  `vendor_name`와 `app_name`을 기반으로 생성된 사용자 데이터 디렉토리를 반환합니다. 디렉토리가 존재하지 않으면 자동으로 생성됩니다. 이 디렉토리는 하이스코어, 게임 진행 상황 등의 데이터를 저장하는 데 사용됩니다.<br>
  예시: `print(pyxel.user_data_dir("Takashi Kitao", "Pyxel Shooter"))`

### Input

- `mouse_x`, `mouse_y`<br>
  현재 마우스 커서의 좌표

- `mouse_wheel`<br>
  현재 마우스 휠 값

- `btn(key)`<br>
  `key`가 눌려있으면 `True`를 반환하고, 그렇지 않으면 `False`를 반환합니다. ([키 정의 목록](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  그 프레임에서 `key`가 눌렸으면 `True`를 반환하고, 그렇지 않으면 `False`를 반환합니다. `hold` 및 `repeat`가 지정되면, `key`가 `hold` 프레임 이상 눌려 있는 경우 `repeat` 프레임마다 `True`를 반환합니다.

- `btnr(key)`<br>
  그 프레임에서 `key`가 해제되었으면 `True`를 반환하고, 그렇지 않으면 `False`를 반환합니다.

- `mouse(visible)`<br>
  `visible`이 `True`면 마우스 커서를 표시하고, `False`면 숨깁니다. 커서가 숨겨져 있어도 위치는 계속 업데이트됩니다.

### Graphics

- `colors`<br>
  팔레트 표시 색상 목록. 표시 색상은 24비트 숫자로 지정됩니다. Python 리스트를 직접 할당하거나 가져오려면 `colors.from_list` 및 `colors.to_list`를 사용하십시오.<br>
  예시: `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  이미지 뱅크 (Image 클래스의 인스턴스) 목록 (0-2)<br>
  예시: `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  타일맵 (Tilemap 클래스의 인스턴스) 목록 (0-7)

- `clip(x, y, w, h)`<br>
  (`x`, `y`)부터 너비 `w`, 높이 `h`로 화면의 그리기 영역을 설정합니다. `clip()`을 호출하면 그리기 영역이 전체 화면으로 재설정됩니다.

- `camera(x, y)`<br>
  화면의 좌상단 좌표를 (`x`, `y`)로 변경합니다. `camera()`를 호출하면 좌상단 좌표가 (`0`, `0`)로 재설정됩니다.

- `pal(col1, col2)`<br>
  그릴 때 색상 `col1`을 `col2`로 대체합니다. `pal()`을 호출하면 초기 팔레트로 재설정됩니다.

- `dither(alpha)`<br>
  그릴 때 디더링(유사 반투명)을 적용합니다. `alpha`를 `0.0`-`1.0` 범위로 설정하며, `0.0`은 투명, `1.0`은 불투명을 나타냅니다.

- `cls(col)`<br>
  화면을 색상 `col`로 지웁니다.

- `pget(x, y)`<br>
  (`x`, `y`)의 픽셀 색상을 가져옵니다.

- `pset(x, y, col)`<br>
  (`x`, `y`)에 색상 `col`의 픽셀을 그립니다.

- `line(x1, y1, x2, y2, col)`<br>
  색상 `col`의 선을 (`x1`, `y1`)에서 (`x2`, `y2`)로 그립니다.

- `rect(x, y, w, h, col)`<br>
  너비 `w`, 높이 `h`의 색상 `col`의 사각형을 (`x`, `y`)에 그립니다.

- `rectb(x, y, w, h, col)`<br>
  너비 `w`, 높이 `h`의 색상 `col`의 사각형 외곽선을 (`x`, `y`)에 그립니다.

- `circ(x, y, r, col)`<br>
  반지름 `r`의 색상 `col`의 원을 (`x`, `y`)에 그립니다.

- `circb(x, y, r, col)`<br>
  반지름 `r`의 색상 `col`의 원 외곽선을 (`x`, `y`)에 그립니다.

- `elli(x, y, w, h, col)`<br>
  너비 `w`, 높이 `h`의 색상 `col`의 타원을 (`x`, `y`)에 그립니다.

- `ellib(x, y, w, h, col)`<br>
  너비 `w`, 높이 `h`의 색상 `col`의 타원 외곽선을 (`x`, `y`)에 그립니다.

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  정점이 (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`)인 색상 `col`의 삼각형을 그립니다.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  정점이 (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`)인 색상 `col`의 삼각형 외곽선을 그립니다.

- `fill(x, y, col)`<br>
  (`x`, `y`)와 같은 색상으로 연결된 영역을 색상 `col`로 채웁니다.

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  이미지 뱅크 `img`(0-2)의 (`u`, `v`)에서 크기 (`w`, `h`)의 영역을 (`x`, `y`)로 복사합니다. `w`와/또는 `h`에 음수를 지정하면 수평 및/또는 수직으로 영역이 뒤집힙니다. `colkey`가 지정되면 이를 투명색으로 처리합니다. `rotate`(도 단위), `scale`(1.0 = 100%) 또는 둘 다 지정하면 해당 변환이 적용됩니다.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  타일맵 `tm`(0-7)의 (`u`, `v`)에서 크기 (`w`, `h`)의 영역을 (`x`, `y`)로 복사합니다. `w`와/또는 `h`에 음수를 지정하면 수평 및/또는 수직으로 영역이 뒤집힙니다. `colkey`가 지정되면 이를 투명색으로 처리합니다. `rotate`(도 단위), `scale`(1.0 = 100%) 또는 둘 다 지정하면 해당 변환이 적용됩니다. 타일의 크기는 8x8 픽셀이며, 타일맵에 `(image_tx, image_ty)`의 튜플로 저장됩니다.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  색상 `col`의 문자열 `s`를 (`x`, `y`)에 그립니다.

### Audio

- `sounds`<br>
  사운드 (Sound 클래스의 인스턴스) 목록 (0-63)<br>
  예시: `pyxel.sounds[0].speed = 60`

- `musics`<br>
  뮤직 (Music 클래스의 인스턴스) 목록 (0-7)

- `play(ch, snd, [tick], [loop], [resume])`<br>
  채널 `ch`(0-3)에서 사운드 `snd`(0-63)를 재생합니다. `snd`가 리스트인 경우, 사운드가 순차적으로 재생됩니다. 재생 시작 위치는 `tick`(1 tick = 1/120 초)로 지정할 수 있습니다. `loop`를 `True`로 설정하면 루프 재생이 수행됩니다. 재생이 끝난 후 이전 사운드로 돌아가려면 `resume`을 `True`로 설정합니다.

- `playm(msc, [tick], [loop])`<br>
  뮤직 `msc`(0-7)를 재생합니다. 재생 시작 위치는 `tick`(1 tick = 1/120 초)로 지정할 수 있습니다. `loop`를 `True`로 설정하면 루프 재생이 수행됩니다.

- `stop([ch])`<br>
  지정한 채널 `ch`(0-3)의 재생을 중지합니다. `stop()`을 호출하면 모든 채널의 재생이 중지됩니다.

- `play_pos(ch)`<br>
  채널 `ch`(0-3)의 사운드 재생 위치를 `(sound_no, note_no)`의 튜플로 가져옵니다. 재생이 중지되면 `None`을 반환합니다.

### Math

- `ceil(x)`<br>
  `x`보다 크거나 같은 가장 작은 정수를 반환합니다.

- `floor(x)`<br>
  `x`보다 작거나 같은 가장 큰 정수를 반환합니다.

- `sgn(x)`<br>
  `x`가 양수일 때 `1`, 0일 때 `0`, 음수일 때 `-1`을 반환합니다.

- `sqrt(x)`<br>
  `x`의 제곱근을 반환합니다.

- `sin(deg)`<br>
  `deg`도의 사인 값을 반환합니다.

- `cos(deg)`<br>
  `deg`도의 코사인 값을 반환합니다.

- `atan2(y, x)`<br>
  `y`/`x`의 아크탄젠트 값을 도 단위로 반환합니다.

- `rseed(seed)`<br>
  난수 생성기의 시드를 설정합니다.

- `rndi(a, b)`<br>
  `a` 이상 `b` 이하의 임의의 정수를 반환합니다.

- `rndf(a, b)`<br>
  `a` 이상 `b` 이하의 임의의 부동소수점을 반환합니다.

- `nseed(seed)`<br>
  퍼린 노이즈의 시드를 설정합니다.

- `noise(x, [y], [z])`<br>
  지정된 좌표의 퍼린 노이즈 값을 반환합니다.

### Image 클래스

- `width`, `height`<br>
  이미지의 너비와 높이

- `set(x, y, data)`<br>
  문자열 리스트를 사용하여 (`x`, `y`)에 이미지를 설정합니다.<br>
  예시: `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  (`x`, `y`)에 이미지 파일 (PNG/GIF/JPEG)을 로드합니다.

- `pget(x, y)`<br>
  (`x`, `y`)의 픽셀 색상을 가져옵니다.

- `pset(x, y, col)`<br>
  (`x`, `y`)에 색상 `col`의 픽셀을 그립니다.

### Tilemap 클래스

- `width`, `height`<br>
  타일맵의 너비와 높이

- `imgsrc`<br>
  타일맵이 참조하는 이미지 뱅크(0-2)

- `set(x, y, data)`<br>
  문자열 리스트를 사용하여 (`x`, `y`)에 타일맵을 설정합니다.<br>
  예시: `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  (`x`, `y`)에 TMX 파일 (Tiled Map File)로부터 `layer`(0-) 레이어를 로드합니다.

- `pget(x, y)`<br>
  (`x`, `y`)의 타일을 가져옵니다. 타일은 `(image_tx, image_ty)`의 튜플로 표현됩니다.

- `pset(x, y, tile)`<br>
  (`x`, `y`)에 타일을 설정합니다. 타일은 `(image_tx, image_ty)`의 튜플로 표현됩니다.

### Sound 클래스

- `notes`<br>
  음정 목록 (0-127). 숫자가 클수록 음정이 높아집니다. 음정 `33`은 'A2'(440Hz)에 해당합니다. 쉼표는 `-1`로 표현됩니다.

- `tones`<br>
  음색 목록 (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
  볼륨 목록 (0-7)

- `effects`<br>
  효과 목록 (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  재생 속도. `1`이 가장 빠르고, 숫자가 클수록 재생 속도가 느려집니다. `120`에서는 1음의 길이가 1초가 됩니다.

- `set(notes, tones, volumes, effects, speed)`<br>
  문자열을 사용하여 음정, 음색, 볼륨, 효과를 설정합니다. 음색, 볼륨 또는 효과의 길이가 음정보다 짧으면 처음부터 반복됩니다.

- `set_notes(notes)`<br>
  `CDEFGAB`+`#-`+`01234` 또는 `R`로 이루어진 문자열로 음정을 설정합니다. 대소문자를 구분하지 않으며, 공백은 무시됩니다.<br>
  예시: `pyxel.sounds[0].set_notes("g2b-2d3r rf3f3f3")`

- `set_tones(tones)`<br>
  `TSPN`으로 이루어진 문자열로 음색을 설정합니다. 대소문자를 구분하지 않으며, 공백은 무시됩니다.<br>
  예시: `pyxel.sounds[0].set_tones("ttss pppn")`

- `set_volumes(volumes)`<br>
  `01234567`로 이루어진 문자열로 볼륨을 설정합니다. 대소문자를 구분하지 않으며, 공백은 무시됩니다.<br>
  예시: `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  `NSVFHQ`로 이루어진 문자열로 효과를 설정합니다. 대소문자를 구분하지 않으며, 공백은 무시됩니다.<br>
  예시: `pyxel.sounds[0].set_effects("nfnf nvvs")`

- `mml(mml_str)`<br>
  [Music Macro Language (MML)](https://en.wikipedia.org/wiki/Music_Macro_Language)를 사용하여 관련 매개변수를 설정합니다. 사용할 수 있는 명령어는 `T`(1-900), `@`(0-3), `O`(0-4), `>`, `<`, `Q`(1-8), `V`(0-7), `X`(0-7), `L`(1/2/4/8/16/32), 그리고 `CDEFGABR`+`#+-`+`.~&`입니다. 명령어에 대한 자세한 내용은 [이 페이지](faq-en.md)를 참조하세요.<br>
  예시: `pyxel.sounds[0].mml("t120 @1 o3 q6 l8 x0:12345 c4&c<g16r16>c.<g16 v4 >c.&d16 x0 e2~c2~")`

- `save(filename, count, [ffmpeg])`<br>
  사운드를 `count`번 반복한 WAV 파일을 생성합니다. FFmpeg가 설치되어 있고 `ffmpeg`가 `True`로 설정된 경우, MP4 파일도 생성됩니다.

### Music 클래스

- `seqs`<br>
  여러 채널의 사운드 (0-63)로 이루어진 2차원 리스트

- `set(seq0, seq1, seq2, ...)`<br>
  각 채널에 대한 사운드 (0-63) 리스트를 설정합니다. 빈 리스트가 지정되면 해당 채널은 재생에 사용되지 않습니다.<br>
  예시: `pyxel.musics[0].set([0, 1], [], [3])`

- `save(filename, count, [ffmpeg])`<br>
  음악을 `count`번 반복한 WAV 파일을 생성합니다. FFmpeg가 설치되어 있고 `ffmpeg`가 `True`로 설정된 경우, MP4 파일도 생성됩니다.

### 고급 API

Pyxel에는 사용자에게 혼란을 줄 수 있거나 사용에 전문 지식이 필요할 수 있는 "고급 API"가 포함되어 있지만, 이 문서에서는 언급되지 않았습니다.

자신의 능력에 자신이 있다면 [이것](../python/pyxel/__init__.pyi)을 가이드로 사용하여 놀라운 작품을 만들어 보세요!

## 기여 방법

### 문제 신고

[문제 추적기](https://github.com/kitao/pyxel/issues)를 사용하여 버그 보고서 및 기능 또는 개선 요청을 제출하세요. 새로운 문제를 제출하기 전에 유사한 열린 문제가 없는지 확인해 주세요.

### 기능 테스트

코드를 수동으로 테스트하고 [문제 추적기](https://github.com/kitao/pyxel/issues)에서 버그 또는 개선 사항을 제안하는 분은 언제든지 환영합니다!

### 풀 리퀘스트 제출

패치 및 수정 사항은 풀 리퀘스트(PR) 형식으로 수락됩니다. 풀 리퀘스트가 해결하는 문제는 문제 추적기에서 열려 있어야 합니다.

풀 리퀘스트를 제출하면 [MIT 라이센스](../LICENSE) 아래에서 기여 내용을 라이센스하는 데 동의하는 것으로 간주됩니다.

## 기타 정보

- [자주 묻는 질문](faq-en.md)
- [사용자 예제](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [개발자의 X 계정](https://x.com/kitao)
- [디스코드 서버 (영어)](https://discord.gg/Z87eYHN)
- [디스코드 서버 (일본어)](https://discord.gg/qHA5BCS)

## 라이센스

Pyxel은 [MIT 라이센스](../LICENSE) 하에 라이센스가 부여됩니다. 소스 코드 및 라이센스 표시용 파일에 저작권 및 라이센스 조건을 표시하면 자유롭게 판매 및 배포할 수 있습니다.

## 스폰서 모집

Pyxel은 GitHub Sponsors에서 스폰서를 모집하고 있습니다. Pyxel의 지속적인 유지 관리 및 기능 개발을 지원하기 위해 스폰서를 고려해 주세요. 스폰서에게는 Pyxel 개발자와 직접 상담할 수 있는 혜택이 제공됩니다. 자세한 내용은 [이 페이지](https://github.com/sponsors/kitao)를 참조하세요.
