use time;
use iron;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use serde_json::error::Error as JsonError;
use serde::ser;
use serde_json as json;

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

#[inline]
pub fn json<T>(status: status::Status, value: &T) -> IronResult<Response>
    where T: ser::Serialize
{
    let content_type = "application/json; charset=utf-8".parse::<Mime>().unwrap();
    let s = try!(json::to_string(value).map_err(JsonEncodeErrorWrapper));

    Ok(Response::with((content_type, status, s)))
}

#[inline]
pub fn json_ok<T>(value: &T) -> IronResult<Response>
    where T: ser::Serialize
{
    return json(status::Ok, value)
}

pub struct JsonEncodeErrorWrapper(pub JsonError);

impl From<JsonEncodeErrorWrapper> for IronError {
    fn from(wrapper: JsonEncodeErrorWrapper) -> IronError {
        IronError::new(wrapper.0, (status::InternalServerError, "json encode error"))
    }
}
