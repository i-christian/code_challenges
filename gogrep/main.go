package main

import (
	"fmt"
	"io"
	"log"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatalf("Not enough command line arguments.\nUsage: gogrep [flag] filename")
	}

	fileName := os.Args[2]
	flag := os.Args[1]

	if len(flag) == 0 {
		file, err := os.Open(fileName)
		if err != nil {
			log.Fatal("failed to open and read file", err.Error())
		}

		defer file.Close()
		buf := make([]byte, 2048)
		for {
			n, err := file.Read(buf)
			if err == io.EOF {
				break
			}
			if err != nil {
				fmt.Println(err)
				continue
			}

			if n > 0 {
				fmt.Println(string(buf[:n]))
			}
		}

	}
}
