Feature: Operators
  Background:
    Given I successfully run `ein init command foo`
    And I cd to "foo"

  Scenario: Use arithmetic operators
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system = 0 + 1 * 1 - 1 / 1
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Use boolean operators
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system = if False || True && True then 0 else 1
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Use an equal operator with numbers
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system = if 0 == 1 then 1 else 0
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Use an equal operator with records
    Given a file named "Main.ein" with:
    """
    type Foo { foo : Number, bar : Boolean }

    main : System -> Number
    main system =
      if Foo{ foo = 42, bar = True } == Foo{ foo = 42, bar = True } then
        0
      else
        1
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Use an equal operator with unions
    Given a file named "Main.ein" with:
    """
    a : Number | None
    a = 0

    b : Number | None
    b = None

    main : System -> Number
    main system = if a == b then 1 else 0
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Use a not-equal operator
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system = if 1 /= 2 then 0 else 1
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Use a pipe operator
    Given a file named "Main.ein" with:
    """
    f : Number -> Number
    f x = x * 2

    g : Number -> Number
    g x = x - 1

    main : System -> Number
    main system = 0.5 |> f |> g
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`
