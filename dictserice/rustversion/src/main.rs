#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

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
    router.post("/dict/logs", handles::create_logs);
    router.get("/dict/logs", handles::list_logs);

    info!("Listen on :3000...");
    let chain = Chain::new(router);
    Iron::new(chain).http("localhost:3000").unwrap();
}
