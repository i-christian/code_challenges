package handlers

import (
	"bufio"
	"fmt"
	"io"
	"io/fs"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// EmptyFlag function handles the empty flag grep option
func EmptyFlag(file *os.File) {
	_, err := io.Copy(os.Stdout, file)
	if err != nil {
		fmt.Println(err.Error())
	}
}

// IgnoreCaseSearch function ignores the case of your search parameter and search for both upper and lower case variations.
func IgnoreCaseSearch(file *os.File, word string) {
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

		if strings.Contains(strings.ToLower(line), strings.ToLower(word)) {
			buf = append(buf, line)
		}
	}

	printMatches(buf)
}

// InvertSearch function returns all lines that do not contain the specified pattern
func InvertSearch(file *os.File, word string) {
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

		if !strings.Contains(line, word) {
			buf = append(buf, line)
		}
	}

	printMatches(buf)
}

// WordSearch function handles the single letter pattern
func WordSearch(file *os.File, flag string) {
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

	printMatches(buf)
}

// Recursive supports the goal to recurse a directory tree
func Recursive(word string) {
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

			if strings.Contains(line, word) {
				buf = append(buf, fmt.Sprintf("%s:%s", file, line))
			}
		}

	}

	printMatches(buf)
}

// DigitRegex function performs a regex search for digits
func DigitRegex(file *os.File, flag string) {
	reader := bufio.NewReader(file)
	re, _ := regexp.Compile(`\d`)

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

		if re.MatchString(line) {
			buf = append(buf, line)
		}
	}

	printMatches(buf)
}

// WordRegex function performs a regex search for word character
func WordRegex(file *os.File, flag string) {
	reader := bufio.NewReader(file)
	re, _ := regexp.Compile(`\w`)

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

		if re.MatchString(line) {
			buf = append(buf, line)
		}
	}

	printMatches(buf)
}

// AnchorRegex function performs regex search using front anchor(^)
func AnchorRegex(file *os.File, flag string) {
	reader := bufio.NewReader(file)
	re := regexp.MustCompile(fmt.Sprintf("(?m)%s", flag))

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

		if re.MatchString(line) {
			buf = append(buf, line)
		}
	}

	printMatches(buf)
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

func printMatches(buf []string) {
	if len(buf) == 0 {
		os.Exit(1)
	} else {
		for _, line := range buf {
			fmt.Print(line)
		}
		os.Exit(0)
	}
}

func OpenFile(name string) *os.File {
	file, err := os.Open(name)
	if err != nil {
		log.Fatal(err.Error())
	}

	ok := IsExecutable(file)
	if ok {
		fmt.Printf("%s: binary file matches\n", name)
		os.Exit(1)
	}

	return file
}

func IsExecutable(f *os.File) bool {
	info, _ := f.Stat()

	// Check if any of the executable bits (owner, group, or others) are set
	// The 0111 octal bitmask represents the executable permissions for all user types.
	return info.Mode().Perm()&0o111 != 0
}
