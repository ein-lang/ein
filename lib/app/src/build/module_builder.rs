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
        let mut module_interfaces = external_module_interfaces
            .iter()
            .map(|(path, module_interface)| (path.clone().into(), module_interface.clone()))
            .collect::<HashMap<ein::UnresolvedModulePath, ein::ast::ModuleInterface>>();

        let mut object_file_paths = vec![];
        let mut interface_file_paths = vec![];

        for source_file_path in self.sort_source_file_paths(
            &self.file_storage.glob(
                self.file_path_manager
                    .configuration()
                    .source_file_extension(),
                &[self.file_path_manager.configuration().output_directory()],
            )?,
        )? {
            let (object_file_path, interface_file_path) =
                self.module_compiler
                    .compile(package, &module_interfaces, &source_file_path)?;

            let module_interface = serde_json::from_str::<ein::ast::ModuleInterface>(
                &self.file_storage.read_to_string(&interface_file_path)?,
            )?;
            module_interfaces.insert(
                module_interface.path().internal_unresolved().into(),
                module_interface,
            );

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
            let module = ein::parse_module(
                &self.file_storage.read_to_string(source_file_path)?,
                &format!("{}", source_file_path),
            )?;

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
