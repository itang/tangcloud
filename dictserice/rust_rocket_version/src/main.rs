#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;
//#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate redis;
extern crate time;

use serde::ser::Serialize;
use rocket_contrib::{JSON};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
#[allow(non_camel_case_types)]
enum Reply<T: Serialize, E: Serialize> {
    ok {
        message: Option<String>,
        data: Option<T>,
    },
    error {
        code: i32,
        message: Option<String>,
        data: Option<E>,
    },
}

#[derive(Serialize, Debug)]
struct Id<T: Serialize>(T);

use Reply::error;

mod dict {
    use rocket_contrib::{JSON};
    use Reply::{self, ok, error};
    use Id;
    use redis;
    use redis::Commands;
    use rocket::State;
    use serde_json;
    use time;

    const DICT_LOG_KEY: &'static str = "tc:dict:log";
    const DICT_LOG_DATA_KEY: &'static str = "tc:dict:log:data";

    fn timestamp() -> f64 {
        let timespec = time::get_time();
        // 1459440009.113178
        let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
        mills
    }


    #[derive(Deserialize, Serialize, Debug)]
    struct Log {
        from: String,
        to: String,
    }

    #[get("/dict/logs")]
    fn list(redis: State<redis::Client>) -> JSON<Reply<Vec<Log>, ()>> {
        //TODO: 错误处理
        let conn = redis.get_connection().expect("无法获取Redis连接");

        let res: Vec<String> = conn.hvals(DICT_LOG_DATA_KEY).unwrap();
        let res: Vec<Log> = res.into_iter()
            .map(|it| serde_json::from_str(&it).unwrap())
            .collect();

        JSON(ok {
                 message: None,
                 data: Some(res),
             })
    }

    #[post("/dict/logs", format = "application/json", data = "<log>")]
    fn new(log: JSON<Log>, redis: State<redis::Client>) -> JSON<Reply<Id<i64>, ()>> {
        //TODO: 错误处理
        if log.0.from == "hello" {
            return JSON(error {
                            code: 404,
                            message: Some("hello都不知道啊, 老子不干了".to_string()),
                            data: None,
                        });
        }

        let conn = redis.get_connection().expect("无法获取Redis连接");

        let log_json = serde_json::to_string(&log.0).unwrap();
        let id = timestamp() as i64;
        let value = format!("{}", id);
        let score = id;

        let _: () = conn.zadd(DICT_LOG_KEY, &value, score).unwrap();
        let _: () = conn.hset(DICT_LOG_DATA_KEY, &value, log_json).unwrap();
        JSON(ok {
                 message: Some("ok".to_string()),
                 data: Some(Id(id)),
             })
    }
}

#[error(404)]
fn not_found() -> JSON<Reply<(), ()>> {
    JSON(error {
             code: 404,
             message: Some("Resource was not found.".to_string()),
             data: None,
         })
}

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    rocket::ignite()
        .mount("/", routes![index, ping, dict::list, dict::new])
        .catch(errors![not_found])
        .manage(client)
        .launch();
}
