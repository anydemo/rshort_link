use crate::api::utils;
use crate::db::{
    self,
    tiny_link::{NewTinyLink, TinyLink},
};
use actix_session::Session;
use actix_web::{http::StatusCode, web, HttpResponse, Result};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InputGenLink {
    pub link: String,
}

pub fn generate_short_link(
    session: Session,
    input: web::Json<InputGenLink>,
    pool: web::Data<db::PgPool>,
) -> Result<HttpResponse> {
    let user_id: Option<String> = session.get::<String>("user_id").unwrap();
    let tiny_link = NewTinyLink::new(&input.link, &user_id);
    if let Err(err) = db::create_tiny_link(tiny_link.clone(), &pool) {
        error!(
            "generate short link err, link: {}, err: {}",
            &tiny_link.origin, err
        );
        Ok(HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
    } else {
        Ok(HttpResponse::Ok().json(tiny_link))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InputRedirectFromShortLink {
    pub link_id: String,
}

pub fn redirect_from_short_link(
    db_pool: web::Data<db::PgPool>,
    pool: web::Data<db::PgPool>,
    input: web::Path<InputRedirectFromShortLink>,
) -> Result<HttpResponse> {
    match db::find_tiny_link(&input.link_id, &db_pool) {
        Ok(val) => Ok(utils::redirect_to(&val.origin)),
        Err(err) => {
            error!("redirect({}) err: {}", &input.link_id, err);
            Ok(HttpResponse::new(StatusCode::NOT_FOUND))
        }
    }
}
