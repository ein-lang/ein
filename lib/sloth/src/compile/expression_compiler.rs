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
        local_variables: &HashMap<String, Type>,
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
                    self.compile(function, local_variables)?
                        .to_variable()
                        .expect("variable")
                        .clone(),
                    arguments
                        .iter()
                        .rev()
                        .map(|argument| self.compile(argument, local_variables))
                        .collect::<Result<_, _>>()?,
                )
                .into())
            }
            ast::Expression::Let(let_) => match let_.definitions()[0] {
                ast::Definition::FunctionDefinition(_) => {
                    Ok(self.compile_let_functions(let_, local_variables)?.into())
                }
                ast::Definition::ValueDefinition(_) => {
                    Ok(self.compile_let_values(let_, local_variables)?.into())
                }
            },
            ast::Expression::Number(number) => Ok(core::ast::Expression::Number(number.value())),
            ast::Expression::Operation(operation) => Ok(core::ast::Operation::new(
                operation.operator().into(),
                self.compile(operation.lhs(), local_variables)?,
                self.compile(operation.rhs(), local_variables)?,
            )
            .into()),
            ast::Expression::Variable(variable) => Ok(core::ast::Expression::Variable(
                core::ast::Variable::new(variable.name()),
            )),
        }
    }

    fn compile_let_functions(
        &self,
        let_: &ast::Let,
        local_variables: &HashMap<String, Type>,
    ) -> Result<core::ast::LetFunctions, CompileError> {
        let function_definitions = let_
            .definitions()
            .iter()
            .map(|definition| match definition {
                ast::Definition::FunctionDefinition(function_definition) => Ok(function_definition),
                ast::Definition::ValueDefinition(value_definition) => {
                    Err(CompileError::MixedDefinitionsInLet(
                        value_definition.source_information().clone(),
                    ))
                }
            })
            .collect::<Result<Vec<&ast::FunctionDefinition>, _>>()?;

        let variables = &local_variables
            .iter()
            .map(|(name, type_)| (name.clone(), type_.clone()))
            .chain(function_definitions.iter().map(|function_definition| {
                (
                    function_definition.name().into(),
                    function_definition.type_().clone(),
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
                        .expect("function type");

                    Ok(core::ast::FunctionDefinition::new(
                        function_definition.name(),
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
                            .unwrap_or_else(|| vec![]),
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
                                .iter()
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
                .collect::<Result<Vec<_>, CompileError>>()?,
            self.compile(let_.expression(), variables)?,
        ))
    }

    fn compile_let_values(
        &self,
        let_: &ast::Let,
        local_variables: &HashMap<String, Type>,
    ) -> Result<core::ast::LetValues, CompileError> {
        let value_definitions = let_
            .definitions()
            .iter()
            .map(|definition| match definition {
                ast::Definition::FunctionDefinition(function_definition) => {
                    Err(CompileError::MixedDefinitionsInLet(
                        function_definition.source_information().clone(),
                    ))
                }
                ast::Definition::ValueDefinition(value_definition) => Ok(value_definition),
            })
            .collect::<Result<Vec<_>, _>>()?;

        let variables = &local_variables
            .iter()
            .map(|(name, type_)| (name.clone(), type_.clone()))
            .chain(value_definitions.iter().map(|value_definition| {
                (
                    value_definition.name().into(),
                    value_definition.type_().clone(),
                )
            }))
            .collect::<HashMap<_, _>>();

        Ok(core::ast::LetValues::new(
            value_definitions
                .iter()
                .map(|value_definition| {
                    Ok(core::ast::ValueDefinition::new(
                        value_definition.name(),
                        self.compile(value_definition.body(), variables)?,
                        self.type_compiler.compile_value(value_definition.type_()),
                    ))
                })
                .collect::<Result<Vec<_>, CompileError>>()?,
            self.compile(let_.expression(), variables)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::super::type_compiler::TypeCompiler;
    use super::ExpressionCompiler;
    use crate::ast::*;
    use crate::debug::SourceInformation;
    use crate::types;
    use std::collections::HashMap;

    #[test]
    fn compile_operation() {
        assert_eq!(
            ExpressionCompiler::new(&TypeCompiler::new()).compile(
                &Operation::new(
                    Operator::Add,
                    Number::new(1.0, SourceInformation::dummy()),
                    Number::new(2.0, SourceInformation::dummy()),
                    SourceInformation::dummy()
                )
                .into(),
                &HashMap::new()
            ),
            Ok(core::ast::Operation::new(core::ast::Operator::Add, 1.0, 2.0).into())
        );
    }

    #[test]
    fn compile_let_values() {
        assert_eq!(
            ExpressionCompiler::new(&TypeCompiler::new()).compile(
                &Let::new(
                    vec![ValueDefinition::new(
                        "x",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                )
                .into(),
                &HashMap::new()
            ),
            Ok(core::ast::LetValues::new(
                vec![core::ast::ValueDefinition::new(
                    "x",
                    core::ast::Expression::Number(42.0),
                    core::types::Value::Number,
                )],
                core::ast::Variable::new("x")
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
                        "f",
                        vec!["x".into()],
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                )
                .into(),
                &HashMap::new()
            ),
            Ok(core::ast::LetFunctions::new(
                vec![core::ast::FunctionDefinition::new(
                    "f",
                    vec![],
                    vec![core::ast::Argument::new("x", core::types::Value::Number)],
                    42.0,
                    core::types::Value::Number,
                )],
                core::ast::Variable::new("x")
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
                        "f",
                        vec!["x".into()],
                        Application::new(
                            Variable::new("f", SourceInformation::dummy()),
                            Variable::new("x", SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                )
                .into(),
                &HashMap::new()
            ),
            Ok(core::ast::LetFunctions::new(
                vec![core::ast::FunctionDefinition::new(
                    "f",
                    vec![core::ast::Argument::new(
                        "f",
                        core::types::Function::new(
                            vec![core::types::Value::Number.into()],
                            core::types::Value::Number
                        )
                    )],
                    vec![core::ast::Argument::new("x", core::types::Value::Number)],
                    core::ast::Application::new(
                        core::ast::Variable::new("f"),
                        vec![core::ast::Variable::new("x").into()]
                    ),
                    core::types::Value::Number,
                )],
                core::ast::Variable::new("x")
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
                        "f",
                        vec!["x".into()],
                        Let::new(
                            vec![FunctionDefinition::new(
                                "g",
                                vec!["y".into()],
                                Variable::new("x", SourceInformation::dummy()),
                                types::Function::new(
                                    types::Number::new(SourceInformation::dummy()),
                                    types::Number::new(SourceInformation::dummy()),
                                    SourceInformation::dummy()
                                ),
                                SourceInformation::dummy()
                            )
                            .into()],
                            Variable::new("x", SourceInformation::dummy())
                        ),
                        types::Function::new(
                            types::Number::new(SourceInformation::dummy()),
                            types::Number::new(SourceInformation::dummy()),
                            SourceInformation::dummy()
                        ),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Variable::new("x", SourceInformation::dummy())
                )
                .into(),
                &HashMap::new()
            ),
            Ok(core::ast::LetFunctions::new(
                vec![core::ast::FunctionDefinition::new(
                    "f",
                    vec![],
                    vec![core::ast::Argument::new("x", core::types::Value::Number)],
                    core::ast::LetFunctions::new(
                        vec![core::ast::FunctionDefinition::new(
                            "g",
                            vec![core::ast::Argument::new("x", core::types::Value::Number)],
                            vec![core::ast::Argument::new("y", core::types::Value::Number)],
                            core::ast::Variable::new("x"),
                            core::types::Value::Number,
                        )],
                        core::ast::Variable::new("x")
                    ),
                    core::types::Value::Number,
                )],
                core::ast::Variable::new("x")
            )
            .into())
        );
    }

    #[test]
    fn compile_let_values_with_free_variables() {
        assert_eq!(
            ExpressionCompiler::new(&TypeCompiler::new()).compile(
                &Let::new(
                    vec![ValueDefinition::new(
                        "y",
                        Number::new(42.0, SourceInformation::dummy()),
                        types::Number::new(SourceInformation::dummy()),
                        SourceInformation::dummy()
                    )
                    .into()],
                    Let::new(
                        vec![FunctionDefinition::new(
                            "f",
                            vec!["x".into()],
                            Variable::new("y", SourceInformation::dummy()),
                            types::Function::new(
                                types::Number::new(SourceInformation::dummy()),
                                types::Number::new(SourceInformation::dummy()),
                                SourceInformation::dummy()
                            ),
                            SourceInformation::dummy()
                        )
                        .into()],
                        Variable::new("y", SourceInformation::dummy())
                    )
                )
                .into(),
                &HashMap::new()
            ),
            Ok(core::ast::LetValues::new(
                vec![core::ast::ValueDefinition::new(
                    "y",
                    42.0,
                    core::types::Value::Number,
                )],
                core::ast::LetFunctions::new(
                    vec![core::ast::FunctionDefinition::new(
                        "f",
                        vec![core::ast::Argument::new("y", core::types::Value::Number)],
                        vec![core::ast::Argument::new("x", core::types::Value::Number)],
                        core::ast::Variable::new("y"),
                        core::types::Value::Number,
                    )],
                    core::ast::Variable::new("y")
                )
            )
            .into())
        );
    }
}
