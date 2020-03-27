use super::boolean::Boolean;
use super::none::None;
use super::number::Number;
use super::variable::Variable;
use crate::debug::SourceInformation;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Boolean(Boolean),
    None(None),
    Number(Number),
    Variable(Variable),
}

impl Pattern {
    pub fn source_information(&self) -> &Rc<SourceInformation> {
        match self {
            Self::Boolean(boolean) => boolean.source_information(),
            Self::None(none) => none.source_information(),
            Self::Number(number) => number.source_information(),
            Self::Variable(variable) => variable.source_information(),
        }
    }
}

impl From<Boolean> for Pattern {
    fn from(boolean: Boolean) -> Self {
        Self::Boolean(boolean)
    }
}

impl From<None> for Pattern {
    fn from(none: None) -> Self {
        Self::None(none)
    }
}

impl From<Number> for Pattern {
    fn from(number: Number) -> Self {
        Self::Number(number)
    }
}

impl From<Variable> for Pattern {
    fn from(variable: Variable) -> Self {
        Self::Variable(variable)
    }
}
