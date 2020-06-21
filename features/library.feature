Feature: Library
  Scenario: Build a library
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Library"
      },
      "dependencies": {}
    }
    """
    And a file named "Foo.ein" with:
    """
    export { foo }

    foo : Number -> Number
    foo x = x
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Build a library with a dependency
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Library"
      },
      "dependencies": {
        "github.com/ein-lang/sample-package": { "version": "master" }
      }
    }
    """
    And a file named "Bar.ein" with:
    """
    export { bar }

    import "github.com/ein-lang/sample-package/Foo"

    bar : Number -> Number
    bar = Foo.foo
    """
    When I run `ein build`
    Then the exit status should be 0
