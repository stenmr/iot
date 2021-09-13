use std::sync::Mutex;

use ::iot::*;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use iot::index::Index;
use serde::Deserialize;

#[derive(Default)]
struct Temperature {
    value: Mutex<f32>,
}

#[derive(Deserialize)]
struct UpdateTemp {
    temp: f32,
}

#[get("/")]
async fn index(current_temp: web::Data<Temperature>) -> impl Responder {
    let current_temp = current_temp.value.lock().unwrap();
    let index_page = Index::new(*current_temp).render().unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(&index_page)
}

#[get("/u/")]
async fn update_temp(
    temp: web::Query<UpdateTemp>,
    current_temp: web::Data<Temperature>,
) -> impl Responder {
    let mut value = current_temp.value.lock().unwrap();
    *value = temp.temp as f32;
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:80/");

    HttpServer::new(move || {
        App::new()
            .data(Temperature::default())
            .service(update_temp)
            .service(index)
    })
    .bind(&ADDRESS)?
    .run()
    .await
}
