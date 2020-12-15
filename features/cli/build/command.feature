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
    main : System -> Number
    main system = 0
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Build a command twice
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system = 0
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Build a command in an inner directory
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system = 0
    """
    And a directory named "Foo"
    And I cd to "Foo"
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

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

    main : System -> Number
    main system = Foo.foo 0
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`
