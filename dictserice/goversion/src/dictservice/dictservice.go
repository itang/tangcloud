package main

import (
	"fmt"

	"dictservice/handlers"
	"github.com/kataras/iris"
)

func init() {
	fmt.Printf("main package init...")
}

func main() {
	api := iris.Party("/api")
	api.Any("/ping", handlers.Ping)

	log := api.Party("/dict/logs")
	log.Post("", handlers.CreateLog)
	log.Get("", handlers.ListLogs)

	iris.StaticWeb("/", "../../../public", 0)

	iris.Listen(":8080")
}
