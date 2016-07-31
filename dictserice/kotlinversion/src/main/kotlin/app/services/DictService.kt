package app.services

import app.dto.DictHistory


interface DictService {

    fun recordDictHistory(dictHistory: DictHistory): Long

    fun findDictHistories(): List<DictHistory>
}
