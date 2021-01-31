Feature: Case expressions
  Background:
    Given I successfully run `ein init library .`

  Scenario: Use an argument of a union type
    Given a file named "Foo.ein" with:
    """
    x : Number
    x = case y = if True then 0 else None
      Number => y
      None => 1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Bind a variable with a union type
    Given a file named "Foo.ein" with:
    """
    x : Number | Boolean | None
    x = None

    y : Number
    y =
      case x = x
        Number | None => 0
        Boolean => 1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use an argument of any type
    Given a file named "Foo.ein" with:
    """
    x : Any
    x = 0

    y : Number
    y =
      case x = x
        Number => x
        Any => 1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Distinguish different list types
    Given a file named "Foo.ein" with:
    """
    y : List Any
    y = []

    z : List None
    z = []

    x : Number
    x =
      case y = if True then y else z
        List None => 1
        List Any => 0
    """
    When I run `ein build`
    Then the exit status should be 0
