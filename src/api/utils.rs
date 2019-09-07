use actix_web::{dev, error, http, web, Error, HttpRequest, HttpResponse, Responder, Result};

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}
