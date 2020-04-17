use super::subsumption::Subsumption;
use crate::types::Type;
use std::collections::HashSet;

#[derive(Debug)]
pub struct SubsumptionSet {
    cache: HashSet<Subsumption>,
    subsumptions: Vec<Subsumption>,
}

impl SubsumptionSet {
    pub fn new() -> Self {
        Self {
            cache: HashSet::new(),
            subsumptions: vec![],
        }
    }

    pub fn add_subsumption(&mut self, lower: impl Into<Type>, upper: impl Into<Type>) {
        let subsumption = Subsumption::new(lower, upper);

        if self.cache.contains(&subsumption) {
            return;
        }

        self.cache.insert(subsumption.clone());
        self.subsumptions.push(subsumption);
    }

    pub fn remove(&mut self) -> Option<Subsumption> {
        self.subsumptions.pop()
    }
}
