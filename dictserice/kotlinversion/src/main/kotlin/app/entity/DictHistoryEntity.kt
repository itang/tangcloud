package app.entity

import java.util.*


data class DictHistoryEntity(
        var id: Long? = null,
        var fromLang: String? = null,
        var from: String? = null,
        var toLang: String? = null,
        var to: String? = null,
        var createdAt: Date? = null)
