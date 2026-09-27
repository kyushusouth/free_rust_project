use crate::item::Item;
use crate::request::Request;

/// アイテムに推薦理由の説明文を付与する(3rd stage)
pub fn annotate(request: &Request, mut items: Vec<Item>) -> Vec<Item> {
    for item in items.iter_mut() {
        item.explanation = Some(format!(
            "{} さんの検索意図に合致するため推薦されました",
            request.user_name
        ));
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{Condition, Location};
    use chrono::Utc;

    #[test]
    fn annotate_returns_empty_vec_unchanged() {
        let request = Request {
            request_number: 0,
            user_name: "Alice".to_string(),
            query: "query".to_string(),
            location: Location {
                latitude: 0.0,
                longitude: 0.0,
            },
            time: Utc::now(),
            condition: Condition::Sunny,
        };
        let actual = annotate(&request, vec![]);

        assert!(actual.is_empty());
    }

    #[test]
    fn annotate_sets_explanation_for_each_item() {
        let request = Request {
            request_number: 0,
            user_name: "Alice".to_string(),
            query: "query".to_string(),
            location: Location {
                latitude: 0.0,
                longitude: 0.0,
            },
            time: Utc::now(),
            condition: Condition::Sunny,
        };
        let items = vec![
            Item {
                id: "1".to_string(),
                name: "Item A".to_string(),
                score: 0.5,
                explanation: None,
            },
            Item {
                id: "2".to_string(),
                name: "Item B".to_string(),
                score: 0.3,
                explanation: None,
            },
        ];
        let actual = annotate(&request, items);

        for item in actual {
            assert_eq!(
                item.explanation,
                Some("Alice さんの検索意図に合致するため推薦されました".to_string())
            );
        }
    }
}
