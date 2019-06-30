use super::error::CompileError;
use super::free_variable_finder::FreeVariableFinder;
use super::type_compiler::TypeCompiler;
use crate::ast;
use crate::types::Type;
use std::collections::HashMap;

pub struct ExpressionCompiler<'a> {
    type_compiler: &'a TypeCompiler,
}

impl<'a> ExpressionCompiler<'a> {
    pub fn new(type_compiler: &'a TypeCompiler) -> Self {
        Self { type_compiler }
    }

    pub fn compile(
        &self,
        expression: &ast::Expression,
        variables: &HashMap<String, Type>,
    ) -> Result<core::ast::Expression, CompileError> {
        match expression {
            ast::Expression::Application(application) => {
                let mut function = application.function();
                let mut arguments = vec![application.argument()];

                while let ast::Expression::Application(application) = &*function {
                    function = application.function();
                    arguments.push(application.argument());
                }

                Ok(core::ast::Application::new(
                    self.compile(function, variables)?,
                    arguments
                        .iter()
                        .rev()
                        .map(|argument| self.compile(argument, variables))
                        .collect::<Result<_, _>>()?,
                )
                .into())
            }
            ast::Expression::Let(let_) => match let_.definitions()[0] {
                ast::Definition::FunctionDefinition(_) => {
                    Ok(self.compile_let_functions(let_, variables)?.into())
                }
                ast::Definition::ValueDefinition(_) => {
                    Ok(self.compile_let_values(let_, variables)?.into())
                }
            },
            ast::Expression::Number(number) => Ok(core::ast::Expression::Number(*number)),
            ast::Expression::Operation(operation) => Ok(core::ast::Operation::new(
                operation.operator().into(),
                self.compile(operation.lhs(), variables)?,
                self.compile(operation.rhs(), variables)?,
            )
            .into()),
            ast::Expression::Variable(name) => Ok(core::ast::Expression::Variable(name.clone())),
        }
    }

