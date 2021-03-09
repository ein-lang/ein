Feature: If
  Background:
    Given I successfully run `ein init -l .`

  Scenario: Use if expressions
    Given a file named "Foo.ein" with:
    """
    x : Number
    x = if True then 0 else 1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use nested if expressions
    Given a file named "Foo.ein" with:
    """
    x : Number
    x =
      if False then
        1
      else if True then
        0
      else
        1
    """
    When I run `ein build`
    Then the exit status should be 0
