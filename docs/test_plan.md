# テスト計画

## 単体テスト

stages/retrieval::item

- 引数に対して期待する結果が返ることのチェック

stages/retrieval::search

- request_number = 1 のときに、適切な結果が返ることのチェック
- request_number = 2 のときに、適切な結果が返ることのチェック
- request_number = 3 のときに、空の Vec が返ることのチェック

stages/reranking::rerank

- items の長さが1のときに、items がそのまま返ることのチェック
- items の長さが4のときに、score によって降順にソートされた結果が返ることのチェック

explanation

- items に推薦文をつけられるかのチェック

## 結合テスト

`pipeline::run` を対象に、3ステージ(retrieval → reranking → explanation)を通しで実行した結果を検証する。

- request_number = 1 のときに、`Response.items` が score 降順(id: "1", "2")に並び、かつ全アイテムに `explanation`(`"{user_name} さんの検索意図に合致するため推薦されました"`)が設定されていることのチェック
- request_number = 2 のときに、`Response.items` が score 降順(id: "3", "5", "4")に並び、かつ全アイテムに `explanation` が設定されていることのチェック
- request_number = 3(該当なし)のときに、`Response.items` が空であることのチェック
- `user_name` を変更したときに、`explanation` の文言にその `user_name` が反映されることのチェック(reranking の結果が explanation に正しく引き継がれることの確認)
