use std::sync::Mutex;

use ::iot::*;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use iot::index::Index;
use serde::Deserialize;

#[derive(Default)]
struct Value {
    value: Mutex<u32>,
}

#[derive(Deserialize)]
struct UpdateValue {
    value: u32,
}

#[get("/")]
async fn index(current_value: web::Data<Value>) -> impl Responder {
    let current_value = current_value.value.lock().unwrap();
    let index_page = Index::new(*current_value).render().unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(&index_page)
}

#[get("/u/")]
async fn update_value(
    update_value: web::Query<UpdateValue>,
    value: web::Data<Value>,
) -> impl Responder {
    let mut value = value.value.lock().unwrap();
    *value = update_value.value;
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:80/");

    HttpServer::new(move || {
        App::new()
            .data(Value::default())
            .service(update_value)
            .service(index)
    })
    .bind(&ADDRESS)?
    .run()
    .await
}
