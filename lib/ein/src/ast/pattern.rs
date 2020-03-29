use super::boolean::Boolean;
use super::none::None;
use super::number::Number;
use super::record_pattern::RecordPattern;
use super::variable::Variable;
use crate::debug::SourceInformation;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Boolean(Boolean),
    None(None),
    Number(Number),
    Record(RecordPattern),
    Variable(Variable),
}

impl Pattern {
    pub fn source_information(&self) -> &Rc<SourceInformation> {
        match self {
            Self::Boolean(boolean) => boolean.source_information(),
            Self::None(none) => none.source_information(),
            Self::Number(number) => number.source_information(),
            Self::Record(record) => record.source_information(),
            Self::Variable(variable) => variable.source_information(),
        }
    }

    pub fn to_boolean(&self) -> Option<&Boolean> {
        if let Self::Boolean(boolean) = self {
            Some(&boolean)
        } else {
            None
        }
    }

    pub fn to_number(&self) -> Option<&Number> {
        if let Self::Number(number) = self {
            Some(&number)
        } else {
            None
        }
    }

    pub fn to_variable(&self) -> Option<&Variable> {
        if let Self::Variable(variable) = self {
            Some(&variable)
        } else {
            None
        }
    }

    pub fn is_variable(&self) -> bool {
        self.to_variable().is_some()
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

impl From<RecordPattern> for Pattern {
    fn from(record: RecordPattern) -> Self {
        Self::Record(record)
    }
}

impl From<Variable> for Pattern {
    fn from(variable: Variable) -> Self {
        Self::Variable(variable)
    }
}
