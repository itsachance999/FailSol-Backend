use actix::Actor;
use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web::{self, Data}, App, HttpServer};
use env_logger::Env;

mod model;
mod routes;
mod services;

mod utils;
use routes::signature::{get_image, get_signatures_handler,get_specific_signature};
use services::db::Database;

use crate::{routes::{update::{get_histories, update_img_url}, ws::get_ws}, services::ws::lobby::WsServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let db = Database::init().await.unwrap();
    let db_data = Data::new(db);
    let ws_server = WsServer::new().start();

    let server = HttpServer::new(move || {
        // Create CORS middleware configuration
        let cors = Cors::default()
        .allow_any_origin()
        .allow_any_header()
        .allow_any_method();
        App::new()
            .wrap(Logger::default())
            .wrap(cors) // Apply CORS middleware globally
            .app_data(db_data.clone())
            .app_data(web::Data::new(ws_server.clone()))
            .service(get_signatures_handler)
            .service(get_image).service(get_specific_signature).service(update_img_url).service(get_ws).service(get_histories)
    })
    .bind(("0.0.0.0", 8081))?;

    println!("Server is running on port 8081");

    server.run().await
}
