Feature: Variables
  Scenario: Define a global variable
    Given a file named "main.sl" with:
    """
    y : Number
    y = 42

    main : Number -> Number
    main x = y
    """
    And I successfully run `sloth main.sl`
    When I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use let-values expression
    Given a file named "main.sl" with:
    """
    main : Number -> Number
    main x = (
      let
        y : Number
        y = x
      in
        y
    )
    """
    And I successfully run `sloth main.sl`
    When I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "42"
    And the exit status should be 0
