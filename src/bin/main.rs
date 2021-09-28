use std::sync::Mutex;

use ::iot::*;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use awc::Client;
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

#[derive(Default)]
struct MultipleValues {
    values: Mutex<Vec<u32>>,
}

#[get("/")]
async fn index(current_value: web::Data<Value>) -> impl Responder {
    let current_value = current_value.value.lock().unwrap();
    let index_page = Index::new(*current_value).render().unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(&index_page)
}

#[get("/update")]
async fn update_value(
    update_value: web::Query<UpdateValue>,
    value: web::Data<Value>,
) -> impl Responder {
    let mut value = value.value.lock().unwrap();
    *value = update_value.value;
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(&value.to_string())
}

#[get("/read")]
async fn read_value(current_value: web::Data<Value>) -> impl Responder {
    let value = *current_value.value.lock().unwrap();

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(&value.to_string())
}

#[get("/aggregate")]
async fn aggregate(current_value: web::Data<Value>) -> impl Responder {
    let addresses = vec![
        "http://asjadeinternet2021.000webhostapp.com/valgus.txt",
        "http://raimondlaatspera.000webhostapp.com/ValgusKontroll.txt",
        "http://testwebsite33.000webhostapp.com/27.09.21/valgus.txt",
    ];

    let ise = HttpResponse::InternalServerError().finish();

    let mut obtained_values: Vec<String> = Default::default();
    let client = Client::default();

    for address in &addresses {
        let response = client.get(*address).send().await;

        let body = match response {
            Ok(mut client_response) => client_response.body().await,
            Err(_) => return ise,
        };

        let content = match body {
            Ok(content) => content.to_ascii_lowercase(),
            Err(_) => return ise,
        };

        let string = match String::from_utf8(content) {
            Ok(content) => content,
            Err(_) => return ise,
        };

        match string.clone().split('\n').last() {
            Some(val) => obtained_values.push(val.to_string()),
            None => (),
        }
    }

    let mut final_string = addresses.join(", ");
    final_string.push('\n');

    HttpResponse::Ok()
        .content_type("text/csv")
        .body(final_string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:80/");

    HttpServer::new(move || {
        App::new()
            .data(Value::default())
            .service(update_value)
            .service(read_value)
            .service(index)
            .service(aggregate)
    })
    .bind(&ADDRESS)?
    .run()
    .await
}
