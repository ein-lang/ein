Feature: Function
  Background:
    Given I successfully run `ein init -l .`

  Scenario: Use an argument
    Given a file named "Main.ein" with:
    """
    f : Number -> Number
    f x = x
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Apply a function to arguments
    Given a file named "Main.ein" with:
    """
    f : Number -> Number
    f x = x

    a : Number
    a = f 0
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use multiple arguments
    Given a file named "Main.ein" with:
    """
    f : Number -> Number -> Number
    f x y = x

    a : Number
    a = f 0 1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Define a function with an omitted argument
    Given a file named "Main.ein" with:
    """
    f : Number -> Number
    f x = x

    g : Number -> Number
    g = f
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Define a function with one of its arguments omitted
    Given a file named "Main.ein" with:
    """
    f : Number -> Number -> Number
    f x =
      let
        g y = x
      in
        g
    """
    When I run `ein build`
    Then the exit status should be 0
