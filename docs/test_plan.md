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
