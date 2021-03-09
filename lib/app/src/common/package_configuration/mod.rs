mod application_target;
mod build_configuration;
mod external_package_configuration;
mod system_package_configuration;
mod target;

use crate::common::FilePath;
pub use application_target::ApplicationTarget;
pub use build_configuration::BuildConfiguration;
pub use external_package_configuration::ExternalPackageConfiguration;
pub use system_package_configuration::SystemPackageConfiguration;
pub use target::Target;

#[derive(Clone, Debug)]
pub struct PackageConfiguration {
    package: lang::Package,
    build_configuration: BuildConfiguration,
    directory_path: FilePath,
}

impl PackageConfiguration {
    pub fn new(
        package: lang::Package,
        build_configuration: BuildConfiguration,
        directory_path: FilePath,
    ) -> Self {
        Self {
            package,
            build_configuration,
            directory_path,
        }
    }

    pub fn package(&self) -> &lang::Package {
        &self.package
    }

    pub fn build_configuration(&self) -> &BuildConfiguration {
        &self.build_configuration
    }

    pub fn directory_path(&self) -> &FilePath {
        &self.directory_path
    }
}
