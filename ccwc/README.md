# ccwc 

## This is from `Build Your Own wc Tool` Challenge

- This challenge is to build your own version of the Unix command line tool wc!

The command line app supports `-c`, `-m`, `-l`, `-w` flags

### Instructions to run
- Build the project 
  ```
    go build
  ```
- Run using a flag e.g
  ```
    ./ccwc -c test.txt
    
    342190 test.txt
  ```
- Run without any optional flags
  ```
    ./ccwc -c test.txt

        7145   58164  342190 test.txt
  ```
- Read from standard input if no filename is specified
  ```
    cat test.txt | ./ccwc -l
      
      7145
  ```
