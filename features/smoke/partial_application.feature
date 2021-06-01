Feature: Partial application
  Background:
    Given I successfully run `ein init foo`
    And I cd to "foo"

  Scenario: Apply a function of 1 argument with 1 argument
    Given a file named "Main.ein" with:
    """
    import "github.com/ein-lang/os/Os"

    f : Number -> Number
    f x = x

    main : Os.Os -> Number
    main os =
      let
        _ = f 42
      in
        main os
    """
    When I successfully run `ein build`
    Then I successfully run `check_memory_leak.sh ./foo`

  Scenario: Apply a function of 2 arguments with 1 and 1 arguments
    Given a file named "Main.ein" with:
    """
    import "github.com/ein-lang/os/Os"

    f : Number -> Boolean -> Number
    f x y = x

    main : Os.Os -> Number
    main os =
      let
        g = f 42
        _ = g True
      in
        main os
    """
    When I successfully run `ein build`
    Then I successfully run `check_memory_leak.sh ./foo`

  Scenario: Apply a function of 2 arguments with 2 arguments
    Given a file named "Main.ein" with:
    """
    import "github.com/ein-lang/os/Os"

    f : Number -> Boolean -> Number
    f x y = x

    main : Os.Os -> Number
    main os =
      let
        _ = f 42 True
      in
        main os
    """
    When I successfully run `ein build`
    Then I successfully run `check_memory_leak.sh ./foo`

  Scenario: Apply a function of 3 arguments with 1, 1 and 1 arguments
    Given a file named "Main.ein" with:
    """
    import "github.com/ein-lang/os/Os"

    f : Number -> Boolean -> String -> Number
    f x y z = x

    main : Os.Os -> Number
    main os =
      let
        g = f 42
        h = g True
        _ = h "foo"
      in
        main os
    """
    When I successfully run `ein build`
    Then I successfully run `check_memory_leak.sh ./foo`

  Scenario: Apply a function of 3 arguments with 1 and 2 arguments
    Given a file named "Main.ein" with:
    """
    import "github.com/ein-lang/os/Os"

    f : Number -> Boolean -> String -> Number
    f x y z = x

    main : Os.Os -> Number
    main os =
      let
        g = f 42
        h = g True "foo"
      in
        main os
    """
    When I successfully run `ein build`
    Then I successfully run `check_memory_leak.sh ./foo`

  Scenario: Apply a function of 3 arguments with 2 and 1 arguments
    Given a file named "Main.ein" with:
    """
    import "github.com/ein-lang/os/Os"

    f : Number -> Boolean -> String -> Number
    f x y z = x

    main : Os.Os -> Number
    main os =
      let
        g = f 42 True
        _ = g "foo"
      in
        main os
    """
    When I successfully run `ein build`
    Then I successfully run `check_memory_leak.sh ./foo`

  Scenario: Apply a function of 3 arguments with 3 arguments
    Given a file named "Main.ein" with:
    """
    import "github.com/ein-lang/os/Os"

    f : Number -> Boolean -> String -> Number
    f x y z = x

    main : Os.Os -> Number
    main os =
      let
        _ = f 42 True "foo"
      in
        main os
    """
    When I successfully run `ein build`
    Then I successfully run `check_memory_leak.sh ./foo`
