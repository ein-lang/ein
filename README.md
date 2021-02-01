# Ein

[![GitHub Action](https://img.shields.io/github/workflow/status/ein-lang/ein/test?style=flat-square)](https://github.com/ein-lang/ein/actions)
[![Codecov](https://img.shields.io/codecov/c/github/ein-lang/ein.svg?style=flat-square)](https://codecov.io/gh/ein-lang/ein)
[![License](https://img.shields.io/github/license/ein-lang/ein.svg?style=flat-square)](LICENSE)

The functional programming language for scalable development.

> Ein is currently under heavy development. Please give us feedback creating new issues!

## Goals

Ein is designed for software developed by a large number of people and/or over a long period of time.

To make such development efficient, it's focused on:

- Simplicity
  - Its syntax and type system are minimal and easy to learn.
  - Its minimal language features keep codes maintainable.
- Portability (WIP)
  - Software written in Ein is easy to port to other platforms including different operating systems, web browsers and [WASI](https://wasi.dev/).
  - Libraries written in Ein can be used by other languages via FFI compatible with C ABI.

## Features

- Statically typed
- Immutable values
- Pure functions by default
- No runtime error

## Install

- It requires [`clang`](https://clang.llvm.org/) installed on your system.

```
cargo install --git https://github.com/ein-lang/ein
```

## Documentation

- [Guides](doc/guides.md)
- [Language specification](doc/language_specification.md)

## License

[MIT](LICENSE)
