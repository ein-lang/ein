mod build_configuration;
mod command_target;
mod external_package_configuration;
mod target;
mod target_type;

use crate::common::FilePath;
pub use build_configuration::BuildConfiguration;
pub use command_target::CommandTarget;
pub use external_package_configuration::ExternalPackageConfiguration;
pub use target::Target;

#[derive(Clone, Debug)]
pub struct PackageConfiguration {
    package: ein::Package,
    build_configuration: BuildConfiguration,
    directory_path: FilePath,
}

impl PackageConfiguration {
    pub fn new(
        package: ein::Package,
        build_configuration: BuildConfiguration,
        directory_path: FilePath,
    ) -> Self {
        Self {
            package,
            build_configuration,
            directory_path,
        }
    }

    pub fn package(&self) -> &ein::Package {
        &self.package
    }

    pub fn build_configuration(&self) -> &BuildConfiguration {
        &self.build_configuration
    }

    pub fn directory_path(&self) -> &FilePath {
        &self.directory_path
    }
}
