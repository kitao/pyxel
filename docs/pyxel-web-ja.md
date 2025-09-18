# Web 版 Pyxel の使い方

Web 版 Pyxel を使うと Python や Pyxel をインストールせずに、PC、スマートフォン、タブレット等の Web ブラウザーで Pyxel のアプリケーションを実行できます。

Web 版 Pyxel の利用方法には、次の 3 種類があります。

- **Pyxel Web Launcher に GitHub リポジトリを指定する**<br>
  Pyxel Web Launcher の URL に GitHub のリポジトリ名を指定すると、指定したリポジトリを直接読み込み、Web ブラウザ上でアプリを実行できます。アプリを GitHub で公開している場合、最も簡単な実行方法です。

- **Pyxel アプリを HTML ファイルに変換する**<br>
  アプリが Pyxel アプリケーション形式 (.pyxapp) になっている場合は、`pyxel app2html`コマンドを使って HTML ファイルに変換できます。変換後の HTML ファイルはサーバーを必要とせず、単体で実行可能です。

- **Pyxel カスタムタグを使って HTML ファイルを作成する**<br>
  Pyxel 専用のカスタムタグを使用して、アプリ実行用の HTML ファイルを作成します。作成した HTML ファイルはサーバーでホスティングする必要がありますが、既存の HTML ページへの組み込みやカスタマイズが可能です。

それぞれの利用方法について説明します。

## Pyxel Web Launcher に GitHub リポジトリを指定する

Python コードや Pyxel アプリ (.pyxapp) が GitHub 上で公開されている場合、Pyxel Web Launcher を使用して直接実行できます。Pyxel Web Launcher は常に最新版の Pyxel を参照します。

Pyxel Web Launcher の URL 書式は以下の通りです。

```
https://kitao.github.io/pyxel/wasm/launcher/?<コマンド>=<githubのユーザー名>.<リポジトリ名>.<アプリのディレクトリ>.<拡張子を取ったファイル名>
```

コマンドには次の３つが指定できます。

- `run`: Python スクリプトを実行する
- `play`: Pyxel アプリを実行する
- `edit`: Pyxel Editor を起動する

例えば、ユーザー名が`taro`、リポジトリ名が`my_repo`、ファイルのディレクトリが`src/scenes`、Python スクリプトが`title.py`の場合は、URL は以下のようになります。

```
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.src.scenes.title
```

`dist/games`にある`shooter.pyxapp`を実行する場合の URL は次の通りです。

```
https://kitao.github.io/pyxel/wasm/launcher/?play=taro.my_repo.dist.games.shooter
```

複数のファイルに分かれたアプリを`run`コマンドで実行すると読み込みに時間がかかるため、その場合は Pyxel アプリケーションファイル（.pyxapp）に変換して`play`コマンドで実行することをおすすめします。

`run`および`play`コマンドには、バーチャルゲームパッドを有効にする`gamepad`属性や、追加パッケージを指定する`packages`属性を指定することが可能です。

例えば、バーチャルゲームパッドを有効にし、追加パッケージとして NumPy と Pandas を使用する場合は、次のような URL になります。

```
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.src.scenes.title&gamepad=enabled&packages=numpy,pandas
```

