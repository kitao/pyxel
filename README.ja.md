# Pyxel

Pyxel (ピクセル) はPython向けのゲーム開発環境です。

使える色は16色のみ、同時に再生できる音は4音までなど、レトロゲーム機を意識したシンプルな仕様で、Pythonでドット絵スタイルのゲームづくりが気軽に楽しめます。

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/02_simple_game.py" target="_blank">
<img
src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/02_simple_game.gif" width="32%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/03_draw_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/03_draw_api.gif" width="32%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/04_sound_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/04_sound_api.gif" width="32%">
</a>

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

## APIリファレンス

### システム
- `width`, `height`  
画面の幅と高さ
- `frame_count`  
経過フレーム数
- `init(width, height, [caption], [scale], [palette], [fps], [border_width], [border_color])`  
Pyxelアプリを画面サイズ(width, height)で初期化する
- `run(update, draw)`  
Pyxelアプリを開始し、フレーム更新時にupdate、描画時にdrawを呼ぶ
- `quit()`  
現在フレーム終了時にPyxelアプリを終了する

## 入力
- `mouse_x`, `mouse_y`  
現在のマウスカーソル座標
- `btn(key)`  
keyが押されていたらTrue、押されていなければFalseを返す([キー定義一覧]())
- `btnp(key, [hold], [period])`  
そのフレームにキーが押されたらTrue、押されなければFalseを返す。holdとperiodを指定すると、holdフレーム以上ボタンを押し続けた際にもperiod周期でTrueが返る
- `btnr(key)`  
そのフレームにキーが離されたらTrue、離されなければFalseを返す

## グラフィックス
- `image(no)`  
イメージno(0-3)を操作する(イメージクラスを参照のこと)  
例：`pyxel.image(0).load(0, 0, "title.png")`
- `clip(x1, y1, x2, y2)`  
画面の描画領域を(x1, y1)-(x2, y2)にする。clip()で描画領域をリセットする
- `pal(col1, col2)`  
色col1をcol2として描画する。pal()で初期状態にリセットする
- `cls(col)`  
画面を色col1でクリアする
- `pix(x, y, col)`  
(x, y)に色colのピクセルを描画する
- `line(x1, y1, x2, y2, col)`  
(x1, y1)-(x2, y2)に色colの直線を描画する
- `rect(x1, y1, x2, y2, col)`  
(x1, y1)-(x2, y2)に色colの矩形を描画する
- `rectb(x1, y1, x2, y2, col)`  
(x1, y1)-(x2, y2)に色colの矩形の輪郭を描画する
- `circ(x, y, r, col)`  
(x, y)に半径r、色colの円を描画する
- `circb(x, y, r, col)`  
(x, y)に半径r、色colの円の輪郭を描画する
- `blt(x, y, no, sx, sy, w, h, [colkey])`  
(x, y)にイメージno(0-3)の(sx, sy)から(w, h)サイズの画像をコピーする。colkeyに色を設定すると透明色として扱われる
- `text(x, y, s, col)`  
(x, y)に色colで文字列sを描画する

## オーディオ
- `sound(no)`  
サウンドno(0-63)を操作する(サウンドクラスを参照のこと)  
例：`pyxel.sound(0).speed = 60`
- `play(ch, no, loop=False)`  
チャンネルch(0-3)でサウンドno(0-63)を再生する。noがリストの場合順に再生する
- `stop(ch)`  
チャンネルch(0-3)の再生を停止する

### イメージクラス
- `width`, `height`  
イメージの幅と高さ
- `data`  
イメージのデータ(Numpy配列)
- `resize(width, height)`  
イメージのサイズを(width, height)に変更する
- `set(x, y, data)`  
(x, y)に文字列のリストでイメージを設定する  
例：`pyxel.image(0).set(10, 10, ['1234', '5678', '9abc', 'defg'])`
- `load(x, y, filenamse)`  
(x, y)にpngファイルを読み込む
- `copy(x, y, no, width, height)`  
(x, y)にイメージno(0-3)から(width, height)のサイズをコピーする

### サウンドクラス
- `note`  
音程(0-127)のリスト(33=A2=440Hz)
- `tone`  
音色(0:Triagnle/1:Square/2:Pulse/3:Noise)のリスト
- `volume`  
音量(0-7)のリスト
- `effect`  
エフェクト(0:None/1:Slide/2:Vibrato/3:FadeOut)のリスト
- `speed`  
1音の長さ(120で1音1秒)
- `set(note, tone, volume, effect, speed)`  
文字列で音程、音色、音量、エフェクトを設定する
- `set_note(note)`  
'CDEFGAB'+'#-'+'0123'または'R'の文字列で音程(0-127)を設定する  
例：`pyxel.sound(0).set_note('G2B-2RD3RF3')`
- `set_tone(tone)`  
'TSPN'の文字列で音色(0-3)を設定する
- `set_volume(volume)`  
'01234567'の文字列で音量(0-7)を設定する
- `set_effect(effect)`  
'NSVF'の文字列でエフェクト(0-3)を設定する

## ライセンス

Pyxelは[MITライセンス](http://en.wikipedia.org/wiki/MIT_License)です。ソースコードやライセンス表示用のファイル等で、[著作権とライセンス全文](https://raw.githubusercontent.com/kitao/pyxel/master/LICENSE)の表示を行えば、自由に販売や配布をすることができます。
