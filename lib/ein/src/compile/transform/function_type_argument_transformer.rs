use super::super::error::CompileError;
use super::super::name_generator::NameGenerator;
use crate::ast::*;
use crate::types;

// Transforms all arguments in expressions into let-bound variables so that all
// partial applications are removed later.
pub struct FunctionTypeArgumentTransformer {
    name_generator: NameGenerator,
}

impl FunctionTypeArgumentTransformer {
    pub fn new() -> Self {
        Self {
            name_generator: NameGenerator::new("fta_function_"),
        }
    }

    pub fn transform(&self, module: &Module) -> Result<Module, CompileError> {
        module.transform_expressions(&mut |expression| self.transform_expression(expression))
    }

    fn transform_expression(&self, expression: &Expression) -> Result<Expression, CompileError> {
        match expression {
            Expression::Application(application) => {
                let source_information = application.source_information();

                Ok(Application::new(
                    application.function().clone(),
                    self.transform_argument(application.argument())?,
                    source_information.clone(),
                )
                .into())
            }
            Expression::RecordConstruction(record_construction) => {
                Ok(RecordConstruction::new(
                    record_construction.type_().clone(),
                    record_construction
                        .elements()
                        .iter()
                        .map(|(key, expression)| {
                            Ok((
                                key.clone(),
                                self.transform_argument(expression)?,
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
            | Expression::Variable(_) => Ok(expression.clone()),
            Expression::RecordUpdate(_) | Expression::TypeCoercion(_) => unreachable!(),
        }
    }

    fn transform_argument(&self, expression: &Expression) -> Result<Expression, CompileError> {
        let source_information = expression.source_information();
        let name = self.name_generator.generate();

        Ok(Let::new(
            vec![ValueDefinition::new(
                &name,
                expression.clone(),
                types::Unknown::new(source_information.clone()),
                source_information.clone(),
            )
            .into()],
            Variable::new(name, source_information.clone()),
            source_information.clone(),
        )
        .into())
    }
}
