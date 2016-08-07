package app.dao.impl

import app.dao.DictLogRepository
import app.dao.entity.DictLogEntity
import com.fasterxml.jackson.databind.ObjectMapper
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Repository

import app.util.*
import org.springframework.beans.factory.annotation.Qualifier
import org.springframework.data.redis.core.StringRedisTemplate


@Repository
@Qualifier("redis")
open class DictLogRepositoryImpl(
        @Autowired private val redisTemplate: StringRedisTemplate,
        @Autowired private val JSON: ObjectMapper
) : DictLogRepository {

    private val DICT_LOG_KEY = "log"
    private val DICT_LOG_DATA_KEY = "log:data"

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
