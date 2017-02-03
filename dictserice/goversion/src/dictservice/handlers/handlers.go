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
	return okJSONAny(ctx, map[string]string{"message": "pong"})
}

func (c *dictLogController) CreateLog(ctx echo.Context) error {
	c.logger.Info("Post to /dict/logs")

	dictLog := types.DictLog{}
	if err := ctx.Bind(&dictLog); err != nil {
		c.logger.Error(fmt.Sprintf("error: %v", err))
		return errorJSON(ctx, respMessage(err.Error()))
	}

	id, err := c.dictLogService.CreateLog(dictLog)
	if err != nil {
		c.logger.Error(fmt.Sprintf("error: %v", err))
		return errorJSON(ctx, respMessage(err.Error()))
	}

	return okJSON(ctx, respData(types.Id(id)))
}

func (c *dictLogController) ListLogs(ctx echo.Context) error {
	c.logger.Info("Get /dict/logs")

	logs, err := c.dictLogService.FindAllLogs()
	if err != nil {
		c.logger.Error(fmt.Sprintf("error: %v", err))
		return errorJSON(ctx, respMessage(err.Error()))
	}

	return okJSON(ctx, respData(logs))
}

func (c *dictLogController) DeleteLog(ctx echo.Context) error {
	id := ctx.Param("id")
	c.logger.Info("Delete /dict/" + id)
	if id == "" {
		c.logger.Warn("路径参数id为空")
		return badRequestSON(ctx, respMessage("路径参数id为空"))
	}

	if err := c.dictLogService.DeleteLog(id); err != nil {
		switch err.(type) {
		case types.LogNoExistsError:
			return badRequestSON(ctx, respMessage(err.Error()))
		default:
			c.logger.Error(fmt.Sprintf("error: %v", err))
			return errorJSON(ctx, respMessage("删除日志出错"))
		}
	}

	return okJSON(ctx, respMessage("success"))
}

func respMessage(message string) types.Response {
	return types.Response{Message: message}
}

func respData(data interface{}) types.Response {
	return types.Response{Data: data}
}

//func respMessageData(message string, data interface{}) types.Response {
//	return types.Response{Message: message, Data: data}
//}

func badRequestSON(ctx echo.Context, resp types.Response) error {
	trySetStatus(&resp, http.StatusBadRequest)
	return ctx.JSON(http.StatusBadRequest, resp)
}

func errorJSON(ctx echo.Context, resp types.Response) error {
	trySetStatus(&resp, http.StatusInternalServerError)
	return ctx.JSON(http.StatusInternalServerError, resp)
}

func okJSON(ctx echo.Context, resp types.Response) error {
	trySetStatus(&resp, http.StatusOK)
	return okJSONAny(ctx, resp)
}

func okJSONAny(ctx echo.Context, resp interface{}) error {
	return ctx.JSON(http.StatusOK, resp)
}

func trySetStatus(resp *types.Response, status int) {
	if resp.Status == 0 {
		resp.Status = status
	}
}
