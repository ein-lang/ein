use super::super::error::CompileError;
use super::super::expression_type_extractor::ExpressionTypeExtractor;
use super::super::module_environment_creator::ModuleEnvironmentCreator;
use super::super::name_generator::NameGenerator;
use super::super::reference_type_resolver::ReferenceTypeResolver;
use super::super::type_equality_checker::TypeEqualityChecker;
use crate::ast::*;
use crate::debug::SourceInformation;
use crate::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

pub struct FunctionTypeArgumentDesugarer<'a> {
    name_generator: NameGenerator,
    reference_type_resolver: &'a ReferenceTypeResolver,
    type_equality_checker: &'a TypeEqualityChecker<'a>,
}

impl<'a> FunctionTypeArgumentDesugarer<'a> {
    pub fn new(
        reference_type_resolver: &'a ReferenceTypeResolver,
        type_equality_checker: &'a TypeEqualityChecker<'a>,
    ) -> Self {
        FunctionTypeArgumentDesugarer {
            name_generator: NameGenerator::new("fta_function_"),
            reference_type_resolver,
            type_equality_checker,
        }
    }

    pub fn desugar(&mut self, module: &Module) -> Result<Module, CompileError> {
        let variables = ModuleEnvironmentCreator::create(module);

        Ok(Module::new(
            module.path().clone(),
            module.export().clone(),
            module.imported_modules().to_vec(),
            module.type_definitions().to_vec(),
            module
                .definitions()
                .iter()
                .map(|definition| {
                    Ok(match definition {
                        Definition::FunctionDefinition(function_definition) => self
                            .desugar_function_definition(function_definition, &variables)?
                            .into(),
                        Definition::ValueDefinition(value_definition) => self
                            .desugar_value_definition(value_definition, &variables)?
                            .into(),
                    })
                })
                .collect::<Result<_, CompileError>>()?,
        ))
    }

    fn desugar_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<FunctionDefinition, CompileError> {
        let mut variables = variables.clone();
        let mut type_ = function_definition.type_();

        for argument_name in function_definition.arguments() {
            let function_type = type_.to_function().unwrap();

            variables.insert(argument_name.into(), function_type.argument().clone());

            type_ = function_type.result();
        }

        Ok(FunctionDefinition::new(
            function_definition.name(),
            function_definition.arguments().to_vec(),
            self.desugar_expression(function_definition.body(), &variables)?,
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
            self.desugar_expression(value_definition.body(), &variables)?,
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
                let function_type =
                    ExpressionTypeExtractor::extract(application.function(), variables);

                Ok(Application::new(
                    self.desugar_expression(application.function(), variables)?,
                    self.desugar_function_type_argument(
                        application.argument(),
                        function_type.to_function().unwrap().argument(),
                        source_information.clone(),
                        variables,
                    )?,
                    source_information.clone(),
                )
                .into())
            }
            Expression::If(if_) => Ok(If::new(
                self.desugar_expression(if_.condition(), variables)?,
                self.desugar_expression(if_.then(), variables)?,
                self.desugar_expression(if_.else_(), variables)?,
                if_.source_information().clone(),
            )
            .into()),
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
                        Definition::ValueDefinition(value_definition) => {
                            if let_.has_functions() {
                                variables.insert(
                                    value_definition.name().into(),
                                    value_definition.type_().clone(),
                                );
                            }
                        }
                    }
                }

                let mut definitions = vec![];

                for definition in let_.definitions() {
                    definitions.push(match definition {
                        Definition::FunctionDefinition(function_definition) => self
                            .desugar_function_definition(function_definition, &variables)?
                            .into(),
                        Definition::ValueDefinition(value_definition) => {
                            let definition =
                                self.desugar_value_definition(value_definition, &variables)?;

                            variables.insert(
                                value_definition.name().into(),
                                value_definition.type_().clone(),
                            );

                            definition.into()
                        }
                    })
                }

                Ok(Let::new(
                    definitions,
                    self.desugar_expression(let_.expression(), &variables)?,
                )
                .into())
            }
            Expression::Operation(operation) => Ok(Operation::new(
                operation.operator(),
                self.desugar_expression(operation.lhs(), &variables)?,
                self.desugar_expression(operation.rhs(), &variables)?,
                operation.source_information().clone(),
            )
            .into()),
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
                                self.desugar_function_type_argument(
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
            | Expression::None(_)
            | Expression::Number(_)
            | Expression::Variable(_) => Ok(expression.clone()),
            Expression::RecordUpdate(_) | Expression::TypeCoercion(_) => unreachable!(),
        }
    }

    fn desugar_function_type_argument(
        &mut self,
        expression: &Expression,
        to_type: &Type,
        source_information: Rc<SourceInformation>,
        variables: &HashMap<String, Type>,
    ) -> Result<Expression, CompileError> {
        let expression = self.desugar_expression(expression, variables)?;
        let from_type = self
            .reference_type_resolver
            .resolve(&ExpressionTypeExtractor::extract(&expression, variables))?;
        let to_type = self.reference_type_resolver.resolve(to_type)?;

        Ok(
            if to_type.is_function()
                && (!self.type_equality_checker.equal(&from_type, &to_type)?
                    || !expression.is_variable())
            {
                let name = self.name_generator.generate();

                Let::new(
                    vec![ValueDefinition::new(
                        &name,
                        expression,
                        to_type,
                        source_information.clone(),
                    )
                    .into()],
                    Variable::new(name, source_information),
                )
                .into()
            } else {
                expression
            },
        )
    }
}
