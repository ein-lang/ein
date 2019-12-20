use super::utilities;
use crate::ast::*;
use crate::debug::*;
use crate::path::*;
use crate::types::{self, Type};
use combine::parser::char::{alpha_num, letter, string};
use combine::parser::choice::optional;
use combine::parser::combinator::{lazy, look_ahead, no_partial, not_followed_by};
use combine::parser::regex::find;
use combine::parser::repeat::{many, many1};
use combine::parser::sequence::between;
use combine::stream::position::{self, SourcePosition};
use combine::stream::state;
use combine::{
    attempt, choice, easy, from_str, none_of, one_of, sep_by1, sep_end_by1, unexpected_any, value,
    Parser, Positioned,
};
use lazy_static::lazy_static;
use std::rc::Rc;

const KEYWORDS: &[&str] = &["export", "import", "in", "let"];
const OPERATOR_CHARACTERS: &str = "+-*/=<>&|";
const SPACE_CHARACTERS: &str = " \t\r";

lazy_static! {
    static ref NUMBER_REGEX: regex::Regex =
        regex::Regex::new(r"^-?([123456789][0123456789]*|0)(\.[0123456789]+)?").unwrap();
}

pub struct State<'a> {
    source_name: &'a str,
    lines: Vec<&'a str>,
}

pub type Stream<'a> =
    easy::Stream<state::Stream<position::Stream<&'a str, SourcePosition>, State<'a>>>;

pub fn stream<'a>(source: &'a str, source_name: &'a str) -> Stream<'a> {
    state::Stream {
        stream: position::Stream::new(source),
        state: State {
            source_name,
            lines: source.split('\n').collect(),
        },
    }
    .into()
}

pub fn module<'a>() -> impl Parser<Stream<'a>, Output = UnresolvedModule> {
    (
        optional(export()),
        many(import()),
        many(type_definition()),
        many(definition()),
    )
        .skip(blank())
        .skip(eof())
        .map(|(export, imports, type_definitions, definitions)| {
            UnresolvedModule::new(
                export.unwrap_or_else(|| Export::new(Default::default())),
                imports,
                type_definitions,
                definitions,
            )
        })
}

fn export<'a>() -> impl Parser<Stream<'a>, Output = Export> {
    keyword("export")
        .with(between(
            sign("{"),
            sign("}"),
            sep_end_by1(identifier(), sign(",")),
        ))
        .map(Export::new)
}

fn import<'a>() -> impl Parser<Stream<'a>, Output = Import> {
    keyword("import").with(module_path()).map(Import::new)
}

fn module_path<'a>() -> impl Parser<Stream<'a>, Output = UnresolvedModulePath> {
    token(between(
        string("\""),
        string("\""),
        choice((
            internal_module_path().map(UnresolvedModulePath::from),
            external_module_path().map(UnresolvedModulePath::from),
        )),
    ))
}

fn internal_module_path<'a>() -> impl Parser<Stream<'a>, Output = InternalUnresolvedModulePath> {
    string(".")
        .with(many1(string("/").with(path_component())))
        .map(InternalUnresolvedModulePath::new)
}

fn external_module_path<'a>() -> impl Parser<Stream<'a>, Output = ExternalUnresolvedModulePath> {
    sep_by1(path_component(), string("/")).map(ExternalUnresolvedModulePath::new)
}

fn path_component<'a>() -> impl Parser<Stream<'a>, Output = String> {
    (many1(letter()), many(alpha_num().or(one_of(".-".chars()))))
        .map(|(head, tail): (String, String)| [head, tail].concat())
}

fn definition<'a>() -> impl Parser<Stream<'a>, Output = Definition> {
    choice((
        function_definition().map(Definition::from),
        value_definition().map(Definition::from),
    ))
    .expected("definition")
}

fn function_definition<'a>() -> impl Parser<Stream<'a>, Output = FunctionDefinition> {
    attempt((
        source_information(),
        type_annotation(),
        identifier(),
        many1(identifier()),
        sign("="),
        expression(),
    ))
    .then(
        |(source_information, (typed_name, type_), name, arguments, _, expression)| {
            if typed_name == name {
                value(FunctionDefinition::new(
                    name,
                    arguments,
                    expression,
                    type_,
                    source_information,
                ))
                .left()
            } else {
                unexpected_any("unmatched identifiers in definition").right()
            }
        },
    )
}

