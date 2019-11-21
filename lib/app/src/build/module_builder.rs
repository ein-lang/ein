use super::error::BuildError;
use super::module_compiler::ModuleCompiler;
use super::relative_module_path_converter::RelativeModulePathConverter;
use crate::infra::{FilePath, FileStorage};
use petgraph::algo::toposort;
use petgraph::graph::Graph;
use std::collections::HashMap;

pub struct ModuleBuilder<'a, S: FileStorage> {
    module_compiler: &'a ModuleCompiler<'a, S>,
    relative_module_path_converter: &'a RelativeModulePathConverter<'a>,
    source_file_storage: &'a S,
}

impl<'a, S: FileStorage> ModuleBuilder<'a, S> {
    pub fn new(
        module_compiler: &'a ModuleCompiler<'a, S>,
        relative_module_path_converter: &'a RelativeModulePathConverter<'a>,
        source_file_storage: &'a S,
    ) -> Self {
        Self {
            module_compiler,
            relative_module_path_converter,
            source_file_storage,
        }
    }

    pub fn build(&self) -> Result<Vec<FilePath>, Box<dyn std::error::Error>> {
        let file_paths = self.source_file_storage.get_file_paths()?;

        let modules = file_paths
            .iter()
            .map(|file_path| -> Result<_, Box<dyn std::error::Error>> {
                Ok((
                    file_path.clone(),
                    ein::parse_module(ein::Source::new(
                        &format!("{}", file_path),
                        &self.source_file_storage.read_to_string(file_path)?,
                    ))?,
                ))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        for file_path in &self.sort_file_paths(&modules)? {
            self.module_compiler
                .compile(file_path, &modules[file_path])?;
        }

        Ok(file_paths)
    }

    fn sort_file_paths(
        &self,
        modules: &'a HashMap<FilePath, ein::ast::UnresolvedModule>,
    ) -> Result<Vec<&'a FilePath>, Box<dyn std::error::Error>> {
        let mut graph = Graph::<&FilePath, ()>::new();
        let mut indices = HashMap::<&FilePath, _>::new();

        for file_path in modules.keys() {
            indices.insert(file_path, graph.add_node(file_path));
        }

        for (file_path, module) in modules {
            for import in module.imports() {
                if let ein::UnresolvedModulePath::Relative(relative_module_path) =
                    import.module_path()
                {
                    graph.add_edge(
                        indices[&self
                            .relative_module_path_converter
                            .convert_to_file_path(relative_module_path)],
                        indices[file_path],
                        (),
                    );
                }
            }
        }

        Ok(toposort(&graph, None)
            .map_err(|_| BuildError::CircularDepdendency)?
            .into_iter()
            .map(|index| graph[index])
            .collect())
    }
}
