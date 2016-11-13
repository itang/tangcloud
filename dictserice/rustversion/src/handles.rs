use iron;
use iron::prelude::*;
use bodyparser;

use types::*;
use utils::*;
use models::*;

pub struct LogController<'a> {
    pub log_service: &'a LogService,
}

pub fn ping(_: &mut Request) -> IronResult<Response> {
    json_ok(&ROk { data: "pong" })
    // return Ok(Response::with((iron::status::Ok, "pong")))
}

impl<'a> LogController<'a> {
    pub fn create_logs(&self, req: &mut Request) -> IronResult<Response> {
        info!("create_logs...");
        let log = req.get::<bodyparser::Struct<DictLog>>()
            .map_err(|err| badrequest_error(err, "解析json出错"))?
            .ok_or(IronError::new(EmptyError("空json".to_string()),
                                  (iron::status::BadRequest, "解析json出错")))?;

        if log.from == "" {
            return json(iron::status::BadRequest,
                        &RError::<()> {
                            errors: vec![RErrorItem::builder()
                                             .title("'from' can't by empty".into())
                                             .source("log.from".into())
                                             .code("123".into())
                                             .build()],
                        });
        }

        let id = timestamp() as i64;
        let entity = DictLogEntity {
            id: id,
            from: log.from,
            from_lang: log.from_lang,
            to: log.to,
            to_lang: log.to_lang,
        };

        let _: () = self.log_service
            .create(&entity)
            .map_err(|err| server_error(err, "log service create error"))?;

        json(iron::status::Created, &ROk { data: entity })
    }

    pub fn list_logs(&self, _: &mut Request) -> IronResult<Response> {
        info!("list_logs...");
        let res: Vec<DictLogEntity> = self.log_service
            .find_all()
            .map_err(|err| server_error(err, "log service find_all error"))?;
        json_ok(&ROk { data: res })
    }
}
