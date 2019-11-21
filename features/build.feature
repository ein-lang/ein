Feature: Build
  Background:
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Command",
        "name": "command"
      },
      "dependencies": {}
    }
    """

  Scenario: Build a command
    Given a file named "Main.ein" with:
    """
    main : Number -> Number
    main x = 1 * 3 - 4 / 2
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "1"
    And the exit status should be 0

  Scenario: Fail to build a command
    Given a file named "Main.ein" with:
    """
    f : Number
    f = 42

    main : Number -> Number
    main x = f x
    """
    And I run `ein build`
    Then stderr from "ein build" should contain "TypeInferenceError"
    And the exit status should not be 0
