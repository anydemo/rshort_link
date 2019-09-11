pub mod auth;
pub mod err;
pub mod short_link;

mod utils;

use crate::db::tiny_link::{NewTinyLink, TinyLink};
use actix_session::Session;
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

/// TODO: move to model
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct IndexResponse {
    user_id: Option<String>,
    counter: i32,
}

pub fn index(session: Session) -> Result<HttpResponse> {
    let user_id: Option<String> = session.get::<String>("user_id").unwrap();
    let counter: i32 = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .unwrap_or(0);
    Ok(HttpResponse::Ok().json(IndexResponse { user_id, counter }))
}
