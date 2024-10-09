# Pyxel よくある質問

## 新バージョンへの移行方法

<details>
<summary>バージョン1.5への移行方法</summary>

コードをバージョン 1.5 に対応させるために、以下の変更を行ってください。

- `init` の `caption` オプションを `title` にリネーム
- `init` の `scale` オプションを `display_scale` にリネーム
- `init` から `palette` オプションを削除。初期化後に `colors` 配列でパレットカラーを変更できます。
- `init` から `fullscreen` オプションを削除。初期化後に `fullscreen` 関数でフルスクリーンを切り替えることができます。
- 未定義エラーが発生した場合、[キー定義](https://github.com/kitao/pyxel/blob/main/python/pyxel/__init__.pyi) に従ってキー名をリネームしてください。
- `Image` クラスおよび `Tilemap` クラスの `get` と `set` をそれぞれ `pget` と `pset` に変更
- `bltm` の `u`, `v`, `w`, `h` パラメータを 8 倍に変更。`bltm` はピクセル単位で動作するようになりました。
- `Sound` および `Music` クラスのメンバーとメソッドを新しい命名規則に従って更新
</details>

<details>
<summary>バージョン1.5以降で<code>pyxeleditor</code>コマンドが使えないのはなぜですか？</summary>

バージョン 1.5 以降、Pyxel のツールは`pyxel`コマンドに統合されました。リソースエディタにアクセスするには、次のコマンドを使用してください： `pyxel edit [PYXEL_RESOURCE_FILE]`

</details>

## Pyxel の学び方

<details>
<summary>Pyxelを学習するにはどこから始めればいいですか？</summary>

Pyxel のサンプルコードを試してみることをお勧めします。次の順に試してください： 01, 05, 03, 04, 02。

</details>

<details>
<summary>Pyxelに関する書籍はありますか？</summary>

現在、日本語で 2 冊の本が出版されていますが、どちらも Pyxel の開発者によるものではありません。残念ながら、今のところ英語版はありませんが、今後英語版を含む Pyxel の本がさらに出版される可能性があります！

</details>

## API 仕様と使い方

<details>
<summary><code>update</code>関数と<code>draw</code>関数の違いは何ですか？</summary>

`update`関数は毎フレーム呼び出されますが、`draw`関数は処理時間が許容限界を超えた場合にスキップされることがあります。この設計により、レンダリング負荷や割り込み処理にかかわらず、Pyxel は滑らかなアニメーションを維持します。

</details>

## Pyxel ツールの使い方

## 今後の開発計画

<details>
<summary>今後のPyxelリリースで追加される予定の機能は何ですか？</summary>

今後の機能には、以下が含まれます：

- Pyxel Editor の使い勝手の向上
- 子ども向けの Python と Pyxel のチュートリアル
</details>

## ライセンスとスポンサーシップ

<details>
<summary>Pyxelを商業目的で許可なしに使用できますか？</summary>

はい、MIT ライセンスに従い、開発者にクレジットを表記すれば、開発者の許可なく商業目的で Pyxel を使用できます。ただし、Pyxel をスポンサーしていただけると非常にありがたいです！

</details>
