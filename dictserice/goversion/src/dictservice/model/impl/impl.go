package impl

import (
	"encoding/json"
	"fmt"
	"time"

	"github.com/uber-go/zap"
	"gopkg.in/redis.v5"

	"dictservice/model"
	"dictservice/types"
)

func NewDictLogService(redisClient *redis.Client, logger zap.Logger) model.DictLogService {
	return &dictLogServiceImpl{redisClient, logger}
}

const (
	dict_log_key      = "tc:dict:log"
	dict_log_data_key = "tc:dict:log:data"
)

type dictLogServiceImpl struct {
	redisClient *redis.Client
	logger      zap.Logger
}

func (s *dictLogServiceImpl) CreateLog(log types.DictLog) error {
	id := time.Now().Unix()
	logEntity := types.DictLogEntity{Id: id, DictLog: log}

	v, err := json.Marshal(logEntity)
	if err != nil {
		return err
	}

	value := fmt.Sprintf("%v", id)
	score := id
	logEntityJson := string(v)

	//TODO: in transaction
	s.redisClient.ZAdd(dict_log_key, redis.Z{Member: value, Score: float64(score)})
	s.redisClient.HSet(dict_log_data_key, value, logEntityJson)

	return nil
}

func (s *dictLogServiceImpl) FindAllLogs() ([]types.DictLog, error) {
	reply, err := s.redisClient.HVals(dict_log_data_key).Result()
	if err != nil {
		s.logger.Error(fmt.Sprintf("error: %v", err))
		return nil, err
	}

	logs := make([]types.DictLog, len(reply))
	for i, v := range reply {
		log := types.DictLog{}
		if err := json.Unmarshal([]byte(v), &log); err != nil {
			return nil, err
		}

		logs[i] = log
	}
	return logs, nil
}
