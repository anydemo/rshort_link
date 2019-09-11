#[macro_use]
extern crate log;
#[warn(unused_imports)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate juniper;

use crate::gql_schema::create_schema;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_redis::RedisSession;
use actix_web::middleware::{errhandlers::ErrorHandlers, Logger};
use actix_web::{http, web, App, HttpServer};
use std::sync::Arc;
use std::{env, error::Error};

mod api;
mod db;
mod gql_api;
mod gql_schema;
mod model;
mod schema;
mod session;

pub fn run(config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    let run_config = Arc::clone(&config);
    debug!("input config = {:?}", run_config);

    let db_pool = db::init_pool(&config.db_link).expect("Failed to create pool");
    let gql_shma = std::sync::Arc::new(create_schema());

    let app_config = Arc::clone(&config);
    let app = move || {
        let config = Arc::clone(&app_config);

        debug!("Constructing the App");
        let error_handlers = ErrorHandlers::new()
            .handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                api::err::internal_server_error,
            )
            .handler(http::StatusCode::BAD_REQUEST, api::err::bad_request)
            .handler(http::StatusCode::NOT_FOUND, api::err::not_found);

        App::new()
            .data(db_pool.clone())
            .data(gql_shma.clone())
            .data(web::JsonConfig::default().limit(4096))
            .wrap(Logger::default())
            .wrap(RedisSession::new(&config.redis_link, &[0; 32]))
            .wrap(error_handlers)
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("rshort_link_auth")
                    .secure(false),
            ))
            .service(web::resource("/login").route(web::post().to(api::auth::back_door_handler)))
            .service(web::resource("/logout").to(api::auth::logout))
            // .service(web::resource("/").route(web::get().to(api::index)))
            // init graphql endpoint
            .service(web::resource("/graphql").route(web::post().to_async(gql_api::graphql)))
            .service(web::resource("/graphiql").route(web::get().to(gql_api::graphiql)))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/gen")
                            .route(web::post().to(api::short_link::generate_short_link)),
                    )
                    .service(
                        web::resource("/origin/{link_id}")
                            .route(web::get().to(api::short_link::redirect_from_short_link)),
                    ),
            )
    };

    info!("Starting server listen on {}", &run_config.listen_on);
    if let Err(err) = HttpServer::new(app)
        .workers(2)
        .bind(&run_config.listen_on)?
        .run()
    {
        error!("err: {}", err);
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    pub db_link: String,
    pub redis_link: String,
    pub listen_on: String,
}

impl Config {
    pub fn new() -> Result<Arc<Config>, &'static str> {
        info!("------- env --------");
        for (k, v) in env::vars() {
            info!("{}={}", k, v);
        }
        info!("====== env end =======");
        Ok(Arc::new(Config {
            db_link: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            redis_link: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            listen_on: env::var("LISTEN_ON").unwrap_or(String::from("127.0.0.1:8088")),
        }))
    }
}

#[cfg(test)]
mod test_run {
    use super::*;
    #[test]
    fn test_run() {
        let config = Config::new();
        println!("config = {:?}", config);
    }
}
