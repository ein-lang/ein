Feature: Build
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

  Scenario: Build a library
    Given a file named "Foo.ein" with:
    """
    foo : Number -> Number
    foo x = x
    """
    When I run `ein build`
    Then the exit status should be 0
