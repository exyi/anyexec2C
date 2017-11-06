# What works in ReCodEx

## D lang

D has 3 major compilers. One of them - LLVM based LDC - does not work. That's caused by dynamic linking of standard library. Other compilers - GDC and DMD - work out of the box.

## Rust

Standard `rustc` compiler does not work. The binaries are sometimes too big and must be stripped. But even if you do that, the binary does not execute properly. Currently, we have no idea why.

## C

It's pretty much useless, but you can run any C binaries. The two main compilers, GCC and `clang`, both work out of the box.