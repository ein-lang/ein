use super::error::BuildError;
use super::module_compiler::ModuleCompiler;
use super::path::InternalModulePathManager;
use crate::infra::{FilePath, FileStorage};
use petgraph::algo::toposort;
use petgraph::graph::Graph;
use std::collections::HashMap;

pub struct ModuleBuilder<'a, D: FileStorage> {
    module_compiler: &'a ModuleCompiler<'a, D>,
    file_storage: &'a D,
    internal_module_path_converter: &'a InternalModulePathManager<'a>,
}

impl<'a, D: FileStorage> ModuleBuilder<'a, D> {
    pub fn new(
        module_compiler: &'a ModuleCompiler<'a, D>,
        file_storage: &'a D,
        internal_module_path_converter: &'a InternalModulePathManager<'a>,
    ) -> Self {
        Self {
            module_compiler,
            file_storage,
            internal_module_path_converter,
        }
    }

    pub fn build(
        &self,
        package: &ein::Package,
    ) -> Result<Vec<(FilePath, FilePath)>, Box<dyn std::error::Error>> {
        let mut file_paths = vec![];

        for source_file_path in self.sort_source_file_paths(&self.file_storage.glob("**/*.ein")?)? {
            let module_path = self
                .internal_module_path_converter
                .convert_to_module_path(source_file_path, package);
            let object_file_path = self
                .internal_module_path_converter
                .convert_to_object_file_path(&module_path);
            let interface_file_path = self
                .internal_module_path_converter
                .convert_to_interface_file_path(&module_path);

            self.module_compiler.compile(
                package,
                &source_file_path,
                &object_file_path,
                &interface_file_path,
            )?;

            file_paths.push((object_file_path, interface_file_path));
        }

        Ok(file_paths)
    }

    fn sort_source_file_paths<'b>(
        &self,
        source_file_paths: &'b [FilePath],
    ) -> Result<Vec<&'b FilePath>, Box<dyn std::error::Error>> {
        let mut graph = Graph::<&FilePath, ()>::new();
        let mut indices = HashMap::<&FilePath, _>::new();

        for source_file_path in source_file_paths {
            indices.insert(source_file_path, graph.add_node(source_file_path));
        }

        for source_file_path in source_file_paths {
            let module = ein::parse_module(ein::Source::new(
                &format!("{}", source_file_path),
                &self.file_storage.read_to_string(source_file_path)?,
            ))?;

            for import in module.imports() {
                if let ein::UnresolvedModulePath::Internal(internal_module_path) =
                    import.module_path()
                {
                    graph.add_edge(
                        indices[&self
                            .internal_module_path_converter
                            .resolve_to_source_file_path(internal_module_path)],
                        indices[&source_file_path],
                        (),
                    );
                }
            }
        }

        Ok(toposort(&graph, None)
            .map_err(|_| BuildError::CircularDependency)?
            .into_iter()
            .map(|index| graph[index])
            .collect())
    }
}
