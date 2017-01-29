package middleware

import (
	//"time"

	"github.com/labstack/echo"
)

func XRuntime(next echo.HandlerFunc) echo.HandlerFunc {
	return func(ctx echo.Context) (err error) {
		// link: https://github.com/labstack/echo/issues/831
		//start := time.Now()
		//
		//ctx.Response().OnBeforeWriteHeader(func(resp *echo.Response) {
		//	resp.Header().Set("x-runtime", time.Since(start).String())
		//})

		return next(ctx)
	}
}
