Feature: Union types
  Background:
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Library"
      },
      "dependencies": {}
    }
    """

  Scenario: Define a union value
    Given a file named "Foo.ein" with:
    """
    foo : Number | None
    foo = 42
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Define a union type
    Given a file named "Foo.ein" with:
    """
    type Foo = Number | None

    foo : Foo
    foo = 42
    """
    When I run `ein build`
    Then the exit status should be 0
