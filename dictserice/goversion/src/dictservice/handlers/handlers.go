package handlers

import (
	"fmt"
	"net/http"

	"github.com/labstack/echo"
	"github.com/uber-go/zap"

	"dictservice/model"
	"dictservice/types"
)

type dictLogController struct {
	dictLogService model.DictLogService
	logger         zap.Logger
}

func NewDictLogController(dictLogService model.DictLogService, logger zap.Logger) *dictLogController {
	return &dictLogController{dictLogService, logger}
}

func (c *dictLogController) Ping(ctx echo.Context) error {
	return ctx.JSON(http.StatusOK, map[string]string{"message": "pong"})
}

func (c *dictLogController) CreateLog(ctx echo.Context) error {
	c.logger.Info("Post to /dict/logs")

	dictLog := types.DictLog{}
	if err := ctx.Bind(&dictLog); err != nil {
		c.logger.Error(fmt.Sprintf("error: %v", err))
		return ctx.JSON(http.StatusInternalServerError, types.Response{Status: 500, Message: err.Error()})
	}

	if err := c.dictLogService.CreateLog(dictLog); err != nil {
		return ctx.JSON(http.StatusInternalServerError, types.Response{Status: 500, Message: err.Error()})
	}

	return ctx.JSON(http.StatusOK, types.Response{Status: 200})
}

func (c *dictLogController) ListLogs(ctx echo.Context) error {
	c.logger.Info("Get /dict/logs")

	logs, err := c.dictLogService.FindAllLogs()
	if err != nil {
		c.logger.Error(fmt.Sprintf("error: %v", err))
		return ctx.JSON(http.StatusInternalServerError, types.Response{Status: http.StatusInternalServerError, Message: err.Error()})
	}

	return ctx.JSON(http.StatusOK, types.Response{Status: http.StatusOK, Data: logs})
}
