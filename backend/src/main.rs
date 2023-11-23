// src/main.rs
use actix_web::{App, HttpServer};
use actix_web::web;
mod controllers;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/user/{id}").route(web::get().to(controllers::get_user_by_id)))
    })
    .bind("{your_ip}:8080")?
    .run()
    .await
}
