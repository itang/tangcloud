package types


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