fn value_definition<'a>() -> impl Parser<Stream<'a>, Output = ValueDefinition> {
    attempt((
        source_information(),
        type_annotation(),
        identifier(),
        sign("="),
        expression(),
    ))
    .then(
        |(source_information, (typed_name, type_), name, _, expression)| {
            if typed_name == name {
                value(ValueDefinition::new(
                    name,
                    expression,
                    type_,
                    source_information,
                ))
                .left()
            } else {
                unexpected_any("unmatched identifiers in definition").right()
            }
        },
    )
}

fn type_annotation<'a>() -> impl Parser<Stream<'a>, Output = (String, Type)> {
    (identifier(), sign(":").with(type_()))
}

fn untyped_definition<'a>() -> impl Parser<Stream<'a>, Output = Definition> {
    choice((
        untyped_function_definition().map(Definition::from),
        untyped_value_definition().map(Definition::from),
    ))
}

fn untyped_function_definition<'a>() -> impl Parser<Stream<'a>, Output = FunctionDefinition> {
    attempt((
        source_information(),
        identifier(),
        many1(identifier()),
        sign("="),
        expression(),
    ))
    .map(|(source_information, name, arguments, _, expression)| {
        let source_information = Rc::new(source_information);
        FunctionDefinition::new(
            name,
            arguments,
            expression,
            types::Variable::new(source_information.clone()),
            source_information,
        )
    })
}

fn untyped_value_definition<'a>() -> impl Parser<Stream<'a>, Output = ValueDefinition> {
    attempt((source_information(), identifier(), sign("="), expression())).map(
        |(source_information, name, _, expression)| {
            let source_information = Rc::new(source_information);
            ValueDefinition::new(
                name,
                expression,
                types::Variable::new(source_information.clone()),
                source_information,
            )
        },
    )
}

fn type_definition<'a>() -> impl Parser<Stream<'a>, Output = TypeDefinition> {
    attempt((keyword("type"), identifier(), sign("="), type_()))
        .map(|(_, name, _, type_)| TypeDefinition::new(name, type_))
}

fn type_<'a>() -> impl Parser<Stream<'a>, Output = Type> {
    lazy(|| no_partial(choice((function_type().map(Type::from), atomic_type())))).boxed()
}

fn function_type<'a>() -> impl Parser<Stream<'a>, Output = types::Function> {
    attempt((source_information(), atomic_type(), sign("->"), type_())).map(
        |(source_information, argument, _, result)| {
            types::Function::new(argument, result, source_information)
        },
    )
}

fn atomic_type<'a>() -> impl Parser<Stream<'a>, Output = Type> {
    choice((
        number_type().map(Type::from),
        between(sign("("), sign(")"), type_()),
    ))
}

fn number_type<'a>() -> impl Parser<Stream<'a>, Output = types::Number> {
    attempt(source_information().skip(keyword("Number"))).map(types::Number::new)
}

fn expression<'a>() -> impl Parser<Stream<'a>, Output = Expression> {
    lazy(|| no_partial(choice((operation().map(Expression::from), term())))).boxed()
}

fn atomic_expression<'a>() -> impl Parser<Stream<'a>, Output = Expression> {
    choice((
        number_literal().map(Expression::from),
        variable().map(Expression::from),
        between(sign("("), sign(")"), expression()),
    ))
}

fn let_<'a>() -> impl Parser<Stream<'a>, Output = Let> {
    attempt((
        keyword("let").expected("let keyword"),
        many1(definition().or(untyped_definition())),
        keyword("in").expected("in keyword"),
        expression(),
    ))
    .map(|(_, definitions, _, expression)| Let::new(definitions, expression))
}

fn application<'a>() -> impl Parser<Stream<'a>, Output = Application> {
    attempt((
        source_information(),
        atomic_expression(),
        many1(attempt((
            many(attempt(
                atomic_expression().skip(not_followed_by(application_terminator())),
            )),
            atomic_expression().skip(look_ahead(application_terminator())),
        ))),
    ))
    .map(
        |(source_information, function, mut argument_sets): (_, _, Vec<(Vec<Expression>, _)>)| {
            let source_information = Rc::new(source_information);
            let mut all_arguments = vec![];

            for (mut arguments, argument) in argument_sets.drain(..) {
                all_arguments.extend(arguments.drain(..));
                all_arguments.push(argument);
            }

            let mut drain = all_arguments.drain(..);
            let first_argument = drain.next().unwrap();

            drain.fold(
                Application::new(function, first_argument, source_information.clone()),
                |application, argument| {
                    Application::new(application, argument, source_information.clone())
                },
            )
        },
    )
}

