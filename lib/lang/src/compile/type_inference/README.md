# Type inference

## Overview

- Generic types (functions and lists) are covariant or contravariant to their element types.
- Union types subsume their member types and union types including subsets of them.
- Generic types can subsume union types.

## Rules

- `a`, `b`, ... are non-union and non-generic types.
- `x`, `y`, ... are any types.
- All union types are canonicalized.

```
a <: a
z <: x & y <: v => x <- y <: z <- v
x <: y => [x] <: [y]
x <: x | y
x <: y => z | x <: z | y
x <: a & y <: a => x | y <: a
```
