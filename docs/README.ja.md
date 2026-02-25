# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel**（ピクセル）は、Python 向けのレトロゲームエンジンです。

使える色は 16 色のみ、同時に再生できる音は 4 音までなど、レトロゲーム機を意識したシンプルな仕様で、Python を使ってドット絵スタイルのゲームづくりが気軽に楽しめます。

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

Pyxel 開発のモチベーションは、ユーザーの皆さんからのフィードバックです。GitHub で Pyxel へのスター登録をぜひお願いします！

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

Pyxel の仕様や API は、[PICO-8](https://www.lexaloffle.com/pico-8.php) や [TIC-80](https://tic80.com/) を参考にしています。

Pyxel は [MIT ライセンス](../LICENSE) のオープンソースで、無料で自由に使えます。Pyxel でレトロゲームづくりを始めましょう！

## 仕様

- Windows、Mac、Linux、Web で動作
- Python によるプログラミング
- 任意の画面サイズ
- 16 色パレット
- 256x256 サイズ、3 イメージバンク
- 256x256 サイズ、8 タイルマップ
- 4 音同時再生、定義可能な 64 サウンド
- 任意のサウンドを組み合わせ可能な 8 ミュージック
- キーボード、マウス、ゲームパッド入力
- 画像・サウンド編集ツール
- パレット、同時発音数、各種バンクのユーザー拡張

### カラーパレット

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## インストール方法

### Windows

[Python 3](https://www.python.org/) (バージョン 3.8 以上) をインストールした後、次のコマンドを実行します。

```sh
pip install -U pyxel
```

公式インストーラーで Python をインストールする場合は、`pyxel` コマンドを有効にするために、`Add Python 3.x to PATH` にチェックを入れて Python のインストールを行ってください。

### Mac

[Homebrew](https://brew.sh/) をインストールした後、次のコマンドを実行します。

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Pyxel をインストールした後にバージョンを更新する場合は、`pipx upgrade pyxel` を実行してください。

### Linux

[Python 3](https://www.python.org/) (バージョン 3.8 以上) をインストールした後、次のコマンドを実行します。

```sh
pip install -U pyxel
```

上記で動作しない場合は、[Makefile](../Makefile) に記載されている手順に従ってビルドを試してみてください。

### Web

Web 版 Pyxel は、Python や Pyxel をインストールすることなく、対応する Web ブラウザーがあれば PC だけでなく、スマートフォンやタブレットでも利用できます。

最も手軽な利用方法は、オンライン IDE の [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/) を利用する方法です。

自分のサイトに Pyxel アプリを埋め込む方法など、その他の利用パターンについては、[こちらのページ](pyxel-web-ja.md) を参照してください。

### サンプルを実行する

Pyxel をインストールした後、次のコマンドでカレントディレクトリに Pyxel のサンプルコードをコピーします。

```sh
pyxel copy_examples
```

サンプル一覧は[Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)からブラウザで確認・実行できます。

ローカル環境では以下のコマンドで実行できます。

```sh
# examples ディレクトリのサンプルを実行
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# examples/apps ディレクトリのアプリを実行
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

## 使い方

### アプリケーションの作成方法

Python スクリプト内で Pyxel モジュールをインポートし、`init` 関数でウィンドウサイズを指定した後、`run` 関数で Pyxel アプリケーションを開始します。

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

`run` 関数の引数には、フレーム更新処理を行う `update` 関数と、描画処理を行う `draw` 関数を指定します。

実際のアプリケーションでは、以下のようにクラスを使って Pyxel の処理をラップすることをおすすめします。

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

アニメーションのないシンプルなグラフィックスを作成する場合は、`show` 関数を使用してコードをより簡潔に記述できます。

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### アプリケーションの実行方法

作成した Python スクリプトは、`python` コマンドで実行できます。

```sh
python Pythonスクリプトファイル
```

`pyxel run` コマンドで実行することも可能です。

```sh
pyxel run Pythonスクリプトファイル
```

また、`pyxel watch` コマンドを使うと、指定したディレクトリ内の変更を監視し、変化があった際に自動でプログラムを再実行します。

```sh
pyxel watch WATCH_DIR Pythonスクリプトファイル
```

ディレクトリの監視は、`Ctrl(Command)+C` で終了します。

### 特殊キー操作

Pyxel アプリケーションの実行中に、以下の特殊キー操作を行うことができます。

- `Esc`<br>
  アプリケーションを終了する
- `Alt(Option)+R` またはゲームパッドで `A+B+X+Y+BACK`<br>
  アプリケーションをリセットする
- `Alt(Option)+1`<br>
  スクリーンショットをデスクトップに保存する
- `Alt(Option)+2`<br>
  画面キャプチャ動画の録画開始時刻をリセットする
- `Alt(Option)+3`<br>
  画面キャプチャ動画をデスクトップに保存する (最大 10 秒)
- `Alt(Option)+8` またはゲームパッドで `A+B+X+Y+DL`<br>
  画面の拡大方法を最大と整数倍で切り替える
- `Alt(Option)+9` またはゲームパッドで `A+B+X+Y+DR`<br>
  画面モード (Crisp/Smooth/Retro) を切り替える
- `Alt(Option)+0` またはゲームパッドで `A+B+X+Y+DU`<br>
  パフォーマンスモニタ (FPS/`update` 時間/`draw` 時間) の表示を切り替える
- `Alt(Option)+Enter` またはゲームパッドで `A+B+X+Y+DD`<br>
  フルスクリーン表示を切り替える
- `Shift+Alt(Option)+1/2/3`<br>
  イメージバンク 0, 1, 2 をデスクトップに保存する
- `Shift+Alt(Option)+0`<br>
  現在のカラーパレットをデスクトップに保存する

### リソースの作成方法

Pyxel Editor を使用して、Pyxel アプリケーションで使用する画像やサウンドを作成できます。

Pyxel Editor は次のコマンドで起動します。

```sh
pyxel edit Pyxelリソースファイル
```

指定した Pyxel リソースファイル (.pyxres) が存在する場合は読み込み、存在しない場合は指定した名前で新規ファイルを作成します。リソースファイルを省略した場合は、`my_resource.pyxres` というファイル名になります。

Pyxel Editor の起動後、別のリソースファイルをドラッグ＆ドロップするとファイルを切り替えることができます。

作成したリソースファイルは、`load` 関数で読み込めます。

Pyxel Editor には、以下の編集モードがあります。

**イメージエディタ**

**イメージバンク**の画像を編集する画面です。

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_editor.gif">
</a>

イメージエディタに画像ファイル (PNG/GIF/JPEG) をドラッグ＆ドロップすると、選択中のイメージバンクに画像を読み込みます。

**タイルマップエディタ**

イメージバンクの画像をタイル状に並べた**タイルマップ**を編集する画面です。

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/tilemap-editor.html">
<img src="images/tilemap_editor.gif">
</a>

タイルマップエディタに TMX ファイル (Tiled Map File) をドラッグ＆ドロップすると、選択中のタイルマップにレイヤー 0 を読み込みます。

**サウンドエディタ**

メロディーや効果音に使用する**サウンド**を編集する画面です。

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
<img src="images/sound_editor.gif">
</a>

**ミュージックエディタ**

サウンドを再生順に並べた**ミュージック**を編集する画面です。

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/music-editor.html">
<img src="images/music_editor.gif">
</a>

### その他のリソース作成方法

Pyxel 用の画像やタイルマップは、以下の方法で作成することもできます。

- `Image.set` や `Tilemap.set` 関数を使って、文字列のリストから作成する
- `Image.load` 関数を使って、Pyxel 向け配色の画像ファイル (PNG/GIF/JPEG) を読み込む

Pyxel 用のサウンドやミュージックは、以下の方法で作成することもできます。

- `Sound.set` や `Music.set` 関数を使って、文字列から作成する

各関数の使い方については API リファレンスを参照してください。

### アプリケーションの配布方法

Pyxel ではプラットフォームによらず動作する、専用のアプリケーション配布ファイル形式 (Pyxel アプリケーションファイル) をサポートしています。

Pyxel アプリケーションファイル (.pyxapp) は、`pyxel package` コマンドで作成します。

```sh
pyxel package アプリケーションのディレクトリ 起動スクリプトファイル
```

リソースや追加モジュールを同梱する場合は、アプリケーションのディレクトリ内に配置します。

起動スクリプトに次の形式でメタデータを記載すると、実行時に表示されます。`title`、`author` 以外のフィールドは省略可能です。

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

作成したアプリケーションファイルは、`pyxel play` コマンドで実行します。

```sh
pyxel play Pyxelアプリケーションファイル
```

Pyxel アプリケーションファイルは、`pyxel app2exe` コマンドや `pyxel app2html` コマンドで、実行可能ファイルや HTML ファイルに変換できます。

## API リファレンス

### システム

- `width`, `height`<br>
  画面の幅と高さ

- `frame_count`<br>
  経過フレーム数

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Pyxel アプリケーションを画面サイズ (`width`, `height`) で初期化します。`title` でウィンドウタイトル、`fps` で動作フレームレート、`quit_key` でアプリケーション終了キー、`display_scale` で画面表示の倍率、`capture_scale` で画面キャプチャの倍率、`capture_sec` で画面キャプチャ動画の最大録画時間を指定します。<br>
  例：`pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Pyxel アプリケーションを開始し、フレーム更新時に `update` 関数、描画時に `draw` 関数を呼びます。

- `show()`<br>
  画面を表示し、`Esc` キーが押されるまで待機します。

- `flip()`<br>
  画面を 1 フレーム更新します。`Esc` を押すとアプリケーションは終了します。この関数は Web 版では動作しません。

- `quit()`<br>
  Pyxel アプリケーションを終了します。

- `reset()`<br>
  Pyxel アプリケーションをリセットします。リセット後も環境変数は保持されます。

### リソース

- `load(filename, [exclude_images], [exclude_tilemaps], [exclude_sounds], [exclude_musics])`<br>
  リソースファイル (.pyxres) を読み込みます。オプションに `True` を指定すると、そのリソースは読み込まれません。また、同名のパレットファイル (.pyxpal) がリソースファイルと同じ場所に存在する場合は、パレットの表示色も変更されます。パレットファイルは、表示色を改行区切りの 16 進数 (例：`1100ff`) で入力します。パレットファイルを使うことで Pyxel Editor の表示色も変更可能です。

- `user_data_dir(vendor_name, app_name)`<br>
  `vendor_name` と `app_name` から生成されたユーザーデータ保存用ディレクトリを返します。該当ディレクトリが存在しない場合は自動で作成されます。ハイスコアやゲームの進行状況の保存先として使用します。<br>
  例：`print(pyxel.user_data_dir("Takashi Kitao", "Pyxel Shooter"))`

### 入力

- `mouse_x`, `mouse_y`<br>
  現在のマウスカーソル座標

- `mouse_wheel`<br>
  現在のマウスホイールの値

- `btn(key)`<br>
  `key` が押されていたら `True`、押されていなければ `False` を返します。([キー定義一覧](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  そのフレームに `key` が押されたら `True`、押されなければ `False` を返します。`hold` と `repeat` を指定すると、`hold` フレーム以上ボタンを押し続けた時に `repeat` フレーム間隔で `True` が返ります。

- `btnr(key)`<br>
  そのフレームに `key` が離されたら `True`、離されなければ `False` を返します。

- `mouse(visible)`<br>
  `visible` が `True` ならマウスカーソルを表示し、`False` なら非表示にします。マウスカーソルが非表示でも座標は更新されます。

### グラフィックス

- `colors`<br>
  パレットの表示色リスト。表示色は 24 ビット数値で指定します。Python リストと同様の操作で、表示色の追加や削除、一括変更が行えます。<br>
  例：`old_colors = list(pyxel.colors); pyxel.colors[:] = [0x111111, 0x222222, 0x333333]; pyxel.colors[15] = 0x112233`

- `images`<br>
  イメージバンク (Image クラスのインスタンス) のリスト (0-2)<br>
  例：`pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  タイルマップ (Tilemap クラスのインスタンス) のリスト (0-7)

- `clip(x, y, w, h)`<br>
  画面の描画領域を (`x`, `y`) から幅 `w`、高さ `h` に設定します。`clip()` で描画領域を全画面にリセットします。

- `camera(x, y)`<br>
  画面の左上隅の座標を (`x`, `y`) に変更します。`camera()` で左上隅の座標を (`0`, `0`) にリセットします。

- `pal(col1, col2)`<br>
  描画時に色 `col1` を `col2` に置き換えます。`pal()` で初期状態にリセットします。

- `dither(alpha)`<br>
  描画時にディザリング (擬似半透明) を適用します。`alpha` は `0.0`-`1.0` の範囲で設定し、`0.0` が透明、`1.0` が不透明になります。

- `cls(col)`<br>
  画面を色 `col` でクリアします。

- `pget(x, y)`<br>
  (`x`, `y`) のピクセルの色を取得します。

- `pset(x, y, col)`<br>
  (`x`, `y`) に色 `col` のピクセルを描画します。

- `line(x1, y1, x2, y2, col)`<br>
  色 `col` の直線を (`x1`, `y1`)-(`x2`, `y2`) に描画します。

- `rect(x, y, w, h, col)`<br>
  幅 `w`、高さ `h`、色 `col` の矩形を (`x`, `y`) に描画します。

- `rectb(x, y, w, h, col)`<br>
  幅 `w`、高さ `h`、色 `col` の矩形の輪郭線を (`x`, `y`) に描画します。

- `circ(x, y, r, col)`<br>
  半径 `r`、色 `col` の円を (`x`, `y`) に描画します。

- `circb(x, y, r, col)`<br>
  半径 `r`、色 `col` の円の輪郭線を (`x`, `y`) に描画します。

- `elli(x, y, w, h, col)`<br>
  幅 `w`、高さ `h`、色 `col` の楕円を (`x`, `y`) に描画します。

- `ellib(x, y, w, h, col)`<br>
  幅 `w`、高さ `h`、色 `col` の楕円の輪郭線を (`x`, `y`) に描画します。

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  頂点が (`x1`, `y1`)、(`x2`, `y2`)、(`x3`, `y3`)、色 `col` の三角形を描画します。

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  頂点が (`x1`, `y1`)、(`x2`, `y2`)、(`x3`, `y3`)、色 `col` の三角形の輪郭線を描画します。

- `fill(x, y, col)`<br>
  (`x`, `y`) と同じ色でつながっている領域を色 `col` で塗りつぶします。

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  イメージバンク `img`(0-2) の (`u`, `v`) からサイズ (`w`, `h`) の領域を (`x`, `y`) にコピーします。`w`、`h` それぞれに負の値を設定すると水平、垂直方向に反転します。`colkey` に色を指定すると透明色として扱われます。`rotate`(度:Degree)、`scale`(1.0=100%)、またはその両方を指定すると対応する変換が適用されます。

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  タイルマップ `tm`(0-7) の (`u`, `v`) からサイズ (`w`, `h`) の領域を (`x`, `y`) にコピーします。`w`、`h` それぞれに負の値を設定すると水平、垂直方向に反転します。`colkey` に色を指定すると透明色として扱われます。`rotate`(度:Degree)、`scale`(1.0=100%)、またはその両方を指定すると対応する変換が適用されます。1 タイルのサイズは 8x8 ピクセルで、`(image_tx, image_ty)` のタプルとしてタイルマップに格納されています。

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  色 `col` の文字列 `s` を (`x`, `y`) に描画します。

### オーディオ

- `sounds`<br>
  サウンド (Sound クラスのインスタンス) のリスト (0-63)<br>
  例：`pyxel.sounds[0].speed = 60`

- `musics`<br>
  ミュージック (Music クラスのインスタンス) のリスト (0-7)

- `play(ch, snd, [sec], [loop], [resume])`<br>
  チャンネル `ch`(0-3) でサウンド `snd`(0-63) を再生します。`snd` にはサウンド番号、複数サウンドのリスト、または MML 文字列を指定できます。`sec` で再生開始位置を秒単位で指定できます。`loop` に `True` を指定するとループ再生します。再生終了後に以前の音へ復帰させたい場合は `resume` に `True` を指定します。

- `playm(msc, [sec], [loop])`<br>
  ミュージック `msc`(0-7) を再生します。`sec` で再生開始位置を秒単位で指定できます。`loop` に `True` を指定するとループ再生します。

- `stop([ch])`<br>
  指定したチャンネル `ch`(0-3) の再生を停止します。`stop()` で全チャンネルの再生を停止します。

- `play_pos(ch)`<br>
  チャンネル `ch`(0-3) のサウンド再生位置を `(sound_no, sec)` のタプルとして取得します。再生停止時は `None` を返します。

- `gen_bgm(preset, instr, [seed], [play])`<br>
  [8bit BGM generator](https://github.com/shiromofufactory/8bit-bgm-generator) をベースにしたアルゴリズムで BGM の MML リストを生成します。`preset` はプリセット番号（0-7）、`instr` は編成番号（0-3）で、`0`=メロディ+リバーブ+ベース、`1`=メロディ+ベース+ドラム、`2`=メロディ+サブ+ベース、`3`=メロディ+サブ+ベース+ドラム です。`seed` を指定しない場合はランダムになります。`play` に `True` を指定すると生成した MML を再生します。

### 数学

- `ceil(x)`<br>
  `x` 以上の最小の整数を返します。

- `floor(x)`<br>
  `x` 以下の最大の整数を返します。

- `clamp(x, lower, upper)`<br>
  `lower` を最小、`upper` を最大として、`x` をその範囲に収めた値を返します。

- `sgn(x)`<br>
  `x` が正の時に `1`、`0` の時に `0`、負の時に `-1` を返します。

- `sqrt(x)`<br>
  `x` の平方根を返します。

- `sin(deg)`<br>
  `deg` 度 (Degree) の正弦を返します。

- `cos(deg)`<br>
  `deg` 度 (Degree) の余弦を返します。

- `atan2(y, x)`<br>
  `y`/`x` の逆正接を度 (Degree) で返します。

- `rseed(seed)`<br>
  乱数生成器のシードを設定します。

- `rndi(a, b)`<br>
  `a` 以上 `b` 以下のランダムな整数を返します。

- `rndf(a, b)`<br>
  `a` 以上 `b` 以下のランダムな小数を返します。

- `nseed(seed)`<br>
  Perlin ノイズのシードを設定します。

- `noise(x, [y], [z])`<br>
  指定された座標の Perlin ノイズ値を返します。

### Image クラス

- `width`, `height`<br>
  イメージの幅と高さ

- `set(x, y, data)`<br>
  (`x`, `y`) に文字列のリストでイメージを設定します。<br>
  例：`pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  (`x`, `y`) に画像ファイル (PNG/GIF/JPEG) を読み込みます。

- `pget(x, y)`<br>
  (`x`, `y`) のピクセルの色を取得します。

- `pset(x, y, col)`<br>
  (`x`, `y`) に色 `col` のピクセルを描画します。

### Tilemap クラス

- `width`, `height`<br>
  タイルマップの幅と高さ

- `imgsrc`<br>
  タイルマップが参照するイメージバンク(0-2)

- `set(x, y, data)`<br>
  (`x`, `y`) に文字列のリストでタイルマップを設定します。<br>
  例：`pyxel.tilemaps[0].set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  (`x`, `y`) に TMX ファイル (Tiled Map File) の `layer`(0-)を読み込みます。

- `pget(x, y)`<br>
  (`x`, `y`) のタイルを取得します。タイルは `(image_tx, image_ty)` のタプルです。

- `pset(x, y, tile)`<br>
  (`x`, `y`) にタイルを設定します。タイルは `(image_tx, image_ty)` のタプルです。

- `collide(x, y, w, h, dx, dy, walls)`<br>
  矩形の位置 (`x`, `y`) とサイズ (`w`, `h`) に移動量 (`dx`, `dy`) を適用した衝突を解決し、補正後の (`dx`, `dy`) を返します。`walls` には壁として扱うタイル `(image_tx, image_ty)` のリストを渡します。

### Sound クラス

- `notes`<br>
  音程 (0-127) のリスト。数値が大きいほど音程は高くなり、`33` で 'A2' (440 Hz) になります。休符は `-1` です。

- `tones`<br>
  音色 (0:Triangle / 1:Square / 2:Pulse / 3:Noise) のリスト

- `volumes`<br>
  音量 (0-7) のリスト

- `effects`<br>
  エフェクト (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut) のリスト

- `speed`<br>
  再生速度。`1` が最も速く、数値が大きいほど再生速度は遅くなります。`120` で 1 音の長さが 1 秒になります。

- `set(notes, tones, volumes, effects, speed)`<br>
  文字列で音程、音色、音量、エフェクトを設定します。音色、音量、エフェクトの長さが音程より短い場合は、先頭から繰り返されます。

- `set_notes(notes)`<br>
  `CDEFGAB`+`#-`+`01234` または `R` の文字列で音程を設定します。大文字と小文字は区別されず、空白は無視されます。<br>
  例：`pyxel.sounds[0].set_notes("g2b-2d3r rf3f3f3")`

- `set_tones(tones)`<br>
  `TSPN` の文字列で音色を設定します。大文字と小文字は区別されず、空白は無視されます。<br>
  例：`pyxel.sounds[0].set_tones("ttss pppn")`

- `set_volumes(volumes)`<br>
  `01234567` の文字列で音量を設定します。大文字と小文字は区別されず、空白は無視されます。<br>
  例：`pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  `NSVFHQ` の文字列でエフェクトを設定します。大文字と小文字は区別されず、空白は無視されます。<br>
  例：`pyxel.sounds[0].set_effects("nfnf nvvs")`

- `mml(code)`<br>
  [MML (Music Macro Language)](https://ja.wikipedia.org/wiki/Music_Macro_Language) の文字列を渡すと MML モードに移行し、その内容に沿ってサウンドが再生されます。MML モードでは `notes` や `speed` などの通常のパラメータは無視され、`mml()` で解除できます。MML の詳細は、[こちらのページ](faq-ja.md) を参照してください。<br>
  例：`pyxel.sounds[0].mml("T120 Q90 @1 V100 O5 L8 C4&C<G16R16>C.<G16 >C.D16 @VIB1{10,20,20} E2C2")`

- `pcm(filename)`<br>
  オーディオファイル (WAV/OGG) を読み込んで再生に使用します。`pcm()` で通常の再生モードに戻します。<br>
  例：`pyxel.sounds[0].pcm("sounds/bgm.ogg")`

- `save(filename, sec, [ffmpeg])`<br>
  サウンドを指定した秒数分再生した WAV ファイルを作成します。FFmpeg がインストールされている環境で、`ffmpeg` に `True` を指定すると、MP4 ファイルも作成します。

- `total_sec()`<br>
  サウンドの再生時間を秒で返します。MML で無限ループが使用されている場合は `None` を返します。

### Music クラス

- `seqs`<br>
  サウンド (0-63) のリストをチャンネル数分連ねた 2 次元リスト

- `set(seq0, seq1, seq2, ...)`<br>
  チャンネルのサウンド (0-63) のリストを設定します。空リストを指定すると、そのチャンネルは再生に使用されません。<br>
  例：`pyxel.musics[0].set([0, 1], [], [3])`

- `save(filename, sec, [ffmpeg])`<br>
  ミュージックを指定した秒数分再生した WAV ファイルを作成します。FFmpeg がインストールされている環境で、`ffmpeg` に `True` を指定すると、MP4 ファイルも作成します。

### 上級者向け API

Pyxel には、ユーザーを混乱させる可能性や、使用に専門知識が必要といった理由から、このリファレンスには記載していない「上級者向け API」があります。

腕に覚えのある方は、[こちら](../python/pyxel/__init__.pyi) を手がかりに、あっと驚くような作品づくりに挑戦してみてください！

## コントリビューション方法

### 問題の報告

不具合の報告や機能の要望は [Issue Tracker](https://github.com/kitao/pyxel/issues) で受け付けています。新しいレポートを作成する前に、同じ内容のものがないか確認をお願いします。

### 動作確認

動作確認を行い、[Issue Tracker](https://github.com/kitao/pyxel/issues) で不具合の報告や改善の提案をしていただける方は大歓迎です！

### プルリクエスト

パッチや修正はプルリクエスト (PR) として受け付けています。提出前に、問題がすでに解決済みでないか [Issue Tracker](https://github.com/kitao/pyxel/issues) で確認をお願いします。

提出されたプルリクエストは、[MIT ライセンス](../LICENSE) で公開することに同意したものと見なされます。

## Web ツール＆サンプル

- [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/) [[ユーザーマニュアル](https://qiita.com/kitao/items/6f3d080f8e1c5d2f2715)]
- [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/) [[ユーザーマニュアル](https://qiita.com/kitao/items/01156ae7ade59d8ff2cc)]

## その他の情報

- [よくある質問](faq-ja.md)
- [ユーザー作品集](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [開発者 X アカウント](https://x.com/kitao)
- [Discord サーバー（英語）](https://discord.gg/Z87eYHN)
- [Discord サーバー（日本語）](https://discord.gg/qHA5BCS)
- [書籍『ゲームで学ぶ Python！ Pyxel ではじめるレトロゲームプログラミング』](https://gihyo.jp/book/2025/978-4-297-14657-3)

## ライセンス

Pyxel は [MIT ライセンス](../LICENSE) です。ソースコードやライセンス表示用のファイル等で、著作権とライセンス全文の表示をすれば、自由に販売や配布できます。

## スポンサー募集

Pyxel は GitHub Sponsors でスポンサーを募っています。Pyxel のメンテナンスと機能追加の継続のために、スポンサーになることをご検討ください。スポンサーは特典として Pyxel についての相談が可能です。詳細は [こちら](https://github.com/sponsors/kitao) をご覧ください。
