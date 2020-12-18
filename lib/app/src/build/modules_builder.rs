use super::error::BuildError;
use super::module_compiler::ModuleCompiler;
use super::module_parser::ModuleParser;
use super::modules_finder::ModulesFinder;
use super::package_interface::PackageInterface;
use crate::common::PackageConfiguration;
use crate::common::{FilePath, FilePathResolver};
use crate::infra::FileSystem;
use petgraph::algo::toposort;
use petgraph::graph::Graph;
use std::collections::HashMap;

pub struct ModulesBuilder<'a> {
    module_parser: &'a ModuleParser<'a>,
    module_compiler: &'a ModuleCompiler<'a>,
    modules_finder: &'a ModulesFinder<'a>,
    file_system: &'a dyn FileSystem,
    file_path_resolver: &'a FilePathResolver<'a>,
}

impl<'a> ModulesBuilder<'a> {
    pub fn new(
        module_parser: &'a ModuleParser<'a>,
        module_compiler: &'a ModuleCompiler<'a>,
        modules_finder: &'a ModulesFinder<'a>,
        file_system: &'a dyn FileSystem,
        file_path_resolver: &'a FilePathResolver<'a>,
    ) -> Self {
        Self {
            module_parser,
            module_compiler,
            modules_finder,
            file_system,
            file_path_resolver,
        }
    }

    pub fn build(
        &self,
        package_configuration: &PackageConfiguration,
        external_module_interfaces: &HashMap<
            lang::ExternalUnresolvedModulePath,
            lang::ModuleInterface,
        >,
        prelude_package_interface: Option<&PackageInterface>,
    ) -> Result<(Vec<FilePath>, Vec<FilePath>), Box<dyn std::error::Error>> {
        let mut module_interfaces = external_module_interfaces
            .iter()
            .map(|(path, module_interface)| (path.clone().into(), module_interface.clone()))
            .collect::<HashMap<lang::UnresolvedModulePath, lang::ModuleInterface>>();

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

            let module_interface = serde_json::from_str::<lang::ModuleInterface>(
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
                if let lang::UnresolvedModulePath::Internal(internal_module_path) =
                    import.module_path()
                {
                    graph.add_edge(
                        indices[&self.file_path_resolver.resolve_source_file_path(
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
