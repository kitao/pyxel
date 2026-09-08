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

サンプルコードは [ユーザーガイドの手順](https://kitao.github.io/pyxel/web/user-guide/#s-usage) でコピーできるほか、[Pyxel Showcase](https://kitao.github.io/pyxel/web/showcase/) でブラウザ上でも実行できます。

</details>

<details>
<summary>Pyxel に関する書籍はありますか？</summary>

日本語版のみですが、[公式の書籍](https://gihyo.jp/book/2025/978-4-297-14657-3) が発売されています。

</details>

## API 仕様と使い方

<details>
<summary><code>update</code> 関数と <code>draw</code> 関数の違いは何ですか？</summary>

`update` は毎フレームの状態を更新し、`draw` は画面を描画します。処理が遅れると `draw` はスキップされることがあるため、移動や当たり判定などは `update` に書きます。

</details>

<details>
<summary>Pyxel MML の使い方を教えてください</summary>

MML (Music Macro Language) は、音符やテンポを文字列で指定する言語です。

Sound の `mml` メソッドで設定し、引数なしの `mml()` で解除します。

```python
pyxel.sounds[0].mml("CDEFGAB>C")
```

`play` 関数にサウンド番号の代わりに MML 文字列を直接渡して再生することもできます。

```python
pyxel.play(0, "CDEFG")
```

利用できる MML コマンドは [Pyxel MML コマンド](https://kitao.github.io/pyxel/web/mml-studio/mml-commands.html) を参照してください。使用例はサンプル 09_shooter.py の [デモ](https://kitao.github.io/pyxel/web/showcase/examples/09-shooter.html) や [コード](https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py) で確認できます。

また、[Pyxel MML Studio](https://kitao.github.io/pyxel/web/mml-studio/) を使えば、MML をブラウザ上で作成・共有できます。

</details>

## ファイル操作とデータ管理

<details>
<summary>環境が変わるとファイルを読み込めなくなるのはなぜですか？</summary>

相対パスはカレントディレクトリ（作業フォルダ）を基準にします。`pyxel.init()` はこれをスクリプトのあるフォルダに変更します。`init` より前に読み込む場合や、後で作業フォルダを変更した場合は、パスの基準を確認してください。

</details>

<details>
<summary>ハイスコアやゲームの進行状況など、アプリケーション固有のデータを保存する方法はありますか？</summary>

`pyxel.user_data_dir(vendor_name, app_name)` に開発者名とアプリ名を渡すと、環境に適した保存用フォルダを作成し、そのパスを返します。このフォルダにデータを保存してください。

</details>

## Pyxel ツールの使い方

<details>
<summary>インストールせずに Pyxel を試せますか？</summary>

[Pyxel Code Maker](https://kitao.github.io/pyxel/web/code-maker/) を使えば、ブラウザ上で Pyxel アプリを作成して実行できます。ただし、コードエディタで編集できるのは `main.py` だけです。複数の Python ファイルを編集する場合は、ローカル環境をおすすめします。

[Pyxel Showcase](https://kitao.github.io/pyxel/web/showcase/) では、サンプルコードやアプリをブラウザ上で一覧・実行できます。

</details>

<details>
<summary>自作の Pyxel アプリを Web で公開するにはどうすればいいですか？</summary>

Pyxel Code Maker、Pyxel Web Launcher、app2html、HTML カスタム要素の 4 つの方法があります。詳しくは [Web 版 Pyxel の使い方](https://kitao.github.io/pyxel/web/web-usage/) を参照してください。

</details>

<details>
<summary>Pyxel Editor でパレットの色を変更できますか？</summary>

Pyxel リソースファイル (.pyxres) と同じディレクトリに、拡張子を .pyxpal に変えた同名のファイルを配置すると、Pyxel Editor のパレット表示色が変更されます。パレットファイルは `save_pal` 関数で作成できるほか、1 行に 1 色の RRGGBB 形式の 16 進数カラーコードを記述したテキストファイルとして手動で作成することもできます。

</details>

## バージョン移行ガイド

<details>
<summary>バージョン 2.4 への移行方法</summary>

Pyxel 2.4 ではサウンドエンジンと MML 文法が刷新されています。

コードをバージョン 2.4 に対応させるには、以下の変更を行ってください。

- Tone クラスの `waveform` フィールドを `wavetable` にリネームする
- `play` 関数、`playm` 関数の `tick` 引数を `sec` に変更し、値を `tick / 120` 秒に換算する
- `play_pos` 関数の戻り値が `(sound_index, sec)` に変わったことに対応する
- Sound クラス、Music クラスの `save` 関数では、再生回数 `count` の代わりに保存する秒数を `sec` に指定する
- サウンドの再生秒数が必要な場合は、Sound クラスの `total_sec` 関数を利用する
- Sound クラスの `mml` 関数には、新 MML 文法に書き換えたコードを指定する
- `save`、`load` 関数の `excl_*` オプションを `exclude_*` に変更する
- `save`、`load` 関数の `incl_*` オプションの指定を削除する

新しい MML 文法は上記の「[Pyxel MML の使い方](#api-仕様と使い方)」を参照してください。

</details>

## ライセンスとスポンサーシップ

<details>
<summary>Pyxel を作者の許可なく商業目的で使用することはできますか？</summary>

MIT ライセンスに従い、ソースコードやライセンス表示用のファイルに著作権およびライセンスの全文を明示すれば、作者の許可を得ることなく自由に販売や配布が可能です。ただし、Pyxel は個人で開発しているため、作者へのご連絡やスポンサーによるご支援をいただけるとありがたいです。

</details>
