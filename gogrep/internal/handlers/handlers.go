package handlers

import (
	"bufio"
	"fmt"
	"io"
	"io/fs"
	"os"
	"path/filepath"
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

// Recursive supports the goal to recurse a directory tree
func Recursive(word string, inverts bool) {
	filePath := "."
	files := findFiles(filePath)
	buf := make([]string, 0)

	for _, file := range files {
		f, err := os.Open(file)
		if err != nil {
			fmt.Println(err.Error())
			continue
		}
		defer f.Close()

		// check if file is executable and skip reading it if it is
		if IsExecutable(f) {
			continue
		}

		reader := bufio.NewReader(f)

		for {
			line, err := reader.ReadString('\n')
			if err == io.EOF {
				break
			}
			if err != nil {
				fmt.Println(err)
				continue
			}

			if !inverts {
				if strings.Contains(line, word) {
					buf = append(buf, fmt.Sprintf("%s:%s", file, line))
				}
			} else {
				if !strings.Contains(line, word) {
					buf = append(buf, fmt.Sprintf("%s:%s", file, line))
				}
			}
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

func findFiles(rootPath string) []string {
	files := make([]string, 0)
	filepath.WalkDir(rootPath, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		if !d.IsDir() {
			files = append(files, path)
		}

		return nil
	})

	return files
}

func IsExecutable(f *os.File) bool {
	info, _ := f.Stat()

	// Check if any of the executable bits (owner, group, or others) are set
	// The 0111 octal bitmask represents the executable permissions for all user types.
	if info.Mode().Perm()&0o111 != 0 {
		return true
	}

	return false
}
