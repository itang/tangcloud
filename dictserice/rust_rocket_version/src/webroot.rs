use types::Resp;
use rocket_contrib::JSON;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[error(404)]
fn not_found() -> JSON<Resp<()>> {
    Resp::json_err(404, Some("Resource was not found."), None)
}
