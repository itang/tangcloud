package main

import (
	"encoding/json"
	"fmt"
	"time"

	"github.com/itang/gotang"
	"github.com/kataras/iris"
	"github.com/uber-go/zap"

	"dictservice/types"
)

const (
	DICT_LOG_KEY      = "tc:dict:log"
	DICT_LOG_DATA_KEY = "tc:dict:log:data"
)

var (
	client *redis.Client = redis.NewClient(&redis.Options{
		Addr:     "localhost:6379",
		Password: "", // no password set
		DB:       0,  // use default DB
	})

	logger = zap.New(zap.NewJSONEncoder( /*zap.NoTime()*/ )) // drop timestamps in tests

	_ = test()
)

func test() string {
	fmt.Println("var init test")
	return "test"
}

func init() {
	fmt.Printf("package init...")
}

func main() {
	fmt.Println("entry main...")

	pingErr := client.Ping().Err()
	if pingErr != nil {
		logger.Warn(client.Ping().Err().Error())
	}

	iris.Any("/ping", func(ctx *iris.Context) {
		ctx.Text(200, "pong")
	})

	log := iris.Party("/dict/logs")
	{
		log.Post("", func(ctx *iris.Context) {
			logger.Info("Post to /dict/logs")
			dictLog := &types.DictLog{}
			if err := ctx.ReadJSON(dictLog); err != nil {
				ctx.JSON(500, types.Response{Status: 500, Message: err.Error()})
			} else {
				id := time.Now().Unix()
				logEntity := types.DictLogEntity{Id: id, DictLog: *dictLog}

				v, err := json.Marshal(logEntity)
				gotang.AssertNoError(err, "err json encode")

				value := fmt.Sprintf("%v", id)
				score := id
				logEntityJson := string(v)

				client.ZAdd(DICT_LOG_KEY, redis.Z{Member: value, Score: float64(score)})
				client.HSet(DICT_LOG_DATA_KEY, value, logEntityJson)

				if err != nil {
					ctx.JSON(500, types.Response{Status: 500, Message: err.Error()})
				} else {
					ctx.JSON(200, types.Response{Status: 200, Message: ""})
				}
			}
		})

		log.Get("", func(ctx *iris.Context) {
			logger.Info("Get /dict/logs")

			reply := client.HVals(DICT_LOG_DATA_KEY)
			if reply.Err() != nil {
				fmt.Printf("error: %v", reply.Err())
				ctx.Error(reply.Err().Error(), 500)
			} else {
				logs := make([]types.DictLog, len(reply.Val()))
				for _, v := range reply.Val() {
					log := types.DictLog{}
					err := json.Unmarshal([]byte(v), &log)
					gotang.AssertNoError(err, "json decode")
					logs = append(logs, log)
				}

				ctx.JSON(200, types.Response{Status: 200, Message: "", Data: logs})
			}
		})
	}

	iris.Listen(":8080")
}
