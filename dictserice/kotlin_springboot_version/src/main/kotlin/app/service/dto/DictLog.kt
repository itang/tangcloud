package app.service.dto

import java.io.Serializable


data class DictLog(
        var id: Long?,
        var fromLang: String?,
        var from: String?,
        var toLang: String?,
        var to: String?
) : Serializable
