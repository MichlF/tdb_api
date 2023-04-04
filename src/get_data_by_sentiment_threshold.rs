use crate::create_response::create_response;
use crate::data_structs::SentimentThresholdQuery;
use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use tokio_postgres::Client;

pub async fn get_data_by_sentiment_threshold(
    client: web::Data<Arc<Mutex<Client>>>,
    query: web::Query<SentimentThresholdQuery>,
) -> impl Responder {
    let query_params = query.into_inner();

    let client = client.lock().unwrap();

    let rows = client
        .query(
            "SELECT * FROM posts WHERE sentiment > $1 AND sentiment < $2",
            &[&query_params.lower, &query_params.upper],
        )
        .await
        .expect("Error fetching data");

    if rows.is_empty() {
        return HttpResponse::Ok().body("No data available for your request.");
    }

    let response = rows
        .into_iter()
        .map(create_response)
        .collect::<Vec<String>>()
        .join("\n");

    HttpResponse::Ok().body(response)
}
