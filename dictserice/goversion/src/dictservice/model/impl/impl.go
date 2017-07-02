package impl

import (
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/go-redis/redis"
	"github.com/pkg/errors"
	gouuid "github.com/satori/go.uuid"
	"go.uber.org/zap"

	"dictservice/model"
	"dictservice/types"
)

func NewDictLogService(redis *redis.Client) model.DictLogService {
	return &dictLogServiceImpl{redis}
}

const (
	dict_log_key      = "tc:dict:log"
	dict_log_data_key = "tc:dict:log:data"
)

type dictLogServiceImpl struct {
	redis *redis.Client
}

func (s *dictLogServiceImpl) CreateLog(log types.DictLog) (id int64, err error) {
	id = time.Now().Unix()
	logEntity := types.DictLogEntity{Id: types.Id(id), DictLog: log}

	v, err := json.Marshal(logEntity)
	if err != nil {
		logger, _ := zap.NewProduction()
		defer logger.Sync()
		logger.Error(fmt.Sprintf("error: %v", err))
		return
	}

	value := id
	score := float64(time.Now().Unix())
	logEntityJson := string(v)

	//TODO: in transaction
	s.redis.ZAdd(dict_log_key, redis.Z{Member: value, Score: score})
	s.redis.HSet(dict_log_data_key, fmt.Sprintf("%d", id), logEntityJson)

	return
}

func (s *dictLogServiceImpl) FindAllLogs() ([]types.DictLogEntity, error) {
	reply, err := s.redis.HVals(dict_log_data_key).Result()
	if err != nil {
		logger, _ := zap.NewProduction()
		defer logger.Sync()
		logger.Error(fmt.Sprintf("error: %v", err))
		return nil, err
	}

	logs := make([]types.DictLogEntity, len(reply))
	for i, v := range reply {
		log := types.DictLogEntity{}
		if err := json.Unmarshal([]byte(v), &log); err != nil {
			return nil, err
		}

		logs[i] = log
	}
	return logs, nil
}

func (s *dictLogServiceImpl) DeleteLog(id int64) error {
	if id == 0 {
		return errors.New("id不能为空")
	}

	c, err := s.redis.HDel(dict_log_data_key, fmt.Sprintf("%d", id)).Result()
	if err != nil {
		logger, _ := zap.NewProduction()
		defer logger.Sync()
		logger.Error(fmt.Sprintf("error: %v", err))
		return err
	}
	if c <= 0 {
		msg := fmt.Sprintf("id为%s的log不存在", id)
		logger, _ := zap.NewProduction()
		defer logger.Sync()
		logger.Error(msg)
		return types.LogNoExistsError{Id: types.Id(id)}
	}

	return nil
}

func (s *dictLogServiceImpl) ExistsLog(id int64) (exists bool, err error) {
	return s.redis.HExists(dict_log_data_key, fmt.Sprintf("%d", id)).Result()
}

func uuid() string {
	return strings.Replace(gouuid.NewV4().String(), "-", "", -1)
}
