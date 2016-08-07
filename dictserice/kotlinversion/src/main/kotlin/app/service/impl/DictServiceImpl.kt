package app.service.impl

import app.dao.DictLogRepository
import app.service.dto.DictLog

import app.service.DictLogService
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.beans.factory.annotation.Qualifier
import org.springframework.stereotype.Service
import java.util.*


@Service
class DictServiceImpl(
        @Autowired @Qualifier("redis") private val dictLogRepository: DictLogRepository
) : DictLogService {

    override fun create(dictLog: DictLog): Long {
        val now = Date()
        val ret = dictLogRepository.create(dictLog.toEntity().apply {
            id = now.time
            createdAt = now
        })

        return ret.id!!
    }

    override fun findAll(): List<DictLog> {
        return dictLogRepository.findAll().map { it.toDto() }
    }
}
