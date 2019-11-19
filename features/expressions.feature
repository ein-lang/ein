Feature: Expressions
  Background:
    Given a directory named "src"
    And a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Command",
        "name": "command"
      },
      "dependencies": {}
    }
    """

  Scenario: Apply a function of a let expression to arguments
    Given a file named "src/Main.ein" with:
    """
    main : Number -> Number
    main x = (
      (
        let
          f : Number -> Number
          f y = y
        in
          f
      )
      x
    )
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0
