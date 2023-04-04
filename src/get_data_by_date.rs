use crate::create_response::create_response;
use crate::data_structs::DateTimeRangeQuery;
use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use tokio_postgres::Client;

pub async fn get_data_by_date(
    client: web::Data<Arc<Mutex<Client>>>,
    query: web::Query<DateTimeRangeQuery>,
) -> impl Responder {
    let query_params = query.into_inner();

    // Parse the start and end dates from the query parameters
    let start_date = match chrono::NaiveDate::parse_from_str(&query_params.start, "%d-%m-%Y") {
        Ok(date) => date,
        Err(e) => match e {
            _ => {
                return HttpResponse::BadRequest()
                    .body("An error occurred while parsing the start date.");
            }
        }
    };
    let end_date = match chrono::NaiveDate::parse_from_str(&query_params.end, "%d-%m-%Y") {
        Ok(date) => date,
        Err(e) => match e {
            _ => {
                return HttpResponse::BadRequest()
                    .body("An error occurred while parsing the end date.");
            }
        }
    };

    // Convert the start and end dates to UNIX timestamps
    let start_timestamp = start_date.and_hms_opt(0, 0, 0).unwrap().timestamp();
    let end_timestamp = end_date.and_hms_opt(23, 59, 59).unwrap().timestamp();

    // Obtain a mutable lock on the shared client object which returns a guard that is unlocked when it is dropped
    let client = client.lock().unwrap();

    // This queries the database for the data which returns a tokio:postgres::Row object
    let rows = client
        .query(
            "SELECT * FROM posts WHERE date >= $1 AND date <= $2",
            &[&start_timestamp, &end_timestamp],
        )
        .await
        .expect("Error fetching data");

    if rows.is_empty() {
        return HttpResponse::Ok().body("No data available for your request.");
    }

    // Create the response string by mapping over the rows and calling the create_response function for each row
    let response = rows
        .into_iter()
        .map(create_response)
        .collect::<Vec<String>>()
        .join("\n");

    // And return it
    HttpResponse::Ok().body(response)
}