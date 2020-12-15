# Feature: Functions
#   Background:
#     Given a file named "ein.json" with:
#     """
#     {
#       "target": {
#         "type": "Command",
#         "name": "foo"
#       },
#       "dependencies": {}
#     }
#     """

#   Scenario: Use an argument
#     Given a file named "Main.ein" with:
#     """
#     main : System -> Number
#     main system = x
#     """
#     And I successfully run `ein build`
#     When I run `sh -c ./foo`
#     Then stdout from "sh -c ./foo" should contain exactly "42"
#     And the exit status should be 0

#   Scenario: Apply a function to arguments
#     Given a file named "Main.ein" with:
#     """
#     f : Number -> Number
#     f x = x

#     main : System -> Number
#     main system = f x
#     """
#     And I successfully run `ein build`
#     When I run `sh -c ./foo`
#     Then stdout from "sh -c ./foo" should contain exactly "42"
#     And the exit status should be 0

#   Scenario: Use multiple arguments
#     Given a file named "Main.ein" with:
#     """
#     f : Number -> Number -> Number
#     f x y = x

#     main : System -> Number
#     main system = f x 13
#     """
#     And I successfully run `ein build`
#     When I run `sh -c ./foo`
#     Then stdout from "sh -c ./foo" should contain exactly "42"
#     And the exit status should be 0

#   Scenario: Define a function with an omitted argument
#     Given a file named "Main.ein" with:
#     """
#     f : Number -> Number
#     f x = x

#     main : System -> Number
#     main = f
#     """
#     And I successfully run `ein build`
#     When I run `sh -c ./foo`
#     Then stdout from "sh -c ./foo" should contain exactly "42"
#     And the exit status should be 0

#   Scenario: Define a function with one of its arguments omitted
#     Given a file named "Main.ein" with:
#     """
#     f : Number -> Number -> Number
#     f x =
#       let
#         g y = x
#       in
#         g

#     main : System -> Number
#     main system = f x 13
#     """
#     And I successfully run `ein build`
#     When I run `sh -c ./foo`
#     Then stdout from "sh -c ./foo" should contain exactly "42"
#     And the exit status should be 0

#   Scenario: Handle covariance and contravariance
#     Given a file named "Main.ein" with:
#     """
#     f : Number | None -> Number
#     f x = 42

#     g : (Number -> Number | None) -> Number
#     g h = let x = h 42 in 42

#     main : System -> Number
#     main system = g f
#     """
#     And I successfully run `ein build`
#     When I run `sh -c ./foo`
#     Then stdout from "sh -c ./foo" should contain exactly "42"
#     And the exit status should be 0
