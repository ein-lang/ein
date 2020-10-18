use super::super::error::CompileError;
use super::super::list_type_configuration::ListTypeConfiguration;
use super::super::name_generator::NameGenerator;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::super::type_comparability_checker::TypeComparabilityChecker;
use super::super::type_equality_checker::TypeEqualityChecker;
use crate::ast::*;
use crate::debug::SourceInformation;
use crate::types::{self, Type};
use std::sync::Arc;

pub struct EqualOperationTransformer {
    name_generator: NameGenerator,
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_comparability_checker: Arc<TypeComparabilityChecker>,
    type_equality_checker: Arc<TypeEqualityChecker>,
    list_type_configuration: Arc<ListTypeConfiguration>,
}

impl EqualOperationTransformer {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_comparability_checker: Arc<TypeComparabilityChecker>,
        type_equality_checker: Arc<TypeEqualityChecker>,
        list_type_configuration: Arc<ListTypeConfiguration>,
    ) -> Self {
        Self {
            name_generator: NameGenerator::new("equal_operation_argument_"),
            reference_type_resolver,
            type_comparability_checker,
            type_equality_checker,
            list_type_configuration,
        }
    }

    pub fn transform(&mut self, expression: &Expression) -> Result<Expression, CompileError> {
        Ok(if let Expression::Operation(operation) = expression {
            if operation.operator() == Operator::Equal {
                self.transform_equal_operation(
                    operation.type_(),
                    operation.lhs(),
                    operation.rhs(),
                    operation.source_information().clone(),
                )?
            } else {
                expression.clone()
            }
        } else {
            expression.clone()
        })
    }

    fn transform_equal_operation(
        &mut self,
        type_: &Type,
        lhs: &Expression,
        rhs: &Expression,
        source_information: Arc<SourceInformation>,
    ) -> Result<Expression, CompileError> {
        Ok(match self.reference_type_resolver.resolve(type_)? {
            Type::Any(_) => return Err(CompileError::AnyEqualOperation(source_information)),
            Type::Boolean(_) => If::new(
                lhs.clone(),
                If::new(
                    rhs.clone(),
                    Boolean::new(true, source_information.clone()),
                    Boolean::new(false, source_information.clone()),
                    source_information.clone(),
                ),
                If::new(
                    rhs.clone(),
                    Boolean::new(false, source_information.clone()),
                    Boolean::new(true, source_information.clone()),
                    source_information.clone(),
                ),
                source_information,
            )
            .into(),
            Type::Function(_) => {
                return Err(CompileError::FunctionEqualOperation(source_information))
            }
            Type::List(list_type) => {
                let element_type = list_type.element();

                Let::new(
                    vec![FunctionDefinition::new(
                        "$equalElements",
                        vec!["lhs".into(), "rhs".into()],
                        Case::with_type(
                            types::Any::new(source_information.clone()),
                            "lhs",
                            Variable::new("lhs", source_information.clone()),
                            vec![
                                Alternative::new(
                                    element_type.clone(),
                                    Case::with_type(
                                        types::Any::new(source_information.clone()),
                                        "rhs",
                                        Variable::new("rhs", source_information.clone()),
                                        vec![
                                            Alternative::new(
                                                element_type.clone(),
                                                self.transform_equal_operation(
                                                    element_type,
                                                    &Variable::new(
                                                        "lhs",
                                                        source_information.clone(),
                                                    )
                                                    .into(),
                                                    &Variable::new(
                                                        "rhs",
                                                        source_information.clone(),
                                                    )
                                                    .into(),
                                                    source_information.clone(),
                                                )?,
                                            ),
                                            Alternative::new(
                                                types::Any::new(source_information.clone()),
                                                Boolean::new(false, source_information.clone()),
                                            ),
                                        ],
                                        source_information.clone(),
                                    ),
                                ),
                                Alternative::new(
                                    types::Any::new(source_information.clone()),
                                    Boolean::new(false, source_information.clone()),
                                ),
                            ],
                            source_information.clone(),
                        ),
                        types::Function::new(
                            types::Any::new(source_information.clone()),
                            types::Function::new(
                                types::Any::new(source_information.clone()),
                                types::Boolean::new(source_information.clone()),
                                source_information.clone(),
                            ),
                            source_information.clone(),
                        ),
                        source_information.clone(),
                    )
                    .into()],
                    Application::new(
                        Application::new(
                            Application::new(
                                Variable::new(
                                    self.list_type_configuration.equal_function_name(),
                                    source_information.clone(),
                                ),
                                Variable::new("$equalElements", source_information.clone()),
                                source_information.clone(),
                            ),
                            lhs.clone(),
                            source_information.clone(),
                        ),
                        rhs.clone(),
                        source_information,
                    ),
                )
                .into()
            }
            Type::None(_) => Boolean::new(true, source_information).into(),
            Type::Number(_) => Operation::with_type(
                type_.clone(),
                Operator::Equal,
                lhs.clone(),
                rhs.clone(),
                source_information,
            )
            .into(),
            Type::Record(record) => {
                if self.type_comparability_checker.check(type_)? {
                    Application::new(
                        Application::new(
                            Variable::new(
                                self.get_record_equal_function_name(&record),
                                source_information.clone(),
                            ),
                            lhs.clone(),
                            source_information.clone(),
                        ),
                        rhs.clone(),
                        source_information,
                    )
                    .into()
                } else {
                    return Err(CompileError::RecordEqualOperation(source_information));
                }
            }
            Type::Union(union) => {
                let lhs_name = self.name_generator.generate();
                let rhs_name = self.name_generator.generate();

                Case::with_type(
                    union.clone(),
                    &lhs_name,
                    lhs.clone(),
                    union
                        .types()
                        .iter()
                        .map(|lhs_type| {
                            Ok(Alternative::new(
                                lhs_type.clone(),
                                Case::with_type(
                                    union.clone(),
                                    &rhs_name,
                                    rhs.clone(),
                                    union
                                        .types()
                                        .iter()
                                        .map(|rhs_type| {
                                            Ok(Alternative::new(
                                                rhs_type.clone(),
                                                if self
                                                    .type_equality_checker
                                                    .equal(lhs_type, rhs_type)?
                                                {
                                                    self.transform_equal_operation(
                                                        rhs_type,
                                                        &Variable::new(
                                                            &lhs_name,
                                                            source_information.clone(),
                                                        )
                                                        .into(),
                                                        &Variable::new(
                                                            &rhs_name,
                                                            source_information.clone(),
                                                        )
                                                        .into(),
                                                        source_information.clone(),
                                                    )?
                                                } else {
                                                    Boolean::new(false, source_information.clone())
                                                        .into()
                                                },
                                            ))
                                        })
                                        .collect::<Result<_, CompileError>>()?,
                                    source_information.clone(),
                                ),
                            ))
                        })
                        .collect::<Result<_, CompileError>>()?,
                    source_information,
                )
                .into()
            }
            Type::Reference(_) | Type::Unknown(_) | Type::Variable(_) => unreachable!(),
        })
    }

    // TODO Share this function with RecordEqualFunctionTransformer.
    fn get_record_equal_function_name(&self, record_type: &types::Record) -> String {
        format!("{}.$equal", record_type.name())
    }
}
