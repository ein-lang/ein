mod build_configuration;
mod command_target;
mod external_package;
mod target;
mod target_type;

pub use build_configuration::BuildConfiguration;
pub use command_target::CommandTarget;
pub use external_package::ExternalPackage;
pub use target::Target;

pub struct PackageConfiguration {
    package: ein::Package,
    build_configuration: BuildConfiguration,
}

impl PackageConfiguration {
    pub fn new(package: ein::Package, build_configuration: BuildConfiguration) -> Self {
        Self {
            package,
            build_configuration,
        }
    }

    pub fn package(&self) -> &ein::Package {
        &self.package
    }

    pub fn build_configuration(&self) -> &BuildConfiguration {
        &self.build_configuration
    }
}
