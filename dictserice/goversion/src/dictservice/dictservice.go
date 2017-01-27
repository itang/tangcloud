package main

import (
	"time"

	"dictservice/handlers"
	"github.com/labstack/echo"
)

func XRuntimeMiddleware(next echo.HandlerFunc) echo.HandlerFunc {
	return func(ctx echo.Context) (err error) {
		start := time.Now()

		//link: https://golang.org/pkg/net/http/#example_ResponseWriter_trailers
		ctx.Response().Header().Set("Trailer", "x-runtime")

		err = next(ctx)

		//FIXME: 未生效
		ctx.Response().Header().Set("x-runtime", time.Since(start).String())
		return
	}
}

func main() {
	e := echo.New()

	api := e.Group("/api", XRuntimeMiddleware)
	api.GET("/ping", handlers.Ping)

	log := api.Group("/dict/logs")
	log.POST("", handlers.CreateLog)
	log.GET("", handlers.ListLogs)

	//e.Static("/", "../../../public")

	e.Logger.Fatal(e.Start(":8080"))
}
