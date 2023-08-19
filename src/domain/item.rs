#[derive(Debug, Clone)]
pub struct Item {
    pub item_id: ItemId,
    pub name: String,
}

#[derive(Debug, Clone, Hash, std::cmp::Eq, PartialEq)]
pub struct ItemId(pub String);
