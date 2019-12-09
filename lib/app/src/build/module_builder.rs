use super::error::BuildError;
use super::module_compiler::ModuleCompiler;
use super::path::FilePathManager;
use crate::infra::{FilePath, FileStorage};
use petgraph::algo::toposort;
use petgraph::graph::Graph;
use std::collections::HashMap;

pub struct ModuleBuilder<'a, S: FileStorage> {
    module_compiler: &'a ModuleCompiler<'a, S>,
    file_storage: &'a S,
    file_path_manager: &'a FilePathManager<'a>,
}

impl<'a, S: FileStorage> ModuleBuilder<'a, S> {
    pub fn new(
        module_compiler: &'a ModuleCompiler<'a, S>,
        file_storage: &'a S,
        file_path_manager: &'a FilePathManager<'a>,
    ) -> Self {
        Self {
            module_compiler,
            file_storage,
            file_path_manager,
        }
    }

    pub fn build(
        &self,
        package: &ein::Package,
        external_module_interfaces: &HashMap<
            ein::ExternalUnresolvedModulePath,
            ein::ast::ModuleInterface,
        >,
    ) -> Result<(Vec<FilePath>, Vec<FilePath>), Box<dyn std::error::Error>> {
        let mut object_file_paths = vec![];
        let mut interface_file_paths = vec![];

        for source_file_path in self.sort_source_file_paths(
            &self
                .file_storage
                .glob(self.file_path_manager.source_file_glob_pattern())?,
        )? {
            let module_path = self
                .file_path_manager
                .convert_to_module_path(source_file_path, package);
            let object_file_path = self
                .file_path_manager
                .convert_to_object_file_path(&module_path);
            let interface_file_path = self
                .file_path_manager
                .convert_to_interface_file_path(&module_path);

            self.module_compiler.compile(
                package,
                external_module_interfaces,
                &source_file_path,
                &object_file_path,
                &interface_file_path,
            )?;

            object_file_paths.push(object_file_path);
            interface_file_paths.push(interface_file_path);
        }

        Ok((object_file_paths, interface_file_paths))
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
                            .file_path_manager
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
