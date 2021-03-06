package app.config

import com.fasterxml.jackson.module.kotlin.KotlinModule
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.http.converter.json.Jackson2ObjectMapperBuilder


@Configuration
open class AppConfig {

    @Bean
    open fun objectMapperBuilder(): Jackson2ObjectMapperBuilder {
        return Jackson2ObjectMapperBuilder().modulesToInstall(KotlinModule())
    }

}
