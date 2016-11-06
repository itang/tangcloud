package app.controller

import org.springframework.stereotype.Controller
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.ResponseBody


@Controller
class WelcomeController {

    @GetMapping("/")
    @ResponseBody
    fun welcome(): String {
        return "Dict Service..."
    }
}
