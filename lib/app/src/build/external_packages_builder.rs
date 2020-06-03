use super::error::BuildError;
use super::external_package_id::ExternalPackageId;
use super::package_builder::PackageBuilder;
use super::package_configuration::PackageConfiguration;
use super::package_interface::PackageInterface;
use super::path::FilePathManager;
use crate::infra::{FilePath, FileStorage};
use petgraph::algo::toposort;
use petgraph::graph::Graph;
use std::collections::HashMap;

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
        package_configurations: &HashMap<ExternalPackageId, PackageConfiguration>,
    ) -> Result<
        (
            Vec<FilePath>,
            HashMap<
                ExternalPackageId,
                HashMap<ein::ExternalUnresolvedModulePath, ein::ModuleInterface>,
            >,
        ),
        Box<dyn std::error::Error>,
    > {
        let mut object_file_paths = vec![];
        let mut module_interfaces = HashMap::new();

        for external_package_id in self.sort_external_packages(package_configurations)? {
            let directory_path = package_configurations[&external_package_id].directory_path();

            self.package_builder
                .build(directory_path, &module_interfaces)?;

            object_file_paths.push(
                directory_path.join(
                    self.file_path_manager
                        .configuration()
                        .package_object_file_path(),
                ),
            );

            module_interfaces.insert(
                external_package_id.clone(),
                serde_json::from_str::<PackageInterface>(
                    &self.file_storage.read_to_string(
                        &directory_path.join(
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
        package_configurations: &HashMap<ExternalPackageId, PackageConfiguration>,
    ) -> Result<Vec<ExternalPackageId>, Box<dyn std::error::Error>> {
        let mut graph = Graph::<ExternalPackageId, ()>::new();
        let mut indices = HashMap::<ExternalPackageId, _>::new();

        for (external_package_id, _) in package_configurations {
            indices.insert(
                external_package_id.clone(),
                graph.add_node(external_package_id.clone()),
            );
        }

        for (external_package_id, package_configuration) in package_configurations {
            for (name, configuration) in package_configuration.build_configuration().dependencies()
            {
                graph.add_edge(
                    indices[&ExternalPackageId::new(name, configuration.version())],
                    indices[external_package_id],
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
