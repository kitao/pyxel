# Pyxel よくある質問

## Pyxel の学び方

<details>
<summary>Pyxel を学習するにはどこから始めればいいですか？</summary>

Pyxel のサンプルコードを以下の順に試すのがおすすめです。

1. 01_hello_pyxel — Pyxel の基本
2. 05_color_palette — カラーパレット
3. 03_draw_api — 描画 API
4. 04_sound_api — サウンド API
5. 02_jump_game — ゲーム実装

サンプルコードは `pyxel copy_examples` でコピーできるほか、[Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/) でブラウザ上でも実行できます。

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

MML（Music Macro Language）は、音符やテンポなどを文字列で記述してサウンドを定義する言語です。

Sound クラスの `mml` 関数に MML 文字列を渡すと、その Sound が MML の内容に沿って再生されるようになります。`mml()` を引数なしで呼ぶと MML 設定を解除できます。

```python
pyxel.sounds[0].mml("CDEFGAB>C")
```

`play` 関数にサウンド番号の代わりに MML 文字列を直接渡して再生することもできます。

```python
pyxel.play(0, "CDEFG")
```

利用できる MML コマンドは [Pyxel MML コマンド](https://kitao.github.io/pyxel/wasm/mml-studio/mml-commands.html) を参照してください。使用例はサンプル 09_shooter.py の [デモ](https://kitao.github.io/pyxel/wasm/showcase/examples/09-shooter.html) や [コード](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py) で確認できます。

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
<summary>インストールせずに Pyxel を試せますか？</summary>

[Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/) を使えば、ブラウザ上で Pyxel アプリの作成・実行ができます。ただし、複数ファイル構成には対応していないため、本格的な開発にはローカル環境をおすすめします。

[Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/) では、サンプルコードやアプリをブラウザ上で一覧・実行できます。

</details>

<details>
<summary>自作の Pyxel アプリを Web で公開するにはどうすればいいですか？</summary>

Web Launcher、app2html、カスタムタグの 3 つの方法があります。詳しくは [Web 版 Pyxel の使い方](pyxel-web-ja.md) を参照してください。

</details>

<details>
<summary>Pyxel Editor でパレットの色を変更できますか？</summary>

Pyxel リソースファイル（.pyxres）と同じディレクトリに、拡張子を .pyxpal に変えた同名のファイルを配置すると、Pyxel Editor のパレット表示色が変更されます。パレットファイルは `save_pal` 関数で作成できるほか、1 行に 1 色の 16 進数カラーコードを記述したテキストファイルとして手動で作成することもできます。

</details>

## バージョン移行ガイド

<details>
<summary>バージョン 2.4 への移行方法</summary>

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

新しい MML 文法は上記の「[Pyxel の MML の使い方](#api-仕様と使い方)」を参照してください。

</details>

## ライセンスとスポンサーシップ

<details>
<summary>Pyxel を作者の許可なく商業目的で使用することはできますか？</summary>

MIT ライセンスに従い、ソースコードやライセンス表示用のファイルに著作権およびライセンスの全文を明示すれば、作者の許可を得ることなく自由に販売や配布が可能です。ただし、もし可能であれば、作者にご連絡いただいたり、スポンサーとしてご支援いただけるとありがたいです。

</details>
