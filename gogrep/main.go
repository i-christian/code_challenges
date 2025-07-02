package main

import (
	"fmt"
	"io"
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
		handleOptions(nil)
	} else {
		r, w := io.Pipe()
		defer r.Close()

		go func() {
			defer w.Close()
			_, _ = io.Copy(w, os.Stdin)
		}()

		handleOptions(r)
	}
}

func handleOptions(pipedInput *io.PipeReader) {
	if pipedInput != nil {
		_, _ = io.Copy(os.Stdout, pipedInput)
	}

	if len(os.Args) < 3 {
		log.Printf("Not enough command line arguments.\nUsage: gogrep [flag] filename")
		os.Exit(1)
	}

	flag := strings.TrimSpace(os.Args[1])

	if len(os.Args) > 3 {
		word := os.Args[2]

		if flag == "-r" {
			handlers.Recursive(word)
		} else {
			fileName := os.Args[3]
			file := handlers.OpenFile(fileName)
			defer file.Close()

			switch flag {
			case "-i":
				handlers.InvertSearch(file, word)
			case "-v":
				handlers.InvertSearch(file, word)
			}
		}
	} else {
		fileName := os.Args[2]
		file := handlers.OpenFile(fileName)
		defer file.Close()

		if len(flag) >= 1 {
			handlers.WordSearch(file, flag)
		} else {
			handlers.EmptyFlag(file)
		}
	}
}
