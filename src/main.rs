mod hambone;

use actix_web::{get, web, Responder, Result};
use serde::Serialize;
use hambone::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(||
            App::new()
                .service(index)
                .service(hambone::all_v1_hambones)
        )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await}

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok("try /v1/hambones".to_string())
}