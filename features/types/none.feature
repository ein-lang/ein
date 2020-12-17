Feature: None
  Background:
    Given I successfully run `ein init library .`

  Scenario: Use None type
    Given a file named "Foo.ein" with:
    """
    x : None
    x = None
    """
    When I run `ein build`
    Then the exit status should be 0
