Feature: Operators
  Background:
    Given I successfully run `ein init -l .`

  Scenario: Use arithmetic operators
    Given a file named "Foo.ein" with:
    """
    x : Number
    x = 0 + 1 * 1 - 1 / 1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use boolean operators
    Given a file named "Foo.ein" with:
    """
    x : Number
    x = if False || True && True then 0 else 1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use an equal operator with numbers
    Given a file named "Foo.ein" with:
    """
    x : Number
    x = if 0 == 1 then 1 else 0
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use an equal operator with records
    Given a file named "Foo.ein" with:
    """
    type Foo { foo : Number, bar : Boolean }

    x : Number
    x =
      if Foo{ foo = 42, bar = True } == Foo{ foo = 42, bar = True } then
        0
      else
        1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use an equal operator with unions
    Given a file named "Foo.ein" with:
    """
    a : Number | None
    a = 0

    b : Number | None
    b = None

    x : Number
    x = if a == b then 1 else 0
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use a not-equal operator
    Given a file named "Foo.ein" with:
    """
    x : Number
    x = if 1 /= 2 then 0 else 1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use a pipe operator
    Given a file named "Foo.ein" with:
    """
    f : Number -> Number
    f x = x * 2

    g : Number -> Number
    g x = x - 1

    x : Number
    x = 0.5 |> f |> g
    """
    When I run `ein build`
    Then the exit status should be 0
