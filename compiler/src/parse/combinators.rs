use super::input::Input;
use super::utilities::*;
use crate::ast::*;
use crate::debug::SourceInformation;
use crate::types::{self, Type};
use nom::{
    branch::*, character::complete::*, combinator::*, error::*, multi::*, sequence::*, Err, IResult,
};
use std::collections::HashSet;
use std::rc::Rc;
use std::str::FromStr;

const KEYWORDS: &[&str] = &["export", "import", "in", "let"];

pub fn module(input: Input) -> IResult<Input, Module> {
    terminated(
        tuple((
            opt(terminated(export, line_break)),
            many0(terminated(import, line_break)),
            many0(terminated(typed_definition, line_break)),
        )),
        tuple((convert_combinator(multispace0), eof)),
    )(input)
    .map(|(input, (exported_names, imports, definitions))| {
        (
            input,
            Module::new(
                exported_names.unwrap_or_else(|| Default::default()),
                imports,
                definitions,
            ),
        )
    })
}

fn export(input: Input) -> IResult<Input, HashSet<String>> {
    map(
        delimited(
            tuple((keyword("export"), left_brace)),
            terminated(
                tuple((identifier, many0(preceded(comma, identifier)))),
                opt(comma),
            ),
            right_brace,
        ),
        |(identifier, identifiers)| {
            vec![identifier]
                .iter()
                .chain(identifiers.iter())
                .cloned()
                .collect()
        },
    )(input)
}

fn import(input: Input) -> IResult<Input, Import> {
    map(preceded(keyword("import"), module_path), |module_path| {
        Import::new(module_path)
    })(input)
}

fn module_path(input: Input) -> IResult<Input, ModulePath> {
    map(
        tuple((
            opt(keyword(".")),
            identifier,
            many0(preceded(tag("."), identifier)),
        )),
        |(period, identifier, identifiers)| {
            (match period {
                Some(_) => ModulePath::Internal,
                None => ModulePath::External,
            })(
                vec![identifier]
                    .into_iter()
                    .chain(identifiers.into_iter())
                    .collect(),
            )
        },
    )(input)
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
        source_information,
        identifier,
        keyword(":"),
        type_,
        line_break,
        identifier,
        many1(identifier),
        keyword("="),
        body,
    ))(original_input.clone())
    .and_then(
        |(input, (source_information, name, _, type_, _, same_name, arguments, _, body))| {
            if name == same_name {
                Ok((
                    input,
                    FunctionDefinition::new(name, arguments, body, type_, source_information),
                ))
            } else {
                Err(nom::Err::Error((original_input, ErrorKind::Verify)))
            }
        },
    )
}

fn value_definition(original_input: Input) -> IResult<Input, ValueDefinition> {
    tuple((
        source_information,
        identifier,
        keyword(":"),
        type_,
        line_break,
        identifier,
        keyword("="),
        body,
    ))(original_input.clone())
    .and_then(
        |(input, (source_information, name, _, type_, _, same_name, _, body))| {
            if name == same_name {
                Ok((
                    input,
                    ValueDefinition::new(name, body, type_, source_information),
                ))
            } else {
                Err(nom::Err::Error((original_input, ErrorKind::Verify)))
            }
        },
    )
}

fn untyped_function_definition(input: Input) -> IResult<Input, FunctionDefinition> {
    map(
        tuple((
            source_information,
            identifier,
            many1(identifier),
            keyword("="),
            body,
        )),
        |(source_information, name, arguments, _, body)| {
            let source_information = Rc::new(source_information);

            FunctionDefinition::new(
                name,
                arguments,
                body,
                types::Function::new(
                    types::Variable::new(source_information.clone()),
                    types::Variable::new(source_information.clone()),
                    source_information.clone(),
                ),
                source_information,
            )
        },
    )(input)
}

fn untyped_value_definition(input: Input) -> IResult<Input, ValueDefinition> {
    map(
        tuple((source_information, identifier, keyword("="), body)),
        |(source_information, name, _, body)| {
            let source_information = Rc::new(source_information);

            ValueDefinition::new(
                name,
                body,
                types::Variable::new(source_information.clone()),
                source_information,
            )
        },
    )(input)
}

fn body(input: Input) -> IResult<Input, Expression> {
    let braces = input.braces();

    expression(input.set_braces(0))
        .map(|(input, expression)| (input.set_braces(braces), expression))
}

