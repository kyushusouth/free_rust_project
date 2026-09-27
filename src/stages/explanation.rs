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
