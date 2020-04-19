use super::boolean::Boolean;
use super::function::Function;
use super::none::None;
use super::number::Number;
use super::record::Record;
use super::reference::Reference;
use super::union::Union;
use super::unknown::Unknown;
use super::variable::Variable;
use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Type {
    Boolean(Boolean),
    Function(Function),
    None(None),
    Number(Number),
    Record(Record),
    Reference(Reference),
    Unknown(Unknown),
    Union(Union),
    Variable(Variable),
}

impl Type {
    pub fn source_information(&self) -> &Rc<SourceInformation> {
        match self {
            Self::Boolean(boolean) => boolean.source_information(),
            Self::Function(function) => function.source_information(),
            Self::None(none) => none.source_information(),
            Self::Number(number) => number.source_information(),
            Self::Record(record) => record.source_information(),
            Self::Reference(reference) => reference.source_information(),
            Self::Unknown(unknown) => unknown.source_information(),
            Self::Union(union) => union.source_information(),
            Self::Variable(variable) => variable.source_information(),
        }
    }

    pub fn substitute_variable(&self, id: usize, type_: &Self) -> Self {
        self.substitute_variables(&vec![(id, type_.clone())].into_iter().collect())
    }

    pub fn substitute_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        self.convert_types(&mut |type_| match type_ {
            Self::Variable(variable) => match substitutions.get(&variable.id()) {
                Some(type_) => type_.clone(),
                None => type_.clone(),
            },
            _ => type_.clone(),
        })
    }

    pub fn convert_types(&self, convert: &mut impl FnMut(&Self) -> Self) -> Self {
        let type_ = match self {
            Self::Function(function) => function.convert_types(convert).into(),
            Self::Record(record) => record.convert_types(convert).into(),
            Self::Union(union) => union.convert_types(convert).into(),
            Self::Boolean(_)
            | Self::None(_)
            | Self::Number(_)
            | Self::Reference(_)
            | Self::Unknown(_)
            | Self::Variable(_) => self.clone(),
        };

        convert(&type_)
    }

    pub fn simplify(&self) -> Self {
        match self {
            Self::Union(union) => union.simplify(),
            _ => self.clone(),
        }
    }

    pub fn to_function(&self) -> Option<&Function> {
        if let Self::Function(function) = self {
            Some(&function)
        } else {
            None
        }
    }

    pub fn to_record(&self) -> Option<&Record> {
        if let Self::Record(record) = self {
            Some(&record)
        } else {
            None
        }
    }

    pub fn to_union(&self) -> Option<&Union> {
        if let Self::Union(union) = self {
            Some(union)
        } else {
            None
        }
    }

    pub fn is_function(&self) -> bool {
        self.to_function().is_some()
    }

    pub fn is_union(&self) -> bool {
        self.to_union().is_some()
    }
}

impl From<Boolean> for Type {
    fn from(boolean: Boolean) -> Self {
        Self::Boolean(boolean)
    }
}

impl From<Function> for Type {
    fn from(function: Function) -> Self {
        Self::Function(function)
    }
}

impl From<None> for Type {
    fn from(none: None) -> Self {
        Self::None(none)
    }
}

impl From<Number> for Type {
    fn from(number: Number) -> Self {
        Self::Number(number)
    }
}

impl From<Record> for Type {
    fn from(record: Record) -> Self {
        Self::Record(record)
    }
}

impl From<Reference> for Type {
    fn from(reference: Reference) -> Self {
        Self::Reference(reference)
    }
}

impl From<Unknown> for Type {
    fn from(unknown: Unknown) -> Self {
        Self::Unknown(unknown)
    }
}

impl From<Union> for Type {
    fn from(union: Union) -> Self {
        Self::Union(union)
    }
}

impl From<Variable> for Type {
    fn from(variable: Variable) -> Self {
        Self::Variable(variable)
    }
}
