# What works in ReCodEx

## D lang

D has 3 major compilers. One of them - LLVM based LDC - does not work. That's caused by dynamic linking of standard library. Other compilers - GDC and DMD - work out of the box.

## Rust

Rust works... ...when you compile it statically (which is not the default, surprisingly).

```sh
# install new SDK
rustup target add x86_64-unknown-linux-musl

# build your code
cargo build --target x86_64-unknown-linux-musl --release
```

It might also be a good idea to strip the binary ahead of running it through `anyexec2c`, because it's quite big and by stripping it, you can get it from 5MB to something like 0.5MB.

```sh
strip build/x86_64-unknown-linux-musl/release/$YOUR_PROJECT_NAME
anyexec2C -x build/x86_64-unknown-linux-musl/release/$YOUR_PROJECT_NAME > /tmp/source.c
```

## C

It's pretty much useless, but you can run any C binaries. The two main compilers, `GCC` and `clang`, both work out of the box.

## Go

Resulting binaries work just as you would expect. Nothing special is required.

## Python

Both version 2 and 3 are installed in their standard location. Beware of long startup time. And don't forget to add the shebang `#!/usr/bin/python3` or `#!/usr/bin/python` for Python2.

## Perl (5)

Perl is installed at `/usr/bin/perl` and also seems to work. Beware of worse performance compared to C and don't forget the shebang `#!/usr/bin/perl`
