use super::input::Input;
use super::utilities::*;
use crate::ast::*;
use crate::debug::Location;
use crate::types::{self, Type};
use nom::{
    branch::*, character::complete::*, combinator::*, error::*, multi::*, sequence::*, Err, IResult,
};
use std::str::FromStr;

const KEYWORDS: &[&str] = &["in", "let"];

pub fn module(input: Input) -> IResult<Input, Module> {
    terminated(
        many0(terminated(typed_definition, line_break)),
        tuple((convert_combinator(multispace0), eof)),
    )(input)
    .map(|(input, definitions)| (input, Module::new(definitions)))
}

fn typed_definition(input: Input) -> IResult<Input, Definition> {
    alt((
        map(function_definition, |function_definition| {
            function_definition.into()
        }),
        map(value_definition, |value_definition| value_definition.into()),
    ))(input)
}

fn definition(input: Input) -> IResult<Input, Definition> {
    alt((
        typed_definition,
        map(untyped_function_definition, |function_definition| {
            function_definition.into()
        }),
        map(untyped_value_definition, |value_definition| {
            value_definition.into()
        }),
    ))(input)
}

fn function_definition(original_input: Input) -> IResult<Input, FunctionDefinition> {
    tuple((
        identifier,
        keyword(":"),
        function_type,
        line_break,
        identifier,
        many1(identifier),
        keyword("="),
        body,
    ))(original_input.clone())
    .and_then(
        |(input, (name, _, type_, _, same_name, arguments, _, body))| {
            if name == same_name {
                Ok((input, FunctionDefinition::new(name, arguments, body, type_)))
            } else {
                Err(nom::Err::Error((original_input, ErrorKind::Verify)))
            }
        },
    )
}

fn value_definition(original_input: Input) -> IResult<Input, ValueDefinition> {
    tuple((
        identifier,
        keyword(":"),
        type_,
        line_break,
        identifier,
        keyword("="),
        body,
    ))(original_input.clone())
    .and_then(|(input, (name, _, type_, _, same_name, _, body))| {
        if name == same_name {
            Ok((input, ValueDefinition::new(name, body, type_)))
        } else {
            Err(nom::Err::Error((original_input, ErrorKind::Verify)))
        }
    })
}

fn untyped_function_definition(input: Input) -> IResult<Input, FunctionDefinition> {
    map(
        tuple((identifier, many1(identifier), keyword("="), body)),
        |(name, arguments, _, body)| {
            FunctionDefinition::new(
                name,
                arguments,
                body,
                types::Function::new(types::Variable::new().into(), types::Variable::new().into()),
            )
        },
    )(input)
}

fn untyped_value_definition(input: Input) -> IResult<Input, ValueDefinition> {
    map(
        tuple((identifier, keyword("="), body)),
        |(name, _, body)| ValueDefinition::new(name, body, types::Variable::new().into()),
    )(input)
}

fn body(input: Input) -> IResult<Input, Expression> {
    let braces = input.braces();

    expression(input.set_braces(0))
        .map(|(input, expression)| (input.set_braces(braces), expression))
}

fn expression(input: Input) -> IResult<Input, Expression> {
    alt((
        map(operation, |operation| Expression::Operation(operation)),
        term,
    ))(input)
}

fn let_(input: Input) -> IResult<Input, Let> {
    map(
        tuple((
            keyword("let"),
            definition,
            many0(preceded(line_break, definition)),
            opt(line_break),
            keyword("in"),
            expression,
        )),
        |(_, definition, definitions, _, _, expression)| {
            Let::new(
                [definition]
                    .iter()
                    .chain(&definitions)
                    .map(|definition| definition.clone())
                    .collect::<Vec<_>>(),
                expression,
            )
        },
    )(input)
}

fn application(input: Input) -> IResult<Input, Application> {
    map(
        tuple((atomic_expression, many1(atomic_expression))),
        |(function, mut arguments)| {
            let mut drain = arguments.drain(..);
            let mut application = Application::new(function, drain.next().unwrap());

            for argument in drain {
                application = Application::new(application.into(), argument);
            }

            application
        },
    )(input)
}

fn atomic_expression(input: Input) -> IResult<Input, Expression> {
    alt((
        map(number_literal, |number| Expression::Number(number)),
        map(identifier, |identifier| Expression::Variable(identifier)),
        parenthesesed(expression),
    ))(input)
}

