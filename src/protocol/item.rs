use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub item_id: String,
    pub name: String,
}