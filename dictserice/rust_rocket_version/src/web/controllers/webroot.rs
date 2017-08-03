use types::Resp;
use rocket_contrib::Json;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world! - rustrocketversion-v0.1"
}

#[error(404)]
pub fn not_found() -> Json<Resp<()>> {
    Resp::json_err(404, Some("Resource was not found."), None)
}
