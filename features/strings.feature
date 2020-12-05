Feature: Strings
  Background:
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Command",
        "name": "foo"
      },
      "dependencies": {}
    }
    """

  Scenario: Create an empty string
    Given a file named "Main.ein" with:
    """
    foo : String
    foo = ""

    main : Number -> Number
    main x = 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Create a non-empty string
    Given a file named "Main.ein" with:
    """
    foo : String
    foo = "foo"

    main : Number -> Number
    main x = 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Compare strings
    Given a file named "Main.ein" with:
    """
    main : Number -> Number
    main x =
      if "foo" == "foo" && "foo" /= "bar"
      then 42
      else 13
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0
