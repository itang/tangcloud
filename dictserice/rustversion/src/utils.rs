use time;
use iron::{self, IronError};

pub fn timestamp() -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills
}

pub fn server_error<E: iron::Error>(cause: E, error: &str) -> IronError {
    IronError::new(Box::new(cause), (iron::status::InternalServerError, error))
}

pub fn badrequest_error<E: iron::Error>(cause: E, error: &str) -> IronError {
    IronError::new(cause, (iron::status::BadRequest, error))
}
