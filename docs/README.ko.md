# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** (/ˈpɪksəl/)은 Python을 위한 복고풍 게임 엔진입니다.

사양은 복고풍 게임 콘솔에서 영감을 받아 16색만 지원하고 4개의 사운드 채널을 사용할 수 있어 픽셀 아트 스타일의 게임 제작을 쉽게 즐길 수 있습니다.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

Pyxel 개발은 사용자 피드백에 의해 이루어집니다. GitHub에서 Pyxel에 별을 주세요!

<p>
<a href="https://kitao.github.io/pyxel/wasm/showcase/examples/10-platformer.html">
<img src="images/10_platformer.gif" width="290">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/apps/30sec-of-daylight.html">
<img src="images/30sec_of_daylight.gif" width="350">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/examples/02-jump-game.html">
<img src="images/02_jump_game.gif" width="330">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/apps/megaball.html">
<img src="images/megaball.gif" width="310">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
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
- 256x256 이미지 뱅크 3개
- 256x256 타일맵 8개
- 64개의 정의 가능한 사운드와 4개의 채널
- 결합 가능한 8개의 음악 트랙
- 키보드, 마우스 및 게임패드 입력
- 이미지 및 사운드 편집 도구
- 사용자 확장 가능한 색상, 사운드 채널 및 뱅크

### 색상 팔레트

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## 설치 방법

### Windows