fn application_terminator<'a>() -> impl Parser<Stream<'a>, Output = &'static str> {
    choice((newlines1(), sign(")"), operator().with(value(()))))
        .with(value("application terminator"))
}

fn term<'a>() -> impl Parser<Stream<'a>, Output = Expression> {
    choice((
        application().map(Expression::from),
        let_().map(Expression::from),
        atomic_expression(),
    ))
}

fn operation<'a>() -> impl Parser<Stream<'a>, Output = Operation> {
    attempt((
        term(),
        many1((source_information(), operator(), term()).map(
            |(source_information, operator, expression)| (operator, expression, source_information),
        )),
    ))
    .map(|(expression, pairs)| utilities::reduce_operations(expression, pairs))
}

fn operator<'a>() -> impl Parser<Stream<'a>, Output = Operator> {
    choice((
        concrete_operator("+", Operator::Add),
        concrete_operator("-", Operator::Subtract),
        concrete_operator("*", Operator::Multiply),
        concrete_operator("/", Operator::Divide),
    ))
}

fn concrete_operator<'a>(
    literal: &'static str,
    operator: Operator,
) -> impl Parser<Stream<'a>, Output = Operator> {
    token(
        many1(one_of(OPERATOR_CHARACTERS.chars())).then(move |parsed_literal: String| {
            if parsed_literal == literal {
                value(operator).left()
            } else {
                unexpected_any("unknown operator").right()
            }
        }),
    )
}

fn number_literal<'a>() -> impl Parser<Stream<'a>, Output = Number> {
    let regex: &'static regex::Regex = &NUMBER_REGEX;
    token((source_information(), from_str(find(regex))))
        .map(|(source_information, number)| Number::new(number, source_information))
}

fn variable<'a>() -> impl Parser<Stream<'a>, Output = Variable> {
    token((
        source_information(),
        optional(attempt((raw_identifier(), string(".")))),
        raw_identifier(),
    ))
    .map(|(source_information, prefix, identifier)| {
        Variable::new(
            prefix
                .map(|(prefix, _)| [&prefix, ".", &identifier].concat())
                .unwrap_or(identifier),
            source_information,
        )
    })
}

fn identifier<'a>() -> impl Parser<Stream<'a>, Output = String> {
    token(raw_identifier())
}

fn raw_identifier<'a>() -> impl Parser<Stream<'a>, Output = String> {
    attempt(
        (many1(letter()), many(alpha_num()))
            .map(|(head, tail): (String, String)| [head, tail].concat())
            .then(|identifier| {
                if KEYWORDS.iter().any(|keyword| &identifier == keyword) {
                    unexpected_any("keyword").left()
                } else {
                    value(identifier).right()
                }
            }),
    )
}

fn keyword<'a>(name: &'static str) -> impl Parser<Stream<'a>, Output = ()> {
    token(string(name).skip(not_followed_by(alpha_num()))).with(value(()))
}

fn sign<'a>(sign: &'static str) -> impl Parser<Stream<'a>, Output = ()> {
    token(string(sign)).with(value(()))
}

fn token<'a, O, P: Parser<Stream<'a>, Output = O>>(p: P) -> impl Parser<Stream<'a>, Output = O> {
    attempt(blank().with(p))
}

fn source_information<'a>() -> impl Parser<Stream<'a>, Output = SourceInformation> {
    blank().map_input(|_, stream: &mut Stream<'a>| {
        let position = stream.position();
        SourceInformation::new(
            stream.0.state.source_name,
            Location::new(position.line as usize, position.column as usize),
            stream.0.state.lines[position.line as usize - 1],
        )
    })
}

fn blank<'a>() -> impl Parser<Stream<'a>, Output = ()> {
    many::<Vec<_>, _, _>(choice((spaces1(), newline()))).with(value(()))
}

fn spaces1<'a>() -> impl Parser<Stream<'a>, Output = ()> {
    many1::<String, _, _>(one_of(SPACE_CHARACTERS.chars())).with(value(()))
}

fn newlines1<'a>() -> impl Parser<Stream<'a>, Output = ()> {
    choice((
        many1(newline()),
        many::<Vec<_>, _, _>(newline()).with(eof()),
    ))
}

fn newline<'a>() -> impl Parser<Stream<'a>, Output = ()> {
    attempt(optional(spaces1()).with(choice((
        combine::parser::char::newline().with(value(())),
        comment(),
    ))))
}

