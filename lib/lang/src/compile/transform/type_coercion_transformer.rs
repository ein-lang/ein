use super::super::error::CompileError;
use super::super::expression_type_extractor::ExpressionTypeExtractor;
use super::super::last_result_type_calculator::LastResultTypeCalculator;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::super::type_canonicalizer::TypeCanonicalizer;
use super::super::type_equality_checker::TypeEqualityChecker;
use super::typed_meta_transformer::TypedTransformer;
use crate::ast::*;
use crate::debug::SourceInformation;
use crate::types::{self, Type};
use std::collections::HashMap;
use std::sync::Arc;

/// TypeCoercionTransformer transforms value-to-union, function-to-union,
/// value-to-any, function-to-any and function-to-function type coercions.
pub struct TypeCoercionTransformer {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_equality_checker: Arc<TypeEqualityChecker>,
    expression_type_extractor: Arc<ExpressionTypeExtractor>,
    type_canonicalizer: Arc<TypeCanonicalizer>,
    last_result_type_calculator: Arc<LastResultTypeCalculator>,
}

impl TypeCoercionTransformer {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_equality_checker: Arc<TypeEqualityChecker>,
        expression_type_extractor: Arc<ExpressionTypeExtractor>,
        type_canonicalizer: Arc<TypeCanonicalizer>,
        last_result_type_calculator: Arc<LastResultTypeCalculator>,
    ) -> Self {
        Self {
            reference_type_resolver,
            type_equality_checker,
            expression_type_extractor,
            type_canonicalizer,
            last_result_type_calculator,
        }
    }

    fn coerce_type(
        &self,
        expression: &Expression,
        to_type: &Type,
        source_information: Arc<SourceInformation>,
        variables: &HashMap<String, Type>,
    ) -> Result<Expression, CompileError> {
        let from_type = self.reference_type_resolver.resolve(
            &self
                .expression_type_extractor
                .extract(&expression, variables)?,
        )?;
        let to_type = self.reference_type_resolver.resolve(to_type)?;

        Ok(
            if self.type_equality_checker.equal(&from_type, &to_type)?
                || self.reference_type_resolver.is_list(&from_type)?
                    && self.reference_type_resolver.is_list(&to_type)?
            {
                expression.clone()
            } else {
                TypeCoercion::new(expression.clone(), from_type, to_type, source_information).into()
            },
        )
    }
}

impl TypedTransformer for TypeCoercionTransformer {
    fn transform_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<FunctionDefinition, CompileError> {
        let mut variables = variables.clone();

        for (name, type_) in function_definition.arguments().iter().zip(
            self.reference_type_resolver
                .resolve_to_function(function_definition.type_())?
                .unwrap()
                .arguments(),
        ) {
            variables.insert(name.into(), type_.clone());
        }

        Ok(FunctionDefinition::new(
            function_definition.name(),
            function_definition.arguments().to_vec(),
            self.coerce_type(
                function_definition.body(),
                &self.last_result_type_calculator.calculate(
                    function_definition.type_(),
                    function_definition.arguments().len(),
                )?,
                function_definition.source_information().clone(),
                &variables,
            )?,
            function_definition.type_().clone(),
            function_definition.source_information().clone(),
        ))
    }

