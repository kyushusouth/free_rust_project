# free

## プロジェクトの概要

検索システム(推薦パイプライン)を模したコードで Rust を学ぶためのプロジェクト。

「候補の取得 → 並べ替え → 説明の付与」という3ステージ構成の検索パイプラインを、小さなコードで実装している。`main` が `Request` を組み立てて `pipeline::run` に渡すと、以下の順にステージが呼ばれ `Response` が返る。

```text
Request ─▶ retrieval::search ─▶ reranking::rerank ─▶ explanation::annotate ─▶ Response
              (1st stage)          (2nd stage)            (3rd stage)
```

## 技術スタック

- 言語: Rust(edition 2024、Rust 1.85 以降)
- ビルド: Cargo(lib + bin 構成、パッケージ名 `free`)
- 依存クレート: `chrono`(時刻の扱い)のみ

## 主要コマンド

| コマンド | 説明 |
| --- | --- |
| `cargo run` | 実行 |
| `cargo check` | 型チェック |
| `cargo build` | ビルド |
| `cargo test` | テスト |
| `cargo clippy` | lint |
| `cargo fmt` | フォーマット |
