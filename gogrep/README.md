# Build Your Own grep

This challenge is to build your own version of the Unix command line tool grep.

The Unix command line tools are a great metaphor for good software engineering and they follow the Unix Philosophies of:
  - Writing simple parts connected by clean interfaces - each tool does just one thing and provides a simple CLI that handles text input from either files or file streams.
  - Design for simplicity; add complexity only where you must.
  - Design programs to be connected to other programs - each tool can be easily connected to other tools to create incredibly powerful compositions.

## The functional requirements for grep
```
  % man grep
	       The grep utility searches any given input files, selecting lines that
     match one or more patterns.  By default, a pattern matches an input line
     if the regular expression (RE) in the pattern matches the input line
     without its trailing newline.  An empty expression matches every line.
     Each input line that matches at least one of the patterns is written to
     the standard output.
```

## How to Use 🛠️
- Compile the project: `go build .`
## Running examples:
- Basic search
- Recursive search option
- Piping mode
  
