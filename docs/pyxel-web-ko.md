# Web버전 Pyxel 사용법

Pyxel을 웹 버전으로 사용하면 Python이나 Pyxel을 설치할 필요 없이 PC, 스마트폰 또는 태블릿에서 웹 브라우저에서 Pyxel 애플리케이션을 실행할 수 있습니다.

Pyxel을 웹 버전으로 사용하는 방법은 다음의 3가지가 있습니다.

- **GitHub 리포지토리를 Pyxel Web Launcher에 지정하기**<br>
  Pyxel Web Launcher의 URL에 GitHub 리포지토리 이름을 지정하면, 지정한 리포지토리를 직접 불러와 웹 브라우저에서 애플리케이션을 실행할 수 있습니다. 애플리케이션이 GitHub에 공개되어 있는 경우 가장 간단한 실행 방법입니다.
  
- **Pyxel 애플리케이션을 HTML 파일로 변환하기**<br>
  애플리케이션이 Pyxel 애플리케이션 형식(.pyxapp)인 경우 `pyxel app2html` 명령어를 사용하여 HTML 파일로 변환할 수 있습니다. 변환 후의 HTML 파일은 서버 없이 단독으로 실행 가능합니다.
  
- **HTML 파일을 생성하기 위해 Pyxel 커스텀 태그 사용하기**<br>
  Pyxel 전용 커스텀 태그를 사용하여 애플리케이션 실행용 HTML 파일을 만듭니다. 만든 HTML 파일은 서버에서 호스팅해야 하지만 기존 HTML 페이지에 통합하거나 커스터마이징이 가능합니다.
  
각 방법은 아래에 설명되어 있습니다.

## GitHub 리포지토리를 Pyxel Web Launcher에 지정하기

Python 코드나 Pyxel 애플리케이션(.pyxapp)이 GitHub에 공개되어 있는 경우, Pyxel Web Launcher를 사용하여 직접 실행할 수 있습니다.

Pyxel Web Launcher의 URL 형식은 다음과 같습니다.

```
https://kitao.github.io/pyxel/wasm/launcher/?<명령어>=<github 사용자명>.<리포지토리 이름>.<파일 디렉토리>.<확장자가 없는 파일명>
```

사용 가능한 명령어는 세 가지가 있습니다.

- `run`: Python 스크립트를 실행합니다.
- `play`: Pyxel 애플리케이션을 실행합니다.
- `edit`: Pyxel Editor를 시작합니다.

예를 들어 사용자 이름이 `taro`, 리포지토리 이름이 `my_repo`, 파일 디렉토리가 `src/scenes`, Python 스크립트가 `title.py`인 경우 URL은 다음과 같이 됩니다.

```
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.src.scenes.title
```

`dist/games` 디렉토리에 있는 `shooter.pyxapp`을 실행할 경우 URL은 다음과 같습니다.

```
https://kitao.github.io/pyxel/wasm/launcher/?play=taro.my_repo.dist.games.shooter
```

여러 파일로 나뉜 애플리케이션을 `run` 명령어로 실행하면 시간이 오래 걸릴 수 있으므로, 그 경우에는 Pyxel 애플리케이션 파일(.pyxapp)로 변환하여 `play` 명령어로 실행하는 것을 추천합니다.

`run` 및 `play` 명령어에는 가상 게임 패드를 활성화하는 `gamepad`속성이나 추가 패키지를 지정하는 `packages` 속성을 지정할 수 있습니다.

예를 들어 가상 게임 패드를 활성화하고, 추가 패키지로 NumPy와 Pandas를 사용하는 경우 URL은 다음과 같습니다.

```
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.src.scenes.title&gamepad=enabled&packages=numpy,pandas
```

