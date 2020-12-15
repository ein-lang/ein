Feature: Prelude functions and types
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

  Scenario: Use not function
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system = if not False then 0 else 1
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`
