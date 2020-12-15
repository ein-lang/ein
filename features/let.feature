Feature: Let expressions
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

  Scenario: Use let-values expression
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system =
      let
        y : Number
        y = 0
      in
        y
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Use untyped let-values expression
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system =
      let
        y = 0
      in
        y
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Use nested let-values expression
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system =
      let
        y =
          let
            z = 0
          in
            z
      in
        y
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Use let-functions expression
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system =
      let
        f : Number -> Number
        f y = y
      in
        f 0
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Use untyped let-functions expression
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system =
      let
        f y = y
      in
        f 0
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`

  Scenario: Define multiple functions in a let-functions expression
    Given a file named "Main.ein" with:
    """
    main : System -> Number
    main system =
      let
        f y = y
        g z = f z
      in
        g 0
    """
    When I successfully run `ein build`
    Then I successfully run `sh -c ./foo`
