Feature: Modules
  Background:
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

  Scenario: Import a module
    Given a file named "Main.ein" with:
    """
    import "./Foo"

    main : Number -> Number
    main x = x
    """
    And a file named "Foo.ein" with:
    """
    export { a }

    a : Number
    a = 42
    """
    When I successfully run `ein build`
    And I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0

  Scenario: Import a name in a module
    Given a file named "Main.ein" with:
    """
    import "./Foo"

    main : Number -> Number
    main x = Foo.a
    """
    And a file named "Foo.ein" with:
    """
    export { a }

    a : Number
    a = 42
    """
    When I successfully run `ein build`
    And I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0
