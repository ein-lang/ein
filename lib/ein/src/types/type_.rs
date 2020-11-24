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
        self.transform_types(&mut |type_| -> Result<_, ()> {
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

    pub fn transform_types<E>(
        &self,
        transform: &mut impl FnMut(&Self) -> Result<Self, E>,
    ) -> Result<Self, E> {
        let type_ = match self {
            Self::Function(function) => function.transform_types(transform)?.into(),
            Self::List(list) => list.transform_types(transform)?.into(),
            Self::Record(record) => record.transform_types(transform)?.into(),
            Self::Union(union) => union.transform_types(transform)?.into(),
            Self::Any(_)
            | Self::Boolean(_)
            | Self::None(_)
            | Self::Number(_)
            | Self::Reference(_)
            | Self::Unknown(_)
            | Self::Variable(_) => self.clone(),
        };

        transform(&type_)
    }

    pub fn is_any(&self) -> bool {
        matches!(self, Self::Any(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    pub fn is_union(&self) -> bool {
        matches!(self, Self::Union(_))
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
