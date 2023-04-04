use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use tokio_postgres::NoTls;

mod create_response;
mod data_structs;
mod index;
mod get_all_data;
mod get_data_by_date;
mod get_data_by_sentiment_threshold;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let connection_string =
        std::env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .expect("Error connecting to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("error: {}", e);
        }
    });

    let client = Arc::new(Mutex::new(client));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/", web::get().to(index::index))
            .route("/get_alldata", web::get().to(get_all_data::get_all_data)) // Example: http://127.0.0.1:8080/get_alldata
            .route("/get_data_by_sentiment_threshold", web::get().to(get_data_by_sentiment_threshold::get_data_by_sentiment_threshold)) // Example: http://127.0.0.1:8080/get_data_by_sentiment_threshold?upper=0.9&lower=-0.3
            .route("/get_data_by_date", web::get().to(get_data_by_date::get_data_by_date)) // Example: http://127.0.0.1:8080/get_data_by_date?start=01-12-2020&end=31-02-2021
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
