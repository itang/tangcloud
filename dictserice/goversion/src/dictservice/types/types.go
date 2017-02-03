package types

import "fmt"

type Id string

type DictLog struct {
	From string `json:"from"`

	FromLang string `json:"fromLang"`

	ToLang string `json:"toLang"`
	To     string `json:"to"`
}

type DictLogEntity struct {
	Id Id `json:"id"`
	DictLog
}

type Response struct {
	Status  int         `json:"status"`
	Message string      `json:"message"`
	Data    interface{} `json:"data"`
}

type LogNoExistsError struct {
	Id Id
}

func (c LogNoExistsError) Error() string {
	return fmt.Sprintf("ID为%s的日志不存在", c.Id)
}
