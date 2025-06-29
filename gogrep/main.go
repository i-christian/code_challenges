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

	flag := strings.TrimSpace(os.Args[1])
	// os.Pipe()

	if len(os.Args) == 3 && len(flag) == 0 {
		fileName := os.Args[2]
		file, err := os.Open(fileName)
		if err != nil {
			log.Fatal(err.Error())
		}
		defer file.Close()

		// check if file is executable and skip reading it if it is
		if !handlers.IsExecutable(file) {
			if len(flag) == 0 {
				handlers.EmptyFlag(file)
			} else if len(flag) == 1 {
				handlers.SingleLetter(file, flag)
			}
		}
	} else if len(os.Args) > 3 {
		word := os.Args[2]
		if flag == "-r" && len(os.Args[3]) != 0 {
			handlers.Recursive(word, false)
		} else if flag == "-v" && len(os.Args[3]) != 0 {
			handlers.Recursive(word, true)
		}
	}
}
