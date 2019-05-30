Feature: Build
  Scenario: Build an executable
    Given a file named "main.sl" with:
    """
    (+ 1 2)
    """
    And I successfully run `sloth main.sl`
    When I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "3"
    And the exit status should be 0
