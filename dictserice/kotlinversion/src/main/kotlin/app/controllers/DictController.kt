package app.controllers

import app.dto.DictHistory
import app.dto.Ok
import app.dto.Error
import app.dto.Response
import app.services.DictService
import app.util.logger
import org.slf4j.Logger
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.web.bind.annotation.*

@RestController
@RequestMapping("/dict")
class DictController {

    val LOG: Logger = logger()

    @Autowired
    private lateinit var dictService: DictService

    @PostMapping("/history")
    fun recordDictHistory(@RequestBody dictHistory: DictHistory): Response<Long> {
        try {
            val newId: Long = dictService.recordDictHistory(dictHistory)
            return Ok(data = newId)
        } catch(e: Exception) {
            LOG.warn("Record dict history error.", e)
            return Error(e.message ?: "")
        }
    }

    @GetMapping("/history")
    fun dictHistory(): List<DictHistory> {
        return dictService.findDictHistories()
    }
}
