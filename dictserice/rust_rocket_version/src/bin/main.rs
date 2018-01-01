#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate redis;
extern crate rocket;
extern crate rust_rocket_version;

use rust_rocket_version::web::routes::routes;

fn main() {
    let rkt = rocket::ignite();

    let config = rkt.config().clone();
    let redis_url = config
        .get_str("redis_url")
        .expect("can't get redis url from config");
    let redis_client = redis::Client::open(redis_url).expect("open redis error");

    routes(rkt).manage(redis_client).launch();
}
