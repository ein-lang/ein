mod equation;
mod error;
mod type_inferer;

use crate::ast::*;
use error::*;
use type_inferer::*;

pub fn infer_types(module: &Module) -> Result<Module, TypeInferenceError> {
    TypeInferer::new().infer(module)
}

#[cfg(test)]
mod test {
    use super::error::*;
    use super::infer_types;
    use crate::ast::*;
    use crate::types::{self, Type};

    #[test]
    fn infer_types_with_empty_modules() {
        assert_eq!(infer_types(&Module::new(vec![])), Ok(Module::new(vec![])));
    }

    #[test]
    fn infer_types_of_variables() {
        let module = Module::new(vec![ValueDefinition::new(
            "x".into(),
            42.0.into(),
            Type::Number,
        )
        .into()]);
        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn fail_to_infer_types_of_variables() {
        let module = Module::new(vec![ValueDefinition::new(
            "x".into(),
            42.0.into(),
            types::Function::new(Type::Number, Type::Number).into(),
        )
        .into()]);

        assert_eq!(
            infer_types(&module),
            Err(TypeInferenceError::new("type inference error".into()))
        );
    }

    #[test]
    fn infer_types_of_functions() {
        let module = Module::new(vec![FunctionDefinition::new(
            "f".into(),
            vec!["x".into()],
            42.0.into(),
            types::Function::new(Type::Number, Type::Number),
        )
        .into()]);

        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn fail_to_infer_types_of_functions() {
        let module = Module::new(vec![FunctionDefinition::new(
            "f".into(),
            vec!["x".into()],
            42.0.into(),
            types::Function::new(
                Type::Number,
                types::Function::new(Type::Number, Type::Number).into(),
            ),
        )
        .into()]);

        assert_eq!(
            infer_types(&module),
            Err(TypeInferenceError::new("type inference error".into()))
        );
    }

    #[test]
    fn infer_types_of_applications() {
        let module = Module::new(vec![
            FunctionDefinition::new(
                "f".into(),
                vec!["x".into()],
                42.0.into(),
                types::Function::new(Type::Number, Type::Number),
            )
            .into(),
            ValueDefinition::new(
                "x".into(),
                Application::new(Expression::Variable("f".into()), Expression::Number(42.0)).into(),
                Type::Number,
            )
            .into(),
        ]);

        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn fail_to_infer_types_of_applications() {
        let module = Module::new(vec![
            FunctionDefinition::new(
                "f".into(),
                vec!["x".into()],
                42.0.into(),
                types::Function::new(Type::Number, Type::Number),
            )
            .into(),
            ValueDefinition::new(
                "x".into(),
                Application::new(
                    Application::new(Expression::Variable("f".into()), Expression::Number(42.0))
                        .into(),
                    Expression::Number(42.0),
                )
                .into(),
                Type::Number,
            )
            .into(),
        ]);

        assert_eq!(
            infer_types(&module),
            Err(TypeInferenceError::new("type inference error".into()))
        );
    }

    #[test]
    fn infer_types_of_let_values() {
        let module = Module::new(vec![ValueDefinition::new(
            "x".into(),
            Let::new(
                vec![
                    ValueDefinition::new("y".into(), Expression::Number(42.0), Type::Number).into(),
                ],
                Expression::Variable("y".into()),
            )
            .into(),
            Type::Number,
        )
        .into()]);

        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn fail_to_infer_types_of_let_values() {
        let module = Module::new(vec![ValueDefinition::new(
            "x".into(),
            Let::new(
                vec![ValueDefinition::new(
                    "y".into(),
                    Expression::Number(42.0),
                    types::Function::new(Type::Number, Type::Number).into(),
                )
                .into()],
                Expression::Variable("y".into()),
            )
            .into(),
            Type::Number,
        )
        .into()]);

        assert_eq!(
            infer_types(&module),
            Err(TypeInferenceError::new("type inference error".into()))
        );
    }

    #[test]
    fn infer_types_of_let_functions() {
        let module = Module::new(vec![ValueDefinition::new(
            "x".into(),
            Let::new(
                vec![FunctionDefinition::new(
                    "f".into(),
                    vec!["z".into()],
                    Expression::Variable("z".into()),
                    types::Function::new(Type::Number, Type::Number),
                )
                .into()],
                Application::new(Expression::Variable("f".into()), Expression::Number(42.0)).into(),
            )
            .into(),
            Type::Number,
        )
        .into()]);

        assert_eq!(infer_types(&module), Ok(module));
    }

    #[test]
    fn fail_to_infer_types_of_let_functions() {
        let module = Module::new(vec![ValueDefinition::new(
            "x".into(),
            Let::new(
                vec![FunctionDefinition::new(
                    "f".into(),
                    vec!["z".into()],
                    Expression::Variable("z".into()),
                    types::Function::new(
                        Type::Number,
                        types::Function::new(Type::Number, Type::Number).into(),
                    ),
                )
                .into()],
                Application::new(Expression::Variable("f".into()), Expression::Number(42.0)).into(),
            )
            .into(),
            Type::Number,
        )
        .into()]);

        assert_eq!(
            infer_types(&module),
            Err(TypeInferenceError::new("type inference error".into()))
        );
    }

    #[test]
    fn infer_types_of_let_values_with_type_variables() {
        assert_eq!(
            infer_types(&Module::new(vec![ValueDefinition::new(
                "x".into(),
                Let::new(
                    vec![ValueDefinition::new(
                        "y".into(),
                        Expression::Number(42.0),
                        types::Variable::new().into()
                    )
                    .into(),],
                    Expression::Variable("y".into()),
                )
                .into(),
                Type::Number,
            )
            .into()])),
            Ok(Module::new(vec![ValueDefinition::new(
                "x".into(),
                Let::new(
                    vec![
                        ValueDefinition::new("y".into(), Expression::Number(42.0), Type::Number)
                            .into(),
                    ],
                    Expression::Variable("y".into()),
                )
                .into(),
                Type::Number,
            )
            .into()]))
        );
    }

    #[test]
    fn fail_to_infer_types_with_missing_variables() {
        let module = Module::new(vec![ValueDefinition::new(
            "x".into(),
            Expression::Variable("y".into()),
            Type::Number,
        )
        .into()]);

        assert_eq!(
            infer_types(&module),
            Err(TypeInferenceError::new("variable missing".into()))
        );
    }
}
