package app

import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.databind.DeserializationFeature
import io.vertx.core.AbstractVerticle
import io.vertx.core.http.HttpServer
import io.vertx.ext.web.Router
import com.fasterxml.jackson.module.kotlin.*
import io.vertx.core.Handler
import io.vertx.ext.web.RoutingContext
import io.vertx.ext.web.handler.BodyHandler
import io.vertx.ext.web.handler.LoggerHandler
import io.vertx.ext.web.handler.ResponseTimeHandler
import io.vertx.ext.web.handler.TimeoutHandler
import io.vertx.redis.RedisClient
import io.vertx.redis.RedisOptions

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

enum class Status {
    Success, Failure
}

data class Response<out T>(val status: Status, val message: String? = null, val data: T? = null)

class ResponseJSONHandler private constructor() : Handler<RoutingContext> {
    override fun handle(ctx: RoutingContext) {
        ctx.response().putHeader("Content-Type", "application/json; charset=utf-8")
        ctx.next()
    }

    companion object {
        val instance = ResponseJSONHandler()
    }
}

class DictServcieVerticle : AbstractVerticle() {

    private lateinit var httpServer: HttpServer

    private val mapper = jacksonObjectMapper().configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false)

    var redisConfig = RedisOptions().setHost("127.0.0.1")

    private val DICT_LOG_KEY = "tc:dict:log"
    private val DICT_LOG_DATA_KEY = "tc:dict:log:data"

    override fun start() {
        httpServer = vertx.createHttpServer()
        val redis = RedisClient.create(vertx, redisConfig)

        val mainRouter = Router.router(vertx).apply {
            route().handler(LoggerHandler.create())

            get("/").handler { ctx ->
                ctx.renderJSON("Dict Service")
            }
        }

        val apiRouter = Router.router(vertx).apply {
            route().handler(ResponseTimeHandler.create())
            post().handler(BodyHandler.create())
            route().handler(TimeoutHandler.create(8000))
            route().handler(ResponseJSONHandler.instance)

            route().failureHandler { ctx ->
                ctx.failure()?.let {
                    println(it.message)
                }

                ctx.renderJSON(Response<Unit>(Status.Failure, ctx.failure()?.message))
            }

            route("/exception").handler { ctx ->
                throw RuntimeException("test exception")
            }

            route("/ping").handler { ctx ->
                ctx.renderJSON(mapOf("message" to "pong"))
            }

            post("/dict/logs").handler { ctx ->
                val req = ctx.bindJSON<DictLogReq>()
                val id = System.currentTimeMillis()
                val score = id.toDouble()
                val member = id.toString()
                redis.transaction().apply {
                    zadd(DICT_LOG_KEY, score, member) { res ->
                        if (res.failed()) {
                            res.cause()?.printStackTrace()
                            ctx.renderJSON(Response<Unit>(Status.Failure))
                        }
                    }

                    val dictLogEntity = DictLogEntity(id, from = req.from, to = req.to)
                    hset(DICT_LOG_DATA_KEY, member, mapper.writeValueAsString(dictLogEntity)) { res ->
                        if (res.failed())
                            res.cause()?.printStackTrace()
                        ctx.renderJSON(Response<Unit>(Status.Failure))
                    }
                }

                ctx.renderJSON(Response<Unit>(Status.Success))
            }

            get("/dict/logs").handler { ctx ->
                redis.hvals(DICT_LOG_DATA_KEY) { res ->
                    if (res.succeeded()) {
                        ctx.renderJSON(Response(Status.Success, data = res.result().map { mapper.readValue<DictLogEntity>(it.toString()) }))
                    } else {
                        res.cause()?.printStackTrace()
                        ctx.renderJSON(Response<Unit>(Status.Failure))
                    }
                }
            }

        }

        mainRouter.mountSubRouter("/api", apiRouter)

        httpServer.requestHandler { req -> mainRouter.accept(req) }.listen(8080)
    }

    override fun stop() {
        httpServer.close()
    }

    private inline fun <reified T : Any> RoutingContext.bindJSON(): T {
        return jacksonObjectMapper().readValue<T>(this.bodyAsString)
    }

    private fun RoutingContext.renderJSON(obj: Any) {
        this.response().end(mapper.writeValueAsString(obj))
    }

}

