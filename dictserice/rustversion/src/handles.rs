use iron;
use iron::prelude::*;
use redis::Commands;
use bodyparser;
use serde_json;

use types::*;
use utils::*;
use global::*;


const DICT_LOG_KEY: &'static str = "tc:dict:log";
const DICT_LOG_DATA_KEY: &'static str = "tc:dict:log:data";

pub fn create_logs(req: &mut Request) -> IronResult<Response> {
    info!("create_logs...");
    let log = {
        let log_opt = try!(req.get::<bodyparser::Struct<DictLog>>()
            .map_err(|err| badrequest_error(err, "解析json出错")));
        try!(log_opt.ok_or(IronError::new(EmptyError("空json".to_string()),
                                          (iron::status::BadRequest, "解析json出错"))))
    };

    let id = timestamp() as i64;

    let entity = DictLogEntity {
        id: id,
        from: log.from,
        from_lang: log.from_lang,
        to: log.to,
        to_lang: log.to_lang,
    };
    let log_entity_json = {
        try!(serde_json::to_string(&entity).map_err(|err| server_error(err, "解析json出错")))
    };

    info!("log_entity_json: {:?}", log_entity_json);

    let conn = try!(conn_pool().get().map_err(|err| server_error(err, "Redis无法访问")));

    let value = format!("{}", id);
    let score = id;

    let _: () = try!(conn.zadd(DICT_LOG_KEY, value.clone(), score)
        .map_err(|err| server_error(err, "Redis操作出错")));
    let _: () = try!(conn.hset(DICT_LOG_DATA_KEY, value.clone(), log_entity_json)
        .map_err(|err| server_error(err, "Redis操作出错")));

    json(iron::status::Created, &entity)
}

pub fn list_logs(_: &mut Request) -> IronResult<Response> {
    info!("list_logs...");

    let conn = try!(conn_pool().get().map_err(|err| server_error(err, "Redis无法访问")));

    let res: Vec<String> = try!(conn.hvals(DICT_LOG_DATA_KEY)
        .map_err(|err| server_error(err, "Redis操作出错")));
    info!("res: {:?}", res);
    let res: Vec<DictLogEntity> = res.into_iter()
        .map(|it| serde_json::from_str(&it).unwrap())
        .collect();

    json_ok(&res)
}
