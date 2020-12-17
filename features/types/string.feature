Feature: String
  Background:
    Given I successfully run `ein init command foo`
    And I cd to "foo"

  Scenario: Create an empty string
    Given a file named "Main.ein" with:
    """
    foo : String
    foo = ""

    main : System -> Number
    main system = 0
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then the exit status should be 0

  Scenario: Create a non-empty string
    Given a file named "Main.ein" with:
    """
    foo : String
    foo = "foo"

    main : System -> Number
    main system = 0
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then the exit status should be 0

  Scenario: Compare strings
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system =
      if "foo" == "foo" && "foo" /= "bar"
      then 0
      else 1
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then the exit status should be 0
