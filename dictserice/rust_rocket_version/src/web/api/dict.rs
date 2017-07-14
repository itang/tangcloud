use rocket_contrib::Json;
use redis;
use redis::Commands;
use rocket::State;
use serde_json;
use time;
use serde::ser::Serialize;

use types::{Id, Resp, ResultJSONResp};


const DICT_LOG_KEY: &'static str = "tc:dict:log";
const DICT_LOG_DATA_KEY: &'static str = "tc:dict:log:data";


#[derive(Deserialize, Serialize, Debug)]
struct LogForm {
    from: String,
    to: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct LogEntity {
    #[serde(default = "empty_id")]
    id: Id<i64>,
    from: String,
    to: String,
    #[serde(default = "none_i64")]
    created_at: Option<i64>,
    #[serde(default = "none_i64")]
    updated_at: Option<i64>,
}


#[post("/logs", format = "application/json", data = "<log>")]
fn new(log: Json<LogForm>, redis: State<redis::Client>) -> ResultJSONResp<Id<i64>, ()> {
    let conn = redis_conn(redis)?;

    let LogForm { from, to } = log.0;
    if from == "hello" {
        return Err(Resp::json_err(
            404,
            Some("hello都不知道啊, 老子不干了"),
            None,
        ));
    }

    let timestamp_int = timestamp() as i64;
    let id = Id(timestamp_int);
    let value = format!("{}", timestamp_int);
    let score = timestamp_int;

    let log_entity = LogEntity {
        id: id.clone(),
        from: from,
        to: to,
        created_at: Some(timestamp_int),
        updated_at: Some(timestamp_int),
    };

    let log_json = serde_json::to_string(&log_entity).map_err(|_| {
        Resp::json_err(500, Some("to json error"), None)
    })?;


    let _: () = conn.zadd(DICT_LOG_KEY, &value, score).map_err(|_| {
        Resp::json_err(500, Some("Redis error"), None)
    })?;
    let _: () = conn.hset(DICT_LOG_DATA_KEY, &value, log_json).map_err(
        |_| {
            Resp::json_err(500, Some("Redis error"), None)
        },
    )?;

    Ok(Resp::json_ok(200, Some("ok"), Some(id)))
}

#[get("/logs")]
// FIXME: Result::Err表达server internal error(500)
fn list(redis: State<redis::Client>) -> ResultJSONResp<Vec<LogEntity>, ()> {
    let conn = redis_conn(redis)?;

    let res: Vec<String> = conn.hvals(DICT_LOG_DATA_KEY).map_err(|_| {
        Resp::json_err(500, Some("Redis error"), None)
    })?;
    let res: Result<Vec<LogEntity>, String> = res.into_iter()
        .map(|it| {
            serde_json::from_str(&it).map_err(|_| "无法获取Redis连接".to_string())
        })
        .collect();

    res.map(|x| Resp::json_ok(200, Some(""), Some(x))).map_err(
        |x| {
            Resp::json_err(500, Some(x), None)
        },
    )
}


fn redis_conn(redis: State<redis::Client>) -> Result<redis::Connection, Json<Resp<()>>> {
    redis.get_connection().map_err(|_| {
        Resp::json_err(500, Some("无法获取Redis连接"), None)
    })
}

fn timestamp() -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills
}

fn none_i64<T: Serialize>() -> Option<T> {
    None
}

fn empty_id() -> Id<i64> {
    Id(-1)
}
