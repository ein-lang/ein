mod error;
mod type_check;

use super::ast::*;
pub use error::*;
use type_check::check_types;

pub fn verify(module: &Module) -> Result<(), VerificationError> {
    check_global_free_variables(module)?;
    check_types(module)?;

    Ok(())
}

fn check_global_free_variables(module: &Module) -> Result<(), VerificationError> {
    for definition in module.definitions() {
        if let Definition::FunctionDefinition(function_definition) = definition {
            if function_definition.environment().len() != 0 {
                return Err(VerificationError::InvalidFreeVariable);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::super::ast::*;
    use super::super::types;
    use super::check_global_free_variables;
    use super::error::*;

    #[test]
    fn check_no_global_free_variables() {
        assert_eq!(
            check_global_free_variables(&Module::new(vec![FunctionDefinition::new(
                "f",
                vec![],
                vec![Argument::new("x", types::Value::Number)],
                42.0,
                types::Value::Number,
            )
            .into()])),
            Ok(())
        );
    }

    #[test]
    fn fail_to_check_global_free_variables() {
        assert_eq!(
            check_global_free_variables(&Module::new(vec![FunctionDefinition::new(
                "f",
                vec![Argument::new("y", types::Value::Number)],
                vec![Argument::new("x", types::Value::Number)],
                42.0,
                types::Value::Number,
            )
            .into()])),
            Err(VerificationError::InvalidFreeVariable)
        );
    }
}