fn expression(input: Input) -> IResult<Input, Expression> {
    alt((map(operation, Expression::Operation), term))(input)
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
                    .cloned()
                    .collect::<Vec<_>>(),
                expression,
            )
        },
    )(input)
}

fn application(input: Input) -> IResult<Input, Application> {
    map(
        tuple((
            source_information,
            atomic_expression,
            many1(atomic_expression),
        )),
        |(source_information, function, mut arguments)| {
            let source_information = Rc::new(source_information);
            let mut drain = arguments.drain(..);
            let mut application =
                Application::new(function, drain.next().unwrap(), source_information.clone());

            for argument in drain {
                application = Application::new(application, argument, source_information.clone());
            }

            application
        },
    )(input)
}

fn atomic_expression(input: Input) -> IResult<Input, Expression> {
    alt((
        map(
            tuple((source_information, number_literal)),
            |(source_information, number)| Number::new(number, source_information).into(),
        ),
        map(
            tuple((source_information, identifier)),
            |(source_information, identifier)| Variable::new(identifier, source_information).into(),
        ),
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
    tuple((term, many1(tuple((source_information, operator, term)))))(input).map(
        |(input, (lhs, pairs))| {
            (
                input,
                reduce_operations(
                    lhs,
                    pairs
                        .into_iter()
                        .map(|(source_information, operator, term)| {
                            (operator, term, source_information)
                        })
                        .collect(),
                ),
            )
        },
    )
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
    map(
        token(tuple((
            opt(tag("-")),
            many1(one_of("123456789")),
            opt(tuple((tag("."), many1(convert_combinator(digit1))))),
        ))),
        |(sign, head, tail)| {
            (if sign.is_some() { -1.0 } else { 1.0 })
                * f64::from_str(
                    &[
                        head.iter().collect(),
                        tail.map(|(_, digits)| [".", &digits.concat()].concat())
                            .unwrap_or_else(|| "".into()),
                    ]
                    .concat(),
                )
                .unwrap()
        },
    )(input)
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
    alt((function_type, atomic_type))(input)
}

fn function_type(input: Input) -> IResult<Input, Type> {
    tuple((source_information, atomic_type, keyword("->"), type_))(input).map(
        |(input, (source_information, argument, _, result))| {
            (
                input,
                types::Function::new(argument, result, source_information).into(),
            )
        },
    )
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

fn left_brace(input: Input) -> IResult<Input, ()> {
    keyword("{")(input)
}

fn right_brace(input: Input) -> IResult<Input, ()> {
    keyword("}")(input)
}

fn comma(input: Input) -> IResult<Input, ()> {
    keyword(",")(input)
}

fn number_type(input: Input) -> IResult<Input, Type> {
    map(
        tuple((source_information, keyword("Number"))),
        |(source_information, _)| types::Number::new(source_information).into(),
    )(input)
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

fn source_information(original_input: Input) -> IResult<Input, SourceInformation> {
    convert_combinator(nom::character::complete::space0)(original_input.clone()).map(
        |(input, _)| {
            let source_information =
                SourceInformation::new(input.filename(), input.location(), input.line());
            (original_input, source_information)
        },
    )
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
                        input.set(
                            source,
                            braces + string.matches('(').count() - string.matches(')').count(),
                            string.chars().fold(location, |location, character| {
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
            .map_err(|error| convert_error(error, &input))
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
                        '\n' => input.set(source, braces, location.increment_line_number()),
                        '(' => input.set(source, braces + 1, location.increment_column_number()),
                        ')' => input.set(source, braces - 1, location.increment_column_number()),
                        _ => input.set(source, braces, location.increment_column_number()),
                    },
                    character,
                )
            })
            .map_err(|error| convert_error(error, &input))
    }
}

fn convert_error<'a>(
    error: Err<(&'a str, ErrorKind)>,
    input: &Input<'a>,
) -> Err<(Input<'a>, ErrorKind)> {
    match error {
        Err::Error((_, kind)) => Err::Error((input.clone(), kind)),
        Err::Failure((_, kind)) => Err::Failure((input.clone(), kind)),
        Err::Incomplete(needed) => Err::Incomplete(needed),
    }
}

#[cfg(test)]
mod test {
    use super::{
        application, blank, export, expression, function_definition, identifier, import, keyword,
        let_, line_break, module, number_literal, number_type, source_information, type_,
        value_definition, Input,
    };
    use crate::ast::*;
    use crate::debug::*;
    use crate::types::{self, Type};
    use nom::error::*;

    #[test]
    fn parse_blank() {
        let input = Input::new("", "");

        assert_eq!(
            blank(input.clone()),
            Ok((input.set("", 0, Location::default()), ()))
        );

        let input = Input::new(" ", "");

        assert_eq!(
            blank(input.clone()),
            Ok((input.set("", 0, Location::new(1, 2)), ()))
        );

        let input = Input::new("\t", "");

        assert_eq!(
            blank(input.clone()),
            Ok((input.set("", 0, Location::new(1, 2)), ()))
        );

        let input = Input::new("  ", "");

        assert_eq!(
            blank(input.clone()),
            Ok((input.set("", 0, Location::new(1, 3)), ()))
        );

        let input = Input::new("", "");

        assert_eq!(
            blank(input.set("\n", 1, Location::default())),
            Ok((input.set("", 1, Location::new(2, 1)), ()))
        );

        let input = Input::new("\n", "");

        assert_eq!(
            blank(input.clone()),
            Ok((input.set("\n", 0, Location::default()), ()))
        );
    }

    #[test]
    fn parse_number_type() {
        let input = Input::new("Number", "");

        assert_eq!(
            number_type(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 7)),
                types::Number::new(SourceInformation::dummy()).into()
            ))
        );

        let input = Input::new("Numbe", "");

        assert_eq!(
            number_type(input.clone()),
            Err(nom::Err::Error((
                input.set("Numbe", 0, Location::default()),
                ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn parse_type() {
        let number_type: Type = types::Number::new(SourceInformation::dummy()).into();
        let input = Input::new("Number", "");

        assert_eq!(
            type_(input.clone()),
            Ok((input.set("", 0, Location::new(1, 7)), number_type.clone()))
        );

        let input = Input::new("(Number)", "");

        assert_eq!(
            type_(input.clone()),
            Ok((input.set("", 0, Location::new(1, 9)), number_type.clone()))
        );

        let input = Input::new("( Number )", "");

        assert_eq!(
            type_(input.clone()),
            Ok((input.set("", 0, Location::new(1, 11)), number_type.clone()))
        );

        let input = Input::new("Number -> Number", "");

        assert_eq!(
            type_(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 17)),
                types::Function::new(
                    number_type.clone(),
                    number_type.clone(),
                    SourceInformation::dummy()
                )
                .into()
            ))
        );

        let input = Input::new("Number -> Number -> Number", "");

        assert_eq!(
            type_(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 27)),
                Type::Function(types::Function::new(
                    number_type.clone(),
                    Type::Function(types::Function::new(
                        number_type.clone(),
                        number_type.clone(),
                        SourceInformation::dummy()
                    )),
                    SourceInformation::dummy()
                ))
            ))
        );
    }

    #[test]
    fn parse_keyword() {
        let input = Input::new("foo", "");

        assert_eq!(
            keyword("foo")(input.clone()),
            Ok((input.set("", 0, Location::new(1, 4)), ()))
        );

        let input = Input::new("fo", "");

        assert_eq!(
            keyword("foo")(input.clone()),
            Err(nom::Err::Error((
                input.set("fo", 0, Location::default()),
                ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn parse_identifier() {
        let input = Input::new("foo", "");

        assert_eq!(
            identifier(input.clone()),
            Ok((input.set("", 0, Location::new(1, 4)), "foo".into()))
        );

        let input = Input::new("x1", "");

        assert_eq!(
            identifier(input.clone()),
            Ok((input.set("", 0, Location::new(1, 3)), "x1".into()))
        );

        let input = Input::new("1st", "");

        assert_eq!(
            identifier(input.clone()),
            Err(nom::Err::Error((
                input.set("1st", 0, Location::default()),
                ErrorKind::Alpha
            )))
        );

        let input = Input::new("let", "");

        assert_eq!(
            identifier(input.clone()),
            Err(nom::Err::Error((
                input.set("let", 0, Location::default()),
                ErrorKind::Verify
            )))
        );

        let input = Input::new("in", "");

        assert_eq!(
            identifier(input.clone()),
            Err(nom::Err::Error((
                input.set("in", 0, Location::default()),
                ErrorKind::Verify
            )))
        );
    }

    #[test]
    fn parse_number_literal() {
        let input = Input::new("1", "");

        assert_eq!(
            number_literal(input.clone()),
            Ok((input.set("", 0, Location::new(1, 2)), 1.0))
        );

        let input = Input::new("01", "");

        assert_eq!(
            number_literal(input.clone()),
            Err(nom::Err::Error((
                input.set("01", 0, Location::default()),
                ErrorKind::OneOf
            )))
        );

        let input = Input::new("-1", "");

        assert_eq!(
            number_literal(input.clone()),
            Ok((input.set("", 0, Location::new(1, 3)), -1.0))
        );

        let input = Input::new("42", "");

        assert_eq!(
            number_literal(input.clone()),
            Ok((input.set("", 0, Location::new(1, 3)), 42.0))
        );

        let input = Input::new("3.14", "");

        assert_eq!(
            number_literal(input.clone()),
            Ok((input.set("", 0, Location::new(1, 5)), 3.14))
        );
    }

    #[test]
    fn parse_operation() {
        let input = Input::new("1 + 2", "");

        assert_eq!(
            expression(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 6)),
                Operation::new(
                    Operator::Add,
                    Number::new(1.0, SourceInformation::dummy()),
                    Number::new(2.0, SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into()
            ))
        );

        let input = Input::new("1 * 2", "");

        assert_eq!(
            expression(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 6)),
                Operation::new(
                    Operator::Multiply,
                    Number::new(1.0, SourceInformation::dummy()),
                    Number::new(2.0, SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into()
            ))
        );

        let input = Input::new("1 * 2 - 3", "");

        assert_eq!(
            expression(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 10)),
                Operation::new(
                    Operator::Subtract,
                    Operation::new(
                        Operator::Multiply,
                        Number::new(1.0, SourceInformation::dummy()),
                        Number::new(2.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    Number::new(3.0, SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into()
            ))
        );

        let input = Input::new("1 + 2 * 3", "");

        assert_eq!(
            expression(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 10)),
                Operation::new(
                    Operator::Add,
                    Number::new(1.0, SourceInformation::dummy()),
                    Operation::new(
                        Operator::Multiply,
                        Number::new(2.0, SourceInformation::dummy()),
                        Number::new(3.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy(),
                )
                .into()
            ))
        );

        let input = Input::new("1 * 2 - 3 / 4", "");

        assert_eq!(
            expression(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 14)),
                Operation::new(
                    Operator::Subtract,
                    Operation::new(
                        Operator::Multiply,
                        Number::new(1.0, SourceInformation::dummy()),
                        Number::new(2.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    Operation::new(
                        Operator::Divide,
                        Number::new(3.0, SourceInformation::dummy()),
                        Number::new(4.0, SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
                .into()
            ))
        );
    }

    #[test]
    fn parse_line_break() {
        let input = Input::new("\n", "");

        assert_eq!(
            line_break(input.clone()),
            Ok((input.set("", 0, Location::new(2, 1)), ()))
        );

        let input = Input::new(" \n", "");

        assert_eq!(
            line_break(input.clone()),
            Ok((input.set("", 0, Location::new(2, 1)), ()))
        );

        let input = Input::new("\n\n", "");

        assert_eq!(
            line_break(input.clone()),
            Ok((input.set("", 0, Location::new(3, 1)), ()))
        );

        let input = Input::new("\n \n", "");

        assert_eq!(
            line_break(input.clone()),
            Ok((input.set("", 0, Location::new(3, 1)), ()))
        );

        // EOF

        let input = Input::new("", "");

        assert_eq!(
            line_break(input.clone()),
            Ok((input.set("", 0, Location::new(1, 1)), ()))
        );

        let input = Input::new(" ", "");

        assert_eq!(
            line_break(input.clone()),
            Ok((input.set("", 0, Location::new(1, 2)), ()))
        );
    }

    #[test]
    fn parse_module() {
        let input = Input::new("", "");

        assert_eq!(
            module(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 1)),
                Module::without_exported_names(vec![])
            ))
        );

        let input = Input::new(" ", "");

        assert_eq!(
            module(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 2)),
                Module::without_exported_names(vec![])
            ))
        );

        let input = Input::new("\n", "");

        assert_eq!(
            module(input.clone()),
            Ok((
                input.set("", 0, Location::new(2, 1)),
                Module::without_exported_names(vec![])
            ))
        );

        let input = Input::new("x", "");

        assert_eq!(
            module(input.clone()),
            Err(nom::Err::Error((
                input.set("x", 0, Location::default()),
                ErrorKind::Eof
            )))
        );
    }

    #[test]
    fn parse_function_definition() {
        let input = Input::new("f : Number -> Number\nf x = x", "");

        assert_eq!(
            function_definition(input.clone()),
            Ok((
                input.set("", 0, Location::new(2, 8)),
                FunctionDefinition::new(
                    "f",
                    vec!["x".into()],
                    Variable::new("x", SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
            ))
        );

        let input = Input::new("f : (\n  Number ->\n  Number\n)\nf x = x", "");

        assert_eq!(
            function_definition(input.clone()),
            Ok((
                input.set("", 0, Location::new(5, 8)),
                FunctionDefinition::new(
                    "f",
                    vec!["x".into()],
                    Variable::new("x", SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
            ))
        );

        let input = Input::new("f : ((Number -> Number))\nf x = x", "");

        assert_eq!(
            function_definition(input.clone()),
            Ok((
                input.set("", 0, Location::new(2, 8)),
                FunctionDefinition::new(
                    "f",
                    vec!["x".into()],
                    Variable::new("x", SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
            ))
        );
    }

    #[test]
    fn parse_value_definition() {
        let input = Input::new("x : Number\nx = 42", "");

        assert_eq!(
            value_definition(input.clone()),
            Ok((
                input.set("", 0, Location::new(2, 7)),
                ValueDefinition::new(
                    "x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
            ))
        );
    }

    #[test]
    fn parse_application() {
        let input = Input::new("f x", "");

        assert_eq!(
            expression(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 4)),
                Application::new(
                    Variable::new("f", SourceInformation::dummy()),
                    Variable::new("x", SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into()
            ))
        );

        let input = Input::new("f x y", "");

        assert_eq!(
            expression(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 6)),
                Application::new(
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Variable::new("x", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    Variable::new("y", SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into()
            ))
        );

        let input = Input::new("f", "");

        assert_eq!(
            application(input.clone()),
            Err(nom::Err::Error((
                input.set("", 0, Location::new(1, 2)),
                ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn parse_let() {
        let input = Input::new("let x = 42\nin x", "");

        assert_eq!(
            let_(input.clone()),
            Ok((
                input.set("", 0, Location::new(2, 5)),
                Let::new(
                    vec![ValueDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Variable::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                )
            ))
        );

        let input = Input::new("let x = 42 in x", "");

        assert_eq!(
            let_(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 16)),
                Let::new(
                    vec![ValueDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Variable::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                )
            ))
        );

        let input = Input::new("let x = 42\ny = 42\nin x", "");

        assert_eq!(
            let_(input.clone()),
            Ok((
                input.set("", 0, Location::new(3, 5)),
                Let::new(
                    vec![
                        ValueDefinition::new(
                            "x",
                            Number::new(42.0, SourceInformation::dummy()),
                            types::Variable::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                        .into(),
                        ValueDefinition::new(
                            "y",
                            Number::new(42.0, SourceInformation::dummy()),
                            types::Variable::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                        .into()
                    ],
                    Variable::new("x", SourceInformation::dummy())
                )
            ))
        );

        let input = Input::new("let x : Number\nx = 42\nin x", "");

        assert_eq!(
            let_(input.clone()),
            Ok((
                input.set("", 0, Location::new(3, 5)),
                Let::new(
                    vec![ValueDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                )
            ))
        );

        let input = Input::new("let f x = x\nin f", "");

        assert_eq!(
            let_(input.clone()),
            Ok((
                input.set("", 0, Location::new(2, 5)),
                Let::new(
                    vec![FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        Variable::new("x", SourceInformation::dummy()),
                        types::Function::new(
                            types::Variable::new(SourceInformation::dummy()),
                            types::Variable::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("f", SourceInformation::dummy())
                )
            ))
        );

        let input = Input::new("let f : Number -> Number\nf x = x\nin f", "");

        assert_eq!(
            let_(input.clone()),
            Ok((
                input.set("", 0, Location::new(3, 5)),
                Let::new(
                    vec![FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        Variable::new("x", SourceInformation::dummy()),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("f", SourceInformation::dummy())
                )
            ))
        );

        let input = Input::new("let f x = x\ng x = x\nin x", "");

        assert_eq!(
            let_(input.clone()),
            Ok((
                input.set("", 0, Location::new(3, 5)),
                Let::new(
                    vec![
                        FunctionDefinition::new(
                            "f",
                            vec!["x".into()],
                            Variable::new("x", SourceInformation::dummy()),
                            types::Function::new(
                                types::Variable::new(SourceInformation::dummy()),
                                types::Variable::new(SourceInformation::dummy()),
                                SourceInformation::dummy()
                            ),
                            SourceInformation::dummy()
                        )
                        .into(),
                        FunctionDefinition::new(
                            "g",
                            vec!["x".into()],
                            Variable::new("x", SourceInformation::dummy()),
                            types::Function::new(
                                types::Variable::new(SourceInformation::dummy()),
                                types::Variable::new(SourceInformation::dummy()),
                                SourceInformation::dummy()
                            ),
                            SourceInformation::dummy()
                        )
                        .into()
                    ],
                    Variable::new("x", SourceInformation::dummy())
                )
            ))
        );
    }

    #[test]
    fn parse_let_as_expression() {
        let input = Input::new("let x = 42\nin x", "");

        assert_eq!(
            expression(input.clone()),
            Ok((
                input.set("", 0, Location::new(2, 5)),
                Let::new(
                    vec![ValueDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Variable::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                )
                .into()
            ))
        );

        let input = Input::new("(\nlet\nx = 42\ny = 42\nin x\n)", "");

        assert_eq!(
            expression(input.clone()),
            Ok((
                input.set("", 0, Location::new(6, 2)),
                Let::new(
                    vec![
                        ValueDefinition::new(
                            "x",
                            Number::new(42.0, SourceInformation::dummy()),
                            types::Variable::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                        .into(),
                        ValueDefinition::new(
                            "y",
                            Number::new(42.0, SourceInformation::dummy()),
                            types::Variable::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                        .into()
                    ],
                    Variable::new("x", SourceInformation::dummy())
                )
                .into()
            ))
        );
    }

    #[test]
    fn parse_let_in_definition() {
        let input = Input::new("x : Number\nx = (let y = 42\nin y)", "");

        assert_eq!(
            value_definition(input.clone()),
            Ok((
                input.set("", 0, Location::new(3, 6)),
                ValueDefinition::new(
                    "x",
                    Let::new(
                        vec![ValueDefinition::new(
                            "y",
                            Number::new(42.0, SourceInformation::dummy()),
                            types::Variable::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        )
                        .into()],
                        Variable::new("y", SourceInformation::dummy())
                    ),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into()
            ))
        );
    }

    #[test]
    fn get_source_information() {
        assert_eq!(
            format!("{}", source_information(Input::new("x", "file")).unwrap().1),
            "file:1:1:\tx\n         \t^"
        );

        assert_eq!(
            format!(
                "{}",
                source_information(Input::new(" x", "file")).unwrap().1
            ),
            "file:1:2:\t x\n         \t ^"
        );
    }

    #[test]
    fn parse_export() {
        let input = Input::new("export {}", "");

        assert_eq!(
            export(input.clone()),
            Err(nom::Err::Error((
                input.set("}", 0, Location::new(1, 9)),
                ErrorKind::Alpha
            )))
        );

        let input = Input::new("export { name }", "");

        assert_eq!(
            export(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 16)),
                vec!["name".into()].iter().cloned().collect()
            ))
        );

        let input = Input::new("export { name, }", "");

        assert_eq!(
            export(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 17)),
                vec!["name".into()].iter().cloned().collect()
            ))
        );

        let input = Input::new("export { name, anotherName }", "");

        assert_eq!(
            export(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 29)),
                vec!["name".into(), "anotherName".into()]
                    .iter()
                    .cloned()
                    .collect()
            ))
        );
    }

    #[test]
    fn parse_import() {
        let input = Input::new("import external", "");

        assert_eq!(
            import(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 16)),
                Import::new(ModulePath::External(vec!["external".into()]))
            ))
        );

        let input = Input::new("import .internal", "");

        assert_eq!(
            import(input.clone()),
            Ok((
                input.set("", 0, Location::new(1, 17)),
                Import::new(ModulePath::Internal(vec!["internal".into()]))
            ))
        );
    }
}
