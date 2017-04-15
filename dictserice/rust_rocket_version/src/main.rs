#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;
//#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate redis;
extern crate time;


mod types;
mod webroot;
mod dict;


use rocket::config;


fn main() {
    let rkt = rocket::ignite();
    let config = config::active().unwrap();
    let redis_url = config
        .get_str("redis_url")
        .expect("can't get redis url from config");
    let client = redis::Client::open(redis_url).expect("open redis error");

    rkt.mount("/", routes![webroot::index, webroot::ping])
        .mount("/dict", routes![dict::list, dict::new])
        .catch(errors![webroot::not_found])
        .manage(client)
        .launch();
}
