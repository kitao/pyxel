# <img src="https://raw.githubusercontent.com/kitao/pyxel/master/images/pyxel_logo_152x64.png">

[ [English](https://github.com/kitao/pyxel/blob/master/README.md) | [日本語](https://github.com/kitao/pyxel/blob/master/README.ja.md) | [中文](https://github.com/kitao/pyxel/blob/master/README.cn.md) | [한국어](https://github.com/kitao/pyxel/blob/master/README.ko.md) | [Español](https://github.com/kitao/pyxel/blob/master/README.es.md) | [Português](https://github.com/kitao/pyxel/blob/master/README.pt.md | [Русский](https://github.com/kitao/pyxel/blob/master/README.ru.md) ]

**Pyxel (ピクセル)** はPython向けのレトロゲームエンジンです。

使える色は16色のみ、同時に再生できる音は4音までなど、レトロゲーム機を意識したシンプルな仕様で、Pythonでドット絵スタイルのゲームづくりが気軽に楽しめます。

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

Pyxelのゲーム機の仕様やAPIは、
[PICO-8](https://www.lexaloffle.com/pico-8.php)や[TIC-80](https://tic.computer/)のデザインを参考にしています。

Pyxelはオープンソースで、無料で自由に使えます。Pyxelでレトロゲームづくりを始めましょう！

## 仕様

- Windows、Mac、Linux対応
- Python3によるコード記述
- 16色固定パレット
- 256x256サイズ、3画像バンク
- 256x256サイズ、8タイルマップ
- 4音同時再生、定義可能な64サウンド
- 任意のサウンドを組み合わせ可能な8ミュージック
- キーボード、マウス、ゲームパッド
- 画像・サウンド編集ツール

### カラーパレット

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/05_color_palette.png">
<br><br>
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/images/pyxel_palette.png">

## インストール方法

### Windows

最初に[Python3](https://www.python.org/) (バージョン3.6.8以上) をインストールします。

公式のPythonインストーラーを使用する場合は、次のボタンをチェックして、**Pythonをパスに追加**してください。

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/images/python_installer.png">

続いて、コマンドプロンプトから、以下の`pip`コマンドでPyxelをインストールします。

```sh
pip install -U pyxel
```

### Mac

最初に、[Homebrew](https://brew.sh/)を導入した環境で、以下のコマンドで[Python3](https://www.python.org/) (バージョン3.6.8以上) と必要なパッケージをインストールします。

```sh
brew install python3 gcc sdl2 sdl2_image gifsicle
```

Python3は他の方法でインストールしても問題ありませんが、他のライブラリのインストールは必須なので注意してください。

続いて、**ターミナルを再起動**した後に、`pip3`コマンドでPyxelをインストールします。

```sh
pip3 install -U pyxel
```

### Linux

各ディストリビューションに適した方法で[Python3](https://www.python.org/) (バージョン3.6.8以上) と必要なパッケージをインストールしてください。

**Ubuntu:**

```sh
sudo apt install python3 python3-pip libsdl2-dev libsdl2-image-dev gifsicle
sudo -H pip3 install -U pyxel
```

### その他の環境

上記以外の環境 (32bit版LinuxやRaspberry PI等) にPyxelをインストールするには、次の手順でビルドを行なってください。

#### 必要となるツールやパッケージをインストールする

- C++のビルド環境 (gcc、makeコマンドを含む)
- libsdl2-dev、libsdl2-image-dev
- [Python3](https://www.python.org/) (バージョン3.6.8以上)、pipコマンド

#### 任意のフォルダで以下のコマンドを実行する

```sh
git clone https://github.com/kitao/pyxel.git
cd pyxel
make -C pyxel/core clean all
pip3 install .
```

### サンプルのインストール

Pyxelインストール後に、以下のコマンドでカレントディレクトリにPyxelのサンプルコード一式をコピーできます。

```sh
install_pyxel_examples
```

コピーされるサンプルは以下の通りです。

- [01_hello_pyxel.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/01_hello_pyxel.py) - シンプルなアプリケーション
- [02_jump_game.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/02_jump_game.py) - Pyxelリソースファイルを使ったジャンプゲーム
- [03_draw_api.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/03_draw_api.py) - 描画APIのデモ
- [04_sound_api.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/04_sound_api.py) - サウンドAPIのデモ
- [05_color_palette.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/05_color_palette.py) - カラーパレット一覧
- [06_click_game.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/06_click_game.py) - マウスクリックゲーム
- [07_snake.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/07_snake.py) - BGM付きスネークゲーム
- [08_triangle_api.py](https://github.com/kitao/pyxel/blob/master/pyxel/examples/08_triangle_api.py) - 三角形描画APIのデモ

サンプルは通常のPythonコードと同様に実行できます。

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

## 使い方

### アプリケーションの作成方法

Pythonコード内でPyxelモジュールをインポートして、`init`関数でウィンドウサイズを指定した後に、`run`関数でPyxelアプリケーションを開始します。

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

`run`関数の引数にはフレーム更新処理を行う`update`関数と、描画処理を行う`draw`関数を指定します。

実際のアプリケーションでは、以下のようにクラスでPyxelの処理をラップするのがおすすめです。

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

また、シンプルなグラフィックスやアニメーションを作成する場合は、`show`関数や`flip`関数を使った簡易的な記述も可能です。

`show`関数は画面を表示して、`ESC`キーが押されるまで待機します。

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

`flip`関数は画面を一度更新します。

```python
import pyxel

pyxel.init(120, 80)

while True:
    pyxel.cls(3)
    pyxel.rectb(pyxel.frame_count % 160 - 40, 20, 40, 40, 7)
    pyxel.flip()
```


### 特殊操作

Pyxelアプリケーション実行中に、以下の特殊操作を行うことができます。

- `Esc`<br>
アプリケーションを終了する
- `Alt(Option)+1`<br>
スクリーンショットをデスクトップに保存する
- `Alt(Option)+2`<br>
画面キャプチャ動画の録画開始時刻をリセットする
- `Alt(Option)+3`<br>
画面キャプチャ動画 (gif) をデスクトップに保存する (最大30秒)
- `Alt(Option)+0`<br>
パフォーマンスモニタ (fps、update時間、draw時間) の表示を切り替える
- `Alt(Option)+Enter`<br>
フルスクリーン表示を切り替える

### リソースの作成方法

付属するPyxel EditorでPyxelアプリケーションで使用する画像やサウンドを作成することができます。

Pyxel Editorは以下のコマンドで起動します。

```sh
pyxeleditor [Pyxelリソースファイル]
```

指定したPyxelリソースファイル (.pyxres) が存在する場合は読み込み、存在しない場合は指定した名前で新規にファイルを作成します。リソースファイルを省略した場合は`my_resource.pyxres`がファイル名になります。

Pyxel Editorの起動後に、別のリソースファイルをドラッグ＆ドロップすることでファイルを切り替えることができます。また、``Ctrl``(``Cmd``)キーを押しながらリソースファイルをドラッグ＆ドロップすると、現在編集中のリソースタイプ(イメージ/タイルマップ/サウンド/ミュージック)のみが読み込まれます。この操作により、複数のリソースファイルを1つにまとめることができます。

作成したリソースファイルはPyxelアプリケーションから`load`関数で読み込めます。

Pyxel Editorには次の編集モードがあります。

**イメージエディタ:**

イメージバンクの画像を編集する画面です。

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/image_editor.gif">

イメージエディタ画面にpngファイルをドラッグ＆ドロップすると、画像を選択中のイメージバンクに読み込むことができます。

**タイルマップエディタ:**

イメージバンクの画像をタイル状に並べたタイルマップを編集する画面です。

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/tilemap_editor.gif">

**サウンドエディタ:**

サウンドを編集する画面です。

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/sound_editor.gif">

**ミュージックエディタ:**

サウンドを再生順に並べたミュージックを編集する画面です。

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/editor/screenshots/music_editor.gif">

### その他のリソース作成方法

Pyxel用の画像やタイルマップは以下の方法で作成することもできます。

- `Image.set`や`Tilemap.set`関数で文字列のリストから作成する
- `Image.load`関数でPyxel向け配色のpngファイルを読み込む

Pyxel用のサウンドやミュージックは以下の方法で作成することもできます。

- `Sound.set`や`Music.set`関数で文字列から作成する

各関数の使い方はAPIリファレンスを参照してください。

### 単体実行ファイルの作成方法

付属するPyxel Packagerを使用することで、Pythonがインストールされていない環境でも動作する、単体実行ファイルを作成することができます。

単体実行ファイルを作成するには、[PyInstaller](https://www.pyinstaller.org/)がインストール済みの環境で、次のように`pyxelpackager`コマンドでアプリケーションの起動に使用するPythonファイルを指定します。

```sh
pyxelpackager python_file
```

処理に成功すると、distフォルダに単体実行可能なファイルが作成されます。

.pyxresファイルや.pngファイル等のリソースも同梱する必要がある場合は、`assets`フォルダ以下に置くと取り込まれます。

``-i icon_file``オプションでアイコンファイルを設定することも可能です。

## APIリファレンス

### システム

- `width`, `height`<br>
画面の幅と高さ

- `frame_count`<br>
経過フレーム数

- `init(width, height, [caption], [scale], [palette], [fps], [quit_key], [fullscreen])`<br>
Pyxelアプリを画面サイズ (`width`, `height`) で初期化する。画面の最大の幅と高さは255<br>
また、`caption`でウィンドウタイトル、`scale`で表示倍率、`palette`でパレット色、`fps`で動作フレームレート、`quit_key`でアプリケーション終了キー、`fullscreen`でフルスクリーンでの起動を指定できる。`palette`は24ビットカラーの16要素のリストで指定する<br>
例：`pyxel.init(160, 120, caption="Pyxel with PICO-8 palette", palette=[0x000000, 0x1D2B53, 0x7E2553, 0x008751, 0xAB5236, 0x5F574F, 0xC2C3C7, 0xFFF1E8, 0xFF004D, 0xFFA300, 0xFFEC27, 0x00E436, 0x29ADFF, 0x83769C, 0xFF77A8, 0xFFCCAA], quit_key=pyxel.KEY_NONE, fullscreen=True)`

- `run(update, draw)`<br>
Pyxelアプリを開始し、フレーム更新時に`update`関数、描画時に`draw`関数を呼ぶ

- `run_with_profiler(update, draw)`<br>
プロファイラ付きでPyxelアプリを開始し、フレーム更新時に`update`関数、描画時に`draw`関数を呼ぶ。アプリ終了時に各関数の処理時間を出力する

- `quit()`<br>
現在フレーム終了時にPyxelアプリを終了する

- `flip()`<br>
強制的に画面を描画する (通常のアプリケーションでは使用しない)

- `show()`<br>
画面を描画して待ち続ける (通常のアプリケーションでは使用しない)

### リソース

- `save(filename)`<br>
実行スクリプトのディレクトリにリソースファイル (.pyxres) を保存する

- `load(filename, [image], [tilemap], [sound], [music])`<br>
実行スクリプトのディレクトリからリソースファイル (.pyxres) を読み込む。リソースタイプ(イメージ/タイルマップ/サウンド/ミュージック)に``False``を指定すると、そのリソースは読み込まれない

### 入力
- `mouse_x`, `mouse_y`<br>
現在のマウスカーソル座標

- `mouse_wheel`<br>
現在のマウスホイールの値

- `btn(key)`<br>
`key`が押されていたら`True`、押されていなければ`False`を返す ([キー定義一覧](https://github.com/kitao/pyxel/blob/master/pyxel/__init__.py))

- `btnp(key, [hold], [period])`<br>
そのフレームに`key`が押されたら`True`、押されなければ`False`を返す。`hold`と`period`を指定すると、`hold`フレーム以上ボタンを押し続けた際に`period`フレーム間隔で`True`が返る

- `btnr(key)`<br>
そのフレームに`key`が離されたら`True`、離されなければ`False`を返す

- `mouse(visible)`<br>
`visible`が`True`ならマウスカーソルを表示し、`False`なら非表示にする。マウスカーソルが非表示でも座標は更新される

### グラフィックス

- `image(img, [system])`<br>
イメージバンク`img`(0-2) を操作する (イメージクラスを参照のこと)。`system`に`True`を指定すると、システム用のイメージバンクにアクセスできる。3がフォント、リソースエディタ用。4が表示スクリーン用<br>
例：`pyxel.image(0).load(0, 0, "title.png")`

- `tilemap(tm)`<br>
タイルマップ`tm`(0-7)を操作する (タイルマップクラスを参照のこと)

- `clip(x, y, w, h)`<br>
画面の描画領域を (`x`, `y`) から幅`w`、高さ`h`に設定する。`clip()`で描画領域を全画面にリセットする

- `pal(col1, col2)`<br>
描画時に色`col1`を`col2`に置き換える。`pal()`で初期状態にリセットする

- `cls(col)`<br>
画面を色`col`でクリアする

- `pget(x, y)`<br>
(`x`, `y`) のピクセルの色を取得する

- `pset(x, y, col)`<br>
(`x`, `y`) に色`col`のピクセルを描画する

- `line(x1, y1, x2, y2, col)`<br>
色`col`の直線を (`x1`, `y1`)-(`x2`, `y2`) に描画する

- `rect(x, y, w, h, col)`<br>
幅`w`、高さ`h`、色`col`の矩形を (`x`, `y`) に描画する

- `rectb(x, y, w, h, col)`<br>
幅`w`、高さ`h`、色`col`の矩形の輪郭線を (`x`, `y`) に描画する

- `circ(x, y, r, col)`<br>
半径`r`、色`col`の円を (`x`, `y`) に描画する

- `circb(x, y, r, col)`<br>
半径`r`、色`col`の円の輪郭線を (`x`, `y`) に描画する

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
頂点が (`x1`, `y1`)、(`x2`, `y2`)、(`x3`, `y3`)、色`col`の三角形を描画する

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
頂点が (`x1`, `y1`)、(`x2`, `y2`)、(`x3`, `y3`)、色`col`の三角形の輪郭線を描画する

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
イメージバンク`img`(0-2) の (`u`, `v`) からサイズ (`w`, `h`) の領域を (`x`, `y`) にコピーする。`w`、`h`それぞれに負の値を設定すると水平、垂直方向に反転する。`colkey`に色を指定すると透明色として扱われる

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
タイルマップ`tm`(0-7) を (`u`, `v`) からサイズ (`w`, `h`) のタイル情報に従って (`x`, `y`) に描画する。`colkey`に色を指定すると透明色として扱われる。タイルマップは1タイルが8x8のサイズで描画され、タイル番号が0ならイメージバンクの (0, 0)-(7, 7) の領域、1なら (8, 0)-(15, 0) の領域を表す

- `text(x, y, s, col)`<br>
色`col`の文字列`s`を (`x`, `y`) に描画する

### オーディオ

- `sound(snd, [system])`<br>
サウンド`snd`(0-63) を操作する (サウンドクラスを参照のこと)。`system`に`True`を指定すると、システム用のサウンド64にアクセスできる<br>
例：`pyxel.sound(0).speed = 60`

- `music(msc)`<br>
ミュージック`msc`(0-7) を操作する (ミュージッククラスを参照のこと)

- `play_pos(ch)`<br>
チャンネル`ch`(0-3) のサウンド再生位置を取得する。100と1000の位はサウンド番号、1と10の位はノート番号。再生停止時は`-1`を返す

- `play(ch, snd, loop=False)`<br>
チャンネル`ch`(0-3) でサウンド`snd`(0-63) を再生する。`snd`がリストの場合順に再生する

- `playm(msc, loop=False)`<br>
ミュージック`msc`(0-7) を再生する

- `stop([ch])`<br>
全チャンネルのサウンドの再生を停止する。`ch`(0-3) を指定すると該当チャンネルのみを停止する

### イメージクラス

- `width`, `height`<br>
イメージの幅と高さ

- `data`<br>
イメージのデータ (256x256の二次元リスト)

- `get(x, y)`<br>
イメージの (`x`,`y`) のデータを取得する

- `set(x, y, data)`<br>
(`x`, `y`) に値または文字列のリストでイメージのデータを設定する<br>
例：`pyxel.image(0).set(10, 10, ["1234", "5678", "9abc", "defg"])`

- `load(x, y, filename)`<br>
(`x`, `y`) に実行スクリプトのディレクトリからpngファイルを読み込む

- `copy(x, y, img, u, v, w, h)`<br>
イメージバンク`img`(0-2) の (`u`, `v`) からサイズ (`w`, `h`) の領域を (`x`, `y`) にコピーする

### タイルマップクラス

- `width`, `height`<br>
タイルマップの幅と高さ

- `data`<br>
タイルマップのデータ (256x256の二次元リスト)

- `refimg`<br>
タイルマップが参照するイメージバンク

- `get(x, y)`<br>
タイルマップの (`x`,`y`) のデータを取得する

- `set(x, y, data)`<br>
(`x`, `y`) に値または文字列のリストでタイルマップのデータを設定する。<br>
e.g. `pyxel.tilemap(0).set(0, 0, ["000102", "202122", "a0a1a2", "b0b1b2"])`

- `copy(x, y, tm, u, v, w, h)`<br>
タイルマップ`tm`(0-7) の (`u`, `v`) からサイズ (`w`, `h`) の領域を (`x`, `y`) にコピーする

### サウンドクラス

- `note`<br>
音程 (0-127) のリスト (33 = 'A2' = 440Hz)

- `tone`<br>
音色 (0:Triangle / 1:Square / 2:Pulse / 3:Noise) のリスト

- `volume`<br>
音量 (0-7) のリスト

- `effect`<br>
エフェクト (0:None / 1:Slide / 2:Vibrato / 3:FadeOut) のリスト

- `speed`<br>
1音の長さ (120 = 1音1秒)

- `set(note, tone, volume, effect, speed)`<br>
文字列で音程、音色、音量、エフェクトを設定する。音色、音量、エフェクトの長さが音程より短い場合は、先頭から繰り返される

- `set_note(note)`<br>
'CDEFGAB'+'#-'+'0123'または'R'の文字列で音程を設定する。大文字と小文字を区別せず、空白は無視される<br>
例：`pyxel.sound(0).set_note("G2B-2D3R RF3F3F3")`

- `set_tone(tone)`<br>
'TSPN'の文字列で音色を設定する。大文字と小文字を区別せず、空白は無視される<br>
例：`pyxel.sound(0).set_tone("TTSS PPPN")`

- `set_volume(volume)`<br>
'01234567'の文字列で音量を設定する。大文字と小文字を区別せず、空白は無視される<br>
例：`pyxel.sound(0).set_volume("7777 7531")`

- `set_effect(effect)`<br>
'NSVF'の文字列でエフェクトを設定する。大文字と小文字を区別せず、空白は無視される<br>
例：`pyxel.sound(0).set_effect("NFNF NVVS")`

### ミュージッククラス

- `ch0`<br>
チャンネル0で再生するサウンド (0-63) のリスト。空リストを指定すると再生にそのチャンネルを使用しない

- `ch1`<br>
チャンネル1で再生するサウンド (0-63) のリスト。空リストを指定すると再生にそのチャンネルを使用しない

- `ch2`<br>
チャンネル2で再生するサウンド (0-63) のリスト。空リストを指定すると再生にそのチャンネルを使用しない

- `ch3`<br>
チャンネル3で再生するサウンド (0-63) のリスト。空リストを指定すると再生にそのチャンネルを使用しない

- `set(ch0, ch1, ch2, ch3)`<br>
全チャンネルのサウンド (0-63) のリストを設定する。空リストを指定すると再生にそのチャンネルを使用しない<br>
例：`pyxel.music(0).set([0, 1], [2, 3], [4], [])`

- `set_ch0(data)`<br>
チャンネル0のサウンド (0-63) のリストを設定する

- `set_ch1(data)`<br>
チャンネル1のサウンド (0-63) のリストを設定する

- `set_ch2(data)`<br>
チャンネル2のサウンド (0-63) のリストを設定する

- `set_ch3(data)`<br>
チャンネル3のサウンド (0-63) のリストを設定する

## コントリビューション方法

### 問題の報告

不具合の報告や機能の要望は[Issue Tracker](https://github.com/kitao/pyxel/issues)で受け付けています。
新しいレポートを作成する前に、同じ内容のものがないか確認をお願いします。

新しいレポートを作成する際は、[こちらのリンク](https://github.com/kitao/pyxel/issues/new/choose)から内容に適したテンプレートを選択してください。

### 動作確認

動作確認を行い、[Issue Tracker](https://github.com/kitao/pyxel/issues)で不具合の報告や改善の提案をしてくれる方は大歓迎です！

### プルリクエスト

パッチや修正はプルリクエスト(PR)として受け付けています。提出の前に問題がすでに解決済みでないか[Issue Tracker](https://github.com/kitao/pyxel/issues)で確認をお願いします。

提出されたプルリクエストは[MITライセンス](https://github.com/kitao/pyxel/blob/master/LICENSE)で公開することに同意したものを見なされます。

## その他の情報

- [Wiki](https://github.com/kitao/pyxel/wiki)
- [Subreddit](https://www.reddit.com/r/pyxel/)
- [Discord server (English)](https://discord.gg/FC7kUZJ)
- [Discord server (Japanese - 日本語版)](https://discord.gg/qHA5BCS)

## ライセンス

Pyxelは[MITライセンス](http://en.wikipedia.org/wiki/MIT_License)です。ソースコードやライセンス表示用のファイル等で、[著作権とライセンス全文](https://raw.githubusercontent.com/kitao/pyxel/master/LICENSE)の表示を行えば、自由に販売や配布をすることができます。

Pyxelは以下のソフトウェアを使用しています。

- [SDL2](https://www.libsdl.org/)
- [miniz-cpp](https://github.com/tfussell/miniz-cpp)
- [Gifsicle](https://www.lcdf.org/gifsicle/)
