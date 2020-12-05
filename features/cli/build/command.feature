Feature: Command
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

  Scenario: Build a command
    Given a file named "Main.ein" with:
    """
    main : Number -> Number
    main x = x
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Build a command twice
    Given a file named "Main.ein" with:
    """
    main : Number -> Number
    main x = x
    """
    And I successfully run `ein build`
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Build a command in an inner directory
    Given a file named "Main.ein" with:
    """
    main : Number -> Number
    main x = x
    """
    And a directory named "Foo"
    And I cd to "Foo"
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Build a command with a dependency
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Command",
        "name": "foo"
      },
      "dependencies": {
        "github.com/ein-lang/sample-package": { "version": "master" }
      }
    }
    """
    And a file named "Main.ein" with:
    """
    import "github.com/ein-lang/sample-package/Foo"

    main : Number -> Number
    main = Foo.foo
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0
