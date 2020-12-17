Feature: If
  Background:
    Given I successfully run `ein init command foo`
    And I cd to "foo"

  Scenario: Use if expressions
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system = if True then 0 else 1
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Use nested if expressions
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system =
      if False then
        1
      else if True then
			  0
			else
        1
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`
