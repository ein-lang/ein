use super::super::error::CompileError;
use super::super::expression_type_extractor::ExpressionTypeExtractor;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::super::type_canonicalizer::TypeCanonicalizer;
use super::super::type_equality_checker::TypeEqualityChecker;
use super::typed_meta_transformer::TypedTransformer;
use crate::ast::*;
use crate::debug::SourceInformation;
use crate::types::{self, Type};
use std::collections::HashMap;
use std::sync::Arc;

/// TypeCoercionTransformer transforms value-to-union, function-to-union and
/// value-to-any type coercions.
/// Note that it does not transform function-to-function ones.
pub struct TypeCoercionTransformer {
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    type_equality_checker: Arc<TypeEqualityChecker>,
    expression_type_extractor: Arc<ExpressionTypeExtractor>,
    type_canonicalizer: Arc<TypeCanonicalizer>,
}

impl TypeCoercionTransformer {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        type_equality_checker: Arc<TypeEqualityChecker>,
        expression_type_extractor: Arc<ExpressionTypeExtractor>,
        type_canonicalizer: Arc<TypeCanonicalizer>,
    ) -> Self {
        Self {
            reference_type_resolver,
            type_equality_checker,
            expression_type_extractor,
            type_canonicalizer,
        }
    }

    fn coerce_type(
        &mut self,
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

        if !to_type.is_union()
            && !to_type.is_any()
            && !self.type_equality_checker.equal(&from_type, &to_type)?
        {
            unreachable!()
        }

        Ok(if self.type_equality_checker.equal(&from_type, &to_type)? {
            expression.clone()
        } else {
            TypeCoercion::new(expression.clone(), from_type, to_type, source_information).into()
        })
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
            function_definition
                .type_()
                .to_function()
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
                function_definition
                    .type_()
                    .to_function()
                    .unwrap()
                    .last_result(),
                function_definition.source_information().clone(),
                &variables,
            )?,
            function_definition.type_().clone(),
            function_definition.source_information().clone(),
        ))
    }

    fn transform_value_definition(
        &mut self,
        value_definition: &ValueDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<ValueDefinition, CompileError> {
        Ok(ValueDefinition::new(
            value_definition.name(),
            self.coerce_type(
                value_definition.body(),
                value_definition.type_(),
                value_definition.source_information().clone(),
                &variables,
            )?,
            value_definition.type_().clone(),
            value_definition.source_information().clone(),
        ))
    }

    fn transform_expression(
        &mut self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Expression, CompileError> {
        match expression {
            Expression::Application(application) => {
                let source_information = application.source_information();

                Ok(Application::new(
                    application.function().clone(),
                    self.coerce_type(
                        application.argument(),
                        self.expression_type_extractor
                            .extract(application.function(), variables)?
                            .to_function()
                            .unwrap()
                            .argument(),
                        source_information.clone(),
                        variables,
                    )?,
                    source_information.clone(),
                )
                .into())
            }
            Expression::Case(case) => {
                let result_type = self
                    .expression_type_extractor
                    .extract(expression, variables)?;

                Ok(Case::with_type(
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
                .into())
            }
            Expression::If(if_) => {
                let result_type = self
                    .expression_type_extractor
                    .extract(expression, variables)?;

                Ok(If::new(
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
                .into())
            }
            Expression::Operation(operation) => {
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

                Ok(Operation::with_type(
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
                .into())
            }
            Expression::RecordConstruction(record_construction) => {
                let type_ = self
                    .reference_type_resolver
                    .resolve(record_construction.type_())?;
                let record_type = type_.to_record().unwrap();

                Ok(RecordConstruction::new(
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
                .into())
            }
            Expression::Boolean(_)
            | Expression::Let(_)
            | Expression::None(_)
            | Expression::Number(_)
            | Expression::RecordElementOperation(_)
            | Expression::Variable(_) => Ok(expression.clone()),
            Expression::List(_) | Expression::RecordUpdate(_) | Expression::TypeCoercion(_) => {
                unreachable!()
            }
        }
    }
}