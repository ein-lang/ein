mod error;
mod type_checker;

use crate::ast::*;
pub use error::*;
use type_checker::*;

pub fn check_types(module: &Module) -> Result<(), TypeCheckError> {
    TypeChecker::new().check(module)
}

#[cfg(test)]
mod test {
    use super::check_types;
    use super::error::*;
    use crate::ast::*;
    use crate::types;

    #[test]
    fn check_types_with_empty_modules() {
        assert_eq!(check_types(&Module::new(vec![])), Ok(()));
    }

    #[test]
    fn check_types_of_variables() {
        let module = Module::new(vec![
            ValueDefinition::new("x", 42.0, types::Value::Number).into()
        ]);
        assert_eq!(check_types(&module), Ok(()));
    }

    #[test]
    fn fail_to_check_types_of_variables() {
        let module = Module::new(vec![
            FunctionDefinition::new(
                "f",
                vec![],
                vec![Argument::new("x", types::Value::Number)],
                42.0,
                types::Value::Number,
            )
            .into(),
            ValueDefinition::new("x", Variable::new("f"), types::Value::Number)
                .into(),
        ]);

        assert_eq!(check_types(&module), Err(TypeCheckError));
    }

    #[test]
    fn check_types_of_functions() {
        let module = Module::new(vec![FunctionDefinition::new(
            "f",
            vec![],
            vec![Argument::new("x", types::Value::Number)],
            42.0,
            types::Value::Number,
        )
        .into()]);

        assert_eq!(check_types(&module), Ok(()));
    }

    #[test]
    fn fail_to_check_types_of_functions() {
        let module = Module::new(vec![
            FunctionDefinition::new(
                "f",
                vec![],
                vec![Argument::new("x", types::Value::Number)],
                42.0,
                types::Value::Number,
            )
            .into(),
            FunctionDefinition::new(
                "g",
                vec![],
                vec![Argument::new("x", types::Value::Number)],
                Variable::new("f"),
                types::Value::Number,
            )
            .into(),
        ]);

        assert_eq!(check_types(&module), Err(TypeCheckError));
    }

    #[test]
    fn check_types_of_applications() {
        let module = Module::new(vec![
            FunctionDefinition::new(
                "f",
                vec![],
                vec![Argument::new("x", types::Value::Number)],
                42.0,
                types::Value::Number,
            )
            .into(),
            ValueDefinition::new(
                "x",
                Application::new(
                    Variable::new("f"),
                    vec![Expression::Number(42.0)],
                ),
                types::Value::Number,
            )
            .into(),
        ]);

        assert_eq!(check_types(&module), Ok(()));
    }

    #[test]
    fn fail_to_check_types_of_applications() {
        let module = Module::new(vec![
            FunctionDefinition::new(
                "f",
                vec![],
                vec![Argument::new("x", types::Value::Number)],
                42.0,
                types::Value::Number,
            )
            .into(),
            ValueDefinition::new(
                "x",
                Application::new(
                    Variable::new("f"),
                    vec![Expression::Number(42.0), Expression::Number(42.0)],
                ),
                types::Value::Number,
            )
            .into(),
        ]);

        assert_eq!(check_types(&module), Err(TypeCheckError));
    }

    #[test]
    fn fail_to_check_types_because_of_missing_variables() {
        let module = Module::new(vec![ValueDefinition::new(
            "x",
            Variable::new("y"),
            types::Value::Number,
        )
        .into()]);

        assert_eq!(check_types(&module), Err(TypeCheckError));
    }

    #[test]
    fn check_types_of_let_values() {
        let module = Module::new(vec![ValueDefinition::new(
            "x",
            LetValues::new(
                vec![
                    ValueDefinition::new("y", 42.0, types::Value::Number),
                    ValueDefinition::new(
                        "z",
                        Variable::new("y"),
                        types::Value::Number,
                    ),
                ],
                Variable::new("z"),
            ),
            types::Value::Number,
        )
        .into()]);

        assert_eq!(check_types(&module), Ok(()));
    }

    #[test]
    fn fail_to_check_types_of_let_values() {
        let module = Module::new(vec![
            FunctionDefinition::new(
                "f",
                vec![],
                vec![Argument::new("x", types::Value::Number)],
                42.0,
                types::Value::Number,
            )
            .into(),
            ValueDefinition::new(
                "x",
                LetValues::new(
                    vec![ValueDefinition::new(
                        "y",
                        Variable::new("f"),
                        types::Value::Number,
                    )],
                    Variable::new("y"),
                ),
                types::Value::Number,
            )
            .into(),
        ]);

        assert_eq!(check_types(&module), Err(TypeCheckError));
    }
}
