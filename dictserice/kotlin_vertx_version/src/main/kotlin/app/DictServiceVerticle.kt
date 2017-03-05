package app

import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.databind.DeserializationFeature
import io.vertx.core.http.HttpServer
import io.vertx.ext.web.Router
import com.fasterxml.jackson.module.kotlin.*
import io.vertx.core.*
import io.vertx.core.http.ServerWebSocket
import io.vertx.ext.web.RoutingContext
import io.vertx.ext.web.handler.BodyHandler
import io.vertx.ext.web.handler.LoggerHandler
import io.vertx.ext.web.handler.ResponseTimeHandler
import io.vertx.ext.web.handler.TimeoutHandler
import io.vertx.redis.RedisClient
import io.vertx.redis.RedisOptions
import java.time.Duration
import java.util.*

data class DictLogReq(
        val from: String,
        val fromLang: String = "en",
        val to: String,
        val toLang: String = "zh"
)

@JsonIgnoreProperties(ignoreUnknown = true)
data class DictLogEntity(
        val id: Long,
        val from: String,
        val fromLang: String? = "en",
        val to: String,
        val toLang: String? = "zh",
        val createdAt: String? = null
)


sealed class Result(val ok: Boolean, open val message: String? = null)
data class Ok<out T>(override val message: String? = null, val data: T? = null) : Result(true, message)
data class Err<out E>(val status: Int = -1, override val message: String? = null, val data: E? = null) : Result(false, message)

class ResponseJSONHandler private constructor() : Handler<RoutingContext> {
    override fun handle(ctx: RoutingContext) {
        ctx.response().putHeader("Content-Type", "application/json; charset=utf-8")
        ctx.next()
    }

    companion object {
        val instance = ResponseJSONHandler()
    }
}

object GlobalConfig {
    val PORT: Int = 8080
    val TIMEOUT: Duration = Duration.ofSeconds(8)
}

@Suppress("unused")
class DictServiceVerticle : AbstractVerticle() {

    private lateinit var httpServer: HttpServer

    private val mapper = jacksonObjectMapper().configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false)
    private val redisConfig = RedisOptions().setHost("127.0.0.1")

    private val DICT_LOG_KEY = "tc:dict:log"
    private val DICT_LOG_DATA_KEY = "tc:dict:log:data"

    private val wsHolder = mutableListOf<ServerWebSocket>()

    override fun start() {
        httpServer = vertx.createHttpServer()
        val redis = RedisClient.create(vertx, redisConfig)

        val mainRouter = Router.router(vertx).apply {
            route().handler(LoggerHandler.create())

            get("/").handler { ctx ->
                ctx.renderJSON(Ok<Unit>(message = "Dict Service"))
            }
        }

        val apiRouter = Router.router(vertx).apply {
            route().handler(ResponseTimeHandler.create())
            post().handler(BodyHandler.create())
            route().handler(TimeoutHandler.create(GlobalConfig.TIMEOUT.toMillis()))
            route().handler(ResponseJSONHandler.instance)

            route().failureHandler { ctx ->
                ctx.failure()?.let {
                    println(it.message)
                }

                ctx.renderJSON(Err<Unit>(message = ctx.failure()?.message))
            }

            route("/ping").handler { ctx ->
                ctx.renderJSON(Ok(data = mapOf("message" to "pong")))
            }

            route("/exception").handler { _ ->
                throw RuntimeException("test exception")
            }

            // 演示WebSocket
            route("/ws").handler { ctx ->
                val ws = ctx.request().upgrade()
                println("ws $ws connected!")
                wsHolder.add(ws) //　hold connection

                wsHolder.forEach {
                    it.writeFinalTextFrame(mapper.writeValueAsString("${ws.remoteAddress()} connected"))
                }

                ws.writeFinalTextFrame(mapper.writeValueAsString(mapOf("message" to "Hello")))


                ws.handler { buf ->
                    val content = buf.toString("utf-8")
                    println("Received: $content")
                    ws.writeFinalTextFrame(content + ", date: ${Date()}")
                }.closeHandler {
                    println("ws $ws closed")
                    wsHolder.remove(ws)
                    wsHolder.forEach {
                        it.writeFinalTextFrame("${ws.remoteAddress()} disconnected")
                    }
                }
            }

            post("/dict/logs").handler { ctx ->
                val req = ctx.bindJSON<DictLogReq>()
                val id = System.currentTimeMillis()
                val score = id.toDouble()
                val member = id.toString()

                val fut1 = Future.future<String>()
                val fut2 = Future.future<String>()
                redis.transaction().apply {
                    zadd(DICT_LOG_KEY, score, member, fut1.completer())

                    val dictLogEntity = DictLogEntity(id, from = req.from, to = req.to)
                    hset(DICT_LOG_DATA_KEY, member, mapper.writeValueAsString(dictLogEntity), fut2.completer())
                }

                await(listOf(fut1, fut2)) { ar ->
                    if (ar.succeeded()) {
                        ctx.renderJSON(Ok<Unit>())
                    } else {
                        ar.cause()?.printStackTrace()
                        ctx.renderJSON(Err<Unit>(message = ar.cause()?.message))
                    }
                }
            }

            get("/dict/logs").handler { ctx ->
                redis.hvals(DICT_LOG_DATA_KEY) { res ->
                    if (res.succeeded()) {
                        ctx.renderJSON(Ok(data = res.result().map { mapper.readValue<DictLogEntity>(it.toString()) }))
                    } else {
                        res.cause()?.printStackTrace()
                        ctx.renderJSON(Err<Unit>())
                    }
                }
            }

        }

        mainRouter.mountSubRouter("/api", apiRouter)

        httpServer.requestHandler { req -> mainRouter.accept(req) }.listen(GlobalConfig.PORT)
    }

    override fun stop() {
        httpServer.close()
    }

    private fun await(list: List<Future<*>>, handler: (AsyncResult<CompositeFuture>) -> Unit) {
        CompositeFuture.all(list).setHandler(handler)
    }

    private inline fun <reified T : Any> RoutingContext.bindJSON(): T {
        return mapper.readValue(this.bodyAsString)
    }

    private fun RoutingContext.renderJSON(obj: Result) {
        response().end(mapper.writeValueAsString(obj))
    }

}
