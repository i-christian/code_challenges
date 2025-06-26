package handlers

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

// EmptyFlag function handles the empty flag grep option
func EmptyFlag(file *os.File) {
	buf := make([]byte, 1024)
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

// SingleLetter function handles the single letter pattern
func SingleLetter(file *os.File, flag string) {
	reader := bufio.NewReader(file)

	buf := make([]string, 0)
	for {
		line, err := reader.ReadString('\n')
		if err == io.EOF {
			break
		}
		if err != nil {
			fmt.Println(err)
			continue
		}

		if strings.Contains(line, flag) {
			buf = append(buf, line)
		}
	}

	if len(buf) == 0 {
		os.Exit(1)
	} else {
		for _, line := range buf {
			fmt.Print(line)
		}
		os.Exit(0)
	}
}
