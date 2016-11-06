package app.util

import org.slf4j.Logger
import org.slf4j.LoggerFactory

//@see: http://stackoverflow.com/questions/34416869/idiomatic-way-of-logging-in-kotlin
fun <T : Any> T.logger(): Logger = LoggerFactory.getLogger(this.javaClass)
