package app.util

import com.fasterxml.jackson.databind.ObjectMapper

fun ObjectMapper.stringify(obj: Any): String {
    return this.writeValueAsString(obj)
}

fun <T> ObjectMapper.parse(json: String, clazz: Class<T>): T {
    return this.readValue(json, clazz)
}
