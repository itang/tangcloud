use r2d2;
use r2d2_redis::RedisConnectionManager;

const REDIS_URL: &'static str = "redis://127.0.0.1/";

lazy_static! {
    static ref POOL: r2d2::Pool<RedisConnectionManager> = make_pool();
}

pub fn conn_pool<'a>() -> &'a r2d2::Pool<RedisConnectionManager> {
    &(*POOL)
}

fn make_pool() -> r2d2::Pool<RedisConnectionManager> {
    let config = Default::default();
    let manager = RedisConnectionManager::new(REDIS_URL).unwrap();
    let pool = r2d2::Pool::new(config, manager).unwrap();

    pool
}
