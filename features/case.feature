Feature: Case expressions
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

  Scenario: Use an argument of a union type
    Given a file named "Main.ein" with:
    """
    main : Number -> Number
    main x = case y = if True then 42 else None
      Number => y
      None => 13
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Bind a variable with a union type
    Given a file named "Main.ein" with:
    """
    x : Number | None
    x =
      case y = if True then 42 else None
        Number | None => y

    main : Number -> Number
    main x = x
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use an argument of any type
    Given a file named "Main.ein" with:
    """
    x : Any
    x = 42

    y : Any
    y =
      case z = x
        Number | None => z
        Any => z

    main : Number -> Number
    main x = x
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Distinguish different list types
    Given a file named "Main.ein" with:
    """
    y : List Any
    y = []

    z : List None
    z = []

    main : Number -> Number
    main x =
      case y = if True then y else z
        List None => 13
        List Any => 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0
