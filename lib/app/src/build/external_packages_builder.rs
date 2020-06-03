use super::error::BuildError;
use super::external_package::ExternalPackage;
use super::package_builder::PackageBuilder;
use super::package_configuration::PackageConfiguration;
use super::package_interface::PackageInterface;
use super::path::FilePathManager;
use crate::infra::{FilePath, FileStorage};
use petgraph::algo::toposort;
use petgraph::graph::Graph;
use std::collections::HashMap;

type ExternalModuleInterfaces =
    HashMap<ExternalPackage, HashMap<ein::ExternalUnresolvedModulePath, ein::ModuleInterface>>;

pub struct ExternalPackagesBuilder<'a> {
    file_storage: &'a dyn FileStorage,
    package_builder: &'a PackageBuilder<'a>,
    file_path_manager: &'a FilePathManager<'a>,
}

impl<'a> ExternalPackagesBuilder<'a> {
    pub fn new(
        file_storage: &'a dyn FileStorage,
        package_builder: &'a PackageBuilder<'a>,
        file_path_manager: &'a FilePathManager<'a>,
    ) -> Self {
        Self {
            file_storage,
            package_builder,
            file_path_manager,
        }
    }

    pub fn build(
        &self,
        package_configurations: &HashMap<ExternalPackage, PackageConfiguration>,
    ) -> Result<(Vec<FilePath>, ExternalModuleInterfaces), Box<dyn std::error::Error>> {
        let mut object_file_paths = vec![];
        let mut module_interfaces = HashMap::new();

        for external_package in self.sort_external_packages(package_configurations)? {
            let package_configuration = &package_configurations[&external_package];

            self.package_builder
                .build(package_configuration, &module_interfaces)?;

            object_file_paths.push(
                package_configuration.directory_path().join(
                    self.file_path_manager
                        .configuration()
                        .package_object_file_path(),
                ),
            );

            module_interfaces.insert(
                external_package.clone(),
                serde_json::from_str::<PackageInterface>(
                    &self.file_storage.read_to_string(
                        &package_configuration.directory_path().join(
                            self.file_path_manager
                                .configuration()
                                .package_interface_file_path(),
                        ),
                    )?,
                )?
                .modules()
                .iter()
                .map(|module_interface| {
                    (
                        module_interface.path().external_unresolved(),
                        module_interface.clone(),
                    )
                })
                .collect(),
            );
        }

        Ok((object_file_paths, module_interfaces))
    }

    fn sort_external_packages(
        &self,
        package_configurations: &HashMap<ExternalPackage, PackageConfiguration>,
    ) -> Result<Vec<ExternalPackage>, Box<dyn std::error::Error>> {
        let mut graph = Graph::<ExternalPackage, ()>::new();
        let mut indices = HashMap::<ExternalPackage, _>::new();

        for external_package in package_configurations.keys() {
            indices.insert(
                external_package.clone(),
                graph.add_node(external_package.clone()),
            );
        }

        for (external_package, package_configuration) in package_configurations {
            for (name, configuration) in package_configuration.build_configuration().dependencies()
            {
                graph.add_edge(
                    indices[&ExternalPackage::new(name, configuration.version())],
                    indices[external_package],
                    (),
                );
            }
        }

        Ok(toposort(&graph, None)
            .map_err(|_| BuildError::CircularDependency)?
            .into_iter()
            .map(|index| graph[index].clone())
            .collect())
    }
}
