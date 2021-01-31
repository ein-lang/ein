Feature: String
  Background:
    Given I successfully run `ein init library .`

  Scenario: Create an empty string
    Given a file named "Main.ein" with:
    """
    foo : String
    foo = ""
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Create a non-empty string
    Given a file named "Main.ein" with:
    """
    foo : String
    foo = "foo"
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Compare strings
    Given a file named "Main.ein" with:
    """
    x : Boolean
    x = "foo" == "foo" && "foo" /= "bar"
    """
    When I run `ein build`
    Then the exit status should be 0
