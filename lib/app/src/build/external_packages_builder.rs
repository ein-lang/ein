use super::error::BuildError;
use super::package_builder::PackageBuilder;
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
        prelude_module_interfaces: &[lang::ModuleInterface],
    ) -> Result<(Vec<FilePath>, ExternalModuleInterfaces), Box<dyn std::error::Error>> {
        let mut package_object_file_paths = vec![];
        let mut external_module_interfaces = HashMap::new();

        for external_package in self.sort_external_packages(package_configurations)? {
            let package_configuration = &package_configurations[&external_package];

            let (object_file_paths, module_interfaces) = self.package_builder.build(
                package_configuration,
                &external_module_interfaces,
                &prelude_module_interfaces,
            )?;

            package_object_file_paths.extend(object_file_paths);

            external_module_interfaces.insert(
                external_package.clone(),
                self.convert_module_interfaces(&module_interfaces),
            );
        }

        Ok((package_object_file_paths, external_module_interfaces))
    }

    fn convert_module_interfaces(
        &self,
        module_interfaces: &[lang::ModuleInterface],
    ) -> HashMap<lang::ExternalUnresolvedModulePath, lang::ModuleInterface> {
        module_interfaces
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

        for package_configuration in package_configurations.values() {
            for external_package in package_configuration.build_configuration().dependencies() {
                graph.add_edge(indices[&external_package], indices[&external_package], ());
            }
        }

        Ok(toposort(&graph, None)
            .map_err(|cycle| BuildError::PackageCircularDependency(graph[cycle.node_id()].clone()))?
            .into_iter()
            .map(|index| graph[index].clone())
            .collect())
    }
}
