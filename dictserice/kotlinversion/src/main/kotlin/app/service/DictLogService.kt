package app.service

import app.service.dto.DictLog


interface DictLogService {

    fun create(dictLog: DictLog): Long

    fun findAll(): List<DictLog>
}
