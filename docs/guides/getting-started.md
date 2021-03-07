# Getting started

## Install

It requires the following software on your system.

- [`cargo`](https://github.com/rust-lang/cargo), the Rust package manager
- [`clang`](https://clang.llvm.org/), the C compiler

When you install them, run the following command.

```
cargo install --git https://github.com/ein-lang/ein
```

## Initializing a package

```
ein init command foo
cd foo
ein build
```

Then, you can start editing `*.ein` files in the directory.
