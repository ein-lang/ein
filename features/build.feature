Feature: Build
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

  Scenario: Build an executable
    Given a file named "src/Main.ein" with:
    """
    main : Number -> Number
    main x = 1 * 3 - 4 / 2
    """
    And I successfully run `ein build`
    When I run `sh -c ./package`
    Then stdout from "sh -c ./package" should contain exactly "1"
    And the exit status should be 0

  Scenario: Fail to build an executable
    Given a file named "src/Main.ein" with:
    """
    f : Number
    f = 42

    main : Number -> Number
    main x = f x
    """
    And I run `ein build`
    Then stderr from "ein build" should contain "TypeInferenceError"
    And the exit status should not be 0
