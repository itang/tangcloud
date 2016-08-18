package main

import (
	"encoding/json"
	"fmt"
	"time"

	"github.com/kataras/iris"
	"github.com/garyburd/redigo/redis"
	"github.com/itang/gotang"
)

type DictLog struct {
	From     string `json:"from"`

	FromLang string `json:"fromLang"`

	ToLang   string `json:"toLang"`
	To       string `json:"to"`
}

type DictLogEntity struct {
	Id int64  `json:"id"`
	DictLog
}

type Response struct {
	Status  int `json:"status"`
	Message string `json:"message"`
	Data    interface{} `json:"data"`
}

const (
	DICT_LOG_KEY = "tc:dict:log"
	DICT_LOG_DATA_KEY = "tc:dict:log:data"
)

func main() {
	c, err := redis.DialURL("redis://127.0.0.1")
	gotang.AssertNoError(err, "")
	defer c.Close()

	log := iris.Party("/dict/logs")
	{
		log.Post("", func(ctx *iris.Context) {
			dictLog := &DictLog{}
			if err := ctx.ReadJSON(dictLog); err != nil {
				ctx.JSON(500, Response{Status:500, Message:err.Error()})
			} else {
				id := time.Now().Unix()
				logEntity := DictLogEntity{Id: id, DictLog: *dictLog}

				v, err := json.Marshal(logEntity)
				gotang.AssertNoError(err, "")

				value := fmt.Sprintf("%v", id)
				score := id
				logEntityJson := string(v)

				_, err = multi(c, func(c redis.Conn) {
					c.Do("ZADD", DICT_LOG_KEY, value, score)
					c.Do("HSET", DICT_LOG_DATA_KEY, value, logEntityJson)
				})
				gotang.AssertNoError(err, "")

				ctx.JSON(200, Response{Status:200, Message:""})
			}
		})

		log.Get("", func(ctx *iris.Context) {
			reply, err := redis.Strings(c.Do("HVALS", DICT_LOG_DATA_KEY))
			gotang.AssertNoError(err, "")

			logs := make([]DictLog, len(reply))
			for _, v := range reply {
				log := DictLog{}
				err := json.Unmarshal([]byte(v), &log)
				gotang.AssertNoError(err, "")
				logs = append(logs, log)
			}

			ctx.JSON(200, Response{Status:200, Message:"", Data:logs})
		})
	}

	iris.Listen(":9800")
}

func multi(c redis.Conn, action func(c redis.Conn)) (ret interface{}, err error) {
	c.Send("MULTI")
	action(c)
	ret, err = c.Do("EXEC")
	return
}
