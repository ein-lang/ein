Feature: Subtyping
  Background:
    Given I successfully run `ein init -l .`

  Scenario: Handle covariance and contravariance of functions
    Given a file named "Main.ein" with:
    """
    f : Number | None -> Number
    f x = 0

    g : Number -> Number | None
    g = f

    x : Number
    x =
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

    x : Number
    x =
      case ys
        [] => 1
        [ f, ...fs ] =>
          case x = f 0
            Number => 0
            None => 1
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Let a function type subsume a union type in a list expression
    Given a file named "Main.ein" with:
    """
    f : Number | None -> Number
    f x = 0

    g : Number -> Number | None
    g x = 0

    xs : List (Number -> Number | None)
    xs = [ f, g ]

    x : Number
    x =
      case xs
        [] => 1
        [ f, ...fs ] =>
          case x = f 0
            Number => 0
            None => 1
    """
    When I run `ein build`
    Then the exit status should be 0
