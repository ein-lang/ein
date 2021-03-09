Feature: Application
  Background:
    Given I successfully run `ein init foo`
    And I cd to "foo"

  Scenario: Build an application
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Build an application with a dependency
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Application",
        "name": "foo",
        "systemPackage": {
          "name": "github.com/ein-lang/os",
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
