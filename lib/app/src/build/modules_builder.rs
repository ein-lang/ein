use super::error::BuildError;
use super::module_compiler::ModuleCompiler;
use super::module_parser::ModuleParser;
use super::modules_finder::ModulesFinder;
use super::package_interface::PackageInterface;
use crate::infra::{FilePath, FileSystem};
use crate::package_configuration::PackageConfiguration;
use crate::path::FilePathManager;
use petgraph::algo::toposort;
use petgraph::graph::Graph;
use std::collections::HashMap;

pub struct ModulesBuilder<'a> {
    module_parser: &'a ModuleParser<'a>,
    module_compiler: &'a ModuleCompiler<'a>,
    modules_finder: &'a ModulesFinder<'a>,
    file_system: &'a dyn FileSystem,
    file_path_manager: &'a FilePathManager<'a>,
}

impl<'a> ModulesBuilder<'a> {
    pub fn new(
        module_parser: &'a ModuleParser<'a>,
        module_compiler: &'a ModuleCompiler<'a>,
        modules_finder: &'a ModulesFinder<'a>,
        file_system: &'a dyn FileSystem,
        file_path_manager: &'a FilePathManager<'a>,
    ) -> Self {
        Self {
            module_parser,
            module_compiler,
            modules_finder,
            file_system,
            file_path_manager,
        }
    }

    pub fn build(
        &self,
        package_configuration: &PackageConfiguration,
        external_module_interfaces: &HashMap<
            ein::ExternalUnresolvedModulePath,
            ein::ModuleInterface,
        >,
        prelude_package_interface: Option<&PackageInterface>,
    ) -> Result<(Vec<FilePath>, Vec<FilePath>), Box<dyn std::error::Error>> {
        let mut module_interfaces = external_module_interfaces
            .iter()
            .map(|(path, module_interface)| (path.clone().into(), module_interface.clone()))
            .collect::<HashMap<ein::UnresolvedModulePath, ein::ModuleInterface>>();

        let mut object_file_paths = vec![];
        let mut interface_file_paths = vec![];

        for source_file_path in self.sort_source_file_paths(
            &self
                .modules_finder
                .find(package_configuration.directory_path())?,
            package_configuration,
        )? {
            let (object_file_path, interface_file_path) = self.module_compiler.compile(
                &source_file_path,
                &module_interfaces,
                prelude_package_interface,
                package_configuration,
            )?;

            let module_interface = serde_json::from_str::<ein::ModuleInterface>(
                &self.file_system.read_to_string(&interface_file_path)?,
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
        package_configuration: &PackageConfiguration,
    ) -> Result<Vec<&'b FilePath>, Box<dyn std::error::Error>> {
        let mut graph = Graph::<&FilePath, ()>::new();
        let mut indices = HashMap::<&FilePath, _>::new();

        for source_file_path in source_file_paths {
            indices.insert(source_file_path, graph.add_node(source_file_path));
        }

        for source_file_path in source_file_paths {
            let module = self.module_parser.parse(
                &self.file_system.read_to_string(source_file_path)?,
                source_file_path,
            )?;

            for import in module.imports() {
                if let ein::UnresolvedModulePath::Internal(internal_module_path) =
                    import.module_path()
                {
                    graph.add_edge(
                        indices[&self.file_path_manager.resolve_to_source_file_path(
                            package_configuration.directory_path(),
                            internal_module_path,
                        )],
                        indices[&source_file_path],
                        (),
                    );
                }
            }
        }

        Ok(toposort(&graph, None)
            .map_err(|cycle| BuildError::ModuleCircularDependency(graph[cycle.node_id()].clone()))?
            .into_iter()
            .map(|index| graph[index])
            .collect())
    }
}
