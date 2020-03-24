Feature: Records
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

  Scenario: Define a record value
    Given a file named "Main.ein" with:
    """
    type Foo = ( foo : Number )

    foo : Foo
    foo = Foo ( foo = 42 )

    main : Number -> Number
    main x = 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0
