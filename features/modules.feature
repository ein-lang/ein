Feature: Modules
  Background:
    Given I successfully run `ein init command foo`
    And I cd to "foo"

  Scenario: Import a module
    Given a file named "Main.ein" with:
    """
    import "/Foo"

    main : System -> Number
    main system = 0
    """
    And a file named "Foo.ein" with:
    """
    export { foo }

    foo : Number
    foo = 0
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Import a name in a module
    Given a file named "Main.ein" with:
    """
    import "/Foo"

    main : System -> Number
    main system = Foo.foo
    """
    And a file named "Foo.ein" with:
    """
    export { foo }

    foo : Number
    foo = 0
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Allow diamond dependency
    Given a file named "Main.ein" with:
    """
    import "/Bar"
    import "/Foo"

    main : System -> Number
    main system = Foo.foo - Bar.bar
    """
    And a file named "Foo.ein" with:
    """
    export { foo }

    import "/Baz"

    foo : Number
    foo = Baz.baz
    """
    And a file named "Bar.ein" with:
    """
    export { bar }

    import "/Baz"

    bar : Number
    bar = Baz.baz
    """
    And a file named "Baz.ein" with:
    """
    export { baz }

    baz : Number
    baz = 42
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Use mutually recursive types only one of which is exported
    Given a file named "Main.ein" with:
    """
    import "/Foo"

    foo : Foo.Foo
    foo = Foo.foo

    main : System -> Number
    main system = 0
    """
    And a file named "Foo.ein" with:
    """
    export { Foo, foo }

    type Foo { bar : Bar | None }

    type Bar { foo : Foo | None }

    foo : Foo
    foo = Foo{ bar = None }
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Import a curried function
    Given a file named "Main.ein" with:
    """
    import "/Foo"

    main : System -> Number
    main system = Foo.f 0
    """
    And a file named "Foo.ein" with:
    """
    export { f }

    f : Number -> Number
    f = g

    g : Number -> Number
    g x = x
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`