[Python 3](https://www.python.org/) (버전 3.8 이상)을 설치한 후, 다음 명령어를 실행하세요:

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

[Python 3](https://www.python.org/) (버전 3.8 이상)를 설치한 후, 다음 명령어를 실행하세요:

```sh
pip install -U pyxel
```

이전 명령이 작동하지 않으면 [Makefile](../Makefile)에 있는 지침에 따라 소스에서 Pyxel을 빌드하는 것을 고려하세요.

### Web

Pyxel의 Web 버전은 호환되는 웹 브라우저만 있으면 PC, 스마트폰, 태블릿에서 사용할 수 있으며, Python이나 Pyxel을 설치할 필요가 없습니다.

가장 쉬운 사용 방법은 온라인 IDE [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/)를 이용하는 것입니다.

자신의 사이트에 Pyxel 앱을 임베딩하는 등의 다른 사용 패턴에 대해서는 [이 페이지](pyxel-web-en.md)를 참조하세요.

## 기본 사용법

### Pyxel 명령어

Pyxel을 설치하면 `pyxel` 명령어를 사용할 수 있습니다. `pyxel` 뒤에 명령어 이름을 지정하여 다양한 작업을 수행합니다.

인수 없이 실행하면 사용 가능한 명령어 목록을 확인할 수 있습니다:

```sh
pyxel
```

```
Pyxel 2.7.1, a retro game engine for Python
usage:
    pyxel run PYTHON_SCRIPT_FILE(.py)
    pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE(.py)
    pyxel play PYXEL_APP_FILE(.pyxapp)
    pyxel edit [PYXEL_RESOURCE_FILE(.pyxres)]
    pyxel package APP_DIR STARTUP_SCRIPT_FILE(.py)
    pyxel app2exe PYXEL_APP_FILE(.pyxapp)
    pyxel app2html PYXEL_APP_FILE(.pyxapp)
    pyxel copy_examples
```

### 예제 실행하기

다음 명령어로 예제를 현재 디렉토리로 복사할 수 있습니다:

```sh
pyxel copy_examples
```

로컬 환경에서는 다음 명령어로 실행할 수 있습니다:

```sh
# examples 디렉터리에서 샘플 실행
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# examples/apps 디렉터리에서 앱 실행
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

예제 목록은 [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)에서 브라우저로도 확인하고 실행할 수 있습니다.

## 애플리케이션 만들기

### 프로그램 작성

Python 스크립트에서 Pyxel을 가져온 뒤 `init` 함수로 창 크기를 지정하고 `run` 함수로 애플리케이션을 시작합니다.

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

### 프로그램 실행

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

디렉터리 모니터링은 `Ctrl(Command)+C`를 눌러 중지합니다.

### 특수 키 조작

Pyxel 애플리케이션이 실행 중일 때, 다음의 특수 키 조작을 수행할 수 있습니다:

- `Esc`<br>
  애플리케이션 종료
- `Alt(Option)+R` 또는 게임패드에서 `A+B+X+Y+BACK`<br>
  애플리케이션 리셋
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

## 리소스 만들기

### Pyxel Editor

Pyxel Editor로 Pyxel 애플리케이션에서 사용할 이미지와 사운드를 만들 수 있습니다.

다음 명령으로 Pyxel Editor를 시작할 수 있습니다:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

지정된 Pyxel 리소스 파일(.pyxres)이 존재하면 해당 파일이 로드됩니다. 존재하지 않는 경우 지정된 이름으로 새 파일이 생성됩니다. 리소스 파일이 생략된 경우 `my_resource.pyxres`라는 새 파일이 생성됩니다.

Pyxel Editor를 시작한 후 다른 리소스 파일로 전환하려면 해당 파일을 편집기로 드래그 앤 드롭하면 됩니다.

생성된 리소스 파일은 `load` 함수를 사용하여 로드할 수 있습니다.

Pyxel Editor에는 다음과 같은 편집 모드가 있습니다.

**이미지 편집기**

각 **이미지 뱅크**의 이미지를 편집하는 모드입니다.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_editor.gif">
</a>

이미지 편집기로 PNG/GIF/JPEG 파일을 드래그 앤 드롭하면 현재 선택된 이미지 뱅크에 이미지를 로드합니다.

**타일맵 편집기**

이미지 뱅크의 이미지를 타일 패턴으로 배열하여 **타일맵**을 편집하는 모드입니다.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/tilemap-editor.html">
<img src="images/tilemap_editor.gif">
</a>

TMX 파일(Tiled Map File)을 타일맵 편집기로 드래그 앤 드롭하면 현재 선택된 타일맵에 레이어 0이 로드됩니다.

**사운드 편집기**

멜로디와 **사운드** 효과에 사용되는 사운드를 편집하는 모드입니다.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
<img src="images/sound_editor.gif">
</a>

**뮤직 편집기**

재생 순서에 따라 사운드를 배열한 **뮤직**을 편집하는 모드입니다.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/music-editor.html">
<img src="images/music_editor.gif">
</a>

### 기타 생성 방법

Pyxel 이미지와 타일맵은 다음 방법을 사용하여 생성할 수도 있습니다:

- `Image.set` 함수 또는 `Tilemap.set` 함수를 사용해 문자열 목록에서 이미지나 타일맵을 만듭니다.
- `Image.load` 함수로 Pyxel 팔레트에 맞는 이미지 파일(PNG/GIF/JPEG)을 로드합니다.

Pyxel 사운드와 음악은 다음 방법을 사용하여 생성할 수도 있습니다:

- `Sound.set` 함수 또는 `Music.set` 함수로 문자열에서 생성합니다.

이 함수의 사용법은 API 참조를 참조하십시오.

## 애플리케이션 배포 방법

Pyxel은 플랫폼에 관계없이 작동하는 전용 배포 형식인 Pyxel 애플리케이션 파일을 지원합니다.

Pyxel 애플리케이션 파일(.pyxapp)은 `pyxel package` 명령으로 생성합니다:

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

## API 레퍼런스

Pyxel API의 전체 목록은 [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/) 에서 확인할 수 있습니다.

Pyxel에는 전문 지식이 필요한 "고급 API"도 포함되어 있습니다. 레퍼런스 페이지에서 "Advanced" 체크박스를 선택하면 확인할 수 있습니다.

실력에 자신이 있다면 고급 API를 활용하여 놀라운 작품을 만들어 보세요!

## 기여 방법

### 문제 신고

[문제 추적기](https://github.com/kitao/pyxel/issues)를 사용하여 버그 보고서 및 기능 또는 개선 요청을 제출하세요. 새로운 문제를 제출하기 전에 유사한 열린 문제가 없는지 확인해 주세요.

### 기능 테스트

코드를 수동으로 테스트하고 [문제 추적기](https://github.com/kitao/pyxel/issues)에서 버그 또는 개선 사항을 제안하는 분은 언제든지 환영합니다!

### 풀 리퀘스트 제출

패치 및 수정 사항은 풀 리퀘스트(PR) 형식으로 수락됩니다. 풀 리퀘스트가 해결하는 문제는 문제 추적기에서 열려 있어야 합니다.

풀 리퀘스트를 제출하면 [MIT 라이센스](../LICENSE) 아래에서 기여 내용을 라이센스하는 데 동의하는 것으로 간주됩니다.

## 웹 도구 및 예제

- [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)
- [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/)
- [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/)

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
