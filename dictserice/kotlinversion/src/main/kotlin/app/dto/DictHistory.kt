package app.dto

import java.io.Serializable


data class DictHistory(
        var id: Long?,
        var fromLang: String?,
        var from: String?,
        var toLang: String?,
        var to: String?
) : Serializable
