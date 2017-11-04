# AnyExec2C

This is a simple program used to convert any executable to C source code, that will run that executable after being compiled. It works by saving the executable file in the C source code, which will save it to file `myBinaryCode` and run it.

## Purpose

Primary purpose of this program is to bypass programming language limitations in ReCodEx - software used to programmatically check validity of student's code.

## Usage

``` bash
# simplest usage
cat anyExecutableFile | anyexec2c > source.c

# insert actual source code as comments
cat anyExecutableFile | anyexec2c original_source.any > source.c
```