# Getting started

## Install

It requires the following software on your system.

- [`cargo`](https://github.com/rust-lang/cargo), the Rust package manager
- [`clang`](https://clang.llvm.org/), the C compiler

After you install them, run the following command.

```
cargo install --git https://github.com/ein-lang/ein
```

Then, you should be able to run an `ein` command in your shell. Make sure that the `cargo`'s binary directory is included in your `PATH` environment variable.

```sh
ein --help
```

## Initializing a package

When you run the following command, you should see a `foo` directory under the current directory.

```sh
ein init foo
```

When you switch your directory to the `foo` directory, you should see a `Main.ein` source file and a `ein.json` package configuration file.

## Building a package

To build the package, run the following command in the `foo` directory.

```sh
ein build
```

Then, you will see an executable file named `foo` in the directory.

## For more information...

Now, you can start editing `*.ein` files and build your own application!

- To know more about the language, see [the language reference](/references/language/syntax.md).
