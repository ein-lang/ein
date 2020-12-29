Feature: Subtyping
  Background:
    Given I successfully run `ein init command foo`
    And I cd to "foo"

  Scenario: Handle covariance and contravariance of functions
    Given a file named "Main.ein" with:
    """
    f : Number | None -> Number
    f x = 0

    g : Number -> Number | None
    g = f

    main : System -> Number
    main system =
      case x = g 0
        Number => 0
        None => 1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Handle covariance and contravariance of functions in lists
    Given a file named "Main.ein" with:
    """
    f : Number | None -> Number
    f x = 0

    xs : List (Number | None -> Number)
    xs = [ f ]

    ys : List (Number -> Number | None)
    ys = xs

    main : System -> Number
    main system =
      case ys
        [] => 1
        [ f, ...fs ] =>
          case x = f 0
            Number => 0
            None => 1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Let a function type subsume a union type.
    Given a file named "Main.ein" with:
    """
    f : Number | None -> Number
    f x = 0

    g : Number -> Number | None
    g x = 0

    xs : List (Number -> Number | None)
    xs = [ f, g ]

    main : System -> Number
    main system =
      case xs
        [] => 1
        [ f, ...fs ] =>
          case x = f 0
            Number => 0
            None => 1
    """
    When I run `ein build`
    Then the exit status should be 0
