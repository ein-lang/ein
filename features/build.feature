Feature: Build
  Background:
    Given a directory named "src"
    And a file named "package.json" with:
    """
    {
      "name": "command",
      "version": "1.0.0"
    }
    """

  Scenario: Build an executable
    Given a file named "src/main.sl" with:
    """
    main : Number -> Number
    main x = 1 * 3 - 4 / 2
    """
    And I successfully run `sloth-build`
    When I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "1"
    And the exit status should be 0

  Scenario: Fail to build an executable
    Given a file named "src/main.sl" with:
    """
    f : Number
    f = 42

    main : Number -> Number
    main x = f x
    """
    And I run `sloth-build`
    Then stderr from "sloth-build" should contain "TypeInferenceError"
    And the exit status should not be 0
