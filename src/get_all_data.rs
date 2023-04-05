use crate::create_response::create_response;
use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, Mutex};
use tokio_postgres::Client;

/// Retrieves all data from the PostgreSQL database and returns it as an HTTP response.
///
/// # Arguments
///
/// * `client` - A reference to an `Arc<Mutex<Client>>` instance used to make the database connection.
/// * `post_id` - An optional `web::Path<i32>` argument representing the post ID to fetch. If `None`, all posts are fetched.
///
/// # Returns
///
/// An HTTP response containing the fetched data as a string.
///
/// # Panics
///
/// This function will panic if there is an error fetching data from the database.
// This route returns all data from the database: the construction is shared among all routes and thus annotated
// We create a mutex (mutual exclusion lock) arc (atomic reference count) to allow async access with shared ownership
// across multiple threads while preventing a data race
pub async fn get_all_data(
    client: web::Data<Arc<Mutex<Client>>>,
    post_id: Option<web::Path<i32>>,
) -> impl Responder {
    let client = client.lock().unwrap();

    let rows = if let Some(post_id) = post_id {
        client
            .query(
                "SELECT * FROM posts WHERE post_id = $1",
                &[&post_id.into_inner()],
            )
            .await
            .expect("Error fetching post")
    } else {
        client
            .query("SELECT * FROM posts", &[])
            .await
            .expect("Error fetching posts")
    };

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