fn eof<'a>() -> impl Parser<Stream<'a>, Output = ()> {
    optional(spaces1()).with(combine::eof())
}

fn comment<'a>() -> impl Parser<Stream<'a>, Output = ()> {
    string("#")
        .with(many::<Vec<_>, _, _>(none_of("\n".chars())))
        .with(combine::parser::char::newline())
        .with(value(()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_module() {
        assert_eq!(
            module().parse(stream("", "")).unwrap().0,
            UnresolvedModule::from_definitions(vec![])
        );
        assert_eq!(
            module().parse(stream(" ", "")).unwrap().0,
            UnresolvedModule::from_definitions(vec![])
        );
        assert_eq!(
            module().parse(stream("\n", "")).unwrap().0,
            UnresolvedModule::from_definitions(vec![])
        );
        assert_eq!(
            module().parse(stream("export { foo }", "")).unwrap().0,
            UnresolvedModule::new(
                Export::new(vec!["foo".into()].drain(..).collect()),
                vec![],
                vec![],
                vec![]
            )
        );
        assert_eq!(
            module()
                .parse(stream("export { foo }\nimport \"Foo/Bar\"", ""))
                .unwrap()
                .0,
            UnresolvedModule::new(
                Export::new(vec!["foo".into()].drain(..).collect()),
                vec![Import::new(ExternalUnresolvedModulePath::new(vec![
                    "Foo".into(),
                    "Bar".into()
                ]))],
                vec![],
                vec![]
            )
        );
        assert_eq!(
            module().parse(stream("x : Number\nx = 42", "")).unwrap().0,
            UnresolvedModule::new(
                Export::new(Default::default()),
                vec![],
                vec![],
                vec![ValueDefinition::new(
                    "x",
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into()]
            )
        );
        assert_eq!(
            module()
                .parse(stream("x : Number\nx = 42\ny : Number\ny = 42", ""))
                .unwrap()
                .0,
            UnresolvedModule::new(
                Export::new(Default::default()),
                vec![],
                vec![],
                vec![
                    ValueDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into(),
                    ValueDefinition::new(
                        "y",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()
                ]
            )
        );
        assert_eq!(
            module()
                .parse(stream("main : Number -> Number\nmain x = 42", ""))
                .unwrap()
                .0,
            UnresolvedModule::new(
                Export::new(Default::default()),
                vec![],
                vec![],
                vec![FunctionDefinition::new(
                    "main",
                    vec!["x".into()],
                    Number::new(42.0, SourceInformation::dummy()),
                    types::Function::new(
                        types::Number::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
                .into(),]
            )
        );
    }

    #[test]
    fn parse_export() {
        assert!(export().parse(stream("export {}", "")).is_err());
        assert_eq!(
            export().parse(stream("export { foo }", "")).unwrap().0,
            Export::new(vec!["foo".into()].drain(..).collect()),
        );
        assert_eq!(
            export().parse(stream("export { foo, }", "")).unwrap().0,
            Export::new(vec!["foo".into()].drain(..).collect()),
        );
        assert_eq!(
            export().parse(stream("export { foo, bar }", "")).unwrap().0,
            Export::new(vec!["foo".into(), "bar".into()].drain(..).collect()),
        );
        assert_eq!(
            export()
                .parse(stream("export { foo, bar, }", ""))
                .unwrap()
                .0,
            Export::new(vec!["foo".into(), "bar".into()].drain(..).collect()),
        );
        assert_eq!(
            export().parse(stream("export {\nfoo }", "")).unwrap().0,
            Export::new(vec!["foo".into()].drain(..).collect()),
        );
    }

    #[test]
    fn parse_import() {
        assert_eq!(
            import().parse(stream("import \"./Foo\"", "")).unwrap().0,
            Import::new(InternalUnresolvedModulePath::new(vec!["Foo".into()])),
        );
        assert_eq!(
            import().parse(stream("import \"Foo/Bar\"", "")).unwrap().0,
            Import::new(ExternalUnresolvedModulePath::new(vec![
                "Foo".into(),
                "Bar".into()
            ])),
        );
    }

    #[test]
    fn parse_module_path() {
        assert!(module_path().parse(stream("?", "")).is_err());
        assert_eq!(
            module_path().parse(stream("\"./Foo\"", "")).unwrap().0,
            UnresolvedModulePath::Internal(InternalUnresolvedModulePath::new(vec!["Foo".into()])),
        );
        assert_eq!(
            module_path().parse(stream("\"Foo/Bar\"", "")).unwrap().0,
            UnresolvedModulePath::External(ExternalUnresolvedModulePath::new(vec![
                "Foo".into(),
                "Bar".into()
            ])),
        );
        assert_eq!(
            module_path().parse(stream(" \"./Foo\"", "")).unwrap().0,
            UnresolvedModulePath::Internal(InternalUnresolvedModulePath::new(vec!["Foo".into()])),
        );
    }

    #[test]
    fn parse_internal_module_path() {
        assert!(internal_module_path().parse(stream("?", "")).is_err());
        assert_eq!(
            internal_module_path().parse(stream("./Foo", "")).unwrap().0,
            InternalUnresolvedModulePath::new(vec!["Foo".into()]),
        );
        assert_eq!(
            internal_module_path()
                .parse(stream("./Foo/Bar", ""))
                .unwrap()
                .0,
            InternalUnresolvedModulePath::new(vec!["Foo".into(), "Bar".into()]),
        );
    }

    #[test]
    fn parse_external_module_path() {
        assert!(external_module_path().parse(stream("?", "")).is_err());
        assert_eq!(
            external_module_path()
                .parse(stream("Foo/Bar", ""))
                .unwrap()
                .0,
            ExternalUnresolvedModulePath::new(vec!["Foo".into(), "Bar".into()]),
        );
    }

    #[test]
    fn parse_path_component() {
        assert!(path_component().parse(stream("?", "")).is_err());

        for component in &["foo", "github.com", "foo-rs"] {
            assert_eq!(
                path_component().parse(stream(component, "")).unwrap().0,
                component.to_string()
            );
        }
    }

    #[test]
    fn parse_definition() {
        assert_eq!(
            definition()
                .parse(stream("x : Number\nx = 0", ""))
                .unwrap()
                .0,
            ValueDefinition::new(
                "x",
                Number::new(0.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .into()
        );
        assert_eq!(
            definition()
                .parse(stream("main : Number -> Number\nmain x = 42", ""))
                .unwrap()
                .0,
            FunctionDefinition::new(
                "main",
                vec!["x".into()],
                Number::new(42.0, SourceInformation::dummy()),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy()
            )
            .into()
        );
    }

    #[test]
    fn parse_value_definition() {
        assert_eq!(
            value_definition()
                .parse(stream("x : Number\nx = 0", ""))
                .unwrap()
                .0,
            ValueDefinition::new(
                "x",
                Number::new(0.0, SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy()
            )
        );
    }

    #[test]
    fn parse_untyped_definition() {
        assert_eq!(
            untyped_definition().parse(stream("x = 0", "")).unwrap().0,
            ValueDefinition::new(
                "x",
                Number::new(0.0, SourceInformation::dummy()),
                types::Variable::new(SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .into()
        );
        assert_eq!(
            untyped_definition()
                .parse(stream("main x = 42", ""))
                .unwrap()
                .0,
            FunctionDefinition::new(
                "main",
                vec!["x".into()],
                Number::new(42.0, SourceInformation::dummy()),
                types::Variable::new(SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .into()
        );
        assert_eq!(
            (untyped_definition(), untyped_definition())
                .parse(stream(
                    indoc!(
                        "
                        f x = x
                         y = (
                             f x
                         )
                        "
                    ),
                    ""
                ))
                .unwrap()
                .0,
            (
                FunctionDefinition::new(
                    "f",
                    vec!["x".into()],
                    Variable::new("x", SourceInformation::dummy()),
                    types::Variable::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
                ValueDefinition::new(
                    "y",
                    Application::new(
                        Variable::new("f", SourceInformation::dummy()),
                        Variable::new("x", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    types::Variable::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into()
            )
        );
    }

    #[test]
    fn parse_type_definition() {
        assert_eq!(
            type_definition()
                .parse(stream("type Foo = Number", ""))
                .unwrap()
                .0,
            TypeDefinition::new("Foo", types::Number::new(SourceInformation::dummy()))
        );
        assert_eq!(
            type_definition()
                .parse(stream("type Foo = Number -> Number", ""))
                .unwrap()
                .0,
            TypeDefinition::new(
                "Foo",
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
            )
        );
    }

    #[test]
    fn parse_type() {
        assert!(type_().parse(stream("?", "")).is_err());
        assert_eq!(
            type_().parse(stream("Number", "")).unwrap().0,
            types::Number::new(SourceInformation::dummy()).into()
        );
        assert_eq!(
            type_().parse(stream("Number -> Number", "")).unwrap().0,
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .into()
        );
        assert_eq!(
            type_()
                .parse(stream("Number -> Number -> Number", ""))
                .unwrap()
                .0,
            types::Function::new(
                types::Number::new(SourceInformation::dummy()),
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy()
            )
            .into()
        );
        assert_eq!(
            type_()
                .parse(stream("(Number -> Number) -> Number", ""))
                .unwrap()
                .0,
            types::Function::new(
                types::Function::new(
                    types::Number::new(SourceInformation::dummy()),
                    types::Number::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                types::Number::new(SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .into()
        );
    }

    #[test]
    fn parse_expression() {
        assert!(expression().parse(stream("?", "")).is_err());
        assert_eq!(
            expression().parse(stream("1", "")).unwrap().0,
            Number::new(1.0, SourceInformation::dummy()).into()
        );
        assert_eq!(
            expression().parse(stream("x", "")).unwrap().0,
            Variable::new("x", SourceInformation::dummy()).into()
        );
        assert_eq!(
            expression().parse(stream("x + y z", "")).unwrap().0,
            Operation::new(
                Operator::Add,
                Variable::new("x", SourceInformation::dummy()),
                Application::new(
                    Variable::new("y", SourceInformation::dummy()),
                    Variable::new("z", SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy()
            )
            .into()
        );
        assert_eq!(
            expression().parse(stream("(x + y) z", "")).unwrap().0,
            Application::new(
                Operation::new(
                    Operator::Add,
                    Variable::new("x", SourceInformation::dummy()),
                    Variable::new("y", SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                Variable::new("z", SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .into()
        );
        assert_eq!(
            expression()
                .parse(stream(
                    indoc!(
                        "
                        (f x
                         )
                        "
                    ),
                    ""
                ))
                .unwrap()
                .0,
            Application::new(
                Variable::new("f", SourceInformation::dummy()),
                Variable::new("x", SourceInformation::dummy()),
                SourceInformation::dummy()
            )
            .into()
        );
    }

    #[test]
    fn parse_atomic_expression() {
        assert!(atomic_expression().parse(stream("?", "")).is_err());
        assert_eq!(
            atomic_expression().parse(stream("1", "")).unwrap().0,
            Number::new(1.0, SourceInformation::dummy()).into()
        );
        assert_eq!(
            atomic_expression().parse(stream("x", "")).unwrap().0,
            Variable::new("x", SourceInformation::dummy()).into()
        );
        assert_eq!(
            atomic_expression().parse(stream(" x", "")).unwrap().0,
            Variable::new("x", SourceInformation::dummy()).into()
        );
    }

    #[test]
    fn parse_let() {
        assert!(let_().parse(stream("let in 0", "")).is_err());
        assert_eq!(
            let_()
                .parse(stream("let x : Number\nx = 42 in x", ""))
                .unwrap()
                .0,
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
        );
        assert_eq!(
            let_().parse(stream("let x = 42 in x", "")).unwrap().0,
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
        );
        assert_eq!(
            let_().parse(stream("let\nx = 42 in x", "")).unwrap().0,
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
        );
        assert_eq!(
            let_().parse(stream("let\n x = 42 in x", "")).unwrap().0,
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
        );
        assert_eq!(
            let_().parse(stream("let f x = x in f", "")).unwrap().0,
            Let::new(
                vec![FunctionDefinition::new(
                    "f",
                    vec!["x".into()],
                    Variable::new("x", SourceInformation::dummy()),
                    types::Variable::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into()],
                Variable::new("f", SourceInformation::dummy())
            )
        );
        assert_eq!(
            let_()
                .parse(stream(
                    indoc!(
                        "
                        let
                            f x = x
                            y = (
                                f x
                            )
                        in
                            y
                        "
                    ),
                    ""
                ))
                .unwrap()
                .0,
            Let::new(
                vec![
                    FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        Variable::new("x", SourceInformation::dummy()),
                        types::Variable::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into(),
                    ValueDefinition::new(
                        "y",
                        Application::new(
                            Variable::new("f", SourceInformation::dummy()),
                            Variable::new("x", SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        types::Variable::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()
                ],
                Variable::new("y", SourceInformation::dummy())
            )
        );
        assert_eq!(
            let_()
                .parse(stream(
                    indoc!(
                        "
                        let
                            f x = g x
                        in
                            f
                        "
                    ),
                    ""
                ))
                .unwrap()
                .0,
            Let::new(
                vec![FunctionDefinition::new(
                    "f",
                    vec!["x".into()],
                    Application::new(
                        Variable::new("g", SourceInformation::dummy()),
                        Variable::new("x", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    types::Variable::new(SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),],
                Variable::new("f", SourceInformation::dummy())
            )
        );
        assert_eq!(
            let_()
                .parse(stream(
                    indoc!(
                        "
                        let
                            f x = g x
                            h x = i x
                        in
                            f
                        "
                    ),
                    ""
                ))
                .unwrap()
                .0,
            Let::new(
                vec![
                    FunctionDefinition::new(
                        "f",
                        vec!["x".into()],
                        Application::new(
                            Variable::new("g", SourceInformation::dummy()),
                            Variable::new("x", SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        types::Variable::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into(),
                    FunctionDefinition::new(
                        "h",
                        vec!["x".into()],
                        Application::new(
                            Variable::new("i", SourceInformation::dummy()),
                            Variable::new("x", SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        types::Variable::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()
                ],
                Variable::new("f", SourceInformation::dummy())
            )
        );
    }

    #[test]
    fn parse_application() {
        assert!(application().parse(stream("f", "")).is_err());
        assert_eq!(
            application().parse(stream("f 1", "")).unwrap().0,
            Application::new(
                Variable::new("f", SourceInformation::dummy()),
                Number::new(1.0, SourceInformation::dummy()),
                SourceInformation::dummy()
            )
        );
        assert_eq!(
            application().parse(stream("f x", "")).unwrap().0,
            Application::new(
                Variable::new("f", SourceInformation::dummy()),
                Variable::new("x", SourceInformation::dummy()),
                SourceInformation::dummy()
            )
        );
        assert_eq!(
            application().parse(stream("f 1 2", "")).unwrap().0,
            Application::new(
                Application::new(
                    Variable::new("f", SourceInformation::dummy()),
                    Number::new(1.0, SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                Number::new(2.0, SourceInformation::dummy()),
                SourceInformation::dummy()
            )
        );
        assert_eq!(
            application()
                .parse(stream(
                    indoc!(
                        "
                        f x
                        g x =
                        "
                    ),
                    ""
                ))
                .unwrap()
                .0,
            Application::new(
                Variable::new("f", SourceInformation::dummy()),
                Variable::new("x", SourceInformation::dummy()),
                SourceInformation::dummy()
            )
        );
        assert_eq!(
            application()
                .parse(stream(
                    indoc!(
                        "
                        f x
                         g x =
                        "
                    ),
                    ""
                ))
                .unwrap()
                .0,
            Application::new(
                Variable::new("f", SourceInformation::dummy()),
                Variable::new("x", SourceInformation::dummy()),
                SourceInformation::dummy()
            )
        );
    }

    #[test]
    fn parse_application_terminator() {
        for source in &["", "\n", " \n", "\n\n", "+", ")", "\n)", "\n )"] {
            assert!(application_terminator().parse(stream(source, "")).is_ok());
        }
    }

    #[test]
    fn parse_operation() {
        assert!(application().parse(stream("1", "")).is_err());
        assert_eq!(
            operation().parse(stream("1 + 1", "")).unwrap().0,
            Operation::new(
                Operator::Add,
                Number::new(1.0, SourceInformation::dummy()),
                Number::new(1.0, SourceInformation::dummy()),
                SourceInformation::dummy()
            )
        );
        assert_eq!(
            operation().parse(stream("1 + 1 + 1", "")).unwrap().0,
            Operation::new(
                Operator::Add,
                Operation::new(
                    Operator::Add,
                    Number::new(1.0, SourceInformation::dummy()),
                    Number::new(1.0, SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                Number::new(1.0, SourceInformation::dummy()),
                SourceInformation::dummy()
            )
        );
        assert_eq!(
            operation().parse(stream("1 + (1 + 1)", "")).unwrap().0,
            Operation::new(
                Operator::Add,
                Number::new(1.0, SourceInformation::dummy()),
                Operation::new(
                    Operator::Add,
                    Number::new(1.0, SourceInformation::dummy()),
                    Number::new(1.0, SourceInformation::dummy()),
                    SourceInformation::dummy()
                ),
                SourceInformation::dummy()
            )
        );
        assert_eq!(
            operation().parse(stream("1 * 2 - 3", "")).unwrap().0,
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
        );
        assert_eq!(
            operation().parse(stream("1 + 2 * 3", "")).unwrap().0,
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
        );
        assert_eq!(
            operation().parse(stream("1 * 2 - 3 / 4", "")).unwrap().0,
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
        );
    }

    #[test]
    fn parse_operator() {
        assert!(operator().parse(stream("", "")).is_err());
        assert!(operator().parse(stream("++", "")).is_err());
        assert_eq!(operator().parse(stream("+", "")).unwrap().0, Operator::Add);
    }

    #[test]
    fn parse_variable() {
        assert!(variable().parse(stream("Foo. x", "")).is_err());
        assert_eq!(
            variable().parse(stream("x", "")).unwrap().0,
            Variable::new("x", SourceInformation::dummy()),
        );
        assert_eq!(
            variable().parse(stream("Foo.x", "")).unwrap().0,
            Variable::new("Foo.x", SourceInformation::dummy()),
        );
        assert_eq!(
            variable().parse(stream("Foo .x", "")).unwrap().0,
            Variable::new("Foo", SourceInformation::dummy()),
        );
    }

    #[test]
    fn parse_number_literal() {
        assert!(number_literal().parse(stream("", "")).is_err());
        assert!(number_literal().parse(stream("foo", "")).is_err());
        assert!(number_literal().parse(stream("x1", "")).is_err());

        for (source, value) in &[
            ("01", 0.0),
            ("0", 0.0),
            ("1", 1.0),
            ("123456789", 123456789.0),
            ("-1", -1.0),
            ("0.1", 0.1),
            ("0.01", 0.01),
        ] {
            assert_eq!(
                number_literal().parse(stream(source, "")).unwrap().0,
                Number::new(*value, SourceInformation::dummy())
            );
        }
    }

    #[test]
    fn parse_identifier() {
        assert!(identifier().parse(stream("let", "")).is_err());
        assert!(identifier().parse(stream("1foo", "")).is_err());
        assert_eq!(
            identifier().parse(stream("foo", "")).unwrap().0,
            "foo".to_string()
        );
        assert_eq!(
            identifier().parse(stream("foo1", "")).unwrap().0,
            "foo1".to_string()
        );
        assert_eq!(
            identifier().parse(stream(" foo", "")).unwrap().0,
            "foo".to_string()
        );
    }

    #[test]
    fn parse_keyword() {
        assert!(keyword("foo").parse(stream("bar", "")).is_err());
        assert!(keyword("foo").parse(stream("fool", "")).is_err());
        assert!(keyword("foo").parse(stream("foo", "")).is_ok());
        assert!(keyword("foo").parse(stream(" foo", "")).is_ok());
    }

    #[test]
    fn parse_sign() {
        assert!(sign("+").parse(stream("", "")).is_err());
        assert!(sign("+").parse(stream("-", "")).is_err());
        assert!(sign("+").parse(stream("+", "")).is_ok());
        assert!(sign("+").parse(stream(" +", "")).is_ok());
        assert!(sign("+").parse(stream(" +x", "")).is_ok());
    }

    #[test]
    fn parse_source_information() {
        assert!(source_information()
            .with(combine::eof())
            .parse(stream(" \n \n \n", ""))
            .is_ok());
    }

    #[test]
    fn parse_blank() {
        for source in &[
            "",
            " ",
            "  ",
            "\n",
            "\n\n",
            " \n",
            "\n ",
            " \n \n \n",
            "\n \n \n ",
        ] {
            assert!(blank().parse(stream(source, "")).is_ok());
        }
    }

    #[test]
    fn parse_spaces1() {
        assert!(spaces1()
            .with(combine::eof())
            .parse(stream("", ""))
            .is_err());

        for source in &[" ", "  ", "\t", "\r"] {
            assert!(spaces1()
                .with(combine::eof())
                .parse(stream(source, ""))
                .is_ok());
        }
    }

    #[test]
    fn parse_newlines1() {
        for source in &["", "\n", " \n", "\n\n", "#\n", " #\n"] {
            assert!(newlines1()
                .with(combine::eof())
                .parse(stream(source, ""))
                .is_ok());
        }
    }

    #[test]
    fn parse_comment() {
        assert!(comment().parse(stream("#\n", "")).is_ok());
        assert!(comment().parse(stream("#x\n", "")).is_ok());
    }
}
