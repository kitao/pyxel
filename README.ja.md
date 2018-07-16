# Pyxel【ピクセル】

PyxelはPython向けのゲーム開発環境です。

使える色は16色のみ、同時に再生できる音は4音までなど、レトロゲーム機を意識したシンプルな仕様で、Pythonでドット絵スタイルのゲームづくりが気軽に楽しめます。

Pyxelのゲーム機の仕様やAPI、パレットなどは、
[PICO-8](https://www.lexaloffle.com/pico-8.php)や[TIC-80](https://tic.computer/)のデザインを参考にしています。

Pyxelはオープンソースで、無料で自由に使えます。Pyxelでレトロゲームづくりを始めましょう！

## 仕様

- Windows・Mac対応
- Python3によるコード記述
- 16色固定パレット
- 128x128サイズ、4画像バンク
- 4音同時再生、定義可能な64サウンド
- キーボード、マウス、ジョイスティック(予定)
- 画像・サウンド編集ツール(予定)

## 動作画面

サンプルの動作画面です。画像をクリックすると実際のコードを確認できます。

### 01_hello_pyxel.py

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/01_hello_pyxel.py" target="_blank">
<img
src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/01_hello_pyxel.gif">
</a>

### 02_simple_game.py

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/02_simple_game.py" target="_blank">
<img
src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/02_simple_game.gif">
</a>

### 03_draw_api.py

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/03_draw_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/03_draw_api.gif">
</a>

### 04_sound_api.py

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/04_sound_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/04_sound_api.gif">
</a>

## インストール方法

### Windows

```sh
pip install pyxel
```

### Mac

```sh
brew install glfw
pip3 install pyxel
```

### サンプルのインストール

カレントディレクトリにPyxelのサンプルをコピーする。

```sh
install_pyxel_examples
```

## 使い方

- 作成中

## リファレンス

- 作成中

## ライセンス

Pyxelは[MITライセンス](http://en.wikipedia.org/wiki/MIT_License)です。ソースコードやライセンス表示用のファイル等で、[著作権とライセンス全文](https://raw.githubusercontent.com/kitao/pyxel/master/LICENSE)の表示を行えば、自由に販売や配布をすることができます。
