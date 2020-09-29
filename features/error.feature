Feature: Error
  Background:
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Command",
        "name": "foo"
      },
      "dependencies": {}
    }
    """

  Scenario: Define an error value
    Given a file named "Main.ein" with:
    """
    x : Error
    x = error 42

    main : Number -> Number
    main x = 42
    """
    When I run `ein build`
    Then the exit status should be 0
