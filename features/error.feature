Feature: Error
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
