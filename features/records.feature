Feature: Records
  Background:
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Command",
        "name": "foo"
      },
      "dependencies": {}
    }
    """

  Scenario: Define a record value
    Given a file named "Main.ein" with:
    """
    type Foo ( foo : Number )

    foo : Foo
    foo = Foo ( foo = 42 )

    main : Number -> Number
    main x = 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Access a record's element
    Given a file named "Main.ein" with:
    """
    type Foo ( foo : Number )

    main : Number -> Number
    main x = Foo.foo (Foo ( foo = 42 ))
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Update a record's element
    Given a file named "Main.ein" with:
    """
    type Foo ( foo : Number, bar : Number )

    foo : Foo
    foo = Foo ( foo = 13, bar = 13 )

    main : Number -> Number
    main x = Foo.foo (Foo ( ...foo, foo = 42 ))
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0

  Scenario: Define a recursive record value
    Given a file named "Main.ein" with:
    """
    type Foo ( foo : Foo )

    foo : Number -> Foo
    foo x = Foo ( foo = foo x )

    main : Number -> Number
    main x = 42
    """
    And I successfully run `ein build`
    When I run `sh -c ./foo`
    Then stdout from "sh -c ./foo" should contain exactly "42"
    And the exit status should be 0
