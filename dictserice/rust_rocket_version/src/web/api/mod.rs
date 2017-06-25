#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}

pub mod dict;
