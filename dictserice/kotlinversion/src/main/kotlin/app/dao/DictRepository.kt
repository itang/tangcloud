package app.dao

import app.entity.DictHistoryEntity

interface DictRepository {

    fun create(dictHistoryEntity: DictHistoryEntity): DictHistoryEntity

    fun findDictHistories(): List<DictHistoryEntity>

}
