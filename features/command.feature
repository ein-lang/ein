Feature: Command
  Scenario: Build a command
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
    And a file named "Main.ein" with:
    """
    main : Number -> Number
    main x = 1 * 3 - 4 / 2
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "1"
    And the exit status should be 0

  Scenario: Build a command twice
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
    And a file named "Main.ein" with:
    """
    main : Number -> Number
    main x = 1 * 3 - 4 / 2
    """
    And I successfully run `ein build`
    When I run `ein build`
    Then the exit status should be 0
