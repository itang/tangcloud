#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;
extern crate time;
extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate serde;
extern crate serde_json;
extern crate redis;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate r2d2;
extern crate r2d2_redis;

mod global;
mod utils;
mod types;
mod handles;


use iron::prelude::*;
use router::Router;

fn main() {
    log4rs::init_file("config/log4rs.toml", Default::default()).unwrap();

    let mut router = Router::new();
    router.any("/api/ping", handles::ping, "ping");
    router.post("/api/dict/logs", handles::create_logs, "logs_create");
    router.get("/api/dict/logs", handles::list_logs, "logs_list");

    info!("Listen on :8080...");
    let chain = Chain::new(router);
    Iron::new(chain).http("localhost:8080").unwrap();
}
