# Syntax

## Statements

### Variable definition

```
x = ...
```

### Function definition

```
f x y = ...
```

### Record type definitions

See [Type system](type_system.md#records).

### Type alias

See [Type system](type_system.md#type-alias).

### Module import and export

See [Module system](module_system.md).

## Expressions

### Operators

#### Arithmetic

```
1 + 1
1 - 1
1 * 1
1 / 1
```

#### Comparison

```
1 == 1
1 /= 1
1 < 1
1 <= 1
1 > 1
1 >= 1
```

##### Generic equality

`==` and `/=` operators can be used for any types except functions and types which might include them.

```
"foo" == "bar"
Foo{ foo : 0 } == Foo{ foo : 1 }
42 /= None
```

#### Boolean

```
True && True
True || True
```

### Function application

```
f x
```

### Conditionals

#### `if` expression

```
if x then
  ...
else
  ...
```

#### `case` expression

##### Lists

```
case xs
  [] => ...
  [ y, ...ys ] => ...
```

##### Types

```
case x = y
  Foo => ...
  Bar | Baz => ...
```
