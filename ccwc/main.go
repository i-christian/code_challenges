package main

import (
	"fmt"
	"log/slog"
	"os"

	count "ccwc/internal"
)

// define main for the application to recieve command line args ()
func main() {
	args := os.Args
	if len(args) < 3 {
		slog.Error("Not enough arguments")
		os.Exit(1)
	}

	mappedCommands := make(map[string]func(*os.File) (int, error))
	mappedCommands["-c"] = count.Bytes
	mappedCommands["-l"] = count.Lines
	mappedCommands["-w"] = count.Words
	mappedCommands["-m"] = count.Characters

	f, err := os.Open(args[2])
	if err != nil {
		slog.Error("failed to open file", "error", err.Error())
		os.Exit(1)
	}

	defer f.Close()

	result, err := mappedCommands[args[1]](f)
	if err != nil {
		slog.Error("An error occured", "error", err.Error())
		os.Exit(1)
	}

	fmt.Printf("%d %s\n", result, args[2])
}
