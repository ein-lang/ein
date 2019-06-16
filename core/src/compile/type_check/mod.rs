mod error;
mod type_checker;

use crate::ast::*;
use error::*;
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
        let module = Module::new(vec![VariableDefinition::new(
            "x".into(),
            42.0.into(),
            types::Value::Number,
        )
        .into()]);
        assert_eq!(check_types(&module), Ok(()));
    }

    #[test]
    fn fail_to_check_types_of_variables() {
        let module = Module::new(vec![
            FunctionDefinition::new(
                "f".into(),
                vec![Argument::new("x".into(), types::Value::Number.into())],
                42.0.into(),
                types::Value::Number,
            )
            .into(),
            VariableDefinition::new(
                "x".into(),
                Expression::Variable("f".into()).into(),
                types::Value::Number,
            )
            .into(),
        ]);

        assert_eq!(check_types(&module), Err(TypeCheckError));
    }

    #[test]
    fn check_types_of_functions() {
        let module = Module::new(vec![FunctionDefinition::new(
            "f".into(),
            vec![Argument::new("x".into(), types::Value::Number.into())],
            42.0.into(),
            types::Value::Number,
        )
        .into()]);

        assert_eq!(check_types(&module), Ok(()));
    }

    #[test]
    fn fail_to_check_types_of_functions() {
        let module = Module::new(vec![
            FunctionDefinition::new(
                "f".into(),
                vec![Argument::new("x".into(), types::Value::Number.into())],
                42.0.into(),
                types::Value::Number,
            )
            .into(),
            FunctionDefinition::new(
                "g".into(),
                vec![Argument::new("x".into(), types::Value::Number.into())],
                Expression::Variable("f".into()).into(),
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
                "f".into(),
                vec![Argument::new("x".into(), types::Value::Number.into())],
                42.0.into(),
                types::Value::Number,
            )
            .into(),
            VariableDefinition::new(
                "x".into(),
                Application::new(
                    Expression::Variable("f".into()),
                    vec![Expression::Number(42.0)],
                )
                .into(),
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
                "f".into(),
                vec![Argument::new("x".into(), types::Value::Number.into())],
                42.0.into(),
                types::Value::Number,
            )
            .into(),
            VariableDefinition::new(
                "x".into(),
                Application::new(
                    Expression::Variable("f".into()),
                    vec![Expression::Number(42.0), Expression::Number(42.0)],
                )
                .into(),
                types::Value::Number,
            )
            .into(),
        ]);

        assert_eq!(check_types(&module), Err(TypeCheckError));
    }

    #[test]
    fn fail_to_check_types_because_of_missing_variables() {
        let module = Module::new(vec![VariableDefinition::new(
            "x".into(),
            Expression::Variable("y".into()).into(),
            types::Value::Number,
        )
        .into()]);

        assert_eq!(check_types(&module), Err(TypeCheckError));
    }
}
