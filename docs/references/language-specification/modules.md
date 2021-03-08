# Modules

In Ein, each source file composes a module.

## `export` statement

The `export` statement exports variables, functions and types outside the module.

```
export { foo, Bar }
```

## `import` statement

The `import` statement imports variables, functions and types from other modules.

```
import "github.com/ein-lang/foo/Foo"
```

Then, you can access members of the `Foo` module with its prefix.

```
type Bar = Foo.Foo

bar x = Foo.foo x
```

### Module names

#### Modules in the same package

Modules in the same package are referenced by their paths relative to their package root directory.

For example, a module of a file `<package directory>/Foo/Bar.ein` is imported as below.

```
import "/Foo/Bar"
```

#### Modules in other packages

Modules in other packages are referenced by their package names and module paths.

For example, a module of a file `<package directory>/Foo/Bar.ein` in a package `github.com/foo/bar` is imported as below.

```
import "github.com/foo/bar/Foo/Bar"
```

### Custom prefixes

> WIP

Imported modules can have different prefixes.

```
import Bar "github.com/ein-lang/foo/Foo"
```
