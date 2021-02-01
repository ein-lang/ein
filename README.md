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
  - Its syntax and type system are simple and easy to learn.
  - Its minimal language features keep codes maintainable.
- Portability (WIP)
  - Software written in the language can be easily
    - Ported to different platforms including operating systems, web browsers and [WASI](https://wasi.dev/).
    - Reused by other languages via FFI.

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

## Roadmap

- [ ] Automatic reference counting
- [ ] CPS transformation
- [ ] Asynchronous operations
- [ ] Parallel and concurrent computation

## License

[MIT](LICENSE)
