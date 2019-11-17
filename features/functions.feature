Feature: Functions
  Background:
    Given a directory named "src"
    And a file named "ein.json" with:
    """
    {
      "name": "package",
      "version": "1.0.0",
      "target": {
        "type": "Command",
        "name": "command"
      },
      "dependencies": {}
    }
    """

  Scenario: Use an argument
    Given a file named "src/Main.ein" with:
    """
    main : Number -> Number
    main x = x
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Apply a function to arguments
    Given a file named "src/Main.ein" with:
    """
    f : Number -> Number
    f x = x

    main : Number -> Number
    main x = f x
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use multiple arguments
    Given a file named "src/Main.ein" with:
    """
    f : Number -> Number -> Number
    f x y = x

    main : Number -> Number
    main x = f x 13
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Define a function with an omitted argument
    Given a file named "src/Main.ein" with:
    """
    f : Number -> Number
    f x = x

    main : Number -> Number
    main = f
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Define a function with one of its arguments omitted
    Given a file named "src/Main.ein" with:
    """
    f : Number -> Number -> Number
    f x = (
      let
        g y = x
      in
        g
    )

    main : Number -> Number
    main x = f x 13
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0
