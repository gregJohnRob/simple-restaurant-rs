pub mod item_model;

pub struct Model {
    pub item_model: Box<dyn item_model::ItemModel>,
}

impl Model {
    pub fn build() -> Model {
        Model {
            item_model: Box::new(item_model::DummyItemModel::default()),
        }
    }
}
