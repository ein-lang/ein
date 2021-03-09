Feature: FFI
  Background:
    Given I successfully run `ein init -l .`

  Scenario: Export a foreign function
    Given a file named "Foo.ein" with:
    """
    export foreign { f }

    f : Number -> Number
    f x = x
    """
    When I run `ein build`
    Then the exit status should be 0
