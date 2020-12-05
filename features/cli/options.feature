Feature: Options
  Scenario: Show help
		When I run `ein --help`
    Then the exit status should be 0
    And stdout from "ein --help" should contain "USAGE"

  Scenario: Show version
		When I run `ein --version`
    Then the exit status should be 0
