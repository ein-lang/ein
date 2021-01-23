use super::error::BuildError;
use super::package_builder::PackageBuilder;
use super::package_interface::PackageInterface;
use crate::common::{ExternalPackage, FilePath, PackageConfiguration};
use petgraph::algo::toposort;
use petgraph::graph::Graph;
use std::collections::HashMap;

type ExternalModuleInterfaces =
    HashMap<ExternalPackage, HashMap<lang::ExternalUnresolvedModulePath, lang::ModuleInterface>>;

pub struct ExternalPackagesBuilder<'a> {
    package_builder: &'a PackageBuilder<'a>,
}

impl<'a> ExternalPackagesBuilder<'a> {
    pub fn new(package_builder: &'a PackageBuilder<'a>) -> Self {
        Self { package_builder }
    }

    pub fn build(
        &self,
        package_configurations: &HashMap<ExternalPackage, PackageConfiguration>,
        prelude_package_interface: &PackageInterface,
    ) -> Result<(Vec<FilePath>, ExternalModuleInterfaces), Box<dyn std::error::Error>> {
        let mut package_object_file_paths = vec![];
        let mut module_interfaces = HashMap::new();

        for external_package in self.sort_external_packages(package_configurations)? {
            let package_configuration = &package_configurations[&external_package];

            let (object_file_paths, package_interface) = self.package_builder.build(
                package_configuration,
                &module_interfaces,
                Some(prelude_package_interface),
            )?;

            package_object_file_paths.extend(object_file_paths);

            module_interfaces.insert(
                external_package.clone(),
                self.convert_package_interface(&package_interface),
            );
        }

        Ok((package_object_file_paths, module_interfaces))
    }

    fn convert_package_interface(
        &self,
        package_interface: &PackageInterface,
    ) -> HashMap<lang::ExternalUnresolvedModulePath, lang::ModuleInterface> {
        package_interface
            .modules()
            .iter()
            .map(|module_interface| {
                (
                    module_interface.path().external_unresolved(),
                    module_interface.clone(),
                )
            })
            .collect()
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
            .map_err(|cycle| BuildError::PackageCircularDependency(graph[cycle.node_id()].clone()))?
            .into_iter()
            .map(|index| graph[index].clone())
            .collect())
    }
}
