# Types

## Number

```
Number
```

### Literals

```
3.14
-42
```

## Boolean

```
Boolean
```

### Literals

```
False
True
```

## None

```
None
```

### Literals

```
None
```

## String

```
String
```

### Literals

```
"foo"
```

## Functions

```
a -> b
```

## Lists

```
[a]
```

### Literals

```
[ 1, 2, 3 ]
[ x, ...xs ]
```

## Records

```
type Person {
  name : String,
  age : Number,
}
```

### Operations

- Elements are private outside modules.

```
Person.name person
Person{ name = "foo", age = 42 }
Person{ ...person, name = "bar" }
```

## Enumerated types

```
type Foo
```

### Literals

```
Foo
```

## Unions

```
Foo | Bar | Baz
```

## Any

```
Any
```
