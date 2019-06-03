use super::application::Application;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Number(f64),
    Application(Application),
}

impl From<f64> for Expression {
    fn from(number: f64) -> Expression {
        Expression::Number(number)
    }
}

impl From<Application> for Expression {
    fn from(appication: Application) -> Expression {
        Expression::Application(appication)
    }
}
