use super::utilities;
use crate::ast::*;
use crate::debug::*;
use crate::path::*;
use crate::types::{self, Type};
use combine::error::Info;
use combine::parser::char::{alpha_num, letter, newline, spaces, string};
use combine::parser::choice::optional;
use combine::parser::combinator::{lazy, look_ahead, no_partial, not_followed_by};
use combine::parser::regex::find;
use combine::parser::repeat::{many, many1};
use combine::parser::sequence::between;
use combine::stream::position::{self, SourcePosition};
use combine::stream::state;
use combine::{
    attempt, choice, easy, eof, from_str, none_of, one_of, sep_by1, sep_end_by1, unexpected_any,
    value, Parser, Positioned,
};
use std::rc::Rc;

const KEYWORDS: &[&str] = &["export", "import", "in", "let"];

lazy_static! {
    static ref NUMBER_REGEX: regex::Regex =
        regex::Regex::new(r"^-?([123456789][0123456789]*|0)(\.[0123456789]+)?").unwrap();
    static ref SPACES1_REGEX: regex::Regex = regex::Regex::new("^[ \t\r]+").unwrap();
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
    (optional(export()), many(import()), many(definition()))
        .skip(blank())
        .skip(eof())
        .map(|(export, imports, definitions)| {
            UnresolvedModule::new(
                export.unwrap_or_else(|| Export::new(Default::default())),
                imports,
                definitions,
            )
        })
}

