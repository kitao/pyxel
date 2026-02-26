# Web 版 Pyxel の使い方

Pyxel アプリは Web ブラウザ上で実行できます。Python や Pyxel のインストールは不要で、PC・スマートフォン・タブレットから利用可能です。

このページでは、Pyxel アプリの Web 公開方法と、ブラウザ上で使える Web ツールについて説明します。

## アプリの Web 公開

Pyxel アプリを Web 上で公開する方法は 3 種類あります。

| 方法 | 特徴 |
| --- | --- |
| [Web Launcher](#web-launcher) | GitHub リポジトリを URL 指定で直接実行。最も手軽 |
| [app2html](#app2html) | Pyxel アプリケーション (.pyxapp) を HTML に変換して公開 |
| [カスタムタグ](#カスタムタグ) | Pyxel 専用タグで既存の HTML ページに組み込み |

### Web Launcher

[Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/) は、GitHub 上で公開されている Python スクリプトや Pyxel アプリ (.pyxapp) を URL 指定で直接実行できるツールです。

必要な情報を入力するだけで起動 URL を自動作成できるほか、以下の書式で手動作成も可能です。なお、Web Launcher では常に最新版の Pyxel が使用されます。

#### URL 書式

```
https://kitao.github.io/pyxel/wasm/launcher/?<コマンド>=<ユーザー名>.<リポジトリ名>.<パス>.<ファイル名（拡張子なし）>
```

パスの区切りにはドット (`.`) を使用します（例: `src/scenes` → `src.scenes`）。

#### コマンド

| コマンド | 動作 |
| --- | --- |
| `run` | Python スクリプト (.py) を実行 |
| `play` | Pyxel アプリ (.pyxapp) を実行 |
| `edit` | Pyxel Editor でリソースファイル (.pyxres) を編集 |

#### URL の例

ユーザー `taro` のリポジトリ `my_repo` にある `src/scenes/title.py` を実行する場合:

```
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.src.scenes.title
```

同じリポジトリの `dist/games/shooter.pyxapp` を実行する場合:

```
https://kitao.github.io/pyxel/wasm/launcher/?play=taro.my_repo.dist.games.shooter
```

#### 属性

`run` および `play` コマンドには以下の属性を追加できます。

| 属性 | 説明 |
| --- | --- |
| `gamepad=enabled` | バーチャルゲームパッド（タッチデバイスに表示される画面上コントローラー）を有効化 |
| `packages=pkg1,pkg2` | 追加の [Pyodide 対応パッケージ](https://pyodide.org/en/stable/usage/packages-in-pyodide.html)（Web 版 Python で使用可能なライブラリ）を指定 |

`run` コマンドに属性を追加した URL の例:

```
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.src.scenes.title&gamepad=enabled&packages=numpy,pandas
```

`edit` コマンドには `editor` 属性で起動画面を指定できます（`image`、`tilemap`、`sound`、`music`）。

`edit` コマンドの URL の例:

```
https://kitao.github.io/pyxel/wasm/launcher/?edit=taro.my_repo.assets.shooter&editor=tilemap
```

複数ファイルのアプリを `run` で実行すると読み込みに時間がかかるため、`pyxel package` コマンドで `.pyxapp` に変換して `play` で実行することをおすすめします。

### app2html

`pyxel app2html` コマンドで、Pyxel アプリケーション (.pyxapp) を HTML ファイルに変換できます。

```sh
pyxel app2html your_app.pyxapp
```

アプリのデータは HTML に埋め込まれるため、生成されたファイルを配布するだけで公開できます。

Pyxel のランタイムは変換時のバージョンに固定されるため、将来のバージョンアップで動作が変わる心配がありません。

バーチャルゲームパッドはデフォルトで有効になっています（タッチデバイスでのみ表示）。無効にするには、生成された HTML 内の `gamepad: "enabled"` を削除してください。

### カスタムタグ

HTML ファイルに Pyxel のカスタムタグを記述することで、Pyxel アプリを既存の Web ページに組み込めます。

#### セットアップ

以下のスクリプトタグを HTML に追加します。

```html
<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel/wasm/pyxel.js"></script>
```

将来のバージョンアップによる互換性の問題を避けたい場合は、`@` の後にバージョン番号を指定してバージョンを固定できます。

```html
<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel@v2.7.2/wasm/pyxel.js"></script>
```

#### pyxel-run

Python コードを直接実行するには `script` 属性にコードを記述します。

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

外部ファイルを読み込む場合は `root` と `name` を指定します。`root` はファイル検索の起点となるディレクトリ、`name` はファイルパスです。

この方法では[サーバーでのホスティング](#ローカルでの実行)が必要です。

```html
<pyxel-run root="." name="test.py"></pyxel-run>
```

`root` がカレントディレクトリの場合（`root="."`）、`root` 属性は省略できます。

#### pyxel-play

Pyxel アプリ (.pyxapp) を実行します。`root` には URL も指定できます。

```html
<pyxel-play
  root="https://cdn.jsdelivr.net/gh/kitao/pyxel/python/pyxel/examples/apps"
  name="megaball.pyxapp"
></pyxel-play>
```

#### pyxel-edit

Pyxel Editor を起動します。`editor` 属性で起動画面を指定できます。

```html
<pyxel-edit root="assets" name="shooter.pyxres" editor="image"></pyxel-edit>
```

#### 共通属性（pyxel-run / pyxel-play のみ）

| 属性 | 説明 |
| --- | --- |
| `gamepad="enabled"` | バーチャルゲームパッドを有効化（タッチデバイスでのみ表示） |
| `packages="pkg1,pkg2"` | 追加の [Pyodide 対応パッケージ](https://pyodide.org/en/stable/usage/packages-in-pyodide.html)を指定 |

#### 画面のカスタマイズ

デフォルトでは Pyxel の画面はページ全体に表示されます。HTML に `id="pyxel-screen"` の `<div>` タグを配置すると、その要素内に画面が表示されるようになり、位置やサイズを自由に調整できます。

#### ローカルでの実行

外部ファイルを読み込むカスタムタグを使用する場合はサーバーでのホスティングが必要です。Python 環境があれば簡易サーバーを利用できます。

```sh
python -m http.server
# Mac や Linux の場合は python3 を使用
```

サーバー起動後、ブラウザで `http://localhost:8000/test.html` にアクセスします。

## Web ツール

Pyxel はアプリの開発に役立つオンラインツールも提供しています。各ツールの詳しい使い方は、それぞれのページにあるマニュアルを参照してください。

| ツール | 概要 |
| --- | --- |
| [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/) | Pyxel アプリを作成・実行できるオンライン IDE |
| [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/) | サンプルコードやアプリを一覧・実行できるギャラリー |
| [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/) | MML（Music Macro Language）でチップチューンを作曲・再生できるエディタ |
| [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/) | Pyxel API の検索・閲覧ができるリファレンス |
