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
extern crate mount;

mod global;
mod utils;
mod types;
mod handles;
mod models;

use iron::prelude::*;
use router::Router;
use mount::Mount;

fn main() {
    log4rs::init_file("config/log4rs.toml", Default::default()).unwrap();

    let mut api_router = Router::new();
    api_router.any("/ping", handles::ping, "ping");
    api_router.post("/dict/logs",
                    |req: &mut Request| global::LOG_CONTROLLER.create_logs(req),
                    "logs_create");
    api_router.get("/dict/logs",
                   |req: &mut Request| global::LOG_CONTROLLER.list_logs(req),
                   "logs_list");

    let mut mount = Mount::new();
    mount.mount("/api", api_router);

    info!("Listen on :8080...");
    // let chain = Chain::new(router);

    Iron::new(mount).http("localhost:8080").unwrap();
}
