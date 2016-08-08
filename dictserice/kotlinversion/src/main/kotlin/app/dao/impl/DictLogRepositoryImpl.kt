package app.dao.impl

import app.dao.DictLogRepository
import app.dao.entity.DictLogEntity
import app.util.inTransaction
import app.util.opsForStringHash
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
        val value = dictLogEntity.id!!.toString()
        val score = dictLogEntity.createdAt!!.time.toDouble()
        val key = DICT_LOG_KEY

        redisTemplate.inTransaction {
            it.opsForZSet().add(key, value, score)
            it.opsForStringHash.put(DICT_LOG_DATA_KEY, value, JSON.stringify(dictLogEntity))
        }

        return dictLogEntity
    }

    override fun findAll(): List<DictLogEntity> {
        return redisTemplate.opsForStringHash.values(DICT_LOG_DATA_KEY).map {
            JSON.parse(it, DictLogEntity::class.java)
        }
    }
}
