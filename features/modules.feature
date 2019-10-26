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

  Scenario: Build a package with multiple modules
    Given a file named "src/main.sl" with:
    """
    import package.foo

    main : Number -> Number
    main x = f x
    """
    And a file named "src/foo.sl" with:
    """
    export { f }

    f : Number -> Number
    f x = x
    """
    When I successfully run `sloth-build`
    And I run `sh -c ./a.out`
    Then stdout from "sh -c ./a.out" should contain exactly "42"
    And the exit status should be 0
