package app.dto

import com.fasterxml.jackson.annotation.JsonFormat


@JsonFormat(shape = JsonFormat.Shape.OBJECT)
enum class Status(val code: Int) {
    ok(200), error(400)
}

class Response<out T>(val status: Status, val message: String?, val data: T?) {
}

fun <T> Ok(message: String? = null, data: T? = null): Response<T> {
    return Response(Status.ok, message, data)
}

fun <T> Error(message: String? = null, data: T? = null): Response<T> {
    return Response(Status.error, message, data)
}
