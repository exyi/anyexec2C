# AnyExec2C

This is a simple program used wrap any executable into some source code. When the resulting source is compiled and run, it acts as a bootstrap stage. It unpacks the executable from within itself, dumps it to disk and runs it afterwards via Unix `exec` syscall. Historically, main target language has been C. As of now, also C# and Python3 are supported as a target language.

## Purpose

Primary purpose of this program is to bypass programming language limitations in [ReCodEx](https://github.com/ReCodEx) - software used to programmatically check validity of student's code. Check out [WORKING_LANGUAGES.md](WORKING_LANGUAGES.md) for more info about which languages work in ReCodEx and what we have found out about them. However, this tool is not limited to ReCodEx. The generated code should work in other program testing environments too.

## Usage

``` bash
# simplest usage
anyexec2c -x executableFile > source.c

# insert actual source code as comments
anyexec2c -x executableFile -c original_source.any > source.c

# if something does not work, you can add some diagnostic return codes using `-e` flag
anyexec2c -e -x executable > source.c

# for supported languages, you can just pass in the source code
anyexec2c -b source.d > source.c      # (D lang with  DMD compiler)
anyexec2c -b source.go > source.c     # (Go)
anyexec2c -b src/main.rs > source.c   # (Rust using cargo - necessary to call from projects main dir)

# we can also generate C# programs for cases when C/C++ is not an allowed language (using --target or -t switch)
# this packs our memory test tool into a C# environement
anyexec2c -b tools/memtest.c -t C# > memtest.cs
# or a Python program :)
anyexec2c -b tools/memtest.c -t python > memtest.py
```
