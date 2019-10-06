Feature: Build
  Scenario: Build an executable
    Given a file named "main.sl" with:
    """
    main : Number -> Number
    main x = 1 * 3 - 4 / 2
    """
    And I successfully run `builder`
    When I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "1"
    And the exit status should be 0

  Scenario: Fail to build an executable
    Given a file named "main.sl" with:
    """
    f : Number
    f = 42

    main : Number -> Number
    main x = f x
    """
    And I run `builder`
    Then stderr from "builder" should contain "TypeInferenceError"
    And the exit status should not be 0
