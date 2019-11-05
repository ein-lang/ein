Feature: Expressions
  Background:
    Given a directory named "src"
    And a file named "package.json" with:
    """
    {
      "name": "Package",
      "version": "1.0.0"
    }
    """

  Scenario: Apply a function of a let expression to arguments
    Given a file named "src/Main.sl" with:
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
    And I successfully run `sloth-build`
    When I run `sh -c ./Package`
    Then stdout from "sh -c ./Package" should contain exactly "42"
    And the exit status should be 0
