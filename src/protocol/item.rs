use serde::Serialize;

use crate::domain::item;

#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub item_id: String,
    pub name: String,
}

impl From<item::Item> for Item {
    fn from(item: item::Item) -> Self {
        Self {
            item_id: item.item_id.0,
            name: item.name,
        }
    }
}
