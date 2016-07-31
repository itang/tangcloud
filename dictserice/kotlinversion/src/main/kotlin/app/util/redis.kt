package app.util

import org.springframework.data.redis.core.RedisTemplate


fun RedisTemplate<String, String>.inTransaction(action: (RedisTemplate<String, String>) -> Unit): List<Any> {
    //TODO: Redis事务处理
    //this.multi()

    action(this)

    //val ret = this.exec()

    //return ret
    return listOf()
}
