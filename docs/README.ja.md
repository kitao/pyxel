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

## 基本的な使い方

### Pyxel コマンド

Pyxel をインストールすると `pyxel` コマンドが使えるようになります。`pyxel` の後にコマンド名を指定して、さまざまな操作を行います。

引数なしで実行すると、利用可能なコマンドの一覧を確認できます。

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

### サンプルを実行する

次のコマンドでカレントディレクトリに Pyxel のサンプルをコピーできます。

```sh
pyxel copy_examples
```

サンプル一覧は [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/) からブラウザで確認・実行できます。

ローカル環境では以下のコマンドでサンプルを実行できます。

```sh
# examples ディレクトリのサンプルを実行
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# examples/apps ディレクトリのアプリを実行
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

## アプリケーションの作り方

### プログラムの作成

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

### プログラムの実行

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

## リソースの作り方

### Pyxel Editor

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

### その他の作成方法

Pyxel 用の画像やタイルマップは、以下の方法で作成することもできます。

- `Image.set` や `Tilemap.set` 関数を使って、文字列のリストから作成する
- `Image.load` 関数を使って、Pyxel 向け配色の画像ファイル (PNG/GIF/JPEG) を読み込む

Pyxel 用のサウンドやミュージックは、以下の方法で作成することもできます。

- `Sound.set` や `Music.set` 関数を使って、文字列から作成する

各関数の使い方については API リファレンスを参照してください。

## アプリケーションの配布方法

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

Pyxel の API は [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/) で一覧できます。

Pyxel には、使用に専門知識が必要な「上級者向け API」もあります。リファレンスページで「Advanced」にチェックを入れると表示されます。

腕に覚えのある方は、上級者向け API を活用して、あっと驚くような作品づくりに挑戦してみてください！

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
- [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/)
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
