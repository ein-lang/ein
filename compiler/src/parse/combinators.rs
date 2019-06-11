use super::input::Input;
use super::utilities::*;
use crate::ast::*;
use crate::types::{self, Type};
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, error::*, multi::*,
    sequence::*, Err, IResult,
};
use std::str::FromStr;

pub fn module(input: Input) -> IResult<Input, Module> {
    terminated(
        many0(definition),
        tuple((convert_combinator(multispace0), eof)),
    )(input)
    .map(|(input, definitions)| (input, Module::new(definitions)))
}

fn definition(input: Input) -> IResult<Input, Definition> {
    alt((
        map(function_definition, |function_definition| {
            function_definition.into()
        }),
        map(variable_definition, |variable_definition| {
            variable_definition.into()
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
        expression,
        line_break,
    ))(original_input.clone())
    .and_then(
        |(input, (name, _, type_, _, same_name, arguments, _, body, _))| {
            if name == same_name {
                Ok((input, FunctionDefinition::new(name, arguments, body, type_)))
            } else {
                Err(nom::Err::Error((original_input, ErrorKind::Verify)))
            }
        },
    )
}

fn variable_definition(original_input: Input) -> IResult<Input, VariableDefinition> {
    tuple((
        identifier,
        keyword(":"),
        type_,
        line_break,
        identifier,
        keyword("="),
        expression,
        line_break,
    ))(original_input.clone())
    .and_then(|(input, (name, _, type_, _, same_name, _, body, _))| {
        if name == same_name {
            Ok((input, VariableDefinition::new(name, body, type_)))
        } else {
            Err(nom::Err::Error((original_input, ErrorKind::Verify)))
        }
    })
}

fn expression(input: Input) -> IResult<Input, Expression> {
    alt((
        map(operation, |operation| Expression::Operation(operation)),
        term,
    ))(input)
}

fn term(input: Input) -> IResult<Input, Expression> {
    alt((
        map(number_literal, |number| Expression::Number(number)),
        map(identifier, |identifier| Expression::Variable(identifier)),
        parenthesesed(expression),
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
    token(convert_combinator(tuple((
        opt(tag("-")),
        many1(one_of("123456789")),
        opt(tuple((tag("."), many1(digit1)))),
    ))))(input)
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

fn identifier(input: Input) -> IResult<Input, String> {
    token(convert_combinator(tuple((alpha1, alphanumeric0))))(input)
        .map(|(input, (head, tail))| (input, format!("{}{}", head, tail)))
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
    keyword("(")(input).map(|(input, ())| (Input::new(input.source(), input.braces() + 1), ()))
}

fn right_parenthesis(input: Input) -> IResult<Input, ()> {
    keyword(")")(input).map(|(input, ())| (Input::new(input.source(), input.braces() - 1), ()))
}

fn number_type(input: Input) -> IResult<Input, Type> {
    keyword("Number")(input).map(|(input, _)| (input, Type::Number))
}

fn keyword<'a>(keyword: &'static str) -> impl Fn(Input<'a>) -> IResult<Input<'a>, ()> {
    nullify(token(convert_combinator(tag(keyword))))
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
    nullify(convert_combinator(many0(one_of(if input.braces() > 0 {
        " \t\n"
    } else {
        " \t"
    }))))(input)
}

fn line_break(input: Input) -> IResult<Input, ()> {
    token(alt((nullify(convert_combinator(newline)), eof)))(input)
}

fn eof(input: Input) -> IResult<Input, ()> {
    if input.source() == "" {
        Ok((input, ()))
    } else {
        Err(nom::Err::Failure((input, ErrorKind::Eof)))
    }
}

fn convert_combinator<'a, T>(
    combinator: impl Fn(&'a str) -> IResult<&'a str, T>,
) -> impl Fn(Input<'a>) -> IResult<Input<'a>, T> {
    move |input| {
        let braces = input.braces();
        convert_result(combinator(input.source()), braces)
    }
}

fn convert_result<T>(result: IResult<&str, T>, braces: u64) -> IResult<Input, T> {
    result
        .map(|(source, x)| (Input::new(source, braces), x))
        .map_err(|error| match error {
            Err::Error((source, kind)) => Err::Error((Input::new(source, braces), kind)),
            Err::Failure((source, kind)) => Err::Failure((Input::new(source, braces), kind)),
            Err::Incomplete(needed) => Err::Incomplete(needed),
        })
}

#[cfg(test)]
mod test {
    use super::{
        blank, expression, function_definition, identifier, keyword, line_break, module,
        number_literal, number_type, type_, variable_definition, Input,
    };
    use crate::ast::*;
    use crate::types::{self, Type};
    use nom::error::*;

    #[test]
    fn parse_blank() {
        assert_eq!(blank(Input::new("", 0)), Ok((Input::new("", 0), ())));
        assert_eq!(blank(Input::new(" ", 0)), Ok((Input::new("", 0), ())));
        assert_eq!(blank(Input::new("\t", 0)), Ok((Input::new("", 0), ())));
        assert_eq!(blank(Input::new("  ", 0)), Ok((Input::new("", 0), ())));
        assert_eq!(blank(Input::new("\n", 1)), Ok((Input::new("", 1), ())));
        assert_eq!(blank(Input::new("\n", 0)), Ok((Input::new("\n", 0), ())));
    }

    #[test]
    fn parse_number_type() {
        assert_eq!(
            number_type(Input::new("Number", 0)),
            Ok((Input::new("", 0), Type::Number))
        );
        assert_eq!(
            number_type(Input::new("Numbe", 0)),
            Err(nom::Err::Error((Input::new("Numbe", 0), ErrorKind::Tag)))
        );
    }

    #[test]
    fn parse_type() {
        assert_eq!(
            type_(Input::new("Number", 0)),
            Ok((Input::new("", 0), Type::Number))
        );
        assert_eq!(
            type_(Input::new("(Number)", 0)),
            Ok((Input::new("", 0), Type::Number))
        );
        assert_eq!(
            type_(Input::new("( Number )", 0)),
            Ok((Input::new("", 0), Type::Number))
        );
        assert_eq!(
            type_(Input::new("Number -> Number", 0)),
            Ok((
                Input::new("", 0),
                Type::Function(types::Function::new(Type::Number, Type::Number))
            ))
        );
        assert_eq!(
            type_(Input::new("Number -> Number -> Number", 0)),
            Ok((
                Input::new("", 0),
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
            keyword("foo")(Input::new("foo", 0)),
            Ok((Input::new("", 0), ()))
        );
        assert_eq!(
            keyword("foo")(Input::new("fo", 0)),
            Err(nom::Err::Error((Input::new("fo", 0), ErrorKind::Tag)))
        );
    }

    #[test]
    fn parse_identifier() {
        assert_eq!(
            identifier(Input::new("foo", 0)),
            Ok((Input::new("", 0), "foo".into()))
        );
        assert_eq!(
            identifier(Input::new("x1", 0)),
            Ok((Input::new("", 0), "x1".into()))
        );
        assert_eq!(
            identifier(Input::new("1st", 0)),
            Err(nom::Err::Error((Input::new("1st", 0), ErrorKind::Alpha)))
        );
    }

    #[test]
    fn parse_number_literal() {
        assert_eq!(
            number_literal(Input::new("1", 0)),
            Ok((Input::new("", 0), 1.0))
        );
        assert_eq!(
            number_literal(Input::new("01", 0)),
            Err(nom::Err::Error((Input::new("01", 0), ErrorKind::OneOf)))
        );
        assert_eq!(
            number_literal(Input::new("-1", 0)),
            Ok((Input::new("", 0), -1.0))
        );
        assert_eq!(
            number_literal(Input::new("42", 0)),
            Ok((Input::new("", 0), 42.0))
        );
        assert_eq!(
            number_literal(Input::new("3.14", 0)),
            Ok((Input::new("", 0), 3.14))
        );
    }

    #[test]
    fn parse_operation() {
        assert_eq!(
            expression(Input::new("1 + 2", 0)),
            Ok((
                Input::new("", 0),
                Operation::new(Operator::Add, 1.0.into(), 2.0.into()).into()
            ))
        );
        assert_eq!(
            expression(Input::new("1 * 2", 0)),
            Ok((
                Input::new("", 0),
                Operation::new(Operator::Multiply, 1.0.into(), 2.0.into()).into()
            ))
        );
        assert_eq!(
            expression(Input::new("1 * 2 - 3", 0)),
            Ok((
                Input::new("", 0),
                Operation::new(
                    Operator::Subtract,
                    Operation::new(Operator::Multiply, 1.0.into(), 2.0.into()).into(),
                    3.0.into()
                )
                .into()
            ))
        );
        assert_eq!(
            expression(Input::new("1 + 2 * 3", 0)),
            Ok((
                Input::new("", 0),
                Operation::new(
                    Operator::Add,
                    1.0.into(),
                    Operation::new(Operator::Multiply, 2.0.into(), 3.0.into()).into(),
                )
                .into()
            ))
        );
        assert_eq!(
            expression(Input::new("1 * 2 - 3 / 4", 0)),
            Ok((
                Input::new("", 0),
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
        assert_eq!(line_break(Input::new("\n", 0)), Ok((Input::new("", 0), ())));
        assert_eq!(
            line_break(Input::new(" \n", 0)),
            Ok((Input::new("", 0), ()))
        );
        assert_eq!(line_break(Input::new("", 0)), Ok((Input::new("", 0), ())));
        assert_eq!(line_break(Input::new(" ", 0)), Ok((Input::new("", 0), ())));
    }

    #[test]
    fn parse_mdoule() {
        assert_eq!(
            module(Input::new("", 0)),
            Ok((Input::new("", 0), Module::new(vec![])))
        );
        assert_eq!(
            module(Input::new(" ", 0)),
            Ok((Input::new("", 0), Module::new(vec![])))
        );
        assert_eq!(
            module(Input::new("\n", 0)),
            Ok((Input::new("", 0), Module::new(vec![])))
        );
        assert_eq!(
            module(Input::new("x", 0)),
            Err(nom::Err::Failure((Input::new("x", 0), ErrorKind::Eof)))
        );
    }

    #[test]
    fn parse_function_definition() {
        assert_eq!(
            function_definition(Input::new("f : Number -> Number\nf x = x", 0)),
            Ok((
                Input::new("", 0),
                FunctionDefinition::new(
                    "f".into(),
                    vec!["x".into()],
                    Expression::Variable("x".into()),
                    types::Function::new(Type::Number, Type::Number).into()
                )
            ))
        );
        assert_eq!(
            function_definition(Input::new("f : (\n  Number ->\n  Number\n)\nf x = x", 0)),
            Ok((
                Input::new("", 0),
                FunctionDefinition::new(
                    "f".into(),
                    vec!["x".into()],
                    Expression::Variable("x".into()),
                    types::Function::new(Type::Number, Type::Number).into()
                )
            ))
        );
        assert_eq!(
            function_definition(Input::new("f : ((Number -> Number))\nf x = x", 0)),
            Ok((
                Input::new("", 0),
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
    fn parse_variable_definition() {
        assert_eq!(
            variable_definition(Input::new("x : Number\nx = 42", 0)),
            Ok((
                Input::new("", 0),
                VariableDefinition::new("x".into(), Expression::Number(42.0), Type::Number,)
            ))
        );
    }
}
