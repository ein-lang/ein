use super::equation::Equation;
use std::collections::HashSet;

#[derive(Debug)]
pub struct EquationSet {
    cache: HashSet<Equation>,
    equations: Vec<Equation>,
}

impl EquationSet {
    pub fn new() -> Self {
        Self {
            cache: HashSet::new(),
            equations: vec![],
        }
    }

    pub fn add(&mut self, equation: Equation) {
        if self.cache.contains(&equation) {
            return;
        }

        self.cache.insert(equation.clone());
        self.equations.push(equation);
    }

    pub fn remove(&mut self) -> Option<Equation> {
        self.equations.pop()
    }

    pub fn iter_mut(&mut self) -> impl IntoIterator<Item = &mut Equation> {
        self.equations.iter_mut()
    }
}
