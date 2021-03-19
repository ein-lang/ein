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
      "application": {
        "name": "foo",
        "system": {
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
    import "github.com/ein-lang/os/Os"
    import "github.com/ein-lang/sample-package/Foo"

    main : Os.Os -> Number
    main os = Foo.foo 0
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Build an application of hello world
    Given a file named "ein.json" with:
    """
    {
      "application": {
        "name": "foo",
        "system": {
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
    import "github.com/ein-lang/os/Os"
    import "github.com/ein-lang/sample-package/Foo"

    main : Os.Os -> Number
    main os =
      let
        _ = Os.fdWrite os Os.stdout "Hello, world!"
      in
        0
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`
    And stdout from "sh -c ./foo" should contain "Hello, world!"
