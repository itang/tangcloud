use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::prelude::*;
use time::precise_time_ns;


impl typemap::Key for Runtime {
    type Value = u64;
}

header! { (XRuntime, "x-runtime") => [String] }


pub struct Runtime;
impl BeforeMiddleware for Runtime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Runtime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for Runtime {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<Runtime>().unwrap();

        let xrstr = format!("{} ms", (delta as f64) / 1000000.0);
        res.headers.set(XRuntime(xrstr));

        Ok(res)
    }
}
