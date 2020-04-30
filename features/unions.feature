Feature: Union types
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

  Scenario: Define a union value
    Given a file named "Main.ein" with:
    """
    foo : Boolean | None
    foo = true

    main : Number -> Number
    main x = 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Define a union type
    Given a file named "Main.ein" with:
    """
    type Foo = Boolean | None

    foo : Foo
    foo = true

    main : Number -> Number
    main x = 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use a case expression
    Given a file named "Main.ein" with:
    """
    main : Number -> Number
    main x = case y = if true then 42 else none
      Number => y
      None => 13
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0
