use super::error::CompileError;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_canonicalizer::TypeCanonicalizer;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ExpressionTypeExtractor {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_canonicalizer: Arc<TypeCanonicalizer>,
}

impl ExpressionTypeExtractor {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_canonicalizer: Arc<TypeCanonicalizer>,
    ) -> Arc<Self> {
        Self {
            reference_type_resolver,
            type_canonicalizer,
        }
        .into()
    }

    pub fn extract(
        &self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Type, CompileError> {
        Ok(match expression {
            Expression::Application(application) => self
                .reference_type_resolver
                .resolve_to_function(&self.extract(application.function(), variables)?)?
                .unwrap()
                .result()
                .clone(),
            Expression::Boolean(boolean) => {
                types::Boolean::new(boolean.source_information().clone()).into()
            }
            Expression::Case(case) => self.type_canonicalizer.canonicalize(
                &types::Union::new(
                    case.alternatives()
                        .iter()
                        .map(|alternative| {
                            let mut variables = variables.clone();

                            variables.insert(case.name().into(), alternative.type_().clone());

                            self.extract(alternative.expression(), &variables)
                        })
                        .collect::<Result<_, _>>()?,
                    case.source_information().clone(),
                )
                .into(),
            )?,
            Expression::If(if_) => self.type_canonicalizer.canonicalize(
                &types::Union::new(
                    vec![
                        self.extract(if_.then(), variables)?,
                        self.extract(if_.else_(), variables)?,
                    ],
                    if_.source_information().clone(),
                )
                .into(),
            )?,
            Expression::Let(let_) => {
                let mut variables = variables.clone();

                for definition in let_.definitions() {
                    match definition {
                        Definition::FunctionDefinition(function_definition) => {
                            variables.insert(
                                function_definition.name().into(),
                                function_definition.type_().clone(),
                            );
                        }
                        Definition::VariableDefinition(variable_definition) => {
                            variables.insert(
                                variable_definition.name().into(),
                                variable_definition.type_().clone(),
                            );
                        }
                    }
                }

                self.extract(let_.expression(), &variables)?
            }
            Expression::List(list) => list.type_().clone(),
            Expression::ListCase(case) => self.type_canonicalizer.canonicalize(
                &types::Union::new(
                    vec![self.extract(case.empty_alternative(), &variables)?, {
                        let mut variables = variables.clone();

                        variables.insert(
                            case.first_name().into(),
                            self.reference_type_resolver
                                .resolve_to_list(case.type_())?
                                .unwrap()
                                .element()
                                .clone(),
                        );
                        variables.insert(case.rest_name().into(), case.type_().clone());

                        self.extract(case.non_empty_alternative(), &variables)?
                    }],
                    case.source_information().clone(),
                )
                .into(),
            )?,
            Expression::None(none) => types::None::new(none.source_information().clone()).into(),
            Expression::Number(number) => {
                types::Number::new(number.source_information().clone()).into()
            }
            Expression::Operation(operation) => match operation {
                Operation::Arithmetic(_) => {
                    types::Number::new(operation.source_information().clone()).into()
                }
                Operation::Boolean(_) | Operation::Equality(_) | Operation::Order(_) => {
                    types::Boolean::new(operation.source_information().clone()).into()
                }
                Operation::Pipe(operation) => self
                    .reference_type_resolver
                    .resolve_to_function(&self.extract(operation.rhs(), &variables)?)?
                    .unwrap()
                    .result()
                    .clone(),
            },
            Expression::RecordConstruction(record) => record.type_().clone(),
            Expression::RecordElementOperation(operation) => {
                let mut variables = variables.clone();

                variables.insert(
                    operation.variable().into(),
                    self.reference_type_resolver
                        .resolve_to_record(operation.type_())?
                        .unwrap()
                        .elements()[operation.key()]
                    .clone(),
                );

                self.extract(operation.expression(), &variables)?
            }
            Expression::String(string) => {
                types::EinString::new(string.source_information().clone()).into()
            }
            Expression::TypeCoercion(coercion) => coercion.to().clone(),
            Expression::Variable(variable) => variables[variable.name()].clone(),
            Expression::RecordUpdate(_) => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::reference_type_resolver::ReferenceTypeResolver;
    use super::super::type_equality_checker::TypeEqualityChecker;
    use super::*;
    use crate::debug::SourceInformation;

    #[test]
    fn extract_type_of_case_expression() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let type_canonicalizer =
            TypeCanonicalizer::new(reference_type_resolver.clone(), type_equality_checker);

        assert_eq!(
            ExpressionTypeExtractor::new(reference_type_resolver, type_canonicalizer).extract(
                &Case::new(
                    "",
                    None::new(SourceInformation::dummy()),
                    vec![
                        Alternative::new(
                            types::Boolean::new(SourceInformation::dummy()),
                            Boolean::new(false, SourceInformation::dummy()),
                        ),
                        Alternative::new(
                            types::None::new(SourceInformation::dummy()),
                            None::new(SourceInformation::dummy()),
                        )
                    ],
                    SourceInformation::dummy()
                )
                .into(),
                &Default::default(),
            ),
            Ok(types::Union::new(
                vec![
                    types::Boolean::new(SourceInformation::dummy()).into(),
                    types::None::new(SourceInformation::dummy()).into()
                ],
                SourceInformation::dummy()
            )
            .into())
        );
    }

    #[test]
    fn extract_type_of_record_element_operation() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let type_canonicalizer =
            TypeCanonicalizer::new(reference_type_resolver.clone(), type_equality_checker);
        let record_type = types::Record::new(
            "Foo",
            vec![(
                "foo".into(),
                types::Number::new(SourceInformation::dummy()).into(),
            )]
            .into_iter()
            .collect(),
            SourceInformation::dummy(),
        );

        assert_eq!(
            ExpressionTypeExtractor::new(reference_type_resolver, type_canonicalizer).extract(
                &RecordElementOperation::new(
                    record_type.clone(),
                    "foo",
                    RecordConstruction::new(
                        record_type,
                        vec![(
                            "foo".into(),
                            Number::new(42.0, SourceInformation::dummy()).into()
                        )]
                        .into_iter()
                        .collect(),
                        SourceInformation::dummy()
                    ),
                    "bar",
                    OrderOperation::new(
                        OrderOperator::LessThan,
                        Variable::new("bar", SourceInformation::dummy()),
                        Variable::new("bar", SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    SourceInformation::dummy()
                )
                .into(),
                &Default::default(),
            ),
            Ok(types::Boolean::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn extract_type_of_list_case_expression_with_element() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let type_canonicalizer =
            TypeCanonicalizer::new(reference_type_resolver.clone(), type_equality_checker);

        assert_eq!(
            ExpressionTypeExtractor::new(reference_type_resolver, type_canonicalizer).extract(
                &ListCase::new(
                    List::new(vec![], SourceInformation::dummy()),
                    types::List::new(
                        types::None::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    ),
                    "x",
                    "xs",
                    None::new(SourceInformation::dummy()),
                    Variable::new("x", SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
                &Default::default(),
            ),
            Ok(types::None::new(SourceInformation::dummy()).into())
        );
    }

    #[test]
    fn extract_type_of_list_case_expression_with_list() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let type_canonicalizer =
            TypeCanonicalizer::new(reference_type_resolver.clone(), type_equality_checker);
        let list_type = types::List::new(
            types::None::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        );

        assert_eq!(
            ExpressionTypeExtractor::new(reference_type_resolver, type_canonicalizer).extract(
                &ListCase::new(
                    List::new(vec![], SourceInformation::dummy()),
                    list_type.clone(),
                    "x",
                    "xs",
                    List::with_type(list_type.clone(), vec![], SourceInformation::dummy()),
                    Variable::new("xs", SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
                &Default::default(),
            ),
            Ok(list_type.into())
        );
    }

    #[test]
    fn extract_type_of_pipe_operation() {
        let reference_type_resolver = ReferenceTypeResolver::new(&Module::dummy());
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let type_canonicalizer =
            TypeCanonicalizer::new(reference_type_resolver.clone(), type_equality_checker);

        assert_eq!(
            ExpressionTypeExtractor::new(reference_type_resolver, type_canonicalizer).extract(
                &PipeOperation::new(
                    None::new(SourceInformation::dummy()),
                    Variable::new("f", SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
                &vec![(
                    "f".into(),
                    types::Function::new(
                        types::None::new(SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()
                )]
                .into_iter()
                .collect(),
            ),
            Ok(types::Number::new(SourceInformation::dummy()).into())
        );
    }
}