fn term(input: Input) -> IResult<Input, Expression> {
    alt((
        map(application, |application| application.into()),
        map(let_, |let_| let_.into()),
        atomic_expression,
    ))(input)
}

fn operation(input: Input) -> IResult<Input, Operation> {
    tuple((term, many1(tuple((operator, term)))))(input)
        .map(|(input, (lhs, pairs))| (input, reduce_operations(lhs, pairs.into())))
}

fn operator(input: Input) -> IResult<Input, Operator> {
    alt((
        create_operator("+", Operator::Add),
        create_operator("-", Operator::Subtract),
        create_operator("*", Operator::Multiply),
        create_operator("/", Operator::Divide),
    ))(input)
}

fn create_operator<'a>(
    literal: &'static str,
    operator: Operator,
) -> impl Fn(Input<'a>) -> IResult<Input<'a>, Operator> {
    move |input| keyword(literal)(input).map(|(input, _)| (input, operator))
}

fn number_literal(input: Input) -> IResult<Input, f64> {
    token(tuple((
        opt(tag("-")),
        many1(one_of("123456789")),
        opt(tuple((tag("."), many1(convert_combinator(digit1))))),
    )))(input)
    .map(|(input, (sign, head, tail))| {
        (
            input,
            if sign.is_some() { -1.0 } else { 1.0 }
                * f64::from_str(
                    &[
                        head.iter().collect(),
                        tail.map(|(_, digits)| [".", &digits.concat()].concat())
                            .unwrap_or("".into()),
                    ]
                    .concat(),
                )
                .unwrap(),
        )
    })
}

fn identifier(original_input: Input) -> IResult<Input, String> {
    token(tuple((
        convert_combinator(alpha1),
        convert_combinator(alphanumeric0),
    )))(original_input.clone())
    .map(|(input, (head, tail))| (input, format!("{}{}", head, tail)))
    .and_then(|(input, identifier)| {
        if KEYWORDS.iter().any(|keyword| &identifier == keyword) {
            Err(nom::Err::Error((original_input, ErrorKind::Verify)))
        } else {
            Ok((input, identifier))
        }
    })
}

fn type_(input: Input) -> IResult<Input, Type> {
    alt((map(bare_function_type, |type_| type_.into()), atomic_type))(input)
}

fn function_type(input: Input) -> IResult<Input, types::Function> {
    alt((bare_function_type, parenthesesed(function_type)))(input)
}

fn bare_function_type(input: Input) -> IResult<Input, types::Function> {
    tuple((atomic_type, keyword("->"), type_))(input)
        .map(|(input, (argument, _, result))| (input, types::Function::new(argument, result)))
}

fn atomic_type(input: Input) -> IResult<Input, Type> {
    alt((number_type, parenthesesed_type))(input)
}

fn parenthesesed_type(input: Input) -> IResult<Input, Type> {
    parenthesesed(type_)(input)
}

fn parenthesesed<'a, T>(
    combinator: impl Fn(Input) -> IResult<Input, T>,
) -> impl Fn(Input<'a>) -> IResult<Input<'a>, T> {
    delimited(left_parenthesis, combinator, right_parenthesis)
}

fn left_parenthesis(input: Input) -> IResult<Input, ()> {
    keyword("(")(input)
}

fn right_parenthesis(input: Input) -> IResult<Input, ()> {
    keyword(")")(input)
}

fn number_type(input: Input) -> IResult<Input, Type> {
    keyword("Number")(input).map(|(input, _)| (input, Type::Number))
}

fn keyword<'a>(keyword: &'static str) -> impl Fn(Input<'a>) -> IResult<Input<'a>, ()> {
    nullify(token(tag(keyword)))
}

fn nullify<'a, T>(
    combinator: impl Fn(Input<'a>) -> IResult<Input<'a>, T>,
) -> impl Fn(Input<'a>) -> IResult<Input<'a>, ()> {
    move |input| combinator(input).map(|(input, _)| (input, ()))
}

fn token<'a, T>(
    combinator: impl Fn(Input<'a>) -> IResult<Input<'a>, T>,
) -> impl Fn(Input<'a>) -> IResult<Input<'a>, T> {
    preceded(blank, combinator)
}

fn blank(input: Input) -> IResult<Input, ()> {
    nullify(many0(one_of(if input.braces() > 0 {
        " \t\n"
    } else {
        " \t"
    })))(input)
}

fn line_break(input: Input) -> IResult<Input, ()> {
    alt((
        nullify(many1(preceded(
            white_space,
            convert_character_combinator(newline),
        ))),
        token(eof),
    ))(input)
}

fn white_space(input: Input) -> IResult<Input, ()> {
    nullify(many0(one_of(" \t")))(input)
}

fn eof(input: Input) -> IResult<Input, ()> {
    if input.source() == "" {
        Ok((input, ()))
    } else {
        Err(nom::Err::Error((input, ErrorKind::Eof)))
    }
}

fn tag<'a>(tag: &'static str) -> impl Fn(Input<'a>) -> IResult<Input<'a>, &str> {
    convert_combinator(nom::bytes::complete::tag(tag))
}

fn one_of<'a>(characters: &'static str) -> impl Fn(Input<'a>) -> IResult<Input<'a>, char> {
    convert_character_combinator(nom::character::complete::one_of(characters))
}

fn convert_combinator<'a>(
    combinator: impl Fn(&'a str) -> IResult<&'a str, &str>,
) -> impl Fn(Input<'a>) -> IResult<Input<'a>, &str> {
    move |input| {
        let braces = input.braces();
        let location = input.location();

        combinator(input.source())
            .map(|(source, string)| {
                (
                    {
                        Input::with_metadata(
                            source,
                            braces + string.matches("(").count() - string.matches(")").count(),
                            string
                                .chars()
                                .into_iter()
                                .fold(location, |location, character| {
                                    if character == '\n' {
                                        location.increment_line_number()
                                    } else {
                                        location.increment_column_number()
                                    }
                                }),
                        )
                    },
                    string,
                )
            })
            .map_err(|error| convert_error(error, braces, location))
    }
}

fn convert_character_combinator<'a>(
    combinator: impl Fn(&'a str) -> IResult<&'a str, char>,
) -> impl Fn(Input<'a>) -> IResult<Input<'a>, char> {
    move |input| {
        let braces = input.braces();
        let location = input.location();

        combinator(input.source())
            .map(|(source, character)| {
                (
                    match character {
                        '\n' => {
                            Input::with_metadata(source, braces, location.increment_line_number())
                        }
                        '(' => Input::with_metadata(
                            source,
                            braces + 1,
                            location.increment_column_number(),
                        ),
                        ')' => Input::with_metadata(
                            source,
                            braces - 1,
                            location.increment_column_number(),
                        ),
                        _ => {
                            Input::with_metadata(source, braces, location.increment_column_number())
                        }
                    },
                    character,
                )
            })
            .map_err(|error| convert_error(error, braces, location))
    }
}

fn convert_error(
    error: Err<(&str, ErrorKind)>,
    braces: usize,
    location: Location,
) -> Err<(Input, ErrorKind)> {
    match error {
        Err::Error((source, kind)) => {
            Err::Error((Input::with_metadata(source, braces, location), kind))
        }
        Err::Failure((source, kind)) => {
            Err::Failure((Input::with_metadata(source, braces, location), kind))
        }
        Err::Incomplete(needed) => Err::Incomplete(needed),
    }
}

#[cfg(test)]
mod test {
    use super::{
        application, blank, expression, function_definition, identifier, keyword, let_, line_break,
        module, number_literal, number_type, type_, value_definition, Input,
    };
    use crate::ast::*;
    use crate::debug::Location;
    use crate::types::{self, Type};
    use nom::error::*;

    #[test]
    fn parse_blank() {
        assert_eq!(
            blank(Input::new("")),
            Ok((Input::with_metadata("", 0, Location::default()), ()))
        );
        assert_eq!(
            blank(Input::new(" ")),
            Ok((Input::with_metadata("", 0, Location::new(1, 2)), ()))
        );
        assert_eq!(
            blank(Input::new("\t")),
            Ok((Input::with_metadata("", 0, Location::new(1, 2)), ()))
        );
        assert_eq!(
            blank(Input::new("  ")),
            Ok((Input::with_metadata("", 0, Location::new(1, 3)), ()))
        );
        assert_eq!(
            blank(Input::with_metadata("\n", 1, Location::default())),
            Ok((Input::with_metadata("", 1, Location::new(2, 1)), ()))
        );
        assert_eq!(
            blank(Input::new("\n")),
            Ok((Input::with_metadata("\n", 0, Location::default()), ()))
        );
    }

    #[test]
    fn parse_number_type() {
        assert_eq!(
            number_type(Input::new("Number")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 7)),
                Type::Number
            ))
        );
        assert_eq!(
            number_type(Input::new("Numbe")),
            Err(nom::Err::Error((
                Input::with_metadata("Numbe", 0, Location::default()),
                ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn parse_type() {
        assert_eq!(
            type_(Input::new("Number")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 7)),
                Type::Number
            ))
        );
        assert_eq!(
            type_(Input::new("(Number)")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 9)),
                Type::Number
            ))
        );
        assert_eq!(
            type_(Input::new("( Number )")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 11)),
                Type::Number
            ))
        );
        assert_eq!(
            type_(Input::new("Number -> Number")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 17)),
                Type::Function(types::Function::new(Type::Number, Type::Number))
            ))
        );
        assert_eq!(
            type_(Input::new("Number -> Number -> Number")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 27)),
                Type::Function(types::Function::new(
                    Type::Number,
                    Type::Function(types::Function::new(Type::Number, Type::Number))
                ))
            ))
        );
    }

    #[test]
    fn parse_keyword() {
        assert_eq!(
            keyword("foo")(Input::new("foo")),
            Ok((Input::with_metadata("", 0, Location::new(1, 4)), ()))
        );
        assert_eq!(
            keyword("foo")(Input::new("fo")),
            Err(nom::Err::Error((
                Input::with_metadata("fo", 0, Location::default()),
                ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn parse_identifier() {
        assert_eq!(
            identifier(Input::new("foo")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 4)),
                "foo".into()
            ))
        );
        assert_eq!(
            identifier(Input::new("x1")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 3)),
                "x1".into()
            ))
        );
        assert_eq!(
            identifier(Input::new("1st")),
            Err(nom::Err::Error((
                Input::with_metadata("1st", 0, Location::default()),
                ErrorKind::Alpha
            )))
        );
        assert_eq!(
            identifier(Input::new("let")),
            Err(nom::Err::Error((
                Input::with_metadata("let", 0, Location::default()),
                ErrorKind::Verify
            )))
        );
        assert_eq!(
            identifier(Input::new("in")),
            Err(nom::Err::Error((
                Input::with_metadata("in", 0, Location::default()),
                ErrorKind::Verify
            )))
        );
    }

    #[test]
    fn parse_number_literal() {
        assert_eq!(
            number_literal(Input::new("1")),
            Ok((Input::with_metadata("", 0, Location::new(1, 2)), 1.0))
        );
        assert_eq!(
            number_literal(Input::new("01")),
            Err(nom::Err::Error((
                Input::with_metadata("01", 0, Location::default()),
                ErrorKind::OneOf
            )))
        );
        assert_eq!(
            number_literal(Input::new("-1")),
            Ok((Input::with_metadata("", 0, Location::new(1, 3)), -1.0))
        );
        assert_eq!(
            number_literal(Input::new("42")),
            Ok((Input::with_metadata("", 0, Location::new(1, 3)), 42.0))
        );
        assert_eq!(
            number_literal(Input::new("3.14")),
            Ok((Input::with_metadata("", 0, Location::new(1, 5)), 3.14))
        );
    }

    #[test]
    fn parse_operation() {
        assert_eq!(
            expression(Input::new("1 + 2")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 6)),
                Operation::new(Operator::Add, 1.0.into(), 2.0.into()).into()
            ))
        );
        assert_eq!(
            expression(Input::new("1 * 2")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 6)),
                Operation::new(Operator::Multiply, 1.0.into(), 2.0.into()).into()
            ))
        );
        assert_eq!(
            expression(Input::new("1 * 2 - 3")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 10)),
                Operation::new(
                    Operator::Subtract,
                    Operation::new(Operator::Multiply, 1.0.into(), 2.0.into()).into(),
                    3.0.into()
                )
                .into()
            ))
        );
        assert_eq!(
            expression(Input::new("1 + 2 * 3")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 10)),
                Operation::new(
                    Operator::Add,
                    1.0.into(),
                    Operation::new(Operator::Multiply, 2.0.into(), 3.0.into()).into(),
                )
                .into()
            ))
        );
        assert_eq!(
            expression(Input::new("1 * 2 - 3 / 4")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 14)),
                Operation::new(
                    Operator::Subtract,
                    Operation::new(Operator::Multiply, 1.0.into(), 2.0.into()).into(),
                    Operation::new(Operator::Divide, 3.0.into(), 4.0.into()).into()
                )
                .into()
            ))
        );
    }

    #[test]
    fn parse_line_break() {
        assert_eq!(
            line_break(Input::new("\n")),
            Ok((Input::with_metadata("", 0, Location::new(2, 1)), ()))
        );
        assert_eq!(
            line_break(Input::new(" \n")),
            Ok((Input::with_metadata("", 0, Location::new(2, 1)), ()))
        );
        assert_eq!(
            line_break(Input::new("\n\n")),
            Ok((Input::with_metadata("", 0, Location::new(3, 1)), ()))
        );
        assert_eq!(
            line_break(Input::new("\n \n")),
            Ok((Input::with_metadata("", 0, Location::new(3, 1)), ()))
        );

        // EOF
        assert_eq!(
            line_break(Input::new("")),
            Ok((Input::with_metadata("", 0, Location::new(1, 1)), ()))
        );
        assert_eq!(
            line_break(Input::new(" ")),
            Ok((Input::with_metadata("", 0, Location::new(1, 2)), ()))
        );
    }

    #[test]
    fn parse_module() {
        assert_eq!(
            module(Input::new("")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 1)),
                Module::new(vec![])
            ))
        );
        assert_eq!(
            module(Input::new(" ")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 2)),
                Module::new(vec![])
            ))
        );
        assert_eq!(
            module(Input::new("\n")),
            Ok((
                Input::with_metadata("", 0, Location::new(2, 1)),
                Module::new(vec![])
            ))
        );
        assert_eq!(
            module(Input::new("x")),
            Err(nom::Err::Error((
                Input::with_metadata("x", 0, Location::default()),
                ErrorKind::Eof
            )))
        );
    }

    #[test]
    fn parse_function_definition() {
        assert_eq!(
            function_definition(Input::new("f : Number -> Number\nf x = x")),
            Ok((
                Input::with_metadata("", 0, Location::new(2, 8)),
                FunctionDefinition::new(
                    "f".into(),
                    vec!["x".into()],
                    Expression::Variable("x".into()),
                    types::Function::new(Type::Number, Type::Number).into()
                )
            ))
        );
        assert_eq!(
            function_definition(Input::new("f : (\n  Number ->\n  Number\n)\nf x = x")),
            Ok((
                Input::with_metadata("", 0, Location::new(5, 8)),
                FunctionDefinition::new(
                    "f".into(),
                    vec!["x".into()],
                    Expression::Variable("x".into()),
                    types::Function::new(Type::Number, Type::Number).into()
                )
            ))
        );
        assert_eq!(
            function_definition(Input::new("f : ((Number -> Number))\nf x = x")),
            Ok((
                Input::with_metadata("", 0, Location::new(2, 8)),
                FunctionDefinition::new(
                    "f".into(),
                    vec!["x".into()],
                    Expression::Variable("x".into()),
                    types::Function::new(Type::Number, Type::Number).into()
                )
            ))
        );
    }

    #[test]
    fn parse_value_definition() {
        assert_eq!(
            value_definition(Input::new("x : Number\nx = 42")),
            Ok((
                Input::with_metadata("", 0, Location::new(2, 7)),
                ValueDefinition::new("x".into(), Expression::Number(42.0), Type::Number)
            ))
        );
    }

    #[test]
    fn parse_application() {
        assert_eq!(
            expression(Input::new("f x")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 4)),
                Application::new(
                    Expression::Variable("f".into()),
                    Expression::Variable("x".into())
                )
                .into()
            ))
        );
        assert_eq!(
            expression(Input::new("f x y")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 6)),
                Application::new(
                    Application::new(
                        Expression::Variable("f".into()),
                        Expression::Variable("x".into())
                    )
                    .into(),
                    Expression::Variable("y".into())
                )
                .into()
            ))
        );
        assert_eq!(
            application(Input::new("f")),
            Err(nom::Err::Error((
                Input::with_metadata("", 0, Location::new(1, 2)),
                ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn parse_let() {
        assert_eq!(
            let_(Input::new("let x = 42\nin x")),
            Ok((
                Input::with_metadata("", 0, Location::new(2, 5)),
                Let::new(
                    vec![ValueDefinition::new(
                        "x".into(),
                        Expression::Number(42.0),
                        types::Variable::new().into()
                    )
                    .into()],
                    Expression::Variable("x".into())
                )
            ))
        );
        assert_eq!(
            let_(Input::new("let x = 42 in x")),
            Ok((
                Input::with_metadata("", 0, Location::new(1, 16)),
                Let::new(
                    vec![ValueDefinition::new(
                        "x".into(),
                        Expression::Number(42.0),
                        types::Variable::new().into()
                    )
                    .into()],
                    Expression::Variable("x".into())
                )
            ))
        );
        assert_eq!(
            let_(Input::new("let x = 42\ny = 42\nin x")),
            Ok((
                Input::with_metadata("", 0, Location::new(3, 5)),
                Let::new(
                    vec![
                        ValueDefinition::new(
                            "x".into(),
                            Expression::Number(42.0),
                            types::Variable::new().into()
                        )
                        .into(),
                        ValueDefinition::new(
                            "y".into(),
                            Expression::Number(42.0),
                            types::Variable::new().into()
                        )
                        .into()
                    ],
                    Expression::Variable("x".into())
                )
            ))
        );
        assert_eq!(
            let_(Input::new("let x : Number\nx = 42\nin x")),
            Ok((
                Input::with_metadata("", 0, Location::new(3, 5)),
                Let::new(
                    vec![
                        ValueDefinition::new("x".into(), Expression::Number(42.0), Type::Number)
                            .into()
                    ],
                    Expression::Variable("x".into())
                )
            ))
        );
        assert_eq!(
            let_(Input::new("let f x = x\nin f")),
            Ok((
                Input::with_metadata("", 0, Location::new(2, 5)),
                Let::new(
                    vec![FunctionDefinition::new(
                        "f".into(),
                        vec!["x".into()],
                        Expression::Variable("x".into()),
                        types::Function::new(
                            types::Variable::new().into(),
                            types::Variable::new().into()
                        )
                    )
                    .into()],
                    Expression::Variable("f".into())
                )
            ))
        );
        assert_eq!(
            let_(Input::new("let f : Number -> Number\nf x = x\nin f")),
            Ok((
                Input::with_metadata("", 0, Location::new(3, 5)),
                Let::new(
                    vec![FunctionDefinition::new(
                        "f".into(),
                        vec!["x".into()],
                        Expression::Variable("x".into()),
                        types::Function::new(Type::Number, Type::Number)
                    )
                    .into()],
                    Expression::Variable("f".into())
                )
            ))
        );
        assert_eq!(
            let_(Input::new("let f x = x\ng x = x\nin x")),
            Ok((
                Input::with_metadata("", 0, Location::new(3, 5)),
                Let::new(
                    vec![
                        FunctionDefinition::new(
                            "f".into(),
                            vec!["x".into()],
                            Expression::Variable("x".into()),
                            types::Function::new(
                                types::Variable::new().into(),
                                types::Variable::new().into()
                            )
                        )
                        .into(),
                        FunctionDefinition::new(
                            "g".into(),
                            vec!["x".into()],
                            Expression::Variable("x".into()),
                            types::Function::new(
                                types::Variable::new().into(),
                                types::Variable::new().into()
                            )
                        )
                        .into()
                    ],
                    Expression::Variable("x".into())
                )
            ))
        );
    }

    #[test]
    fn parse_let_as_expression() {
        assert_eq!(
            expression(Input::new("let x = 42\nin x")),
            Ok((
                Input::with_metadata("", 0, Location::new(2, 5)),
                Let::new(
                    vec![ValueDefinition::new(
                        "x".into(),
                        Expression::Number(42.0),
                        types::Variable::new().into()
                    )
                    .into()],
                    Expression::Variable("x".into())
                )
                .into()
            ))
        );
        assert_eq!(
            expression(Input::new("(\nlet\nx = 42\ny = 42\nin x\n)")),
            Ok((
                Input::with_metadata("", 0, Location::new(6, 2)),
                Let::new(
                    vec![
                        ValueDefinition::new(
                            "x".into(),
                            Expression::Number(42.0),
                            types::Variable::new().into()
                        )
                        .into(),
                        ValueDefinition::new(
                            "y".into(),
                            Expression::Number(42.0),
                            types::Variable::new().into()
                        )
                        .into()
                    ],
                    Expression::Variable("x".into())
                )
                .into()
            ))
        );
    }

    #[test]
    fn parse_let_in_definition() {
        assert_eq!(
            value_definition(Input::new("x : Number\nx = (let y = 42\nin y)")),
            Ok((
                Input::with_metadata("", 0, Location::new(3, 6)),
                ValueDefinition::new(
                    "x".into(),
                    Let::new(
                        vec![ValueDefinition::new(
                            "y".into(),
                            Expression::Number(42.0),
                            types::Variable::new().into()
                        )
                        .into()],
                        Expression::Variable("y".into())
                    )
                    .into(),
                    Type::Number
                )
                .into()
            ))
        );
    }
}
