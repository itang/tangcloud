package app.dao.impl

import app.dao.DictLogRepository
import app.dao.entity.DictLogEntity
import app.util.inTransaction
import app.util.parse
import app.util.stringify
import com.fasterxml.jackson.databind.ObjectMapper
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.beans.factory.annotation.Qualifier
import org.springframework.data.redis.core.StringRedisTemplate
import org.springframework.stereotype.Repository


@Repository
@Qualifier("redis")
open class DictLogRepositoryImpl(
        @Autowired private val redisTemplate: StringRedisTemplate,
        @Autowired private val JSON: ObjectMapper
) : DictLogRepository {

    private val DICT_LOG_KEY = "tc:dict:log"
    private val DICT_LOG_DATA_KEY = "tc:dict:log:data"

    override fun create(dictLogEntity: DictLogEntity): DictLogEntity {
        val v = dictLogEntity.id!!.toString()
        val score = dictLogEntity.createdAt!!.time.toDouble()
        val key = DICT_LOG_KEY

        redisTemplate.inTransaction {
            it.opsForZSet().add(key, v, score)
            it.opsForHash<String, String>().put(DICT_LOG_DATA_KEY, v, JSON.stringify(dictLogEntity))
        }

        return dictLogEntity
    }

    override fun findAll(): List<DictLogEntity> {
        return redisTemplate.opsForHash<String, String>().values(DICT_LOG_DATA_KEY).map {
            JSON.parse(it, DictLogEntity::class.java)
        }
    }
}
