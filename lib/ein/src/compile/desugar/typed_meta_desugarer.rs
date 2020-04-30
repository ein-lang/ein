use super::super::error::CompileError;
use super::super::module_environment_creator::ModuleEnvironmentCreator;
use crate::ast::*;
use crate::types::Type;
use std::collections::HashMap;

pub trait TypedDesugarer {
    fn desugar_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<FunctionDefinition, CompileError>;
    fn desugar_value_definition(
        &mut self,
        value_definition: &ValueDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<ValueDefinition, CompileError>;
    fn desugar_expression(
        &mut self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Expression, CompileError>;
}

pub struct TypedMetaDesugarer<D> {
    component_desugarer: D,
}

impl<D: TypedDesugarer> TypedMetaDesugarer<D> {
    pub fn new(component_desugarer: D) -> Self {
        Self {
            component_desugarer,
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

        for (name, type_) in function_definition.arguments().iter().zip(
            function_definition
                .type_()
                .to_function()
                .unwrap()
                .arguments(),
        ) {
            variables.insert(name.into(), type_.clone());
        }

        let body = self.desugar_expression(function_definition.body(), &variables)?;

        Ok(self.component_desugarer.desugar_function_definition(
            &FunctionDefinition::new(
                function_definition.name(),
                function_definition.arguments().to_vec(),
                body,
                function_definition.type_().clone(),
                function_definition.source_information().clone(),
            ),
            &variables,
        )?)
    }

    fn desugar_value_definition(
        &mut self,
        value_definition: &ValueDefinition,
        variables: &HashMap<String, Type>,
    ) -> Result<ValueDefinition, CompileError> {
        let body = self.desugar_expression(value_definition.body(), variables)?;

        Ok(self.component_desugarer.desugar_value_definition(
            &ValueDefinition::new(
                value_definition.name(),
                body,
                value_definition.type_().clone(),
                value_definition.source_information().clone(),
            ),
            variables,
        )?)
    }

    fn desugar_expression(
        &mut self,
        expression: &Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<Expression, CompileError> {
        let expression = match expression {
            Expression::Application(application) => Application::new(
                self.desugar_expression(application.function(), variables)?,
                self.desugar_expression(application.argument(), variables)?,
                application.source_information().clone(),
            )
            .into(),
            Expression::Case(case) => Case::with_type(
                case.type_().clone(),
                case.name(),
                self.desugar_expression(case.argument(), variables)?,
                case.alternatives()
                    .iter()
                    .map(|alternative| {
                        let mut variables = variables.clone();

                        variables.insert(case.name().into(), alternative.type_().clone());

                        Ok(Alternative::new(
                            alternative.type_().clone(),
                            self.desugar_expression(alternative.expression(), &variables)?,
                        ))
                    })
                    .collect::<Result<_, CompileError>>()?,
                case.source_information().clone(),
            )
            .into(),
            Expression::If(if_) => If::new(
                self.desugar_expression(if_.condition(), variables)?,
                self.desugar_expression(if_.then(), variables)?,
                self.desugar_expression(if_.else_(), variables)?,
                if_.source_information().clone(),
            )
            .into(),
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

                Let::new(
                    definitions,
                    self.desugar_expression(let_.expression(), &variables)?,
                )
                .into()
            }
            Expression::Operation(operation) => Operation::new(
                operation.operator(),
                self.desugar_expression(operation.lhs(), &variables)?,
                self.desugar_expression(operation.rhs(), &variables)?,
                operation.source_information().clone(),
            )
            .into(),
            Expression::RecordConstruction(record_construction) => RecordConstruction::new(
                record_construction.type_().clone(),
                record_construction
                    .elements()
                    .iter()
                    .map(|(key, expression)| {
                        Ok((
                            key.clone(),
                            self.desugar_expression(expression, &variables)?,
                        ))
                    })
                    .collect::<Result<_, CompileError>>()?,
                record_construction.source_information().clone(),
            )
            .into(),
            Expression::Boolean(_)
            | Expression::None(_)
            | Expression::Number(_)
            | Expression::Variable(_) => expression.clone(),
            Expression::RecordUpdate(_) | Expression::TypeCoercion(_) => unreachable!(),
        };

        Ok(self
            .component_desugarer
            .desugar_expression(&expression, &variables)?)
    }
}
