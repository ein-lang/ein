Feature: Prelude functions and types
  Background:
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Command",
        "name": "foo"
      },
      "dependencies": {
        "github.com/ein-lang/std": { "version": "main" }
      }
    }
    """

  Scenario: Use not function
    Given a file named "Main.ein" with:
    """
    import "github.com/ein-lang/std/Number"

    main : System -> Number
    main system =
      let
        _ = fdWrite system stdout (Number.string (if not False then 42 else 13))
      in
        0
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0
