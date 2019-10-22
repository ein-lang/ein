Feature: Expressions
  Scenario: Apply a function of a let expression to arguments
    Given a file named "main.sl" with:
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
    When I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "42"
    And the exit status should be 0
