use types::Resp;
use rocket_contrib::Json;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world! - rust rocket version - v0.1"
}

#[catch(404)]
pub fn not_found() -> Json<Resp<()>> {
    Resp::json_err(404, Some("Resource was not found."), None)
}
