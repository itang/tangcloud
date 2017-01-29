package main

import (
	"github.com/labstack/echo"

	"dictservice/handlers"
	local_middleware "dictservice/middleware"
)

func main() {
	e := echo.New()

	api := e.Group("/api", local_middleware.XRuntime)
	api.GET("/ping", handlers.Ping)

	log := api.Group("/dict/logs")
	log.POST("", handlers.CreateLog)
	log.GET("", handlers.ListLogs)

	//e.Static("/", "../../../public")

	e.Logger.Fatal(e.Start(":8080"))
}
