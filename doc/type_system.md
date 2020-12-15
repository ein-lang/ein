# Type system

## Types

### Number

```
Number
```

#### Literals

```
3.14
-42
```

### Boolean

```
Boolean
```

#### Literals

```
False
True
```

### None

```
None
```

#### Literals

```
None
```

### String

```
String
```

#### Literals

```
"foo"
```

### Functions

```
a -> b
```

### Lists

```
[a]
```

#### Literals

```
[ 1, 2, 3 ]
[ 42, ...list ]
```

### Records

```
type Person {
  name : String,
  age : Number,
}
```

#### Operations

- Elements are private outside modules.

```
Person.name person
Person{ name = "foo", age = 42 }
Person{ ...person, name = "bar" }
```

### Enumerated types

```
type Foo
```

#### Literals

```
Foo
```

### Unions

```
Foo | Bar | Baz
```

### Any

```
Any
```

## Expressions

### If expressions

```
if True then 42 else 13
```

### Case expressions

#### Lists

```
case xs
  [] => ...
  [ x, ...xs ] => ...
```

#### Types

```
case x = expression
  Person => ...
  Number => ...
  Boolean | None => ...
```

## Statements

### Type alias

```
type Foo = Bar
```
