Feature: Functions
  Scenario: Use an argument
    Given a file named "main.sl" with:
    """
    main : Number -> Number;
    main x = x;
    """
    And I successfully run `sloth main.sl`
    When I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "42"
    And the exit status should be 0
