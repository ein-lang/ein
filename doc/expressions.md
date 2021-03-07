# Expressions

## Operators

### Arithmetic

```
1 + 1
1 - 1
1 * 1
1 / 1
```

### Comparison

```
1 == 1
1 /= 1
1 < 1
1 <= 1
1 > 1
1 >= 1
```

#### Generic equality

`==` and `/=` operators can be used for any types except functions and types which might include them.

```
"foo" == "bar"
Foo{ foo : 0 } == Foo{ foo : 1 }
42 /= None
```

### Boolean

```
True && True
True || True
```

## Function application

```
f x y z
```

## Conditionals

### `if` expression

```
if True then
  ...
else
  ...
```

### `case` expression

#### Lists

```
case xs
  [] => ...
  [ y, ...ys ] => ...
```

#### Types

```
case x = y
  Foo => ...
  Bar | Baz => ...
```
