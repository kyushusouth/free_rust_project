use crate::item::Item;
use crate::request::Request;

/// 検索結果を並び替える(2nd stage)
pub fn rerank(_request: &Request, mut items: Vec<Item>) -> Vec<Item> {
    // 実際には機械学習モデルによるスコアリングなどが入る
    items.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{Condition, Location};
    use crate::stages::retrieval;
    use chrono::Utc;

    fn dummy_request() -> Request {
        Request {
            request_number: 0,
            user_name: "Alice".to_string(),
            query: "query".to_string(),
            location: Location {
                latitude: 0.0,
                longitude: 0.0,
            },
            time: Utc::now(),
            condition: Condition::Sunny,
        }
    }

    #[test]
    fn rerank_returns_empty_vec_unchanged() {
        let request = dummy_request();
        let items: Vec<Item> = vec![];
        let actual = rerank(&request, items);

        assert!(actual.is_empty());
    }

    #[test]
    fn rerank_returns_single_item_unchanged() {
        let request = dummy_request();
        let items = vec![retrieval::item("1", "Item A", 0.5)];
        let actual = rerank(&request, items);

        let expected = vec![retrieval::item("1", "Item A", 0.5)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn rerank_sorts_items_by_score_descending() {
        let request = dummy_request();
        let items = vec![
            retrieval::item("1", "Item A", 0.3),
            retrieval::item("2", "Item B", 0.9),
            retrieval::item("3", "Item C", 0.1),
            retrieval::item("4", "Item D", 0.7),
        ];
        let actual = rerank(&request, items);

        let expected = vec![
            retrieval::item("2", "Item B", 0.9),
            retrieval::item("4", "Item D", 0.7),
            retrieval::item("1", "Item A", 0.3),
            retrieval::item("3", "Item C", 0.1),
        ];

        assert_eq!(actual, expected);
    }
}
