Feature: Library
  Background:
    Given I successfully run `ein init library .`

  Scenario: Build a library
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
        "github.com/ein-lang/sample-package": { "version": "HEAD" }
      }
    }
    """
    And a file named "Foo.ein" with:
    """
    export { bar }

    import "github.com/ein-lang/sample-package/Foo"

    bar : Number -> Number
    bar = Foo.foo
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Build a library twice
    When I run `ein build`
    And I run `ein build`
    Then the exit status should be 0

  Scenario: Build a library in an inner directory
    Given a directory named "Foo"
    And I cd to "Foo"
    When I run `ein build`
    Then the exit status should be 0
