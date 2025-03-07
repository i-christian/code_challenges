package count

import (
	"bufio"
	"os"
)

// Bytes function handles the command for byte count
func Bytes(f *os.File) (int, error) {
	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanBytes)
	byteCounts := 0

	for scanner.Scan() {
		byteCounts++
	}

	if err := scanner.Err(); err != nil {
		return 0, err
	}

	return byteCounts, nil
}

// Lines function handles the command for line count
func Lines(f *os.File) (int, error) {
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

// Words function returns the total word count in a file
func Words(f *os.File) (int, error) {
	wordCount := 0
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

// CountBytes function handles the command for byte count
func CountBytes(f *os.File) (int, error) {
	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanBytes)
	byteCounts := 0

	for scanner.Scan() {
		byteCounts++
	}

	if err := scanner.Err(); err != nil {
		return 0, err
	}

	return byteCounts, nil
}

// CountLines function handles the command for line count
func CountLines(f *os.File) (int, error) {
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

// Characters function returns the total word count in a file
func Characters(f *os.File) (int, error) {
	charCount := 0
	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanRunes)
	for scanner.Scan() {
		charCount++
	}

	if err := scanner.Err(); err != nil {
		return 0, err
	}

	return charCount, nil
}
