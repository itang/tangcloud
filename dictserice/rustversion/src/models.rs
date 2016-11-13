use serde_json;
use redis::Commands;

use global::*;
use utils::*;
use types::*;


pub trait LogService: Send + Sync {
    fn create(&self, log: &DictLogEntity) -> Result<(), ServerError>;
    fn find_all(&self) -> Result<Vec<DictLogEntity>, ServerError>;
}


pub struct LogServiceImpl {}


const DICT_LOG_KEY: &'static str = "tc:dict:log";
const DICT_LOG_DATA_KEY: &'static str = "tc:dict:log:data";

impl LogService for LogServiceImpl {
    fn create(&self, entity: &DictLogEntity) -> Result<(), ServerError> {
        let log_entity_json =
            serde_json::to_string(entity).map_err(|err| ServerError(err.to_string()))?;

        info!("log_entity_json: {:?}", log_entity_json);

        let conn = conn_pool().get()
            .map_err(|err| ServerError(err.to_string()))?;

        let id = timestamp() as i64;
        let value = format!("{}", id);
        let score = id;

        let _: () = conn.zadd(DICT_LOG_KEY, &value, score)
            .map_err(|err| ServerError(err.to_string()))?;

        let _: () = conn.hset(DICT_LOG_DATA_KEY, &value, log_entity_json)
            .map_err(|err| ServerError(err.to_string()))?;

        Ok(())
    }

    fn find_all(&self) -> Result<Vec<DictLogEntity>, ServerError> {
        let conn = conn_pool().get()
            .map_err(|err| ServerError(err.to_string()))?;

        let res: Vec<String> = conn.hvals(DICT_LOG_DATA_KEY)
            .map_err(|err| ServerError(err.to_string()))?;
        info!("res: {:?}", res);
        let res: Vec<DictLogEntity> = res.into_iter()
            .map(|it| serde_json::from_str(&it).unwrap())
            .collect();

        Ok(res)
    }
}
