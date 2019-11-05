Feature: Modules
  Background:
    Given a directory named "src"
    And a file named "package.json" with:
    """
    {
      "name": "Package",
      "version": "1.0.0"
    }
    """

  Scenario: Import a module
    Given a file named "src/Main.sl" with:
    """
    import Package.Foo

    main : Number -> Number
    main x = x
    """
    And a file named "src/Foo.sl" with:
    """
    export { a }

    a : Number
    a = 42
    """
    When I successfully run `sloth-build`
    And I run `sh -c ./Package`
    Then stdout from "sh -c ./Package" should contain exactly "42"
    And the exit status should be 0

  Scenario: Import a name in a module
    Given a file named "src/Main.sl" with:
    """
    import Package.Foo

    main : Number -> Number
    main x = Foo.a
    """
    And a file named "src/Foo.sl" with:
    """
    export { a }

    a : Number
    a = 42
    """
    When I successfully run `sloth-build`
    And I run `sh -c ./Package`
    Then stdout from "sh -c ./Package" should contain exactly "42"
    And the exit status should be 0
