Feature: Records
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

  Scenario: Create an empty list
    Given a file named "Main.ein" with:
    """
    foo : List Number
    foo = []

    main : Number -> Number
    main x = 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Create a list with an element
    Given a file named "Main.ein" with:
    """
    foo : List Number
    foo = [ 42 ]

    main : Number -> Number
    main x = 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Prepend an element to a list
    Given a file named "Main.ein" with:
    """
    foo : List Number -> List Number
    foo x = [ 42, ...x ]

    main : Number -> Number
    main x = 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0