추가 가능한 패키지는 [Pyodide 지원 패키지](https://pyodide.org/en/stable/usage/packages-in-pyodide.html)에 제한됩니다

`edit` 명령어를 사용하는 경우, `editor` 속성으로 Pyxel Editor의 시작 화면을 지정할 수 있습니다.

예를 들어, `assets` 디렉토리에 있는 `shooter.pyxres` 파일을 Tilemap Editor 화면에서 시작하려면, 다음 URL을 사용합니다.

```html
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.assets.shooter&editor=tilemap
```

[Pyxel Web Launcher page](https://kitao.github.io/pyxel/wasm/launcher/)에서 필요한 정보를 입력하여 애플리케이션 실행 URL을 자동으로 생성하는 것도 가능합니다.

## Pyxel 애플리케이션을 HTML 파일로 변환하기

Pyxel 애플리케이션 파일(.pyxapp)은 다음 명령어로 단독으로 동작하는 HTML 파일로 변환할 수 있습니다.

```sh
pyxel app2html your_app.pyxapp
```

생성된 HTML 파일에서는 가상 게임 패드가 기본적으로 활성화되어 있지만, 커스텀 태그를 수정하여 비활성화할 수도 있습니다.

## Pyxel 커스텀 태그를 사용하여 HTML 파일 생성하기

HTML 파일에 Pyxel 전용 커스텀 태그를 작성함으로써 Pyxel 애플리케이션을 실행할 수 있습니다.

Pyxel 커스텀 태그를 사용하려면 다음 스크립트 태그를 HTML 파일에 추가합니다.

```html
<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel/wasm/pyxel.js"></script>
```

Python 코드를 직접 실행하려면 다음과 같이 `pyxel-run` 태그의 script 속성에 코드를 작성합니다.

```html
<pyxel-run
  script="
import pyxel
pyxel.init(200, 150)
pyxel.cls(8)
pyxel.line(20, 20, 180, 130, 7)
pyxel.show()
"
></pyxel-run>
```

외부 Python 파일을 읽어 실행하는 경우 `pyxel-run` 태그에 `root`와 `name` 속성을 지정합니다.

`root`는 검색의 기점이 되는 디렉터리, `name`은 파일 경로입니다.

예를 들어 위의 코드를 `test.py`라는 파일로 저장하고 HTML 파일과 같은 디렉터리에 배치한 경우 다음과 같이 작성합니다.

```html
<pyxel-run root="." name="test.py"></pyxel-run>
```

`root`가 현재 디렉터리인 경우(`root="."`) `root` 속성은 생략 가능합니다.

로컬 HTML 파일에서 외부 파일을 읽어오려면 서버에서 호스팅이 필요합니다.

Python 환경이 있다면 다음 명령어로 간이 서버를 시작할 수 있습니다.

```python
python -m http.server
# MacOS나 Linux의 경우 python3를 사용해 주세요.
```

서버 시작 후 브라우저에서 `http://localhost:8000/test.html`.에 접근할 수 있습니다.

마찬가지로 Pyxel 애플리케이션(.pyxapp)은 `pyxel-play` 태그로 실행할 수 있습니다.

```html
<pyxel-play
  root="https://cdn.jsdelivr.net/gh/kitao/pyxel/python/pyxel/examples"
  name="megaball.pyxapp"
></pyxel-play>
```

이 예제에서는 `root` 속성에 URL을 지정하고 있습니다.

`pyxel-run` 태그와 `pyxel-play` 태그에는 가상 게임 패드를 활성화하는 `gamepad` 속성이나 추가 패키지를 지정하는 `packages` 속성을 지정할 수 있습니다.

예를 들어 가상 게임 패드를 활성화하고 NumPy와 Pandas를 사용하는 경우 다음과 같이 됩니다.

```html
<pyxel-run name="test.py" gamepad="enabled" packages="numpy,pandas"></pyxel-run>
```

사용 가능한 패키지는 [Pyodide 지원 패키지](https://pyodide.org/en/stable/usage/packages-in-pyodide.html)에 제한됩니다.

또한 `pyxel-edit` 태그를 사용하여 Pyxel Editor를 시작할 수 있습니다.

예를 들어 `assets` 디렉토리에 있는 `shooter.pyxres` 파일을 이미지 에디터 화면에서 시작하려면 다음과 같이 작성합니다.

```html
<pyxel-edit root="assets" name="sample.pyxres" editor="image"></pyxel-edit>
```

Pyxel을 실행하는 HTML 파일에 `id="pyxel-screen"`인 `<div>` 태그를 추가하면 해당 요소를 Pyxel의 화면으로 사용할 수 있습니다. 이 `<div>` 태그의 위치나 크기를 조정함으로써 Pyxel 화면의 배치나 크기를 변경할 수 있습니다.