    fn compile_let_functions(
        &self,
        let_: &ast::Let,
        variables: &HashMap<String, Type>,
    ) -> Result<core::ast::LetFunctions, CompileError> {
        let function_definitions = let_
            .definitions()
            .iter()
            .map(|definition| match definition {
                ast::Definition::FunctionDefinition(function_definition) => Ok(function_definition),
                ast::Definition::ValueDefinition(_) => Err(CompileError::new(
                    "cannot define values together with functions",
                )),
            })
            .collect::<Result<Vec<&ast::FunctionDefinition>, _>>()?;

        let variables = &variables
            .into_iter()
            .map(|(name, type_)| (name.clone(), type_.clone()))
            .chain(function_definitions.iter().map(|function_definition| {
                (
                    function_definition.name().into(),
                    function_definition.type_().clone().into(),
                )
            }))
            .collect::<HashMap<_, _>>();

        Ok(core::ast::LetFunctions::new(
            function_definitions
                .iter()
                .map(|function_definition| {
                    let type_ = function_definition
                        .type_()
                        .to_function()
                        .ok_or(CompileError::new("function expected"))?;

                    Ok(core::ast::FunctionDefinition::new(
                        function_definition.name().into(),
                        FreeVariableFinder::new()
                            .find(function_definition)
                            .iter()
                            .map(|name| {
                                variables.get(name).map(|type_| {
                                    core::ast::Argument::new(
                                        name.clone(),
                                        self.type_compiler.compile(type_),
                                    )
                                })
                            })
                            .collect::<Option<_>>()
                            .unwrap_or(vec![]),
                        function_definition
                            .arguments()
                            .iter()
                            .zip(type_.arguments())
                            .map(|(name, type_)| {
                                core::ast::Argument::new(
                                    name.clone(),
                                    self.type_compiler.compile(type_),
                                )
                            })
                            .collect(),
                        self.compile(
                            function_definition.body(),
                            &variables
                                .into_iter()
                                .map(|(name, type_)| (name.clone(), type_.clone()))
                                .chain(
                                    function_definition
                                        .arguments()
                                        .iter()
                                        .zip(type_.arguments())
                                        .map(|(argument, type_)| (argument.clone(), type_.clone())),
                                )
                                .collect(),
                        )?,
                        self.type_compiler.compile_value(type_.last_result()),
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?,
            self.compile(let_.expression(), variables)?,
        ))
    }

    fn compile_let_values(
        &self,
        let_: &ast::Let,
        variables: &HashMap<String, Type>,
    ) -> Result<core::ast::LetValues, CompileError> {
        let value_definitions = let_
            .definitions()
            .iter()
            .map(|definition| match definition {
                ast::Definition::FunctionDefinition(_) => Err(CompileError::new(
                    "cannot define functions together with values",
                )),
                ast::Definition::ValueDefinition(value_definition) => Ok(value_definition),
            })
            .collect::<Result<Vec<_>, _>>()?;

        let variables = &variables
            .into_iter()
            .map(|(name, type_)| (name.clone(), type_.clone()))
            .chain(value_definitions.iter().map(|value_definition| {
                (
                    value_definition.name().into(),
                    value_definition.type_().clone().into(),
                )
            }))
            .collect::<HashMap<_, _>>();

        Ok(core::ast::LetValues::new(
            value_definitions
                .iter()
                .map(|value_definition| {
                    Ok(core::ast::ValueDefinition::new(
                        value_definition.name().into(),
                        self.compile(value_definition.body(), variables)?,
                        self.type_compiler.compile_value(value_definition.type_()),
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?,
            self.compile(let_.expression(), variables)?,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::super::type_compiler::TypeCompiler;
    use super::ExpressionCompiler;
    use crate::ast::*;
    use crate::types::{self, Type};
    use std::collections::HashMap;

    #[test]
    fn compile_operation() {
        assert_eq!(
            ExpressionCompiler::new(&TypeCompiler::new()).compile(
                &Operation::new(Operator::Add, 1.0.into(), 2.0.into()).into(),
                &HashMap::new()
            ),
            Ok(core::ast::Operation::new(core::ast::Operator::Add, 1.0.into(), 2.0.into()).into())
        );
    }

    #[test]
    fn compile_let_values() {
        assert_eq!(
            ExpressionCompiler::new(&TypeCompiler::new()).compile(
                &Let::new(
                    vec![
                        ValueDefinition::new("x".into(), Expression::Number(42.0), Type::Number)
                            .into()
                    ],
                    Expression::Variable("x".into())
                )
                .into(),
                &HashMap::new()
            ),
            Ok(core::ast::LetValues::new(
                vec![core::ast::ValueDefinition::new(
                    "x".into(),
                    core::ast::Expression::Number(42.0),
                    core::types::Value::Number,
                )],
                core::ast::Expression::Variable("x".into())
            )
            .into())
        );
    }

    #[test]
    fn compile_let_functions() {
        assert_eq!(
            ExpressionCompiler::new(&TypeCompiler::new()).compile(
                &Let::new(
                    vec![FunctionDefinition::new(
                        "f".into(),
                        vec!["x".into()],
                        Expression::Number(42.0),
                        types::Function::new(Type::Number, Type::Number)
                    )
                    .into()],
                    Expression::Variable("x".into())
                )
                .into(),
                &HashMap::new()
            ),
            Ok(core::ast::LetFunctions::new(
                vec![core::ast::FunctionDefinition::new(
                    "f".into(),
                    vec![],
                    vec![core::ast::Argument::new(
                        "x".into(),
                        core::types::Value::Number.into()
                    )],
                    core::ast::Expression::Number(42.0),
                    core::types::Value::Number,
                )],
                core::ast::Expression::Variable("x".into())
            )
            .into())
        );
    }

    #[test]
    fn compile_let_functions_with_recursive_functions() {
        assert_eq!(
            ExpressionCompiler::new(&TypeCompiler::new()).compile(
                &Let::new(
                    vec![FunctionDefinition::new(
                        "f".into(),
                        vec!["x".into()],
                        Application::new(
                            Expression::Variable("f".into()),
                            Expression::Variable("x".into())
                        )
                        .into(),
                        types::Function::new(Type::Number, Type::Number)
                    )
                    .into()],
                    Expression::Variable("x".into())
                )
                .into(),
                &HashMap::new()
            ),
            Ok(core::ast::LetFunctions::new(
                vec![core::ast::FunctionDefinition::new(
                    "f".into(),
                    vec![core::ast::Argument::new(
                        "f".into(),
                        core::types::Function::new(
                            vec![core::types::Value::Number.into()],
                            core::types::Value::Number.into()
                        )
                        .into()
                    )],
                    vec![core::ast::Argument::new(
                        "x".into(),
                        core::types::Value::Number.into()
                    )],
                    core::ast::Application::new(
                        core::ast::Expression::Variable("f".into()),
                        vec![core::ast::Expression::Variable("x".into())]
                    )
                    .into(),
                    core::types::Value::Number,
                )],
                core::ast::Expression::Variable("x".into())
            )
            .into())
        );
    }

    #[test]
    fn compile_nested_let_functions() {
        assert_eq!(
            ExpressionCompiler::new(&TypeCompiler::new()).compile(
                &Let::new(
                    vec![FunctionDefinition::new(
                        "f".into(),
                        vec!["x".into()],
                        Let::new(
                            vec![FunctionDefinition::new(
                                "g".into(),
                                vec!["y".into()],
                                Expression::Variable("x".into()),
                                types::Function::new(Type::Number, Type::Number)
                            )
                            .into()],
                            Expression::Variable("x".into())
                        )
                        .into(),
                        types::Function::new(Type::Number, Type::Number)
                    )
                    .into()],
                    Expression::Variable("x".into())
                )
                .into(),
                &HashMap::new()
            ),
            Ok(core::ast::LetFunctions::new(
                vec![core::ast::FunctionDefinition::new(
                    "f".into(),
                    vec![],
                    vec![core::ast::Argument::new(
                        "x".into(),
                        core::types::Value::Number.into()
                    )],
                    core::ast::LetFunctions::new(
                        vec![core::ast::FunctionDefinition::new(
                            "g".into(),
                            vec![core::ast::Argument::new(
                                "x".into(),
                                core::types::Value::Number.into()
                            )],
                            vec![core::ast::Argument::new(
                                "y".into(),
                                core::types::Value::Number.into()
                            )],
                            core::ast::Expression::Variable("x".into()),
                            core::types::Value::Number,
                        )],
                        core::ast::Expression::Variable("x".into())
                    )
                    .into(),
                    core::types::Value::Number,
                )],
                core::ast::Expression::Variable("x".into())
            )
            .into())
        );
    }

    #[test]
    fn compile_let_values_with_free_variables() {
        assert_eq!(
            ExpressionCompiler::new(&TypeCompiler::new()).compile(
                &Let::new(
                    vec![
                        ValueDefinition::new("y".into(), Expression::Number(42.0), Type::Number)
                            .into()
                    ],
                    Let::new(
                        vec![FunctionDefinition::new(
                            "f".into(),
                            vec!["x".into()],
                            Expression::Variable("y".into()).into(),
                            types::Function::new(Type::Number, Type::Number)
                        )
                        .into()],
                        Expression::Variable("y".into())
                    )
                    .into()
                )
                .into(),
                &HashMap::new()
            ),
            Ok(core::ast::LetValues::new(
                vec![core::ast::ValueDefinition::new(
                    "y".into(),
                    core::ast::Expression::Number(42.0),
                    core::types::Value::Number,
                )],
                core::ast::LetFunctions::new(
                    vec![core::ast::FunctionDefinition::new(
                        "f".into(),
                        vec![core::ast::Argument::new(
                            "y".into(),
                            core::types::Value::Number.into()
                        )],
                        vec![core::ast::Argument::new(
                            "x".into(),
                            core::types::Value::Number.into()
                        )],
                        core::ast::Expression::Variable("y".into()),
                        core::types::Value::Number,
                    )],
                    core::ast::Expression::Variable("y".into())
                )
                .into()
            )
            .into())
        );
    }
}
