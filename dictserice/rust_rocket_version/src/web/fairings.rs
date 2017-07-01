use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use time;


#[derive(Default)]
pub struct XRuntime {}

const XRUNTIME_KEY: &'static str = "x-runtime";

//TODO: 使用Request context(state)机制 ??
impl Fairing for XRuntime {
    fn info(&self) -> Info {
        Info {
            name: "GET/POST x-runtime",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _data: &Data) {
        let start_str = format!("{}", time::precise_time_ns());
        request.add_header(Header::new(XRUNTIME_KEY, start_str));
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        let start_str = request.headers().get_one(XRUNTIME_KEY).unwrap();
        let start = start_str.parse::<u64>().unwrap();
        let now = time::precise_time_ns();
        let value = format!("{} ms", ((now - start) as f64) / 1000.0 / 1000.0);

        response.set_header(Header::new(XRUNTIME_KEY, value));
    }
}
