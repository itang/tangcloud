package model

import (
	"dictservice/types"
)

type DictLogService interface {
	CreateLog(log types.DictLog) (id int64, err error)

	FindAllLogs() (logs []types.DictLogEntity, err error)

	// Delete log by id.
	//
	// error: LogNoExistsError | other
	DeleteLog(id int64) error

	ExistsLog(id int64) (exists bool, err error)
}
