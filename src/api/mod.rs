use actix_web::{get, web, HttpResponse, Responder};

use crate::protocol::item::Item;

#[get("/items/{id}")]
pub async fn get_item(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(Item {
        item_id: id.to_string(),
        name: "pizza".to_string(),
    })
}

#[get("/items")]
pub async fn get_items() -> impl Responder {
    HttpResponse::Ok().json(vec![
        Item {
            item_id: "0".to_string(),
            name: "pizza".to_string(),
        },
        Item {
            item_id: "1".to_string(),
            name: "beer".to_string(),
        },
    ])
}
