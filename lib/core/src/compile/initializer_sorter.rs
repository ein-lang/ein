use super::error::CompileError;
use crate::ast::*;
use petgraph::algo::toposort;
use petgraph::graph::Graph;
use std::collections::{HashMap, HashSet};

pub struct InitializerSorter;

impl InitializerSorter {
    pub fn sort(module: &Module) -> Result<Vec<&str>, CompileError> {
        let value_names = module
            .definitions()
            .iter()
            .map(|definition| match definition {
                Definition::FunctionDefinition(_) => None,
                Definition::ValueDefinition(value_definition) => Some(value_definition.name()),
            })
            .filter(|option| option.is_some())
            .collect::<Option<HashSet<&str>>>()
            .unwrap_or_else(HashSet::new);

        let mut graph = Graph::<&str, ()>::new();
        let mut name_indices = HashMap::<&str, _>::new();

        for definition in module.definitions() {
            name_indices.insert(definition.name(), graph.add_node(definition.name()));
        }

        for definition in module.definitions() {
            for name in definition.find_global_variables(&HashSet::new()) {
                if value_names.contains(name.as_str()) {
                    graph.add_edge(
                        name_indices[name.as_str()],
                        name_indices[definition.name()],
                        (),
                    );
                }
            }
        }

        Ok(toposort(&graph, None)?
            .into_iter()
            .map(|index| graph[index])
            .filter(|name| value_names.contains(name))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types;

    #[test]
    fn sort_no_constants() {
        assert_eq!(
            InitializerSorter::sort(&Module::new(vec![], vec![])),
            Ok(vec![])
        );
    }

    #[test]
    fn sort_a_constant() {
        assert_eq!(
            InitializerSorter::sort(&Module::new(
                vec![],
                vec![ValueDefinition::new("x", 42.0, types::Value::Number).into()]
            )),
            Ok(vec!["x"])
        );
    }

    #[test]
    fn sort_sorted_constants() {
        assert_eq!(
            InitializerSorter::sort(&Module::new(
                vec![],
                vec![
                    ValueDefinition::new("x", 42.0, types::Value::Number).into(),
                    ValueDefinition::new("y", Variable::new("x"), types::Value::Number).into()
                ]
            )),
            Ok(vec!["x", "y"])
        );
    }

    #[test]
    fn sort_constants_not_sorted() {
        assert_eq!(
            InitializerSorter::sort(&Module::new(
                vec![],
                vec![
                    ValueDefinition::new("y", Variable::new("x"), types::Value::Number).into(),
                    ValueDefinition::new("x", 42.0, types::Value::Number).into(),
                ]
            )),
            Ok(vec!["x", "y"])
        );
    }

    #[test]
    fn sort_constants_not_sorted_with_function() {
        assert_eq!(
            InitializerSorter::sort(&Module::new(
                vec![],
                vec![
                    ValueDefinition::new(
                        "y",
                        Application::new(Variable::new("f"), vec![Expression::Number(42.0)]),
                        types::Value::Number
                    )
                    .into(),
                    FunctionDefinition::new(
                        "f",
                        vec![],
                        vec![Argument::new("a", types::Value::Number)],
                        Variable::new("x"),
                        types::Value::Number
                    )
                    .into(),
                    ValueDefinition::new("x", 42.0, types::Value::Number).into(),
                ]
            )),
            Ok(vec!["x", "y"])
        );
    }

    #[test]
    fn sort_constants_not_sorted_with_recursive_functions() {
        assert_eq!(
            InitializerSorter::sort(&Module::new(
                vec![],
                vec![
                    ValueDefinition::new(
                        "y",
                        Application::new(Variable::new("f"), vec![Expression::Number(42.0)]),
                        types::Value::Number
                    )
                    .into(),
                    FunctionDefinition::new(
                        "f",
                        vec![],
                        vec![Argument::new("a", types::Value::Number)],
                        Application::new(Variable::new("g"), vec![Variable::new("x").into()]),
                        types::Value::Number
                    )
                    .into(),
                    FunctionDefinition::new(
                        "g",
                        vec![],
                        vec![Argument::new("a", types::Value::Number)],
                        Application::new(Variable::new("f"), vec![Variable::new("x").into()]),
                        types::Value::Number
                    )
                    .into(),
                    ValueDefinition::new("x", 42.0, types::Value::Number).into(),
                ]
            )),
            Ok(vec!["x", "y"])
        );
    }

    #[test]
    fn fail_to_sort_recursively_defined_constant() {
        assert_eq!(
            InitializerSorter::sort(&Module::new(
                vec![],
                vec![ValueDefinition::new("x", Variable::new("x"), types::Value::Number).into()]
            )),
            Err(CompileError::CircularInitialization)
        );
    }

    #[test]
    fn fail_to_sort_recursively_defined_constants() {
        assert_eq!(
            InitializerSorter::sort(&Module::new(
                vec![],
                vec![
                    ValueDefinition::new("x", Variable::new("y"), types::Value::Number).into(),
                    ValueDefinition::new("y", Variable::new("x"), types::Value::Number).into(),
                ]
            )),
            Err(CompileError::CircularInitialization)
        );
    }
}
