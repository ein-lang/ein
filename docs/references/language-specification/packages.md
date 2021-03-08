# Packages

- Each repository of some version control system (VCS) composes a package.
  - Currently, only [Git](https://git-scm.com/) is supported.
- Packages contain modules.
- Modules in packages of the `Library` type can be imported by other packages.

## Package names

Packages are referenced by host names and paths in their VCS's URLs. For example, a package of a Git repository at a URL of `https://github.com/foo/bar` is referenced as `github.com/foo/bar`.

To import modules in other packages, see [Modules](modules.md).

## Package configuration

- Each package has a configuration file named `ein.json` at its root directory.
- Packages have types of either `Command` or `Library`.

### Command

```json
{
  "target": {
    "type": "Command",
    "name": "foo",
    "systemPackage": {
      "name": "github.com/ein-lang/system",
      "version": "main"
    }
  },
  "dependencies": {
    "github.com/bar/baz": { "version": "main" }
  }
}
```

### Library

```json
{
  "target": {
    "type": "Library"
  },
  "dependencies": {
    "github.com/bar/baz": { "version": "main" }
  }
}
```
