use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use tokio_postgres::NoTls;

mod create_response;
mod data_structs;
mod get_all_data;
mod get_data_by_date;
mod get_data_by_sentiment_threshold;
mod index;

/// Starts the actix-web server that serves HTTP requests for the sentiment analysis data.
///
/// The server connects to a PostgreSQL database specified in the environment variable
/// `DATABASE_URL`. The server provides the following endpoints:
///
/// * `GET /`: Returns a greeting message to confirm the server is up and running.
///
/// * `GET /posts/`: Returns all posts in the database in JSON format.
///
/// * `GET /posts/{post_id}`: Returns a single post specified by its ID in JSON format.
///
/// * `GET /posts/sentiment_threshold`: Returns all posts in the database whose sentiment scores
///       fall between the specified upper and lower thresholds in JSON format.
///
/// * `GET /posts/date`: Returns all posts in the database whose date of publication falls
///       between the specified start and end dates in JSON format.
///
/// The server uses Arc and Mutex from the `std::sync` module to safely share a single instance of
/// a tokio_postgres::Client object among multiple threads handling incoming requests.
///
/// # Examples
///
/// Starting the server:
///
/// ```
/// use std::io;
/// fn main() -> io::Result<()> {
///     sentiment_analysis_server::main()
/// }
/// ```
///
/// After starting the server, you can test the different endpoints using tools such as curl
/// or a web browser, for example:
///
/// ```
/// curl http://localhost:8080/
/// curl http://localhost:8080/posts/
/// curl http://localhost:8080/posts/123
/// curl http://localhost:8080/posts/sentiment_threshold?upper=0.9&lower=-0.3
/// curl http://localhost:8080/posts/date?start=01-12-2020&end=31-02-2021
/// ```
/// This will return the corresponding responses for each endpoint.
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
            .route("/posts/", web::get().to(get_all_data::get_all_data))
            .route(
                "/posts/{post_id}",
                web::get().to(get_all_data::get_all_data),
            )
            .route(
                "/posts/sentiment_threshold",
                web::get().to(get_data_by_sentiment_threshold::get_data_by_sentiment_threshold),
            )
            .route(
                "/posts/date",
                web::get().to(get_data_by_date::get_data_by_date),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
