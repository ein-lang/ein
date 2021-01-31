Feature: Module
  Background:
    Given I successfully run `ein init library .`

  Scenario: Import a module
    Given a file named "Foo.ein" with:
    """
    export { foo }

    foo : Number
    foo = 0
    """
    And a file named "Bar.ein" with:
    """
    import "/Foo"
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Import a name in a module
    Given a file named "Foo.ein" with:
    """
    export { foo }

    foo : Number
    foo = 0
    """
    And a file named "Bar.ein" with:
    """
    import "/Foo"

    bar : Number
    bar = Foo.foo
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Allow diamond dependency
    Given a file named "Foo.ein" with:
    """
    export { foo }

    foo : Number
    foo = 42
    """
    And a file named "Bar.ein" with:
    """
    export { bar }

    import "/Foo"

    bar : Number
    bar = Foo.foo
    """
    And a file named "Baz.ein" with:
    """
    export { baz }

    import "/Foo"

    baz : Number
    baz = Foo.foo
    """
    And a file named "Blah.ein" with:
    """
    import "/Bar"
    import "/Foo"

    blah : Number
    blah = Foo.foo - Bar.bar
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Use mutually recursive types only one of which is exported
    Given a file named "Foo.ein" with:
    """
    export { Foo, foo }

    type Foo { bar : Bar | None }

    type Bar { foo : Foo | None }

    foo : Foo
    foo = Foo{ bar = None }
    """
    And a file named "Bar.ein" with:
    """
    import "/Foo"

    foo : Foo.Foo
    foo = Foo.foo
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Import a curried function
    Given a file named "Foo.ein" with:
    """
    export { f }

    f : Number -> Number
    f = g

    g : Number -> Number
    g x = x
    """
    And a file named "Bar.ein" with:
    """
    import "/Foo"

    bar : Number
    bar = Foo.f 0
    """
    When I run `ein build`
    Then the exit status should be 0
