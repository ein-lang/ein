Feature: Error
  Background:
    Given I successfully run `ein init library .`

  Scenario: Define an error value
    Given a file named "Foo.ein" with:
    """
    x : Error
    x = error 42
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use a let-error expression
    Given a file named "Foo.ein" with:
    """
    x : Number | Error
    x = error 42

    y : Number | Error
    y =
      let
        x ?= x
      in
        x + 1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use a let-error expression with multiple definitions
    Given a file named "Foo.ein" with:
    """
    x : Number | Error
    x = error 42

    y : Number | Error
    y =
      let
        v ?= x
        w ?= x
      in
        v + w
    """
    When I run `ein build`
    Then the exit status should be 0
