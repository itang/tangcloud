package app.services.impl

import app.dto.DictHistory
import app.entity.DictHistoryEntity


fun entityToDto(entity: DictHistoryEntity): DictHistory {
    return DictHistory(
            id = entity.id,
            fromLang = entity.fromLang,
            from = entity.from,
            toLang = entity.toLang,
            to = entity.to
    )
}

fun dtoToEntity(dto: DictHistory): DictHistoryEntity {
    return DictHistoryEntity(
            id = dto.id,
            fromLang = dto.fromLang,
            from = dto.from,
            toLang = dto.toLang,
            to = dto.to
    )
}