fn export<'a>() -> impl Parser<Stream<'a>, Output = Export> {
    keyword("export")
        .with(between(
            keyword("{"),
            keyword("}"),
            sep_end_by1(identifier(), keyword(",")),
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
}

fn function_definition<'a>() -> impl Parser<Stream<'a>, Output = FunctionDefinition> {
    attempt((
        source_information(),
        type_annotation(),
        identifier(),
        many1(identifier()),
        keyword("="),
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
        keyword("=").with(expression()),
    ))
    .then(
        |(source_information, (typed_name, type_), name, expression)| {
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
    (identifier(), keyword(":").with(type_()))
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
        keyword("="),
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
    attempt((
        source_information(),
        identifier(),
        keyword("="),
        expression(),
    ))
    .map(|(source_information, name, _, expression)| {
        let source_information = Rc::new(source_information);
        ValueDefinition::new(
            name,
            expression,
            types::Variable::new(source_information.clone()),
            source_information,
        )
    })
}

fn type_<'a>() -> impl Parser<Stream<'a>, Output = Type> {
    lazy(|| no_partial(choice((function_type().map(Type::from), atomic_type())))).boxed()
}

fn function_type<'a>() -> impl Parser<Stream<'a>, Output = types::Function> {
    attempt((source_information(), atomic_type(), keyword("->"), type_())).map(
        |(source_information, argument, _, result)| {
            types::Function::new(argument, result, source_information)
        },
    )
}

fn atomic_type<'a>() -> impl Parser<Stream<'a>, Output = Type> {
    choice((
        number_type().map(Type::from),
        between(keyword("("), keyword(")"), type_()),
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
        between(keyword("("), keyword(")"), expression()),
    ))
}

fn let_<'a>() -> impl Parser<Stream<'a>, Output = Let> {
    attempt((
        keyword("let"),
        many1(definition().or(untyped_definition())),
        keyword("in"),
        expression(),
    ))
    .map(|(_, definitions, _, expression)| Let::new(definitions, expression))
}

fn application<'a>() -> impl Parser<Stream<'a>, Output = Application> {
    attempt((
        source_information(),
        atomic_expression(),
        many1((
            many(attempt(
                atomic_expression().skip(not_followed_by(application_terminator())),
            )),
            atomic_expression().skip(look_ahead(application_terminator())),
        )),
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
    choice((newlines1(), keyword(")"), operator().with(value(()))))
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
        keyword("+").with(value(Operator::Add)),
        keyword("-").with(value(Operator::Subtract)),
        keyword("*").with(value(Operator::Multiply)),
        keyword("/").with(value(Operator::Divide)),
    ))
}

fn number_literal<'a>() -> impl Parser<Stream<'a>, Output = Number> {
    let regex: &'static regex::Regex = &NUMBER_REGEX;
    token((source_information(), from_str(find(regex))))
        .map(|(source_information, number)| Number::new(number, source_information))
}

fn variable<'a>() -> impl Parser<Stream<'a>, Output = Variable> {
    token((
        source_information(),
        optional(attempt((identifier(), keyword(".")))),
        identifier(),
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
    token(
        (many1(letter()), many(alpha_num()))
            .map(|(head, tail): (String, String)| [head, tail].concat())
            .then(|identifier| {
                if KEYWORDS.iter().any(|keyword| &identifier == keyword) {
                    unexpected_any("keyword").right()
                } else {
                    value(identifier).left()
                }
            }),
    )
}

fn keyword<'a>(name: &'static str) -> impl Parser<Stream<'a>, Output = ()> {
    token(string(name)).with(value(()))
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
    // TODO
    spaces().silent()
}

fn spaces1<'a>() -> impl Parser<Stream<'a>, Output = ()> {
    let regex: &'static regex::Regex = &SPACES1_REGEX;
    find(regex).with(value(()))
}

fn newlines1<'a>() -> impl Parser<Stream<'a>, Output = ()> {
    let spaces0 = || optional(spaces1());
    let newlines = || many1(spaces0().with(choice((newline().with(value(())), comment()))));
    newlines().or(optional(newlines()).with(spaces0()).with(eof()))
}

fn comment<'a>() -> impl Parser<Stream<'a>, Output = ()> {
    string("#")
        .with(many::<Vec<_>, _, _>(none_of("\n".chars())))
        .with(newline())
        .with(value(()))
}

#[cfg(test)]
mod tests {
    use super::*;

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
                vec![]
            )
        );
        assert_eq!(
            module().parse(stream("x : Number\nx = 42", "")).unwrap().0,
            UnresolvedModule::new(
                Export::new(Default::default()),
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
        assert_eq!(
            path_component().parse(stream("foo", "")).unwrap().0,
            "foo".to_string()
        );
        assert_eq!(
            path_component().parse(stream("github.com", "")).unwrap().0,
            "github.com".to_string()
        );
        assert_eq!(
            path_component().parse(stream("foo-rs", "")).unwrap().0,
            "foo-rs".to_string()
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
    fn parse_variable() {
        assert_eq!(
            variable().parse(stream("x", "")).unwrap().0,
            Variable::new("x", SourceInformation::dummy()),
        );
        assert_eq!(
            variable().parse(stream("Foo.x", "")).unwrap().0,
            Variable::new("Foo.x", SourceInformation::dummy()),
        );
    }

    #[test]
    fn parse_number_literal() {
        assert!(number_literal().parse(stream("", "")).is_err());
        assert!(number_literal().parse(stream("foo", "")).is_err());
        assert!(number_literal().parse(stream("x1", "")).is_err());
        assert_eq!(
            number_literal().parse(stream("01", "")).unwrap().0,
            Number::new(0.0, SourceInformation::dummy())
        );
        assert_eq!(
            number_literal().parse(stream("0", "")).unwrap().0,
            Number::new(0.0, SourceInformation::dummy())
        );
        assert_eq!(
            number_literal().parse(stream("1", "")).unwrap().0,
            Number::new(1.0, SourceInformation::dummy())
        );
        assert_eq!(
            number_literal().parse(stream("123456789", "")).unwrap().0,
            Number::new(123456789.0, SourceInformation::dummy())
        );
        assert_eq!(
            number_literal().parse(stream("-1", "")).unwrap().0,
            Number::new(-1.0, SourceInformation::dummy())
        );
        assert_eq!(
            number_literal().parse(stream("0.1", "")).unwrap().0,
            Number::new(0.1, SourceInformation::dummy())
        );
        assert_eq!(
            number_literal().parse(stream("0.01", "")).unwrap().0,
            Number::new(0.01, SourceInformation::dummy())
        );
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
        assert!(keyword("foo").parse(stream("foo", "")).is_ok());
        assert!(keyword("foo").parse(stream(" foo", "")).is_ok());
    }

    #[test]
    fn parse_spaces1() {
        assert!(spaces1().parse(stream("", "")).is_err());
        assert!(spaces1().parse(stream(" ", "")).is_ok());
        assert!(spaces1().parse(stream("  ", "")).is_ok());
        assert!(spaces1().parse(stream("\t", "")).is_ok());
        assert!(spaces1().parse(stream("\r", "")).is_ok());
    }

    #[test]
    fn parse_newlines1() {
        assert!(newlines1().parse(stream("", "")).is_ok());
        assert!(newlines1().parse(stream("\n", "")).is_ok());
        assert!(newlines1().parse(stream(" \n", "")).is_ok());
        assert!(newlines1().parse(stream("\n\n", "")).is_ok());
        assert!(newlines1().parse(stream("#\n", "")).is_ok());
        assert!(newlines1().parse(stream(" #\n", "")).is_ok());
    }

    #[test]
    fn parse_comment() {
        assert!(newlines1().parse(stream("#\n", "")).is_ok());
        assert!(newlines1().parse(stream("#x\n", "")).is_ok());
    }
}
