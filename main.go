package main

import (
	"net/http"
	"new-online-librarian-backend/src/config"
	"new-online-librarian-backend/src/features/health"

	"github.com/gin-gonic/gin"
)

func main() {
	config.LoadEnvVariables()
	r := gin.Default()

	r.GET("/health", health.GetHealthCheck)

	r.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})
	r.Run(":" + config.Porta)
}
