use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use tokio_postgres::Client;
use crate::create_response::create_response;

// This route returns all data from the database: the construction is shared among all routes and thus annotated
// We create a mutex (mutual exclusion lock) arc (atomic reference count) to allow async access with shared ownership
// across multiple threads while preventing a data race
pub async fn get_all_data(client: web::Data<Arc<Mutex<Client>>>) -> impl Responder {
    // Obtain a mutable lock on the shared client object which returns a guard that is unlocked when it is dropped
    let client = client.lock().unwrap();

    // This queries the database for the data which returns a tokio:postgres::Row object
    let rows = client
        .query("SELECT * FROM posts", &[])
        .await
        .expect("Error fetching test");

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
