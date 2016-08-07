package app.dao

import app.dao.entity.DictLogEntity

interface DictLogRepository {

    fun create(dictLogEntity: DictLogEntity): DictLogEntity

    fun findAll(): List<DictLogEntity>

}
