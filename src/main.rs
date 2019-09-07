use dotenv::dotenv;
use env_logger;
use log;
use rshort_link::{self, Config};
use std::env;
use std::process;

fn main() {
    dotenv().ok();
    env::set_var("RUST_LOG", "rshort_link=trace,actix_web=info");
    env_logger::init();

    log::info!("RUST_LOG: {:?}", env::var("RUST_LOG"));

    let config = Config::new().unwrap_or_else(|err| {
        log::error!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rshort_link::run(config) {
        log::error!("App err: {}", e);
        process::exit(1);
    }

    log::info!("Stop!");
}
