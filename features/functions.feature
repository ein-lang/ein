Feature: Functions
  Background:
    Given I successfully run `ein init library .`

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

  Scenario: Handle covariance and contravariance
    Given a file named "Main.ein" with:
    """
    f : Number | None -> Number
    f x = 42

    g : (Number -> Number | None) -> Number
    g h = let x = h 0 in 0

    a : Number
    a = g f
    """
    When I run `ein build`
    Then the exit status should be 0
