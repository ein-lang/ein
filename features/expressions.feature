Feature: Expressions
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
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use if expressions
    Given a file named "Main.ein" with:
    """
    main : Number -> Number
    main x = if true then 42 else 13
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use arithmetic operators
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
    And a file named "Main.ein" with:
    """
    main : Number -> Number
    main x = 0 + 1 * 3 - 4 / 2
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "1"
    And the exit status should be 0

  Scenario: Use an equality operator with numbers
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
    And a file named "Main.ein" with:
    """
    main : Number -> Number
    main x = if x == 0 then 13 else 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use an equality operator with unions
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
    And a file named "Main.ein" with:
    """
    a : Number | None
    a = 0

    b : Number | None
    b = none

    main : Number -> Number
    main x = if a == b then 13 else 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0
