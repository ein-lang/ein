# Syntax

## Statements

Variables and functions can be defined using the `:` sign to specify their types and the `=` sign to specify their values.

### Variable definition

```
x : Number
x = ...
```

### Function definition

```
f : Number -> Number -> Number
f x y = ...
```

### Record type definition

See [Records](types.md#records).

### Type alias

```
type Foo = ...
```

### Module import and export

See [Modules](modules.md).

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

##### Unions and `Any`

- Values of unions and `Any` types can be downcasted using the `case` expression.
- In each branch, the variable `x` is bound to different types of the branch.

```
case x = ...
  Foo => ...
  Bar | Baz => ...
```
