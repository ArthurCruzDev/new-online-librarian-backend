package health

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func GetHealthCheck(context *gin.Context) {
	context.Status(http.StatusOK)
}
