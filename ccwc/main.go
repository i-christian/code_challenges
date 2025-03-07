package main

import (
	"bufio"
	"fmt"
	"log/slog"
	"os"
)

// countBytes function handles the command for byte count
func countBytes(fileName string) (int, error) {
	fileBytes, err := os.ReadFile(fileName)
	if err != nil {
		return 0, err
	}

	return len(fileBytes), nil
}

// countLines function handles the command for line count
func countLines(fileName string) (int, error) {
	f, err := os.Open(fileName)
	if err != nil {
		return 0, err
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)
	lineCount := 0
	for scanner.Scan() {
		lineCount++
	}

	if err := scanner.Err(); err != nil {
		return 0, err
	}

	return lineCount, nil
}

// countWords function returns the total word count in a file
func countWords(fileName string) (int, error) {
	f, err := os.Open(fileName)
	if err != nil {
		return 0, nil
	}

	wordCount := 0
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanWords)
	for scanner.Scan() {
		wordCount++
	}

	if err := scanner.Err(); err != nil {
		return 0, err
	}

	return wordCount, nil
}

// define main for the application to recieve command line args ()
func main() {
	args := os.Args
	if len(args) < 3 {
		slog.Error("Not enough arguments")
		os.Exit(1)
	}

	mappedCommands := make(map[string]func(string) (int, error))
	mappedCommands["-c"] = countBytes
	mappedCommands["-l"] = countLines
	mappedCommands["-w"] = countWords

	result, err := mappedCommands[args[1]](args[2])
	if err != nil {
		slog.Error("An error occured", "error", err.Error())
		os.Exit(1)
	}

	fmt.Printf("%d %s\n", result, args[2])
}
