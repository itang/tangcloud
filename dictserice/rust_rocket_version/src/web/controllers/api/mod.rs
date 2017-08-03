pub mod dict;

#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}
