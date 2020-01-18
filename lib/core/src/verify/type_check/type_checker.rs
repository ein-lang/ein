use super::error::TypeCheckError;
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

        for declaration in module.declarations() {
            variables.insert(declaration.name(), declaration.type_().clone());
        }

        for definition in module.definitions() {
            match definition {
                Definition::FunctionDefinition(function_definition) => {
                    variables.insert(
                        function_definition.name(),
                        function_definition.type_().clone().into(),
                    );
                }
                Definition::ValueDefinition(value_definition) => {
                    variables.insert(
                        value_definition.name(),
                        value_definition.type_().clone().into(),
                    );
                }
            }
        }

        for definition in module.definitions() {
            match definition {
                Definition::FunctionDefinition(function_definition) => {
                    self.check_function_definition(function_definition, &variables)?;
                }
                Definition::ValueDefinition(value_definition) => {
                    self.check_value_definition(value_definition, &variables)?;
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

        for argument in function_definition
            .environment()
            .iter()
            .chain(function_definition.arguments())
        {
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

    fn check_value_definition(
        &mut self,
        value_definition: &ValueDefinition,
        variables: &HashMap<&str, Type>,
    ) -> Result<(), TypeCheckError> {
        if self.check_expression(value_definition.body(), &variables)?
            == value_definition.type_().clone().into()
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
                match self.check_variable(application.function(), variables)? {
                    Type::Function(function_type) => {
                        if function_type.arguments().len() != application.arguments().len() {
                            return Err(TypeCheckError);
                        }

                        for (argument, expected_type) in application
                            .arguments()
                            .iter()
                            .zip(function_type.arguments())
                        {
                            if &self.check_expression(argument, variables)? != expected_type {
                                return Err(TypeCheckError);
                            }
                        }

                        Ok(function_type.result().clone().into())
                    }
                    Type::Value(_) => Err(TypeCheckError),
                }
            }
            Expression::LetFunctions(let_functions) => {
                let mut variables = variables.clone();

                for definition in let_functions.definitions() {
                    variables.insert(definition.name(), definition.type_().clone().into());
                }

                for definition in let_functions.definitions() {
                    self.check_function_definition(definition, &variables)?;
                }

                self.check_expression(let_functions.expression(), &variables)
            }
            Expression::LetValues(let_values) => {
                let mut variables = variables.clone();

                for definition in let_values.definitions() {
                    self.check_value_definition(definition, &variables)?;
                    variables.insert(definition.name(), definition.type_().clone().into());
                }

                self.check_expression(let_values.expression(), &variables)
            }
            Expression::Number(_) => Ok(types::Value::Number.into()),
            Expression::Operation(operation) => {
                if self.check_expression(operation.lhs(), variables)? != types::Value::Number.into()
                {
                    return Err(TypeCheckError);
                }

                Ok(types::Value::Number.into())
            }
            Expression::Variable(variable) => self.check_variable(variable, variables),
        }
    }

    fn check_variable(
        &self,
        variable: &Variable,
        variables: &HashMap<&str, Type>,
    ) -> Result<Type, TypeCheckError> {
        variables
            .get(variable.name())
            .cloned()
            .ok_or(TypeCheckError)
    }
}