なお、追加できるパッケージは[Pyodide 対応パッケージ](https://pyodide.org/en/stable/usage/packages-in-pyodide.html)に限られます。

`edit`コマンドを使用する場合、`editor`属性で Pyxel Editor の起動画面を指定できます。

例えば、`assets`ディレクトリにある`shooter.pyxres`ファイルをタイルマップエディタ画面で起動するには、以下の URL を使用します。

```html
https://kitao.github.io/pyxel/wasm/launcher/?run=taro.my_repo.assets.shooter&editor=tilemap
```

[Pyxel Web Launcher ページ](https://kitao.github.io/pyxel/wasm/launcher/)では、必要な情報を入力して、アプリの起動 URL を自動作成することができます。

また、MML List に複数チャンネルの MML を `CDE;EFG` のようにセミコロン (`;`) で区切って入力することで、MML を再生する URL も作成できます。MML の使い方は[こちらのページ](faq-ja)を参照してください。

## Pyxel アプリを HTML ファイルに変換する

Pyxel アプリケーションファイル (.pyxapp) は、次のコマンドで単独で動作する HTML ファイルに変換できます。作成した HTML ファイルは、変換に使用したバージョンの Pyxel を参照します。

```sh
pyxel app2html your_app.pyxapp
```

作成された HTML ファイルでは、バーチャルゲームパッドがデフォルトで有効になっていますが、カスタムタグを編集することで無効にすることも可能です。

## Pyxel カスタムタグを使って HTML ファイルを作成する

HTML ファイルに Pyxel 専用のカスタムタグを記述することで、Pyxel アプリを実行できます。

Pyxel カスタムタグを利用するには、以下のスクリプトタグを HTML ファイルに追加します。

```html
<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel@latest/wasm/pyxel.js"></script>
```

また、`@`の後にバージョン番号を指定することで、実行時に参照する Pyxel のバージョンを固定できます。将来のバージョンアップによる互換性の問題を避けたい場合は、バージョン番号を指定してください。

```html
<script src="https://cdn.jsdelivr.net/gh/kitao/pyxel@2.4.6/wasm/pyxel.js"></script>
```

Python コードを直接実行するには、次のように`pyxel-run`タグの`script`属性にコードを記述します。

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

外部の Python ファイルを読み込んで実行する場合は、`pyxel-run`タグに`root`と`name`属性を指定します。

`root`は検索の起点となるディレクトリ、`name`はファイルパスです。

例えば先ほどのコードを`test.py`というファイルに保存し、HTML ファイルと同じディレクトリに配置した場合、次のように記述します。

```html
<pyxel-run root="." name="test.py"></pyxel-run>
```

`root`がカレントディレクトリの場合(`root="."`)、`root`属性は省略可能です。

ローカルの HTML ファイルから外部ファイルを読み込むにはサーバーでのホスティングが必要です。

Python 環境があれば、次のコマンドで簡易サーバーを起動できます。

```python
python -m http.server
# MacやLinuxの場合はpython3を使用してください
```

サーバー起動後、ブラウザで`http://localhost:8000/test.html`にアクセスできます。

同様に、Pyxel アプリ(.pyxapp)は`pyxel-play`タグで実行できます。

```html
<pyxel-play
  root="https://cdn.jsdelivr.net/gh/kitao/pyxel/python/pyxel/examples/apps"
  name="megaball.pyxapp"
></pyxel-play>
```

この例では、`root`属性に URL を指定しています。

`pyxel-run`タグと`pyxel-play`タグには、バーチャルゲームパッドを有効にする`gamepad`属性や、追加パッケージを指定する`packages`属性を指定できます。

例えば、バーチャルゲームパッドを有効にし、NumPy と Pandas を使用する場合は次のようになります。

```html
<pyxel-run name="test.py" gamepad="enabled" packages="numpy,pandas"></pyxel-run>
```

使用できるパッケージは[Pyodide 対応パッケージ](https://pyodide.org/en/stable/usage/packages-in-pyodide.html)に限られます。

また、`pyxel-edit`タグを使って Pyxel Editor を起動できます。

例えば、`assets`ディレクトリにある`shooter.pyxres`ファイルをイメージエディタ画面で起動するには、次のように記述します。

```html
<pyxel-edit root="assets" name="sample.pyxres" editor="image"></pyxel-edit>
```

Pyxel を実行する HTML ファイルに`id="pyxel-screen"`の`<div>`タグを追加すると、その要素を Pyxel の画面として使用します。この`<div>`タグの位置やサイズを調整することで、Pyxel の画面の配置や大きさを変更できます。
