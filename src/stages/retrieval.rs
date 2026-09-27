use crate::item::Item;
use crate::request::Request;

/// クエリに基づいてアイテムを検索する(1st stage)
pub fn search(request: &Request) -> Vec<Item> {
    // 実際にはインデックスやDBへの問い合わせ処理などが入る
    vec![
        Item { id: "1".to_string(), name: "Item A".to_string(), score: 0.5, explanation: None },
        Item { id: "2".to_string(), name: "Item B".to_string(), score: 0.3, explanation: None },
    ]
}
