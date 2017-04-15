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
use rocket_contrib::JSON;


#[derive(Serialize, Debug)]
struct Resp<T: Serialize> {
    ok: bool,
    code: i32,
    message: Option<String>,
    data: Option<T>,
}

impl<T: Serialize> Resp<T> {
    fn json_ok<S: Into<String>>(code: i32, message: Option<S>, data: Option<T>) -> JSON<Resp<T>> {
        Resp::json(true, code, message, data)
    }

    fn json_err<S: Into<String>>(code: i32, message: Option<S>, data: Option<T>) -> JSON<Resp<T>> {
        Resp::json(false, code, message, data)
    }

    fn json<S>(ok: bool, code: i32, message: Option<S>, data: Option<T>) -> JSON<Resp<T>>
        where S: Into<String>
    {
        JSON(Resp {
            ok: ok,
            code: code,
            message: message.map(|x| x.into()),
            data: data,
        })
    }
}


type ResultJSONResp<T/*: Serialize*/, E/*: Serialize*/> = Result<JSON<Resp<T>>, JSON<Resp<E>>>;


#[derive(Serialize, Debug)]
struct Id<T: Serialize>(T);


mod webroot {
    #[get("/")]
    fn index() -> &'static str {
        "Hello, world!"
    }

    #[get("/ping")]
    fn ping() -> &'static str {
        "pong"
    }
}

mod dict {
    use rocket_contrib::JSON;
    use Resp;
    use ResultJSONResp;
    use Id;
    use redis;
    use redis::Commands;
    use rocket::State;
    use serde_json;
    use time;

    const DICT_LOG_KEY: &'static str = "tc:dict:log";
    const DICT_LOG_DATA_KEY: &'static str = "tc:dict:log:data";


    #[derive(Deserialize, Serialize, Debug)]
    struct Log {
        from: String,
        to: String,
    }


    #[get("/logs")]
    // 尝试使用Result表达Action返回值
    fn list(redis: State<redis::Client>) -> ResultJSONResp<Vec<Log>, ()> {
        let conn = redis_conn(redis)?;

        let res: Vec<String> = conn.hvals(DICT_LOG_DATA_KEY)
            .map_err(|_| Resp::json_err(500, Some("Redis error"), None))?;
        let res: Result<Vec<Log>, String> = res.into_iter()
            .map(|it| serde_json::from_str(&it).map_err(|_| "无法获取Redis连接".to_string()))
            .collect();

        res.map(|x| Resp::json_ok(200, Some(""), Some(x)))
            .map_err(|x| Resp::json_err(500, Some(x), None))
    }

    #[post("/logs", format = "application/json", data = "<log>")]
    fn new(log: JSON<Log>, redis: State<redis::Client>) -> ResultJSONResp<Id<i64>, ()> {
        if log.0.from == "hello" {
            return Err(Resp::json_err(404, Some("hello都不知道啊, 老子不干了"), None));
        }

        let conn = redis_conn(redis)?;

        let log_json = serde_json::to_string(&log.0)
            .map_err(|_| Resp::json_err(500, Some("to json error"), None))?;
        let id = timestamp() as i64;
        let value = format!("{}", id);
        let score = id;

        let _: () = conn.zadd(DICT_LOG_KEY, &value, score)
            .map_err(|_| Resp::json_err(500, Some("Redis error"), None))?;
        let _: () = conn.hset(DICT_LOG_DATA_KEY, &value, log_json)
            .map_err(|_| Resp::json_err(500, Some("Redis error"), None))?;

        Ok(Resp::json_ok(200, Some("ok"), Some(Id(id))))
    }

    fn redis_conn(redis: State<redis::Client>) -> Result<redis::Connection, JSON<Resp<()>>> {
        redis.get_connection().map_err(|_| Resp::json_err(500, Some("无法获取Redis连接"), None))
    }

    fn timestamp() -> f64 {
        let timespec = time::get_time();
        // 1459440009.113178
        let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
        mills
    }
}

#[error(404)]
fn not_found() -> JSON<Resp<()>> {
    Resp::json_err(404, Some("Resource was not found."), None)
}

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").expect("open redis error");
    rocket::ignite()
        .mount("/", routes![webroot::index, webroot::ping])
        .mount("/dict", routes![dict::list, dict::new])
        .catch(errors![not_found])
        .manage(client)
        .launch();
}
