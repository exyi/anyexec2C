# What works in ReCodEx

## D lang

D has 3 major compilers. One of them - LLVM based LDC - does not work. That's caused by dynamic linking of standard library. Other compilers - GDC and DMD - work out of the box.

## Rust

Rust works without standard library. It's necessary to use `#[no_std]` and external crate `libc`, which will provide the most basic functionality. And the binaries are sometimes too big. You can make them considerably smaller by stripping symbols with `strip`.

## C

It's pretty much useless, but you can run any C binaries. The two main compilers, `GCC` and `clang`, both work out of the box.

## Go

Resulting binaries work just as you would expect. Nothing special is required.

## Python

Both version 2 and 3 are installed in their standard location. Beware of long startup time. And don't forget to add the shebang `#!/usr/bin/python3` or `#!/usr/bin/python` for Python2.

## Perl (5)

Perl is installed at `/usr/bin/perl` and also seems to work. Beware of worse performance compared to C and don't forget the shebang `#!/usr/bin/perl`
