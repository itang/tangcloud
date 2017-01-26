package handlers

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/itang/gotang"
	"github.com/labstack/echo"
	"github.com/uber-go/zap"
	"gopkg.in/redis.v5"

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

func init() {
	log.Println("handlers package init...")
	pingErr := client.Ping().Err()
	if pingErr != nil {
		logger.Warn(client.Ping().Err().Error())
	}
}

func test() string {
	fmt.Println("var init test")
	return "test"
}

func Ping(ctx echo.Context) error {
	return ctx.JSON(http.StatusOK, map[string]string{"message": "pong"})
}

func CreateLog(ctx echo.Context) error {
	logger.Info("Post to /dict/logs")
	dictLog := &types.DictLog{}
	if err := ctx.Bind(dictLog); err != nil {
		return ctx.JSON(http.StatusInternalServerError, types.Response{Status: 500, Message: err.Error()})
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
			return ctx.JSON(http.StatusInternalServerError, types.Response{Status: 500, Message: err.Error()})
		} else {
			return ctx.JSON(http.StatusOK, types.Response{Status: 200, Message: ""})
		}
	}
}

func ListLogs(ctx echo.Context) error {
	logger.Info("Get /dict/logs")

	reply, err := client.HVals(DICT_LOG_DATA_KEY).Result()
	if err != nil {
		fmt.Printf("error: %v", err)
		return ctx.JSON(http.StatusInternalServerError, types.Response{Status: 500, Message: err.Error()})
	} else {
		logs := make([]types.DictLog, len(reply))
		for i, v := range reply {
			log := types.DictLog{}
			err := json.Unmarshal([]byte(v), &log)
			gotang.AssertNoError(err, "json decode")
			logs[i] = log
		}

		return ctx.JSON(http.StatusOK, types.Response{Status: 200, Message: "", Data: logs})
	}
}
