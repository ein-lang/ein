Feature: Functions
  Scenario: Use an argument
    Given a file named "main.sl" with:
    """
    main : Number -> Number
    main x = x
    """
    And I successfully run `sloth-build`
    When I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "42"
    And the exit status should be 0

  Scenario: Apply a function to arguments
    Given a file named "main.sl" with:
    """
    f : Number -> Number
    f x = x

    main : Number -> Number
    main x = f x
    """
    And I successfully run `sloth-build`
    When I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "42"
    And the exit status should be 0

  Scenario: Use multiple arguments
    Given a file named "main.sl" with:
    """
    f : Number -> Number -> Number
    f x y = x

    main : Number -> Number
    main x = f x 13
    """
    And I successfully run `sloth-build`
    When I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "42"
    And the exit status should be 0

  Scenario: Define a function with an omitted argument
    Given a file named "main.sl" with:
    """
    f : Number -> Number
    f x = x

    main : Number -> Number
    main = f
    """
    And I successfully run `sloth-build`
    When I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "42"
    And the exit status should be 0

  Scenario: Define a function with one of its arguments omitted
    Given a file named "main.sl" with:
    """
    f : Number -> Number -> Number
    f x = (
      let
        g y = x
      in
        g
    )

    main : Number -> Number
    main x = f x 13
    """
    And I successfully run `sloth-build`
    When I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "42"
    And the exit status should be 0
