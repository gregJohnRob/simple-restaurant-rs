use crate::domain::item::{Item, ItemId};
use async_trait::async_trait;
use deadpool_postgres::Pool;
use deadpool_postgres::PoolError;

use std::collections::HashMap;

#[derive(Debug)]
pub enum ItemModelError {
    PoolError(PoolError),
}

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

pub struct PostgresItemModel {
    pub pool: Pool,
}

impl PostgresItemModel {
    const SELECT_ITEM: &str = "SELECT item_id, name FROM restaurant.items WHERE item_id = $1;";
    const SELECT_ALL_ITEMS: &str = "SELECT item_id, name FROM restaurant.items;";
}

#[async_trait]
impl ItemModel for PostgresItemModel {
    async fn get_item(&self, item_id: &ItemId) -> Result<Option<Item>, ItemModelError> {
        let client = self.pool.get().await.map_err(ItemModelError::PoolError)?;

        let statement = client
            .prepare(PostgresItemModel::SELECT_ITEM)
            .await
            .unwrap();

        Ok(client
            .query_opt(&statement, &[&item_id.0])
            .await
            .unwrap()
            .map(|row| {
                let id = row.get("item_id");
                let name = row.get("name");
                Item {
                    item_id: ItemId(id),
                    name,
                }
            }))
    }

    async fn get_items(&self) -> Result<Vec<Item>, ItemModelError> {
        let client = self.pool.get().await.map_err(ItemModelError::PoolError)?;

        let statement = client
            .prepare(PostgresItemModel::SELECT_ALL_ITEMS)
            .await
            .unwrap();

        Ok(client
            .query(&statement, &[])
            .await
            .unwrap()
            .iter()
            .map(|row| {
                let id = row.get("item_id");
                let name = row.get("name");
                Item {
                    item_id: ItemId(id),
                    name,
                }
            })
            .collect::<Vec<Item>>())
    }
}