    fn transform_variable_definition(
        &mut self,
        variable_definition: &VariableDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<VariableDefinition, CompileError> {
        Ok(VariableDefinition::new(
            variable_definition.name(),
            self.coerce_type(
                variable_definition.body(),
                variable_definition.type_(),
                variable_definition.source_information().clone(),
                &variables,
            )?,
            variable_definition.type_().clone(),
            variable_definition.source_information().clone(),
        ))
    }

    fn transform_expression(
        &mut self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Expression, CompileError> {
        Ok(match expression {
            Expression::Application(application) => {
                let source_information = application.source_information();

                Application::new(
                    application.function().clone(),
                    self.coerce_type(
                        application.argument(),
                        self.reference_type_resolver
                            .resolve_to_function(
                                &self
                                    .expression_type_extractor
                                    .extract(application.function(), variables)?,
                            )?
                            .unwrap()
                            .argument(),
                        source_information.clone(),
                        variables,
                    )?,
                    source_information.clone(),
                )
                .into()
            }
            Expression::Case(case) => {
                let result_type = self
                    .expression_type_extractor
                    .extract(expression, variables)?;

                Case::with_type(
                    case.type_().clone(),
                    case.name(),
                    self.coerce_type(
                        case.argument(),
                        case.type_(),
                        case.source_information().clone(),
                        &variables,
                    )?,
                    case.alternatives()
                        .iter()
                        .map(|alternative| {
                            let mut variables = variables.clone();

                            variables.insert(case.name().into(), alternative.type_().clone());

                            Ok(Alternative::new(
                                alternative.type_().clone(),
                                self.coerce_type(
                                    alternative.expression(),
                                    &result_type,
                                    case.source_information().clone(),
                                    &variables,
                                )?,
                            ))
                        })
                        .collect::<Result<_, CompileError>>()?,
                    case.source_information().clone(),
                )
                .into()
            }
            Expression::If(if_) => {
                let result_type = self
                    .expression_type_extractor
                    .extract(expression, variables)?;

                If::new(
                    if_.condition().clone(),
                    self.coerce_type(
                        if_.then(),
                        &result_type,
                        if_.source_information().clone(),
                        variables,
                    )?,
                    self.coerce_type(
                        if_.else_(),
                        &result_type,
                        if_.source_information().clone(),
                        variables,
                    )?,
                    if_.source_information().clone(),
                )
                .into()
            }
            Expression::LetError(let_) => {
                let mut variables = variables.clone();

                for variable_definition in let_.definitions() {
                    variables.insert(
                        variable_definition.name().into(),
                        variable_definition.type_().clone(),
                    );
                }

                LetError::with_type(
                    let_.type_().clone(),
                    let_.definitions().to_vec(),
                    self.coerce_type(
                        let_.expression(),
                        &self
                            .expression_type_extractor
                            .extract(expression, &variables)?,
                        let_.expression().source_information().clone(),
                        &variables,
                    )?,
                    let_.source_information().clone(),
                )
                .into()
            }
            Expression::ListCase(case) => {
                let result_type = self
                    .expression_type_extractor
                    .extract(expression, variables)?;

                ListCase::new(
                    self.coerce_type(
                        case.argument(),
                        case.type_(),
                        case.source_information().clone(),
                        &variables,
                    )?,
                    case.type_().clone(),
                    case.first_name(),
                    case.rest_name(),
                    self.coerce_type(
                        case.empty_alternative(),
                        &result_type,
                        case.source_information().clone(),
                        &variables,
                    )?,
                    {
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

                        self.coerce_type(
                            case.non_empty_alternative(),
                            &result_type,
                            case.source_information().clone(),
                            &variables,
                        )?
                    },
                    case.source_information().clone(),
                )
                .into()
            }
            Expression::Operation(operation) => match operation {
                Operation::Equality(operation) => {
                    let argument_type = self.type_canonicalizer.canonicalize(
                        &types::Union::new(
                            vec![
                                self.expression_type_extractor
                                    .extract(operation.lhs(), variables)?,
                                self.expression_type_extractor
                                    .extract(operation.rhs(), variables)?,
                            ],
                            operation.source_information().clone(),
                        )
                        .into(),
                    )?;

                    EqualityOperation::with_type(
                        operation.type_().clone(),
                        operation.operator(),
                        self.coerce_type(
                            operation.lhs(),
                            &argument_type,
                            operation.source_information().clone(),
                            variables,
                        )?,
                        self.coerce_type(
                            operation.rhs(),
                            &argument_type,
                            operation.source_information().clone(),
                            variables,
                        )?,
                        operation.source_information().clone(),
                    )
                    .into()
                }
                Operation::Arithmetic(_)
                | Operation::Boolean(_)
                | Operation::Order(_)
                | Operation::Pipe(_) => operation.clone().into(),
            },
            Expression::RecordConstruction(record_construction) => {
                let record_type = self
                    .reference_type_resolver
                    .resolve_to_record(record_construction.type_())?
                    .unwrap();

                RecordConstruction::new(
                    record_construction.type_().clone(),
                    record_construction
                        .elements()
                        .iter()
                        .map(|(key, expression)| {
                            Ok((
                                key.clone(),
                                self.coerce_type(
                                    expression,
                                    &record_type.elements()[key],
                                    record_construction.source_information().clone(),
                                    variables,
                                )?,
                            ))
                        })
                        .collect::<Result<_, CompileError>>()?,
                    record_construction.source_information().clone(),
                )
                .into()
            }
            Expression::Boolean(_)
            | Expression::Let(_)
            | Expression::LetRecursive(_)
            | Expression::List(_)
            | Expression::None(_)
            | Expression::Number(_)
            | Expression::RecordElementOperation(_)
            | Expression::String(_)
            | Expression::Variable(_) => expression.clone(),
            Expression::RecordUpdate(_) | Expression::TypeCoercion(_) => unreachable!(),
        })
    }
}
