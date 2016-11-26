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
#[macro_use]
extern crate hyper;

use std::env;

mod global;
mod utils;
mod types;
mod handles;
mod models;
mod middlewares;

use iron::prelude::*;
use router::Router;
use mount::Mount;

fn main() {
    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>().expect("port 值不合法"),
        Err(_) => 9903,
    };

    log4rs::init_file("config/log4rs.toml", Default::default()).unwrap();

    let mut api_router = Router::new();
    api_router.any("/ping", handles::ping, "ping");
    api_router.post("/dict/logs",
                    |req: &mut Request| global::LOG_CONTROLLER.create_logs(req),
                    "logs_create");
    api_router.get("/dict/logs",
                   |req: &mut Request| global::LOG_CONTROLLER.list_logs(req),
                   "logs_list");

    let mut api_chain = Chain::new(api_router);
    api_chain.link_before(middlewares::Runtime);
    api_chain.link_after(middlewares::Runtime);

    let mut mount = Mount::new();
    mount.mount("/api", api_chain);

    info!("Listen on :{}...", port);
    // let chain = Chain::new(router);

    Iron::new(mount).http(("localhost", port)).unwrap();
}
