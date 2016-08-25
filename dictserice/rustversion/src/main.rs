#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate iron;
extern crate time;
extern crate router;
extern crate bodyparser;
extern crate serde;
extern crate serde_json;
extern crate redis;


use redis::Commands;
use iron::prelude::*;
use router::Router;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DictLog {
    from_lang: Option<String>,
    from: String,
    to_lang: Option<String>,
    to: String,
}

#[derive(Serialize, Deserialize)]
struct DictLogEntity {
    id: i64,
    from_lang: Option<String>,
    from: String,
    to_lang: Option<String>,
    to: String,
}

const DICT_LOG_KEY: &'static str = "tc:dict:log";
const DICT_LOG_DATA_KEY: &'static str = "tc:dict:log:data";
const REDIS_URL: &'static str = "redis://127.0.0.1/";

fn create_logs(req: &mut Request) -> IronResult<Response> {
    let log = {
        let log_opt = try!(req.get::<bodyparser::Struct<DictLog>>().map_err(|err| badrequest_error(err, "解析json出错")));
        try!(log_opt.ok_or(IronError::new(EmptyError("空json".to_string()), (iron::status::BadRequest, "解析json出错"))))
    };

    let id = timestamp() as i64;

    let log_entity_json = {
        let entity = DictLogEntity { id: id, from: log.from, from_lang: log.from_lang, to: log.to, to_lang: log.to_lang };
        try!(serde_json::to_string(&entity).map_err(|err| server_error(err, "解析json出错")))
    };

    let conn = {
        let client = try!(redis::Client::open(REDIS_URL).map_err(|err| server_error(err, "Redis无法访问")));
        try!(client.get_connection().map_err(|err| server_error(err, "Redis无法访问")))
    };

    let value = format!("{}", id);
    let score = id;

    let _: () = try!(conn.zadd(DICT_LOG_KEY, value.clone(), score).map_err(|err| server_error(err, "Redis操作出错")));
    let _: () = try!(conn.hset(DICT_LOG_DATA_KEY, value.clone(), log_entity_json).map_err(|err| server_error(err, "Redis操作出错")));

    Ok(Response::with((iron::status::Created, "")))
}


fn list_logs(_: &mut Request) -> IronResult<Response> {
    let client = redis::Client::open(REDIS_URL).unwrap();
    let conn = client.get_connection().unwrap();

    let res: Vec<String> = conn.hvals(DICT_LOG_DATA_KEY).unwrap();
    println!("res: {:?}", res);
    let res: Vec<DictLogEntity> = res.into_iter().map(|it| serde_json::from_str(&it).unwrap()).collect();

    Ok(Response::with((iron::status::Ok, serde_json::to_string(&res).unwrap())))
}

// TODO:
// 错误转化
// 链式处理
// Redis 连接管理
fn main() {
    let mut router = Router::new();
    router.post("/dict/logs", create_logs);
    router.get("/dict/logs", list_logs);

    let chain = Chain::new(router);
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn timestamp() -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills
}

fn server_error<E: iron::Error>(cause: E, error: &str) -> IronError {
    IronError::new(Box::new(cause), (iron::status::InternalServerError, error))
}

fn badrequest_error<E: iron::Error>(cause: E, error: &str) -> IronError {
    IronError::new(Box::new(cause), (iron::status::BadRequest, error))
}

#[derive(Debug)]
struct EmptyError(String);

impl std::fmt::Display for EmptyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "EmptyError error {}", self.0)
    }
}

impl std::error::Error for EmptyError {
    fn description(&self) -> &str {
        &self.0
    }
}
