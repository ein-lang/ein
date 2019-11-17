Feature: Variables
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
  Scenario: Define a global variable
    Given a file named "src/Main.ein" with:
    """
    y : Number
    y = 42

    main : Number -> Number
    main x = y
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Define global variables
    Given a file named "src/Main.ein" with:
    """
    y : Number
    y = z

    z : Number
    z = 42

    main : Number -> Number
    main x = y
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use let-values expression
    Given a file named "src/Main.ein" with:
    """
    main : Number -> Number
    main x = (
      let
        y : Number
        y = x
      in
        y
    )
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use untyped let-values expression
    Given a file named "src/Main.ein" with:
    """
    main : Number -> Number
    main x = (
      let
        y = x
      in
        y
    )
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use nested let-values expression
    Given a file named "src/Main.ein" with:
    """
    main : Number -> Number
    main x = (
      let
        y = (
          let
            z = x
          in
            z
        )
      in
        y
    )
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use let-functions expression
    Given a file named "src/Main.ein" with:
    """
    main : Number -> Number
    main x = (
      let
        f : Number -> Number
        f y = y
      in
        f x
    )
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use untyped let-functions expression
    Given a file named "src/Main.ein" with:
    """
    main : Number -> Number
    main x = (
      let
        f y = y
      in
        f x
    )
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Define multiple functions in a let-functions expression
    Given a file named "src/Main.ein" with:
    """
    main : Number -> Number
    main x = (
      let
        f y = y
        g z = f z
      in
        g x
    )
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0
