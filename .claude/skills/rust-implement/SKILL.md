---
name: implement
description: 実装用 skill
---

## 手順

1. 現状を把握する。
2. 方針を簡潔明瞭に述べる。設計の選択肢がある場合は原案を1つ示す。
3. 実装する。`cargo fmt` を実行してフォーマットを整える。
4. `cargo clippy` `cargo check` `cargo test` `/skill rust-review` により検証する。修正点が見つかった場合は Step.3 に戻る。
5. 結果を報告する。

## 注意

- スコープを広げない。
- `git add` `git commit` `git push` は依頼があったときのみ行う。指示がない場合は行わない。
