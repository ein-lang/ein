Feature: Types
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

  Scenario: Use Boolean type
    Given a file named "Foo.ein" with:
    """
    x : Boolean
    x = True

    y : Boolean
    y = False
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use None type
    Given a file named "Foo.ein" with:
    """
    x : None
    x = None
    """
    When I run `ein build`
    Then the exit status should be 0
