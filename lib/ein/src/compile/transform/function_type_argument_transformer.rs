use super::super::error::CompileError;
use super::super::expression_type_extractor::ExpressionTypeExtractor;
use super::super::name_generator::NameGenerator;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::typed_meta_transformer::TypedTransformer;
use crate::ast::*;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::HashMap;
use std::sync::Arc;

/// Transforms all arguments of function types into variables.
/// Those arguments can be partial applications or lambda expressions.
pub struct FunctionTypeArgumentTransformer {
    name_generator: NameGenerator,
    reference_type_resolver: Arc<ReferenceTypeResolver>,
    expression_type_extractor: Arc<ExpressionTypeExtractor>,
}

impl FunctionTypeArgumentTransformer {
    pub fn new(
        reference_type_resolver: Arc<ReferenceTypeResolver>,
        expression_type_extractor: Arc<ExpressionTypeExtractor>,
    ) -> Self {
        Self {
            name_generator: NameGenerator::new("fta_function_"),
            reference_type_resolver,
            expression_type_extractor,
        }
    }

    fn transform_function_type_argument(
        &mut self,
        expression: &Expression,
        to_type: &Type,
        source_information: Arc<SourceInformation>,
    ) -> Result<Expression, CompileError> {
        let to_type = self.reference_type_resolver.resolve(to_type)?;

        Ok(if to_type.is_function() && !expression.is_variable() {
            let name = self.name_generator.generate();

            Let::new(
                vec![ValueDefinition::new(
                    &name,
                    expression.clone(),
                    to_type,
                    source_information.clone(),
                )
                .into()],
                Variable::new(name, source_information),
            )
            .into()
        } else {
            expression.clone()
        })
    }
}

impl TypedTransformer for FunctionTypeArgumentTransformer {
    fn transform_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
        _: &HashMap<String, Type>,
    ) -> Result<FunctionDefinition, CompileError> {
        Ok(function_definition.clone())
    }

    fn transform_value_definition(
        &mut self,
        value_definition: &ValueDefinition,
        _: &HashMap<String, Type>,
    ) -> Result<ValueDefinition, CompileError> {
        Ok(value_definition.clone())
    }

    fn transform_expression(
        &mut self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Expression, CompileError> {
        match expression {
            Expression::Application(application) => {
                let source_information = application.source_information();
                let function_type = self
                    .expression_type_extractor
                    .extract(application.function(), variables)?;

                Ok(Application::new(
                    application.function().clone(),
                    self.transform_function_type_argument(
                        application.argument(),
                        function_type.to_function().unwrap().argument(),
                        source_information.clone(),
                    )?,
                    source_information.clone(),
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
                                self.transform_function_type_argument(
                                    expression,
                                    &record_type.elements()[key],
                                    record_construction.source_information().clone(),
                                )?,
                            ))
                        })
                        .collect::<Result<_, CompileError>>()?,
                    record_construction.source_information().clone(),
                )
                .into())
            }
            Expression::Boolean(_)
            | Expression::Case(_) // TODO Transform case expression arguments.
            | Expression::If(_)
            | Expression::Let(_)
            | Expression::List(_) // TODO Transform list elements.
            | Expression::None(_)
            | Expression::Number(_)
            | Expression::Operation(_) // There is no operation applicable to functions.
            | Expression::RecordElementOperation(_)
            | Expression::TypeCoercion(_) // TODO Transform type coercions.
            | Expression::Variable(_) => Ok(expression.clone()),
            Expression::RecordUpdate(_)  => unreachable!(),
        }
    }
}
