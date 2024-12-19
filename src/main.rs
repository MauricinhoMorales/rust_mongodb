mod config;
mod models;
mod routes;
mod services;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use routes::wallet_route::*;
use services::db::Database;

#[get("/")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("RUNNING...")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(test)
            .service(create_wallet)
    })
    .bind(("127.0.0.1", 5001))?
    .run()
    .await
}
