use actix_web::{get, web, HttpResponse, Responder};

use crate::{domain::item::ItemId, model::Model, protocol::item::Item};

#[get("/items/{id}")]
pub async fn get_item(model: web::Data<Model>, id: web::Path<String>) -> impl Responder {
    match model.item_model.get_item(&ItemId(id.into_inner())).await {
        Ok(Some(item)) => HttpResponse::Ok().json(Item::from(item)),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/items")]
pub async fn get_items(model: web::Data<Model>) -> impl Responder {
    match model.item_model.get_items().await {
        Ok(items) => HttpResponse::Ok().json(items.into_iter().map(Item::from).collect::<Vec<_>>()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
