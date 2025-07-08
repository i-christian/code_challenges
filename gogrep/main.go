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
		processInput(nil)
	} else {
		r, w := io.Pipe()
		defer r.Close()

		go func() {
			defer w.Close()
			_, _ = io.Copy(w, os.Stdin)
		}()

		processInput(r)
	}
}

func processInput(pipedInput *io.PipeReader) {
	flag := strings.TrimSpace(os.Args[1])
	if pipedInput != nil {
		f, err := os.CreateTemp("", "temp")
		if err != nil {
			log.Fatal(err)
		}
		defer os.Remove(f.Name())

		_, err = io.Copy(f, pipedInput)
		if err != nil {
			log.Fatal(err)
		}

		handleOptions(flag, f.Name())
	}

	if len(os.Args) < 3 {
		log.Printf("Not enough command line arguments.\nUsage: gogrep [flag] filename")
		os.Exit(1)
	}

	handleOptions(flag, "")
}

func handleOptions(input ...string) {
	flag := input[0]
	tempFile := input[1]
	var fileName string

	if len(os.Args) > 3 || (len(tempFile) > 0 && len(os.Args) == 3) {
		word := os.Args[2]

		if flag == "-r" {
			handlers.Recursive(word)
		} else {
			if len(strings.TrimSpace(tempFile)) > 0 {
				fileName = tempFile
			} else {
				fileName = os.Args[3]
			}

			file := handlers.OpenFile(fileName)
			defer file.Close()

			switch flag {
			case "-i":
				handlers.IgnoreCaseSearch(file, word)
			case "-v":
				handlers.InvertSearch(file, word)
			}
		}
	} else if len(os.Args) == 3 {
		if len(strings.TrimSpace(tempFile)) > 0 {
			fileName = tempFile
		} else {
			fileName = os.Args[2]
		}

		file := handlers.OpenFile(fileName)
		defer file.Close()

		if flag == `\d` {
			handlers.DigitRegex(file, flag)
		} else if flag == `\w` {
			handlers.WordRegex(file, flag)
		} else if strings.HasPrefix(flag, `^`) || strings.HasSuffix(flag, `$`) {
			handlers.AnchorRegex(file, flag)
		} else if len(flag) >= 1 {
			handlers.WordSearch(file, flag)
		} else {
			handlers.EmptyFlag(file)
		}
	} else {
		log.Printf("Not enough command line arguments.\nUsage: ./gogrep [flag] filename")
		os.Exit(1)

	}
}
