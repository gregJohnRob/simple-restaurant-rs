use std::sync::Arc;
use deadpool_postgres::Pool;

pub mod item_model;

#[derive(Clone)]
pub struct Model {
    pub item_model: Arc<dyn item_model::ItemModel + Sync + Send>,
}

impl Model {
    pub fn build(pool: Pool) -> Model {
        Model {
            item_model: Arc::new(item_model::PostgresItemModel{pool}),
        }
    }
}
