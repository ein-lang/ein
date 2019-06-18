use super::error::CompileError;
use super::type_compiler::TypeCompiler;
use crate::ast;

pub struct ExpressionCompiler {
    type_compiler: TypeCompiler,
}

impl ExpressionCompiler {
    pub fn new() -> Self {
        Self {
            type_compiler: TypeCompiler::new(),
        }
    }

    pub fn compile(
        &self,
        expression: &ast::Expression,
    ) -> Result<core::ast::Expression, CompileError> {
        match expression {
            ast::Expression::Application(application) => {
                let mut function = application.function();
                let mut arguments = vec![application.argument()];

                while let ast::Expression::Application(application) = &*function {
                    function = application.function();
                    arguments.push(application.argument());
                }

                let mut core_arguments = Vec::with_capacity(arguments.len());

                for argument in arguments.iter().rev() {
                    core_arguments.push(self.compile(argument)?);
                }

                Ok(core::ast::Application::new(self.compile(function)?, core_arguments).into())
            }
            ast::Expression::Let(let_) => match let_.definitions()[0] {
                ast::Definition::FunctionDefinition(_) => unimplemented!(),
                ast::Definition::ValueDefinition(_) => Ok(self.compile_let_values(let_)?.into()),
            },
            ast::Expression::Number(number) => Ok(core::ast::Expression::Number(*number)),
            ast::Expression::Operation(operation) => Ok(core::ast::Operation::new(
                operation.operator().into(),
                self.compile(operation.lhs())?,
                self.compile(operation.rhs())?,
            )
            .into()),
            ast::Expression::Variable(name) => Ok(core::ast::Expression::Variable(name.clone())),
        }
    }

    fn compile_let_values(&self, let_: &ast::Let) -> Result<core::ast::LetValues, CompileError> {
        Ok(core::ast::LetValues::new(
            let_.definitions()
                .iter()
                .map(|definition| match definition {
                    ast::Definition::FunctionDefinition(_) => Err(CompileError::new(
                        "cannot define functions together with values",
                    )),
                    ast::Definition::ValueDefinition(value_definition) => {
                        Ok(core::ast::ValueDefinition::new(
                            value_definition.name().into(),
                            self.compile(value_definition.body())?,
                            self.type_compiler.compile_value(value_definition.type_()),
                        ))
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
            self.compile(let_.expression())?,
        ))
    }
}
