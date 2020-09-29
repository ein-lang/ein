Feature: Compile error
  Background:
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Command",
        "name": "foo"
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
    Then stderr from "ein build" should contain "types not matched"
    And the exit status should not be 0
