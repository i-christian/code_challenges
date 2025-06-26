package main

import (
	"log"
	"os"
	"strings"

	"gogrep/internal/handlers"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatalf("Not enough command line arguments.\nUsage: gogrep [flag] filename")
	}

	fileName := os.Args[2]
	flag := strings.TrimSpace(os.Args[1])
	file, err := os.Open(fileName)
	if err != nil {
		log.Fatal("failed to open and read file", err.Error())
	}
	defer file.Close()

	if len(flag) == 0 {
		handlers.EmptyFlag(file)
	} else if len(flag) == 1 {
		handlers.SingleLetter(file, flag)
	}
}
