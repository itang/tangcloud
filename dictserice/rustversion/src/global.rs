use r2d2_redis::RedisConnectionManager;
use redis;
use r2d2;

lazy_static! {
    static ref POOL: r2d2::Pool<RedisConnectionManager> = make_pool();
}

pub fn conn_pool<'a>() -> &'a r2d2::Pool<RedisConnectionManager> {
    &(*POOL)
}

fn make_pool() -> r2d2::Pool<RedisConnectionManager> {
    let config = Default::default();
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::new(config, manager).unwrap();

    pool
}
