use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is the index of the tennis db api. Fire away with your request!")
}
