#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

extern crate redis;

extern crate rust_rocket_version;

use rocket::config;

use rust_rocket_version::web::webroot;
use rust_rocket_version::web::api::{self, dict};


fn main() {
    let rkt = rocket::ignite();
    let config = config::active().unwrap();
    let redis_url = config.get_str("redis_url").expect(
        "can't get redis url from config",
    );
    let client = redis::Client::open(redis_url).expect("open redis error");

    rkt.mount("/", routes![webroot::index])
        .mount("/api", routes![api::ping])
        .mount("/api/dict", routes![dict::list, dict::new])
        .catch(errors![webroot::not_found])
        .manage(client)
        .launch();
}
