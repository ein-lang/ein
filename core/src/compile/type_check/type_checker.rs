use super::error::*;
use crate::ast::*;
use crate::types::{self, Type};
use std::collections::*;

#[derive(Clone, Debug)]
pub struct TypeChecker {}

impl TypeChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check(&mut self, module: &Module) -> Result<(), TypeCheckError> {
        let mut variables = HashMap::<&str, Type>::new();

        for definition in module.definitions() {
            match definition {
                Definition::FunctionDefinition(function_definition) => {
                    variables.insert(
                        function_definition.name(),
                        function_definition.type_().clone().into(),
                    );
                }
                Definition::VariableDefinition(variable_definition) => {
                    variables.insert(
                        variable_definition.name(),
                        variable_definition.type_().clone().into(),
                    );
                }
            }
        }

        for definition in module.definitions() {
            match definition {
                Definition::FunctionDefinition(function_definition) => {
                    self.check_function_definition(function_definition, &variables)?;
                }
                Definition::VariableDefinition(variable_definition) => {
                    self.check_variable_definition(variable_definition, &variables)?;
                }
            };
        }

        Ok(())
    }

    fn check_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
        variables: &HashMap<&str, Type>,
    ) -> Result<(), TypeCheckError> {
        let mut variables = variables.clone();

        for argument in function_definition.arguments() {
            variables.insert(argument.name(), argument.type_().clone());
        }

        if self.check_expression(function_definition.body(), &variables)?
            == function_definition.result_type().clone().into()
        {
            Ok(())
        } else {
            Err(TypeCheckError)
        }
    }

    fn check_variable_definition(
        &mut self,
        variable_definition: &VariableDefinition,
        variables: &HashMap<&str, Type>,
    ) -> Result<(), TypeCheckError> {
        if self.check_expression(variable_definition.body(), &variables)?
            == variable_definition.type_().clone().into()
        {
            Ok(())
        } else {
            Err(TypeCheckError)
        }
    }

    fn check_expression(
        &mut self,
        expression: &Expression,
        variables: &HashMap<&str, Type>,
    ) -> Result<Type, TypeCheckError> {
        match expression {
            Expression::Application(application) => {
                match self.check_expression(application.function(), variables)? {
                    Type::Function(function_type) => {
                        if function_type.arguments().len() != application.arguments().len() {
                            return Err(TypeCheckError);
                        }

                        for (argument, expected_type) in application
                            .arguments()
                            .iter()
                            .zip(function_type.arguments())
                        {
                            if self.check_expression(argument, variables)? != *expected_type {
                                return Err(TypeCheckError);
                            }
                        }

                        Ok(function_type.result().clone().into())
                    }
                    Type::Value(_) => Err(TypeCheckError),
                }
            }
            Expression::Number(_) => Ok(types::Value::Number.into()),
            Expression::Operation(operation) => {
                if self.check_expression(operation.lhs(), variables)? != types::Value::Number.into()
                {
                    return Err(TypeCheckError);
                }

                Ok(types::Value::Number.into())
            }
            Expression::Variable(variable) => variables
                .get(variable.as_str())
                .map(|type_| type_.clone())
                .ok_or(TypeCheckError),
        }
    }
}
