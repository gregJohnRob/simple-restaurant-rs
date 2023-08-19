use crate::domain::item::{Item, ItemId};
use async_trait::async_trait;

use std::collections::HashMap;

pub enum ItemModelError {}

#[async_trait]
pub trait ItemModel {
    async fn get_item(&self, item_id: &ItemId) -> Result<Option<Item>, ItemModelError>;
    async fn get_items(&self) -> Result<Vec<Item>, ItemModelError>;
}

pub struct DummyItemModel {
    pub items: HashMap<ItemId, Item>,
}

#[async_trait]
impl ItemModel for DummyItemModel {
    async fn get_item(&self, item_id: &ItemId) -> Result<Option<Item>, ItemModelError> {
        Ok(self.items.get(item_id).map(|v| v.to_owned()))
    }

    async fn get_items(&self) -> Result<Vec<Item>, ItemModelError> {
        Ok(self.items.values().cloned().collect())
    }
}

impl Default for DummyItemModel {
    fn default() -> Self {
        Self {
            items: HashMap::from([
                (
                    ItemId("0".to_string()),
                    Item {
                        item_id: ItemId("0".to_string()),
                        name: "pizza".to_string(),
                    },
                ),
                (
                    ItemId("1".to_string()),
                    Item {
                        item_id: ItemId("1".to_string()),
                        name: "beer".to_string(),
                    },
                ),
            ]),
        }
    }
}
