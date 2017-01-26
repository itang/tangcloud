package main

import (
	"dictservice/handlers"
	"github.com/labstack/echo"
)

func main() {
	e := echo.New()

	api := e.Group("/api")
	api.GET("/ping", handlers.Ping)

	log := api.Group("/dict/logs")
	log.POST("", handlers.CreateLog)
	log.GET("", handlers.ListLogs)

	//e.Static("/", "../../../public")

	e.Logger.Fatal(e.Start(":8080"))
}
