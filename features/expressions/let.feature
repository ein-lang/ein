Feature: Let expressions
  Background:
    Given I successfully run `ein init library .`

  Scenario: Use let-values expression
    Given a file named "Foo.ein" with:
    """
    x : Number
    x =
      let
        y : Number
        y = 0
      in
        y
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use untyped let-values expression
    Given a file named "Foo.ein" with:
    """
    x : Number
    x =
      let
        y = 0
      in
        y
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use nested let-values expression
    Given a file named "Foo.ein" with:
    """
    x : Number
    x =
      let
        y =
          let
            z = 0
          in
            z
      in
        y
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use let-functions expression
    Given a file named "Foo.ein" with:
    """
    x : Number
    x =
      let
        f : Number -> Number
        f y = y
      in
        f 0
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use untyped let-functions expression
    Given a file named "Foo.ein" with:
    """
    x : Number
    x =
      let
        f y = y
      in
        f 0
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Define multiple functions in a let-functions expression
    Given a file named "Foo.ein" with:
    """
    x : Number
    x =
      let
        f y = y
        g z = f z
      in
        g 0
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Apply a function of a let expression to arguments
    Given a file named "Foo.ein" with:
    """
    x : Number
    x =
      (
        let
          f : Number -> Number
          f y = y
        in
          f
      )
      0
    """
    When I run `ein build`
    Then the exit status should be 0
