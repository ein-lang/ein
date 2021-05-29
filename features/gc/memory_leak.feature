Feature: Memory leak
  Background:
    Given I successfully run `ein init foo`
    And I cd to "foo"

  Scenario: Run an infinite loop
    Given a file named "Main.ein" with:
    """
    import "github.com/ein-lang/os/Os"

    main : Os.Os -> Number
    main os = main os
    """
    When I successfully run `ein build`
    Then I successfully run `check_memory_leak.sh ./foo`

  Scenario: Run hello world
    Given a file named "Main.ein" with:
    """
    import "github.com/ein-lang/os/Os"

    main : Os.Os -> Number
    main os =
      let
        _ = Os.fdWrite os Os.stdout "Hello, world!\n"
      in
        main os
    """
    When I successfully run `ein build`
    Then I successfully run `check_memory_leak.sh ./foo`

  Scenario: Stringify a number
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
    "github.com/ein-lang/core": { "version": "HEAD" }
      }
    }
    """
    And a file named "Main.ein" with:
    """
    import "github.com/ein-lang/core/Number"
    import "github.com/ein-lang/os/Os"

    main : Os.Os -> Number
    main os =
      let
        _ = Number.string 42
      in
        main os
    """
    When I successfully run `ein build`
    Then I successfully run `check_memory_leak.sh ./foo`
