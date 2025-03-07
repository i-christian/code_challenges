package main

import (
	"fmt"
	"log/slog"
	"os"
)

// define main for the application to recieve command line args ()
func main() {
	args := os.Args
	if len(args) < 3 {
		slog.Error("Not enough arguments, It should be like: go run . -c test.txt")
		os.Exit(1)
	}

	fileBytes, err := os.ReadFile(args[2])
	if err != nil {
		slog.Error("Failed to read file", "file", args[2], "error", err.Error())
		return
	}

	fmt.Println(len(fileBytes), args[2])
}
