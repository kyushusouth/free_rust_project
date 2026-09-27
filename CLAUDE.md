# CLAUDE.md

検索システム(推薦パイプライン)を模したコードで Rust を学ぶためのプロジェクト。

## 技術スタック

- 言語: Rust(edition 2024、Rust 1.85 以降)
- ビルド: Cargo(lib + bin 構成、パッケージ名 `free`)
- 依存クレート: `chrono`(時刻の扱い)のみ

## ディレクトリ構成

```
.
├── Cargo.toml
├── READMD.md              # README(ファイル名は typo の可能性)
└── src/
    ├── main.rs            # エントリポイント。Request を組み立てて pipeline::run を呼ぶ
    ├── lib.rs             # モジュール公開
    ├── request.rs         # Request / Location / Condition
    ├── item.rs            # Item
    ├── response.rs        # Response
    ├── pipeline.rs        # 各ステージを順に呼ぶ
    └── stages/
        ├── mod.rs
        ├── retrieval.rs   # 1st: 候補取得(request_number で返却内容が変わる)
        ├── reranking.rs   # 2nd: score 降順に並べ替え
        └── explanation.rs # 3rd: 推薦理由を付与
```

## 主要コマンド

- `cargo run` — 実行
- `cargo check` — 型チェック
- `cargo build` — ビルド
- `cargo test` — テスト
- `cargo clippy` — lint
- `cargo fmt` — フォーマット
