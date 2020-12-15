Feature: Records
  Background:
    Given a file named "ein.json" with:
    """
    {
      "target": {
        "type": "Library"
      },
      "dependencies": {}
    }
    """

  Scenario: Define a record value
    Given a file named "Main.ein" with:
    """
    type Foo { foo : Number }

    foo : Foo
    foo = Foo{ foo = 42 }
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Access a record's element
    Given a file named "Main.ein" with:
    """
    type Foo { foo : Number }

    x : Number
    x = Foo.foo Foo{ foo = 42 }
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Update a record's element
    Given a file named "Main.ein" with:
    """
    type Foo { foo : Number, bar : Number }

    foo : Foo
    foo = Foo{ foo = 13, bar = 13 }

    bar : Foo
    bar = Foo{ ...foo, foo = 42 }
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Define a recursive record value
    Given a file named "Main.ein" with:
    """
    type Foo { foo : Foo }

    foo : Foo -> Foo
    foo x = Foo{ foo = x }
    """
    When I run `ein build`
    Then the exit status should be 0

  Scenario: Define a record with no member
    Given a file named "Main.ein" with:
    """
    type Foo

    foo : Foo
    foo = Foo
    """
    When I run `ein build`
    Then the exit status should be 0
