Feature: Expressions
  Background:
    Given a directory named "src"
    And a file named "ein.json" with:
    """
    {
      "name": "package",
      "version": "1.0.0",
      "target": { "type": "Binary" },
      "dependencies": []
    }
    """

  Scenario: Apply a function of a let expression to arguments
    Given a file named "src/Main.ein" with:
    """
    main : Number -> Number
    main x = (
      (
        let
          f : Number -> Number
          f y = y
        in
          f
      )
      x
    )
    """
    And I successfully run `ein build`
    When I run `sh -c ./package`
    Then stdout from "sh -c ./package" should contain exactly "42"
    And the exit status should be 0
