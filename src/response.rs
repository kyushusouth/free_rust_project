use crate::item::Item;

#[derive(Debug)]
pub struct Response {
    pub items: Vec<Item>,
}
