package main

import (
	"github.com/labstack/echo"
	"github.com/uber-go/zap"
	"gopkg.in/redis.v5"

	"dictservice/handlers"
	local_middleware "dictservice/middleware"
	"dictservice/model"
	model_impl "dictservice/model/impl"
)

var (
	client *redis.Client = redis.NewClient(&redis.Options{
		Addr:     "localhost:6379",
		Password: "", // no password set
		DB:       0,  // use default DB
	})
	logger                                 = zap.New(zap.NewJSONEncoder( /*zap.NoTime()*/)) // drop timestamps in tests
	dictLogService    model.DictLogService = model_impl.NewDefaultDictLogServiceImpl(client, logger)
	dictLogController                      = handlers.NewDictLogController(dictLogService, logger)
)

func init() {
	logger.Info("redis client ping...")
	pingErr := client.Ping().Err()
	if pingErr != nil {
		logger.Error(pingErr.Error())
	}
}

func main() {
	e := echo.New()

	api := e.Group("/api", local_middleware.XRuntime)
	api.GET("/ping", dictLogController.Ping)

	log := api.Group("/dict/logs")
	log.POST("", dictLogController.CreateLog)
	log.GET("", dictLogController.ListLogs)

	//e.Static("/", "../../../public")

	e.Logger.Fatal(e.Start(":8080"))
}
