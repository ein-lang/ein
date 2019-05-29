#[derive(Clone, Debug, PartialEq)]
pub struct Application {
    operator: Operator,
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

impl Application {
    pub fn new(operator: Operator, lhs: Expression, rhs: Expression) -> Self {
        Application {
            operator,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn operator(&self) -> &Operator {
        &self.operator
    }

    pub fn lhs(&self) -> &Expression {
        &self.lhs
    }

    pub fn rhs(&self) -> &Expression {
        &self.rhs
    }
}

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
