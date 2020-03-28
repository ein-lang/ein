Feature: Expressions
  Background:
    Given a file named "ein.json" with:
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
    Given a file named "Main.ein" with:
    """
    main : Number -> Number
    main x =
      (
        let
          f : Number -> Number
          f y = y
        in
          f
      )
      x
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use if expressions
    Given a file named "Main.ein" with:
    """
    main : Number -> Number
    main x = if true then 42 else 13
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use case expressions
    Given a file named "Main.ein" with:
    """
    main : Number -> Number
    main x =
      case x of
        42 -> x
        y -> y + 13
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0
