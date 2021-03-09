Feature: List
  Background:
    Given I successfully run `ein init -l .`

  Scenario: Create an empty list
    Given a file named "Foo.ein" with:
    """
    foo : List Number
    foo = []
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Create a list with an element
    Given a file named "Foo.ein" with:
    """
    foo : List Number
    foo = [ 42 ]
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Prepend an element to a list
    Given a file named "Foo.ein" with:
    """
    foo : List Number -> List Number
    foo x = [ 42, ...x ]
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use a list case expression
    Given a file named "Foo.ein" with:
    """
    foo : Number
    foo =
      case [ 29, 13 ]
        [] => 13
        [ x, ...xs ] =>
          case xs
            [] => 13
            [ y, ...ys ] => x + y
    """
    When I run `ein build`
    Then the exit status should be 0
