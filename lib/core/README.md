# Core language library

This is a library of the core language. It consists of the following components.

- AST
- Compiler
- Types

## Design

- Correct AST's always succeed to be compiled.
- Wrong AST's always fail to be compiled.
- Everything is typed already.
- Top-level definitions are unordered.
- The language is agnostic to module systems in the higher-level languages.
