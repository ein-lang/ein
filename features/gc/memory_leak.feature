Feature: Memory leak
  Background:
    Given I successfully run `ein init foo`
    And I cd to "foo"

  Scenario: Run an infinite loop
    Given a file named "Main.ein" with:
    """
    import "github.com/ein-lang/os/Os"

    main : Os.Os -> Number
    main os = main os
    """
    When I successfully run `ein build`
    Then I successfully run `check_memory_leak.sh ./foo`
