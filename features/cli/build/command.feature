Feature: Command
  Background:
    Given I successfully run `ein init command foo`
    And I cd to "foo"

  Scenario: Build a command
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Build a command with a dependency
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Command",
        "name": "foo",
        "systemPackage": {
          "name": "github.com/ein-lang/system",
          "version": "main"
        }
      },
      "dependencies": {
        "github.com/ein-lang/sample-package": { "version": "HEAD" }
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
