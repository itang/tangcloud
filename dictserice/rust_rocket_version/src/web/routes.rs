use rocket::Rocket;

use web::fairings::XRuntime;
use web::controllers::webroot;
use web::controllers::api::{self, dict};

pub fn routes(rkt: Rocket) -> Rocket {
    rkt.mount("/", routes![webroot::index])
        .mount("/api", routes![api::ping])
        .mount("/api/dict", routes![dict::list, dict::new])
        .attach(XRuntime::default())
        .catch(catchers![webroot::not_found])
}
