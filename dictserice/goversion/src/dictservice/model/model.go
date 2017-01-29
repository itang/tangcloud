package model

import (
	"dictservice/types"
	"encoding/json"
	"fmt"
	"github.com/uber-go/zap"
	"gopkg.in/redis.v5"
	"time"
)

type DictLogService interface {
	CreateLog(log types.DictLog) error
	FindAllLogs() (logs []types.DictLog, err error)
}

func NewDefaultDictLogServiceImpl(redisClient *redis.Client, logger zap.Logger) DictLogService {
	return &dictLogServiceImpl{redisClient, logger}
}

const (
	DICT_LOG_KEY      = "tc:dict:log"
	DICT_LOG_DATA_KEY = "tc:dict:log:data"
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
	s.redisClient.ZAdd(DICT_LOG_KEY, redis.Z{Member: value, Score: float64(score)})
	s.redisClient.HSet(DICT_LOG_DATA_KEY, value, logEntityJson)

	return nil
}

func (s *dictLogServiceImpl) FindAllLogs() ([]types.DictLog, error) {
	reply, err := s.redisClient.HVals(DICT_LOG_DATA_KEY).Result()
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
