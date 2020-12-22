use super::error::CompileError;
use super::error_type_configuration::ErrorTypeConfiguration;
use super::reference_type_resolver::ReferenceTypeResolver;
use super::type_canonicalizer::TypeCanonicalizer;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ExpressionTypeExtractor {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_canonicalizer: Arc<TypeCanonicalizer>,
    error_type_configuration: Arc<ErrorTypeConfiguration>,
}

impl ExpressionTypeExtractor {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_canonicalizer: Arc<TypeCanonicalizer>,
        error_type_configuration: Arc<ErrorTypeConfiguration>,
    ) -> Arc<Self> {
        Self {
            reference_type_resolver,
            type_canonicalizer,
            error_type_configuration,
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

                for variable_definition in let_.definitions() {
                    variables.insert(
                        variable_definition.name().into(),
                        variable_definition.type_().clone(),
                    );
                }

                self.extract(let_.expression(), &variables)?
            }
            Expression::LetError(let_) => {
                let mut variables = variables.clone();

                for variable_definition in let_.definitions() {
                    variables.insert(
                        variable_definition.name().into(),
                        variable_definition.type_().clone(),
                    );
                }

                self.type_canonicalizer.canonicalize(
                    &types::Union::new(
                        vec![
                            self.extract(let_.expression(), &variables)?,
                            types::Reference::new(
                                &self.error_type_configuration.error_type_name,
                                let_.source_information().clone(),
                            )
                            .into(),
                        ],
                        let_.source_information().clone(),
                    )
                    .into(),
                )?
            }
            Expression::LetRecursive(let_) => {
                let mut variables = variables.clone();

                for function_definition in let_.definitions() {
                    variables.insert(
                        function_definition.name().into(),
                        function_definition.type_().clone(),
                    );
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
    use super::super::error_type_configuration::ERROR_TYPE_CONFIGURATION;
    use super::super::reference_type_resolver::ReferenceTypeResolver;
    use super::super::type_equality_checker::TypeEqualityChecker;
    use super::*;
    use crate::debug::SourceInformation;
    use crate::package::Package;
    use crate::path::ModulePath;

    fn create_expression_type_extractor(module: &Module) -> Arc<ExpressionTypeExtractor> {
        let reference_type_resolver = ReferenceTypeResolver::new(module);
        let type_equality_checker = TypeEqualityChecker::new(reference_type_resolver.clone());
        let type_canonicalizer =
            TypeCanonicalizer::new(reference_type_resolver.clone(), type_equality_checker);

        ExpressionTypeExtractor::new(
            reference_type_resolver,
            type_canonicalizer,
            ERROR_TYPE_CONFIGURATION.clone(),
        )
    }

    #[test]
    fn extract_type_of_case_expression() {
        assert_eq!(
            create_expression_type_extractor(&Module::dummy()).extract(
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
            create_expression_type_extractor(&Module::dummy()).extract(
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
        assert_eq!(
            create_expression_type_extractor(&Module::dummy()).extract(
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
        let list_type = types::List::new(
            types::None::new(SourceInformation::dummy()),
            SourceInformation::dummy(),
        );

        assert_eq!(
            create_expression_type_extractor(&Module::dummy()).extract(
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
        assert_eq!(
            create_expression_type_extractor(&Module::dummy()).extract(
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

    #[test]
    fn extract_type_of_let_error() {
        let union_type = types::Union::new(
            vec![
                types::Number::new(SourceInformation::dummy()).into(),
                types::Reference::new(
                    &ERROR_TYPE_CONFIGURATION.error_type_name,
                    SourceInformation::dummy(),
                )
                .into(),
            ],
            SourceInformation::dummy(),
        );

        let module = Module::new(
            ModulePath::new(Package::new("", ""), vec![]),
            Export::new(Default::default()),
            vec![Import::new(
                ModuleInterface::new(
                    ModulePath::new(Package::new("m", ""), vec![]),
                    Default::default(),
                    vec![(
                        "Error".into(),
                        types::Record::new("Error", Default::default(), SourceInformation::dummy())
                            .into(),
                    )]
                    .into_iter()
                    .collect(),
                    Default::default(),
                    Default::default(),
                ),
                false,
            )],
            vec![],
            vec![],
        );

        assert_eq!(
            create_expression_type_extractor(&module).extract(
                &LetError::new(
                    vec![VariableDefinition::new(
                        "y",
                        Variable::new("x", SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    )],
                    ArithmeticOperation::new(
                        ArithmeticOperator::Add,
                        Variable::new("y", SourceInformation::dummy()),
                        Number::new(42.0, SourceInformation::dummy()),
                        SourceInformation::dummy(),
                    ),
                    SourceInformation::dummy(),
                )
                .into(),
                &vec![("x".into(), union_type.clone().into())]
                    .into_iter()
                    .collect(),
            ),
            Ok(union_type.into())
        );
    }
}
