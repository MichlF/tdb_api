use actix_web::{HttpResponse, Responder};

/// This function serves as the endpoint for the index route of the tennis db api.
/// It returns a response with an HTTP status code of 200 (OK) and a message to indicate that the server is up and running.
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is the index of the tennis db api. Fire away with your request!")
}
