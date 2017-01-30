package model

import (
	"dictservice/types"
)

type DictLogService interface {
	CreateLog(log types.DictLog) error

	FindAllLogs() (logs []types.DictLog, err error)
}
