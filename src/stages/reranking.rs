use crate::item::Item;
use crate::request::Request;

/// 検索結果を並び替える(2nd stage)
pub fn rerank(request: &Request, mut items: Vec<Item>) -> Vec<Item> {
    // 実際には機械学習モデルによるスコアリングなどが入る
    items.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    items
}
