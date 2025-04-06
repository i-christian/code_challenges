# ccwc 

## This is from `Build Your Own wc Tool` Challenge

- This challenge is to build your own version of the Unix command line tool wc!

The command line app supports `-c`, `-m`, `-l`, `-w` flags

### Instructions to run
- Build the project 
  ```
    cargo build
  ```
- Run using a flag e.g
  ```
    ./target/debug/ccwc -c test.txt
    
    342190 test.txt
  ```
- Run without any optional flags
  ```
    ./target/debug/ccwc test.txt

        7145   58164  342190 test.txt
  ```
- Read from standard input if no filename is specified
  ```
    cat test.txt | ./target/debug/ccwc -l
      
      7145
  ```
