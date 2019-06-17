use crate::ast;

pub struct ExpressionCompiler {}

impl ExpressionCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&self, expression: &ast::Expression) -> core::ast::Expression {
        match expression {
            ast::Expression::Application(application) => {
                let mut function = application.function();
                let mut arguments = vec![application.argument()];

                while let ast::Expression::Application(application) = &*function {
                    function = application.function();
                    arguments.push(application.argument());
                }

                arguments.reverse();

                core::ast::Application::new(
                    self.compile(function),
                    arguments
                        .iter()
                        .map(|argument| self.compile(argument))
                        .collect::<Vec<_>>(),
                )
                .into()
            }
            ast::Expression::Number(number) => core::ast::Expression::Number(*number),
            ast::Expression::Operation(operation) => core::ast::Operation::new(
                operation.operator().into(),
                self.compile(operation.lhs()),
                self.compile(operation.rhs()),
            )
            .into(),
            ast::Expression::Variable(name) => core::ast::Expression::Variable(name.clone()),
        }
    }
}
