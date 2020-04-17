use super::type_::Type;
use super::union::Union;
use crate::debug::SourceInformation;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_VARIABLE_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug, Derivative, Deserialize, Serialize)]
#[derivative(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variable {
    id: usize,
    #[derivative(Hash = "ignore")]
    #[derivative(Ord = "ignore")]
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    lower_types: BTreeSet<Type>,
    #[derivative(Hash = "ignore")]
    #[derivative(Ord = "ignore")]
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    upper_types: BTreeSet<Type>,
    source_information: Rc<SourceInformation>,
}

impl Variable {
    pub fn new(source_information: impl Into<Rc<SourceInformation>>) -> Self {
        Self::with_lower_types(vec![], source_information)
    }

    pub fn with_lower_types(
        lower_types: Vec<Type>,
        source_information: impl Into<Rc<SourceInformation>>,
    ) -> Self {
        Self {
            id: GLOBAL_VARIABLE_ID.fetch_add(1, Ordering::SeqCst),
            lower_types: lower_types.into_iter().collect(),
            upper_types: Default::default(),
            source_information: source_information.into(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn lower_types(&self) -> &BTreeSet<Type> {
        &self.lower_types
    }

    pub fn upper_types(&self) -> &BTreeSet<Type> {
        &self.upper_types
    }

    pub fn source_information(&self) -> &Rc<SourceInformation> {
        &self.source_information
    }

    pub fn add_lower_type(&self, type_: impl Into<Type>) -> Self {
        Self {
            id: self.id,
            lower_types: self
                .lower_types
                .iter()
                .cloned()
                .chain(vec![type_.into()])
                .collect(),
            upper_types: self.upper_types.clone(),
            source_information: self.source_information.clone(),
        }
    }

    pub fn add_upper_type(&self, type_: impl Into<Type>) -> Self {
        Self {
            id: self.id,
            lower_types: self.lower_types.clone(),
            upper_types: self
                .upper_types
                .iter()
                .cloned()
                .chain(vec![type_.into()])
                .collect(),
            source_information: self.source_information.clone(),
        }
    }

    pub fn simplify(&self) -> Type {
        Union::new(
            self.lower_types
                .iter()
                .map(|type_| type_.simplify())
                .collect(),
            self.source_information.clone(),
        )
        .simplify()
    }

    pub fn substitute_variables(&self, substitutions: &HashMap<usize, Type>) -> Type {
        match substitutions.get(&self.id) {
            Some(type_) => type_.clone(),
            None => Self {
                id: self.id,
                lower_types: self
                    .lower_types
                    .iter()
                    .map(|type_| type_.substitute_variables(substitutions))
                    .collect(),
                upper_types: self
                    .upper_types
                    .iter()
                    .map(|type_| type_.substitute_variables(substitutions))
                    .collect(),
                source_information: self.source_information.clone(),
            }
            .into(),
        }
    }
}
