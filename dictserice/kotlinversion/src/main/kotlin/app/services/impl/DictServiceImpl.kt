package app.services.impl

import app.dao.DictRepository
import app.dto.DictHistory
import app.entity.DictHistoryEntity
import app.services.DictService
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.beans.factory.annotation.Qualifier
import org.springframework.stereotype.Service
import java.util.*


@Service
class DictServiceImpl : DictService {

    @Autowired
    @Qualifier("redis")
    private lateinit var dictRepository: DictRepository

    override fun recordDictHistory(dictHistory: DictHistory): Long {
        val now = Date()
        val ret = dictRepository.create(dtoToEntity(dictHistory).apply {
            id = now.time
            createdAt = now
        })

        return ret.id!!
    }

    override fun findDictHistories(): List<DictHistory> {
        return dictRepository.findDictHistories().map { entityToDto(it) }
    }
}
