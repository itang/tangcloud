package main

import (
	"os"

	"github.com/go-redis/redis"
	"github.com/labstack/echo"
	"go.uber.org/zap"

	"dictservice/handlers"
	local_middleware "dictservice/middleware"
	"dictservice/model"
	model_impl "dictservice/model/impl"
)

var (
	client *redis.Client = redis.NewClient(&redis.Options{
		Addr:     getRedisURL(),
		Password: "", // no password set
		DB:       0,  // use default DB
	})

	dictLogService    model.DictLogService = model_impl.NewDictLogService(client)
	dictLogController                      = handlers.NewDictLogController(dictLogService)
)

func getRedisURL() string {
	url := os.Getenv("REDIS_URL")
	if url != "" {
		return url
	} else {
		return "localhost:6379"
	}
}

func init() {
	pingRedis()
}

func main() {
	e := echo.New()

	api := e.Group("/api", local_middleware.XRuntime)
	api.GET("/ping", dictLogController.Ping)

	log := api.Group("/dict/logs")
	log.POST("", dictLogController.CreateLog)
	log.GET("", dictLogController.ListLogs)
	log.DELETE("/:id", dictLogController.DeleteLog)

	//e.Static("/", "../../../public")

	e.Logger.Fatal(e.Start(":8080")) //TODO: PORT from env config
}

func pingRedis() {
	logger, _ := zap.NewProduction()
	defer logger.Sync()

	logger.Info("redis client ping...")
	pingErr := client.Ping().Err()
	if pingErr != nil {
		logger.Error(pingErr.Error())
	}
}
