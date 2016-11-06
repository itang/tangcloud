package app.service.impl

import app.service.dto.DictLog
import app.dao.entity.DictLogEntity


internal fun DictLogEntity.toDto(): DictLog {
    return DictLog(
            id = this.id,
            fromLang = this.fromLang,
            from = this.from,
            toLang = this.toLang,
            to = this.to
    )
}

internal fun DictLog.toEntity(): DictLogEntity {
    return DictLogEntity(
            id = this.id,
            fromLang = this.fromLang,
            from = this.from,
            toLang = this.toLang,
            to = this.to
    )
}
