package main

import (
	"fmt"
	"log"
	"log/slog"
	"os"
	"strings"

	count "ccwc/internal"
)

// processFlags function accepts a file and a flag and calls the function that matches the flag to handle the file
func processFlags(flag, fileName string) (int, error) {
	f, err := os.Open(fileName)
	if err != nil {
		slog.Error("failed to open file", "error", err.Error())
		os.Exit(1)
	}

	defer f.Close()

	mappedCommands := make(map[string]func(*os.File) (int, error))
	mappedCommands["-c"] = count.Bytes
	mappedCommands["-l"] = count.Lines
	mappedCommands["-w"] = count.Words
	mappedCommands["-m"] = count.Characters

	result, err := mappedCommands[flag](f)
	return result, err
}

// define main for the application to recieve command line args ()
func main() {
	args := os.Args
	if len(args) <= 1 {
		log.Fatal("Not enough arguments")
	}

	countSlice := make([]int, 0)
	defaultFlags := []string{"-l", "-w", "-c"}
	var fileName string

	if len(args) > 1 {
		if len(args) == 2 && !strings.Contains(args[1], "-") {
			fileName = args[1]
			for _, flag := range defaultFlags {
				result, err := processFlags(flag, fileName)
				if err != nil {
					slog.Error("An error occured", "error", err.Error())
					break
				}
				countSlice = append(countSlice, result)
			}
		} else if len(args) == 3 {
			fileName = args[2]
			result, err := processFlags(args[1], fileName)
			if err != nil {
				slog.Error("An error occured", "error", err.Error())
			}

			countSlice = append(countSlice, result)
		} else {
			fmt.Println("Too many command line arguments")
			return
		}
	}

	for i := range countSlice {
		fmt.Print("   ", countSlice[i])
	}
	fmt.Println(" ", fileName)
}
