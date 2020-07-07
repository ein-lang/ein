use super::any::Any;
use super::boolean::Boolean;
use super::function::Function;
use super::list::List;
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
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Type {
    Any(Any),
    Boolean(Boolean),
    Function(Function),
    List(List),
    None(None),
    Number(Number),
    Record(Record),
    Reference(Reference),
    Unknown(Unknown),
    Union(Union),
    Variable(Variable),
}

impl Type {
    pub fn source_information(&self) -> &Arc<SourceInformation> {
        match self {
            Self::Any(any) => any.source_information(),
            Self::Boolean(boolean) => boolean.source_information(),
            Self::Function(function) => function.source_information(),
            Self::List(list) => list.source_information(),
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
        self.convert_types(&mut |type_| -> Result<_, ()> {
            Ok(match type_ {
                Self::Variable(variable) => match substitutions.get(&variable.id()) {
                    Some(type_) => type_.clone(),
                    None => type_.clone(),
                },
                _ => type_.clone(),
            })
        })
        .unwrap()
    }

    pub fn convert_types<E>(
        &self,
        convert: &mut impl FnMut(&Self) -> Result<Self, E>,
    ) -> Result<Self, E> {
        let type_ = match self {
            Self::Function(function) => function.convert_types(convert)?.into(),
            Self::List(list) => list.convert_types(convert)?.into(),
            Self::Record(record) => record.convert_types(convert)?.into(),
            Self::Union(union) => union.convert_types(convert)?.into(),
            Self::Any(_)
            | Self::Boolean(_)
            | Self::None(_)
            | Self::Number(_)
            | Self::Reference(_)
            | Self::Unknown(_)
            | Self::Variable(_) => self.clone(),
        };

        convert(&type_)
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

    pub fn is_any(&self) -> bool {
        if let Self::Any(_) = self {
            true
        } else {
            false
        }
    }
}

impl From<Any> for Type {
    fn from(any: Any) -> Self {
        Self::Any(any)
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

impl From<List> for Type {
    fn from(list: List) -> Self {
        Self::List(list)
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
