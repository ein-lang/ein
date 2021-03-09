Feature: Error
  Background:
    Given I successfully run `ein init -l .`

  Scenario: Fail to build due to type check
    Given a file named "Main.ein" with:
    """
    f : Number
    f = 0

    x : Number
    x = f 0
    """
    When I run `ein build`
    Then stderr from "ein build" should contain "types not matched"
    And the exit status should not be 0

  Scenario: Fail to build due to duplicate names
    Given a file named "Main.ein" with:
    """
    a : Number
    a = 42

    a : Number
    a = 42
    """
    When I run `ein build`
    Then stderr from "ein build" should contain "duplicate names"
    And the exit status should not be 0
