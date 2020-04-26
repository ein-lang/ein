use super::super::error::CompileError;
use super::super::expression_type_extractor::ExpressionTypeExtractor;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::super::type_equality_checker::TypeEqualityChecker;
use super::typed_meta_desugarer::TypedDesugarer;
use crate::ast::*;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

pub struct TypeCoercionDesugarer<'a> {
    reference_type_resolver: &'a ReferenceTypeResolver,
    type_equality_checker: &'a TypeEqualityChecker<'a>,
    expression_type_extractor: &'a ExpressionTypeExtractor<'a>,
}

impl<'a> TypeCoercionDesugarer<'a> {
    pub fn new(
        reference_type_resolver: &'a ReferenceTypeResolver,
        type_equality_checker: &'a TypeEqualityChecker<'a>,
        expression_type_extractor: &'a ExpressionTypeExtractor<'a>,
    ) -> Self {
        Self {
            reference_type_resolver,
            type_equality_checker,
            expression_type_extractor,
        }
    }

    fn coerce_type(
        &mut self,
        expression: &Expression,
        to_type: &Type,
        source_information: Rc<SourceInformation>,
        variables: &HashMap<String, Type>,
    ) -> Result<Expression, CompileError> {
        let from_type = self.reference_type_resolver.resolve(
            &self
                .expression_type_extractor
                .extract(&expression, variables)?,
        )?;
        let to_type = self.reference_type_resolver.resolve(to_type)?;

        if !to_type.is_union() && !self.type_equality_checker.equal(&from_type, &to_type)? {
            unreachable!()
        }

        Ok(if self.type_equality_checker.equal(&from_type, &to_type)? {
            expression.clone()
        } else {
            TypeCoercion::new(expression.clone(), from_type, to_type, source_information).into()
        })
    }
}

impl<'a> TypedDesugarer for TypeCoercionDesugarer<'a> {
    fn desugar_function_definition(
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

    fn desugar_value_definition(
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

    fn desugar_expression(
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
            Expression::RecordConstruction(record_construction) => {
                let type_ = self
                    .reference_type_resolver
                    .resolve_reference(record_construction.type_())?;
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
            | Expression::Operation(_)
            | Expression::Variable(_) => Ok(expression.clone()),
            Expression::RecordUpdate(_) | Expression::TypeCoercion(_) => unreachable!(),
        }
    }
}
