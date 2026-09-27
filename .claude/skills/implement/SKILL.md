---
name: implement
description: 実装用 skill
---

## 手順

1. 現状を把握するために、`docs` 配下を対象にドキュメントを読み取る。
2. ドキュメントの内容に従うよう実装する。`cargo fmt` を実行してフォーマットを整える。
3. `cargo clippy` `cargo check` `cargo test` `/skill rust-review` により検証する。修正点が見つかった場合は Step.3 に戻る。
4. 結果を報告する。

## 注意

- スコープを広げない。
- `git add` `git commit` `git push` は依頼があったときのみ行う。指示がない場合は行わない。
