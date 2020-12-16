# Expressions

## Function applications

```
f x y z
```

## If expressions

```
if True then 42 else 13
```

## Case expressions

### Lists

```
case xs
  [] => ...
  [ y, ...ys ] => ...
```

### Types

```
case x = y
  Foo => ...
  Bar | Baz => ...
```

## Operators

### Number

#### Arithmetics

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
1 => 1
```

### Boolean

```
True && True
True || True
```

### Generic equality

```
"foo" == "bar"
Foo{ foo : 0 } == Foo{ foo : 1 }
```
