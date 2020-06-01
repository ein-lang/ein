mod build_configuration;
mod command_target;
mod external_package;
mod target;
mod target_type;

pub use build_configuration::BuildConfiguration;
pub use command_target::CommandTarget;
pub use external_package::ExternalPackage;
use std::collections::HashMap;
pub use target::Target;

pub struct PackageConfiguration {
    package: ein::Package,
    build_configuration: BuildConfiguration,
}

impl PackageConfiguration {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        build_configuration: BuildConfiguration,
    ) -> Self {
        Self {
            package: ein::Package::new(name, version),
            build_configuration,
        }
    }

    pub fn target(&self) -> &Target {
        &self.build_configuration.target()
    }

    pub fn dependencies(&self) -> &HashMap<String, ExternalPackage> {
        &self.build_configuration.dependencies()
    }

    pub fn as_package(&self) -> &ein::Package {
        &self.package
    }
}
