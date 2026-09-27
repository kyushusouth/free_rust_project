use crate::item::Item;
use crate::request::Request;

fn item(id: &str, name: &str, score: f64) -> Item {
    Item { id: id.to_string(), name: name.to_string(), score, explanation: None }
}

/// クエリに基づいてアイテムを検索する(1st stage)
pub fn search(request: &Request) -> Vec<Item> {
    // 実際にはインデックスやDBへの問い合わせ処理などが入る
    match request.request_number {
        1 => vec![
            item("1", "Item A", 0.5),
            item("2", "Item B", 0.3),
        ],
        2 => vec![
            item("3", "Item C", 0.9),
            item("4", "Item D", 0.1),
            item("5", "Item E", 0.7),
        ],
        _ => vec![],
    }
}
