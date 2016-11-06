package app.dao.entity

import java.util.*


data class DictLogEntity(
        var id: Long? = null,
        var fromLang: String? = null,
        var from: String? = null,
        var toLang: String? = null,
        var to: String? = null,
        var createdAt: Date? = null)
