use super::error::BuildError;
use super::module_parser::ModuleParser;
use super::package_configuration::PackageConfiguration;
use super::package_interface::PackageInterface;
use super::path::FilePathManager;
use crate::infra::{FilePath, FileStorage, Logger};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub struct ModuleCompiler<'a> {
    module_parser: &'a ModuleParser<'a>,
    file_path_manager: &'a FilePathManager<'a>,
    file_storage: &'a dyn FileStorage,
    logger: &'a dyn Logger,
    compile_configuration: Arc<ein::CompileConfiguration>,
}

impl<'a> ModuleCompiler<'a> {
    pub fn new(
        module_parser: &'a ModuleParser<'a>,
        file_path_manager: &'a FilePathManager<'a>,
        file_storage: &'a dyn FileStorage,
        logger: &'a dyn Logger,
        compile_configuration: Arc<ein::CompileConfiguration>,
    ) -> Self {
        Self {
            module_parser,
            file_path_manager,
            file_storage,
            logger,
            compile_configuration,
        }
    }

    pub fn compile(
        &self,
        source_file_path: &FilePath,
        module_interfaces: &HashMap<ein::UnresolvedModulePath, ein::ModuleInterface>,
        prelude_package_interfaces: &[&PackageInterface],
        package_configuration: &PackageConfiguration,
    ) -> Result<(FilePath, FilePath), Box<dyn std::error::Error>> {
        let source = self.file_storage.read_to_string(source_file_path)?;
        let module = self.module_parser.parse(&source, &source_file_path)?;

        let imported_module_interfaces = module
            .imports()
            .iter()
            .map(|import| {
                Ok(module_interfaces
                    .get(import.module_path())
                    .ok_or(BuildError::ModuleNotFound {
                        module_path: import.module_path().clone(),
                    })?
                    .clone())
            })
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

        let object_file_path =
            package_configuration
                .directory_path()
                .join(&self.generate_object_file_path(
                    source_file_path,
                    &source,
                    &imported_module_interfaces,
                ));
        let interface_file_path = object_file_path.with_extension(
            self.file_path_manager
                .configuration()
                .interface_file_extension(),
        );

        if self.file_storage.exists(&object_file_path) {
            return Ok((object_file_path, interface_file_path));
        }

        let module_path = self.file_path_manager.convert_to_module_path(
            &source_file_path.relative_to(package_configuration.directory_path()),
            package_configuration.package(),
        );

        self.logger.log(&format!(
            "compiling module {}",
            &module_path.external_unresolved()
        ))?;

        let (bitcode, module_interface) = ein::compile(
            &module.resolve(
                module_path,
                imported_module_interfaces
                    .into_iter()
                    .map(|module_interface| ein::Import::new(module_interface, true))
                    .chain(
                        prelude_package_interfaces
                            .iter()
                            .map(|package_interface| {
                                package_interface.modules().iter().map(|module_interface| {
                                    ein::Import::new(module_interface.clone(), false)
                                })
                            })
                            .flatten(),
                    )
                    .collect(),
            ),
            self.compile_configuration.clone(),
        )?;

        self.file_storage.write(&object_file_path, &bitcode)?;
        self.file_storage.write(
            &interface_file_path,
            &serde_json::to_string(&module_interface)?.as_bytes(),
        )?;

        Ok((object_file_path, interface_file_path))
    }

    fn generate_object_file_path<'b>(
        &self,
        source_file_path: &FilePath,
        source: &str,
        imported_module_interfaces: impl IntoIterator<Item = &'b ein::ModuleInterface>,
    ) -> FilePath {
        let mut hasher = DefaultHasher::new();

        source_file_path.hash(&mut hasher);
        source.hash(&mut hasher);

        for module_interface in imported_module_interfaces {
            module_interface.hash(&mut hasher);
        }

        self.file_path_manager
            .configuration()
            .object_directory_path()
            .join(&FilePath::new(&[&format!("{:x}", hasher.finish())]))
            .with_extension(
                self.file_path_manager
                    .configuration()
                    .object_file_extension(),
            )
    }
}
