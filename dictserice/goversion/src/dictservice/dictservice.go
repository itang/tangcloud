package main

import (
	"encoding/json"
	"fmt"
	"time"
	
	"github.com/garyburd/redigo/redis"
	"github.com/itang/gotang"
	"github.com/kataras/iris"
)

type DictLog struct {
	From     string `json:"from"`
	
	FromLang string `json:"fromLang"`
	
	ToLang   string `json:"toLang"`
	To       string `json:"to"`
}

type DictLogEntity struct {
	Id int64 `json:"id"`
	DictLog
}

type Response struct {
	Status  int         `json:"status"`
	Message string      `json:"message"`
	Data    interface{} `json:"data"`
}

const (
	DICT_LOG_KEY = "tc:dict:log"
	DICT_LOG_DATA_KEY = "tc:dict:log:data"
)

var (
	pool *redis.Pool
)

func newPool(server, password string) *redis.Pool {
	return &redis.Pool{
		MaxIdle: 3,
		IdleTimeout: 240 * time.Second,
		Dial: func() (redis.Conn, error) {
			c, err := redis.Dial("tcp", server)
			if err != nil {
				return nil, err
			}
			if password != "" {
				if _, err := c.Do("AUTH", password); err != nil {
					c.Close()
					return nil, err
				}
			}
			
			return c, err
		},
		TestOnBorrow: func(c redis.Conn, t time.Time) error {
			_, err := c.Do("PING")
			return err
		},
	}
}

func main() {
	pool = newPool(":6379", "")
	
	log := iris.Party("/dict/logs")
	{
		log.Post("", func(ctx *iris.Context) {
			dictLog := &DictLog{}
			if err := ctx.ReadJSON(dictLog); err != nil {
				ctx.JSON(500, Response{Status: 500, Message: err.Error()})
			} else {
				conn := pool.Get()
				defer conn.Close()
				
				id := time.Now().Unix()
				logEntity := DictLogEntity{Id: id, DictLog: *dictLog}
				
				v, err := json.Marshal(logEntity)
				gotang.AssertNoError(err, "err json encode")
				
				value := fmt.Sprintf("%v", id)
				score := id
				logEntityJson := string(v)
				
				_, err = multi(conn, func(c redis.Conn) {
					c.Do("ZADD", DICT_LOG_KEY, value, score)
					c.Do("HSET", DICT_LOG_DATA_KEY, value, logEntityJson)
				})
				gotang.AssertNoError(err, "err redis multi do")
				
				ctx.JSON(200, Response{Status: 200, Message: ""})
			}
		})
		
		log.Get("", func(ctx *iris.Context) {
			conn := pool.Get()
			//defer conn.Close()
			
			reply, err := redis.Strings(conn.Do("HVALS", DICT_LOG_DATA_KEY))
			if err != nil {
				fmt.Printf("error: %v", err)
				ctx.Error(err.Error(), 500)
				return
			}
			
			logs := make([]DictLog, len(reply))
			for _, v := range reply {
				log := DictLog{}
				err := json.Unmarshal([]byte(v), &log)
				gotang.AssertNoError(err, "json decode")
				logs = append(logs, log)
			}
			
			conn.Close()
			
			ctx.JSON(200, Response{Status: 200, Message: "", Data: logs})
		})
	}
	
	iris.Listen(":9800")
}

func multi(c redis.Conn, action func(c redis.Conn)) (ret interface{}, err error) {
	c.Send("MULTI")
	action(c)
	
	return c.Do("EXEC")
}
