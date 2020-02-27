Feature: Types
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

  Scenario: Use None type
    Given a file named "Main.ein" with:
    """
    f : None -> Number
    f x = 42

    main : Number -> Number
    main x = f None
    """
    And I successfully run `ein build`
    When I run `sh -c ./command`
    Then stdout from "sh -c ./command" should contain exactly "42"
    And the exit status should be 0
