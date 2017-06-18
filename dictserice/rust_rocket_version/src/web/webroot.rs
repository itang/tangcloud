use types::Resp;
use rocket_contrib::JSON;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world! - rustrocketversion-v0.1"
}

#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}

#[error(404)]
pub fn not_found() -> JSON<Resp<()>> {
    Resp::json_err(404, Some("Resource was not found."), None)
}
