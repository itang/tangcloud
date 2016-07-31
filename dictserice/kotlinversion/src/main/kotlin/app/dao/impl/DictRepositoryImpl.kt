package app.dao.impl

import app.dao.DictRepository
import app.entity.DictHistoryEntity
import com.fasterxml.jackson.databind.ObjectMapper
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.stereotype.Repository

import app.util.*
import org.springframework.beans.factory.annotation.Qualifier
import org.springframework.data.redis.core.StringRedisTemplate

@Repository
@Qualifier("redis")
open class DictRepositoryImpl : DictRepository {

    @Autowired
    private lateinit var redisTemplate: StringRedisTemplate

    @Autowired
    private lateinit var JSON: ObjectMapper

    private val DICT_HISTORY_KEY = "history"
    private val DICT_HISTORY_DATA_KEY = "history:data"


    override fun create(dictHistoryEntity: DictHistoryEntity): DictHistoryEntity {
        val v = dictHistoryEntity.id!!.toString()
        val score = dictHistoryEntity.createdAt!!.time.toDouble()
        val key = DICT_HISTORY_KEY

        redisTemplate.inTransaction {
            it.opsForZSet().add(key, v, score)
            it.opsForHash<String, String>().put(DICT_HISTORY_DATA_KEY, v, JSON.stringify(dictHistoryEntity))
        }

        return dictHistoryEntity
    }

    override fun findDictHistories(): List<DictHistoryEntity> {
        return redisTemplate.opsForHash<String, String>().values(DICT_HISTORY_DATA_KEY).map {
            JSON.parse(it, DictHistoryEntity::class.java)
        }
    }
}
