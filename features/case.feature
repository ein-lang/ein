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
    main : System -> Number
    main system = case y = if True then 0 else None
      Number => y
      None => 1
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then the exit status should be 0

  Scenario: Bind a variable with a union type
    Given a file named "Main.ein" with:
    """
		x : Number | Boolean | None
		x = None

    main : System -> Number
    main system =
      case _ = x
        Number | None => 0
				Boolean => 1
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then the exit status should be 0

  Scenario: Use an argument of any type
    Given a file named "Main.ein" with:
    """
    x : Any
    x = 0

    main : System -> Number
    main system =
      case x = x
        Number => x
        Any => 1
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then the exit status should be 0

  Scenario: Distinguish different list types
    Given a file named "Main.ein" with:
    """
    y : List Any
    y = []

    z : List None
    z = []

    main : System -> Number
    main system =
      case y = if True then y else z
        List None => 1
        List Any => 0
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then the exit status should be 0
