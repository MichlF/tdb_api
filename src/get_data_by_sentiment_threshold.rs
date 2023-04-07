use crate::create_response::create_response;
use crate::models::SentimentThresholdQuery;
use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use tokio_postgres::Client;

/// Retrieves data from the database based on the given sentiment threshold query.
///
/// The function takes a client object as well as a query object that contains the lower and upper
/// bounds of the sentiment threshold. It retrieves data from the database that has a sentiment
/// score greater than the lower bound and less than the upper bound. The data is returned as an
/// HTTP response containing a string with a list of data entries formatted using the create_response
/// function.
///
/// # Arguments
///
/// * `client` - A `web::Data<Arc<Mutex<Client>>>` object representing the database client.
///
/// * `query` - A `web::Query<SentimentThresholdQuery>` object containing the lower and upper bounds
///             of the sentiment threshold.
///
/// # Returns
///
/// An HttpResponse object containing the retrieved data entries in string format. If there is no
/// data available for the requested query, a message is returned indicating this.
///
/// # Panics
///
/// The function will panic if there is an error while fetching data from the database.

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
