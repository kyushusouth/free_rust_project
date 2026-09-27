# DesignDoc

## 背景

検索システムは、一般に「候補の取得 → 並べ替え → 説明の付与」のように処理を段階（ステージ）に分ける。
本プロジェクトは、こうした検索システムを模した小さなコードを題材に Rust を学習するためのものである。

## 目的

- 検索システムの 3 ステージ構成を Rust で実装し、基本概念を学ぶ。

## 設計

### 全体像

`main` が `Request` を組み立て、`pipeline::run` に渡す。`pipeline::run` は 3 つのステージを順に呼び出し、`Response` を返す。

```txt
Request ─▶ retrieval::search ─▶ reranking::rerank ─▶ explanation::annotate ─▶ Response
              (1st stage)          (2nd stage)            (3rd stage)
```

### データ構造

#### Request

| フィールド名 | データ型 | 説明 |
| --- | --- | --- |
| request_number | u32 | リクエスト番号。`retrieval::search` の返却内容を切り替える |
| user_name | String | ユーザー名 |
| query | String | 検索クエリ |
| location | Location | 緯度経度。Location は latitude と longitude をフィールドに持つ構造体 |
| time | DateTime[Utc] | リクエストを送った日時 |
| condition | Condition | 天候 (`Sunny` `Cloudy` `Rainy` `Snowy` のいずれか) |

#### Item

| フィールド名 | データ型 | 説明 |
| --- | --- | --- |
| id | String | アイテムID |
| name | String | アイテム名 |
| score | f64 | リランキング用スコア |
| explanation | Option[String] | 推薦理由 |

#### Response

| フィールド名 | データ型 | 説明 |
| --- | --- | --- |
| items | Vec[Item] | 検索結果 |

### 処理フロー

#### 1st stage: `retrieval::search`

`request_number` に応じて固定の候補を返す。

#### 2nd stage: `reranking::rerank`

`score` の降順に並べ替える。

#### 3rd stage: `explanation::annotate`

すべての `Item` の `explanation` に「`{user_name} さんの検索意図に合致するため推薦されました`」を設定する。
