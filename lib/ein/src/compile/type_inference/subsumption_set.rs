use crate::types::Type;
use std::collections::HashSet;

#[derive(Debug)]
pub struct SubsumptionSet {
    cache: HashSet<(Type, Type)>,
    subsumptions: Vec<(Type, Type)>,
}

impl SubsumptionSet {
    pub fn new() -> Self {
        Self {
            cache: HashSet::new(),
            subsumptions: vec![],
        }
    }

    pub fn add(&mut self, lower: impl Into<Type>, upper: impl Into<Type>) {
        let subsumption = (lower.into(), upper.into());

        if self.cache.contains(&subsumption) {
            return;
        }

        self.cache.insert(subsumption.clone());
        self.subsumptions.push(subsumption);
    }

    pub fn remove(&mut self) -> Option<(Type, Type)> {
        self.subsumptions.pop()
    }
}
