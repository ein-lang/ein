use super::application::Application;
use super::let_functions::LetFunctions;
use super::let_values::LetValues;
use super::operation::Operation;
use super::variable::Variable;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Application(Application),
    LetFunctions(LetFunctions),
    LetValues(LetValues),
    Number(f64),
    Operation(Operation),
    Variable(Variable),
}

impl Expression {
    pub fn to_variable(&self) -> Option<&Variable> {
        match self {
            Self::Variable(variable) => Some(variable),
            _ => None,
        }
    }
}

impl From<f64> for Expression {
    fn from(number: f64) -> Expression {
        Expression::Number(number)
    }
}

impl From<Application> for Expression {
    fn from(application: Application) -> Expression {
        Expression::Application(application)
    }
}

impl From<LetFunctions> for Expression {
    fn from(let_functions: LetFunctions) -> Expression {
        Expression::LetFunctions(let_functions)
    }
}

impl From<LetValues> for Expression {
    fn from(let_values: LetValues) -> Expression {
        Expression::LetValues(let_values)
    }
}

impl From<Operation> for Expression {
    fn from(operation: Operation) -> Expression {
        Expression::Operation(operation)
    }
}

impl From<Variable> for Expression {
    fn from(variable: Variable) -> Expression {
        Expression::Variable(variable)
    }
}
