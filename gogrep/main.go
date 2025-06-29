package main

import (
	"fmt"
	"log"
	"os"
	"strings"

	"gogrep/internal/handlers"
)

func main() {
	fi, err := os.Stdin.Stat()
	if err != nil {
		fmt.Println(err.Error())
	}

	if fi.Mode()&os.ModeNamedPipe == 0 {
		handleOptions()
	} else {
		fmt.Println("hi pipe!")
	}
}

func handleOptions() {
	if len(os.Args) < 3 {
		log.Printf("Not enough command line arguments.\nUsage: gogrep [flag] filename")
		os.Exit(1)
	}

	flag := strings.TrimSpace(os.Args[1])

	if len(os.Args) > 3 {
		word := os.Args[2]
		if flag == "-r" && len(os.Args[3]) != 0 {
			handlers.Recursive(word, false)
		} else if flag == "-v" && len(os.Args[3]) != 0 {
			handlers.Recursive(word, true)
		}
	} else {
		fileName := os.Args[2]
		file, err := os.Open(fileName)
		if err != nil {
			log.Fatal(err.Error())
		}
		defer file.Close()

		// check if file is executable and skip reading it if it is
		if !handlers.IsExecutable(file) {
			if len(flag) >= 1 {
				handlers.WordSearch(file, flag)
			} else {
				handlers.EmptyFlag(file)
			}
		}
	}
}
