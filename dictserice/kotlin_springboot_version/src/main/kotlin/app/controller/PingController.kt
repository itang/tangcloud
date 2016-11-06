package app.controller

import app.service.dto.DictLog
import app.controller.dto.Ok
import app.controller.dto.Error
import app.controller.dto.Response
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.web.bind.annotation.*
import org.springframework.stereotype.Controller

@Controller
@RequestMapping("/api/ping")
class PingController {

    @GetMapping("")
    @ResponseBody
    fun welcome(): Map<String, String> {
        return mapOf("message" to "pong")
    }
}
