# Pyxel よくある質問

## 新バージョンへの移行方法

<details>
<summary>バージョン2.4への移行方法</summary>

Pyxel 2.4 ではサウンドエンジンと MML 文法が刷新されています。<br>
コードをバージョン 2.4 に対応させるには、以下の変更を行ってください。

- Tone クラスの `waveform` フィールドを `wavetable` にリネームする
- `play` 関数、`playm` 関数の `tick` 引数を `sec`（小数形式の秒数）に変更する
- `play_pos` 関数の戻り値が `(sound_no, sec)` に変わったことに対応する
- Sound クラス、Music クラスの `save` 関数の `count` 引数を `sec` に変更する
- サウンドの再生秒数が必要な場合は、Sound クラスの `total_sec` 関数を利用する
- Sound クラスの `mml` 関数には新 MML 文法に沿ったコードを指定する
- 旧 MML 文法を使用する場合は、Sound クラスの `old_mml` 関数を使用する
- `save`、`load` 関数の `excl_*` オプションを `exclude_*` に変更する
- `save`、`load` 関数の `incl_*` オプションの指定を削除する

新しい MML 文法は後述の「Pyxel の MML の使い方」を参照してください。

</details>

<details>
<summary>バージョン1.5への移行方法</summary>

コードをバージョン 1.5 に対応させるには、以下の変更を行ってください。

- `init` の `caption` オプションを `title` にリネームする
- `init` の `scale` オプションを `display_scale` にリネームする
- `init` から `palette` オプションを削除する (初期化後に `colors` 配列でパレットカラーを変更できます)
- `init` から `fullscreen` オプションを削除する (初期化後に `fullscreen` 関数でフルスクリーンを切り替えることができます)
- キー名の未定義エラーが発生した場合、[キー定義](https://github.com/kitao/pyxel/blob/main/python/pyxel/__init__.pyi) に従ってキー名をリネームする
- `Image` クラスおよび `Tilemap` クラスの `get` と `set` をそれぞれ `pget` と `pset` に変更する
- `bltm` の `u`, `v`, `w`, `h` パラメータを 8 倍に変更する (`bltm` はピクセル単位で動作するようになりました)
- `Sound` および `Music` クラスのメンバーとメソッドを新しい名前に更新する

</details>

<details>
<summary>バージョン1.5以降で <code>pyxeleditor</code> コマンドが使えません</summary>

バージョン 1.5 以降、Pyxel のツールは `pyxel` コマンドに統合されました。リソースエディタにアクセスするには、次のコマンドを使用してください: `pyxel edit [PYXEL_RESOURCE_FILE]`

</details>

## Pyxel の学び方

<details>
<summary>Pyxel を学習するにはどこから始めればいいですか？</summary>

Pyxel のサンプルコードを 01、05、03、04、02 の順に試すのがおすすめです。

</details>

<details>
<summary>Pyxel に関する書籍はありますか？</summary>

日本語版のみですが、[公式の書籍](https://gihyo.jp/book/2025/978-4-297-14657-3) が発売されています。

</details>

## API 仕様と使い方

<details>
<summary><code>update</code> 関数と <code>draw</code> 関数の違いは何ですか？</summary>

`update` 関数は毎フレーム呼び出されますが、`draw` 関数は処理時間が許容限界を超えた場合にスキップされることがあります。Pyxel はこの設計により、レンダリング負荷や OS の割り込み処理の影響を軽減して、滑らかなアニメーションを実現しています。

</details>

<details>
<summary>Pyxel の MML の使い方を教えてください</summary>

Sound クラスの `mml` 関数に MML (Music Macro Language) 文字列を渡すと、MML モードに移行し、その内容に沿ってサウンドが再生されるようになります。

MML モードでは、`notes` や `speed` などの通常のパラメータは無視され、指定した文字列の内容に沿ってサウンドが再生されます。`mml()` を呼ぶと MML モードをリセットできます。

`play` 関数にサウンド番号の代わりに直接 MML 文字列を渡して再生することもできます。<br>
例：`pyxel.play(0, "CDEFG")`

Pyxel の MML で利用できるコマンドは [Pyxel MML コマンド](https://kitao.github.io/pyxel/wasm/mml-studio/mml-commands.html) で参照できます。

使用例はサンプル 09_shooter.py の [デモ](https://kitao.github.io/pyxel/wasm/showcase/examples/09-shooter.html) や [コード](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py) で確認できます。

また、[Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/) を使えば、MML をブラウザ上で作成・共有できます。

</details>

## ファイル操作とデータ管理

<details>
<summary>ファイルが読み込めません。環境が変わると失敗する時があります。</summary>

ファイルを読み込む際に、カレントディレクトリが意図したものになっているかを確認してください。<br>
Pyxel の `init` 関数が呼ばれると、カレントディレクトリはそのスクリプトファイルと同じ場所に変更され、それ以降は相対パスでファイルを指定できます。しかし、`init` を呼ぶ前にファイルを開こうとした場合や `init` の後にカレントディレクトリを変更した場合には読み込みに失敗する可能性があります。

</details>

<details>
<summary>ハイスコアやゲームの進行状況など、アプリケーション固有のデータを保存する方法はありますか？</summary>

`user_data_dir(vendor_name, app_name)` 関数に開発者名とアプリ名を渡すと、プラットフォームに適したデータ保存用のディレクトリを作成し、そのパスを返します。このディレクトリの下でアプリケーション用のファイルの保存や読み込みを行ってください。

</details>

## Pyxel ツールの使い方

<details>
<summary>Pyxel Editor でパレットの色を変更できますか？</summary>

Pyxel リソースファイル（.pyxres）と同じディレクトリに、Pyxel パレットファイル（.pyxpal）を配置することで、Pyxel Editor で使用するパレットの色をリソースファイルに合わせることができます。Pyxel パレットファイルの作成方法については、README をご参照ください。

</details>

## 今後の開発計画

<details>
<summary>今後のリリースで予定している機能は？</summary>

以下の機能追加や改善を予定しています。

- Pyxel アプリランチャーの追加
- Pyxel Editor の操作性向上
- 子供向け Pyxel チュートリアルの追加

</details>

## ライセンスとスポンサーシップ

<details>
<summary>Pyxel を作者の許可なく商業目的で使用することはできますか？</summary>

MIT ライセンスに従い、ソースコードやライセンス表示用のファイルに著作権およびライセンスの全文を明示すれば、作者の許可を得ることなく自由に販売や配布が可能です。ただし、もし可能であれば、作者にご連絡いただいたり、スポンサーとしてご支援いただけるとありがたいです。

</details>
