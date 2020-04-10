Feature: Types
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

  Scenario: Use Boolean type
    Given a file named "Main.ein" with:
    """
    f : Boolean -> Boolean -> Number
    f x y = 42

    main : Number -> Number
    main x = f false true
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use None type
    Given a file named "Main.ein" with:
    """
    f : None -> Number
    f x = 42

    main : Number -> Number
    main x = f none
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0
