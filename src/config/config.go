package config

import (
	"database/sql"
	"log"
	"os"
	"time"

	_ "github.com/go-sql-driver/mysql"
	"github.com/joho/godotenv"
)

var (
	API_PORT    = ""
	DB_HOST     = ""
	DB_PORT     = ""
	DB_USER     = ""
	DB_PASSWORD = ""
	DB_NAME     = ""
)

func LoadEnvVariables() {

	var erro error

	if erro = godotenv.Load(); erro != nil {
		log.Fatal(erro)
	}

	DB_HOST = os.Getenv("DB_HOST")
	if DB_HOST == "" {
		log.Fatal("Database host not informed")
	}

	DB_USER = os.Getenv("DB_USER")
	if DB_USER == "" {
		log.Fatal("Database user not informed")
	}

	DB_PASSWORD = os.Getenv("DB_PASSWORD")
	if DB_PASSWORD == "" {
		log.Fatal("Database password not informed")
	}

	DB_NAME = os.Getenv("DB_NAME")
	if DB_NAME == "" {
		log.Fatal("Database name not informed")
	}

	DB_PORT = os.Getenv("DB_PORT")
	if DB_PORT == "" {
		DB_PORT = "3306"
	}

	API_PORT = os.Getenv("API_PORT")
	if API_PORT == "" {
		API_PORT = "80"
	}

}

func OpenDatabaseConnection() {
	db, err := sql.Open("mysql", DB_USER+":"+DB_PASSWORD+"@tcp("+DB_HOST+":"+DB_PORT+")/"+DB_NAME)

	if err != nil {
		panic(err)
	}

	err = db.Ping()

	if err != nil {
		log.Fatal("Failed to connect to database: ", err)
	}

	db.SetConnMaxLifetime(time.Minute * 5)
	db.SetMaxOpenConns(20)
	db.SetMaxIdleConns(20)
}
