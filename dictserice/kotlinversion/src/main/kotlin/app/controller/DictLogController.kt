package app.controller

import app.service.dto.DictLog
import app.controller.dto.Ok
import app.controller.dto.Error
import app.controller.dto.Response
import app.service.DictLogService
import app.util.logger
import org.slf4j.Logger
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.web.bind.annotation.*


@RestController
@RequestMapping("/dict/logs")
class DictLogController(
        @Autowired private val dictLogService: DictLogService
) {

    val LOG: Logger = logger()

    @PostMapping
    fun post(@RequestBody dictLog: DictLog): Response<Long> {
        try {
            val newId: Long = dictLogService.create(dictLog)
            return Ok(data = newId)
        } catch(e: Exception) {
            LOG.warn("create dict log error. dictLog: $dictLog", e)
            return Error(e.message ?: "")
        }
    }

    @GetMapping
    fun list(): List<DictLog> {
        return dictLogService.findAll()
    }
}
