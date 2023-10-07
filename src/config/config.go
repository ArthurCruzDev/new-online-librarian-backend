package config

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

var (
	Porta = ""
)

func LoadEnvVariables() {

	var erro error

	if erro = godotenv.Load(); erro != nil {
		log.Fatal(erro)
	}

	Porta = os.Getenv("API_PORT")
	if Porta == "" {
		Porta = "80"
	}
}
