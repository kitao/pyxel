# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel (ピクセル)** は Python 向けのレトロゲームエンジンです。

使える色は 16 色のみ、同時に再生できる音は 4 音までなど、レトロゲーム機を意識したシンプルな仕様で、Python でドット絵スタイルのゲームづくりが気軽に楽しめます。

<img src="images/pyxel_message.png" width="480">

Pyxel 開発のモチベーションはユーザーの皆さんからのフィードバックです。GitHub で Pyxel へのスター登録をよろしくお願いします！

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

Pyxel の仕様や API は、[PICO-8](https://www.lexaloffle.com/pico-8.php)や[TIC-80](https://tic80.com/)を参考にしています。

Pyxel はオープンソースで、無料で自由に使えます。Pyxel でレトロゲームづくりを始めましょう！

## 仕様

- Windows、Mac、Linux、Web で動作
- Python によるプログラミング
- 16 色パレット
- 256x256 サイズ、3 画像バンク
- 256x256 サイズ、8 タイルマップ
- 4 音同時再生、定義可能な 64 サウンド
- 任意のサウンドを組み合わせ可能な 8 ミュージック
- キーボード、マウス、ゲームパッド
- 画像・サウンド編集ツール

### カラーパレット

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## インストール方法

### Windows

[Python3](https://www.python.org/) (バージョン 3.7 以上) をインストールした後に、次のコマンドを実行します。

```sh
pip install -U pyxel
```

公式のインストーラーで Python をインストールする場合は、`pyxel`コマンドを有効にするために、`Add Python 3.x to PATH`にチェックをつけて Python のインストールを行なってください。

### Mac

[Homebrew](https://brew.sh/) をインストールした後に、次のコマンドを実行します。

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Pyxel インストール後にバージョンを更新する場合は、`pipx upgrade pyxel`を実行してください。

### Linux

SDL2 パッケージ (Ubuntu の場合は`libsdl2-dev`)、[Python3](https://www.python.org/) (バージョン 3.7 以上)、`python3-pip`をインストールした後に、次のコマンドを実行します。

```sh
sudo pip3 install -U pyxel
```

上記で動作しない場合は、[Makefile](../Makefile)に記載されている手順に従ってセルフビルドを試してみてください。

### Web

Web 版の Pyxel は、Python や Pyxel のインストールを必要とせず、PC だけではなく、対応する Web ブラウザーを持つスマートフォンやタブレット端末でも動作します。

具体的な使い方については、[こちらのページ](<https://github.com/kitao/pyxel/wiki/How-To-Use-Pyxel-Web-(%E6%97%A5%E6%9C%AC%E8%AA%9E)>)を参照してください。

### サンプルを実行する

Pyxel のインストール後に、次のコマンドでカレントディレクトリに Pyxel のサンプルコードがコピーされます。

```sh
pyxel copy_examples
```

コピーされるサンプルは以下の通りです。

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>シンプルなアプリケーション</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Code</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Pyxel リソースファイルを使ったジャンプゲーム</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Code</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>描画 API のデモ</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Code</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>サウンド API のデモ</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Code</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>カラーパレット一覧</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Code</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>マウスクリックゲーム</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Code</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>BGM 付きスネークゲーム</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Code</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>三角形描画 API のデモ</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Code</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>画面遷移のあるシューティングゲーム</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Code</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>マップのある横スクロールアクションゲーム</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Code</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Image クラスによるオフスクリーン描画</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Code</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>パーリンノイズアニメーション</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Code</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>ビットマップフォント描画</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Code</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>オーディオ拡張機によるシンセサイザー</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Code</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>タイルマップファイル (.tmx) の読み込みと描画</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Code</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>flip 関数によるアニメーション (非 Web 環境のみ)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Code</a></td>
</tr>
<tr>
<td>30SecondsOfDaylight.pyxapp</td>
<td>第 1 回 Pyxel Jam 優勝ゲーム (<a href="https://twitter.com/helpcomputer0">Adam</a>制作)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30SecondsOfDaylight.html">Demo</a></td>
<td><a href="https://github.com/kitao/30SecondsOfDaylight">Code</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>アーケードボール物理ゲーム (<a href="https://twitter.com/helpcomputer0">Adam</a>制作)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Demo</a></td>
<td><a href="https://github.com/helpcomputer/megaball">Code</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>BGM自動作成ツール (<a href="https://twitter.com/frenchbread1222">frenchbread</a>制作)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">Demo</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">Code</a></td>
</tr>
</table>

サンプルは以下のコマンドで実行できます。

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30SecondsOfDaylight.pyxapp
```

## 使い方

### アプリケーションの作成方法

Python スクリプト内で Pyxel モジュールをインポートして、`init`関数でウィンドウサイズを指定した後に、`run`関数で Pyxel アプリケーションを開始します。

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

実際のアプリケーションでは、以下のようにクラスで Pyxel の処理をラップするのがおすすめです。

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

アニメーションのないシンプルなグラフィックスを作成する場合は、`show`関数を使用してコードをより簡潔に記述することも可能です。

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### アプリケーションの実行方法

作成した Python スクリプトは次のコマンドで実行できます。

```sh
pyxel run Pythonスクリプトファイル
```

また、通常の Python スクリプトと同様に実行することもできます。

```sh
python3 Pythonスクリプトファイル
```

### 特殊操作

Pyxel アプリケーション実行中に、以下の特殊操作を行うことができます。

- `Esc`<br>
  アプリケーションを終了する
- `Alt(Option)+1`<br>
  スクリーンショットをデスクトップに保存する
- `Alt(Option)+2`<br>
  画面キャプチャ動画の録画開始時刻をリセットする
- `Alt(Option)+3`<br>
  画面キャプチャ動画をデスクトップに保存する (最大 10 秒)
- `Alt(Option)+9`<br>
  画面モード (Crisp/Smooth/Retro) を切り替える
- `Alt(Option)+0`<br>
  パフォーマンスモニタ (fps、update 時間、draw 時間) の表示を切り替える
- `Alt(Option)+Enter`<br>
  フルスクリーン表示を切り替える
- `Shift+Alt(Option)+1/2/3`<br>
  対応するイメージバンクをデスクトップに保存する
- `Shift+Alt(Option)+0`<br>
  現在のカラーパレットをデスクトップに保存する

### リソースの作成方法

Pyxel Editor を使って、Pyxel アプリケーションで使用する画像やサウンドを作成することができます。

Pyxel Editor は次のコマンドで起動します。

```sh
pyxel edit Pyxelリソースファイル
```

指定した Pyxel リソースファイル (.pyxres) が存在する場合は読み込み、存在しない場合は指定した名前で新規にファイルを作成します。リソースファイルを省略した場合は`my_resource.pyxres`がファイル名になります。

Pyxel Editor の起動後に、別のリソースファイルをドラッグ＆ドロップすることでファイルを切り替えることができます。

作成したリソースファイルは Pyxel アプリケーションから`load`関数で読み込めます。

Pyxel Editor には以下の編集モードがあります。

**イメージエディタ**

イメージバンクの画像を編集する画面です。

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

イメージエディタに画像ファイル (PNG/GIF/JPEG) をドラッグ＆ドロップすると、選択中のイメージバンクに画像を読み込むことができます。

**タイルマップエディタ**

イメージバンクの画像をタイル状に並べたタイルマップを編集する画面です。

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

タイルマップエディタに TMX ファイル (Tiled Map File) をドラッグ＆ドロップすると、選択中のタイルマップ番号に対応した描画順のレイヤーを読み込むことができます。

**サウンドエディタ**

サウンドを編集する画面です。

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**ミュージックエディタ**

サウンドを再生順に並べたミュージックを編集する画面です。

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### その他のリソース作成方法

Pyxel 用の画像やタイルマップは以下の方法で作成することもできます。

- `Image.set`や`Tilemap.set`関数で文字列のリストから作成する
- `Image.load`関数で Pyxel 向け配色の画像ファイル (PNG/GIF/JPEG) を読み込む

Pyxel 用のサウンドやミュージックは以下の方法で作成することもできます。

- `Sound.set`や`Music.set`関数で文字列から作成する

各関数の使い方は API リファレンスを参照してください。

### アプリケーションの配布方法

Pyxel ではプラットフォームによらず動作する、専用のアプリケーション配布ファイル形式 (Pyxel アプリケーションファイル) をサポートしています。

Pyxel アプリケーションファイル (.pyxapp) は次のコマンドで作成します。

```sh
pyxel package アプリケーションのディレクトリ 起動スクリプトファイル
```

リソースや追加モジュールを同梱する場合は、アプリケーションのディレクトリ内に配置してください。

作成したアプリケーションファイルは以下のコマンドで実行します。

```sh
pyxel play Pyxelアプリケーションファイル
```

Pyxel アプリケーションファイルは、`pyxel app2exe`コマンドや`pyxel app2html`コマンドで、実行可能ファイルや HTML ファイルに変換することもできます。

## API リファレンス

### システム

- `width`, `height`<br>
  画面の幅と高さ

- `frame_count`<br>
  経過フレーム数

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Pyxel アプリケーションを画面サイズ (`width`, `height`) で初期化します。`title`でウィンドウタイトル、`fps`で動作フレームレート、`quit_key`でアプリケーション終了キー、`display_scale`で画面表示の倍率、`capture_scale`で画面キャプチャの倍率、`capture_sec`で画面キャプチャ動画の最大録画時間を指定します。<br>
  例：`pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Pyxel アプリケーションを開始し、フレーム更新時に`update`関数、描画時に`draw`関数を呼びます。

- `show()`<br>
  画面を表示して`Esc`キーが押されるまで待機します。

- `flip()`<br>
  画面を 1 フレーム更新します。`ESC`を押すとアプリケーションは終了します。この関数は Web 版では動作しません。

- `quit()`<br>
  Pyxel アプリケーションを終了します。

### リソース

- `load(filename, [excl_images], [excl_tilemaps], [excl_sounds], [excl_musics])`<br>
  リソースファイル (.pyxres) を読み込みます。オプションに`True`を指定すると、そのリソースは読み込まれません。また、同名のパレットファイル (.pyxpal) がリソースファイルと同じ場所に存在する場合は、パレットの表示色も変更されます。パレットファイルは表示色を改行区切りの 16 進数 (例：`1100FF`) で入力します。パレットファイルを使うことで Pyxel Editor の表示色も変更可能です。

### 入力

- `mouse_x`, `mouse_y`<br>
  現在のマウスカーソル座標

- `mouse_wheel`<br>
  現在のマウスホイールの値

- `btn(key)`<br>
  `key`が押されていたら`True`、押されていなければ`False`を返します。([キー定義一覧](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  そのフレームに`key`が押されたら`True`、押されなければ`False`を返します。`hold`と`repeat`を指定すると、`hold`フレーム以上ボタンを押し続けた時に`repeat`フレーム間隔で`True`が返ります。

- `btnr(key)`<br>
  そのフレームに`key`が離されたら`True`、離されなければ`False`を返します。

- `mouse(visible)`<br>
  `visible`が`True`ならマウスカーソルを表示し、`False`なら非表示にします。マウスカーソルが非表示でも座標は更新されます。

### グラフィックス

- `colors`<br>
  パレットの表示色リスト。表示色は 24 ビット数値で指定します。Python リストを直接代入、取得する場合は`colors.from_list`と`colors.to_list`を使用してください。<br>
  例：`old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  イメージバンク (0-2) のリスト。(イメージクラスを参照のこと)<br>
  例：`pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  タイルマップ (0-7) のリスト。(タイルマップクラスを参照のこと)

- `clip(x, y, w, h)`<br>
  画面の描画領域を (`x`, `y`) から幅`w`、高さ`h`に設定します。`clip()`で描画領域を全画面にリセットします。

- `camera(x, y)`<br>
  画面の左上隅の座標を (`x`, `y`) に変更します。`camera()`で左上隅の座標を (`0`, `0`) にリセットします。

- `pal(col1, col2)`<br>
  描画時に色`col1`を`col2`に置き換えます。`pal()`で初期状態にリセットします。

- `dither(alpha)`<br>
  描画時にディザリング (擬似半透明) を適用します。`alpha`は 0.0-1.0 の範囲で設定し、0.0 が透明、1.0 が不透明になります。

- `cls(col)`<br>
  画面を色`col`でクリアします。

- `pget(x, y)`<br>
  (`x`, `y`) のピクセルの色を取得します。

- `pset(x, y, col)`<br>
  (`x`, `y`) に色`col`のピクセルを描画します。

- `line(x1, y1, x2, y2, col)`<br>
  色`col`の直線を (`x1`, `y1`)-(`x2`, `y2`) に描画します。

- `rect(x, y, w, h, col)`<br>
  幅`w`、高さ`h`、色`col`の矩形を (`x`, `y`) に描画します。

- `rectb(x, y, w, h, col)`<br>
  幅`w`、高さ`h`、色`col`の矩形の輪郭線を (`x`, `y`) に描画します。

- `circ(x, y, r, col)`<br>
  半径`r`、色`col`の円を (`x`, `y`) に描画します。

- `circb(x, y, r, col)`<br>
  半径`r`、色`col`の円の輪郭線を (`x`, `y`) に描画します。

- `elli(x, y, w, h, col)`<br>
  幅`w`、高さ`h`、色`col`の楕円を (`x`, `y`) に描画します。

- `ellib(x, y, w, h, col)`<br>
  幅`w`、高さ`h`、色`col`の楕円の輪郭線を (`x`, `y`) に描画します。

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  頂点が (`x1`, `y1`)、(`x2`, `y2`)、(`x3`, `y3`)、色`col`の三角形を描画します。

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  頂点が (`x1`, `y1`)、(`x2`, `y2`)、(`x3`, `y3`)、色`col`の三角形の輪郭線を描画します。

- `fill(x, y, col)`<br>
  (`x`, `y`) と同じ色でつながっている領域を色`col`で塗りつぶします。

- `blt(x, y, img, u, v, w, h, [colkey])`<br>
  イメージバンク`img`(0-2) の (`u`, `v`) からサイズ (`w`, `h`) の領域を (`x`, `y`) にコピーします。`w`、`h`それぞれに負の値を設定すると水平、垂直方向に反転します。`colkey`に色を指定すると透明色として扱われます。

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey])`<br>
  タイルマップ`tm`(0-7) の (`u`, `v`) からサイズ (`w`, `h`) の領域を (`x`, `y`) にコピーします。`w`、`h`それぞれに負の値を設定すると水平、垂直方向に反転します。`colkey`に色を指定すると透明色として扱われます。1 タイルのサイズは 8x8 ピクセルで、`(tile_x, tile_y)`のタプルとしてタイルマップに格納されています。

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  色`col`の文字列`s`を (`x`, `y`) に描画します。

### オーディオ

- `sounds`<br>
  サウンド (0-63) のリスト。(サウンドクラスを参照のこと)<br>
  例：`pyxel.sounds[0].speed = 60`

- `musics`<br>
  ミュージック (0-7) のリスト。(ミュージッククラスを参照のこと)

- `play(ch, snd, [tick], [loop], [resume])`<br>
  チャンネル`ch`(0-3) でサウンド`snd`(0-63) を再生します。`snd`がリストの場合順に再生されます。再生開始位置は`tick`(1 tick = 1/120 秒) で指定できます。`loop`に`True`を指定するとループ再生します。再生終了後に以前の音に復帰させるには`resume`に`True`を指定します。

- `playm(msc, [tick], [loop])`<br>
  ミュージック`msc`(0-7) を再生します。再生開始位置は`tick`(1 tick = 1/120 秒) で指定できます。`loop`に`True`を指定するとループ再生します。

- `stop([ch])`<br>
  指定したチャンネル`ch`(0-3) の再生を停止します。`stop()`で全チャンネルの再生を停止します。

- `play_pos(ch)`<br>
  チャンネル`ch`(0-3) のサウンド再生位置を`(サウンド番号, ノート番号)`のタプルとして取得します。再生停止時は`None`を返します。

### 数学

- `ceil(x)`<br>
  `x`以上の最小の整数を返します。

- `floor(x)`<br>
  `x`以下の最大の整数を返します。

- `sgn(x)`<br>
  `x`が正の時に 1、0 の時に 0、負の時に-1 を返します。

- `sqrt(x)`<br>
  `x`の平方根を返します。

- `sin(deg)`<br>
  `deg`度 (Degree) の正弦を返します。

- `cos(deg)`<br>
  `deg`度 (Degree) の余弦を返します。

- `atan2(y, x)`<br>
  `y`/`x`の逆正接を度 (Degree) で返します。

- `rseed(seed)`<br>
  乱数生成器のシードを設定します。

- `rndi(a, b)`<br>
  `a`以上`b`以下のランダムな整数を返します。

- `rndf(a, b)`<br>
  `a`以上`b`以下のランダムな小数を返します。

- `nseed(seed)`<br>
  Perlin ノイズのシードを設定します。

- `noise(x, [y], [z])`<br>
  指定された座標の Perlin ノイズ値を返します。

### イメージクラス

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
  (`x`, `y`) に色`col`のピクセルを描画します。

### タイルマップクラス

- `width`, `height`<br>
  タイルマップの幅と高さ

- `imgsrc`<br>
  タイルマップが参照するイメージバンク (0-2)

- `set(x, y, data)`<br>
  (`x`, `y`) に文字列のリストでタイルマップを設定します。<br>
  例：`pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  (`x`, `y`) に TMX ファイル (Tiled Map File) から描画順が`layer`(0-) のレイヤーを読み込みます。

- `pget(x, y)`<br>
  (`x`, `y`) のタイルを取得します。タイルは`(tile_x, tile_y)`のタプルです。

- `pset(x, y, tile)`<br>
  (`x`, `y`) にタイルを設定します。タイルは`(tile_x, tile_y)`のタプルです。

### サウンドクラス

- `notes`<br>
  音程 (0-127) のリスト。数値が大きいほど音程は高くなり、33 で 'A2'(440Hz) になります。休符は -1 です。

- `tones`<br>
  音色 (0:Triangle / 1:Square / 2:Pulse / 3:Noise) のリスト

- `volumes`<br>
  音量 (0-7) のリスト

- `effects`<br>
  エフェクト (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut) のリスト

- `speed`<br>
  再生速度。1 が一番速く、数値が大きいほど再生速度は遅くなります。120 で 1 音の長さが 1 秒になります。

- `set(notes, tones, volumes, effects, speed)`<br>
  文字列で音程、音色、音量、エフェクトを設定します。音色、音量、エフェクトの長さが音程より短い場合は、先頭から繰り返されます。

- `set_notes(notes)`<br>
  'CDEFGAB'+'#-'+'01234'または'R'の文字列で音程を設定します。大文字と小文字は区別されず、空白は無視されます。<br>
  例：`pyxel.sounds[0].set_notes("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
  'TSPN'の文字列で音色を設定します。大文字と小文字は区別されず、空白は無視されます。<br>
  例：`pyxel.sounds[0].set_tones("TTSS PPPN")`

- `set_volumes(volumes)`<br>
  '01234567'の文字列で音量を設定します。大文字と小文字は区別されず、空白は無視されます。<br>
  例：`pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  'NSVFHQ'の文字列でエフェクトを設定します。大文字と小文字は区別されず、空白は無視されます。<br>
  例：`pyxel.sounds[0].set_effects("NFNF NVVS")`

### ミュージッククラス

- `seqs`<br>
  サウンド (0-63) のリストをチャンネル数分連ねた 2 次元リスト

- `set(seq0, seq1, seq2, ...)`<br>
  チャンネルのサウンド (0-63) のリストを設定します。空リストを指定するとそのチャンネルは再生に使用しません。<br>
  例：`pyxel.musics[0].set([0, 1], [], [3])`

### 上級者向け API

Pyxel には「ユーザーを混乱させる可能性がある」「使うために専門の知識が必要」などの理由から、このリファレンスには記載していない「上級者向け API」があります。

腕に覚えのある方は、[こちら](../python/pyxel/__init__.pyi)を手がかりにして、あっと驚くような作品づくりに挑戦してみてください！

## コントリビューション方法

### 問題の報告

不具合の報告や機能の要望は[Issue Tracker](https://github.com/kitao/pyxel/issues)で受け付けています。新しいレポートを作成する前に、同じ内容のものがないか確認をお願いします。

### 動作確認

動作確認を行い、[Issue Tracker](https://github.com/kitao/pyxel/issues)で不具合の報告や改善の提案をしてくれる方は大歓迎です！

### プルリクエスト

パッチや修正はプルリクエスト (PR) として受け付けています。提出の前に問題がすでに解決済みでないか[Issue Tracker](https://github.com/kitao/pyxel/issues)で確認をお願いします。

提出されたプルリクエストは[MIT ライセンス](../LICENSE)で公開することに同意したものと見なされます。

## その他の情報

- [Q&A](https://github.com/kitao/pyxel/wiki/Pyxel-Q&A)
- [User Examples](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Developer's Twitter account](https://twitter.com/kitao)

## ライセンス

Pyxel は[MIT ライセンス](../LICENSE)です。ソースコードやライセンス表示用のファイル等で、[著作権とライセンス全文](LICENSE)の表示を行えば、自由に販売や配布をすることができます。

## スポンサー募集

Pyxel は GitHub Sponsors でスポンサーを募っています。Pyxel のメンテナンスと機能追加の継続のためにスポンサーになることをご検討ください。スポンサーは特典として Pyxel についての相談をすることができます。詳細は[こちら](https://github.com/sponsors/kitao)をご覧ください。
