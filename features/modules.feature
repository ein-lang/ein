Feature: Modules
  Background:
    Given a directory named "src"
    And a file named "package.json" with:
    """
    {
      "name": "package",
      "version": "1.0.0"
    }
    """

  Scenario: Import a module
    Given a file named "src/main.sl" with:
    """
    import package.foo

    main : Number -> Number
    main x = x
    """
    And a file named "src/foo.sl" with:
    """
    export { a }

    a : Number
    a = 42
    """
    When I successfully run `sloth-build`
    And I run `sh -c ./package`
    Then stdout from "sh -c ./package" should contain exactly "42"
    And the exit status should be 0

  Scenario: Import a name in a module
    Given a file named "src/main.sl" with:
    """
    import package.foo

    main : Number -> Number
    main x = foo.a
    """
    And a file named "src/foo.sl" with:
    """
    export { a }

    a : Number
    a = 42
    """
    When I successfully run `sloth-build`
    And I run `sh -c ./package`
    Then stdout from "sh -c ./package" should contain exactly "42"
    And the exit status should be 0
