Feature: Types
  Scenario: Use recursive types
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Command",
        "name": "command"
      },
      "dependencies": {}
    }
    """
    And a file named "Main.ein" with:
    """
    type Foo = Foo -> Number

    f : Foo
    f g = 42

    main : Number -> Number
    main x = f f
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0
