Feature: Boolean
  Background:
    Given I successfully run `ein init -l .`

  Scenario: Use Boolean type
    Given a file named "Foo.ein" with:
    """
    x : Boolean
    x = True

    y : Boolean
    y = False
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use not function
    Given a file named "Foo.ein" with:
    """
    x : Number
    x = if not False then 0 else 1
    """
    When I run `ein build`
    Then the exit status should be 0
