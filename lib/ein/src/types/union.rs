use super::Type;
use crate::debug::SourceInformation;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Union {
    types: BTreeSet<Type>,
    source_information: Rc<SourceInformation>,
}

impl Union {
    pub fn new(types: Vec<Type>, source_information: impl Into<Rc<SourceInformation>>) -> Self {
        Self {
            types: types.into_iter().collect(),
            source_information: source_information.into(),
        }
    }

    pub fn types(&self) -> &BTreeSet<Type> {
        &self.types
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn simplify(&self) -> Type {
        let (unions, non_unions): (Vec<Type>, Vec<Type>) = self
            .types
            .iter()
            .cloned()
            .partition(|type_| type_.is_union());
        let unions: Vec<Vec<Type>> = unions
            .into_iter()
            .map(|type_| match type_.simplify() {
                Type::Union(union) => union.types().iter().cloned().collect(),
                type_ => vec![type_],
            })
            .collect();
        let types = vec![non_unions]
            .into_iter()
            .chain(unions)
            .flatten()
            .collect::<HashSet<_>>()
            .drain()
            .collect::<Vec<_>>();

        if types.len() == 1 {
            types[0].clone()
        } else {
            Self::new(types, self.source_information.clone()).into()
        }
    }

    pub fn substitute_variables(&self, substitutions: &HashMap<usize, Type>) -> Self {
        Self::new(
            self.types
                .iter()
                .map(|type_| type_.substitute_variables(substitutions))
                .collect(),
            self.source_information.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::none::None;
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simplify_duplicate_types() {
        assert_eq!(
            Union::new(
                vec![
                    None::new(SourceInformation::dummy()).into(),
                    None::new(SourceInformation::dummy()).into()
                ],
                SourceInformation::dummy()
            )
            .simplify(),
            None::new(SourceInformation::dummy()).into()
        );
    }

    #[test]
    fn simplify_nested_union_types() {
        assert_eq!(
            Union::new(
                vec![
                    Union::new(
                        vec![
                            None::new(SourceInformation::dummy()).into(),
                            None::new(SourceInformation::dummy()).into()
                        ],
                        SourceInformation::dummy()
                    )
                    .into(),
                    None::new(SourceInformation::dummy()).into()
                ],
                SourceInformation::dummy()
            )
            .simplify(),
            None::new(SourceInformation::dummy()).into()
        );
    }
}
