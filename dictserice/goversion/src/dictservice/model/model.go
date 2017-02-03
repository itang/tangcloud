package model

import (
	"dictservice/types"
)

type DictLogService interface {
	CreateLog(log types.DictLog) (id string, err error)

	FindAllLogs() (logs []types.DictLogEntity, err error)

	// Delete log by id.
	//
	// error: LogNoExistsError | other
	DeleteLog(id string) error

	ExistsLog(id string) (exists bool, err error)
}
