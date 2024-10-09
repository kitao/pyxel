# Pyxel よくある質問

## 新バージョンへの移行方法

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
<summary>バージョン1.5以降で<code>pyxeleditor</code>コマンドが使えません</summary>

バージョン 1.5 以降、Pyxel のツールは`pyxel`コマンドに統合されました。リソースエディタにアクセスするには、次のコマンドを使用してください： `pyxel edit [PYXEL_RESOURCE_FILE]`

</details>

## Pyxel の学び方

<details>
<summary>Pyxelを学習するにはどこから始めればいいですか？</summary>

Pyxel のサンプルコードを 01、05、03、04、02 の順に試すのがおすすめです。

</details>

<details>
<summary>Pyxelに関する書籍はありますか？</summary>

現在、日本語で 2 冊の本が出版されていますが、どちらも Pyxel の開発者によるものではありません。また、今のところ英語版はありません。

</details>

## API 仕様と使い方

<details>
<summary><code>update</code>関数と<code>draw</code>関数の違いは何ですか？</summary>

`update`関数は毎フレーム呼び出されますが、`draw`関数は処理時間が許容限界を超えた場合にスキップされることがあります。Pyxel はこの設計により、レンダリング負荷や OS の割り込み処理の影響を軽減して、滑らかなアニメーションを実現しています。

</details>

## Pyxel ツールの使い方

## 今後の開発計画

<details>
<summary>今後のリリースで予定している機能は？</summary>

以下の機能追加や改善を予定しています。

- Pyxel アプリランチャーの追加
- サウンド機能の刷新と MML 対応
- Pyxel Editor の操作性向上
- 子供向け Pyxel チュートリアルの追加
</details>

## ライセンスとスポンサーシップ

<details>
<summary>Pyxelを作者の許可なく商業目的で使用することはできますか？</summary>

MIT ライセンスに従い、ソースコードやライセンス表示用のファイルに著作権およびライセンスの全文を明示すれば、作者の許可を得ることなく自由に販売や配布が可能です。ただし、もし可能であれば、作者にご連絡いただいたり、スポンサーとしてご支援いただけるとありがたいです。

</details>
