# Getting started

## Install

### Requirements

Ein requires the following software on your system.

- [`cargo`](https://github.com/rust-lang/cargo), the Rust package manager
- [`git`](https://git-scm.com/), the version control system
- [`clang`](https://clang.llvm.org/), the C compiler
- [LLVM](https://llvm.org), the compiler infrastructure
  - Both the library and tools

#### On Ubuntu

Run the following commands in your terminal to install the required software.
Note that we need to install LLVM from the external repository to get the specific version of it.

```sh
sudo apt install cargo git
curl -fsSL https://apt.llvm.org/llvm.sh | sudo bash -s 11
```

#### On macOS

To install `clang` and `llc`, install Xcode from the App Store.
Also, install the `cargo` and `git` commands via [Homebrew](https://brew.sh/) by running the following command in your terminal.

```sh
brew install git rust
```

### Installing the `ein` command

Run the following command in your terminal.

```sh
cargo install --git https://github.com/ein-lang/ein --branch main
```

Then, you should be able to run an `ein` command in your shell. Make sure that the `cargo`'s binary directory is included in your `PATH` environment variable.

```sh
ein --help
```

## Initializing a package

To initialize your first package, run the following command.

```sh
ein init foo
```

Then, you should see a `foo` directory under your current directory. When you switch to the `foo` directory, you should see a `Main.ein` source file and a `ein.json` package configuration file there.

## Building a package

To build the package, run the following command in the `foo` directory.

```sh
ein build
```

Then, you will see an executable file named `foo` in the directory. Run the command to see your first "Hello, world!"

```sh
./foo
```

## For more information...

Now, you can start editing `*.ein` files and build your own application!

- To know more about the language, see [the language reference](/references/language/syntax.md).
