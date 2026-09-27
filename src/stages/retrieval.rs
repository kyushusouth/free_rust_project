use crate::item::Item;
use crate::request::Request;

pub fn item(id: &str, name: &str, score: f64) -> Item {
    Item {
        id: id.to_string(),
        name: name.to_string(),
        score,
        explanation: None,
    }
}

/// クエリに基づいてアイテムを検索する(1st stage)
pub fn search(request: &Request) -> Vec<Item> {
    // 実際にはインデックスやDBへの問い合わせ処理などが入る
    match request.request_number {
        1 => vec![item("1", "Item A", 0.5), item("2", "Item B", 0.3)],
        2 => vec![
            item("3", "Item C", 0.9),
            item("4", "Item D", 0.1),
            item("5", "Item E", 0.7),
        ],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{Condition, Location};
    use chrono::Utc;

    fn request_with_number(request_number: u32) -> Request {
        Request {
            request_number,
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
    fn item_sets_fields() {
        let actual = item("1", "Item A", 0.5);

        assert_eq!(actual.id, "1");
        assert_eq!(actual.name, "Item A");
        assert_eq!(actual.score, 0.5);
        assert_eq!(actual.explanation, None);
    }

    #[test]
    fn search_returns_items_for_request_number_1() {
        let request = request_with_number(1);
        let actual = search(&request);

        let expected = vec![item("1", "Item A", 0.5), item("2", "Item B", 0.3)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn search_returns_items_for_request_number_2() {
        let request = request_with_number(2);
        let actual = search(&request);

        let expected = vec![
            item("3", "Item C", 0.9),
            item("4", "Item D", 0.1),
            item("5", "Item E", 0.7),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn search_returns_empty_for_other_request_number() {
        let request = request_with_number(3);
        let actual = search(&request);

        assert!(actual.is_empty());
    }
}
