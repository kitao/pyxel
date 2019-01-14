# <img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/assets/pyxel_logo_152x64.png">

[ [English](https://github.com/kitao/pyxel/blob/master/README.md) | [日本語](https://github.com/kitao/pyxel/blob/master/README.ja.md) | [Other Languages](https://github.com/kitao/pyxel/wiki) ]

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

Pyxelのゲーム機の仕様やAPI、パレットなどは、
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

## インストール方法

### Windows

[Python3](https://www.python.org/)をインストールした後に、以下の`pip`コマンドでPyxelをインストールします。

```sh
pip install pyxel
```

### Mac

[Python3](https://www.python.org/)と[glfw](http://www.glfw.org/) (バージョン3.2.1以上) をインストールをした後に、`pip`コマンドでPyxelをインストールします。

[Homebrew](https://brew.sh/)を導入している環境では、以下のコマンドで必要なパッケージが一通りインストールできます。

```sh
brew install python3 glfw
pip3 install pyxel
```

### Linux

各ディストリビューションに適した方法で必要なパッケージをインストールしてください。[glfw](http://www.glfw.org/)はバージョン3.2.1以上である必要があります。

**Ubuntu / Debian:**

```sh
sudo apt install python3 python3-pip libglfw3 libportaudio2
sudo pip3 install pyxel
```

**Fedora:**

```sh
sudo dnf install glfw portaudio
sudo pip3 install pyxel
```

### インストールに失敗する場合

`pip`の以前のキャッシュの影響で、Pyxelのインストールに失敗するケースがあるようです。その場合は以下のオプションをつけて`pip`コマンドを実行してみてください。

**Windows:**

```sh
pip install --no-cache-dir --ignore-installed pyxel
```

**Mac:**

```sh
pip3 install --no-cache-dir --ignore-installed pyxel
```

**Linux:**

```sh
sudo pip3 install --no-cache-dir --ignore-installed pyxel
```

### サンプルのインストール

Pyxelインストール後に、以下のコマンドでカレントディレクトリにPyxelのサンプルコード一式をコピーできます。

```sh
install_pyxel_examples
```

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
        pyxel.rect(self.x, 0, self.x + 7, 7, 9)

App()
```

### 特殊操作

Pyxelアプリケーション実行中に、以下の特殊操作を行うことができます。

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

### Pyxel Editor

付属するPyxel EditorでPyxelアプリケーションで使用する画像やサウンドを作成することができます。

Pyxel Editorは以下のコマンドで起動します。

```sh
pyxeleditor [Pyxelリソースファイル]
```

指定したPyxelリソースファイル (.pyxel) が存在する場合は読み込み、存在しない場合は指定した名前で新規にファイルを作成します。リソースファイルを省略した場合は`my_resource.pyxel`がファイル名になります。

またPyxel Editorの起動後に、別のリソースファイルをドラッグ＆ドロップすることでファイルを切り替えることができます。

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

Pyxelは[PICO-8](https://www.lexaloffle.com/pico-8.php)と同じパレットを使用しているため、Pyxel向け配色のpngファイルを作成する場合は、[Aseprite](https://www.aseprite.org/)をPICO-8パレット設定にして使用するのがおすすめです。

Pyxel用のサウンドやミュージックは以下の方法で作成することもできます。

- `Sound.set`や`Music.set`関数で文字列から作成する

各関数の使い方はAPIリファレンスを参照してください。

## APIリファレンス

### システム

- `width`, `height`<br>
画面の幅と高さ

- `frame_count`<br>
経過フレーム数

- `init(width, height, [caption], [scale], [palette], [fps], [border_width], [border_color])`<br>
Pyxelアプリを画面サイズ (`width`, `height`) で初期化する。画面の最大の幅と高さは255<br>
`caption`でウィンドウタイトル、`scale`で表示倍率、`palette`でパレット色、`fps`で動作フレームレート、`border_width`と`border_color`で画面外側のマージン幅と色を指定できる。`palette`は24ビットカラーの16要素のリスト、`border_color`は24ビットカラーで指定する

- `run(update, draw)`<br>
Pyxelアプリを開始し、フレーム更新時に`update`関数、描画時に`draw`関数を呼ぶ

- `run_with_profiler(update, draw)`<br>
プロファイラ付きでPyxelアプリを開始し、フレーム更新時に`update`関数、描画時に`draw`関数を呼ぶ。アプリ終了時に各関数の処理時間を出力する

- `quit()`<br>
現在フレーム終了時にPyxelアプリを終了する

### リソース

- `save(filename)`<br>
実行スクリプトのディレクトリにリソースファイル (.pyxel) を保存する

- `load(filename)`<br>
実行スクリプトのディレクトリからリソースファイル (.pyxel) を読み込む

### 入力
- `mouse_x`, `mouse_y`<br>
現在のマウスカーソル座標

- `btn(key)`<br>
`key`が押されていたら`True`、押されていなければ`False`を返す ([キー定義一覧](https://github.com/kitao/pyxel/blob/master/pyxel/constants.py))

- `btnp(key, [hold], [period])`<br>
そのフレームに`key`が押されたら`True`、押されなければ`False`を返す。`hold`と`period`を指定すると、`hold`フレーム以上ボタンを押し続けた際に`period`フレーム間隔で`True`が返る

- `btnr(key)`<br>
そのフレームに`key`が離されたら`True`、離されなければ`False`を返す

- `mouse(visible)`<br>
`visible`が`True`ならマウスカーソルを表示し、`False`なら非表示にする。マウスカーソルが非表示でも座標は更新される

### グラフィックス

- `image(img, [system])`<br>
イメージバンク`img`(0-2) を操作する (イメージクラスを参照のこと)。`system`に`True`を指定すると、システム用のイメージバンク3にアクセスできる<br>
例：`pyxel.image(0).load(0, 0, 'title.png')`

- `tilemap(tm)`<br>
タイルマップ`tm`(0-7)を操作する (タイルマップクラスを参照のこと)

- `clip(x1, y1, x2, y2)`<br>
画面の描画領域を (`x1`, `y1`)-(`x2`, `y2`) にする。`clip()`で描画領域をリセットする

- `pal(col1, col2)`<br>
描画時に色`col1`を`col2`に置き換える。`pal()`で初期状態にリセットする

- `cls(col)`<br>
画面を色`col`でクリアする

- `pix(x, y, col)`<br>
(`x`, `y`) に色`col`のピクセルを描画する

- `line(x1, y1, x2, y2, col)`<br>
色`col`の直線を (`x1`, `y1`)-(`x2`, `y2`) に描画する

- `rect(x1, y1, x2, y2, col)`<br>
色`col`の矩形を (`x1`, `y1`)-(`x2`, `y2`) に描画する

- `rectb(x1, y1, x2, y2, col)`<br>
色`col`の矩形の輪郭線を (`x1`, `y1`)-(`x2`, `y2`) に描画する

- `circ(x, y, r, col)`<br>
半径`r`、色`col`の円を (`x`, `y`) に描画する

- `circb(x, y, r, col)`<br>
半径`r`、色`col`の円の輪郭線を (`x`, `y`) に描画する

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
イメージのデータ (NumPy配列)

- `get(x, y)`<br>
イメージの (`x`,`y`) のデータを取得する

- `set(x, y, data)`<br>
(`x`, `y`) に値または文字列のリストでイメージのデータを設定する<br>
例：`pyxel.image(0).set(10, 10, ['1234', '5678', '9abc', 'defg'])`

- `load(x, y, filename)`<br>
(`x`, `y`) に実行スクリプトのディレクトリからpngファイルを読み込む

- `copy(x, y, img, u, v, w, h)`<br>
イメージバンク`img`(0-2) の (`u`, `v`) からサイズ (`w`, `h`) の領域を (`x`, `y`) にコピーする

### タイルマップクラス

- `width`, `height`<br>
タイルマップの幅と高さ

- `data`<br>
タイルマップのデータ (NumPy配列)

- `refimg`<br>
タイルマップが参照するイメージバンク

- `get(x, y)`<br>
タイルマップの (`x`,`y`) のデータを取得する

- `set(x, y, data, [refimg])`<br>
(`x`, `y`) に値または文字列のリストでタイルマップのデータを設定する。`refimg`を指定すると、タイルマップが参照するイメージバンクも同時に設定される<br>
e.g. `pyxel.tilemap(0).set(0, 0, ['000102', '202122', 'a0a1a2', 'b0b1b2'], 1)`

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
例：`pyxel.sound(0).set_note('G2B-2D3R RF3F3F3')`

- `set_tone(tone)`<br>
'TSPN'の文字列で音色を設定する。大文字と小文字を区別せず、空白は無視される<br>
例：`pyxel.sound(0).set_tone('TTSS PPPN')`

- `set_volume(volume)`<br>
'01234567'の文字列で音量を設定する。大文字と小文字を区別せず、空白は無視される<br>
例：`pyxel.sound(0).set_volume('7777 7531')`

- `set_effect(effect)`<br>
'NSVF'の文字列でエフェクトを設定する。大文字と小文字を区別せず、空白は無視される<br>
例：`pyxel.sound(0).set_effect('NFNF NVVS')`

### ミュージッククラス

- `ch0`<br>
チャンネル0で再生するサウンド (0-63) のリスト

- `ch1`<br>
チャンネル1で再生するサウンド (0-63) のリスト

- `ch2`<br>
チャンネル2で再生するサウンド (0-63) のリスト

- `ch3`<br>
チャンネル3で再生するサウンド (0-63) のリスト

- `set(ch0, ch1, ch2, ch3)`<br>
全チャンネルのサウンド (0-63) のリストを設定する<br>
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

## ライセンス

Pyxelは[MITライセンス](http://en.wikipedia.org/wiki/MIT_License)です。ソースコードやライセンス表示用のファイル等で、[著作権とライセンス全文](https://raw.githubusercontent.com/kitao/pyxel/master/LICENSE)の表示を行えば、自由に販売や配布をすることができます。
